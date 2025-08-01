
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// ISO‑like currency codes supported so far.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
    USD,
    USDT,
    EUR,
    BTC,
    ETH,
}

/// Immutable monetary amount tagged with a [`Currency`].
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Money {
    pub amount: Decimal,
    pub currency: Currency,
}

impl Money {
    pub fn new(amount: impl Into<Decimal>, currency: Currency) -> Self {
        Self {
            amount: amount.into(),
            currency,
        }
    }
}