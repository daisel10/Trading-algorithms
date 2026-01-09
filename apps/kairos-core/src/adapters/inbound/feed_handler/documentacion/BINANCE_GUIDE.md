# 📊 Guía: Conexión a Binance WebSocket

## ✅ ¿Qué se ha implementado?

Se ha creado una implementación completa de [BinanceFeedHandler](file:///c:/Users/david/Documents/Trading-algorithms/apps/kairos-core/src/adapters/inbound/feed_handler/binance.rs#51-55) que:

1. **Conecta a Binance WebSocket** en tiempo real
2. **Suscribe a múltiples símbolos** simultáneamente (ej: BTC/USDT, ETH/USDT)
3. **Parsea datos de mercado** usando aggregated trades (aggTrade)
4. **Convierte a formato estándar** ([MarketTick](file:///c:/Users/david/Documents/Trading-algorithms/libs/kairos-domain/src/models.rs#7-15)) del dominio
5. **Broadcast automático** a todos los suscriptores del sistema
6. **Reconexión automática** en caso de desconexión o errores
7. **Logging completo** con tracing

---

## 🚀 Cómo usar el Handler

### Opción 1: Usar símbolos por defecto (BTC y ETH)

```rust
use tokio::sync::broadcast;
use kairos_domain::MarketTick;
use crate::adapters::inbound::feed_handler::binance::BinanceFeedHandler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Inicializar logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Crear canal broadcast con capacidad de 1000 mensajes
    let (market_data_tx, _rx) = broadcast::channel::<MarketTick>(1000);

    // Crear handler con símbolos por defecto: ["btcusdt", "ethusdt"]
    let binance = BinanceFeedHandler::new(market_data_tx, None);

    // Iniciar conexión (esto corre indefinidamente)
    binance.start().await?;

    Ok(())
}
```

### Opción 2: Especificar símbolos personalizados

```rust
let symbols = vec![
    "btcusdt".to_string(),
    "ethusdt".to_string(),
    "solusdt".to_string(),
    "bnbusdt".to_string(),
];

let binance = BinanceFeedHandler::new(
    market_data_tx.clone(), 
    Some(symbols)
);
```

### Opción 3: Integración completa con consumidores

```rust
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let (market_data_tx, mut market_data_rx) = broadcast::channel(1000);

    // Iniciar el feed handler en un task separado
    let feed_task = tokio::spawn({
        let tx = market_data_tx.clone();
        async move {
            let binance = BinanceFeedHandler::new(tx, None);
            binance.start().await
        }
    });

    // Consumidor 1: Estrategia de trading
    let strategy_task = tokio::spawn({
        let mut rx = market_data_tx.subscribe();
        async move {
            while let Ok(tick) = rx.recv().await {
                // Tu lógica de estrategia aquí
                if tick.symbol == "BTCUSDT" && tick.price > 100000.0 {
                    tracing::info!("🚀 BTC above $100k: ${}", tick.price);
                }
            }
        }
    });

    // Consumidor 2: Persistencia (guardar a DB)
    let persistence_task = tokio::spawn({
        let mut rx = market_data_tx.subscribe();
        async move {
            while let Ok(tick) = rx.recv().await {
                // Guardar en DragonflyDB / TimescaleDB
                tracing::debug!("💾 Saving {} to database", tick.symbol);
            }
        }
    });

    // Esperar todas las tareas
    tokio::try_join!(feed_task, strategy_task, persistence_task)?;
    
    Ok(())
}
```

---

## 📡 Formato de los Datos

Cada mensaje recibido se convierte a:

```rust
pub struct MarketTick {
    pub id: Uuid,              // UUID único para este tick
    pub symbol: String,         // Par de trading (ej: "BTCUSDT")
    pub price: f64,            // Precio del trade
    pub volume: f64,           // Volumen del trade
    pub timestamp: DateTime<Utc>, // Timestamp UTC
    pub exchange: Exchange,     // Exchange::Binance
}
```

---

## 🔧 Características Técnicas

### WebSocket Stream
- **URL Base**: `wss://stream.binance.com:9443/stream`
- **Tipo de Stream**: Aggregated Trades (`aggTrade`)
- **Formato**: Combined Streams (múltiples símbolos simultáneos)

### Manejo de Errores
- ✅ Reconexión automática cada 5 segundos si falla
- ✅ Parsing robusto con manejo de errores detallado
- ✅ Logging con tracing para debugging

### Rendimiento
- ✅ Usa Tokio async runtime (alta concurrencia)
- ✅ Broadcast channel para múltiples consumidores sin costo adicional
- ✅ Sin locks/mutex en la ruta crítica

---

## 🎯 Próximos Pasos Sugeridos

1. **Integrar en main.rs**: Añade el feed handler a tu `TradingEngine`
2. **Conectar a Redis**: Publica los ticks a DragonflyDB Pub/Sub
3. **Persistencia**: Guarda datos históricos en TimescaleDB
4. **Estrategias**: Conecta tu módulo de arbitraje/triangulación
5. **Motor de Riesgo**: Alimenta el gatekeeper con precios en tiempo real

---

## 💡 Ejemplo Completo en `main.rs`

```rust
use tokio::sync::broadcast;
use kairos_domain::MarketTick;

mod adapters;
use adapters::inbound::feed_handler::binance::BinanceFeedHandler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    tracing::info!("🚀 Starting KAIRÓS Trading Engine");

    // Create broadcast channel
    let (market_data_tx, _) = broadcast::channel::<MarketTick>(1000);

    // Configure symbols to track
    let symbols = vec![
        "btcusdt".to_string(),
        "ethusdt".to_string(),
    ];

    // Create and start Binance feed
    let binance_feed = BinanceFeedHandler::new(
        market_data_tx.clone(),
        Some(symbols)
    );

    tracing::info!("🔌 Connecting to Binance WebSocket...");
    
    // This will run forever (with auto-reconnect)
    binance_feed.start().await?;

    Ok(())
}
```

---

## 🐛 Troubleshooting

### El WebSocket no conecta
- Verifica tu conexión a Internet
- Binance puede bloquear IPs si haces demasiadas conexiones
- Revisa los logs con `RUST_LOG=debug`

### No recibo datos
- Verifica que los símbolos estén en minúsculas: `"btcusdt"` no `"BTCUSDT"`
- Asegúrate de que el símbolo existe en Binance
- Revisa que tengas suscriptores activos al canal

### Muchos warnings en compilación
- Los warnings de "never used" son normales en desarrollo
- Se resolverán cuando integres el handler en tu sistema

---

## 📝 Notas Importantes

- ✅ **Sin API Keys necesarias**: Este es un stream público (market data)
- ✅ **Rate Limits**: Binance WebSocket es muy generoso con límites
- ✅ **Latencia**: Típicamente <100ms desde el trade real
- ⚠️ **Producción**: Considera usar Binance Futures para más velocidad

¡La conexión a Binance está lista! 🎉
