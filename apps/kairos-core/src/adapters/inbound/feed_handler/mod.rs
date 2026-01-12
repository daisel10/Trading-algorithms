// Feed Handler - WebSocket connections to exchanges

pub mod binance;
pub mod error;
pub mod okx;

// Re-export credential structs for convenience
// pub use binance::BinanceCredentials;
pub use okx::{OkxConfig, OkxCredentials};

// Re-export error types
pub use error::{FeedError, FeedResult};

// TODO: Implement WebSocket clients for each exchange
// These will listen to market data and publish to broadcast channel
