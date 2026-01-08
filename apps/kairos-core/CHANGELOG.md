# Changelog - kairos-core

All notable changes to the **kairos-core** (Rust Trading Engine) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Planned

- [ ] Implement OKX WebSocket feed handler
- [ ] Add market making strategy
- [ ] Implement position tracking
- [ ] Add metrics export (Prometheus)
- [ ] Implement order fill reconciliation

---

## [0.1.0] - 2026-01-06

### Added

- ✨ Initial implementation of trading engine core
- 🔌 Binance WebSocket feed handler with reconnection logic
- 📊 Market data normalization to `MarketTick` struct
- 🧠 Triangular arbitrage strategy with Bellman-Ford algorithm
- 🛡️ Basic risk validator with order size limits
- 🚀 gRPC server for external command handling
- 📡 Internal communication via Tokio channels (Broadcast & MPSC)
- 💾 Persistence layer for TimescaleDB (OHLCV storage)
- 💾 Redis/DragonflyDB integration for market data caching
- 🔄 Asynchronous order execution adapter for Binance
- 📝 Comprehensive logging with `tracing` crate
- 🐳 Docker support with multi-stage build

### Fixed

- 🐛 Fixed memory leak in WebSocket reconnection handler
- 🐛 Fixed race condition in atomic balance updates
- 🐛 Corrected timestamp parsing for Binance ticker events

### Changed

- ♻️ Refactored domain logic to follow hexagonal architecture
- ⚡ Optimized market tick processing pipeline (40% latency reduction)
- 📦 Updated Tokio to version 1.41
- 📦 Updated Tonic to version 0.12

### Security

- 🔒 Added input validation for gRPC requests
- 🔒 Implemented rate limiting on external API calls

---

## [0.0.1] - 2025-12-26

### Added

- 🎉 Project scaffolding with Cargo workspace
- 📁 Directory structure following hexagonal architecture
- 🧪 Basic unit test setup
- 📚 Initial documentation (README, ADRs)

---

## Types of Changes

- **Added** - for new features
- **Changed** - for changes in existing functionality
- **Deprecated** - for soon-to-be removed features
- **Removed** - for now removed features
- **Fixed** - for any bug fixes
- **Security** - in case of vulnerabilities

---

## Migration Guides

### Upgrading to 0.1.0

No breaking changes (initial release).

---

**Note:** Pre-alpha versions (0.x.x) may have breaking changes between minor versions without notice.

---

**Maintained by:** KAIRÓS Team  
**Last updated:** 2026-01-06
