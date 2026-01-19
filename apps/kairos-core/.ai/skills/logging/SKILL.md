---
name: logging
description: >
  Hybrid logging system with tracing for kairos-core using console and file outputs.
  Trigger: When setting up logging, using structured logging, implementing tracing, configuring log levels, integrating error handling with logging, viewing logs.
license: Apache-2.0
metadata:
  author: kairos-team
  version: "1.0"
---

## When to Use

- Setting up the logging system in kairos-core
- Implementing structured logging with key-value fields
- Integrating error handling with logging
- Using the `#[instrument]` attribute for automatic spans
- Configuring different log formats (human-readable vs JSON)
- Analyzing logs with jq (for JSON logs)
- Setting up environment-specific logging configurations

## Critical Patterns

### Hybrid Logging System

kairos-core uses a **dual-output logging system**:

- **Console**: Human-readable for development OR JSON for production
- **File**: JSON structured logs with automatic rotation

**Configuration Example**:

```toml
[logging]
rust_log = "info,kairos_core=debug"
rust_backtrace = "1"
enable_file_logging = true
console_format = "human"      # or "json"
file_format = "json"
log_directory = "logs"
rotation = "daily"            # or "hourly", "never"
max_log_files = 30
```

### Log Level Selection

| Level | Macro | Use Case |
|-------|-------|----------|
| **TRACE** | `trace!()` | Function entry/exit, loop iterations |
| **DEBUG** | `debug!()` | Variable states, internal flow |
| **INFO** | `info!()` | Startup messages, configuration |
| **WARN** | `warn!()` | Deprecated features, non-critical issues |
| **ERROR** | `error!()` | Failed operations, exceptions |

### Structured Logging Best Practices

✅ **DO** use structured fields:

```rust
info!(
    exchange = "Binance",
    symbol = %tick.symbol,
    price = tick.price,
    volume = tick.volume,
    "Market tick received"
);
```

❌ **DON'T** use string interpolation only:

```rust
// BAD - not machine-parseable
info!("Market tick received from Binance for {} at {}", symbol, price);
```

### Integration with Error Handling

**Golden Rule**: Eliminate double logging

```rust
// ❌ BAD - Logs error twice
match risky_operation().await {
    Err(e) => {
        error!("Operation failed: {}", e);  // ← Logged here
        return Err(e.into());  // ← And context adds it again
    }
}

// ✅ GOOD - Log once at the decision point
match risky_operation().await {
    Ok(result) => info!("Operation succeeded"),
    Err(e) => {
        error!(error = %e, "Operation failed");
        return Err(e.into());
    }
}
```

### Using #[instrument] for Automatic Context

```rust
use tracing::instrument;

#[instrument]
async fn connect_to_exchange(url: &str) -> Result<Connection> {
    info!("Attempting connection");
    // Function arguments automatically included as fields
    // ...
}

// With custom fields and skipping sensitive data
#[instrument(skip(password), fields(user_id = user.id))]
async fn authenticate(username: &str, password: &str, user: &User) -> Result<Token> {
    // 'password' is skipped from logs, 'user_id' is added
}
```

## Code Examples

### Basic Logging

```rust
use tracing::{info, warn, error, debug, trace};

// Simple message
info!("🚀 Starting KAIRÓS Trading Core");

// With variables
let port = 50051;
info!("gRPC server listening on port {}", port);

// Error logging
error!("Failed to connect to database: {}", err);
```

### Structured Logging

```rust
// Feed handler logging
info!(
    exchange = "Binance",
    symbol = "BTCUSDT",
    price = 50000.0,
    latency_ms = 45,
    "Market data received"
);

// Order execution logging
info!(
    order_id = order.id,
    symbol = %order.symbol,
    side = %order.side,
    quantity = order.quantity,
    price = order.price,
    "Order placed"
);
```

### Logging with Spans

```rust
use tracing::{info, info_span};

async fn process_order(order_id: u64) {
    let _span = info_span!("process_order", order_id).entered();
    
    info!("Validating order");
    // ... validation logic
    
    info!("Executing order");
    // ... execution logic
}

// Output:
// process_order{order_id=123}: Validating order
// process_order{order_id=123}: Executing order
```

### Error Logging with Context

```rust
use tracing::error;

match risky_operation().await {
    Ok(result) => info!("Operation succeeded"),
    Err(e) => {
        error!(
            error = %e,
            context = "Failed during initialization",
            "Operation failed"
        );
    }
}
```

### Environment-Specific Configuration

**Development** (`config/default.toml`):

```toml
[logging]
rust_log = "info,kairos_core=debug"
console_format = "human"      # Human-readable
file_format = "json"          # JSON for analysis
enable_file_logging = true
```

