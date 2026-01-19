# KAIRÓS Monorepo: Configuración de Agentes

## 📘 Filosofía de Uso

**Este documento define el contexto del monorepo KAIRÓS para agentes de IA.**
Antes de trabajar en un componente específico, consulta su contexto individual en `apps/*/. ai/00_MAIN.md`.

---

## 🛠 Habilidades Disponibles (Skillset)

Habilidades de nivel monorepo que aplican a todos los componentes.

### 🌐 Habilidades Tecnológicas del Monorepo

*Patrones técnicos comunes a todo el proyecto.*

| Habilidad | Descripción | Archivo de Referencia |
| :--- | :--- | :--- |
| `docker-compose` | Orquestación de servicios, networking, volúmenes | `infrastructure/docker/` |
| `cargo-make` | Build automation, tasks del workspace Rust | `Makefile.toml` |
| `github-actions` | CI/CD pipelines, workflows de deployment | `.github/workflows/` |
| `monorepo-structure` | Organización de apps, libs, docs | `PROJECT_STRUCTURE.md` |

### 🧠 Habilidades de Contexto Específico

*Referencias a contextos de componentes individuales.*

| Habilidad | Descripción | Archivo de Referencia |
| :--- | :--- | :--- |
| `kairos-core-context` | Motor de trading Rust - Arquitectura hexagonal | [apps/kairos-core/.ai/00_MAIN.md](apps/kairos-core/.ai/00_MAIN.md) |
| `kairos-api-context` | API Gateway Java - Spring Boot patterns | [apps/kairos-api/.ai/00_MAIN.md](apps/kairos-api/.ai/00_MAIN.md) |
| `kairos-web-context` | Dashboard Angular - UI components | [apps/kairos-web/.ai/00_MAIN.md](apps/kairos-web/.ai/00_MAIN.md) |

---

## 🤖 Disparadores Automáticos (Auto-invoke)

**REGLA DE ORO:** Antes de realizar una acción, carga el contexto apropiado.

### 🏗 Desarrollo & Arquitectura del Monorepo

| Acción (Lo que vas a hacer) | Habilidad Requerida (Lo que debes leer antes) |
| :--- | :--- |
| Modificar Docker Compose | `docker-compose` |
| Añadir nueva tarea cargo-make | `cargo-make` |
| Modificar CI/CD pipeline | `github-actions` |
| Reorganizar estructura del monorepo | `monorepo-structure` |
| Trabajar en motor de trading | `kairos-core-context` |
| Trabajar en API Gateway | `kairos-api-context` |
| Trabajar en Dashboard | `kairos-web-context` |

### 🚀 Deployment & Infrastructure

| Acción | Habilidad Requerida |
| :--- | :--- |
| Levantar ambiente local | `docker-compose` |
| Configurar nueva base de datos | `docker-compose` + `monorepo-structure` |
| Deploy a producción | `github-actions` |

---

## 🗺 Visión General del Proyecto

**KAIRÓS** es un sistema de trading algorítmico HFT con arquitectura híbrida Rust/Java/Angular.

### Principios de Diseño

1. **El Hierro** - Infraestructura optimizada para baja latencia
2. **El Monolito** - Motor Rust con procesamiento en memoria RAM
3. **Los Satélites** - Servicios complementarios para UI y APIs externas

### Arquitectura de Tres Capas

```
┌─────────────────────────────────────────────┐
│         DASHBOARD (Angular)                 │
│         kairos-web                          │
└──────────────┬──────────────────────────────┘
               │ REST + WebSocket
               ↓
┌─────────────────────────────────────────────┐
│         API GATEWAY (Java Spring Boot)      │
│         kairos-api                          │
└──────────────┬──────────────────────────────┘
               │ gRPC + DragonflyDB Pub/Sub
               ↓
┌─────────────────────────────────────────────┐
│      MOTOR DE TRADING (Rust + Tokio)        │
│      kairos-core                            │
│  ┌──────────────────────────────────────┐  │
│  │ Feed Handler → Strategies → Sniper   │  │
│  └──────────────────────────────────────┘  │
└──────────────┬──────────────────────────────┘
               │
               ↓
┌─────────────────────────────────────────────┐
│  INFRAESTRUCTURA                            │
│  - TimescaleDB (series temporales)          │
│  - DragonflyDB (caché Redis-compatible)     │
│  - Binance/OKX WebSocket APIs               │
└─────────────────────────────────────────────┘
```

