use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a market tick (price update)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTick {
    pub id: Uuid,
    pub symbol: String,
    pub price: f64,
    pub volume: f64,
    pub timestamp: DateTime<Utc>,
    pub exchange: Exchange,
}

/// Supported exchanges
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Exchange {
    Binance,
    OKX,
    Kraken,
}

/// Represents a trading order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub quantity: f64,
    pub price: Option<f64>,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrderType {
    Market,
    Limit,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,
    Approved,
    Rejected,
    Executed,
    Cancelled,
}

/// Internal order representation used by strategies
#[derive(Debug, Clone)]
pub struct InternalOrder {
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: f64,
    pub price: Option<f64>,
    pub risk_score: f64,
}
