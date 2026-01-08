# ADR-003: Uso de gRPC para Comunicación Interna

**Fecha:** 2025-12-26  
**Estado:** Aceptado  
**Autores:** KAIRÓS Team

## Contexto

El sistema KAIRÓS está compuesto por múltiples componentes:
- **kairos-core** (Rust): Motor de trading de baja latencia
- **kairos-api** (Java): API Gateway para el dashboard
- **kairos-web** (Angular): Dashboard web

Los microservicios satélites (como kairos-api) necesitan comunicarse con el monolito para:
- Enviar órdenes de trading desde el dashboard
- Consultar estado del sistema
- Solicitar ejecución de estrategias manuales

### Requisitos

- **Tipado fuerte:** Contratos de API versionados y validados
- **Eficiencia:** Serialización binaria más rápida que JSON
- **Multi-lenguaje:** Soporte para Rust, Java, y posiblemente Python
- **Streaming:** Capacidad para respuestas en tiempo real (opcional)

## Decisión

**Usamos gRPC con Protocol Buffers como protocolo de comunicación entre kairos-core y los satélites.**

### Implementación

- **Librería Rust:** `tonic` (servidor gRPC en kairos-core)
- **Librería Java:** `grpc-spring-boot-starter` (cliente en kairos-api)
- **Definiciones:** `.proto` files en `libs/kairos-proto/`
- **Generación de código:** Automática en build time (Rust: `build.rs`, Java: Maven plugin)

### Contratos Principales

```protobuf
service TradingEngine {
  rpc PlaceOrder (OrderRequest) returns (OrderResponse);
  rpc GetBalance (BalanceRequest) returns (BalanceResponse);
  rpc GetSystemStatus (Empty) returns (SystemStatusResponse);
}
```

## Consecuencias

### Positivas ✅

- **Rendimiento:** Serialización binaria Protobuf ~5x más rápida que JSON
- **Type Safety:** Errores de contrato detectados en compilación
- **Versionado:** Protobuf permite evolución backward-compatible
- **Tooling:** Generación automática de clientes en múltiples lenguajes
- **Streaming:** Soporte nativo para server/client streaming (útil para feeds)
- **HTTP/2:** Multiplexing y flow control integrados

### Negativas ❌

- **Debugging:** Menos human-readable que REST/JSON
- **Complejidad:** Setup más complejo que simple HTTP REST
- **Browser Support:** No funciona nativamente en navegadores (requiere gRPC-Web)
- **Infraestructura:** Algunos load balancers legacy no soportan HTTP/2

## Alternativas Consideradas

### REST API con JSON (Rechazada)
**Por qué se descartó:**
- Overhead de serialización JSON
- Sin tipado fuerte (OpenAPI ayuda pero no es compilado)
- Más lento que Protobuf
- Verbosity en payloads grandes

**Ventajas perdidas:**
- Debugging más fácil (curl, Postman)
- Mayor familiaridad del equipo
- Soporte universal

### GraphQL (Rechazada)
**Por qué se descartó:**
- Overhead de parsing en runtime
- Complejo para microservicios simples
- No diseñado para low-latency
- Más adecuado para agregación de datos

**Ventajas perdidas:**
- Query flexibility
- Resolución de N+1 queries

### Apache Thrift (Rechazada)
**Por qué se descartó:**
- Ecosistema menos activo que gRPC
- Tooling menos maduro en Rust
- Sin HTTP/2 nativo

**Ventajas similares:**
- También usa serialización binaria
- También genera código multi-lenguaje

### MessageQueue (RabbitMQ/Kafka) (Rechazada para comandos)
**Por qué se descartó para request/response:**
- Latencia adicional (paso por broker)
- Overkill para comunicación sincrónica simple
- Mayor complejidad operacional

**Cuándo SÍ se usa:**
- Para eventos asincrónicos (market data, order fills)
- DragonflyDB Pub/Sub para broadcasting

## Patrón de Uso

### kairos-api → kairos-core (Command)

```java
// Java Client
OrderServiceGrpc.OrderServiceBlockingStub stub = ...;
OrderRequest request = OrderRequest.newBuilder()
    .setSymbol("BTCUSDT")
    .setQuantity(0.1)
    .build();
OrderResponse response = stub.placeOrder(request);
```

### kairos-core (Rust Server)

```rust
impl TradingEngine for MyTradingEngine {
    async fn place_order(&self, request: OrderRequest) -> Result<OrderResponse> {
        // Validar con Motor de Riesgo
        // Convertir a OrdenInterna
        // Enviar a canal MPSC
    }
}
```

## Flujo de Datos

```
Dashboard (kairos-web)
    ↓ HTTP REST
kairos-api (Java)
    ↓ gRPC
kairos-core (Rust) → Canal MPSC → Motor de Riesgo → Ejecución
```

## Notas de Implementación

- **Puerto gRPC:** 50051 (configurable via env var)
- **TLS:** En producción usar mTLS para autenticación mutua
- **Health Checks:** Implementar `grpc.health.v1.Health` service
- **Timeouts:** Cliente debe configurar deadlines agresivos (1-5s)
- **Retries:** Idempotencia en PlaceOrder (usar UUID de orden)

## Mitigaciones

### Problema: Debugging difícil
**Solución:** Usar `grpcurl` y logging detallado de requests/responses

### Problema: No funciona en browser
**Solución:** kairos-web se comunica con kairos-api vía REST, NO directamente con kairos-core

## Referencias

- [gRPC Official Docs](https://grpc.io/)
- [Tonic Rust Documentation](https://docs.rs/tonic/)
- [Protocol Buffers Guide](https://developers.google.com/protocol-buffers)

## Estado Actual

Este ADR está **ACEPTADO** y en producción. El servicio gRPC está implementado en `apps/kairos-core/src/adapters/inbound/grpc_server/`.
