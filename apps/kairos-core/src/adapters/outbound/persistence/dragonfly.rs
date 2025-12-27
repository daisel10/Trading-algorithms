// DragonflyDB (Redis-compatible) client for hot data

use redis::{AsyncCommands, Client};

pub struct DragonflyClient {
    client: Client,
}

impl DragonflyClient {
    pub async fn new(url: &str) -> anyhow::Result<Self> {
        let client = Client::open(url)?;
        Ok(Self { client })
    }

    pub async fn publish_market_data(&self, channel: &str, data: &str) -> anyhow::Result<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        conn.publish::<_, _, ()>(channel, data).await?;
        Ok(())
    }

    pub async fn set_key(&self, key: &str, value: &str) -> anyhow::Result<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        conn.set::<_, _, ()>(key, value).await?;
        Ok(())
    }
}
