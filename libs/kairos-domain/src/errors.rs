use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Invalid order: {0}")]
    InvalidOrder(String),

    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: f64, available: f64 },

    #[error("Risk limit exceeded: {0}")]
    RiskLimitExceeded(String),

    #[error("Invalid market data: {0}")]
    InvalidMarketData(String),

    #[error("Exchange error: {0}")]
    ExchangeError(String),

    #[error("Order validation failed: {0}")]
    ValidationFailed(String),

    #[error("Symbol '{symbol}' not found")]
    SymbolNotFound { symbol: String },

    #[error("Invalid price: {0}")]
    InvalidPrice(String),

    #[error("Invalid quantity: {0}")]
    InvalidQuantity(String),
}

pub type DomainResult<T> = Result<T, DomainError>;
