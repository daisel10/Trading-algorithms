# Guía de Manejo de Errores en KAIRÓS

## Filosofía

Este proyecto sigue una filosofía clara para el manejo de errores:

### ¿Quién va a consumir este error?

1. **¿Lo va a consumir tu código (para tomar decisiones)?** → Usa **Error Propio (thiserror)**
2. **¿Lo va a consumir un humano (leyendo un log)?** → Usa **anyhow**

## Tipos de Error por Capa

```
┌─────────────────────────────────────────────┐
│  main.rs, engine.rs                         │
│  → anyhow::Result                           │
│  (Errores para humanos)                     │
└─────────────────────────────────────────────┘
                    │
                    ↓ convierte con .map_err()
┌─────────────────────────────────────────────┐
│  Adapters (Inbound/Outbound)                │
│  → FeedResult, PersistenceResult, etc.      │
│  (Errores para código)                      │
└─────────────────────────────────────────────┘
                    │
                    ↓ usa pattern matching
┌─────────────────────────────────────────────┐
│  Domain (Lógica de Negocio)                 │
│  → DomainResult                             │
│  (Errores para tomar decisiones)            │
└─────────────────────────────────────────────┘
```

## Creando Errores Personalizados con thiserror

### Estructura Básica

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MiError {
    #[error("Descripción del error")]
    VarianteSinDatos,
    
    #[error("Error con contexto: {0}")]
    VarianteConString(String),
    
    #[error("Error estructurado: campo={campo}, valor={valor}")]
    VarianteEstructurada { campo: String, valor: i32 },
}

pub type MiResult<T> = Result<T, MiError>;
```

### Atributos Importantes de thiserror

#### 1. `#[from]` - Conversión Automática

Permite convertir automáticamente un error de otro tipo:

```rust
#[derive(Error, Debug)]
pub enum FeedError {
    // ✅ Conversión automática desde tokio_tungstenite::tungstenite::Error
    #[error("WebSocket error")]
    WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),
    
    // ✅ Conversión automática desde serde_json::Error
    #[error("JSON error")]
    JsonError(#[from] serde_json::Error),
}

// Uso:
async fn connect() -> FeedResult<()> {
    let ws = connect_async(url).await?;  // ← Se convierte automáticamente
    Ok(())
}
```

**⚠️ Importante**: Solo puedes tener un `#[from]` por tipo. Si necesitas múltiples variantes del mismo tipo, usa `#[source]` en su lugar.

#### 2. `#[source]` - Mantener la Cadena de Errores

Mantiene el error original sin conversión automática:

```rust
#[derive(Error, Debug)]
pub enum FeedError {
    // ✅ Mantiene el error original pero necesita conversión manual
    #[error("WebSocket connection failed")]
    ConnectionFailed(#[source] tokio_tungstenite::tungstenite::Error),
    
    // ✅ Para errores estructurados con contexto
    #[error("Failed to parse number: {field}")]
    NumberParseError {
        field: String,
        #[source]
        source: std::num::ParseFloatError,
    },
}

// Uso:
fn parse_price(s: &str) -> FeedResult<f64> {
    s.parse()
        .map_err(|source| FeedError::NumberParseError {
            field: "price".to_string(),
            source,
        })
}
```

**Beneficios de `#[source]`**:

- Mantiene la cadena de errores completa
- Permite hacer `.source()` para obtener el error original
- Stack traces completos para debugging
- No pierde información del error

## Patrones Recomendados

### ✅ Patrón Correcto 1: Conversión Automática con `#[from]`

```rust
// En el enum de error:
#[derive(Error, Debug)]
pub enum MiError {
    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

// En el código:
async fn query() -> MiResult<Data> {
    let data = sqlx::query("SELECT * FROM table")
        .fetch_one(&pool)
        .await?;  // ← Conversión automática, sin .map_err()
    Ok(data)
}
```

### ✅ Patrón Correcto 2: Contexto con `#[source]`

```rust
// En el enum de error:
#[derive(Error, Debug)]
pub enum PersistenceError {
    #[error("Failed to connect to database at {url}")]
    ConnectionFailed {
        url: String,
        #[source]
        source: sqlx::Error,
    },
}

// En el código:
async fn connect(url: &str) -> PersistenceResult<Pool> {
    let pool = PgPoolOptions::new()
        .connect(url)
        .await
        .map_err(|source| PersistenceError::ConnectionFailed {
            url: url.to_string(),
            source,
        })?;
    Ok(pool)
}
```

