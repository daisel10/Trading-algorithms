# Agent Definitions Registry - KAIRÓS

> **Note:** Este archivo define el contexto general del proyecto KAIRÓS para los agentes de IA que colaboran en su desarrollo.

---

## 📌 Resumen del Proyecto

**KAIRÓS** es un sistema de **trading algorítmico de alta frecuencia (HFT)** diseñado con arquitectura híbrida para maximizar la velocidad de ejecución y minimizar la latencia. El proyecto sigue principios de **Domain-Driven Design (DDD)** y **Arquitectura Hexagonal** para separar la lógica de negocio de las implementaciones técnicas.

### Objetivo Principal

Crear un motor de trading de **baja latencia** capaz de ejecutar estrategias de arbitraje y triangulación en tiempo real, con soporte para múltiples exchanges de criptomonedas (Binance, OKX).

### Arquitectura General

El sistema se divide en tres capas:

1. **El Hierro** - Infraestructura física y sistema operativo optimizado
2. **El Monolito** - Motor de trading en Rust (kairos-core) con procesamiento en memoria
3. **Los Satélites** - Microservicios complementarios (API Java, Dashboard Angular)

---

## 🛠️ Stack Tecnológico

### Backend - Motor de Trading (Rust)

- **Lenguaje:** Rust (nightly 2024 edition)
- **Runtime:** Tokio 1.41 (async/await)
- **Networking:**
  - WebSocket: `tokio-tungstenite 0.24`
  - gRPC: `tonic 0.12` + `prost 0.13`
- **Bases de Datos:**
  - PostgreSQL/TimescaleDB: `sqlx 0.8`
  - DragonflyDB (Redis): `redis 0.25`
- **Serialización:** `serde 1.0` + `serde_json 1.0`
- **Observabilidad:** `tracing 0.1` + `tracing-subscriber 0.3`
- **Manejo de Errores:** `anyhow 1.0` + `thiserror 1.0`

### Backend - API Gateway (Java Spring Boot)

- **Lenguaje:** Java 21 (LTS)
- **Framework:** Spring Boot 3.2.1 (WebFlux - Reactivo)
- **Base de Datos:** R2DBC (PostgreSQL reactivo)
- **Redis:** Spring Data Redis Reactive
- **Build:** Maven 3.9+
- **Comunicación:** REST + WebSocket + gRPC Client

### Frontend - Dashboard (Angular)

- **Framework:** Angular 21.0.0
- **Lenguaje:** TypeScript ~5.0.0
- **Runtime:** Node.js 18+
- **State Management:** RxJS ~7.8.0
- **Comunicación:** HTTP REST + WebSocket

### Infraestructura

- **Bases de Datos:**
  - DragonflyDB (caché en memoria, compatible con Redis)
  - TimescaleDB (PostgreSQL 16 + extensión Timescale para series temporales)
- **Containerización:** Docker + Docker Compose
- **CI/CD:** GitHub Actions
- **Orchestration (Futuro):** Kubernetes 1.28+

---

## 📂 Estructura del Monorepo

