# Tech Stack - KAIRÓS Trading System

Este documento lista todos los lenguajes, frameworks, librerías y versiones exactas utilizadas en el sistema KAIRÓS.

---

## 🦀 Backend - kairos-core (Rust)

### Lenguaje

- **Rust:** `nightly` (2024 edition)
  - Requerido para features experimentales
  - Instalación: `rustup install nightly && rustup default nightly`

### Build System

- **Cargo:** `1.75+`
- **Workspace:** Monorepo con 3 crates

### Runtime

- **tokio:** `1.41`
  - Features: `["full"]`
  - Async runtime para I/O non-blocking

### Networking

- **tokio-tungstenite:** `0.24`
  - WebSocket client para exchanges
- **tonic:** `0.12`
  - gRPC server
- **prost:** `0.13`
  - Protocol Buffers serialization

### Database

- **sqlx:** `0.8`
  - Features: `["runtime-tokio-native-tls", "postgres", "chrono", "uuid"]`
  - Async PostgreSQL driver
- **redis:** `0.25`
  - Features: `["tokio-comp", "connection-manager"]`
  - Cliente para DragonflyDB

### Serialization

- **serde:** `1.0`
  - Features: `["derive"]`
- **serde_json:** `1.0`

### Web Framework (gRPC Server)

- **axum:** `0.8` (si se usa para health checks)
- **tower:** `0.4`
- **tower-http:** `0.5`
  - Features: `["cors", "trace"]`

### Observability

- **tracing:** `0.1`
- **tracing-subscriber:** `0.3`
  - Features: `["env-filter"]`

### Error Handling

- **anyhow:** `1.0`
- **thiserror:** `1.0`

### Utilities

- **chrono:** `0.4`
  - Features: `["serde"]`
- **uuid:** `1.10`
  - Features: `["serde", "v4"]`
- **config:** `0.14`

### Build Dependencies

- **tonic-build:** `0.12`
  - Compila `.proto` files en build time

---

## ☕ Backend - kairos-api (Java)

### Lenguaje

- **Java:** `21` (LTS)
  - OpenJDK 21 o Eclipse Temurin 21

### Build System

- **Maven:** `3.9+`

### Framework

- **Spring Boot:** `3.2.1`
  - `spring-boot-starter-webflux` (Reactive)
  - `spring-boot-starter-data-r2dbc` (Reactive DB)
  - `spring-boot-starter-data-redis-reactive`
  - `spring-boot-starter-validation`
  - `spring-boot-starter-test`

### Database Drivers

- **r2dbc-postgresql:** `1.0.2` (runtime)
  - Reactive PostgreSQL driver

### Redis

- **spring-data-redis-reactive:** (incluido en Spring Boot)
  - Cliente reactivo para DragonflyDB

### Serialization

- **jackson-databind:** (incluido en Spring Boot)
- **jackson-datatype-jsr310:** Para soporte de `java.time`

### Code Generation

- **lombok:** `1.18.30`
  - Reduce boilerplate (`@Data`, `@Builder`, etc.)

### Testing

- **spring-boot-starter-test:** `3.2.1`
  - JUnit 5, Mockito, AssertJ
- **reactor-test:** `3.6.x`
  - Testing para código reactivo

### gRPC Client (Potencial - si se añade)

- **grpc-spring-boot-starter:** `3.0.0` (no incluido aún en pom.xml)

---

## 🅰️ Frontend - kairos-web (Angular)

### Lenguaje

- **TypeScript:** `~5.0.0`
- **Node.js:** `18+`
- **npm:** `9+`

### Framework

- **Angular:** `21.0.0`
  - `@angular/core`
  - `@angular/common`
  - `@angular/router`
  - `@angular/forms`

### Build Tools

- **Angular CLI:** `21.0.0`
- **TypeScript Compiler:** `~5.0.0`

### UI Components (Potencial - si se añaden)

- **Angular Material:** `21.0.0` (si se usa)
- **PrimeNG:** (alternativa)

### State Management (Potencial)

- **@ngrx/store:** (si se necesita Redux pattern)
- **RxJS:** `~7.8.0` (incluido con Angular)

### HTTP Client

- **@angular/common/http:** (incluido)
  - Para llamadas REST a kairos-api

### WebSocket

- **rxjs/webSocket:** (incluido en RxJS)
  - Para streaming de market data

### Testing

- **Jasmine:** `~5.1.0`
- **Karma:** `~6.4.0`
- **@angular/cli/testing:** (e2e con Protractor o Cypress)

### Linting/Formatting

- **ESLint:** (configurado vía Angular CLI)
- **Prettier:** `~3.0.0` (si se configura)

---

## 🗄️ Databases

### DragonflyDB