### ✅ Patrón Correcto 3: Conversión a anyhow en main

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Convierte errores específicos a anyhow con contexto
    let settings = Settings::new()
        .map_err(|e| anyhow::anyhow!("Failed to load config: {}", e))?;
    
    let feed = FeedHandler::new().await
        .map_err(|e| anyhow::anyhow!("Feed initialization failed: {}", e))?;
    
    Ok(())
}
```

### ✅ Patrón Correcto 4: Pattern Matching para Decisiones

```rust
// En lógica de negocio:
match risk_engine.validate_order(&order) {
    Ok(_) => {
        // Proceder con la orden
        execute_order(order).await?;
    },
    Err(DomainError::InsufficientBalance { required, available }) => {
        // Decisión específica basada en el tipo de error
        log::warn!("Balance insuficiente: necesita ${}, tiene ${}", required, available);
        notify_user_insufficient_funds(required, available).await?;
    },
    Err(DomainError::RiskLimitExceeded(msg)) => {
        // Otra decisión específica
        log::error!("Límite de riesgo excedido: {}", msg);
        halt_trading().await?;
    },
    Err(e) => {
        // Otros errores
        log::error!("Validación falló: {}", e);
        return Err(e.into());
    }
}
```

## Anti-Patrones a Evitar

### ❌ Anti-Patrón 1: Convertir a String y Perder el Error Original

```rust
// ❌ MAL - Pierde la cadena de errores
#[derive(Error, Debug)]
pub enum FeedError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),  // ← String pierde el error original
}

// ❌ MAL - Uso
let ws = connect_async(url)
    .await
    .map_err(|e| FeedError::ConnectionFailed(e.to_string()))?;
    //                                         ^^^^^^^^^^^^ Pierde info

// ✅ BIEN - Mantiene el error original
#[derive(Error, Debug)]
pub enum FeedError {
    #[error("Connection failed")]
    ConnectionFailed(#[source] tokio_tungstenite::tungstenite::Error),
}

// ✅ BIEN - Uso con #[from]
#[error("Connection failed")]
ConnectionFailed(#[from] tokio_tungstenite::tungstenite::Error),

let ws = connect_async(url).await?;  // ← Conversión automática
```

### ❌ Anti-Patrón 2: Usar anyhow en Bibliotecas/Adapters

```rust
// ❌ MAL - No uses anyhow en bibliotecas
pub async fn connect() -> anyhow::Result<Connection> {
    // El código de llamada no puede hacer pattern matching
}

// ✅ BIEN - Usa tipos propios
pub async fn connect() -> FeedResult<Connection> {
    // El código de llamada puede decidir qué hacer con cada error
}
```

### ❌ Anti-Patrón 3: Errores Genéricos Sin Información

```rust
// ❌ MAL
#[derive(Error, Debug)]
pub enum MiError {
    #[error("Error")]
    GenericError,  // ← Sin información útil
}

// ✅ BIEN
#[derive(Error, Debug)]
pub enum MiError {
    #[error("Failed to parse price from '{input}': expected decimal number")]
    InvalidPrice { input: String },
    
    #[error("Connection timeout after {seconds}s to {host}")]
    Timeout { host: String, seconds: u64 },
}
```

### ❌ Anti-Patrón 4: Múltiples `#[from]` del Mismo Tipo

```rust
// ❌ MAL - Solo puede haber un #[from] por tipo
#[derive(Error, Debug)]
pub enum MiError {
    #[error("Connection error")]
    ConnectionError(#[from] std::io::Error),
    
    #[error("Read error")]
    ReadError(#[from] std::io::Error),  // ← ERROR: conflicto
}

// ✅ BIEN - Usa #[source] para múltiples variantes
#[derive(Error, Debug)]
pub enum MiError {
    #[error("Connection error")]
    ConnectionError(#[source] std::io::Error),
    
    #[error("Read error")]
    ReadError(#[source] std::io::Error),
}

// Y convierte manualmente:
.map_err(MiError::ConnectionError)?
.map_err(MiError::ReadError)?
```

## Ejemplos por Capa

### Config Layer

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to load configuration")]
    LoadFailed(#[from] config::ConfigError),
    
    #[error("Invalid environment: {0}")]
    InvalidEnvironment(String),
    
    #[error("Missing required configuration: {0}")]
    MissingConfig(String),
}

pub type ConfigResult<T> = Result<T, ConfigError>;

// Uso:
pub fn new() -> ConfigResult<Settings> {
    let config = Config::builder()
        .add_source(File::with_name("config/default"))
        .build()?;  // ← Conversión automática con #[from]
    
    config.try_deserialize()
        .map_err(ConfigError::LoadFailed)
}
```

### Adapter Layer (Feed Handler)

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FeedError {
    #[error("WebSocket connection failed")]
    ConnectionFailed(#[from] tokio_tungstenite::tungstenite::Error),
    
    #[error("Missing API credentials for {exchange}")]
    MissingCredentials { exchange: String },
    
    #[error("Failed to parse {field}")]
    NumberParseError {
        field: String,
        #[source]
        source: std::num::ParseFloatError,
    },
}

pub type FeedResult<T> = Result<T, FeedError>;

// Uso:
pub async fn connect() -> FeedResult<WebSocket> {
    let ws = connect_async("wss://example.com").await?;  // ← Auto
    Ok(ws)
}

pub fn parse_price(s: &str) -> FeedResult<f64> {
    s.parse()
        .map_err(|source| FeedError::NumberParseError {
            field: "price".to_string(),
            source,
        })
}
```

### Domain Layer

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: f64, available: f64 },
    
