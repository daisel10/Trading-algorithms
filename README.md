# KAIRÓS: High-Performance Algorithmic Trading Core

![Status](https://img.shields.io/badge/Status-Pre--Alpha-critical)
![Core](https://img.shields.io/badge/Core-Rust-orange)
![API](https://img.shields.io/badge/API-Java%20Spring%20Boot-green)
![Dashboard](https://img.shields.io/badge/Dashboard-Angular-red)
![Architecture](https://img.shields.io/badge/Architecture-Monorepo-blue)
![Infra](https://img.shields.io/badge/Infra-DragonflyDB%20%7C%20Timescale-green)



## 📖 Visión del Proyecto

Kairós es un motor de trading de **baja latencia** y arquitectura híbrida. Diseñado bajo principios de **DDD (Domain-Driven Design)** y **Arquitectura Hexagonal**, su objetivo es desacoplar la inteligencia (Estrategias/IA) de la ejecución crítica (Riesgo/Conectividad/extracción de datos/persistencia).

Actualmente, el proyecto opera como un **Monorepo**, centralizando el desarrollo del Núcleo (Rust), la API (Java Spring Boot), y el Dashboard (Angular).

---
## Brokers Soportados
- OKX
- Binance

## 📚 Estado Actual del Desarrollo
- ✅ **Monolito en Rust** (kairos-core) - Motor de trading con gRPC
- ✅ **API en Java Spring Boot** (kairos-api) - REST + WebSocket gateway
- 🚧 **Dashboard en Angular** (kairos-web) - Interfaz web en desarrollo



## 🏗️ Arquitectura del Sistema

La plataforma se divide en tres capas fundamentales: **El Hierro**, **El Monolito** y **Los Satélites**.

### 1. El Hierro (Hardware & OS)
La base física. Para HFT y arbitraje, priorizamos la velocidad de reloj sobre el paralelismo masivo.
* **CPU:** Alta frecuencia (GHz) es prioridad.
* **OS:** Linux (Alpine/Ubuntu Server) con optimizaciones de Kernel para *Low Latency*.

### 2. El Monolito (Rust Core)
El cerebro de la operación. Todo ocurre en la misma memoria RAM para evitar latencias de red interna. Funciona como una fábrica con 5 órganos vitales:

**A. Ingesta de Datos (The Feed Handler)**

- Misión: Escuchar a Binance/Kraken y traducir.
- Protocolo Externo: WSS (Secure WebSockets). Usarás la librería tokio-tungstenite.
- Acción: Recibe un JSON, lo deserializa con Serde a tu struct estandarizado (ej. MarketTick).
- Comunicación Interna (Salida):
    - Usa un canal Broadcast (tokio::sync::broadcast).
    - Por qué: Porque este dato le interesa a todos: a la estrategia rápida, a la base de datos y a los satélites.

**B. Persistencia (The Logger)**

- Misión: Guardar la historia sin frenar el trading.
- Entrada: Se suscribe al canal Broadcast de la Ingesta.
- Acción: Acumula datos en un buffer y hace escrituras por lotes (Batch Insert).
- Protocolo Salida: TCP hacia DragonflyDB (datos calientes) y PostgreSQL/Timescale (historial) usando SQLx.
- Nota: Corre en un hilo separado de baja prioridad.

**C. Estrategias Rápidas (The Sprinters)**

- Misión: Triangulación y Arbitraje.
- Entrada: Se suscribe al canal Broadcast de la Ingesta.
- Lógica:
    - Actualiza su Grafo en memoria local.
    - Ejecuta algoritmo de ciclo negativo (Bellman-Ford optimizado).
- Comunicación Interna (Salida):
    - Si encuentra dinero, envía una OrdenInterna a través de un canal MPSC (Multiple Producer, Single Consumer).
    - Por qué MPSC: Porque puede haber varias estrategias gritando "¡Compra!", pero solo hay un ejecutor.
    

**D. Motor de Riesgo (The Gatekeeper) - CRÍTICO**

- Misión: Evitar la ruina.
- Entrada: Lee del canal MPSC (las órdenes que vienen de las estrategias).
- Lógica:
    - Consulta el "Saldo Atómico" (una variable AtomicF64 en memoria, no en base de datos).
    - Verifica: if orden.monto < saldo_disponible && orden.riesgo < limite_diario.
- Salida: Si aprueba, pasa la orden al siguiente canal. Si rechaza, loguea el error y descarta.

**E. Ejecución (The Sniper)**

- Misión: Disparar al mercado.
- Entrada: Recibe la orden aprobada del Motor de Riesgo.
- Protocolo Externo: HTTP REST (para órdenes simples) o un WebSocket Privado de trading (si el broker lo soporta, es más rápido).
- Gestión de Estado: Apenas recibe confirmación del Broker ("Comprado"), actualiza el AtomicF64 de saldo para que el Motor de Riesgo sepa cuánto dinero queda.

### 3. Los Satélites (Ecosistema Externo)
Microservicios que añaden inteligencia y visualización sin tocar la memoria crítica de Rust.
* **Dashboard (Angular):** Panel de control en tiempo real.
* **Python AI (Futuro):** Modelos de ML que analizan tendencias.



### 3. Microservicios :


Aquí es donde conectas tus scripts de Python (IA), Node.js, o tu Dashboard en Angular.

#### Comunicación de SALIDA (Monolito → Satélites)

Tus satélites necesitan ver el precio, pero no deben preguntarle al Monolito (lo distraerían).

- Mecanismo: El Monolito ya está guardando datos en DragonflyDB (Redis compatible).
- Protocolo: Tus microservicios se conectan a DragonflyDB y usan Pub/Sub.
- Ejemplo: El script de Python escucha el canal market_data en Dragonfly. Así recibe los precios en tiempo real sin molestar a Rust

#### Comunicación de ENTRADA (Satélites → Monolito)

cuanod un micro servicio necesita decirle al Monolito.

- El Problema: No puedes inyectar datos en la memoria del monolito.

La Solución Profesional: gRPC.

- El Monolito levanta un pequeño servidor gRPC (usando la llosibrería tonic en Rust) en un puerto interno .
- Defines un contrato .proto:
service TradingEngine {
rpc PlaceOrder (OrderRequest) returns (OrderResponse);
}
- Se envía el mensaje gRPC
- El monolito recibe el mensaje, lo convierte en OrdenInterna y lo mete al canal MPSC (el mismo que usan las estrategias rápidas) para que pase por el Motor de Riesgo.

#### Microservicios
- **API en Java (Spring Boot WebFlux)**: Genera endpoints REST y WebSocket para el dashboard. Se conecta a PostgreSQL/TimescaleDB y DragonflyDB. Utiliza gRPC para comunicarse con el monolito de Rust.
  - **Tecnologías**: Spring Boot 3.2, WebFlux (reactivo), R2DBC, Redis Reactive, gRPC Client
  - **Endpoints REST**: Market data, órdenes, balances
  - **WebSocket**: Streaming en tiempo real desde DragonflyDB Pub/Sub
  - **Ubicación**: `apps/kairos-api/` (Java/Maven)
  
- **Dashboard en Angular**: Interfaz web que se conecta a la API de Java usando REST API y WebSocket para datos en tiempo real
  - **Tecnolog ías**: Angular 21, TypeScript, RxJS
  - **Comunicación**: HTTP REST + WebSocket (sin GraphQL)
  - **Ubicación**: `apps/kairos-web/` (Angular/NPM)

---

## Estructura del Directorio Raíz: kairos-monorepo/


kairos-monorepo/
├── .github/                   # CI/CD Workflows
├── apps/                      # Aplicaciones ejecutables
│   ├── kairos-core/           # [EL MONOLITO] Motor de Trading (Rust)
apps/kairos-api/            # [SATÉLITE] API Gateway (Java/Spring Boot/WebFlux/gRPC)
│   └── kairos-web/            # [SATÉLITE] Dashboard (Angular)
├── libs/                      # Librerías compartidas (Rust Crates)
│   ├── kairos-domain/         # Entidades comunes (Order, MarketTick, Enums)
│   └── kairos-proto/          # Definiciones gRPC (.proto) y código generado
├── infrastructure/            # [EL HIERRO / INFRA]
│   ├── docker/                # Dockerfiles y Docker Compose
│   ├── db/                    # Scripts SQL (Timescale) y Config DragonflyDB
│   └── k8s/                   # Manifiestos de Kubernetes (Futuro)
├── research/                  # [FUTURO] Python AI, Jupyter Notebooks
├── Cargo.toml                 # Configuración del Rust Workspace (Root)
└── README.md
1. Detalle del Monolito: apps/kairos-core/
Aquí vive la lógica crítica. Aplicaremos Arquitectura Hexagonal (Ports & Adapters) para separar la lógica de negocio (Estrategias/Riesgo) de las herramientas (Binance/Redis).



apps/kairos-core/
├── Cargo.toml
└── src/
    ├── main.rs                # Entry Point: Inicia Tokio Runtime y orquesta los canales (MPSC/Broadcast)
    ├── domain/                # Lógica pura (Sin dependencias externas)
    │   ├── strategies/        # [THE SPRINTERS] Grafo Bellman-Ford, Lógica de Arbitraje
    │   ├── risk/              # [THE GATEKEEPER] Reglas de validación, Limites
    │   └── entities.rs        # Structs internos
    ├── application/           # Casos de uso y Orquestación
    │   ├── state.rs           # Gestión del "Saldo Atómico" (AtomicF64)
    │   └── engine.rs          # Coordinador de los "Órganos"
    └── adapters/              # Implementación técnica (Salida/Entrada)
        ├── inbound/           # Lo que entra al sistema
        │   ├── feed_handler/  # [THE FEED HANDLER] WebSocket Clients (Binance/OKX)
        │   └── grpc_server/   # Servidor gRPC para recibir órdenes de Satélites
        └── outbound/          # Lo que sale del sistema
            ├── persistence/   # [THE LOGGER] Conexión a DragonflyDB/Timescale (SQLx)
            └── execution/     # [THE SNIPER] HTTP/WS para enviar órdenes al Broker

2. Detalle del API Gateway: apps/kairos-api/
Este servicio actúa como intermediario. No hace trading, solo lee datos y pasa órdenes.

**Implementación actual: Java Spring Boot con WebFlux (Arquitectura Reactiva)**

```
apps/kairos-api/
├── pom.xml                    # Maven configuration
├── README.md                  # API documentation
└── src/
    ├── main/
    │   ├── java/com/kairos/   # Java source code (simplified package structure)
    │   │   ├── KairosApiApplication.java
    │   │   ├── config/        # Spring configurations (Redis, WebSocket, CORS)
    │   │   ├── model/         # Entities and DTOs
    │   │   ├── repository/    # R2DBC repositories (reactive)
    │   │   ├── service/       # Business logic layer
    │   │   ├── grpc/          # gRPC client for kairos-core
    │   │   ├── controller/    # REST endpoints
    │   │   └── websocket/     # WebSocket handlers
    │   ├── resources/
    │   │   ├── application.yml
    │   │   └── proto/
    │   │       └── trading_engine.proto  # Copied from libs/
    └── test/                  # Integration tests
```

**Endpoints REST:**
- `GET /api/market-data/*` - Datos de mercado (TimescaleDB + DragonflyDB)
- `POST /api/orders` - Crear orden (vía gRPC a kairos-core)
- `GET /api/balance/*` - Consultas de balance
- `ws://*/ws/market-data` - WebSocket para streaming en tiempo real

        
3. Detalle de Librerías Compartidas: libs/
Para no duplicar código entre el Core y la API.



libs/
├── kairos-domain/             # Crate: Tipos de datos universales
│   ├── src/
│   │   ├── lib.rs
│   │   ├── models.rs          # Ej: struct Order { ... }
│   │   └── errors.rs          # Errores comunes
└── kairos-proto/              # Crate: Contratos de comunicación
    ├── build.rs               # Script para compilar .proto con Tonic
    ├── proto/
    │   └── trading_engine.proto # Definición del servicio gRPC
    └── src/
        └── lib.rs             # Código Rust generado automáticamente
4. Detalle de Infraestructura: infrastructure/


infrastructure/
├── docker-compose.yml         # Levanta Dragonfly, Timescale, API y Dashboard localmente
├── db/
│   ├── init_timescale.sql     # Tablas para historial (OHLCV)
│   └── dragonfly.conf         # Tuning para baja latencia
└── scripts/
    └── deploy.sh

    
5. Detalle del Dashboard: apps/kairos-web/
Estructura estándar de Angular.



apps/kairos-web/
├── angular.json
├── package.json
└── src/
    ├── app/
    │   ├── core/              # Servicios Singleton (GraphQL Client, Auth)
    │   ├── features/
    │   │   ├── dashboard/     # Gráficos en tiempo real (TradingView charts?)
    │   │   └── settings/      # Configuración de bots
    │   └── shared/            # Componentes UI reusables
    └── assets/
        └── proto/             # Copia de .proto si usas gRPC-web (opcional)