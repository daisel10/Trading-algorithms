// OKX WebSocket feed handler

use super::error::{FeedError, FeedResult};
use crate::config::Settings;
use kairos_domain::MarketTick;
use tokio::sync::broadcast;

/// OKX API credentials
///
/// This struct holds the API credentials required for authenticated
/// OKX operations. Use `from_settings()` to extract and validate
/// credentials from application settings.
#[derive(Debug, Clone)]
pub struct OkxCredentials {
    pub api_key: String,
    pub api_secret: String,
    pub api_passphrase: Option<String>,
}

impl OkxCredentials {
    /// Extract and validate OKX credentials from Settings
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
    /// let credentials = OkxCredentials::from_settings(&settings)?;
    /// ```
    pub fn from_settings(settings: &Settings) -> FeedResult<Self> {
        let api_key = settings
            .exchange
            .okx_api_key
            .as_ref()
            .ok_or_else(|| FeedError::MissingCredentials {
                exchange: "OKX".to_string(),
            })?
            .clone();

        let api_secret = settings
            .exchange
            .okx_api_secret
            .as_ref()
            .ok_or_else(|| FeedError::MissingCredentials {
                exchange: "OKX".to_string(),
            })?
            .clone();

        // Passphrase is optional depending on API key type
        let api_passphrase = settings.exchange.okx_api_passphrase.clone();

        Ok(Self {
            api_key,
            api_secret,
            api_passphrase,
        })
    }
}

/// OKX configuration (non-sensitive)
///
/// This struct holds non-sensitive configuration like WebSocket URLs.
#[derive(Debug, Clone)]
pub struct OkxConfig {
    pub ws_url: String,
}

impl OkxConfig {
    /// Extract OKX configuration from Settings
    ///
    /// # Arguments
    /// * `settings` - Application settings containing connection URLs
    ///
    /// # Example
    /// ```rust,ignore
    /// let settings = Settings::new()?;
    /// let config = OkxConfig::from_settings(&settings);
    /// ```
    pub fn from_settings(settings: &Settings) -> Self {
        Self {
            ws_url: settings.exchange.okx_ws_public_url.clone(),
        }
    }
}

pub struct OkxFeedHandler {
    market_data_tx: broadcast::Sender<MarketTick>,
    api_key: String,
    api_secret: String,
    api_passphrase: Option<String>,
    ws_url: String,
}

impl OkxFeedHandler {
    /// Create a new OKX feed handler with API credentials
    ///
    /// # Arguments
    /// * `credentials` - Validated OKX API credentials
    /// * `config` - OKX configuration (WebSocket URL, etc.)
    /// * `market_data_tx` - Broadcast channel for market data
    ///
    /// # Returns
    /// * Successfully created handler with credentials
    ///
    /// # Example
    /// ```rust,ignore
    /// let settings = Settings::new()?;
    /// let credentials = OkxCredentials::from_settings(&settings)?;
    /// let config = OkxConfig::from_settings(&settings);
    /// let handler = OkxFeedHandler::new(credentials, config, market_data_tx);
    /// ```
    pub fn new(
        credentials: OkxCredentials,
        config: OkxConfig,
        market_data_tx: broadcast::Sender<MarketTick>,
    ) -> Self {
        tracing::info!(
            "✅ OKX Feed Handler initialized with API credentials (key: {}...)",
            &credentials.api_key[..credentials.api_key.len().min(8)]
        );

        Self {
            market_data_tx,
            api_key: credentials.api_key,
            api_secret: credentials.api_secret,
            api_passphrase: credentials.api_passphrase,
            ws_url: config.ws_url,
        }
    }

    /// Create a public-only handler (no authentication, public endpoints only)
    ///
    /// Use this for market data that doesn't require authentication
    ///
    /// # Arguments
    /// * `config` - OKX configuration (WebSocket URL, etc.)
    /// * `market_data_tx` - Broadcast channel for market data
    ///
    /// # Example
    /// ```rust,ignore
    /// let settings = Settings::new()?;
    /// let config = OkxConfig::from_settings(&settings);
    /// let handler = OkxFeedHandler::new_public(config, market_data_tx);
    /// ```
    pub fn new_public(config: OkxConfig, market_data_tx: broadcast::Sender<MarketTick>) -> Self {
        tracing::info!("OKX Feed Handler initialized in PUBLIC mode (no authentication)");

        Self {
            market_data_tx,
            api_key: String::new(),
            api_secret: String::new(),
            api_passphrase: None,
            ws_url: config.ws_url,
        }
    }

    pub async fn start(&self) -> FeedResult<()> {
        // TODO: Implement WebSocket connection to OKX
        // 1. Connect to self.ws_url
        // 2. Authenticate using api_key, api_secret, api_passphrase (if needed)
        // 3. Subscribe to market data streams
        // 4. Parse incoming JSON
        // 5. Convert to MarketTick
        // 6. Broadcast to channel

        tracing::info!("🚀 OKX feed handler started (URL: {})", self.ws_url);
        tracing::debug!(
            "Using API key: {}...",
            &self.api_key[..self.api_key.len().min(8)]
        );

        Ok(())
    }
}
