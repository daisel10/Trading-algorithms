# KAIRÓS Core - High-Performance Trading Engine

**Ultra-low latency algorithmic trading engine built with Rust**

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](../../LICENSE)

---

## 📖 Overview

**KAIRÓS Core** is the heart of the KAIRÓS algorithmic trading system - a high-performance, low-latency trading engine designed to process thousands of market data ticks per second and execute trading strategies with microsecond precision.

Built with **Rust** for maximum performance and safety, KAIRÓS Core implements a **hexagonal architecture** (ports & adapters) to maintain clean separation between business logic and technical infrastructure.

### 🎯 What Does KAIRÓS Core Do?

- **Real-time Market Data Processing**: Connects to multiple cryptocurrency exchanges via WebSocket and processes market data streams
- **Trading Strategy Execution**: Implements algorithmic trading strategies (arbitrage, market making, etc.)
- **Risk Management**: Validates all orders against configurable risk parameters before execution
- **Order Execution**: Sends validated orders to exchanges with minimal latency
- **Data Persistence**: Asynchronously stores market data and trading history without blocking trading operations
- **gRPC API**: Exposes internal functionality to other microservices (kairos-api, monitoring tools)

### ✨ Key Features

- ⚡ **Ultra-low latency** - Microsecond-level processing with zero-copy operations
- 🔄 **Multi-exchange support** - Binance, OKX (extensible to other exchanges)
- 🛡️ **Built-in risk management** - Configurable position limits, stop-loss, take-profit
- 📊 **Structured logging** - Production-ready logging with tracing ecosystem
- ⚙️ **TOML configuration** - Layered configuration system for different environments
- 🚀 **Async-first design** - Built on Tokio for maximum concurrency
- 🔒 **Type-safe error handling** - thiserror + anyhow pattern throughout
- 🐳 **Docker-ready** - Multi-stage builds for optimized containers

---

## 🛠️ Technology Stack

### Core Technologies

