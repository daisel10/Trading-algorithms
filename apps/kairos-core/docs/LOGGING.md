# 📝 Logging Guide for KAIRÓS Core

## 📚 Table of Contents

- [Overview](#overview)
- [System Architecture](#system-architecture)
- [Configuration](#configuration)
- [Log Levels](#log-levels)
- [Usage Patterns](#usage-patterns)
- [Structured Logging](#structured-logging)
- [Best Practices](#best-practices)
- [Environment-Specific Configuration](#environment-specific-configuration)
- [Log Rotation & Management](#log-rotation--management)
- [Troubleshooting](#troubleshooting)

---

## Overview

KAIRÓS Core utilizes a **hybrid logging system** built on top of the [`tracing`](https://docs.rs/tracing) ecosystem, providing:

- ✅ **Console logging** - Human-readable output for development
- ✅ **File logging** - Structured output for production and analysis
- ✅ **Environment-based configuration** - Different settings per environment
- ✅ **Automatic log rotation** - Daily/hourly rotation with retention policies
- ✅ **Structured fields** - Context-rich logging with key-value pairs

---

## System Architecture

### Components

```
┌──────────────────────────────────────────────────────┐
│                  Application Code                    │
│          (uses tracing macros: info!, error!)        │
└────────────────────┬─────────────────────────────────┘
                     │
                     ▼
┌──────────────────────────────────────────────────────┐
│            tracing_subscriber Registry               │
│                  (EnvFilter)                         │
└─────────┬────────────────────────────┬───────────────┘
          │                            │
          ▼                            ▼
┌─────────────────────┐    ┌──────────────────────────┐
│   Console Layer     │    │      File Layer          │
│   (Human-readable)  │    │   (Structured output)    │
│   → stdout          │    │   → Rolling file         │
└─────────────────────┘    └──────────────────────────┘
```

### Files Involved

| File | Purpose |
|------|---------|
| [`src/logging.rs`](file:///c:/Users/david/Documents/Trading-algorithms/apps/kairos-core/src/logging.rs) | Logging system initialization and configuration |
| [`src/config.rs`](file:///c:/Users/david/Documents/Trading-algorithms/apps/kairos-core/src/config.rs) | `LoggingSettings` struct definition |
| [`config/default.toml`](file:///c:/Users/david/Documents/Trading-algorithms/apps/kairos-core/config/default.toml) | Default logging configuration |
| [`config/production.toml`](file:///c:/Users/david/Documents/Trading-algorithms/apps/kairos-core/config/production.toml) | Production logging overrides |
| [`src/main.rs`](file:///c:/Users/david/Documents/Trading-algorithms/apps/kairos-core/src/main.rs) | Logging initialization entry point |

---

## Configuration

### Configuration Structure

```toml
[logging]
# Log level filter (RUST_LOG format)
rust_log = "info,kairos_core=debug"

# Backtrace setting (0=off, 1=on, full=complete)
rust_backtrace = "1"

# Enable file logging
enable_file_logging = true

# Log file settings
log_directory = "logs"
log_file_prefix = "kairos-core"

# Output formats: "human" or "json"
console_format = "human"
file_format = "json"

# Rotation: "hourly", "daily", "never"
rotation = "daily"

# Maximum log files to keep (0 = unlimited)
max_log_files = 30
```

### Configuration Options

| Field | Type | Description | Values |
|-------|------|-------------|--------|
| `rust_log` | String | Log level filter | `trace`, `debug`, `info`, `warn`, `error` |
| `rust_backtrace` | String | Backtrace verbosity | `0`, `1`, `full` |
| `enable_file_logging` | Boolean | Enable file output | `true`, `false` |
| `log_directory` | String | Directory for log files | Any valid path |
| `log_file_prefix` | String | Prefix for log filenames | Any string |
| `console_format` | String | Console output format | `human`, `json` |
| `file_format` | String | File output format | `human`, `json` |
| `rotation` | String | Rotation frequency | `hourly`, `daily`, `never` |
| `max_log_files` | Integer | Max files to retain | `0` = unlimited |

---

## Log Levels

KAIRÓS uses standard `tracing` log levels, from most to least verbose:

| Level | Macro | Use Case | Example |
|-------|-------|----------|---------|
| **TRACE** | `trace!()` | Very fine-grained debugging | Function entry/exit, loop iterations |
| **DEBUG** | `debug!()` | Development debugging | Variable states, internal flow |
| **INFO** | `info!()` | General information | Startup messages, configuration |
| **WARN** | `warn!()` | Warning conditions | Deprecated features, non-critical issues |
| **ERROR** | `error!()` | Error conditions | Failed operations, exceptions |

### Setting Log Levels

Use the `RUST_LOG` environment variable or `rust_log` in configuration:

```toml
# Global level: info, specific module: debug
rust_log = "info,kairos_core=debug"

# Multiple modules
rust_log = "warn,kairos_core=debug,tokio=info"

# Everything at trace level (verbose!)
rust_log = "trace"
```

---

## Usage Patterns

### Basic Logging

```rust
use tracing::{info, warn, error, debug, trace};

// Simple message
info!("Starting KAIRÓS Core");

// With variables
let port = 50051;
info!("gRPC server listening on port {}", port);

// Error logging
error!("Failed to connect to database: {}", err);
```

### Structured Logging with Fields

Add context using key-value pairs:

```rust
use tracing::info;

// Single field
info!(user_id = 123, "User logged in");

// Multiple fields
info!(
    exchange = "OKX",
    symbol = "BTC-USDT",
    price = 50000.0,
    "Market data received"
);

// With expressions
info!(
    latency_ms = start.elapsed().as_millis(),
    "Request completed"
);
```

**Output example:**

```
2026-01-10T20:00:00.000Z INFO exchange=OKX symbol=BTC-USDT price=50000.0: Market data received
```

### Logging with Spans

Create spans to group related operations:

```rust
use tracing::{info, info_span};

async fn process_order(order_id: u64) {
    let _span = info_span!("process_order", order_id).entered();
    
    info!("Validating order");
    // ... validation logic
    
    info!("Executing order");
    // ... execution logic
}
```

**Output:**

```
2026-01-10T20:00:00.000Z INFO process_order{order_id=123}: Validating order
2026-01-10T20:00:00.000Z INFO process_order{order_id=123}: Executing order
```

### Instrument Functions

Use the `#[instrument]` attribute for automatic span creation:

```rust
use tracing::instrument;

#[instrument]
async fn connect_to_exchange(url: &str) -> Result<Connection> {
    info!("Attempting connection");
    // Function arguments are automatically included as fields
    // ...
}

// With custom fields
#[instrument(skip(password), fields(user_id = user.id))]
async fn authenticate(username: &str, password: &str, user: &User) -> Result<Token> {
    // 'password' is skipped, 'user_id' is added
}
```

### Error Logging

```rust
use tracing::error;
use anyhow::Context;

match risky_operation().await {
    Ok(result) => info!("Operation succeeded"),
    Err(e) => {
        error!(
            error = %e,
            "Operation failed"
        );
    }
}

// With context chain
let result = operation()
    .await
    .context("Failed to connect to exchange")?;
```

---

## Structured Logging

### When to Use Structured Fields

✅ **DO** use structured fields for:

- Metrics and measurements (latency, count, size)
- Identifiers (user_id, order_id, symbol)
- Categorization (exchange, environment, type)
- Searchable data in log aggregation systems

❌ **DON'T** use structured fields for:

- Long text messages
- Sensitive data (passwords, API keys)
- High-cardinality data that changes every log

### Example: Feed Handler Logging

```rust
use tracing::{info, warn, error};

impl BinanceFeedHandler {
    async fn process_tick(&self, tick: MarketTick) {
        info!(
            exchange = "Binance",
            symbol = %tick.symbol,
            price = tick.price,
            volume = tick.volume,
            "Market tick received"
        );
    }
    
    async fn handle_websocket_error(&self, err: &Error) {
        error!(
            exchange = "Binance",
            error = %err,
            "WebSocket connection error"
        );
    }
    
    async fn reconnect(&self, attempt: u32) {
        warn!(
            exchange = "Binance",
            attempt,
            max_attempts = self.config.max_reconnect_attempts,
            "Attempting to reconnect"
        );
    }
}
```

---

## Best Practices

### 1. Choose the Right Log Level

| Scenario | Level |
|----------|-------|
| System starting up | `info!` |
| Configuration loaded | `info!` |
| Debug variable values | `debug!` |
| Function call tracing | `trace!` |
| Recoverable error | `warn!` |
| Critical error | `error!` |

### 2. Use Emojis for Visibility (Development)

```rust
info!("🚀 Starting KAIRÓS Trading Core");
info!("⚡ Initializing Tokio Runtime");
info!("🌍 Environment: {}", env);
error!("❌ Failed to connect to exchange");
warn!("⚠️  API rate limit approaching");
```

### 3. Include Context

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

### 4. Don't Log Sensitive Data

```rust
// ❌ NEVER DO THIS
info!("API Key: {}", api_key);
debug!("Password: {}", password);

// ✅ DO THIS
info!("API credentials configured");
debug!(key_length = api_key.len(), "API key loaded");
```

### 5. Use Appropriate Frequency

```rust
// ❌ BAD - Logs on every tick (thousands/second)
for tick in market_ticks {
    info!("Processing tick: {:?}", tick);
}

// ✅ GOOD - Log summary periodically
debug!(ticks_processed = count, "Batch processing complete");
```

### 6. Leverage Structured Fields

```rust
// ✅ Machine-parseable
info!(
    order_id = order.id,
    symbol = %order.symbol,
    side = %order.side,
    quantity = order.quantity,
    price = order.price,
    "Order placed"
);
```

---

## Environment-Specific Configuration

### Development

**File:** [`config/default.toml`](file:///c:/Users/david/Documents/Trading-algorithms/apps/kairos-core/config/default.toml)

```toml
[logging]
rust_log = "info,kairos_core=debug"
rust_backtrace = "1"
enable_file_logging = true
console_format = "human"      # ← Human-readable
file_format = "json"          # ← Structured for analysis
rotation = "daily"
max_log_files = 30
```

**Use Case:** Development and debugging

### Production

**File:** [`config/production.toml`](file:///c:/Users/david/Documents/Trading-algorithms/apps/kairos-core/config/production.toml)

```toml
[logging]
rust_log = "warn,kairos_core=info"  # ← Less verbose
rust_backtrace = "0"                # ← Disabled for performance
enable_file_logging = false         # ← Use Docker logs
console_format = "json"             # ← JSON for aggregators
file_format = "json"
max_log_files = 90
```

**Use Case:** Production deployment with log aggregation (ELK, Loki, etc.)

### Test

```toml
[logging]
rust_log = "error"               # ← Minimal output
enable_file_logging = false      # ← No file writes
console_format = "human"
```

**Use Case:** Automated testing

---

## Log Rotation & Management

### Automatic Rotation

Logs rotate based on the `rotation` setting:

- **`daily`**: New file each day (default)
- **`hourly`**: New file each hour
- **`never`**: Single file (not recommended)

### File Naming

```
logs/
├── kairos-core.log.2026-01-10
├── kairos-core.log.2026-01-09
├── kairos-core.log.2026-01-08
└── ...
```

### Retention Policy

Old logs are cleaned up automatically:

```toml
max_log_files = 30  # Keep last 30 files
```

Set to `0` for unlimited retention (not recommended).

### Manual Cleanup

```bash
# Remove logs older than 30 days
find logs/ -name "kairos-*.log.*" -mtime +30 -delete
```

---

## Troubleshooting

### Problem: No logs appearing

**Solution:**

1. Check `RUST_LOG` environment variable:

   ```bash
   echo $RUST_LOG
   ```

2. Verify logging initialization in `main.rs`
3. Check log level is not set too restrictive (e.g., `error` when using `info!`)

### Problem: Too many logs

**Solution:**

```toml
# Reduce verbosity
rust_log = "warn,kairos_core=info"

# Or for specific noisy modules
rust_log = "info,tokio=warn,hyper=warn"
```

### Problem: Logs not written to file

**Solution:**

1. Check `enable_file_logging = true`
2. Verify `log_directory` exists and is writable
3. Check disk space

### Problem: Performance impact

**Solution:**

1. Reduce log level in production
2. Disable file logging and use Docker log collection
3. Use `RUST_LOG` filtering to exclude noisy dependencies

### Problem: Need to analyze logs

**Tools:**

```bash
# Search logs
grep "ERROR" logs/kairos-core.log.2026-01-10

# Follow live logs
tail -f logs/kairos-core.log.2026-01-10

# Count errors
grep -c "ERROR" logs/*.log.*

# Parse JSON logs (if using JSON format)
jq '.fields.exchange' logs/kairos-core.log.2026-01-10
```

---

## Examples

### Example: Feed Handler

```rust
use tracing::{info, error, instrument};

pub struct OkxFeedHandler {
    config: ExchangeSettings,
}

impl OkxFeedHandler {
    #[instrument(skip(self))]
    pub async fn start(&self) -> Result<()> {
        info!("🚀 Starting OKX feed handler");
        
        match self.connect().await {
            Ok(_) => {
                info!(
                    exchange = "OKX",
                    endpoint = %self.config.okx_ws_public_url,
                    "WebSocket connected"
                );
            }
            Err(e) => {
                error!(
                    exchange = "OKX",
                    error = %e,
                    "Failed to connect"
                );
                return Err(e);
            }
        }
        
        Ok(())
    }
}
```

### Example: Order Execution

```rust
use tracing::{info, warn, error, instrument};

#[instrument(skip(self))]
pub async fn execute_order(&self, order: Order) -> Result<ExecutionResult> {
    info!(
        order_id = order.id,
        symbol = %order.symbol,
        side = %order.side,
        quantity = order.quantity,
        "Executing order"
    );
    
    let start = Instant::now();
    
    match self.send_order(&order).await {
        Ok(result) => {
            let latency = start.elapsed();
            info!(
                order_id = order.id,
                latency_ms = latency.as_millis(),
                fill_price = result.fill_price,
                "Order executed successfully"
            );
            Ok(result)
        }
        Err(e) if e.is_retryable() => {
            warn!(
                order_id = order.id,
                error = %e,
                "Order execution failed, retrying"
            );
            self.retry_order(order).await
        }
        Err(e) => {
            error!(
                order_id = order.id,
                error = %e,
                "Order execution failed permanently"
            );
            Err(e)
        }
    }
}
```

---

## Integration with Docker

When running in Docker, configure to output JSON to stdout:

```toml
[logging]
enable_file_logging = false  # Docker handles persistence
console_format = "json"      # Structured output
```

Docker Compose example:

```yaml
services:
  kairos-core:
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
```

---

## Summary

| Aspect | Development | Production |
|--------|-------------|------------|
| **Console** | Human-readable | JSON |
| **File** | Enabled (JSON) | Disabled |
| **Level** | `debug` | `info`/`warn` |
| **Backtrace** | Enabled | Disabled |
| **Retention** | 30 days | 90 days |

**Key Takeaways:**

- Use `info!` for important events
- Add structured fields for searchability
- Use spans/instrument for context
- Don't log sensitive data
- Adjust verbosity per environment
- Monitor log file growth

---

For more information, see:

- [tracing documentation](https://docs.rs/tracing)
- [ERROR_HANDLING.md](./ERROR_HANDLING.md)
- [CONFIG_ENVIRONMENT.md](./CONFIG_ENVIRONMENT.md)