**Production** (`config/production.toml`):

```toml
[logging]
rust_log = "warn,kairos_core=info"
console_format = "json"       # JSON for log aggregators (ELK, Loki)
file_format = "json"
enable_file_logging = false   # Use Docker logs instead
rust_backtrace = "0"          # Disabled for performance
```

## Commands

```bash
# Run with specific log level
$env:RUST_LOG="debug"
cargo run

# Run with module-specific logging
$env:RUST_LOG="info,kairos_core::adapters=debug"
cargo run

# Show all logs (including dependencies)
$env:RUST_LOG="trace"
cargo run -- --nocapture

# Follow live logs
tail -f logs/kairos-core.log.2026-01-19

# Analyze JSON logs with jq
cat logs/kairos-core.log.* | jq .

# Extract all error messages
cat logs/*.log.* | jq -r 'select(.level == "ERROR") | .fields.message'

# Filter by exchange
cat logs/*.log.* | jq 'select(.fields.exchange == "Binance")'

# Count errors
cat logs/*.log.* | jq -r 'select(.level == "ERROR")' | wc -l

# Get unique exchanges
cat logs/*.log.* | jq -r '.fields.exchange' | sort | uniq
```

## Best Practices

### 1. Choose the Right Log Level

```rust
info!("🚀 Starting KAIRÓS Trading Core");     // System startup
info!("📋 Configuration loaded");              // Configuration
debug!(balance = 1000.0, "Current balance");   // Debug info
trace!("Entering process_tick()");             // Function tracing
warn!("⚠️ API rate limit approaching");        // Recoverable issue
error!("❌ Failed to connect to exchange");    // Critical error
```

### 2. Include Context

```rust
// ❌ BAD
error!("Connection failed");

// ✅ GOOD
error!(
    exchange = "OKX",
    url = %ws_url,
    attempt = retry_count,
    "WebSocket connection failed"
);
```

### 3. Don't Log Sensitive Data

```rust
// ❌ NEVER
info!("API Key: {}", api_key);

// ✅ DO
info!("API credentials configured");
debug!(key_length = api_key.len(), "API key loaded");
```

### 4. Use Appropriate Frequency

```rust
// ❌ BAD - Logs every tick (thousands/second)
for tick in market_ticks {
    info!("Processing tick: {:?}", tick);
}

// ✅ GOOD - Log summary periodically
debug!(ticks_processed = count, "Batch processing complete");
```

### 5. Leverage #[instrument] for Functions

```rust
#[instrument]
async fn execute_order(order_id: u64, symbol: String) -> Result<ExecutionResult> {
    // Automatically logs function entry/exit with arguments
    info!("Executing order");
    // ...
}
```

## Log Analysis

### JSON Log Structure

```json
{
  "timestamp": "2026-01-19T15:00:00.123456Z",
  "level": "INFO",
  "fields": {
    "exchange": "Binance",
    "symbol": "BTCUSDT",
    "price": 50000.0,
    "message": "Market tick received"
  },
  "target": "kairos_core::adapters::inbound::feed_handler"
}
```

### Common jq Queries

```bash
# Pretty print
cat logs/*.log.* | jq .

# Extract messages
cat logs/*.log.* | jq -r '.fields.message'

# Filter errors
cat logs/*.log.* | jq 'select(.level == "ERROR")'

# Group by exchange
cat logs/*.log.* | jq -r '.fields.exchange' | sort | uniq -c

# Calculate average latency
cat logs/*.log.* | jq -r '.fields.latency_ms' | awk '{sum+=$1; count++} END {print sum/count}'
```

## Troubleshooting

**No logs appearing**:

1. Check `RUST_LOG` environment variable
2. Verify logging is initialized in `main.rs`
3. Check log level isn't too restrictive

**Too many logs**:

```toml
rust_log = "warn,kairos_core=info"  # Less verbose
```

**Logs not in file**:

1. Check `enable_file_logging = true`
2. Verify `log_directory` exists and is writable
3. Check disk space

## Resources

- **Source Documentation**:
  - [LOGGING.md](file:///c:/Users/david/Documents/Trading-algorithms/apps/kairos-core/docs/LOGGING.md)
  - [ERROR_LOGGING_INTEGRATION.md](file:///c:/Users/david/Documents/Trading-algorithms/apps/kairos-core/docs/ERROR_LOGGING_INTEGRATION.md)
- **Logging Module**: [src/logging.rs](file:///c:/Users/david/Documents/Trading-algorithms/apps/kairos-core/src/logging.rs)
- **tracing documentation**: <https://docs.rs/tracing>
