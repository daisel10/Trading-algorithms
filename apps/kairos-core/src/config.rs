use config::{Config, ConfigError, Environment as ConfigEnvironment, File};
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Environment {
    #[default]
    Development,
    Test,
    Production,
}

impl Environment {
    /// Detect the current environment from APP_ENV or ENVIRONMENT variable
    /// Falls back to Development if not set
    pub fn detect() -> Self {
        std::env::var("APP_ENV")
            .or_else(|_| std::env::var("ENVIRONMENT"))
            .ok()
            .and_then(|env| env.to_lowercase().parse().ok())
            .unwrap_or(Environment::Development)
    }

    /// Get a display name for logging
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "development",
            Environment::Test => "test",
            Environment::Production => "production",
        }
    }
}

impl std::str::FromStr for Environment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dev" | "development" => Ok(Environment::Development),
            "test" | "testing" => Ok(Environment::Test),
            "prod" | "production" => Ok(Environment::Production),
            _ => Err(format!("Unknown environment: {}", s)),
        }
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    #[serde(skip)]
    pub environment: Environment,
    pub rust_log: String,
    pub rust_backtrace: String,
    pub grpc: GrpcSettings,
    pub database: DatabaseSettings,
    pub exchange: ExchangeSettings,
    pub trading: TradingSettings,
    pub performance: PerformanceSettings,
    pub monitoring: MonitoringSettings,
    pub features: FeatureFlags,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GrpcSettings {
    pub port: u16,
    pub host: String,
    pub max_concurrent_streams: u32,
    pub keepalive_interval_sec: u64,
    pub keepalive_timeout_sec: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_sec: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExchangeSettings {
    // Public connection URLs (safe to be in TOML files)
    pub okx_ws_public_url: String,
    pub okx_ws_private_url: String,
    pub okx_ws_business_url: String,
    pub okx_rest_url: String,
    pub ws_reconnect_delay_ms: u64,
    pub ws_max_reconnect_attempts: u32,
    pub ws_ping_interval_sec: u64,

    // API Credentials (ONLY from environment variables, NEVER in TOML)
    // These are optional because they should only be set via .env or environment
    #[serde(default)]
    pub okx_api_key: Option<String>,
    #[serde(default)]
    pub okx_api_secret: Option<String>,
    #[serde(default)]
    pub okx_api_passphrase: Option<String>,

    #[serde(default)]
    pub binance_api_key: Option<String>,
    #[serde(default)]
    pub binance_api_secret: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TradingSettings {
    pub order_timeout_sec: u64,
    pub max_pending_orders: u32,
    pub order_retry_attempts: u32,
    pub max_position_size: f64,
    pub max_leverage: f64,
    pub stop_loss_percentage: f64,
    pub take_profit_percentage: f64,
    pub tick_buffer_size: usize,
    pub orderbook_depth: u32,
    pub kline_intervals: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PerformanceSettings {
    pub tokio_worker_threads: usize,
    pub rayon_num_threads: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MonitoringSettings {
    pub enable_metrics: bool,
    pub metrics_port: u16,
    pub metrics_interval_sec: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FeatureFlags {
    #[serde(default)]
    pub enable_paper_trading: bool,
    #[serde(default)]
    pub enable_debug_endpoints: bool,
    #[serde(default)]
    pub enable_backtesting: bool,
}

impl Settings {
    /// Load configuration from environment-specific .env file and environment variables
    pub fn new() -> Result<Self, ConfigError> {
        Self::new_with_env(Environment::detect())
    }

    /// Load configuration for a specific environment using layered TOML files
    ///
    /// Configuration loading order (each layer overrides the previous):
    /// 1. config/default.toml        - Base configuration (always loaded)
    /// 2. config/{environment}.toml  - Environment-specific overrides
    /// 3. config/local.toml          - Local overrides (optional, gitignored)
    /// 4. Environment variables      - Highest priority overrides
    pub fn new_with_env(env: Environment) -> Result<Self, ConfigError> {
        let run_mode = env.as_str();

        let config = Config::builder()
            // Layer 1: Start with default configuration
            .add_source(File::with_name("config/default").required(true))
            // Layer 2: Add environment-specific configuration
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            // Layer 3: Add local overrides (optional, gitignored)
            .add_source(File::with_name("config/local").required(false))
            // Layer 4: Add environment variable overrides with KAIROS prefix
            // Example: KAIROS__EXCHANGE__OKX_API_KEY=your_key
            .add_source(
                ConfigEnvironment::with_prefix("KAIROS")
                    .prefix_separator("__")
                    .separator("__")
                    .try_parsing(true),
            )
            // Layer 5: Support non-prefixed nested environment variables
            // Example: EXCHANGE_OKX_API_KEY=your_key
            .add_source(
                ConfigEnvironment::default()
                    .separator("_")
                    .try_parsing(true),
            )
            .build()?;

        // Deserialize into our Settings struct
        let mut settings: Settings = config.try_deserialize()?;
        settings.environment = env;
        Ok(settings)
    }

    /// Get gRPC server address
    pub fn grpc_address(&self) -> String {
        format!("{}:{}", self.grpc.host, self.grpc.port)
    }

    /// Get parsed kline intervals
    pub fn get_kline_intervals(&self) -> Vec<String> {
        self.trading
            .kline_intervals
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            environment: Environment::Development,
            rust_log: "info,kairos_core=debug".to_string(),
            rust_backtrace: "1".to_string(),
            grpc: GrpcSettings {
                port: 50051,
                host: "0.0.0.0".to_string(),
                max_concurrent_streams: 100,
                keepalive_interval_sec: 30,
                keepalive_timeout_sec: 10,
            },
            database: DatabaseSettings {
                max_connections: 10,
                min_connections: 2,
                connect_timeout_sec: 5,
            },
            exchange: ExchangeSettings {
                okx_ws_public_url: "wss://ws.okx.com:8443/ws/v5/public".to_string(),
                okx_ws_private_url: "wss://ws.okx.com:8443/ws/v5/private".to_string(),
                okx_ws_business_url: "wss://ws.okx.com:8443/ws/v5/business".to_string(),
                okx_rest_url: "https://www.okx.com".to_string(),
                ws_reconnect_delay_ms: 5000,
                ws_max_reconnect_attempts: 10,
                ws_ping_interval_sec: 20,
                // API credentials default to None (must be set via environment)
                okx_api_key: None,
                okx_api_secret: None,
                okx_api_passphrase: None,
                binance_api_key: None,
                binance_api_secret: None,
            },
            trading: TradingSettings {
                order_timeout_sec: 30,
                max_pending_orders: 100,
                order_retry_attempts: 3,
                max_position_size: 1000.0,
                max_leverage: 3.0,
                stop_loss_percentage: 2.0,
                take_profit_percentage: 5.0,
                tick_buffer_size: 1000,
                orderbook_depth: 20,
                kline_intervals: "1m,5m,15m,1h,4h,1d".to_string(),
            },
            performance: PerformanceSettings {
                tokio_worker_threads: 4,
                rayon_num_threads: 4,
            },
            monitoring: MonitoringSettings {
                enable_metrics: true,
                metrics_port: 9090,
                metrics_interval_sec: 60,
            },
            features: FeatureFlags {
                enable_paper_trading: false,
                enable_debug_endpoints: false,
                enable_backtesting: false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = Settings::default();
        assert_eq!(settings.grpc.port, 50051);
        assert_eq!(settings.grpc.host, "0.0.0.0");
    }

    #[test]
    fn test_grpc_address() {
        let settings = Settings::default();
        assert_eq!(settings.grpc_address(), "0.0.0.0:50051");
    }

    #[test]
    fn test_kline_intervals_parsing() {
        let settings = Settings::default();
        let intervals = settings.get_kline_intervals();
        assert_eq!(intervals, vec!["1m", "5m", "15m", "1h", "4h", "1d"]);
    }
}
