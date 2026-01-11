# Integración de Error Handling + Logging en KAIRÓS

## 📚 Tabla de Contenidos

- [Principios Fundamentales](#principios-fundamentales)
- [Anti-Patrón: Doble Logging](#anti-patrón-doble-logging)
- [Uso de #[instrument]](#uso-de-instrument-para-contexto-automático)
- [Lógica de Recuperación](#lógica-de-recuperación-vs-errores-fatales)
- [Categorización para Logging](#categorización-de-errores-para-logging)
- [Patrones Completos](#patrones-completos-por-caso-de-uso)
- [Checklist](#checklist-de-integración-error--logging)

---

## Principios Fundamentales

1. **❌ Evita el doble logging** - Solo loguea donde manejas el error, no en cada función
2. **✅ Usa `#[instrument]` con fields** - Agrega contexto automático a todos los logs
3. **✅ Lógica de recuperación** - Distingue errores recuperables de fatales
4. **✅ Retorna `Result`** - Las funciones no deben loguear, solo retornar errores

---

## Anti-Patrón: Doble Logging

### ❌ MAL - Logging en cada función

```rust
use tracing::error;

async fn connect() -> FeedResult<WebSocket> {
    match connect_async(url).await {
        Ok(ws) => Ok(ws),
        Err(e) => {
            // ❌ MAL: Loguea aquí
            error!(error = %e, "Failed to connect");
            Err(e.into())
        }
    }
}

async fn start() -> Result<()> {
    match connect().await {
        Ok(ws) => { /* ... */ },
        Err(e) => {
            // ❌ MAL: Loguea de nuevo aquí
            error!(error = %e, "Feed handler failed");
            return Err(e.into());
        }
    }
}
```

**Problema:** El mismo error se loguea múltiples veces, generando ruido en los logs.

### ✅ BIEN - Logging solo donde se maneja

```rust
use tracing::{error, info, warn};

// ✅ Solo retorna Result, sin logging
async fn connect() -> FeedResult<WebSocket> {
    let ws = connect_async(url).await?;
    Ok(ws)
}

// ✅ Solo retorna Result, sin logging
async fn process_message(&self, msg: Message) -> FeedResult<()> {
    let tick = self.parse_message(msg)?;
    self.broadcast_tx.send(tick)?;
    Ok(())
}

// ✅ Loguea solo donde se maneja el error
async fn start() -> Result<()> {
    loop {
        match self.receive_message().await {
            Ok(msg) => {
                if let Err(e) = self.process_message(msg).await {
                    // ✅ Loguea aquí porque decides qué hacer
                    warn!(
                        error = %e,
                        "Failed to process message, continuing"
                    );
                }
            }
            Err(e) if e.is_connection_error() => {
                // ✅ Loguea aquí porque decides reconectar
                error!(error = %e, "Connection lost, reconnecting");
                self.reconnect().await?;
            }
            Err(e) => {
                // ✅ Loguea aquí porque es fatal
                error!(error = %e, "Fatal error");
                return Err(e.into());
            }
        }
    }
}
```

---

## Uso de `#[instrument]` para Contexto Automático

El atributo `#[instrument]` de `tracing` agrega automáticamente contexto a **todos** los logs dentro de una función y sus funciones hijas.

### ✅ Patrón Recomendado

```rust
use tracing::{instrument, info, warn, error};

pub struct BinanceFeedHandler {
    exchange: String,
}

impl BinanceFeedHandler {
    // ✅ #[instrument] agrega 'exchange' a TODOS los logs
    #[instrument(skip(self), fields(exchange = "Binance"))]
    pub async fn start(&self) -> Result<()> {
        // Automáticamente incluye exchange="Binance"
        info!("Starting feed handler");
        
        self.connect().await?;
        // También incluye exchange="Binance"
        info!("Connected");
        
        self.run_loop().await?;
        Ok(())
    }
    
    // ✅ Campos dinámicos
    #[instrument(skip(self), fields(exchange = %self.exchange))]
    async fn connect(&self) -> FeedResult<()> {
        // Todos los logs aquí tienen exchange="Binance"
        info!("Attempting connection");
        // ...
        Ok(())
    }
    
    // ✅ Campos adicionales
    #[instrument(
        skip(self, msg),
        fields(
            exchange = "Binance",
            msg_type = ?msg.msg_type
        )
    )]
    async fn process_message(&self, msg: Message) -> FeedResult<()> {
        // Incluye exchange Y msg_type
        info!("Processing message");
        // ...
        Ok(())
    }
}
```

**Beneficios:**

- **Menos código repetido** - No necesitas escribir `exchange = "Binance"` en cada log
- **Contexto consistente** - Todos los logs tienen los mismos campos
- **Jerarquía automática** - Los logs de funciones hijas heredan el contexto

### Output JSON Ejemplo

```json
{
  "timestamp": "2026-01-10T22:00:00.123Z",
  "level": "INFO",
  "fields": {
    "exchange": "Binance",
    "message": "Attempting connection"
  },
  "target": "kairos_core::adapters::inbound::feed_handler"
}
```

---

## Lógica de Recuperación vs Errores Fatales

No todos los errores son iguales. Algunos son recuperables, otros son fatales.

### ✅ Patrón Recomendado

```rust
use tracing::{error, warn, info, instrument};

impl BinanceFeedHandler {
    #[instrument(skip(self), fields(exchange = "Binance"))]
    pub async fn start(&self) -> Result<()> {
        info!("Starting feed handler");
        
        // Primera conexión
        self.connect().await?;
        
        // Loop principal con manejo de errores
        loop {
            match self.receive_message().await {
                Ok(msg) => {
                    // ✅ Error recuperable - procesar puede fallar, pero continúa
                    if let Err(e) = self.process_message(msg).await {
                        warn!(
                            error = %e,
                            error_type = ?e,
                            "Failed to process message, continuing"
                        );
                        // Continúa el loop, no es fatal
                    }
                }
                
                // ✅ Error recuperable - reconectar
                Err(e) if e.is_connection_error() => {
                    error!(
                        error = %e,
                        "Connection lost, attempting reconnect"
                    );
                    
                    match self.reconnect().await {
                        Ok(_) => {
                            info!("Reconnected successfully");
                            continue;
                        }
                        Err(e) => {
                            error!(
                                error = %e,
                                "Failed to reconnect"
                            );
                            return Err(e.into());
                        }
                    }
                }
                
                // ✅ Error fatal - detener todo
                Err(e) => {
                    error!(
                        error = %e,
                        error_debug = ?e,
                        "Fatal error, shutting down"
                    );
                    return Err(e.into());
                }
            }
        }
    }
    
    // ✅ Esta función solo retorna Result, sin logging
    async fn connect(&self) -> FeedResult<()> {
        let ws = connect_async(&self.url).await?;
        self.ws = Some(ws);
        Ok(())
    }
    
    // ✅ Reconexión con reintentos
    async fn reconnect(&self) -> FeedResult<()> {
        for attempt in 1..=self.config.max_reconnect_attempts {
            warn!(attempt, "Reconnection attempt");
            
            match self.connect().await {
                Ok(_) => return Ok(()),
                Err(e) if attempt < self.config.max_reconnect_attempts => {
                    warn!(
                        attempt,
                        error = %e,
                        delay_ms = self.config.reconnect_delay_ms,
                        "Reconnection failed, retrying"
                    );
                    tokio::time::sleep(
                        Duration::from_millis(self.config.reconnect_delay_ms)
                    ).await;
                }
                Err(e) => {
                    error!(
                        attempt,
                        error = %e,
                        "All reconnection attempts exhausted"
                    );
                    return Err(e);
                }
            }
        }
        unreachable!()
    }
}
```

---

## Categorización de Errores para Logging

Usa diferentes niveles de log según la severidad:

| Nivel | Cuándo Usar | Acción |
|-------|-------------|--------|
| `error!` | Error fatal que detiene la operación | Loguear y propagar |
| `warn!` | Error recuperable que no detiene | Loguear y continuar |
| `info!` | Operación exitosa después de error | Loguear recuperación |
| `debug!` | Información de debugging | Detalles internos |

### Ejemplo Completo

```rust
use tracing::{error, warn, info, debug, instrument};

impl OrderExecutor {
    #[instrument(
        skip(self, order),
        fields(
            order_id = order.id,
            symbol = %order.symbol,
            side = %order.side
        )
    )]
    pub async fn execute_order(&self, order: Order) -> Result<ExecutionResult> {
        info!("Executing order");
        
        match self.send_to_exchange(&order).await {
            Ok(result) => {
                info!(
                    fill_price = result.fill_price,
                    "Order executed successfully"
                );
                Ok(result)
            }
            
            // ⚠️ Error recuperable - reintentar
            Err(e) if e.is_retryable() && self.retry_count < 3 => {
                warn!(
                    error = %e,
                    retry_count = self.retry_count,
                    "Order execution failed, retrying"
                );
                
                tokio::time::sleep(Duration::from_secs(1)).await;
                self.retry_count += 1;
                self.execute_order(order).await
            }
            
            // ❌ Error fatal
            Err(e) => {
                error!(
                    error = %e,
                    error_chain = ?e,
                    retry_count = self.retry_count,
                    "Order execution failed permanently"
                );
                Err(e.into())
            }
        }
    }
}
```

---

## Patrones Completos por Caso de Uso

### Caso 1: Procesamiento de Mensajes con Recuperación

```rust
#[instrument(skip(self), fields(exchange = "OKX"))]
pub async fn run(&self) -> Result<()> {
    loop {
        match self.ws.next().await {
            Some(Ok(msg)) => {
                // Procesar puede fallar, pero no es fatal
                if let Err(e) = self.handle_message(msg).await {
                    warn!(error = %e, "Failed to handle message");
                    // Continúa procesando otros mensajes
                }
            }
            Some(Err(e)) => {
                error!(error = %e, "WebSocket error, reconnecting");
                self.reconnect().await?;
            }
            None => {
                info!("WebSocket closed cleanly");
                break;
            }
        }
    }
    Ok(())
}

// Sin logging, solo retorna Result
async fn handle_message(&self, msg: Message) -> FeedResult<()> {
    let tick = self.parse_message(msg)?;
    self.broadcast(tick).await?;
    Ok(())
}
```

### Caso 2: Operaciones con Timeout

```rust
use tokio::time::{timeout, Duration};

#[instrument(skip(self))]
async fn fetch_with_timeout(&self, url: &str) -> Result<Data> {
    match timeout(Duration::from_secs(30), self.fetch(url)).await {
        Ok(Ok(data)) => {
            debug!("Fetch successful");
            Ok(data)
        }
        Ok(Err(e)) => {
            error!(error = %e, url, "Fetch failed");
            Err(e.into())
        }
        Err(_) => {
            error!(url, timeout_seconds = 30, "Fetch timeout");
            Err(anyhow::anyhow!("Request timeout"))
        }
    }
}
```

### Caso 3: Cadena de Operaciones con Context

```rust
use anyhow::Context;

#[instrument(skip(self), fields(user_id = user.id))]
pub async fn process_user_order(&self, user: &User, order: Order) -> Result<()> {
    // Cada operación agrega contexto sin loguear
    let balance = self
        .get_balance(user.id)
        .await
        .context("Failed to fetch user balance")?;
    
    self.validate_balance(&order, balance)
        .context("Insufficient balance")?;
    
    let risk_check = self
        .risk_engine
        .validate(&order)
        .await
        .context("Risk validation failed")?;
    
    let result = self
        .execute_order(order)
        .await
        .context("Order execution failed")?;
    
    // ✅ Solo loguea el éxito final
    info!(
        order_id = order.id,
        fill_price = result.fill_price,
        "Order processed successfully"
    );
    
    Ok(())
}

// Manejo en el caller
match process_user_order(&user, order).await {
    Ok(_) => { /* success */ }
    Err(e) => {
        // ✅ Aquí se loguea con toda la cadena de contexto
        error!(
            error = %e,
            user_id = user.id,
            "Failed to process order"
        );
    }
}
```

---

## Checklist de Integración Error + Logging

Al escribir código con errores y logging, verifica:

- [ ] **Las funciones retornan `Result`** sin loguear internamente
- [ ] **Solo logueas donde manejas el error** (decide qué hacer)
- [ ] **Usas `#[instrument]`** para contexto automático
- [ ] **Diferentes niveles según severidad**:
  - `error!` para errores fatales
  - `warn!` para errores recuperables
  - `info!` para recuperación exitosa
- [ ] **Campos estructurados** incluyen:
  - `error = %e` - Mensaje del error
  - `error_debug = ?e` - Debug completo
  - Contexto relevante (user_id, order_id, etc.)
- [ ] **Lógica de recuperación** distingue:
  - Errores recuperables (continúa/reintenta)
  - Errores fatales (propaga y detiene)
- [ ] **Pattern matching** para decisiones específicas por tipo de error

---

## Referencias

- [ERROR_HANDLING.md](./ERROR_HANDLING.md) - Guía completa de manejo de errores
- [LOGGING.md](./LOGGING.md) - Guía completa de logging
- [tracing documentation](https://docs.rs/tracing)
- [thiserror documentation](https://docs.rs/thiserror/)
- [anyhow documentation](https://docs.rs/anyhow/)
