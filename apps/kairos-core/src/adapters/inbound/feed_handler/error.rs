use thiserror::Error;

/// Feed handler errors
#[derive(Error, Debug)]
pub enum FeedError {
    #[error("WebSocket connection failed")]
    ConnectionFailed(#[source] tokio_tungstenite::tungstenite::Error),

    #[error("Failed to subscribe to {symbol}: {reason}")]
    SubscriptionFailed { symbol: String, reason: String },

    #[error("Failed to parse message")]
    ParseError(#[source] serde_json::Error),

    #[error("Invalid market data: {0}")]
    InvalidData(String),

    #[error("Missing API credentials for {exchange}")]
    MissingCredentials { exchange: String },

    #[error("WebSocket error")]
    WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("JSON error")]
    JsonError(#[from] serde_json::Error),

    #[error("Failed to parse number: {field}")]
    NumberParseError {
        field: String,
        #[source]
        source: std::num::ParseFloatError,
    },
}

pub type FeedResult<T> = Result<T, FeedError>;
