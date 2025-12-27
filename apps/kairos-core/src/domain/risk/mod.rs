// Risk management module

use kairos_domain::{DomainError, DomainResult, InternalOrder};
use std::sync::atomic::{AtomicU64, Ordering};

/// The Gatekeeper - validates orders before execution
pub struct RiskEngine {
    // Atomic balance in cents to avoid floating point precision issues
    balance_cents: AtomicU64,
    max_daily_risk: f64,
    current_daily_risk: AtomicU64, // in cents
}

impl RiskEngine {
    pub fn new(initial_balance: f64, max_daily_risk: f64) -> Self {
        Self {
            balance_cents: AtomicU64::new((initial_balance * 100.0) as u64),
            max_daily_risk,
            current_daily_risk: AtomicU64::new(0),
        }
    }

    /// Validates an order against risk limits
    pub fn validate_order(&self, order: &InternalOrder) -> DomainResult<()> {
        let balance = self.get_balance();
        let order_value = order.quantity * order.price.unwrap_or(0.0);

        // Check sufficient balance
        if order_value > balance {
            return Err(DomainError::InsufficientBalance {
                required: order_value,
                available: balance,
            });
        }

        // Check daily risk limit
        let current_risk = (self.current_daily_risk.load(Ordering::Relaxed) as f64) / 100.0;
        if current_risk + order.risk_score > self.max_daily_risk {
            return Err(DomainError::RiskLimitExceeded(format!(
                "Daily risk: {}, Order risk: {}",
                current_risk, order.risk_score
            )));
        }

        Ok(())
    }

    /// Updates balance after order execution
    pub fn update_balance(&self, amount: f64) {
        let cents = (amount * 100.0) as i64;
        if cents > 0 {
            self.balance_cents.fetch_add(cents as u64, Ordering::SeqCst);
        } else {
            self.balance_cents
                .fetch_sub((-cents) as u64, Ordering::SeqCst);
        }
    }

    /// Gets current balance
    pub fn get_balance(&self) -> f64 {
        (self.balance_cents.load(Ordering::Relaxed) as f64) / 100.0
    }

    /// Adds to daily risk counter
    pub fn add_risk(&self, risk: f64) {
        let risk_cents = (risk * 100.0) as u64;
        self.current_daily_risk
            .fetch_add(risk_cents, Ordering::SeqCst);
    }

    /// Resets daily risk (should be called daily)
    pub fn reset_daily_risk(&self) {
        self.current_daily_risk.store(0, Ordering::SeqCst);
    }
}
