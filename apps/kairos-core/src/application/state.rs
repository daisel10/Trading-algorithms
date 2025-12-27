// Global application state

use crate::domain::risk::RiskEngine;
use std::sync::Arc;

/// Shared application state
pub struct AppState {
    pub risk_engine: Arc<RiskEngine>,
    // Add more shared state as needed
    // pub order_book: Arc<OrderBook>,
    // pub market_data: Arc<MarketDataStore>,
}

impl AppState {
    pub fn new(initial_balance: f64, max_daily_risk: f64) -> Self {
        Self {
            risk_engine: Arc::new(RiskEngine::new(initial_balance, max_daily_risk)),
        }
    }
}
