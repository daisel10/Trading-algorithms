# PDR – Proyecto de Software de Arbitraje Intra‑Broker (Spot ⇄ Perpetuos)

## 1. Visión

Desarrollar una plataforma **modular y extensible** que ejecute  diferentes estrategias de arbitraje, se va a inciar con ejecuciones de un mismo broker (OKX como primera implementación) intercambiando posiciones **spot** y **perpetuos** para capturar ineficiencias de precio con la menor latencia posible y riesgo controlado.

## 2. Objetivos

1. **MVP** capaz de identificar spreads spot ⇄ perpetuos y abrir/cerrar posiciones automáticamente.
2. Arquitectura **Clean Architecture + Hexagonal + DDD Táctico** que permita añadir nuevas estrategias (inter‑broker, triangular, estadístico) como módulos independientes.
3. Garantizar **tolerancia a fallos**; el bot debe retomar operaciones tras reinicios sin perder contexto.

## 3. Alcance – Iteración 0

- Soporte exclusivo para **OKX** (REST & WebSocket) en mercado **USDT‑margined**.
- Monitoreo de un conjunto configurable de pares (BTC‑USDT, ETH‑USDT…).
- Ejecución simultánea de órdenes spot y perp usando margen aislado.
- Registro histórico completo de precios, funding y operaciones.

## 4. Requisitos Funcionales

| ID    | Descripción                                                 |
| ----- | ----------------------------------------------------------- |
| RF‑01 | Detectar oportunidades ≥ **tasa mínima configurable**       |
| RF‑02 | Ejecutar entrada spot + perp en < 100 ms                    |
| RF‑03 | Ajustar posiciones según cambios en funding / spread        |
| RF‑04 | Persistir operaciones, PnL, fees y funding en *TimescaleDB* |
| RF‑05 | Notificar vía e‑mail/Telegram los eventos clave             |

## 5. Requisitos No Funcionales

