// Binance WebSocket feed handler

use crate::config::Settings;
use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use futures::StreamExt;
use kairos_domain::{Exchange, MarketTick};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use uuid::Uuid;

/// Binance API credentials
///
/// This struct holds the API credentials required for authenticated
/// Binance operations. Use `from_settings()` to extract and validate
/// credentials from application settings.
#[derive(Debug, Clone)]
pub struct BinanceCredentials {
    pub api_key: String,
    pub api_secret: String,
}

impl BinanceCredentials {
    /// Extract and validate Binance credentials from Settings
    ///
    /// # Arguments
    /// * `settings` - Application settings containing API credentials
    ///
    /// # Returns
    /// * `Ok(Self)` - Successfully extracted valid credentials
    /// * `Err` - If required credentials are not set in environment
    ///
    /// # Example
    /// ```rust,ignore
    /// let settings = Settings::new()?;
    /// let credentials = BinanceCredentials::from_settings(&settings)?;
    /// ```
    pub fn from_settings(settings: &Settings) -> Result<Self> {
        let api_key = settings
            .exchange
            .binance_api_key
            .as_ref()
            .ok_or_else(|| {
                anyhow!(
                    "KAIROS__EXCHANGE__BINANCE_API_KEY not set in environment. \
                Add it to your .env file to enable Binance trading."
                )
            })?
            .clone();

        let api_secret = settings
            .exchange
            .binance_api_secret
            .as_ref()
            .ok_or_else(|| {
                anyhow!(
                    "KAIROS__EXCHANGE__BINANCE_API_SECRET not set in environment. \
                Add it to your .env file to enable Binance trading."
                )
            })?
            .clone();

        Ok(Self {
            api_key,
            api_secret,
        })
    }
}

/// Binance trade stream message structure
#[derive(Debug, Deserialize)]
struct BinanceTradeMessage {
    #[serde(rename = "e")]
    event_type: String, // Event type (e.g., "trade")
    #[serde(rename = "s")]
    symbol: String, // Trading pair (e.g., "BTCUSDT")
    #[serde(rename = "p")]
    price: String, // Trade price
    #[serde(rename = "q")]
    quantity: String, // Trade quantity
    #[serde(rename = "T")]
    trade_time: i64, // Trade timestamp
}

/// Binance aggregated trade stream message
#[derive(Debug, Deserialize)]
struct BinanceAggTradeMessage {
    #[serde(rename = "e")]
    event_type: String,
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "p")]
    price: String,
    #[serde(rename = "q")]
    quantity: String,
    #[serde(rename = "T")]
    trade_time: i64,
}

/// Subscribe message structure for Binance WebSocket
#[derive(Debug, Serialize)]
struct SubscribeMessage {
    method: String,
    params: Vec<String>,
    id: u64,
}

pub struct BinanceFeedHandler {
    market_data_tx: broadcast::Sender<MarketTick>,
    symbols: Vec<String>,
    api_key: String,
    api_secret: String,
}

impl BinanceFeedHandler {
    /// Create a new Binance feed handler with API credentials
    ///
    /// # Arguments
    /// * `credentials` - Validated Binance API credentials
    /// * `market_data_tx` - Broadcast channel sender for market data
    /// * `symbols` - Optional list of trading pairs (e.g., ["btcusdt", "ethusdt"])
    ///               If None, defaults to ["btcusdt", "ethusdt"]
    ///
    /// # Returns
    /// * Successfully created handler with credentials
    ///
    /// # Example
    /// ```rust,ignore
    /// let settings = Settings::new()?;
    /// let credentials = BinanceCredentials::from_settings(&settings)?;
    /// let handler = BinanceFeedHandler::new(credentials, market_data_tx, None);
    /// ```
    pub fn new(
        credentials: BinanceCredentials,
        market_data_tx: broadcast::Sender<MarketTick>,
        symbols: Option<Vec<String>>,
    ) -> Self {
        let symbols = symbols.unwrap_or_else(|| vec!["btcusdt".to_string(), "ethusdt".to_string()]);

        tracing::info!(
            "✅ Binance Feed Handler initialized with API credentials (key: {}...)",
            &credentials.api_key[..credentials.api_key.len().min(8)]
        );

        Self {
            market_data_tx,
            symbols,
            api_key: credentials.api_key,
            api_secret: credentials.api_secret,
        }
    }

