use thiserror::Error;

/// Execution layer errors
#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("Failed to place order on {exchange}: {reason}")]
    OrderFailed { exchange: String, reason: String },
    
    #[error("Failed to cancel order '{order_id}': {reason}")]
    CancelFailed { order_id: String, reason: String },
    
    #[error("Invalid order: {0}")]
    InvalidOrder(String),
    
    #[error("Authentication failed for {exchange}")]
    AuthenticationFailed { exchange: String },
    
    #[error("HTTP request failed: {0}")]
    HttpError(String),
    
    #[error("Rate limit exceeded on {exchange}")]
    RateLimitExceeded { exchange: String },
    
    #[error("Order timeout: {0}")]
    OrderTimeout(String),
}

pub type ExecutionResult<T> = Result<T, ExecutionError>;