| Categoría          | Meta                                                  |
| ------------------ | ----------------------------------------------------- |
| **Rendimiento**    | Latencia de decisión < 50 ms; throughput ≥ 1 k msg/s  |
| **Concurrencia**   | Uso intensivo de *async/await* (Tokio multithread)    |
| **Escalabilidad**  | Horizontal mediante *Docker/K8s*                      |
| **Resiliencia**    | Reintentos exponenciales; snapshot de estado en disco |
| **Observabilidad** | *tracing*, *Prometheus*, *Grafana*                    |
| **Seguridad**      | Secrets en \*HashiCorp Vault/*dotenv‑safe*; TLS 1.3   |

## 6. Arquitectura de Carpetas (Workspace)

```text
proyecto-arbitraje/
├── Cargo.toml                        # Archivo de configuración del workspace (lista los miembros/crates)
├── dominio/                          # Capa de Dominio común (lógica de negocio central compartida)
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs                    # Punto de acceso de la crate (define módulos públicos)
├── strategies/                      # Estrategias de arbitraje (subcrates de la capa de Aplicación)
│   ├── intra/                        # Estrategia de arbitraje *intra-broker* (dentro de un mismo broker)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── aplicacion/           # Lógica de aplicación (casos de uso, comandos) de arbitraje intra-broker
│   │       ├── dominio/             # (Opcional) Extensiones de dominio específicas de intra-broker
│   │       │   └── ...              # Ej: entidades o VOs particulares solo para esta estrategia
│   │       └── lib.rs
│   └── inter/                        # Estrategia de arbitraje *inter-broker* (entre brokers diferentes)
│       ├── Cargo.toml
│       └── src/
│           ├── aplicacion/           # Lógica de aplicación de arbitraje inter-broker
│           ├── dominio/             # (Opcional) Extensiones de dominio para inter-broker
│           │   └── ...              # Ej: reglas o entidades propias de esta variante de estrategia
│           └── lib.rs
├── infrastructure/                  # Adaptadores y conectores externos (capa de Infraestructura)
│   ├── okx/                          # Conector para broker OKX (implementa contratos del dominio)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs                # Implementaciones de APIs/servicios para OKX según interfaces de dominio
│   ├── ibkr/                         # Conector para broker IBKR (Interactive Brokers)
│   │   ├── Cargo.toml
│   │   └── src/ ...                  # Lógica de acceso a IBKR implementando los traits del dominio
│   ├── simulador/                    # Conector simulador (broker simulado para pruebas)
│   │   ├── Cargo.toml
│   │   └── src/ ...                  # Implementación simulada de interfaces (útil en tests/desarrollo)
│   └── ... (otros adaptadores)       # Ejemplos: base de datos, servicios externos adicionales, etc.
├── interfaces/                       # Interfaces de entrada (capa de Interfaces/Presentación)
│   └── zombot_core/                          # Aplicación Bot (interfaz de línea de comando para el usuario)
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs               # Punto de entrada de la aplicación Bot (función main)
│           └── bot_args.rs           # Definición de comandos/argumentos Bot (p.ej. usando clap) y selección de estrategia
└── settings/                    # Capa de Configuración (composición de dependencias e inyección)
    ├── Cargo.toml
    └── src/
        ├── lib.rs                    # Inicialización centralizada de la aplicación (composición de estrategias y adaptadores)
        └── config.toml               # (Ejemplo) Archivo de configuración de la aplicación (brokers, estrategia activa, etc.)


```

## 6.1. Arquitectura de Carpetas (dominio)

```text
dominio/                               # Crate raíz (dominio puro de la app)
├── Cargo.toml                         # Manifest de Rust: deps, features, workspace
└── src
    ├── lib.rs                         # Punto de entrada del crate; re-exporta módulos
    ├── prelude.rs                     # Atajo de imports comunes (VOs, entidades, errores…)
    ├── errors/                        # Jerarquía de errores de dominio
    ├── value_objects/                 # Objetos de Valor (inmutables, sin identidad)
    ├── entities/                      # Entidades con identidad propia
    ├── aggregates/                    # Aggregates Roots y consistencia transaccional
    ├── services/                      # Servicios del dominio (lógica pura, sin IO)
    ├── specifications/                # Reglas reutilizables (Specification Pattern)
    ├── policies/                      # Domain Policies (reglas que cruzan entidades/VOs)
    ├── events/                        # Eventos de dominio
    ├── repositories/                  # Interfaces de persistencia (puertos salientes)
    ├── contracts/                     # Interfaces hacia otros BCs/gateways
    └── tests/                         # Pruebas de integración / BDD del dominio
```

## 7. Stack Tecnológico

| Capa               | Tecnología                                        | Motivo                            |
| ------------------ | ------------------------------------------------- | --------------------------------- |
| Lenguaje principal | **Rust 1.78**                                     | Seguridad de memoria, performance |
| Runtime async      | **Tokio** (multi‑scheduler)                       | Concurrencia masiva               |
| Persistencia TS    | **TimescaleDB 2.x** (extensión PostgreSQL)        | Consultas time‑series eficientes  |
| ORM / DB Layer     | **sqlx async** y **Diesel** para consultas sync | Performance + type‑safety         |
| Cache & Queue      | **Redis 7** (opcional)                            | Pub/Sub, caching, locks           |
| Brokers SDK        | **OKX REST & WS** (custom crate)                  | Ejecución low‑latency             |
| Testing            | **cargo nextest**, **proptest**                   | Concurrency‑aware tests           |
| Observabilidad     | **tracing**, **opentelemetry**, **Prometheus**    | Métricas y tracing                |
| Contenedores       | **Docker + Docker Compose**                       | Deploy reproducible               |
| CI/CD              | **GitHub Actions**                                | Build, test & release pipeline    |

## 8. Concurrencia y Paralelismo

- **Modelo asíncrono**: todas las I/O (WebSocket, DB, FS) se realizan con *async/await*.
- **Actor pattern** (crates: `tokio::sync::mpsc`, `xactor`) para aislar responsabilidades.
- **Task pinning**: operaciones críticas (matching engine) en *core‑affine* tasks.
- **Paralelismo de CPU**: cálculo de indicadores usando *rayon* cuando sea CPU‑bound.

## 9. Persistencia de Datos

- **Esquema TimescaleDB**
  - `prices(symbol, ts, price, source)`
  - `funding_rates(symbol, period, rate)`
  - `trades(id, ts_open, ts_close, side, qty, entry_price, exit_price, fee, pnl)`
- Retención: datos crudos 30 d; agregados 1 año.

## 10. Roadmap Alto Nivel

| Fase                     | Entregables                            | ETA   |
| ------------------------ | -------------------------------------- | ----- |
| **0 – Setup**            | Repo, CI, Docker, scaffold dominio     | 1 sem |
| **1 – MVP Intra**        | Detección + ejecución spot‑perp        | 4 sem |
| **2 – Hardening**        | Back‑test, stress‑test, observabilidad | 3 sem |
| **3 – Multi‑estrategia** | Inter‑broker, triangular               | 6 sem |
| **4 – UI/API**           | Dashboard y REST API                   | 4 sem |

## 11. Riesgos y Mitigaciones

| Riesgo              | Impacto | Plan                                      |
| ------------------- | ------- | ----------------------------------------- |
| Volatilidad extrema | Alto    | Límites de pérdida; orquestador de riesgo |
| Cambios API broker  | Medio   | Abstracción via traits + versión HUCHA    |
| Latencia ISP        | Medio   | VPS cercano (AWS HK) + redundancia        |

## 12. Glosario

- **Spot**: Compra/venta inmediata de activos.
- **Perpetuo**: Contrato de futuros sin vencimiento con *funding*.
- **Funding rate**: Tarifa periódica para anclar precio perpetuo al spot.
- **Spread**: Diferencia de precio entre dos mercados.

