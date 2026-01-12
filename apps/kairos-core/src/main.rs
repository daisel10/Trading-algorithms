use tracing::info;

mod adapters;
mod application;
mod config;
mod domain;
mod logging;

use anyhow::Context;
use config::Settings;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Load configuration
    let settings = Settings::new().context("Failed to initialize system config")?;
    // .map_err(|e| anyhow::anyhow!("Failed to initialize system config: {}", e))?;

    // Initialize hybrid logging (console + file)
    let _guard = logging::init_logging(&settings).context("Failed to initialize logging system")?;
    // .map_err(|e| anyhow::anyhow!("Failed to initialize logging system: {}", e))?;

    // Clean up old log files
    if let Err(e) = logging::cleanup_old_logs(&settings.logging) {
        tracing::warn!("Failed to cleanup old logs: {}", e);
    }

    info!("🚀 Starting KAIRÓS Trading Core...");
    info!("⚡ Initializing Tokio Runtime");
    info!("🌍 Environment: {}", settings.environment);
    info!("📋 Configuration loaded successfully");

    // 1. Create broadcast channel for market data
    info!("📡 Creating market data broadcast channel...");
    let (market_data_tx, _market_data_rx) =
        tokio::sync::broadcast::channel::<kairos_domain::MarketTick>(1000);

    // 2. Configure symbols to track
    let symbols = vec!["btcusdt".to_string(), "ethusdt".to_string()];

    // 3. Start Binance Feed Handler (The Feed Handler)
    info!("🔌 Initializing Binance WebSocket feed handler...");
    let binance_feed = adapters::inbound::feed_handler::binance::BinanceFeedHandler::new_public(
        market_data_tx.clone(),
        Some(symbols.clone()),
    );

    // Spawn feed handler task
    let feed_task = tokio::spawn({
        async move {
            info!("🚀 Starting Binance feed handler...");
            if let Err(e) = binance_feed.start().await {
                tracing::error!("❌ Binance feed handler error: {:?}", e);
            }
        }
    });

    // 4. Start consumer task to display real-time prices
    let price_monitor_task = tokio::spawn({
        let mut rx = market_data_tx.subscribe();
        async move {
            info!("👁️  Starting price monitor...");
            while let Ok(tick) = rx.recv().await {
                info!(
                    "📊 {} | ${:.2} | Vol: {:.4} | {}",
                    tick.symbol,
                    tick.price,
                    tick.volume,
                    tick.timestamp.format("%H:%M:%S%.3f")
                );
            }
        }
    });

    // TODO: Continue with remaining components
    // 3. Start Persistence Layer (The Logger)
    // 4. Start Strategies (The Sprinters)
    // 5. Create MPSC channel for orders
    // 6. Start Risk Engine (The Gatekeeper)
    // 7. Start Execution Engine (The Sniper)
    // 8. Start gRPC Server for external communication

    info!("✅ KAIRÓS Core initialized successfully");
    info!("📡 Listening for market data from Binance...");
    info!("🎯 Tracking symbols: {:?}", symbols);

    // Keep the main task alive
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("🛑 Received shutdown signal, shutting down KAIRÓS Core...");
        }
        _ = feed_task => {
            tracing::error!("Feed handler task terminated unexpectedly");
        }
        _ = price_monitor_task => {
            tracing::error!("Price monitor task terminated unexpectedly");
        }
    }

    Ok(())
}
