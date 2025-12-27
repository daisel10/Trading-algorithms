use kairos_domain::{InternalOrder, MarketTick};

/// Arbitrage strategy - detects price differences across exchanges
pub struct ArbitrageStrategy {
    min_profit_threshold: f64,
}

impl ArbitrageStrategy {
    pub fn new(min_profit_threshold: f64) -> Self {
        Self {
            min_profit_threshold,
        }
    }

    /// Analyzes market ticks to find arbitrage opportunities
    pub fn analyze(&self, _ticks: &[MarketTick]) -> Option<InternalOrder> {
        // TODO: Implement arbitrage logic
        // 1. Compare prices across exchanges
        // 2. Calculate profit potential
        // 3. If profit > threshold, return order
        None
    }
}
