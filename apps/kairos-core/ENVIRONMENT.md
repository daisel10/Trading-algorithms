# Environment Configuration Guide

## Overview

The kairos-core application uses a **layered configuration system** with TOML files. Environments are: **development**, **test**, and **production**.

## Configuration Files

| File | Purpose | Tracked in Git |
| --- | --- | --- |
| `config/default.toml` | Base configuration for all environments | ✅ Yes |
| `config/development.toml` | Development environment overrides | ✅ Yes |
| `config/test.toml` | Test environment overrides | ✅ Yes |
| `config/production.toml` | Production environment overrides | ✅ Yes |
| `config/local.toml` | Local developer overrides | ❌ No (gitignored) |

## Configuration Loading Order (Priority)

Each layer overrides the previous:

1. **config/default.toml** - Base defaults
2. **config/{environment}.toml** - Environment-specific settings
3. **config/local.toml** - Personal local overrides (optional)
4. **Environment variables** - Highest priority

## Switching Environments

### Method 1: Environment Variable (Recommended)

Set the `APP_ENV` or `ENVIRONMENT` variable before running:

**Windows (PowerShell)**

```powershell
# Development (default)
cargo run

# Test
$env:APP_ENV="test"; cargo run

# Production
$env:APP_ENV="production"; cargo run
```

**Linux/MacOS (Bash)**

```bash
# Development (default)
cargo run

# Test
APP_ENV=test cargo run

# Production
APP_ENV=production cargo run
```

### Method 2: Local Configuration File

Create `config/local.toml` to override settings persistently:

```bash
# Copy the example template
Copy-Item config/local.toml.example config/local.toml

# Edit with your preferred settings
notepad config/local.toml
```

Example `config/local.toml`:

```toml
# Force a specific environment for local development
# (Normally not needed, just use APP_ENV variable)

[grpc]
port = 50053  # Use different port locally

[trading]
max_position_size = 50.0  # More conservative locally
```

## Environment Differences

### Development

- **Logging**: `RUST_LOG=debug,kairos_core=trace` - Verbose debugging
- **Exchange**: Demo/testnet URLs for safe testing
- **Trading**: Conservative limits (max position: $100, leverage: 1x)
- **Resources**: Lower thread counts (2 workers)
- **Features**: All debug features enabled

### Test

- **Logging**: `RUST_LOG=info,kairos_core=debug` - Moderate logging
- **Exchange**: Demo/testnet URLs
- **Trading**: Moderate limits (max position: $500, leverage: 2x)
- **Resources**: Moderate thread counts (3 workers)
- **Features**: Paper trading + debug endpoints enabled

### Production

- **Logging**: `RUST_LOG=warn,kairos_core=info` - Minimal for performance
- **Exchange**: Live production URLs - **REAL MONEY** ⚠️
- **Trading**: Full limits (max position: $1000, leverage: 3x)
- **Resources**: Full thread counts (4 workers)
- **Features**: All debug features DISABLED

## Validating Your Environment

When you start the application, check the logs:

```
🚀 Starting KAIRÓS Trading Core...
⚡ Initializing Tokio Runtime
🌍 Environment: production    <-- Verify this!
📋 Configuration loaded successfully
   gRPC Server: 0.0.0.0:50051
   Exchange: wss://ws.okx.com:8443/ws/v5/public
```

> ⚠️ **CRITICAL**: Always verify the environment shown matches your intent, especially before production!

## Overriding Specific Settings

### Via Environment Variables

Override ANY setting using environment variables:

**With KAIROS prefix (hierarchical structure)**:

```powershell
$env:KAIROS__GRPC__PORT="50052"
$env:KAIROS__TRADING__MAX_POSITION_SIZE="500.0"
cargo run
```

**Without prefix (flat structure)**:

```powershell
$env:GRPC_PORT="50052"
$env:MAX_POSITION_SIZE="500.0"
cargo run
```

### Via Local Config File

Edit `config/local.toml`:

```toml
[grpc]
port = 50052

[trading]
max_position_size = 500.0
```

## Docker Deployment

Set environment via Docker:

**docker-compose.yml**:

```yaml
services:
  kairos-core:
    environment:
      - APP_ENV=production
```

**Command line**:

```bash
docker run -e APP_ENV=production kairos-core
```

## Best Practices

1. ✅ **Default to Development**: If no `APP_ENV` is set, development is used
2. ✅ **Use config/local.toml**: For persistent local overrides (gitignored)
3. ✅ **Verify on startup**: Always check logs to confirm environment
4. ⚠️ **Production requires care**: Double-check before deploying
5. 🔒 **Secrets separately**: Never commit API keys, use env vars

## Accepted Environment Values

The following values are accepted (case-insensitive):

- **Development**: `dev`, `development`
- **Test**: `test`, `testing`  
- **Production**: `prod`, `production`

## Troubleshooting

**Q: How do I see which config file is being loaded?**  
A: The application logs show the environment on startup (`🌍 Environment: ...`)

**Q: My local overrides aren't working**  
A: Ensure `config/local.toml` exists and has valid TOML syntax

**Q: Can I have environment-specific secrets?**  
A: No, use environment variables for secrets, never commit them to TOML files

**Q: What if config/default.toml is missing?**  
A: The application will fail to start. This file is required.
