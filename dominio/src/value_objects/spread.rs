use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use super::{Currency, Price};

/// Absolute price difference between two legs.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Spread {
    pub amount: Decimal,
    pub currency: Currency,
}

impl Spread {
    pub fn new(amount: impl Into<Decimal>, currency: Currency) -> Self {
        Self { amount: amount.into(), currency }
    }

    pub fn between(bid: Price, ask: Price) -> Self {
        assert_eq!(bid.currency, ask.currency, "Currency mismatch in spread calc");
        Self { amount: ask.amount - bid.amount, currency: bid.currency }
    }

    /// Spread in basis‑points relative to a reference price.
    pub fn basis_points(&self, reference: Price) -> Decimal {
        (self.amount / reference.amount) * Decimal::new(10_000, 0)
    }
}
