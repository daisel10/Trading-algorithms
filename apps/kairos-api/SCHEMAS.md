# Database Schemas - kairos-api

Documentación de los esquemas de base de datos utilizados por kairos-api.

---

## 🗄️ TimescaleDB (PostgreSQL)

### Tabla: market_ticks

Almacena ticks de mercado en tiempo real (datos completos).

```sql
CREATE TABLE market_ticks (
    id BIGSERIAL PRIMARY KEY,
    symbol VARCHAR(20) NOT NULL,
    exchange VARCHAR(20) NOT NULL,
    bid_price DECIMAL(20, 8) NOT NULL,
    ask_price DECIMAL(20, 8) NOT NULL,
    bid_volume DECIMAL(20, 8),
    ask_volume DECIMAL(20, 8),
    timestamp TIMESTAMPTZ NOT NULL,
    
    INDEX idx_market_ticks_symbol (symbol),
    INDEX idx_market_ticks_timestamp (timestamp DESC)
);

-- Convertir a hypertable (Timescale)
SELECT create_hypertable('market_ticks', 'timestamp');

-- Retention policy: 30 días
SELECT add_retention_policy('market_ticks', INTERVAL '30 days');
```

### Tabla: ohlcv_1h

Datos OHLCV agregados por hora.

```sql
CREATE TABLE ohlcv_1h (
    symbol VARCHAR(20) NOT NULL,
    exchange VARCHAR(20) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    open DECIMAL(20, 8) NOT NULL,
    high DECIMAL(20, 8) NOT NULL,
    low DECIMAL(20, 8) NOT NULL,
    close DECIMAL(20, 8) NOT NULL,
    volume DECIMAL(20, 8) NOT NULL,
    
    PRIMARY KEY (symbol, exchange, timestamp)
);

SELECT create_hypertable('ohlcv_1h', 'timestamp');

-- Continuous aggregate desde market_ticks
CREATE MATERIALIZED VIEW ohlcv_1h
WITH (timescaledb.continuous) AS
SELECT 
    symbol,
    exchange,
    time_bucket('1 hour', timestamp) AS timestamp,
    FIRST(bid_price, timestamp) AS open,
    MAX(ask_price) AS high,
    MIN(bid_price) AS low,
    LAST(ask_price, timestamp) AS close,
    SUM(bid_volume + ask_volume) / 2 AS volume
FROM market_ticks
GROUP BY symbol, exchange, time_bucket('1 hour', timestamp);
```

### Tabla: orders

Historial de órdenes ejecutadas.

```sql
CREATE TABLE orders (
    id UUID PRIMARY KEY,
    symbol VARCHAR(20) NOT NULL,
    exchange VARCHAR(20) NOT NULL,
    side VARCHAR(10) NOT NULL,         -- 'BUY' o 'SELL'
    order_type VARCHAR(10) NOT NULL,   -- 'MARKET' o 'LIMIT'
    quantity DECIMAL(20, 8) NOT NULL,
    price DECIMAL(20, 8),              -- NULL para MARKET orders
    filled_quantity DECIMAL(20, 8) DEFAULT 0,
    avg_price DECIMAL(20, 8),
    status VARCHAR(20) NOT NULL,       -- 'PENDING', 'FILLED', 'REJECTED', etc.
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    filled_at TIMESTAMPTZ,
    client_id UUID,                    -- Para idempotencia
    error_message TEXT,
    
    INDEX idx_orders_symbol (symbol),
    INDEX idx_orders_created_at (created_at DESC),
    INDEX idx_orders_status (status)
);
```

### Tabla: balances

Snapshot de balances (actualizado periódicamente).

```sql
CREATE TABLE balances (
    asset VARCHAR(10) NOT NULL,
    exchange VARCHAR(20) NOT NULL,
    available DECIMAL(20, 8) NOT NULL,
    locked DECIMAL(20, 8) NOT NULL DEFAULT 0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    PRIMARY KEY (asset, exchange)
);
```

---

## 🔴 DragonflyDB (Redis)

### Key Patterns

#### Market Data Cache

```
market:tick:{symbol}
```

**Tipo:** Hash  
**TTL:** 60 segundos  
**Ejemplo:**
```redis
HGETALL market:tick:BTCUSDT
{
  "bidPrice": "50000.50",
  "askPrice": "50001.00",
  "timestamp": "2026-01-06T22:00:00Z"
}
```

#### Pub/Sub Channels

```
market_data:{symbol}
```

**Tipo:** Pub/Sub Channel  
**Uso:** Broadcasting de ticks en tiempo real  
**Ejemplo:**
```redis
SUBSCRIBE market_data:BTCUSDT
# Recibe: {"symbol":"BTCUSDT","bidPrice":50000.50,"askPrice":50001.00}
```

#### Order Cache

```
order:{orderId}
```

**Tipo:** Hash  
**TTL:** 3600 segundos (1 hora)  
**Ejemplo:**
```redis
HGETALL order:123e4567-e89b-12d3-a456-426614174000
{
  "symbol": "BTCUSDT",
  "status": "FILLED",
  "quantity": "0.5"
}
```

---

## 📊 Entity Relationship Diagram

```
market_ticks
    ↓ (time_bucket agregación)
ohlcv_1h

orders
    ├─ symbol → market_ticks.symbol
    └─ exchange → market_ticks.exchange

balances
    └─ updated by order fills
```

---

## 🔄 Data Flow

```
kairos-core (Rust)
    │
    └─> Persiste → TimescaleDB (market_ticks)
    └─> Publica → DragonflyDB (Pub/Sub)
              ↓
          kairos-api (Java)
              ↓
    WebSocket → kairos-web (Angular)
```

---

## 📝 Migration Scripts

Ver `infrastructure/db/init_timescale.sql` para el script inicial de creación de tablas.

---

**Última actualización:** 2026-01-06  
**Mantenido por:** KAIRÓS Team
