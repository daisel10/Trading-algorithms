# kairos-api: Configuración de Agentes

## 📘 Filosofía de Uso

**Este documento define el contexto del API Gateway KAIRÓS (Java Spring Boot) para agentes de IA.**
Consulta la documentación de referencia según la tarea que vayas a realizar.

---

## 🛠 Habilidades Disponibles (Skillset)

Habilidades específicas de kairos-api (API Gateway en Java).

### 🌐 Habilidades Tecnológicas (Java/Spring Stack)

*Patrones técnicos específicos del API Gateway.*

| Habilidad | Descripción | Archivo de Referencia |
| :--- | :--- | :--- |
| `spring-webflux` | Programación reactiva con Mono/Flux, REST reactivo | `docs/SPRING_WEBFLUX.md` (crear si necesario) |
| `r2dbc` | Acceso reactivo a PostgreSQL/TimescaleDB | `docs/DATABASE.md` |
| `redis-reactive` | Cliente Redis reactivo para caché y Pub/Sub | `docs/REDIS.md` |
| `grpc-client` | Cliente gRPC para comunicación con kairos-core | `docs/GRPC_CLIENT.md` (crear si necesario) |
| `websocket-spring` | WebSocket handlers para streaming en tiempo real | `docs/WEBSOCKET.md` (crear si necesario) |

> **Nota**: Los archivos de habilidades en `docs/` pueden crearse según necesidad siguiendo el patrón de kairos-core.

---

## 🤖 Disparadores Automáticos (Auto-invoke)

**REGLA DE ORO:** Antes de realizar una acción, carga la habilidad correspondiente.

### 🏗 Desarrollo & Arquitectura

| Acción (Lo que vas a hacer) | Habilidad Requerida (Lo que debes leer antes) |
| :--- | :--- |
| Crear nuevos endpoints REST | `spring-webflux` |
| Implementar queries a base de datos | `r2dbc` |
| Configurar Redis Pub/Sub | `redis-reactive` |
| Llamar a kairos-core vía gRPC | `grpc-client` |
| Implementar streaming WebSocket | `websocket-spring` |
| Evitar blocking calls | `spring-webflux` (⚠️ CRÍTICO) |

### 🧪 Calidad & Testing

| Acción | Habilidad Requerida |
| :--- | :--- |
| Escribir tests unitarios | Testing patterns (JUnit 5 + Mockito) |
| Escribir tests de integración | Reactor Test + Testcontainers |
| Verificar no hay blocking calls | Code review con `grep .block()` |

---

## 🗺 Visión General del Proyecto

**kairos-api** es el API Gateway reactivo del sistema KAIRÓS, construido con Java Spring Boot WebFlux.

### Propósito

Microservicio que:

- Expone **REST API** para consultas históricas
- Provee **WebSocket** para streaming en tiempo real
- Actúa como **cliente gRPC** hacia kairos-core
- Gestiona conexiones reactivas a DB y Redis
- Implementa CORS para frontend Angular

### Stack Tecnológico

| Categoría | Tecnología | Versión |
| :--- | :--- | :--- |
| **Lenguaje** | Java | 21 LTS |
| **Framework** | Spring Boot WebFlux | 3.2.1 |
| **Build Tool** | Maven | 3.9+ |
| **DB Client** | R2DBC PostgreSQL | 1.0.2 |
| **Redis Client** | Lettuce (Spring Data Redis Reactive) | Incluido |
| **Testing** | JUnit 5 + Reactor Test | Incluido |
| **Utilities** | Lombok | 1.18.30 |

### Arquitectura en Capas

```
controller/    → REST endpoints (@RestController)
websocket/     → WebSocket handlers (streaming)
service/       → Lógica de negocio (gRPC, aggregation)
repository/    → R2DBC repositories (reactive DB)
model/         → Entities y DTOs
config/        → Configuración (Redis, WebSocket, CORS)
```

### Estructura de Directorios

```
apps/kairos-api/
├── pom.xml
├── src/
│   ├── main/
│   │   ├── java/com/kairos/
│   │   │   ├── KairosApiApplication.java    # Entry point
│   │   │   ├── config/                      # Spring configs
│   │   │   ├── controller/                   # REST endpoints
│   │   │   ├── service/                      # Business logic
│   │   │   ├── repository/                   # R2DBC repositories
│   │   │   ├── model/                        # Entities + DTOs
│   │   │   └── websocket/                    # WebSocket handlers
│   │   └── resources/
│   │       └── application.yml               # Spring properties
│   └── test/                                  # Integration tests
└── docs/                                      # API documentation
```

---

## ⚡ Flujo de Trabajo

### Desarrollo Local

```bash
# Configurar application.yml con DB/Redis URLs
# Ya está en application.yml, revisar credenciales

# Compilar
mvn clean install

# Ejecutar
mvn spring-boot:run

# Ejecutar con perfil específico
mvn spring-boot:run -Dspring-boot.run.profiles=dev
```

### Testing

```bash
# Tests unitarios
mvn test

# Tests de integración
mvn verify

# Tests con coverage
mvn clean test jacoco:report
```

### Linting & Build

```bash
# Verificar estilo
mvn checkstyle:check

# Build de producción
mvn clean package -DskipTests

# Build Docker image
mvn spring-boot:build-image
```

---

## 📋 Endpoints REST Principales

| Endpoint | Método | Descripción |
| :--- | :--- | :--- |
| `/api/market-data/latest/{symbol}` | GET | Último precio de un símbolo |
| `/api/market-data/ohlcv/{symbol}` | GET | Datos OHLCV históricos |
| `/api/orders` | POST | Crear orden (gRPC → core) |
| `/api/orders/{id}` | GET | Detalle de orden |
| `/api/balance/{userId}` | GET | Balance de usuario |

### WebSocket

- **Endpoint**: `ws://localhost:8080/ws/market-data`
- **Flujo**: Cliente subscribe → API consume Redis Pub/Sub → Streaming

---

## 📝 Convenciones de Código

### Estilo Java

- **Packages**: lowercase `com.kairos.controller`
- **Classes**: PascalCase `MarketDataController`
- **Methods**: camelCase `getLatestPrice()`
- **Constants**: SCREAMING_SNAKE_CASE `MAX_RETRY_ATTEMPTS`

### Reactivo

```java
// ✅ CORRECTO: Endpoint reactivo
@GetMapping("/latest/{symbol}")
public Mono<MarketDataDTO> getLatest(@PathVariable String symbol) {
    return service.getLatestPrice(symbol)
        .switchIfEmpty(Mono.error(new ResponseStatusException(HttpStatus.NOT_FOUND)));
}

// ❌ INCORRECTO: Blocking call
Mono<String> data = service.getData();
String result = data.block(); // ¡DEADLOCK!
```

---

## 🔗 Comunicación

- **kairos-api → kairos-core**: gRPC (tonic client)
- **kairos-api → TimescaleDB**: R2DBC (reactive)
- **kairos-api → DragonflyDB**: Lettuce (reactive)
- **kairos-web → kairos-api**: REST + WebSocket

---

## 📋 Guía de Commits

**Formato**: `<type>(<scope>): <description>`

**Ejemplos**:

- `feat(api): add market data streaming endpoint`
- `fix(websocket): resolve connection timeout issue`
- `perf(repository): optimize OHLCV query`

---

**Última actualización:** 2026-01-19  
**Mantenido por:** kairos-api Development Team
