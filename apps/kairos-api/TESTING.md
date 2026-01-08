# Testing Guide - kairos-api

Guía para ejecutar y escribir tests en kairos-api (Java Spring Boot).

---

## 🧪 Tipos de Tests

### 1. Unit Tests

Tests de servicios y lógica de negocio aislada.

**Ejecutar:**
```bash
mvn test
```

**Ejemplo:**
```java
@Test
void testMarketDataService_returnsLatestTicks() {
    MarketDataService service = new MarketDataService(mockRepository);
    
    Flux<MarketTick> ticks = service.getLatestTicks("BTCUSDT", 10);
    
    StepVerifier.create(ticks)
        .expectNextCount(10)
        .verifyComplete();
}
```

### 2. Integration Tests

Tests que verifican integración con bases de datos y Redis.

**Ejecutar:**
```bash
mvn verify
```

**Ejemplo:**
```java
@SpringBootTest
@AutoConfigureWebTestClient
class OrderControllerIntegrationTest {
    
    @Autowired
    private WebTestClient webClient;
    
    @Test
    void testPlaceOrder_returnsAccepted() {
        OrderRequest request = OrderRequest.builder()
            .symbol("BTCUSDT")
            .quantity(BigDecimal.valueOf(0.1))
            .orderType(OrderType.MARKET)
            .build();
        
        webClient.post()
            .uri("/api/orders")
            .bodyValue(request)
            .exchange()
            .expectStatus().isOk()
            .expectBody(OrderResponse.class)
            .value(response -> assertThat(response.getStatus()).isEqualTo("ACCEPTED"));
    }
}
```

### 3. WebSocket Tests

```java
@Test
void testWebSocketConnection_receivesMarketData() throws Exception {
    WebSocketClient client = new StandardWebSocketClient();
    WebSocketStompClient stompClient = new WebSocketStompClient(client);
    
    StompSession session = stompClient.connect("ws://localhost:4000/ws/market-data", 
        new StompSessionHandlerAdapter() {}).get(5, TimeUnit.SECONDS);
    
    session.subscribe("/topic/market-data/BTCUSDT", new StompFrameHandler() {
        @Override
        public void handleFrame(StompHeaders headers, Object payload) {
            MarketTick tick = (MarketTick) payload;
            assertThat(tick.getSymbol()).isEqualTo("BTCUSDT");
        }
    });
}
```

---

## 📊 Code Coverage

```bash
# Ejecutar tests con coverage
mvn clean verify

# Ver reporte
# Abrir target/site/jacoco/index.html
```

**Objetivo:** >= 75% coverage

---

## 🐛 Debugging

```bash
# Con logs detallados
mvn test -Dlogging.level.com.kairos=DEBUG

# Debug mode
mvn test -Dmaven.surefire.debug
# Conectar debugger al puerto 5005
```

---

**Última actualización:** 2026-01-06
