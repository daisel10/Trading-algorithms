---
name: grpc-service
description: >
  Internal API documentation for kairos-core gRPC service, traits, and data structures.
  Trigger: When implementing gRPC interfaces, creating strategies, working with trading engine API, implementing adapters, using domain traits.
license: Apache-2.0
metadata:
  author: kairos-team
  version: "1.0"
---

## When to Use

- Implementing gRPC service interfaces
- Creating new trading strategies
- Implementing feed handlers or execution adapters
- Working with core domain data structures
- Understanding internal API contracts
- Implementing risk validators

## Critical Patterns

### Core Traits

kairos-core defines three main traits that all implementations must follow:

1. **Strategy** - For trading strategies
2. **FeedHandler** - For exchange data feeds
3. **ExecutionAdapter** - For order execution
4. **RiskValidator** - For risk management
5. **PersistenceAdapter** - For data persistence

### Strategy Trait

```rust
#[async_trait]
pub trait Strategy: Send + Sync {
    /// Process market tick and optionally generate order
    async fn on_market_tick(&mut self, tick: MarketTick) -> Option<InternalOrder>;
    
    /// Strategy name
    fn name(&self) -> &str;
    
    /// Initialization (e.g., load state from DB)
    async fn initialize(&mut self) -> Result<()>;
    
    /// Cleanup (e.g., persist state)
    async fn shutdown(&mut self) -> Result<()>;
}
```

**Example Implementation**:

```rust
pub struct TriangularArbitrageStrategy {
    graph: CurrencyGraph,
    min_profit_threshold: f64,
}

#[async_trait]
impl Strategy for TriangularArbitrageStrategy {
    async fn on_market_tick(&mut self, tick: MarketTick) -> Option<InternalOrder> {
        // Update graph with new price
        self.graph.update_edge(&tick.symbol, tick.bid_price, tick.ask_price);
        
        // Detect negative cycle (arbitrage opportunity)
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
        Ok(())
    }
    
    async fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}
```

## Code Examples

### Core Data Structures

**MarketTick**:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTick {
    pub symbol: String,
    pub exchange: Exchange,
    pub bid_price: f64,
    pub ask_price: f64,
    pub bid_volume: f64,
    pub ask_volume: f64,
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

**InternalOrder**:

```rust
#[derive(Debug, Clone)]
pub struct InternalOrder {
    pub id: Uuid,
    pub symbol: String,
    pub exchange: Exchange,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub quantity: f64,
    pub price: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub strategy: StrategyType,
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
    ManualOrder,
}
```

### FeedHandler Trait

```rust
#[async_trait]
pub trait FeedHandler: Send + Sync {
    /// Connect to exchange WebSocket
    async fn connect(&mut self) -> Result<()>;
    
    /// Subscribe to trading pairs
    async fn subscribe(&mut self, symbols: Vec<String>) -> Result<()>;
    
    /// Receive next tick
    async fn next_tick(&mut self) -> Result<MarketTick>;
    
    /// Exchange name
    fn exchange_name(&self) -> &str;
}
```

### RiskValidator Trait

```rust
pub trait RiskValidator: Send + Sync {
    /// Validate if order meets risk requirements
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
        // 1. Check order size
        if order.quantity > self.max_order_size {
            return Err(RiskError::OrderTooLarge);
        }
        
        // 2. Check daily loss limit
        let daily_loss = self.current_daily_loss.load(Ordering::Relaxed);
        if daily_loss > self.daily_loss_limit {
            return Err(RiskError::DailyLossLimitExceeded);
        }
        
        // 3. Check available balance
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

### ExecutionAdapter Trait

```rust
#[async_trait]
pub trait ExecutionAdapter: Send + Sync {
    /// Send order to exchange
    async fn execute_order(&self, order: &InternalOrder) -> Result<ExecutedOrder>;
    
    /// Cancel order
    async fn cancel_order(&self, order_id: &str) -> Result<()>;
    
    /// Query order status
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

### PersistenceAdapter Trait

```rust
#[async_trait]
pub trait PersistenceAdapter: Send + Sync {
    /// Save tick to database
    async fn save_tick(&self, tick: &MarketTick) -> Result<()>;
    
    /// Save executed order
    async fn save_order(&self, order: &ExecutedOrder) -> Result<()>;
    
    /// Batch insert ticks
    async fn save_ticks_batch(&self, ticks: Vec<MarketTick>) -> Result<()>;
}
```

## gRPC Service Definition

### TradingEngine Service

**PlaceOrder**:

```protobuf
message OrderRequest {
  string symbol = 1;
  double quantity = 2;
  OrderType order_type = 3;
  optional double price = 4;
  string client_id = 5;
}

message OrderResponse {
  string order_id = 1;
  OrderStatus status = 2;
  optional string error = 3;
}

enum OrderType {
  MARKET = 0;
  LIMIT = 1;
}

enum OrderStatus {
  PENDING = 0;
  ACCEPTED = 1;
  REJECTED = 2;
  FILLED = 3;
  CANCELLED = 4;
}
```

**Usage Example**:

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
```

## Event Types

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

## Important Notes

- All timestamps are in UTC
- Prices and quantities use `f64` (sufficient precision for crypto)
- Order IDs use UUIDv4 for idempotency
- Internal communication uses Tokio channels (no serialization overhead)

## Resources

- **Source Documentation**: [gRPC Service.md](file:///c:/Users/david/Documents/Trading-algorithms/apps/kairos-core/docs/gRPC%20Service.md)
- **Proto Files**: [libs/kairos-proto/proto/](file:///c:/Users/david/Documents/Trading-algorithms/libs/kairos-proto/proto/)
- **Domain Models**: [src/domain/](file:///c:/Users/david/Documents/Trading-algorithms/apps/kairos-core/src/domain/)
- **Adapters**: [src/adapters/](file:///c:/Users/david/Documents/Trading-algorithms/apps/kairos-core/src/adapters/)
