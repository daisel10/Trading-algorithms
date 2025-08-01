
use crate::value_objects::{Instrument, Quantity, Price};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: OrderId,
    pub instrument: Instrument,
    pub side: OrderSide,
    pub quantity: Quantity,
    pub price: Price,
    pub status: OrderStatus,
}

impl Order {
    pub fn new(
        instrument: Instrument,
        side: OrderSide,
        quantity: Quantity,
        price: Price,
    ) -> Self {
        Self {
            id: OrderId(Uuid::new_v4()),
            instrument,
            side,
            quantity,
            price,
            status: OrderStatus::New,
        }
    }

    pub fn mark_filled(&mut self) {
        self.status = OrderStatus::Filled;
    }
}