- **Versión:** `latest` (Docker image)
- **Imagen Docker:** `docker.dragonflydb.io/dragonflydb/dragonfly:latest`
- **Puerto:** `6379`
- **Uso:** Caché en memoria, Pub/Sub para market data
- **Protocolo:** Redis-compatible

### TimescaleDB

- **Versión:** PostgreSQL `16` + Timescale extension
- **Imagen Docker:** `timescale/timescaledb:latest-pg16`
- **Puerto:** `5432`
- **Uso:** Almacenamiento OHLCV, trades históricos
- **Extensiones:** `timescaledb`, `pg_stat_statements`

---

## 🐳 Infrastructure & DevOps

### Containerization

- **Docker:** `24.0+`
- **Docker Compose:** `2.20+`

### Orchestration (Futuro)

- **Kubernetes:** `1.28+` (manifests en `infrastructure/k8s/`)

### CI/CD (Configurado en `.github/workflows/`)

- **GitHub Actions:**
  - Rust: `actions-rs/toolchain@v1`
  - Java: `actions/setup-java@v3`
  - Node: `actions/setup-node@v3`

---

## 🔧 Development Tools

### Rust

- **rustfmt:** (bundled with Rust)
- **clippy:** (bundled with Rust)
- **cargo-watch:** `cargo install cargo-watch`
- **cargo-audit:** `cargo install cargo-audit`
- **tarpaulin:** `cargo install cargo-tarpaulin` (code coverage)

### Java

- **Maven Wrapper:** `./mvnw` (incluido en repo)
- **Checkstyle:** (Maven plugin)
- **JaCoCo:** (Maven plugin para coverage)

### Angular

- **Angular CLI:** `npm install -g @angular/cli@21`

### Protocol Buffers

- **protoc:** `24.0+`
  - Windows: `choco install protoc`
  - Linux: `apt install protobuf-compiler`
  - macOS: `brew install protobuf`

---

## 🌐 External APIs

### Cryptocurrency Exchanges

- **Binance WebSocket API:**
  - URL: `wss://stream.binance.com:9443/ws`
  - Documentación: [Binance API Docs](https://binance-docs.github.io/apidocs/)
  
- **OKX WebSocket API:**
  - URL: `wss://ws.okx.com:8443/ws/v5/public`
  - Documentación: [OKX API Docs](https://www.okx.com/docs-v5/en/)

---

## 📦 Shared Libraries (Rust Crates)

### kairos-domain

- **Ubicación:** `libs/kairos-domain/`
- **Versión:** `0.1.0`
- **Dependencias:**
  - `serde` (serialization)
  - `chrono` (timestamps)
  - `uuid` (IDs)

### kairos-proto

- **Ubicación:** `libs/kairos-proto/`
- **Versión:** `0.1.0`
- **Dependencias:**
  - `tonic` (gRPC)
  - `prost` (Protobuf)
- **Build Script:** `build.rs` (compila `.proto` files)

---

## 🔐 Security & Auth (Futuro)

- **JWT:** (para autenticación en kairos-api)
- **TLS/SSL:** (para conexiones en producción)

---

## 📊 Monitoring & Logging (Futuro)

### Potenciales Herramientas

- **Prometheus:** Métricas
- **Grafana:** Dashboards
- **Jaeger:** Distributed tracing
- **ELK Stack:** Logs centralizados

---

## 🔄 Versioning

### Semantic Versioning

Todos los componentes siguen **SemVer 2.0.0**:
- **MAJOR:** Cambios incompatibles de API
- **MINOR:** Nueva funcionalidad compatible
- **PATCH:** Bug fixes compatibles

### Versiones Actuales

- **kairos-core:** `0.1.0` (pre-alpha)
- **kairos-api:** `0.1.0` (pre-alpha)
- **kairos-web:** `0.1.0` (pre-alpha)
- **kairos-domain:** `0.1.0`
- **kairos-proto:** `0.1.0`

---

## 📝 Notes

### Rust Nightly

Usamos Rust Nightly para aprovechar features en desarrollo. Si una feature se estabiliza, migraremos a stable.

### Dependency Updates

- **Rust:** Actualizar `Cargo.toml` manualmente y ejecutar `cargo update`
- **Java:** Actualizar `pom.xml` y ejecutar `mvn dependency:tree`
- **Angular:** Ejecutar `ng update @angular/cli @angular/core`

### Breaking Changes

Cualquier actualización que rompa compatibilidad debe documentarse en:
1. `CHANGELOG.md` del módulo afectado
2. Nuevo ADR si es una decisión arquitectónica
3. Migration guide si afecta a usuarios

---

## 🔗 Referencias

- [Rust Crates.io](https://crates.io/)
- [Maven Central](https://central.sonatype.com/)
- [npm Registry](https://www.npmjs.com/)
- [Docker Hub](https://hub.docker.com/)

---

**Última actualización:** 2026-01-06  
**Mantenido por:** KAIRÓS Team
