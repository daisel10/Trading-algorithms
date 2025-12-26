# KAIRÓS: High-Performance Algorithmic Trading Core

![Status](https://img.shields.io/badge/Status-Pre--Alpha-critical)
![Core](https://img.shields.io/badge/Core-Rust-orange)
![Dashboard](https://img.shields.io/badge/Dashboard-Angular-red)
![Architecture](https://img.shields.io/badge/Architecture-Monorepo-blue)
![Infra](https://img.shields.io/badge/Infra-DragonflyDB%20%7C%20Timescale-green)



## 📖 Visión del Proyecto

Kairós es un motor de trading de **baja latencia** y arquitectura híbrida. Diseñado bajo principios de **DDD (Domain-Driven Design)**, **Arquitectura Hexagonal** y **Hexagonal**, su objetivo es desacoplar la inteligencia (Estrategias/IA) de la ejecución crítica (Riesgo/Conectividad/extracion de datos / persistencia).

Actualmente, el proyecto opera como un **Monorepo**, centralizando el desarrollo del Núcleo (Rust), la Infraestructura y el Dashboard de control.

---
## Broker actuales
- OKX
- Binance

## 📚 Actual desarrollo
- creacion del monolito en rust

- creacion del api 

- creacion de dashboard



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
- API en rust que genere los endpoints un dashboard, usando GraphQL y gRPC para el monolito
-  Dashboard en Angular que se conecte a la API de rust usando GraphQL 

---

## Estructura del Directorio Raíz: kairos-monorepo/


kairos-monorepo/
├── .github/                   # CI/CD Workflows
├── apps/                      # Aplicaciones ejecutables
│   ├── kairos-core/           # [EL MONOLITO] Motor de Trading (Rust)
│   ├── kairos-api/            # [SATÉLITE] API Gateway (Rust/GraphQL/gRPC)
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



apps/kairos-api/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── graphql/               # Esquemas y Resolvers (Async-graphql o Juniper)
    │   ├── schema.rs
    │   ├── query.rs           # Consultas de lectura (hacia DragonflyDB)
    │   └── mutation.rs        # Acciones (llaman al cliente gRPC)
    └── clients/
        ├── db_reader.rs       # Cliente Redis/Dragonfly para leer datos "calientes"
        └── core_grpc.rs       # Cliente gRPC para hablar con 'kairos-core'
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