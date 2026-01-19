# kairos-core: Configuración de Agentes

## 📘 Filosofía de Uso

**Este documento define el contexto del motor de trading KAIRÓS (Rust) para agentes de IA.**
Consulta las habilidades específicas en `.ai/skills/` según la tarea que vayas a realizar.

---

## 🛠 Habilidades Disponibles (Skillset)

Habilidades específicas de kairos-core (Motor de trading en Rust).

### 🌐 Habilidades Tecnológicas (Rust Stack)

*Patrones técnicos específicos del motor de trading.*

| Habilidad | Descripción | Archivo de Referencia |
| :--- | :--- | :--- |
| `config-environment` | Sistema de configuración TOML por capas, variables de entorno | [skills/config-environment/SKILL.md](skills/config-environment/SKILL.md) |
| `logging` | Sistema híbrido de logging con tracing (console + file, JSON) | [skills/logging/SKILL.md](skills/logging/SKILL.md) |
| `error-handling` | Manejo de errores con thiserror/anyhow por capas | [skills/error-handling/SKILL.md](skills/error-handling/SKILL.md) |
| `testing` | Guía completa de testing (unit, integration, benchmarks) | [skills/testing/SKILL.md](skills/testing/SKILL.md) |
| `grpc-service` | API gRPC y traits internos (Strategy, FeedHandler, etc.) | [skills/grpc-service/SKILL.md](skills/grpc-service/SKILL.md) |
| `binance-realtime` | Integración WebSocket con Binance para datos en tiempo real | [skills/binance-realtime/SKILL.md](skills/binance-realtime/SKILL.md) |

---

## 🤖 Disparadores Automáticos (Auto-invoke)

**REGLA DE ORO:** Antes de realizar una acción, carga la habilidad correspondiente.

### 🏗 Desarrollo & Arquitectura

| Acción (Lo que vas a hacer) | Habilidad Requerida (Lo que debes leer antes) |
| :--- | :--- |
| Configurar entornos (dev/prod/test) | `config-environment` |
| Añadir logging o tracing | `logging` |
| Crear nuevos tipos de error | `error-handling` |
| Implementar nueva estrategia de trading | `grpc-service` |
| Integrar nuevo exchange | `binance-realtime` (como referencia) |
| Implementar FeedHandler trait | `grpc-service` |
| Implementar RiskValidator trait | `grpc-service` |

### 🧪 Calidad & Testing

| Acción | Habilidad Requerida |
| :--- | :--- |
| Escribir tests unitarios | `testing` |
| Escribir tests de integración | `testing` |
| Crear benchmarks de performance | `testing` |
| Medir code coverage | `testing` |

### 🔧 Debugging & Troubleshooting

| Acción | Habilidad Requerida |
| :--- | :--- |
| Depurar configuración que no carga | `config-environment` |
| Analizar logs JSON | `logging` |
| Investigar error chain | `error-handling` |

---

## 🗺 Visión General del Proyecto

**kairos-core** es el **motor crítico** del sistema KAIRÓS, escrito en Rust con enfoque en baja latencia y alta concurrencia.

### Propósito

Motor de trading algorítmico que:

- Ingesta datos en tiempo real de exchanges (Binance, OKX) vía WebSocket
- Ejecuta estrategias de arbitraje y triangulación
- Gestiona riesgos y limita exposición
- Ejecuta órdenes con latencia mínima
- Persiste datos históricos de manera asíncrona

### Arquitectura Hexagonal (Ports & Adapters)

```
domain/         → Lógica de negocio pura (estrategias, riesgo, entidades)
application/    → Orquestación y casos de uso
adapters/
├── inbound/   → Feed handlers (WebSocket), gRPC server
└── outbound/  → Persistence (DB), execution (exchange APIs)
```

### Stack Tecnológico

| Categoría | Tecnología | Versión |
| :--- | :--- | :--- |
| **Lenguaje** | Rust | nightly 2024 |
| **Async Runtime** | Tokio | 1.41 |
| **WebSocket** | tokio-tungstenite | 0.24 |
| **gRPC** | tonic + prost | 0.12 + 0.13 |
| **HTTP Server** | axum | 0.8 |
| **DB Client** | sqlx | 0.8 |
| **Redis Client** | redis | 0.25 |
| **Logging** | tracing + tracing-subscriber | 0.1 + 0.3 |
| **Errors** | anyhow + thiserror | 1.0 + 1.0 |

