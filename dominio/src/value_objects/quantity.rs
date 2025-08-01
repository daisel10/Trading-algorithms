
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub, Mul, Div};

/// Positive (or zero) trade size.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Quantity(pub Decimal);

impl Quantity {
    pub fn new(amount: impl Into<Decimal>) -> Self {
        Self(amount.into())
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }
}

impl Add for Quantity {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Quantity {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul<Decimal> for Quantity {
    type Output = Self;
    fn mul(self, rhs: Decimal) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Div<Decimal> for Quantity {
    type Output = Self;
    fn div(self, rhs: Decimal) -> Self::Output {
        Self(self.0 / rhs)
    }
}
