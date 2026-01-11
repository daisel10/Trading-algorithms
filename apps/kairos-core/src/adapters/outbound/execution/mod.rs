// Execution layer - sends orders to exchanges

pub mod binance;
pub mod error;
pub mod okx;

// Re-export error types
pub use error::{ExecutionError, ExecutionResult};
