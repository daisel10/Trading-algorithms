// DragonflyDB reader for market data

use redis::{Client, AsyncCommands};

pub struct DbReader {
    client: Client,
}

impl DbReader {
    pub async fn new(url: &str) -> anyhow::Result<Self> {
        let client = Client::open(url)?;
        Ok(Self { client })
    }

    pub async fn get_balance(&self, currency: &str) -> anyhow::Result<f64> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let key = format!("balance:{}", currency);
        let balance: Option<String> = conn.get(&key).await?;
        Ok(balance.and_then(|b| b.parse().ok()).unwrap_or(0.0))
    }

    pub async fn get_latest_price(&self, symbol: &str) -> anyhow::Result<Option<f64>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let key = format!("price:{}", symbol);
        let price: Option<String> = conn.get(&key).await?;
        Ok(price.and_then(|p| p.parse().ok()))
    }
}
