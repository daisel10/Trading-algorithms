// DragonflyDB (Redis-compatible) client for hot data

use super::error::{PersistenceError, PersistenceResult};
use redis::{AsyncCommands, Client};

pub struct DragonflyClient {
    client: Client,
}

impl DragonflyClient {
    pub async fn new(url: &str) -> PersistenceResult<Self> {
        let client = Client::open(url).map_err(|e| {
            PersistenceError::ConnectionFailed(format!(
                "Failed to connect to Dragonfly at {}: {}",
                url, e
            ))
        })?;
        Ok(Self { client })
    }

    pub async fn publish_market_data(&self, channel: &str, data: &str) -> PersistenceResult<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        conn.publish::<_, _, ()>(channel, data).await.map_err(|e| {
            PersistenceError::PublishFailed {
                channel: channel.to_string(),
                reason: e.to_string(),
            }
        })?;
        Ok(())
    }

    pub async fn set_key(&self, key: &str, value: &str) -> PersistenceResult<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        conn.set::<_, _, ()>(key, value)
            .await
            .map_err(|e| PersistenceError::CacheFailed {
                key: key.to_string(),
                reason: e.to_string(),
            })?;
        Ok(())
    }
}
