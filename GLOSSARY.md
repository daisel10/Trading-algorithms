# Glosario - KAIRÓS Trading System

Este documento define los términos técnicos y del dominio financiero utilizados en el sistema KAIRÓS para unificar el lenguaje entre desarrolladores, traders e inteligencia artificial.

---

## 🏦 Términos de Trading

### A

**Arbitraje**  
Estrategia que explota diferencias de precio del mismo activo en diferentes mercados o pares de trading para obtener ganancia sin riesgo (teórico).

**Arbitraje Triangular**  
Tipo específico de arbitraje que involucra tres pares de monedas. Ejemplo: BTC/USD → ETH/BTC → ETH/USD, buscando ciclos con ganancia neta positiva.

**Ask (Precio Ask)**  
El precio más bajo al que un vendedor está dispuesto a vender un activo. También llamado "Offer".

### B

**Bid (Precio Bid)**  
El precio más alto al que un comprador está dispuesto a comprar un activo.

**Bid-Ask Spread**  
Diferencia entre el precio Bid y Ask. Representa el costo de transacción implícito del mercado.

**Book (Order Book)**  
Ver "Order Book".

### C

**Ciclo Negativo**  
En el contexto de arbitraje triangular, un camino en el grafo de pares donde multiplicar los tipos de cambio resulta en un valor mayor a 1 (ganancia). Se detecta con algoritmos como Bellman-Ford.

### E

**Exchange**  
Plataforma de intercambio de criptomonedas (ej. Binance, OKX, Kraken).

### F

**Fill (Order Fill)**  
Confirmación de que una orden fue ejecutada total o parcialmente por el exchange.

**Feed Handler**  
Componente que se conecta a los WebSocket de exchanges para recibir datos de mercado en tiempo real.

### H

**HFT (High-Frequency Trading)**  
Trading de alta frecuencia. Estrategias que ejecutan miles de operaciones por segundo aprovechando micro-ineficiencias del mercado.

### L

**Latencia**  
Tiempo que transcurre entre un evento (ej. cambio de precio) y la reacción del sistema (ej. enviar orden). Crítico en HFT.

**Limit Order**  
Orden de compra/venta a un precio específico o mejor. No garantiza ejecución inmediata.

### M

**Maker**  
Orden que añade liquidez al order book (ej. limit order que no se ejecuta inmediatamente). Suele tener menores comisiones.

**Market Data**  
Información del mercado: precios, volumen, order book, trades ejecutados.

**Market Order**  
Orden de compra/venta inmediata al mejor precio disponible. Garantiza ejecución pero no precio.

**Market Tick**  
Actualización individual de datos de mercado (ej. cambio de precio bid/ask).

### O

**OHLCV**  
Open, High, Low, Close, Volume. Formato estándar para datos de velas (candlestick) en un intervalo temporal.

**Order**  
Instrucción de compra o venta de un activo.

**Order Book**  
Lista de todas las órdenes de compra (bids) y venta (asks) pendientes, ordenadas por precio.

### P

**Pair (Trading Pair)**  
Par de criptomonedas para intercambio (ej. BTC/USDT, ETH/BTC). El primer símbolo es la "base", el segundo es la "quote".

**Position**  
Cantidad de un activo que se posee (long position) o se debe (short position).

### S

**Slippage**  
Diferencia entre el precio esperado de una orden y el precio real de ejecución.

**Spread**  
Ver "Bid-Ask Spread".

**Strategy**  
Algoritmo de trading que decide cuándo comprar o vender basándose en datos de mercado.

### T

**Taker**  
Orden que remueve liquidez del order book (ej. market order). Suele tener mayores comisiones que el maker.

**Tick**  
Ver "Market Tick".

**Trade**  
Transacción ejecutada entre un comprador y vendedor.

---

## 🏗️ Términos de Arquitectura

### A

**Adapter (Hexagonal Architecture)**  
Implementación técnica que conecta el core de la aplicación con sistemas externos (ej. WebSocket, database, API REST).

**Atomic Operation**  
Operación que se completa totalmente o no se ejecuta en absoluto. No puede quedar en estado intermedio.

**AtomicF64**  
Tipo de dato concurrente en Rust que permite leer/escribir valores float64 sin race conditions.

### B

**Bellman-Ford Algorithm**  
Algoritmo para detectar ciclos negativos en grafos, usado en KAIRÓS para encontrar oportunidades de arbitraje triangular.

**Broadcast Channel**  
Canal de comunicación donde un emisor puede enviar mensajes a múltiples receptores simultáneamente (fan-out).

### C

**Channel**  
Mecanismo de comunicación entre threads/tareas asíncronas en Rust (via Tokio).

**Core (El Monolito)**  
El motor principal de trading (kairos-core), escrito en Rust, que contiene la lógica crítica de baja latencia.

### D

**DDD (Domain-Driven Design)**  
Metodología de diseño de software que prioriza el modelado del dominio de negocio (trading) sobre la implementación técnica.

