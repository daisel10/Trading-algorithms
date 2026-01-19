---
name: error-handling
description: >
  Hybrid error handling strategy using thiserror for libraries and anyhow for applications.
  Trigger: When creating error types, handling errors in different layers, using thiserror, using anyhow, implementing error conversion, pattern matching errors.
license: Apache-2.0
metadata:
  author: kairos-team
  version: "1.0"
---

## When to Use

- Creating custom error types for modules/adapters
- Deciding between thiserror and anyhow
- Implementing error conversion with `#[from]` and `#[source]`
- Pattern matching on specific error types for recovery logic
- Converting library errors to application errors
- Maintaining error chains for debugging

## Critical Patterns

### Error Type Decision Tree

**Ask: Who will consume this error?**

```
Will YOUR CODE consume it (to make decisions)?
    ↓
  Use thiserror (Custom Error Types)
    Examples: FeedError, PersistenceError, DomainError
    
Will a HUMAN consume it (reading logs)?
    ↓
  Use anyhow (Context-rich errors)
    Examples: main.rs, engine.rs entry points
```

### Error Handling by Layer

```
┌─────────────────────────────────────────────┐
│  main.rs, engine.rs                         │
│  → anyhow::Result                           │
│  (Errors for humans)                        │
└─────────────────────────────────────────────┘
                    │
                    ↓ converts with .map_err()
┌─────────────────────────────────────────────┐
│  Adapters (Inbound/Outbound)                │
│  → FeedResult, PersistenceResult, etc.      │
│  (Errors for code)                          │
└─────────────────────────────────────────────┘
                    │
                    ↓ uses pattern matching
┌─────────────────────────────────────────────┐
│  Domain (Business Logic)                    │
│  → DomainResult                             │
│  (Errors for decisions)                     │
└─────────────────────────────────────────────┘
```

### #[from] vs #[source]

**Use `#[from]`** for automatic conversion (only ONE per type):

```rust
#[derive(Error, Debug)]
pub enum FeedError {
    #[error("WebSocket error")]
    WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),
    
    #[error("JSON error")]
    JsonError(#[from] serde_json::Error),
}

// Usage: automatic conversion with ?
async fn connect() -> FeedResult<()> {
    let ws = connect_async(url).await?;  // ← Auto-converts
    Ok(())
}
```

**Use `#[source]`** for multiple variants of same type or adding context:

```rust
#[derive(Error, Debug)]
pub enum FeedError {
    #[error("Failed to connect to {url}")]
    ConnectionFailed {
        url: String,
        #[source]
        source: tokio_tungstenite::tungstenite::Error,
    },
    
    #[error("Failed to parse {field}")]
    ParseError {
        field: String,
        #[source]
        source: std::num::ParseFloatError,
    },
}

// Usage: manual conversion with .map_err()
async fn connect(url: &str) -> FeedResult<()> {
    connect_async(url)
        .await
        .map_err(|source| FeedError::ConnectionFailed {
            url: url.to_string(),
            source,
        })?;
    Ok(())
}
```

## Code Examples

### Creating Error Types

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
```

### Config Layer Errors

```rust
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

// Usage
pub fn new() -> ConfigResult<Settings> {
    let config = Config::builder()
        .add_source(File::with_name("config/default"))
        .build()?;  // ← Auto-converts with #[from]
    
    config.try_deserialize()
        .map_err(ConfigError::LoadFailed)
}
```

### Domain Layer Errors

```rust
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
```

### Pattern Matching for Recovery

```rust
match risk_engine.validate_order(&order) {
    Ok(_) => {
        execute_order(order).await?;
    },
    Err(DomainError::InsufficientBalance { required, available }) => {
        warn!(
            required, available,
            "Insufficient balance for order"
        );
        notify_user_insufficient_funds(required, available).await?;
    },
    Err(DomainError::RiskLimitExceeded(msg)) => {
        error!(reason = %msg, "Risk limit exceeded");
        halt_trading().await?;
    },
    Err(e) => {
        error!(error = %e, "Order validation failed");
        return Err(e.into());
    }
}
```

### Converting to anyhow in main.rs

```rust
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Convert specific errors to anyhow with context
    let settings = Settings::new()
        .map_err(|e| anyhow::anyhow!("Failed to load configuration: {}", e))?;
    
    let feed = BinanceFeedHandler::start()
        .await
        .map_err(|e| anyhow::anyhow!("Feed handler failed: {}", e))?;
    
    Ok(())
}
```

## Anti-Patterns to Avoid

### ❌ Don't Convert to String

```rust
// ❌ BAD - Loses error chain
#[derive(Error, Debug)]
pub enum FeedError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),  // ← Lost original error
}

