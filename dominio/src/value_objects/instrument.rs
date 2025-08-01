
use serde::{Deserialize, Serialize};
use super::symbol::Symbol;

/// Fully‑qualified instrument, possibly including venue (exchange/broker).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Instrument {
    pub symbol: Symbol,
    /// Optional venue identifier, e.g. "OKX" or "IBKR".
    pub venue: Option<String>,
}

impl Instrument {
    pub fn new(symbol: Symbol, venue: Option<impl Into<String>>) -> Self {
        Self { symbol, venue: venue.map(Into::into) }
    }

    pub fn full_code(&self) -> String {
        match &self.venue {
            Some(v) => format!("{}@{}", self.symbol.as_string(), v),
            None => self.symbol.as_string(),
        }
    }
}