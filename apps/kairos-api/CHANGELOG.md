# Changelog - kairos-api

All notable changes to **kairos-api** (Java Spring Boot API Gateway) will be documented in this file.

---

## [0.1.0] - 2026-01-06

### Added

- ✨ Initial implementation of API Gateway
- 🌐 REST API endpoints for market data, orders, and balances
- 📡 WebSocket streaming for real-time market data
- 🔌 R2DBC reactive database access to TimescaleDB
- 💾 Redis Reactive integration with DragonflyDB
- 🔄 Spring WebFlux reactive architecture
- 📝 Request/response validation
- 🐳 Docker support with multi-stage build

### Fixed

- 🐛 Fixed WebSocket connection leak on client disconnect
- 🐛 Resolved Redis connection pool exhaustion

### Security

- 🔒 Added input validation for all endpoints
- 🔒 Implemented CORS configuration

---

**Maintained by:** KAIRÓS Team  
**Last updated:** 2026-01-06
