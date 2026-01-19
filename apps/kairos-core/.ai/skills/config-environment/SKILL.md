---
name: config-environment
description: >
  Manages TOML-based layered configuration system for kairos-core.
  Trigger: When configuring kairos-core, managing environments, setting up TOML files, using environment variables, troubleshooting configuration issues.
license: Apache-2.0
metadata:
  author: kairos-team
  version: "1.0"
---

## When to Use

- Setting up kairos-core configuration for different environments
- Understanding the layered configuration system (default → environment → local → env vars)
- Troubleshooting configuration loading issues
- Adding new configuration fields
- Managing environment-specific overrides
- Using environment variables to override TOML settings

## Critical Patterns

### Configuration Layer Priority

Configuration is loaded in this order (highest priority last):

```
1. config/default.toml           [Lowest Priority]
   ↓ (overridden by)
2. config/{environment}.toml     (development, test, production)
   ↓ (overridden by)
3. config/local.toml             (gitignored, developer-specific)
   ↓ (overridden by)
4. Environment Variables          [Highest Priority]
```

**Critical Rules**:

- `config/default.toml` is ALWAYS loaded first (base configuration)
- `APP_ENV` environment variable determines which environment file to load (default: "development")
- `config/local.toml` is for personal overrides (NEVER commit this)
- Environment variables always win over TOML files

### Environment Variable Formats

Two formats are supported:

```bash
# Flat format (no prefix)
GRPC_PORT=50052
MAX_POSITION_SIZE=500.0
RUST_LOG=trace

# Hierarchical format (with KAIROS prefix)
KAIROS__GRPC__PORT=50052
KAIROS__TRADING__MAX_POSITION_SIZE=500.0
```

### File Usage Guide

| Need | Use |
|------|-----|
| Base shared configuration | `config/default.toml` |
| Development-specific settings | `config/development.toml` |
| Testing-specific settings | `config/test.toml` |
| Production-specific settings | `config/production.toml` |
| Personal local overrides | `config/local.toml` (create this) |
| Secrets, API keys, passwords | `.env` or system env vars |
| Temporary override for one run | `$env:KEY="value"` (PowerShell) |

## Code Examples

### Basic TOML Configuration

```toml
# config/default.toml
rust_log = "info"

[grpc]
port = 50051
host = "0.0.0.0"

[trading]
max_position_size = 1000.0
max_leverage = 3.0
```

### Environment-Specific Override

```toml
# config/development.toml
rust_log = "debug,kairos_core=trace"  # More verbose

[grpc]
host = "127.0.0.1"  # Only localhost in dev
```

### Local Developer Override

```toml
# config/local.toml (create this file, it's gitignored)
[grpc]
port = 50099  # Custom port to avoid conflicts

[trading]
max_position_size = 10.0  # Conservative for testing
```

### Using Environment Variables

```powershell
# Temporary override
$env:GRPC_PORT="50052"
cargo run

# Using .env file (automatically loaded)
# .env
APP_ENV=production
GRPC_PORT=50052
DATABASE_PASSWORD=secret123
```

### Loading Configuration in Code

```rust
use kairos_core::config::Settings;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file first
    dotenvy::dotenv().ok();
    
    // Load configuration (follows priority order)
    let settings = Settings::new()?;
    
    // APP_ENV determines which config file is loaded
    println!("Environment: {}", std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()));
    println!("gRPC Port: {}", settings.grpc.port);
    
    Ok(())
}
```

## Commands

```bash
# Run with default environment (development)
cargo run

# Run with specific environment
$env:APP_ENV="production"
cargo run

# Run with environment variable override
$env:GRPC_PORT="50052"
cargo run

# Check current environment in logs
cargo run
# Look for: "🌍 Environment: development"

# Verify which config files exist
ls config/

# Create local override file
cp config/local.toml.example config/local.toml
# Then edit config/local.toml
```

## Configuration Flow

```
Application Starts
    ↓
dotenvy::dotenv() loads .env → Environment Variables
    ↓
Read APP_ENV (default: "development")
    ↓
Load config/default.toml (ALWAYS)
    ↓
Load config/{APP_ENV}.toml (e.g., config/development.toml)
    ↓
Load config/local.toml (if exists)
    ↓
Apply Environment Variable Overrides
    ↓
Deserialize to Settings struct
    ↓
Application Ready
```

## Common Issues

**Q: My config/local.toml changes aren't working**

- Verify the file exists and has valid TOML syntax
- Check for section headers like `[grpc]`
- Environment variables override TOML files

**Q: Environment variables from .env not applying**

- Ensure `.env` is in the project root (same directory as `Cargo.toml`)
- Verify `dotenvy::dotenv()` is called before `Settings::new()`
- Check `.env` file format: `KEY=value` (no spaces around `=`)

**Q: Don't know which value is being used**

- Check startup logs for "Environment: X" and "gRPC Server: host:port"
- Add temporary debug logging: `println!("Port: {}", settings.grpc.port);`

## Golden Rules

1. ❌ **NEVER** commit `.env` or `config/local.toml`
2. ✅ **ALWAYS** use `config/default.toml` for base values
3. ✅ **ONLY** override what changes in environment files
4. ✅ **PREFER** environment variables for secrets
5. ✅ **VERIFY** logs on startup to confirm correct environment

## Resources

- **Source Documentation**: [apps/kairos-core/docs/CONFIG_ENVIRONMENT.md](file:///c:/Users/david/Documents/Trading-algorithms/apps/kairos-core/docs/CONFIG_ENVIRONMENT.md)
- **Configuration File**: [src/config.rs](file:///c:/Users/david/Documents/Trading-algorithms/apps/kairos-core/src/config.rs)
- **Default Config**: [config/default.toml](file:///c:/Users/david/Documents/Trading-algorithms/apps/kairos-core/config/default.toml)
