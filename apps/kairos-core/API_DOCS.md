# API Documentation - kairos-core (Internal)

Documentación de los módulos internos y contratos de datos del motor de trading KAIRÓS.

---

## 📡 gRPC Service

### TradingEngine Service

**Proto File:** `../../libs/kairos-proto/proto/trading_engine.proto`

#### PlaceOrder

Envía una orden de trading al motor.

**Request:**
```protobuf
message OrderRequest {
  string symbol = 1;           // Par de trading (ej. "BTCUSDT")
  double quantity = 2;         // Cantidad a comprar/vender
  OrderType order_type = 3;    // MARKET o LIMIT
  optional double price = 4;   // Precio (solo para LIMIT)
  string client_id = 5;        // ID del cliente (para idempotencia)
}

enum OrderType {
  MARKET = 0;
  LIMIT = 1;
}
```

**Response:**
```protobuf
message OrderResponse {
  string order_id = 1;         // ID único de la orden
  OrderStatus status = 2;      // PENDING, ACCEPTED, REJECTED
  optional string error = 3;   // Mensaje de error si fue rechazada
}

enum OrderStatus {
  PENDING = 0;
  ACCEPTED = 1;
  REJECTED = 2;
  FILLED = 3;
  CANCELLED = 4;
}
```

**Ejemplo (Rust):**
```rust
use kairos_proto::trading_engine_client::TradingEngineClient;
use kairos_proto::{OrderRequest, OrderType};

let mut client = TradingEngineClient::connect("http://localhost:50051").await?;

let request = OrderRequest {
    symbol: "BTCUSDT".into(),
    quantity: 0.5,
    order_type: OrderType::Market as i32,
    price: None,
    client_id: uuid::Uuid::new_v4().to_string(),
};

let response = client.place_order(request).await?;
match response.into_inner().status() {
    OrderStatus::Accepted => println!("Order accepted!"),
    OrderStatus::Rejected => println!("Order rejected: {}", response.error),
    _ => {}
}
```

#### GetBalance

Consulta el balance disponible de un activo.

**Request:**
```protobuf
message BalanceRequest {
  string asset = 1;  // Símbolo del activo (ej. "BTC", "USDT")
}
```

**Response:**
```protobuf
message BalanceResponse {
  string asset = 1;
  double available = 2;    // Balance disponible
  double locked = 3;       // Balance bloqueado en órdenes
  double total = 4;        // Total = available + locked
}
```

#### GetSystemStatus

Obtiene el estado del sistema.

**Request:**
```protobuf
message Empty {}
```

**Response:**
```protobuf
message SystemStatusResponse {
  bool is_healthy = 1;
  int64 uptime_seconds = 2;
  int32 active_strategies = 3;
  int32 pending_orders = 4;
  map<string, ExchangeStatus> exchanges = 5;
}

message ExchangeStatus {
  string name = 1;
  bool is_connected = 2;
  int64 last_heartbeat_ms = 3;
}
```

---

## 🔄 Internal Data Structures

### MarketTick

Representa una actualización de mercado normalizada.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTick {
    pub symbol: String,        // Par de trading
    pub exchange: Exchange,    // Exchange de origen
    pub bid_price: f64,        // Mejor precio de compra
    pub ask_price: f64,        // Mejor precio de venta
    pub bid_volume: f64,       // Volumen en bid
    pub ask_volume: f64,       // Volumen en ask
    pub timestamp: DateTime<Utc>,
}

impl MarketTick {
    pub fn spread(&self) -> f64 {
        self.ask_price - self.bid_price
    }
    
    pub fn mid_price(&self) -> f64 {
        (self.bid_price + self.ask_price) / 2.0
    }
}
```

### InternalOrder

Orden interna validada antes de envío al exchange.

```rust
#[derive(Debug, Clone)]
pub struct InternalOrder {
    pub id: Uuid,
    pub symbol: String,
    pub exchange: Exchange,
    pub side: OrderSide,       // BUY o SELL
    pub order_type: OrderType, // MARKET o LIMIT
    pub quantity: f64,
    pub price: Option<f64>,    // Solo para LIMIT
    pub created_at: DateTime<Utc>,
    pub strategy: StrategyType, // Qué estrategia generó esta orden
}

#[derive(Debug, Clone, Copy)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy)]
pub enum OrderType {
    Market,
    Limit,
}

#[derive(Debug, Clone, Copy)]
pub enum StrategyType {
    TriangularArbitrage,
    MarketMaking,
    ManualOrder,  // Desde dashboard
}
```

### Exchange Enum

```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Exchange {
    Binance,
    OKX,
}
```

---

## 📊 Module Interfaces

### Strategy Trait

Todas las estrategias deben implementar este trait:

```rust
#[async_trait]
pub trait Strategy: Send + Sync {
    /// Procesa un tick de mercado y opcionalmente genera una orden
    async fn on_market_tick(&mut self, tick: MarketTick) -> Option<InternalOrder>;
    
    /// Nombre de la estrategia
    fn name(&self) -> &str;
    
    /// Inicialización (ej. cargar estado desde DB)
    async fn initialize(&mut self) -> Result<()>;
    
    /// Cleanup (ej. persistir estado)
    async fn shutdown(&mut self) -> Result<()>;
}
```

**Ejemplo de implementación:**

```rust
pub struct TriangularArbitrageStrategy {
    graph: CurrencyGraph,
    min_profit_threshold: f64,
}