**DragonflyDB**  
Base de datos en memoria compatible con Redis, optimizada para baja latencia. Usada para caché y pub/sub.

### E

**El Hierro**  
Capa de hardware y sistema operativo optimizada para baja latencia (CPU de alta frecuencia, kernel tuning).

**El Monolito**  
Ver "Core".

**Engine**  
Ver "Trading Engine".

### Feed Handler  
Ver término en sección de Trading.

### G

**Gatekeeper (Motor de Riesgo)**  
Componente que valida órdenes antes de enviarlas al exchange para prevenir pérdidas catastróficas.

**gRPC**  
Framework de RPC (Remote Procedure Call) de Google que usa HTTP/2 y Protocol Buffers. Usado para comunicación entre kairos-core y kairos-api.

### H

**Hexagonal Architecture (Ports & Adapters)**  
Patrón arquitectónico que separa la lógica de negocio (puerto) de las implementaciones técnicas (adaptadores).

### L

**Logger (The Logger)**  
Componente que persiste datos históricos en TimescaleDB sin bloquear el trading en tiempo real.

### M

**MPSC (Multi-Producer, Single-Consumer)**  
Tipo de canal donde múltiples emisores pueden enviar mensajes a un único receptor. Usado para que múltiples estrategias envíen órdenes al Motor de Riesgo.

**Monorepo**  
Repositorio único que contiene múltiples proyectos relacionados (kairos-core, kairos-api, kairos-web).

### O

**Ownership (Rust)**  
Sistema de gestión de memoria de Rust donde cada valor tiene un único "dueño". Previene use-after-free y data races.

### P

**Port (Hexagonal Architecture)**  
Interfaz abstracta que define cómo el core de la aplicación interactúa con el mundo exterior.

**Protocol Buffers (Protobuf)**  
Formato de serialización binaria de Google. Usado en gRPC para eficiencia.

### R

**R2DBC**  
Reactive Relational Database Connectivity. Driver reactivo para bases de datos SQL, usado en kairos-api (Java).

**Reactive (Programming)**  
Paradigma de programación asíncrono basado en streams de datos y propagación de cambios. Usado en kairos-api con Spring WebFlux.

### S

**Satélites**  
Microservicios auxiliares que no contienen lógica crítica de trading (ej. kairos-api, kairos-web, futuros scripts de IA).

**Sniper (The Sniper)**  
Componente que ejecuta órdenes en el exchange después de aprobación del Motor de Riesgo.

**Sprinters (The Sprinters)**  
Estrategias de baja latencia que procesan datos de mercado y generan señales de trading.

### T

**TimescaleDB**  
Extensión de PostgreSQL optimizada para series temporales. Usada para almacenar historial de OHLCV y trades.

**Tokio**  
Runtime asíncrono de Rust. Maneja concurrencia, I/O non-blocking, y scheduling de tareas.

**Tonic**  
Librería de Rust para implementar servidores y clientes gRPC.

**Trading Engine**  
Motor principal que coordina los componentes de trading (Feed Handler, Strategies, Risk, Execution).

### W

**WebFlux**  
Framework reactivo de Spring para aplicaciones web no-bloqueantes. Usado en kairos-api.

**WebSocket (WSS)**  
Protocolo de comunicación bidireccional sobre TCP. Usado para recibir datos de mercado en tiempo real desde exchanges.

**Workspace (Cargo)**  
Conjunto de crates (proyectos Rust) que comparten configuración y dependencies. KAIRÓS usa un workspace con kairos-core, kairos-domain, kairos-proto.

---

## 🔧 Términos Técnicos Específicos de KAIRÓS

**OrderRequest**  
Mensaje gRPC definido en proto files para solicitar ejecución de una orden.

**OrdenInterna**  
Struct Rust que representa una orden antes de ser enviada al exchange. Contiene validaciones pre-aprobadas.

**Saldo Atómico**  
Variable AtomicF64 que mantiene el balance disponible en memoria para validación ultra-rápida por el Motor de Riesgo.

**MarketTick**  
Struct estandarizado en Rust que representa una actualización de mercado (bid, ask, volume, timestamp).

---

## 📝 Convenciones de Código

**Crate**  
Unidad de compilación en Rust (equivalente a library o package en otros lenguajes).

**Trait**  
Interfaz en Rust que define comportamiento compartido.

**Async/Await**  
Sintaxis en Rust para programación asíncrona sin callbacks.

---

## 🔗 Referencias

- Para términos de trading: [Investopedia](https://www.investopedia.com/)
- Para arquitectura hexagonal: [Alistair Cockburn's Article](https://alistair.cockburn.us/hexagonal-architecture/)
- Para Rust: [The Rust Book](https://doc.rust-lang.org/book/)
- Para DDD: [Domain-Driven Design by Eric Evans](https://www.domainlanguage.com/ddd/)
