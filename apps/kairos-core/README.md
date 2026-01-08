# kairos-core - Trading Engine (Rust)

El motor de trading de ultra-baja latencia de KAIRÓS. Este componente procesa datos de mercado en tiempo real, ejecuta estrategias de arbitraje, valida riesgos y ejecuta órdenes.

---

## 📖 Descripción

**kairos-core** es el corazón del sistema KAIRÓS - un monolito diseñado para operar completamente en memoria con latencia de microsegundos. Implementa arquitectura hexagonal para separar lógica de dominio de adaptadores técnicos.

### Características Principales

- ⚡ **Procesamiento en Tiempo Real:** Maneja miles de ticks de mercado por segundo
- 🔄 **Arbitraje Triangular:** Detecta oportunidades usando algoritmo Bellman-Ford
- 🛡️ **Motor de Riesgo:** Valida órdenes antes de ejecución
- 🌐 **Multi-Exchange:** Soporta Binance y OKX
- 📡 **gRPC Server:** Expone API para microservicios satélites
- 💾 **Persistencia Asíncrona:** Guarda datos sin bloquear trading

---

## 🏗️ Arquitectura

### Arquitectura Hexagonal (Ports & Adapters)

```
┌─────────────────────────────────────────────┐
│           DOMAIN (Pure Logic)               │
│  - Strategies (Bellman-Ford, etc.)          │
│  - Risk Engine (Validation rules)           │
│  - Entities (Internal structs)              │
└──────────────────┬──────────────────────────┘
                   │
┌──────────────────┴──────────────────────────┐
│         APPLICATION (Use Cases)             │
│  - Engine Orchestration                     │
│  - Atomic State Management                  │
└──────────────────┬──────────────────────────┘
                   │
       ┌───────────┴───────────┐
       ↓                       ↓
┌─────────────┐         ┌─────────────┐
│   INBOUND   │         │  OUTBOUND   │
│  ADAPTERS   │         │  ADAPTERS   │
├─────────────┤         ├─────────────┤
│- Feed       │         │- Persistence│
│  Handler    │         │  (DB, Redis)│
│- gRPC Server│         │- Execution  │
│             │         │  (Exchange) │
└─────────────┘         └─────────────┘
```

### Los 5 Órganos del Monolito

1. **Feed Handler:** Escucha WebSockets de exchanges
2. **Strategies:** Ejecutan algoritmos de trading
3. **Risk Engine:** Valida órdenes contra límites
4. **Execution:** Envía órdenes al exchange
5. **Persistence:** Guarda datos históricos

---

## 🚀 Instalación y Configuración

### Prerequisitos

- **Rust Nightly:** `rustup install nightly && rustup default nightly`
- **Protocol Buffers Compiler:** `choco install protoc` (Windows)
- **DragonflyDB/Redis:** Para caché (opcional en desarrollo)
- **TimescaleDB/PostgreSQL:** Para persistencia (opcional en desarrollo)

### Clonar y Compilar

```bash
# Desde la raíz del monorepo
cd apps/kairos-core

# Compilar
cargo build --release

# Ejecutar tests
cargo test

# Ejecutar
cargo run --release
```

### Variables de Entorno

Crear `.env` en la raíz del proyecto:

```bash
# Logging
RUST_LOG=info

# gRPC Server
GRPC_PORT=50051

# DragonflyDB
DRAGONFLY_URL=redis://localhost:6379

# TimescaleDB
TIMESCALE_URL=postgresql://kairos:kairos_password@localhost:5432/kairos_trading

# Binance API
BINANCE_WS_URL=wss://stream.binance.com:9443/ws

# OKX API
OKX_WS_URL=wss://ws.okx.com:8443/ws/v5/public
```

### Configuración Avanzada

Ver [../../DEPLOYMENT.md](../../DEPLOYMENT.md) para:
- Tuning del kernel Linux para baja latencia
- Configuración de CPU affinity
- Optimizaciones de red

---

## 🧪 Testing

### Tests Unitarios

```bash
# Todos los tests
cargo test

# Tests con output detallado
cargo test -- --nocapture

# Tests de un módulo específico
cargo test domain::strategies::bellman_ford
```

### Tests de Integración

```bash
# Tests de integración (requieren servicios externos)
cargo test --test integration_tests

# Con Docker Compose para servicios de prueba
docker compose -f ../../infrastructure/docker-compose.test.yml up -d
cargo test --test integration_tests
docker compose -f ../../infrastructure/docker-compose.test.yml down
```

### Benchmarks

```bash
# Ejecutar benchmarks (requiere nightly)
cargo bench
```

### Coverage

```bash
# Instalar tarpaulin
cargo install cargo-tarpaulin

# Generar reporte de cobertura
cargo tarpaulin --out Html --output-dir ./coverage
# Ver en coverage/index.html
```

---

## 📡 API Interna (gRPC)

### Servicio TradingEngine

Definido en `../../libs/kairos-proto/proto/trading_engine.proto`