### Estructura de Directorios

```
apps/kairos-core/
├── Cargo.toml
├── config/
│   ├── default.toml          # Configuración base
│   ├── development.toml      # Override para dev
│   ├── production.toml       # Override para prod
│   └── local.toml.example    # Template para overrides locales
├── src/
│   ├── main.rs               # Entry point
│   ├── config.rs             # Sistema de configuración
│   ├── logging.rs            # Setup de logging
│   ├── domain/               # Lógica de negocio pura
│   │   ├── strategies/       # Algoritmos de trading
│   │   ├── risk/             # Motor de riesgo
│   │   └── entities/         # Structs de dominio
│   ├── application/          # Casos de uso
│   │   ├── state.rs          # Estado global
│   │   └── engine.rs         # Coordinador
│   └── adapters/
│       ├── inbound/          # Entrada de datos
│       │   ├── feed_handler/ # WebSocket (Binance/OKX)
│       │   └── grpc_server/  # Servidor gRPC
│       └── outbound/         # Salida de datos
│           ├── persistence/  # SQLx (TimescaleDB)
│           └── execution/    # HTTP/WS exchanges
└── tests/                    # Tests de integración
```

---

## ⚡ Flujo de Trabajo (Workflow)

### Desarrollo Local

```bash
# Cargar variables de entorno
cp .env.example .env
# Editar .env con tus valores

# Compilar
cargo build

# Ejecutar
cargo run

#Ejecutar con logging específico
RUST_LOG=debug,kairos_core=trace cargo run
```

### Testing

```bash
# Tests unitarios
cargo test --lib

# Tests de integración
cargo test --test integration_tests

# Tests con output
cargo test -- --nocapture

# Benchmarks
cargo bench

# Coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

### Linting & Formatting

```bash
# Format código
cargo fmt

# Linter
cargo clippy -- -D warnings

# Ambos
cargo make lint
```

---

## 🔗 Flujo de Datos Interno

```
┌─────────────────────┐
│   Feed Handler      │ (WebSocket → Binance/OKX)
│   (Inbound)         │
└──────────┬──────────┘
           │ Broadcast<MarketTick>
           ↓
┌─────────────────────┐
│   Strategies        │ (Bellman-Ford, Arbitrage)
│   (Domain)          │
└──────────┬──────────┘
           │ MPSC<Order>
           ↓
┌─────────────────────┐
│   Risk Engine       │ (Gatekeeper)
│   (Domain)          │
└──────────┬──────────┘
           │ Validated Orders
           ↓
┌─────────────────────┐
│   Execution         │ (Sniper → Exchanges)
│   (Outbound)        │
└──────────┬──────────┘
           │
        Parallel
           ├─→ Logger (Async persistence to DB/Redis)
           └─→ gRPC Broadcast (to kairos-api)
```

---

## 📝 Convenciones de Código

### Estilo

- **Rust idiomático**: Sigue las guías de Rust 2021 edition
- **No `.unwrap()`**: Usa `?` o `.context()` en producción
- **Async preferido**: Usa `tokio::spawn` para concurrencia
- **Channels sobre Mutex**: Prefer message passing over shared state

### Nomenclatura

- **Structs**: `PascalCase`
- **Functions**: `snake_case`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case`

### Documentación

```rust
/// Summary line
///
/// # Examples
///
/// ```
/// let result = function_name();
/// ```
///
/// # Errors
///
/// Returns `Error` if...
pub fn function_name() -> Result<T> { ... }
```

---

## 📋 Guía de Commits

**Formato**: `<type>(<scope>): <description>`

**Tipos**:

- `feat`: Nueva funcionalidad
- `fix`: Corrección de bug
- `perf`: Mejora de performance
- `refactor`: Refactorización sin cambiar funcionalidad
- `test`: Añadir o modificar tests
- `docs`: Cambios en documentación
- `chore`: Tareas de mantenimiento

**Ejemplos**:

- `feat(strategies): add triangular arbitrage`
- `fix(feed): resolve WebSocket reconnection issue`
- `perf(execution): optimize order submission latency`

---

**Última actualización:** 2026-01-19  
**Mantenido por:** kairos-core Development Team