    /// Create a public-only handler (no authentication, public streams only)
    ///
    /// Use this for market data that doesn't require authentication
    ///
    /// # Arguments
    /// * `market_data_tx` - Broadcast channel sender for market data
    /// * `symbols` - Optional list of trading pairs (e.g., ["btcusdt", "ethusdt"])
    ///               If None, defaults to ["btcusdt", "ethusdt"]
    pub fn new_public(
        market_data_tx: broadcast::Sender<MarketTick>,
        symbols: Option<Vec<String>>,
    ) -> Self {
        let symbols = symbols.unwrap_or_else(|| vec!["btcusdt".to_string(), "ethusdt".to_string()]);

        tracing::info!("Binance Feed Handler initialized in PUBLIC mode (no authentication)");

        Self {
            market_data_tx,
            symbols,
            api_key: String::new(),
            api_secret: String::new(),
        }
    }

    /// Start the WebSocket connection and begin streaming market data
    pub async fn start(&self) -> Result<()> {
        loop {
            match self.connect_and_stream().await {
                Ok(_) => {
                    tracing::warn!("Binance WebSocket connection closed normally, reconnecting...");
                }
                Err(e) => {
                    tracing::error!("Binance WebSocket error: {:?}, reconnecting in 5s...", e);
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }

    /// Internal method to handle connection and streaming
    async fn connect_and_stream(&self) -> Result<()> {
        // Build WebSocket URL for combined streams
        let streams: Vec<String> = self
            .symbols
            .iter()
            .map(|s| format!("{}@aggTrade", s.to_lowercase()))
            .collect();

        let stream_names = streams.join("/");
        let ws_url = format!(
            "wss://stream.binance.com:9443/stream?streams={}",
            stream_names
        );

        tracing::info!("Connecting to Binance WebSocket: {}", ws_url);

        let (ws_stream, _) = connect_async(&ws_url)
            .await
            .context("Failed to connect to Binance WebSocket")?;

        tracing::info!(
            "✅ Connected to Binance WebSocket, subscribed to {} symbols: {:?}",
            self.symbols.len(),
            self.symbols
        );

        let (mut _write, mut read) = ws_stream.split();

        // Process incoming messages
        while let Some(message) = read.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    if let Err(e) = self.process_message(&text).await {
                        tracing::warn!("Failed to process message: {:?}", e);
                    }
                }
                Ok(Message::Ping(ping)) => {
                    tracing::debug!("Received ping: {:?}", ping);
                }
                Ok(Message::Pong(_)) => {
                    tracing::debug!("Received pong");
                }
                Ok(Message::Close(frame)) => {
                    tracing::info!("WebSocket closed: {:?}", frame);
                    break;
                }
                Err(e) => {
                    tracing::error!("WebSocket error: {:?}", e);
                    return Err(e.into());
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Process a single message from Binance WebSocket
    async fn process_message(&self, text: &str) -> Result<()> {
        // Binance combined streams wrap messages in a data field
        #[derive(Debug, Deserialize)]
        struct StreamWrapper {
            stream: String,
            data: serde_json::Value,
        }

        let wrapper: StreamWrapper =
            serde_json::from_str(text).context("Failed to parse stream wrapper")?;

        // Parse the aggregated trade message
        let agg_trade: BinanceAggTradeMessage = serde_json::from_value(wrapper.data)
            .context("Failed to parse aggregated trade message")?;

        // Convert to MarketTick
        let market_tick = self.convert_to_market_tick(agg_trade)?;

        // Broadcast to all subscribers
        let _ = self.market_data_tx.send(market_tick.clone());

        tracing::debug!(
            "📊 Market tick: {} @ ${} (vol: {})",
            market_tick.symbol,
            market_tick.price,
            market_tick.volume
        );

        Ok(())
    }

    /// Convert Binance message to internal MarketTick format
    fn convert_to_market_tick(&self, msg: BinanceAggTradeMessage) -> Result<MarketTick> {
        let price = msg.price.parse::<f64>().context("Failed to parse price")?;
        let volume = msg
            .quantity
            .parse::<f64>()
            .context("Failed to parse quantity")?;

        Ok(MarketTick {
            id: Uuid::new_v4(),
            symbol: msg.symbol,
            price,
            volume,
            timestamp: Utc::now(),
            exchange: Exchange::Binance,
        })
    }
}
