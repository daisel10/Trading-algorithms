# kairos-api - API Gateway (Java Spring Boot)

Gateway reactivo que expone REST API y WebSocket para el dashboard Angular. Actúa como intermediario entre kairos-web y kairos-core.

---

## 📖 Descripción

**kairos-api** es un microservicio construido con Spring Boot 3 WebFlux (arquitectura reactiva) que proporciona:
- REST API para operaciones CRUD
- WebSocket para streaming de datos en tiempo real
- Cliente gRPC para comunicarse con kairos-core
- Acceso a datos históricos (TimescaleDB) y caché (DragonflyDB)

---

## 🚀 Instalación y Configuración

### Prerequisitos

- **Java 21+** (OpenJDK o Eclipse Temurin)
- **Maven 3.9+**
- **TimescaleDB** (o PostgreSQL 16+)
- **DragonflyDB** (o Redis)
- **kairos-core** corriendo (para gRPC)

### Compilar y Ejecutar

```bash
cd apps/kairos-api

# Compilar
mvn clean package

# Ejecutar JAR
java -jar target/kairos-api-0.1.0.jar

# O ejecutar con Maven
mvn spring-boot:run
```

### Variables de Entorno

Configurar en `application.yml` o como variables de entorno:

```yaml
server:
  port: 4000

spring:
  r2dbc:
    url: r2dbc:postgresql://localhost:5432/kairos_trading
    username: kairos
    password: kairos_password
  
  data:
    redis:
      host: localhost
      port: 6379

grpc:
  client:
    kairos-core:
      address: static://localhost:50051
      negotiationType: plaintext
```

---

## 📡 REST API Endpoints

Ver [API_DOCS.md](./API_DOCS.md) para documentación completa.

### Market Data

| Endpoint | Método | Descripción |
|----------|--------|-------------|
| `/api/market-data/{symbol}` | GET | Últimos ticks de un símbolo |
| `/api/market-data/ohlcv/{symbol}` | GET | Datos OHLCV históricos |

### Orders

| Endpoint | Método | Descripción |
|----------|--------|-------------|
| `/api/orders` | POST | Crear orden (vía gRPC a core) |
| `/api/orders/{id}` | GET | Consultar orden |
| `/api/orders` | GET | Listar órdenes |

### Balance

| Endpoint | Método | Descripción |
|----------|--------|-------------|
| `/api/balance/{asset}` | GET | Balance de un activo |

### WebSocket

| Endpoint | Protocolo | Descripción |
|----------|-----------|-------------|
| `/ws/market-data` | WebSocket | Stream de precios en tiempo real |

---

## 🧪 Testing

```bash
# Tests unitarios
mvn test

# Tests de integración
mvn verify

# Con coverage (JaCoCo)
mvn clean verify
# Ver reporte en target/site/jacoco/index.html
```

Ver [TESTING.md](./TESTING.md) para más detalles.

---

## 🐳 Docker

```bash
# Desde la raíz del proyecto
docker build -f infrastructure/docker/Dockerfile.api -t kairos-api:latest .

# Ejecutar
docker run -p 4000:4000 --env-file .env kairos-api:latest
```

---

## 📚 Referencias

- [Spring WebFlux Docs](https://docs.spring.io/spring-framework/reference/web/webflux.html)
- [R2DBC Documentation](https://r2dbc.io/)
- [Project Reactor](https://projectreactor.io/)

---

**Mantenido por:** KAIRÓS Team  
**Última actualización:** 2026-01-06
