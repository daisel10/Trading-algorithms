use thiserror::Error;

/// Persistence layer errors
#[derive(Error, Debug)]
pub enum PersistenceError {
    #[error("Database connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Failed to save market tick: {0}")]
    SaveFailed(String),

    #[error("Failed to publish to Redis channel '{channel}': {reason}")]
    PublishFailed { channel: String, reason: String },

    #[error("Failed to set cache key '{key}': {reason}")]
    CacheFailed { key: String, reason: String },

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),
}

pub type PersistenceResult<T> = Result<T, PersistenceError>;
