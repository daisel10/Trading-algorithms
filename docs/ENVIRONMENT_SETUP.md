# Environment Configuration Guide

This document explains how to configure environment variables for the KAIRÓS Trading System.

## 📁 File Structure

The project uses a **three-level environment configuration** approach:

```
Trading-algorithms/
├── .env.example              # Global configuration template
├── .env                      # Global configuration (create from .example)
├── apps/
│   ├── kairos-api/
│   │   ├── .env.example      # Java API specific template
│   │   └── .env              # Java API specific config (create from .example)
│   ├── kairos-core/
│   │   ├── .env.example      # Rust Core specific template
│   │   └── .env              # Rust Core specific config (create from .example)
│   └── kairos-web/
│       ├── .env.example      # Angular Web specific template
│       └── .env              # Angular Web specific config (create from .example)
```

## 🚀 Quick Start

### 1. Copy the Templates

```bash
# From the project root directory
cp .env.example .env
cp apps/kairos-api/.env.example apps/kairos-api/.env
cp apps/kairos-core/.env.example apps/kairos-core/.env
cp apps/kairos-web/.env.example apps/kairos-web/.env
```

### 2. Update Sensitive Values

Edit each `.env` file and replace placeholder values:

- **Global `.env`**: Update database passwords, JWT secrets, API keys
- **Service-specific `.env`**: Update service-specific runtime configuration

### 3. Run with Docker Compose

```bash
docker compose -f infrastructure/docker-compose.yml up --build
```

Docker Compose automatically loads both global and service-specific `.env` files.

## 📝 Configuration Levels

### Level 1: Global Configuration (`.env`)

Shared variables used by **all services**:
- Database connection strings (DragonflyDB, TimescaleDB)
- Exchange API keys (OKX, Binance)
- Security settings (JWT secrets)
- Feature flags

### Level 2: Service-Specific Configuration

Each service has its own `.env` with service-specific settings:

- **`apps/kairos-api/.env`**: Java/Spring Boot settings, WebSocket config, CORS
- **`apps/kairos-core/.env`**: Rust logging, gRPC server, trading engine config
- **`apps/kairos-web/.env`**: Angular build settings, API endpoints

### Level 3: Docker Compose Overrides

Docker Compose can override variables in the `environment` section when needed for container networking.

## 🔐 Security Best Practices

### ✅ DO:
- Keep `.env` files in `.gitignore` (already configured)
- Use strong passwords and secrets in production
- Rotate API keys regularly
- Use different credentials for dev/staging/production
- Share `.env.example` files in version control

### ❌ DON'T:
- Commit `.env` files with real credentials
- Share `.env` files in Slack/email
- Use production credentials in development
- Hardcode secrets in source code

## 🔄 Variable Priority

When the same variable is defined in multiple places, Docker Compose uses this priority:

1. `environment` section in `docker-compose.yml` (highest priority)
2. Service-specific `.env` (e.g., `apps/kairos-api/.env`)
3. Global `.env` (lowest priority)

This allows you to:
- Define defaults globally
- Override per service as needed
- Override at runtime via Docker Compose

## 🛠️ Development vs Production

### Development (Docker)
Uses default values from `.env` files, suitable for local development.

### Development (Local - No Docker)
Update connection strings to use `localhost`:
```bash
DRAGONFLY_HOST=localhost
TIMESCALE_HOST=localhost
CORE_GRPC_HOST=localhost
```

### Production
- Use **Docker Secrets** or **Kubernetes Secrets**
- Never use default passwords
- Set `ENVIRONMENT=production`
- Enable TLS/SSL for all connections
- Use managed services for databases

## 📋 Required Variables Checklist

Before running the system, ensure you've configured:

- [ ] Database passwords (`POSTGRES_PASSWORD`)
- [ ] JWT secret key (`JWT_SECRET`)
- [ ] Exchange API credentials (`OKX_API_KEY`, etc.)
- [ ] Environment flag (`ENVIRONMENT`)
- [ ] gRPC ports match between services
- [ ] Database URLs are correct

## 🐛 Troubleshooting

### "Connection refused" errors
- Check that database host/port match your environment
- For local dev without Docker, use `localhost`
- For Docker, use service names (`dragonfly`, `timescale`)

### "Missing environment variable" errors
- Ensure you've created `.env` files from `.example` templates
- Check that Docker Compose is loading the files correctly
- Verify paths in `env_file` sections of `docker-compose.yml`

### Variables not being loaded
- Restart Docker Compose completely: `docker compose down && docker compose up`
- Check for syntax errors in `.env` files (no spaces around `=`)
- Ensure `.env` files are in the correct directories

## 📚 Reference

For more details on specific configuration options, see:
- Global config: [`.env.example`](../.env.example)
- Rust Core: [`apps/kairos-core/.env.example`](../apps/kairos-core/.env.example)
- Java API: [`apps/kairos-api/.env.example`](../apps/kairos-api/.env.example)
- Angular Web: [`apps/kairos-web/.env.example`](../apps/kairos-web/.env.example)