    #[error("Risk limit exceeded: {0}")]
    RiskLimitExceeded(String),
    
    #[error("Invalid order: {0}")]
    InvalidOrder(String),
}

pub type DomainResult<T> = Result<T, DomainError>;

// Uso con pattern matching:
pub fn validate_order(order: &Order) -> DomainResult<()> {
    if order.amount > available_balance {
        return Err(DomainError::InsufficientBalance {
            required: order.amount,
            available: available_balance,
        });
    }
    Ok(())
}
```

### Application Layer (main.rs)

```rust
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Convierte errores específicos a anyhow
    let settings = Settings::new()
        .map_err(|e| anyhow::anyhow!("Failed to load configuration: {}", e))?;
    
    let feed = BinanceFeedHandler::start()
        .await
        .map_err(|e| anyhow::anyhow!("Feed handler failed: {}", e))?;
    
    // anyhow proporciona mensajes claros para humanos
    Ok(())
}
```

## Debugging de Errores

### Acceder al Error Original

```rust
use std::error::Error;

fn debug_error(err: &FeedError) {
    // Mensaje principal
    println!("Error: {}", err);
    
    // Error original (source)
    if let Some(source) = err.source() {
        println!("Caused by: {}", source);
    }
    
    // Toda la cadena de errores
    let mut current = err.source();
    while let Some(cause) = current {
        println!("  -> {}", cause);
        current = cause.source();
    }
}
```

### Backtrace con anyhow

```rust
use anyhow::Context;

fn main() -> anyhow::Result<()> {
    // Configurar para mostrar backtraces
    std::env::set_var("RUST_BACKTRACE", "1");
    
    something_that_fails()
        .context("Failed during initialization")?;
    
    Ok(())
}
```

## Checklist de Revisión

Al crear o revisar manejo de errores, verifica:

- [ ] ¿El error usa `thiserror` en adapters/domain?
- [ ] ¿Se usa `anyhow` solo en main/engine?
- [ ] ¿Los errores tienen `#[from]` o `#[source]` en lugar de `.to_string()`?
- [ ] ¿Los mensajes de error son descriptivos?
- [ ] ¿Se mantiene la cadena de errores completa?
- [ ] ¿Hay tipo alias `type XxxResult<T> = Result<T, XxxError>`?
- [ ] ¿El código puede hacer pattern matching cuando lo necesita?
- [ ] ¿Los errores tienen suficiente contexto para debugging?

## Referencias

- [thiserror documentation](https://docs.rs/thiserror/)
- [anyhow documentation](https://docs.rs/anyhow/)
- [Rust Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

---

## 🔗 Integración con Logging

### Principios Fundamentales

1. **❌ Evita el doble logging** - Solo loguea donde manejas el error, no en cada función
2. **✅ Usa `#[instrument]` con fields** - Agrega contexto automático a todos los logs
3. **✅ Lógica de recuperación** - Distingue errores recuperables de fatales
4. **✅ Retorna `Result`** - Las funciones no deben loguear, solo retornar errores

> Para la documentación completa, continúa leyendo...

