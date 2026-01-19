---
name: binance-realtime
description: >
  Binance WebSocket integration for real-time market data streaming in kairos-core.
  Trigger: When integrating Binance WebSocket, setting up real-time feeds, adding exchange connections, troubleshooting WebSocket issues, adding market symbols.
license: Apache-2.0
metadata:
  author: kairos-team
  version: "1.0"
---

## When to Use

- Integrating Binance WebSocket for real-time prices
- Adding new trading symbols to the feed
- Troubleshooting WebSocket connection issues
- Understanding the data flow from Binance to kairos-core
- Setting up auto-reconnection logic
- Adding other exchanges following the same pattern

## Critical Patterns

### WebSocket Configuration

**URL**: `wss://stream.binance.com:9443/stream`  
**Stream Type**: Combined Streams (multiple symbols simultaneously)  
**TLS**: Required (uses `native-tls` feature in `tokio-tungstenite`)  
**Data Format**: Converted to `MarketTick` domain model  
**Broadcast Channel**: 1000 message capacity for multiple consumers

### Auto-Reconnection

The feed handler automatically reconnects on:

- WebSocket disconnection
- Network errors
- Parsing errors (continues with next message)

### No API Keys Required

Binance public market data streams don't require API credentials.

## Code Examples

### Setting Up Binance Feed

```rust
use kairos_core::adapters::inbound::feed_handler::BinanceFeedHandler;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create broadcast channel for market data
    let (market_tx, _market_rx) = broadcast::channel(1000);
    
    // Symbols to track
    let symbols = vec![
        "btcusdt".to_string(),
        "ethusdt".to_string(),
    ];
    
    // Initialize Binance feed handler
    let mut binance_feed = BinanceFeedHandler::new(
        market_tx.clone(),
        symbols,
        None,  // No API credentials needed for public data
    );
    
    // Start feed handler in background
    tokio::spawn(async move {
        if let Err(e) = binance_feed.start().await {
            error!(error = %e, "Binance feed handler error");
        }
    });
    
    // Create consumer for price monitoring
    let mut price_monitor = market_tx.subscribe();
    tokio::spawn(async move {
        while let Ok(tick) = price_monitor.recv().await {
            info!(
                symbol = %tick.symbol,
                price = tick.price,
                volume = tick.volume,
                "📊 Market tick received"
            );
        }
    });
    
    Ok(())
}
```

### Adding More Symbols

```rust
// In main.rs, just add to the symbols vec
let symbols = vec![
    "btcusdt".to_string(),
    "ethusdt".to_string(),
    "solusdt".to_string(),   // ← Add Solana
    "bnbusdt".to_string(),   // ← Add Binance Coin
    "adausdt".to_string(),   // ← Add Cardano
];
```

### Creating Multiple Consumers

```rust
// Consumer 1: Price monitor
let mut price_monitor = market_tx.subscribe();
tokio::spawn(async move {
    while let Ok(tick) = price_monitor.recv().await {
        info!("Price: {} - {}", tick.symbol, tick.price);
    }
});

// Consumer 2: Strategy engine
let mut strategy_consumer = market_tx.subscribe();
tokio::spawn(async move {
    while let Ok(tick) = strategy_consumer.recv().await {
        if let Some(order) = strategy.on_market_tick(tick).await {
            // Execute order
        }
    }
});

// Consumer 3: Persistence
let mut persistence_consumer = market_tx.subscribe();
tokio::spawn(async move {
    while let Ok(tick) = persistence_consumer.recv().await {
        persistence.save_tick(&tick).await.ok();
    }
});
```

## Data Flow

```
Binance WebSocket (wss://stream.binance.com:9443/stream)
    ↓
BinanceFeedHandler::process_messages()
    ↓
Parse aggTrade JSON messages
    ↓
Convert to MarketTick domain model
    ↓
Broadcast to all consumers via channel
    ↓
┌─────────────┬──────────────┬─────────────┐
↓             ↓              ↓             ↓
Price     Strategy    Persistence    Risk
Monitor   Engine      Layer          Engine
```

