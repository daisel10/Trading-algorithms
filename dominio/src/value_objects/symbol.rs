
use serde::{Deserialize, Serialize};

/// Trading pair (e.g., BTC/USD).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Symbol {
    pub base: String,
    pub quote: String,
}

impl Symbol {
    pub fn new(base: impl Into<String>, quote: impl Into<String>) -> Self {
        Self { base: base.into(), quote: quote.into() }
    }

    pub fn as_string(&self) -> String {
        format!("{}/{}", self.base, self.quote)
    }
}
