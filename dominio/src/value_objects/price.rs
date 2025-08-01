use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use super::{Currency, Money, Quantity};

/// Unit price of an `Instrument` in a given `Currency`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Price {
    pub amount: Decimal,
    pub currency: Currency,
}

impl Price {
    pub fn new(amount: impl Into<Decimal>, currency: Currency) -> Self {
        Self { amount: amount.into(), currency }
    }

    /// Convert *price × quantity* into a [`Money`] value.
    pub fn total(self, qty: Quantity) -> Money {
        Money::new(self.amount * qty.0, self.currency)
    }
}