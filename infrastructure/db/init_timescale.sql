-- TimescaleDB initialization script
-- Creates hypertables for time-series market data

-- Enable TimescaleDB extension
CREATE EXTENSION IF NOT EXISTS timescaledb;

-- Market ticks table
CREATE TABLE IF NOT EXISTS market_ticks (
    time TIMESTAMPTZ NOT NULL,
    symbol VARCHAR(20) NOT NULL,
    exchange VARCHAR(20) NOT NULL,
    price DOUBLE PRECISION NOT NULL,
    volume DOUBLE PRECISION NOT NULL,
    PRIMARY KEY (time, symbol, exchange)
);

-- Convert to hypertable for time-series optimization
SELECT create_hypertable('market_ticks', 'time', if_not_exists => TRUE);

-- Create indexes for common queries
CREATE INDEX IF NOT EXISTS idx_market_ticks_symbol ON market_ticks (symbol, time DESC);
CREATE INDEX IF NOT EXISTS idx_market_ticks_exchange ON market_ticks (exchange, time DESC);

-- Orders table
CREATE TABLE IF NOT EXISTS orders (
    id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    symbol VARCHAR(20) NOT NULL,
    side VARCHAR(10) NOT NULL,
    order_type VARCHAR(10) NOT NULL,
    quantity DOUBLE PRECISION NOT NULL,
    price DOUBLE PRECISION,
    status VARCHAR(20) NOT NULL,
    executed_at TIMESTAMPTZ,
    exchange VARCHAR(20) NOT NULL,
    PRIMARY KEY (created_at, id)
);

-- Convert to hypertable
SELECT create_hypertable('orders', 'created_at', if_not_exists => TRUE);

-- Compression policy for older data (compress after 7 days)
SELECT add_compression_policy('market_ticks', INTERVAL '7 days', if_not_exists => TRUE);

-- Retention policy (keep data for 90 days)
SELECT add_retention_policy('market_ticks', INTERVAL '90 days', if_not_exists => TRUE);

-- Create materialized view for OHLCV data (1-minute candles)
CREATE MATERIALIZED VIEW IF NOT EXISTS ohlcv_1m
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('1 minute', time) AS bucket,
    symbol,
    exchange,
    FIRST(price, time) AS open,
    MAX(price) AS high,
    MIN(price) AS low,
    LAST(price, time) AS close,
    SUM(volume) AS volume
FROM market_ticks
GROUP BY bucket, symbol, exchange
WITH NO DATA;

-- Refresh policy for continuous aggregate
SELECT add_continuous_aggregate_policy('ohlcv_1m',
    start_offset => INTERVAL '1 hour',
    end_offset => INTERVAL '1 minute',
    schedule_interval => INTERVAL '1 minute',
    if_not_exists => TRUE
);
