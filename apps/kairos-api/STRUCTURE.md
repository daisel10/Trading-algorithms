# KAIRÓS: Simplified Java Package Structure

## Cambios Realizados

### 1. Estructura de Paquetes Simplificada

**Antes:** `com.kairos.api.*`
**Ahora:** `com.kairos.*`

Esto reduce un nivel de anidamiento manteniendo la convención Maven.

**Estructura final:**
```
apps/kairos-api/src/main/java/com/kairos/
├── KairosApiApplication.java
├── config/
│   ├── RedisConfig.java
│   ├── WebConfig.java
│   └── WebSocketConfig.java
├── model/
│   ├── MarketTick.java
│   ├── Order.java
│   ├── OhlcvCandle.java
│   └── dto/
│       ├── MarketTickDto.java
│       ├── PlaceOrderRequest.java
│       └── OrderResponse.java
├── repository/
│   ├── MarketTickRepository.java
│   ├── OrderRepository.java
│   └── OhlcvRepository.java
├── service/
│   ├── MarketDataService.java
│   ├── RealtimeDataService.java
│   └── TradingService.java
├── grpc/
│   └── TradingEngineGrpcClient.java
├── controller/
│   ├── MarketDataController.java
│   ├── OrderController.java
│   └── BalanceController.java
└── websocket/
    └── MarketDataWebSocketHandler.java
```

**Nota:** La estructura `src/main/java/` es **obligatoria** para Maven y no se puede eliminar.

### 2. Archivo Proto Centralizado

El archivo `trading_engine.proto` ahora se obtiene directamente de:
```
libs/kairos-proto/proto/trading_engine.proto
```

El Dockerfile lo copia automáticamente durante el build:
```dockerfile
COPY libs/kairos-proto/proto/trading_engine.proto ./src/main/proto/
```

### 3. Dockerfile Movido a Infrastructure

**Ubicación:** `infrastructure/docker/Dockerfile.api`

**Build context:** Desde la raíz del repositorio para acceder a:
- `apps/kairos-api/` (código fuente)
- `libs/kairos-proto/proto/` (archivo .proto)

### 4. Docker Compose Actualizado

```yaml
kairos-api:
  build:
    context: ..
    dockerfile: infrastructure/docker/Dockerfile.api
```

## Compilar y Ejecutar

### Maven (local)
```bash
cd apps/kairos-api
mvn clean package
java -jar target/kairos-api-0.1.0.jar
```

### Docker Compose
```bash
cd infrastructure
docker-compose build kairos-api
docker-compose up kairos-api
```

## Beneficios

✅ **Menos anidamiento**: `com.kairos` en lugar de `com.kairos.api`  
✅ **Proto centralizado**: Un solo archivo en `libs/kairos-proto/proto/`  
✅ **Docker organizado**: Todos los Dockerfiles en `infrastructure/docker/`  
✅ **README actualizado**: Documentación central refleja implementación Java
