# KAIRÓS API - Java Spring Boot with WebFlux

REST API and WebSocket server for the KAIRÓS trading dashboard.

## Technology Stack

- **Java 21**
- **Spring Boot 3.2** with WebFlux (reactive)
- **Spring Data R2DBC** for PostgreSQL/TimescaleDB
- **Spring Data Redis Reactive** for DragonflyDB
- **gRPC Client** for kairos-core communication
- **Maven** for build management
- **Package structure**: `com.kairos.*` (simplified)

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    KAIRÓS API (Java)                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  REST Controllers                                           │
│  ├─ /api/market-data/* ──────┐                            │
│  ├─ /api/orders/*            │                            │
│  └─ /api/balance/*           ↓                            │
│                         Service Layer                      │
│  WebSocket Handler           ├─ MarketDataService         │
│  └─ /ws/market-data ────────→├─ TradingService            │
│                              └─ RealtimeDataService        │
│                                      ↓         ↓           │
├─────────────────────────────────────────────────────────────┤
│  Data Layer                          │         │           │
│  ├─ R2DBC Repositories ──────────────┘         │           │
│  ├─ gRPC Client ───────────────────────────────┼──────┐    │
│  └─ Redis Template ────────────────────────────┘      │    │
└───────────────────────────────────────────────────────┼────┘
                                                        │
    ┌─────────────────────┬─────────────────┬──────────┘
    ↓                     ↓                 ↓
┌─────────┐       ┌──────────────┐   ┌────────────┐
│TimescaleDB│       │ DragonflyDB  │   │kairos-core │
│(PostgreSQL)│       │   (Redis)    │   │   (gRPC)   │
└─────────┘       └──────────────┘   └────────────┘
```

## REST API Endpoints

### Market Data
- `GET /api/market-data/ticks/{symbol}` - Recent market ticks
- `GET /api/market-data/ticks/{symbol}/range` - Historical ticks
- `GET /api/market-data/ohlcv/{symbol}` - OHLCV candles
- `GET /api/market-data/latest/{symbol}` - Latest price (real-time)

### Orders
- `POST /api/orders` - Place new order (→ gRPC → kairos-core)
- `DELETE /api/orders/{orderId}` - Cancel order
- `GET /api/orders/{orderId}/status` - Get order status
- `GET /api/orders/history` - Order history
- `GET /api/orders/history/range` - Orders by time range
- `GET /api/orders/status/{status}` - Orders by status

### Balance
- `GET /api/balance/{currency}` - Get balance for currency

## WebSocket

- `ws://localhost:4000/ws/market-data` - Real-time market data stream from DragonflyDB Pub/Sub

## Building

```bash
cd apps/kairos-api
mvn clean package
```

## Running Locally

```bash
# Set environment variables
export TIMESCALE_HOST=localhost
export TIMESCALE_DB=kairos_trading
export TIMESCALE_USER=kairos
export TIMESCALE_PASSWORD=kairos_password
export DRAGONFLY_HOST=localhost
export CORE_GRPC_HOST=localhost

# Run with Maven
mvn spring-boot:run

# Or run JAR directly
java -jar target/kairos-api-0.1.0.jar
```

## Docker

```bash
# Build image
docker build -t kairos-api:latest .

# Run with Docker Compose (from infrastructure/)
cd infrastructure
docker-compose up kairos-api
```

## Configuration

See `src/main/resources/application.yml` for configuration options. Override via environment variables:

- `TIMESCALE_HOST`, `TIMESCALE_PORT`, `TIMESCALE_DB`, `TIMESCALE_USER`, `TIMESCALE_PASSWORD`
- `DRAGONFLY_HOST`, `DRAGONFLY_PORT`
- `CORE_GRPC_HOST`, `CORE_GRPC_PORT`
- `SERVER_PORT`
- `CORS_ALLOWED_ORIGINS`

## Migration from Rust

This Java implementation replaces the previous Rust-based API that used GraphQL. Key changes:

- **GraphQL removed** → Replaced with REST + WebSocket
- **Reactive architecture** using Spring WebFlux
- **Same databases**: PostgreSQL/TimescaleDB and DragonflyDB
- **Same gRPC contract** for kairos-core communication
- **Dashboard compatibility**: Dashboard needs to update from GraphQL to REST/WebSocket