#[async_trait]
impl Strategy for TriangularArbitrageStrategy {
    async fn on_market_tick(&mut self, tick: MarketTick) -> Option<InternalOrder> {
        // Actualizar grafo con nuevo precio
        self.graph.update_edge(&tick.symbol, tick.bid_price, tick.ask_price);
        
        // Detectar ciclo negativo (oportunidad de arbitraje)
        if let Some(cycle) = self.graph.find_negative_cycle() {
            let profit = cycle.calculate_profit();
            if profit > self.min_profit_threshold {
                return Some(cycle.to_order());
            }
        }
        
        None
    }
    
    fn name(&self) -> &str {
        "TriangularArbitrage"
    }
    
    async fn initialize(&mut self) -> Result<()> {
        // Cargar configuración
        Ok(())
    }
    
    async fn shutdown(&mut self) -> Result<()> {
        // Persistir estado
        Ok(())
    }
}
```

### RiskValidator Trait

```rust
pub trait RiskValidator: Send + Sync {
    /// Valida si una orden cumple con las reglas de riesgo
    fn validate(&self, order: &InternalOrder) -> Result<(), RiskError>;
}

pub struct DefaultRiskValidator {
    max_order_size: f64,
    daily_loss_limit: f64,
    current_daily_loss: AtomicF64,
    balances: Arc<RwLock<HashMap<String, f64>>>,
}

impl RiskValidator for DefaultRiskValidator {
    fn validate(&self, order: &InternalOrder) -> Result<(), RiskError> {
        // 1. Verificar tamaño de orden
        if order.quantity > self.max_order_size {
            return Err(RiskError::OrderTooLarge);
        }
        
        // 2. Verificar límite de pérdidas diarias
        let daily_loss = self.current_daily_loss.load(Ordering::Relaxed);
        if daily_loss > self.daily_loss_limit {
            return Err(RiskError::DailyLossLimitExceeded);
        }
        
        // 3. Verificar balance disponible
        let balances = self.balances.read().unwrap();
        let available = balances.get(&order.symbol).unwrap_or(&0.0);
        if order.quantity > *available {
            return Err(RiskError::InsufficientBalance);
        }
        
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RiskError {
    #[error("Order size exceeds maximum allowed")]
    OrderTooLarge,
    
    #[error("Daily loss limit exceeded")]
    DailyLossLimitExceeded,
    
    #[error("Insufficient balance")]
    InsufficientBalance,
}
```

---

## 🔌 Adapter Interfaces

### FeedHandler Trait

```rust
#[async_trait]
pub trait FeedHandler: Send + Sync {
    /// Conecta al WebSocket del exchange
    async fn connect(&mut self) -> Result<()>;
    
    /// Se suscribe a pares de trading
    async fn subscribe(&mut self, symbols: Vec<String>) -> Result<()>;
    
    /// Recibe el siguiente tick
    async fn next_tick(&mut self) -> Result<MarketTick>;
    
    /// Nombre del exchange
    fn exchange_name(&self) -> &str;
}
```

### PersistenceAdapter Trait

```rust
#[async_trait]
pub trait PersistenceAdapter: Send + Sync {
    /// Guarda tick en base de datos
    async fn save_tick(&self, tick: &MarketTick) -> Result<()>;
    
    /// Guarda orden ejecutada
    async fn save_order(&self, order: &ExecutedOrder) -> Result<()>;
    
    /// Batch insert de múltiples ticks
    async fn save_ticks_batch(&self, ticks: Vec<MarketTick>) -> Result<()>;
}
```

### ExecutionAdapter Trait

```rust
#[async_trait]
pub trait ExecutionAdapter: Send + Sync {
    /// Envía orden al exchange
    async fn execute_order(&self, order: &InternalOrder) -> Result<ExecutedOrder>;
    
    /// Cancela orden
    async fn cancel_order(&self, order_id: &str) -> Result<()>;
    
    /// Consulta estado de orden
    async fn get_order_status(&self, order_id: &str) -> Result<OrderStatus>;
}

pub struct ExecutedOrder {
    pub original_id: Uuid,
    pub exchange_order_id: String,
    pub filled_quantity: f64,
    pub avg_price: f64,
    pub commission: f64,
    pub executed_at: DateTime<Utc>,
}
```

---

## 📈 Event Types

### MarketDataEvent

```rust
pub enum MarketDataEvent {
    Tick(MarketTick),
    OrderBookSnapshot(OrderBookSnapshot),
    Trade(PublicTrade),
}

pub struct OrderBookSnapshot {
    pub symbol: String,
    pub exchange: Exchange,
    pub bids: Vec<(f64, f64)>,  // (price, quantity)
    pub asks: Vec<(f64, f64)>,
    pub timestamp: DateTime<Utc>,
}
```

### TradingEvent

```rust
pub enum TradingEvent {
    OrderPlaced(InternalOrder),
    OrderFilled(ExecutedOrder),
    OrderRejected { order: InternalOrder, reason: String },
    OrderCancelled(Uuid),
}
```

---

## 🔐 Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum TradingError {
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    #[error("Parse error: {0}")]
    ParseError(#[from] serde_json::Error),
    
    #[error("Risk validation failed: {0}")]
    RiskError(#[from] RiskError),
    
    #[error("Execution error: {0}")]
    ExecutionError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
```

---

## 📝 Notes

- Todos los timestamps están en UTC
- Precios y cantidades usan `f64` (suficiente precisión para crypto)
- IDs de órdenes usan UUIDv4 para idempotencia
- Comunicación interna usa canales Tokio (sin serialización)

---

**Última actualización:** 2026-01-06  
**Mantenido por:** KAIRÓS Team