| Component | Technology | Purpose |
|-----------|------------|---------|
| **Language** | Rust (nightly) | High performance, memory safety, zero-cost abstractions |
| **Async Runtime** | [Tokio](https://tokio.rs/) | Asynchronous I/O, task scheduling, channels |
| **Networking** | [tokio-tungstenite](https://docs.rs/tokio-tungstenite/) | WebSocket client for exchange connections |
| **gRPC** | [Tonic](https://github.com/hyperium/tonic) | High-performance gRPC server/client |
| **Serialization** | [serde](https://serde.rs/) + serde_json | JSON parsing and serialization |
| **Configuration** | [config](https://docs.rs/config/) | TOML-based layered configuration |
| **Logging** | [tracing](https://docs.rs/tracing/) + tracing-subscriber | Structured, async-aware logging |
| **Error Handling** | [thiserror](https://docs.rs/thiserror/) + [anyhow](https://docs.rs/anyhow/) | Type-safe internal errors + user-facing error context |
| **Database** | [SQLx](https://docs.rs/sqlx/) (async) | PostgreSQL/TimescaleDB access |
| **Cache** | [redis](https://docs.rs/redis/) | DragonflyDB/Redis for hot data |

### Supporting Libraries

- **chrono** - Date/time handling
- **uuid** - Unique ID generation
- **dashmap** - Concurrent HashMap
- **futures** - Async utilities
- **url** - URL parsing

### Shared Workspace Libraries

- **kairos-domain** - Shared domain models (MarketTick, Order, etc.)
- **kairos-proto** - Protocol Buffers definitions for gRPC

---

## 🏗️ Architecture

### Hexagonal Architecture (Ports & Adapters)

KAIRÓS Core follows hexagonal architecture to isolate business logic from infrastructure concerns:

```
┌─────────────────────────────────────────────────────┐
│              DOMAIN (Pure Business Logic)           │
│  - Trading Strategies (Arbitrage, Market Making)    │
│  - Risk Validation Rules                            │
│  - Domain Entities (Order, Position, Trade)         │
└────────────────────┬────────────────────────────────┘
                     │
┌────────────────────┴────────────────────────────────┐
│         APPLICATION (Use Cases & Orchestration)     │
│  - Trading Engine Coordinator                       │
│  - State Management (Atomic, Thread-safe)           │
│  - Channel-based Event Flow                         │
└────────────────────┬────────────────────────────────┘
                     │
        ┌────────────┴────────────┐
        ↓                         ↓
┌──────────────────┐    ┌──────────────────┐
│ INBOUND ADAPTERS │    │ OUTBOUND ADAPTERS│
├──────────────────┤    ├──────────────────┤
│ Feed Handler     │    │ Persistence      │
│  - Binance WS    │    │  - TimescaleDB   │
│  - OKX WS        │    │  - DragonflyDB   │
│                  │    │                  │
│ gRPC Server      │    │ Execution        │
│  - API Endpoints │    │  - Binance API   │
│                  │    │  - OKX API       │
└──────────────────┘    └──────────────────┘
```

### Real-Time Data Flow

```
Exchange WebSocket → Feed Handler → Broadcast Channel → Strategies
                                                            ↓
                                            [Opportunity Detected]
                                                            ↓
                                                    MPSC Channel
                                                            ↓
                                                      Risk Engine
                                                            ↓
                                                    [Validated?]
                                                        Yes ↓
                                                  Execution Engine
                                                            ↓
                                                Exchange REST API
```

### Module Structure

```
src/
├── main.rs                      # Application entry point
├── config.rs                    # TOML configuration system
├── logging.rs                   # Hybrid logging setup
│
├── domain/                      # Pure business logic
│   ├── strategies/              # Trading algorithms
│   ├── risk/                    # Risk management
│   └── entities.rs              # Domain models
│
├── application/                 # Use cases
│   ├── engine.rs                # Main trading engine
│   └── state.rs                 # Atomic state management
│
└── adapters/                    # External integrations
    ├── inbound/                 # Data input
    │   ├── feed_handler/        # Exchange WebSocket clients
    │   │   ├── binance.rs
    │   │   ├── okx.rs
    │   │   └── error.rs
    │   └── grpc_server/         # gRPC service implementation
    │
    └── outbound/                # Data output
        ├── persistence/         # Database adapters
        │   ├── redis.rs
        │   └── timescale.rs
        └── execution/           # Order execution
            ├── binance.rs
            └── okx.rs
```

---

## ⚙️ Configuration

KAIRÓS Core uses a **layered TOML configuration system** that allows environment-specific settings with sensible defaults.

### Configuration Files

```
config/
├── default.toml       # Base configuration (always loaded)
├── development.toml   # Development overrides
├── production.toml    # Production overrides
└── local.toml         # Local overrides (gitignored, optional)
```

### Configuration Loading Order

Each layer overrides the previous:

1. **`default.toml`** - Base settings
2. **`{environment}.toml`** - Environment-specific (dev/prod)
3. **`local.toml`** - Local overrides (not in git)
4. **Environment variables** - Highest priority

### Environment Variables

Use `KAIROS__` prefix for overrides:

```bash
# Override gRPC port
export KAIROS__GRPC__PORT=50052

# Override log level
export KAIROS__LOGGING__RUST_LOG="debug"

# API credentials (NEVER in TOML files!)
export KAIROS__EXCHANGE__OKX_API_KEY="your-key"
export KAIROS__EXCHANGE__OKX_API_SECRET="your-secret"
```

### Example Configuration

**`config/default.toml`:**

```toml
[logging]
rust_log = "info,kairos_core=debug"
enable_file_logging = true
log_directory = "logs"
console_format = "human"
file_format = "json"
rotation = "daily"

[grpc]
port = 50051
host = "0.0.0.0"

[exchange]
okx_ws_public_url = "wss://ws.okx.com:8443/ws/v5/public"
ws_reconnect_delay_ms = 5000
ws_max_reconnect_attempts = 10
```

📚 **For complete configuration guide, see:** [`docs/CONFIG_ENVIRONMENT.md`](./docs/CONFIG_ENVIRONMENT.md)

---

## 🚀 Quick Start

### Prerequisites

- **Rust** (nightly): `rustup install nightly && rustup default nightly`
- **Protocol Buffers Compiler**:
  - Windows: `choco install protoc`
  - macOS: `brew install protobuf`
  - Linux: `apt install protobuf-compiler`
- **Optional (for persistence):**
  - Docker & Docker Compose (recommended)
  - PostgreSQL/TimescaleDB
  - Redis/DragonflyDB

### Installation

```bash
# Clone the repository
cd Trading-algorithms/apps/kairos-core

# Build (debug mode)
cargo build

# Build (release mode, optimized)
cargo build --release

# Run tests
cargo test

# Run the application
cargo run --release
```

### First Run

```bash
# Set environment (optional, defaults to development)
export APP_ENV=development

# Run with custom log level
RUST_LOG=debug cargo run
```

**Expected output:**

```
🚀 Starting KAIRÓS Trading Core...
⚡ Initializing Tokio Runtime
🌍 Environment: development
📋 Configuration loaded successfully
✅ KAIRÓS Core initialized successfully
📡 Listening for market data...
```

---

## 💻 Development

### Project Layout

See [Module Structure](#module-structure) above for detailed file organization.

### Configuration

- **Environment-specific**: Use `config/{environment}.toml`
- **Local overrides**: Create `config/local.toml` (gitignored)
- **Secrets**: Always use environment variables, never commit to TOML

📚 **See:** [`docs/CONFIG_ENVIRONMENT.md`](./docs/CONFIG_ENVIRONMENT.md)

### Logging

KAIRÓS uses the `tracing` ecosystem for structured, async-aware logging:

```rust
use tracing::{info, error, debug, instrument};

// Simple logging
info!("Processing market tick");

// Structured logging with fields
info!(
    exchange = "Binance",
    symbol = "BTC-USDT",
    price = 50000.0,
    "Market data received"
);

// Function instrumentation
#[instrument]
async fn process_order(order_id: u64) -> Result<()> {
    info!("Order processing started");
    // ...
}
```

📚 **Complete logging guide:** [`docs/LOGGING.md`](./docs/LOGGING.md)

### Error Handling

KAIRÓS uses a hybrid error handling approach:

- **Internal errors** (library code): `thiserror` for type-safe errors
- **User-facing errors** (main, endpoints): `anyhow` for context-rich errors

```rust
use anyhow::Context;
use thiserror::Error;

// Internal error type
#[derive(Error, Debug)]
pub enum FeedHandlerError {
    #[error("WebSocket connection failed: {0}")]
    ConnectionFailed(String),
    
    #[error("Failed to parse message")]
    ParseError(#[from] serde_json::Error),
}

// Convert to anyhow at boundaries
let result = connect_websocket()
    .await
    .context("Failed to connect to Binance")?;
```

📚 **See:** [`docs/ERROR_HANDLING.md`](./docs/ERROR_HANDLING.md)

### Testing

```bash
# All tests
cargo test

# With output
cargo test -- --nocapture

# Specific module
cargo test adapters::inbound::feed_handler

# Integration tests (requires services)
cargo test --test integration_tests

# Benchmarks
cargo bench
```

📚 **Testing guide:** [`docs/TESTING.md`](./docs/TESTING.md)

### Code Quality

```bash
# Format code
cargo fmt

# Linting
cargo clippy -- -D warnings

# Check without building
cargo check
```

---

## 🐳 Deployment

### Docker

Build and run using the provided multi-stage Dockerfile:

```bash
# From repository root
docker build -f infrastructure/docker/Dockerfile.core -t kairos-core:latest .

# Run container
docker run -p 50051:50051 \
  -e APP_ENV=production \
  -e KAIROS__EXCHANGE__OKX_API_KEY=$OKX_API_KEY \
  kairos-core:latest
```

### Standalone Binary

```bash
# Build optimized release binary
cargo build --release

# Binary location
./target/release/kairos-core

# Copy to server
scp target/release/kairos-core user@server:/opt/kairos/
```

### Configuration for Production

**Use `production.toml` settings:**

```toml
[logging]
rust_log = "warn,kairos_core=info"  # Less verbose
rust_backtrace = "0"                # Disabled for performance
console_format = "json"             # For log aggregators

[grpc]
port = 50051
```

**Set API credentials via environment:**

```bash
export KAIROS__EXCHANGE__OKX_API_KEY="your-key"
export KAIROS__EXCHANGE__OKX_API_SECRET="your-secret"
export KAIROS__EXCHANGE__OKX_API_PASSPHRASE="your-passphrase"
```

---

## 📡 gRPC API

KAIRÓS Core exposes a gRPC API for communication with other services.

### Service Definition

Protocol Buffers definitions are in the workspace library: `libs/kairos-proto/`

### Available Services

- **TradingEngine** - Core trading operations
  - `PlaceOrder` - Submit new orders
  - `CancelOrder` - Cancel existing orders
  - `GetBalance` - Query account balance
  - `GetSystemStatus` - System health check

### Client Example

```rust
use kairos_proto::trading_engine_client::TradingEngineClient;

let mut client = TradingEngineClient::connect("http://localhost:50051").await?;

let request = tonic::Request::new(OrderRequest {
    symbol: "BTC-USDT".to_string(),
    quantity: 0.1,
    order_type: OrderType::Market as i32,
});

let response = client.place_order(request).await?;
println!("Order placed: {:?}", response.into_inner());
```

📚 **Full API documentation:** [`docs/gRPC Service.md`](./docs/gRPC%20Service.md)

---

## 📚 Documentation

| Document | Description |
|----------|-------------|
| [`docs/LOGGING.md`](./docs/LOGGING.md) | Complete logging guide with examples |
| [`docs/ERROR_HANDLING.md`](./docs/ERROR_HANDLING.md) | Error handling patterns and best practices |
| [`docs/CONFIG_ENVIRONMENT.md`](./docs/CONFIG_ENVIRONMENT.md) | Configuration system deep dive |
| [`docs/TESTING.md`](./docs/TESTING.md) | Testing strategies and guidelines |
| [`docs/gRPC Service.md`](./docs/gRPC%20Service.md) | gRPC API reference |

---

## 🔧 Troubleshooting

### Common Issues

**Problem: "failed to compile protobuf"**

```bash
# Install protoc compiler
choco install protoc  # Windows
brew install protobuf # macOS
apt install protobuf-compiler # Linux
```

**Problem: "connection refused to Redis/PostgreSQL"**

- Ensure services are running: `docker compose up -d dragonfly timescale`
- Check connection URLs in configuration

**Problem: "no logs appearing"**

- Check `RUST_LOG` environment variable
- Verify `enable_file_logging` in config

📚 **More troubleshooting:** See individual documentation files

---

## 🤝 Contributing

We welcome contributions! Please see:

- **Code Standards**: Follow Rust idioms, run `cargo fmt` and `cargo clippy`
- **Testing**: Add tests for new features
- **Documentation**: Update docs for user-facing changes

---

## 📄 License

MIT License - See [LICENSE](../../LICENSE)

---

## 🔗 Related Projects

- **kairos-api** - Spring Boot REST/WebSocket gateway
- **kairos-web** - Angular dashboard
- **kairos-domain** - Shared domain models
- **kairos-proto** - gRPC Protocol Buffers

---

**Maintained by:** KAIRÓS Team  
**Last Updated:** 2026-01-10

---

<div align="center">
  <strong>Built with ❤️ using Rust 🦀</strong>
</div>