## Commands

```bash
# Run kairos-core with Binance feed
cd apps/kairos-core
cargo run

# Run with debug logging
RUST_LOG=debug cargo run

# Expected output
2026-01-19T15:00:00.066Z INFO 📊 ETHUSDT | $3098.07 | Vol: 0.0220
2026-01-19T15:00:00.957Z INFO 📊 BTCUSDT | $92455.46 | Vol: 0.0039
```

## Technical Details

### TLS Configuration

The WebSocket requires TLS. Ensure `Cargo.toml` has:

```toml
tokio-tungstenite = { version = "0.24", features = ["native-tls"] }
```

### Message Format

Binance sends `aggTrade` messages:

```json
{
  "stream": "btcusdt@aggTrade",
  "data": {
    "e": "aggTrade",
    "E": 1705676400000,
    "s": "BTCUSDT",
    "a": 12345,
    "p": "50000.00",
    "q": "0.5",
    "f": 100,
    "l": 105,
    "T": 1705676400000,
    "m": true
  }
}
```

kairos-core parses this and converts to `MarketTick`:

```rust
MarketTick {
    symbol: "BTCUSDT",
    exchange: Exchange::Binance,
    bid_price: 50000.0,
    ask_price: 50000.0,
    bid_volume: 0.5,
    ask_volume: 0.5,
    timestamp: Utc timestamp,
}
```

## Troubleshooting

### Issue: TLS Error

**Error**: "TlsFeatureNotEnabled"

**Solution**: Add `native-tls` feature to `tokio-tungstenite`:

```toml
tokio-tungstenite = { version = "0.24", features = ["native-tls"] }
```

### Issue: No Data Received

**Checks**:

1. Verify symbols are lowercase: `"btcusdt"` not `"BTCUSDT"`
2. Check internet connection
3. Verify Binance API is accessible
4. Check logs for connection errors

### Issue: High Latency

**Typical Latency**: < 100ms from real trade

**Optimization**:

- Use dedicated network connection
- Run closer to Binance servers (e.g., AWS ap-northeast-1)
- Optimize consumer processing time

## Features

✅ **No API Keys Required** - Public market data  
✅ **Auto-Reconnect** - Reconnects on disconnect  
✅ **Low Latency** - Typical < 100ms  
✅ **High Throughput** - Uses Tokio async  
✅ **Type Safe** - Full Rust type safety  
✅ **Structured Logging** - JSON logs for production

## Next Steps

### Add More Consumers

```rust
// Risk management consumer
let mut risk_consumer = market_tx.subscribe();
tokio::spawn(async move {
    while let Ok(tick) = risk_consumer.recv().await {
        risk_engine.update_market_data(tick).await;
    }
});
```

### Add More Exchanges

Follow the same pattern to add OKX, Kraken, etc.:

```rust
let mut okx_feed = OkxFeedHandler::new(
    market_tx.clone(),
    symbols.clone(),
    None,
);
```

### Persist to Database

```rust
let mut db_consumer = market_tx.subscribe();
let pool = setup_db_pool().await?;

tokio::spawn(async move {
    while let Ok(tick) = db_consumer.recv().await {
        sqlx::query("INSERT INTO market_ticks ...")
            .execute(&pool)
            .await
            .ok();
    }
});
```

## Resources

- **Source Documentation**: [BINANCE_REALTIME_SUCCESS.md](file:///c:/Users/david/Documents/Trading-algorithms/apps/kairos-core/docs/BINANCE_REALTIME_SUCCESS.md)
- **Feed Handler**: [src/adapters/inbound/feed_handler/binance.rs](file:///c:/Users/david/Documents/Trading-algorithms/apps/kairos-core/src/adapters/inbound/feed_handler/binance.rs)
- **Main Entry Point**: [src/main.rs](file:///c:/Users/david/Documents/Trading-algorithms/apps/kairos-core/src/main.rs)
- **Binance WebSocket Docs**: <https://binance-docs.github.io/apidocs/spot/en/#websocket-market-streams>