### Estructura del Monorepo

| Directorio | Propósito | Tecnologías |
| :--- | :--- | :--- |
| `/apps/kairos-core` | Motor principal Rust | Tokio, WebSocket, gRPC Server |
| `/apps/kairos-api` | API Gateway Java | Spring Boot WebFlux, R2DBC |
| `/apps/kairos-web` | Dashboard Angular | Angular 21, RxJS |
| `/libs/kairos-proto` | Contratos gRPC | Protocol Buffers |
| `/libs/kairos-domain` | Entidades compartidas | Rust crates |
| `/infrastructure` | Docker, DB, scripts | Docker Compose, SQL |
| `/docs` | Documentación técnica | Markdown |
| `/examples` | Templates de referencia | Markdown |

---

## ⚡ Flujo de Trabajo (Workflow)

### Instalación Inicial

```bash
# Clonar repositorio
git clone <repository-url>
cd Trading-algorithms

# Instalar Rust toolchain
rustup default nightly

# Build completo del workspace
cargo build --workspace
```

### Desarrollo Local

```bash
# Levantar infraestructura (DB + Redis)
docker compose up -d

# Correr motor de trading
cd apps/kairos-core
cargo run

# Correr API (en otra terminal)
cd apps/kairos-api
mvn spring-boot:run

# Correr Dashboard (en otra terminal)
cd apps/kairos-web
npm start
```

### Testing

```bash
# Tests de todo el workspace Rust
cargo test --workspace

# Tests con coverage
cargo make test-coverage

# Tests API Java
cd apps/kairos-api
mvn test

# Tests Dashboard
cd apps/kairos-web  
npm test
```

### Build de Producción

```bash
# Build completo con cargo-make
cargo make build-all

# Build Docker images
docker compose build
```

---

## 📋 Stack Tecnológico

### Backend - Motor (Rust)

- **Lenguaje**: Rust nightly 2024
- **Runtime**: Tokio 1.41
- **Networking**: tokio-tungstenite, tonic, prost
- **DB**: sqlx (PostgreSQL), redis
- **Observability**: tracing, tracing-subscriber

### Backend - API (Java)

- **Lenguaje**: Java 21 LTS
- **Framework**: Spring Boot 3.2.1 WebFlux
- **DB**: R2DBC PostgreSQL, Spring Data Redis Reactive
- **Build**: Maven 3.9+

### Frontend (Angular)

- **Framework**: Angular 21.0.0
- **Lenguaje**: TypeScript ~5.0.0
- **State**: RxJS ~7.8.0

### Infraestructura

- **Bases de Datos**:
  - DragonflyDB (Redis-compatible)
  - TimescaleDB (PostgreSQL 16 + extensión)
- **Containerización**: Docker + Docker Compose
- **CI/CD**: GitHub Actions

---

## 🔗 Comunicación entre Componentes

### Interna (Rust)

- **Broadcast Channel**: Feed Handler → Estrategias + Logger
- **MPSC Channel**: Estrategias → Risk Engine → Execution

### Externa

- **Core → API**: DragonflyDB Pub/Sub + gRPC
- **API → Web**: REST + WebSocket
- **Core → Exchanges**: WebSocket (Binance, OKX)

---

## 📝 Guía de Commits

Usamos **Conventional Commits**:

- `feat`: Nueva funcionalidad
- `fix`: Corrección de bug
- `chore`: Mantenimiento
- `docs`: Documentación
- `test`: Tests
- `refactor`: Refactorización

**Formato**: `<type>(<scope>): <description>`

**Ejemplos**:

- `feat(core): add triangular arbitrage strategy`
- `fix(api): resolve WebSocket reconnection issue`
- `docs(monorepo): update deployment guide`

---

**Última actualización:** 2026-01-19  
**Mantenido por:** KAIRÓS Development Team