let ws = connect_async(url)
    .await
    .map_err(|e| FeedError::ConnectionFailed(e.to_string()))?;

// ✅ GOOD - Preserves error chain
#[derive(Error, Debug)]
pub enum FeedError {
    #[error("Connection failed")]
    ConnectionFailed(#[source] tokio_tungstenite::tungstenite::Error),
}

let ws = connect_async(url).await?;  // Auto-converts
```

### ❌ Don't Use anyhow in Libraries

```rust
// ❌ BAD - Callers can't pattern match
pub async fn connect() -> anyhow::Result<Connection> {
    // ...
}

// ✅ GOOD - Callers can make decisions
pub async fn connect() -> FeedResult<Connection> {
    // ...
}
```

### ❌ Don't Create Generic Errors

```rust
// ❌ BAD - No useful information
#[derive(Error, Debug)]
pub enum MyError {
    #[error("Error")]
    GenericError,
}

// ✅ GOOD - Descriptive with context
#[derive(Error, Debug)]
pub enum MyError {
    #[error("Failed to parse price from '{input}': expected decimal number")]
    InvalidPrice { input: String },
    
    #[error("Connection timeout after {seconds}s to {host}")]
    Timeout { host: String, seconds: u64 },
}
```

### ❌ Don't Use Multiple #[from] for Same Type

```rust
// ❌ BAD - Compiler error: conflicting implementations
#[derive(Error, Debug)]
pub enum MyError {
    #[error("Connection error")]
    ConnectionError(#[from] std::io::Error),
    
    #[error("Read error")]
    ReadError(#[from] std::io::Error),  // ← ERROR!
}

// ✅ GOOD - Use #[source] instead
#[derive(Error, Debug)]
pub enum MyError {
    #[error("Connection error")]
    ConnectionError(#[source] std::io::Error),
    
    #[error("Read error")]
    ReadError(#[source] std::io::Error),
}

// Manual conversion
.map_err(MyError::ConnectionError)?
.map_err(MyError::ReadError)?
```

## Debugging Errors

### Accessing Source Errors

```rust
use std::error::Error;

fn debug_error(err: &FeedError) {
    // Main error message
    println!("Error: {}", err);
    
    // Original source error
    if let Some(source) = err.source() {
        println!("Caused by: {}", source);
    }
    
    // Full error chain
    let mut current = err.source();
    while let Some(cause) = current {
        println!("  → {}", cause);
        current = cause.source();
    }
}
```

### Backtrace with anyhow

```rust
use anyhow::Context;

fn main() -> anyhow::Result<()> {
    // Enable backtraces
    std::env::set_var("RUST_BACKTRACE", "1");
    
    something_that_fails()
        .context("Failed during initialization")?;
    
    Ok(())
}
```

## Review Checklist

When creating or reviewing error handling:

- [ ] Uses `thiserror` in adapters/domain?
- [ ] Uses `anyhow` only in main/engine?
- [ ] Errors have `#[from]` or `#[source]` instead of `.to_string()`?
- [ ] Error messages are descriptive?
- [ ] Error chain is preserved?
- [ ] Has type alias `type XxxResult<T> = Result<T, XxxError>`?
- [ ] Code can pattern match when needed?
- [ ] Errors have sufficient context for debugging?

## Resources

- **Source Documentation**: [ERROR_HANDLING.md](file:///c:/Users/david/Documents/Trading-algorithms/apps/kairos-core/docs/ERROR_HANDLING.md)
- **thiserror documentation**: <https://docs.rs/thiserror/>
- **anyhow documentation**: <https://docs.rs/anyhow/>
- **Rust Error Handling**: <https://doc.rust-lang.org/book/ch09-00-error-handling.html>
