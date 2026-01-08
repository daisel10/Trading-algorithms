# API Documentation - kairos-api

Documentación completa de los endpoints REST y WebSocket expuestos por kairos-api.

---

## Base URL

```
http://localhost:4000
```

---

## 🔐 Authentication (Futuro)

Actualmente sin autenticación. En producción se implementará JWT.

---

## 📊 Market Data Endpoints

### GET /api/market-data/{symbol}

Obtiene los últimos ticks de mercado para un símbolo.

**Parameters:**
- `symbol` (path) - Par de trading (ej. "BTCUSDT")
- `limit` (query, opcional) - Número de ticks (default: 100, max: 1000)

**Response:**
```json
{
  "symbol": "BTCUSDT",
  "data": [
    {
      "exchange": "BINANCE",
      "bidPrice": 50000.50,
      "askPrice": 50001.00,
      "bidVolume": 5.2,
      "askVolume": 3.8,
      "timestamp": "2026-01-06T22:00:00Z"
    }
  ]
}
```

**Example:**
```bash
curl http://localhost:4000/api/market-data/BTCUSDT?limit=10
```

---

### GET /api/market-data/ohlcv/{symbol}

Obtiene datos OHLCV (candlestick) históricos.

**Parameters:**
- `symbol` (path) - Par de trading
- `interval` (query) - Intervalo temporal: "1m", "5m", "1h", "1d"
- `from` (query) - Timestamp inicio (ISO 8601)
- `to` (query, opcional) - Timestamp fin (default: now)

**Response:**
```json
{
  "symbol": "BTCUSDT",
  "interval": "1h",
  "data": [
    {
      "timestamp": "2026-01-06T22:00:00Z",
      "open": 50000.00,
      "high": 50500.00,
      "low": 49800.00,
      "close": 50200.00,
      "volume": 1250.5
    }
  ]
}
```

**Example:**
```bash
curl "http://localhost:4000/api/market-data/ohlcv/BTCUSDT?interval=1h&from=2026-01-06T00:00:00Z"
```

---

## 📝 Order Endpoints

### POST /api/orders

Crea una nueva orden (enviada a kairos-core vía gRPC).

**Request Body:**
```json
{
  "symbol": "BTCUSDT",
  "quantity": 0.5,
  "orderType": "MARKET",
  "price": null,
  "clientId": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Response:**
```json
{
  "orderId": "123e4567-e89b-12d3-a456-426614174000",
  "status": "ACCEPTED",
  "createdAt": "2026-01-06T22:00:00Z"
}
```

**Error Response (4xx):**
```json
{
  "error": "RISK_VALIDATION_FAILED",
  "message": "Order size exceeds maximum allowed",
  "timestamp": "2026-01-06T22:00:00Z"
}
```

**Example:**
```bash
curl -X POST http://localhost:4000/api/orders \
  -H "Content-Type: application/json" \
  -d '{
    "symbol": "BTCUSDT",
    "quantity": 0.1,
    "orderType": "MARKET",
    "clientId": "550e8400-e29b-41d4-a716-446655440000"
  }'
```

---

### GET /api/orders/{id}

Consulta el estado de una orden.

**Parameters:**
- `id` (path) - UUID de la orden

**Response:**
```json
{
  "orderId": "123e4567-e89b-12d3-a456-426614174000",
  "symbol": "BTCUSDT",
  "quantity": 0.5,
  "filledQuantity": 0.5,
  "status": "FILLED",
  "avgPrice": 50100.00,
  "createdAt": "2026-01-06T22:00:00Z",
  "filledAt": "2026-01-06T22:00:05Z"
}
```

---

### GET /api/orders

Lista todas las órdenes (paginado).

**Parameters:**
- `page` (query, opcional) - Número de página (default: 0)
- `size` (query, opcional) - Tamaño de página (default: 20, max: 100)
- `status` (query, opcional) - Filtrar por estado: "PENDING", "FILLED", "REJECTED"

**Response:**
```json
{
  "content": [
    {
      "orderId": "...",
      "symbol": "BTCUSDT",
      "status": "FILLED",
      "createdAt": "2026-01-06T22:00:00Z"
    }
  ],
  "page": 0,
  "size": 20,
  "totalElements": 150
}
```

---

## 💰 Balance Endpoints

### GET /api/balance/{asset}

Obtiene el balance de un activo.

**Parameters:**
- `asset` (path) - Símbolo del activo (ej. "BTC", "USDT")

**Response:**
```json
{
  "asset": "BTC",
  "available": 1.5,
  "locked": 0.2,
  "total": 1.7
}
```

---

## 🔌 WebSocket Endpoints

### WS /ws/market-data

Stream de precios en tiempo real usando WebSocket.

**Connection:**
```javascript
const ws = new WebSocket('ws://localhost:4000/ws/market-data');

ws.onopen = () => {
  // Suscribirse a símbolos
  ws.send(JSON.stringify({
    action: 'subscribe',
    symbols: ['BTCUSDT', 'ETHUSDT']
  }));
};

ws.onmessage = (event) => {
  const tick = JSON.parse(event.data);
  console.log(tick);
  // {
  //   "symbol": "BTCUSDT",
  //   "bidPrice": 50000.50,
  //   "askPrice": 50001.00,
  //   "timestamp": "2026-01-06T22:00:00.123Z"
  // }
};
```

**Actions:**

**Subscribe:**
```json
{
  "action": "subscribe",
  "symbols": ["BTCUSDT", "ETHUSDT"]
}
```

**Unsubscribe:**
```json
{
  "action": "unsubscribe",
  "symbols": ["BTCUSDT"]
}
```

---

## ❌ Error Codes

| Code | HTTP Status | Descripción |
|------|-------------|-------------|
| `INVALID_REQUEST` | 400 | Request body inválido |
| `SYMBOL_NOT_FOUND` | 404 | Símbolo no existe |
| `RISK_VALIDATION_FAILED` | 400 | Orden rechazada por riesgo |
| `GRPC_CONNECTION_ERROR` | 503 | kairos-core no disponible |
| `DATABASE_ERROR` | 500 | Error de base de datos |

**Error Response Format:**
```json
{
  "error": "ERROR_CODE",
  "message": "Human-readable message",
  "timestamp": "2026-01-06T22:00:00Z",
  "path": "/api/orders"
}
```

---

## 📈 Rate Limiting (Futuro)

- **Anónimo:** 100 requests/minuto
- **Autenticado:** 1000 requests/minuto

---

**Última actualización:** 2026-01-06  
**Mantenido por:** KAIRÓS Team
