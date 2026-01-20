# KAIRÓS: Directrices del Monorepo & Configuración de Agentes

## 📘 Filosofía de Uso

**Este documento es la fuente de verdad para el flujo de trabajo en KAIRÓS.**
Su objetivo es reducir la carga cognitiva: no memorices comandos, consulta las tablas de "Habilidades" según el contexto en el que te encuentres.

---

## 🛠 Habilidades Disponibles (Skillset)

Estas son las herramientas permitidas y los patrones de diseño aprobados para KAIRÓS.

### 🌐 Contextos del Monorepo

*Contextos generales que aplican a todo el proyecto.*

| Habilidad | Descripción | Archivo de Referencia |
| :--- | :--- | :--- |
| `kairos-monorepo` | Arquitectura general, estructura del monorepo, comunicación entre componentes | [.ai/00_MAIN.md](.ai/00_MAIN.md) |

### 🎯 Contextos por Aplicación

*Contextos específicos de cada componente del sistema.*

| Habilidad | Descripción | Archivo de Referencia |
| :--- | :--- | :--- |
| `kairos-core` | Motor de trading (Rust) - Arquitectura hexagonal, estrategias, risk management | [apps/kairos-core/.ai/00_MAIN.md](apps/kairos-core/.ai/00_MAIN.md) |
| `kairos-api` | API Gateway (Java Spring Boot) - REST, WebSocket, gRPC client | [apps/kairos-api/.ai/00_MAIN.md](apps/kairos-api/.ai/00_MAIN.md) |
| `kairos-web` | Dashboard (Angular) - UI, RxJS, WebSocket streaming | [apps/kairos-web/.ai/00_MAIN.md](apps/kairos-web/.ai/00_MAIN.md) |
| `kairos-domain` | Entidades de dominio (Rust) - Arquitectura hexagonal, estrategias, risk management | [apps/kairos-domain/.ai/00_MAIN.md](apps/kairos-domain/.ai/00_MAIN.md) |
| `kairos-proto` | Contratos gRPC compartidos | [apps/kairos-proto/.ai/00_MAIN.md](apps/kairos-proto/.ai/00_MAIN.md) |
---

## 🤖 Disparadores Automáticos (Auto-invoke)

**REGLA DE ORO:** Antes de realizar una acción de la columna izquierda, el agente o desarrollador DEBE cargar el contexto de la habilidad a la derecha.

### 🏗 Desarrollo & Arquitectura

| Acción (Lo que vas a hacer) | Habilidad Requerida (Lo que debes leer antes) |
| :--- | :--- |
| Modificar estructura del monorepo | `kairos-monorepo` |
| Crear/modificar estrategias de trading | `kairos-core` |
| Trabajar con WebSocket exchanges (Binance, OKX) | `kairos-core` |
| Implementar endpoints REST/gRPC | `kairos-api` |
| Crear componentes del dashboard | `kairos-web` |
| Modificar configuración TOML (Rust) | `kairos-core` |
| Modificar configuration properties (Java) | `kairos-api` |

### 🧪 Calidad & Testing

| Acción | Habilidad Requerida |
| :--- | :--- |
| Escribir tests para motor de trading | `kairos-core` |
| Escribir tests para API | `kairos-api` |
| Escribir tests para frontend | `kairos-web` |

### 🚀 Despliegue & Mantenimiento (DevOps)

| Acción | Habilidad Requerida |
| :--- | :--- |
| Modificar Docker Compose | `kairos-monorepo` |
| Configurar CI/CD | `kairos-monorepo` |
| Actualizar documentación general | `kairos-monorepo` |

---

## 🗺 Visión General del Proyecto

**KAIRÓS** es un sistema de trading algorítmico de alta frecuencia (HFT) diseñado con arquitectura híbrida Rust/Java/Angular para maximizar velocidad de ejecución y minimizar latencia.

### Arquitectura de Tres Capas

1. **El Hierro** - Infraestructura física y SO optimizado
2. **El Monolito** - Motor de trading (Rust) con procesamiento en memoria
3. **Los Satélites** - Microservicios complementarios (API Java, Dashboard Angular)

### Estructura de Directorios

*Ubicación de los componentes clave para facilitar la navegación rápida.*

| Directorio | Propósito | Tecnologías Clave |
| :--- | :--- | :--- |
| `/apps/kairos-core` | Motor de trading en Rust | Tokio, WebSocket, gRPC, TimescaleDB |
| `/apps/kairos-api` | API Gateway en Java | Spring Boot WebFlux, R2DBC, Redis |
| `/apps/kairos-web` | Dashboard en Angular | Angular 21, RxJS, WebSocket |
| `/libs/kairos-proto` | Contratos gRPC compartidos | Protocol Buffers |
| `/libs/kairos-domain` | Entidades de dominio Rust | Rust crates |
| `/infrastructure` | Docker, DB configs, scripts | Docker Compose, SQL |
| `/docs` | Documentación técnica | Markdown |
| `/examples` | Templates y patrones de referencia | Markdown |

---

## ⚡ Flujo de Trabajo (Workflow)

### Instalación Inicial

```bash
# Clonar repositorio
git clone <repository-url>
cd Trading-algorithms

# Instalar dependencias Rust
cargo build

# Instalar dependencias Java (en apps/kairos-api)
cd apps/kairos-api
mvn clean install

# Instalar dependencias Angular (en apps/kairos-web)
cd apps/kairos-web
npm install
```

### Desarrollo Local

```bash
# Levantar infraestructura (DB, Redis)
docker compose up -d

# Ejecutar motor de trading
cd apps/kairos-core
cargo run

# Ejecutar API Gateway
cd apps/kairos-api
mvn spring-boot:run

# Ejecutar Dashboard
cd apps/kairos-web
npm start
```

### Testing

```bash
# Tests Rust
cd apps/kairos-core
cargo test

# Tests Java
cd apps/kairos-api
mvn test

# Tests Angular
cd apps/kairos-web
npm test
```

### Build de Producción

```bash
# Build completo con cargo-make
cargo make build-all
```

---

## 📋 Guía de Commits

Usamos **Conventional Commits** para mantener un historial claro:

- `feat`: Nueva funcionalidad
- `fix`: Corrección de bug
- `chore`: Tareas de mantenimiento
- `docs`: Cambios en documentación
- `test`: Añadir o modificar tests
- `refactor`: Refactorización de código

**Ejemplo**: `feat(core): add triangular arbitrage strategy`

---

**Última actualización:** 2026-01-19  
**Mantenido por:** KAIRÓS Development Team