```
kairos-monorepo/
├── apps/
│   ├── kairos-core/       # [MONOLITO] Motor de Trading (Rust)
│   ├── kairos-api/        # [SATÉLITE] API Gateway (Java Spring Boot)
│   └── kairos-web/        # [SATÉLITE] Dashboard (Angular)
├── libs/
│   ├── kairos-domain/     # Entidades compartidas (Rust)
│   └── kairos-proto/      # Contratos gRPC (.proto)
├── infrastructure/
│   ├── docker/            # Dockerfiles y Compose
│   ├── db/                # Scripts SQL y configuraciones
│   └── k8s/               # Manifiestos Kubernetes (futuro)
├── research/              # Notebooks y Python AI (futuro)
├── examples/              # Se encuentran plantilas que escifican como de debe hacer las cosas 
└── docs/                  # Documentación técnica
```
```

kairos-monorepo/
├── AGENTS.md               # Contexto global lee docs para enteder el contexto 
│
├── apps/
│   ├── kairos-core/           # [RUST]
│   │   ├── .ai/
│   │   ├── 00_CORE_MANIFEST.md    <-- [SIEMPRE ACTIVO] Reglas Generales del Proyecto Core (Rust, erores, Estilo)
│   │   └── skills/                <-- [BAJO DEMANDA] Reglas Específicas
│   │       ├── strategy_impl.md   # Cómo crear una estrategia de trading (Matemáticas, Risk Mgmt)
│   │       ├── db_migration.md    # Cómo alterar tablas en TimescaleDB (Migraciones, SQLx)
│   │       └── testing_guide.md   # Cómo escribir tests unitarios vs integration
│   │   ├── src/
│   │   ├── AGENTS.md              # Contexto global que lee .ai lo especifioc y lee las skills si es necesario
│   │   └── Cargo.toml
│
├── .ai/
│   ├── 00_CORE_MANIFEST.md    <-- [SIEMPRE ACTIVO] Reglas Generales del Proyecto para (cargo-makefile, docker-compose, documentation, )
│   │
│   └── skills/                <-- [BAJO DEMANDA] Reglas Específicas
│       └── Dockerfile.md   # Cómo escribir docker file siguiendo estas reglas o habilidades 
└── docs/                       # Documentación para Humanos
    ├── ARCHITECTURE.md         # Diagramas de alto nivel
    └── CONVENTIONS.md          # Guías de estilo detalladas



```
---

## 🎯 Componentes Principales

### 1. kairos-core (Rust)

Motor principal con 5 componentes críticos:

- **Feed Handler:** Ingesta de datos de exchanges vía WebSocket
- **Logger:** Persistencia asíncrona en DragonflyDB y TimescaleDB
- **Sprinters:** Estrategias de trading rápido (arbitraje/triangulación)
- **Gatekeeper:** Motor de gestión de riesgo
- **Sniper:** Ejecución de órdenes en exchanges

### 2. kairos-api (Java)

- Endpoints REST para consultas históricas
- WebSocket para streaming en tiempo real
- Cliente gRPC para comunicación con kairos-core

### 3. kairos-web (Angular)

- Dashboard con gráficos en tiempo real
- Configuración de estrategias
- Monitoreo de balances y órdenes

---

## 🔗 Comunicación entre Componentes

### Interna (Rust)

- **Broadcast Channel:** Feed Handler → Estrategias + Logger
- **MPSC Channel:** Estrategias → Motor de Riesgo → Ejecución

### Externa

- **Monolito → Satélites:** DragonflyDB Pub/Sub
- **Satélites → Monolito:** gRPC (tonic)
- **Dashboard → API:** REST + WebSocket

---

## 🔐 Configuración

- **Rust:** Archivos TOML (`config/default.toml`)
- **Java:** `application.yml`
- **Secrets:** Variables de entorno (`.env.example` como referencia)

---

## 📝 Convenciones de Desarrollo

### Versioning

- **SemVer 2.0.0** en todos los componentes
- Versión actual: `0.1.0` (pre-alpha)

### Testing

- **Rust:** `cargo test` + `cargo-tarpaulin` (coverage)
- **Java:** JUnit 5 + Reactor Test + JaCoCo
- **Angular:** Jasmine + Karma

### Linting

- **Rust:** `rustfmt` + `clippy`
- **Java:** Checkstyle (Maven plugin)
- **Angular:** ESLint + Prettier

---

## 🚀 Exchanges Soportados

- ✅ **Binance** (WebSocket API)
- ✅ **OKX** (WebSocket API)
- 🔜 Otros exchanges (futuro)

---
##  🧠 Context & Memory
---
**Última actualización:** 2026-01-14  
**Mantenido por:** KAIRÓS Development Team