#### PlaceOrder

```protobuf
rpc PlaceOrder (OrderRequest) returns (OrderResponse);
```

**Ejemplo de uso (cliente gRPC):**

```rust
let mut client = TradingEngineClient::connect("http://localhost:50051").await?;

let request = OrderRequest {
    symbol: "BTCUSDT".to_string(),
    quantity: 0.1,
    order_type: OrderType::Market as i32,
};

let response = client.place_order(request).await?;
println!("Order ID: {}", response.into_inner().order_id);
```

#### GetBalance

```protobuf
rpc GetBalance (BalanceRequest) returns (BalanceResponse);
```

#### GetSystemStatus

```protobuf
rpc GetSystemStatus (Empty) returns (SystemStatusResponse);
```

Ver [API_DOCS.md](./API_DOCS.md) para documentación completa de endpoints.

---

## 📊 Canales de Comunicación Interna

### Broadcast Channel (Market Data)

```rust
// Feed Handler envía
broadcast_tx.send(market_tick)?;

// Strategies reciben
let mut rx = broadcast_tx.subscribe();
while let Ok(tick) = rx.recv().await {
    // Process tick
}
```

### MPSC Channel (Orders)

```rust
// Strategy envía orden
mpsc_tx.send(internal_order).await?;

// Risk Engine recibe
while let Some(order) = mpsc_rx.recv().await {
    // Validate and forward
}
```

---

## 🔧 Desarrollo

### Estructura de Código

```
src/
├── main.rs                      # Entry point
├── domain/
│   ├── strategies/
│   │   ├── mod.rs
│   │   ├── bellman_ford.rs      # Arbitrage detection
│   │   └── market_making.rs     # Market making (futuro)
│   ├── risk/
│   │   ├── mod.rs
│   │   └── validator.rs         # Order validation
│   └── entities.rs
├── application/
│   ├── engine.rs                # Main orchestrator
│   └── state.rs                 # Atomic state management
└── adapters/
    ├── inbound/
    │   ├── feed_handler/
    │   │   ├── mod.rs
    │   │   ├── binance.rs       # Binance WebSocket
    │   │   └── okx.rs           # OKX WebSocket
    │   └── grpc_server/
    │       ├── mod.rs
    │       └── service.rs
    └── outbound/
        ├── persistence/
        │   ├── mod.rs
        │   ├── redis.rs         # DragonflyDB
        │   └── timescale.rs     # TimescaleDB
        └── execution/
            ├── mod.rs
            ├── binance.rs       # Binance order execution
            └── okx.rs           # OKX order execution
```

### Añadir una Nueva Estrategia

1. Crear archivo en `src/domain/strategies/my_strategy.rs`
2. Implementar trait `Strategy`:

```rust
#[async_trait]
pub trait Strategy {
    async fn on_market_tick(&mut self, tick: MarketTick) -> Option<InternalOrder>;
}
```

3. Registrar en `src/application/engine.rs`

### Añadir un Nuevo Exchange

1. Crear `src/adapters/inbound/feed_handler/new_exchange.rs`
2. Implementar conexión WebSocket
3. Normalizar datos a `MarketTick`
4. Crear `src/adapters/outbound/execution/new_exchange.rs`
5. Implementar envío de órdenes

---

## 🐛 Debugging

### Logs

```bash
# Nivel de log detallado
RUST_LOG=debug cargo run

# Solo logs de un módulo específico
RUST_LOG=kairos_core::adapters::inbound::feed_handler=debug cargo run
```

### Herramientas

- **rust-gdb / rust-lldb:** Debugger nativo
- **cargo-flamegraph:** Profiling de CPU
- **heaptrack:** Análisis de memoria

---

## 📝 Linting y Formateo

```bash
# Formatear código
cargo fmt

# Linter
cargo clippy -- -D warnings

# Verificar sin compilar
cargo check
```

---

## 🚢 Deployment

### Docker

```bash
# Desde la raíz del proyecto
docker build -f infrastructure/docker/Dockerfile.core -t kairos-core:latest .

# Ejecutar
docker run -p 50051:50051 --env-file .env kairos-core:latest
```

### Binary Standalone

```bash
# Compilar release optimizado
cargo build --release --target x86_64-unknown-linux-gnu

# Binary en
./target/release/kairos-core
```

---

## 📚 Referencias

- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Documentation](https://tokio.rs/)
- [Tonic gRPC Guide](https://github.com/hyperium/tonic)
- [Arquitectura Hexagonal](https://alistair.cockburn.us/hexagonal-architecture/)

---

## 🤝 Contribuir

Ver [../../CONTRIBUTING.md](../../CONTRIBUTING.md) para:
- Estándares de código Rust
- Flujo de trabajo Git
- Proceso de code review

---

## 📄 Licencia

MIT License - Ver [../../LICENSE](../../LICENSE)

---

**Mantenido por:** KAIRÓS Team  
**Última actualización:** 2026-01-06
