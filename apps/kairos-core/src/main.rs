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
    let settings =
        Settings::new()
        .context("Failed to initialize system config")?;
        // .map_err(|e| anyhow::anyhow!("Failed to initialize system config: {}", e))?;

    // Initialize hybrid logging (console + file)
    let _guard = logging::init_logging(&settings)
        .context("Failed to initialize logging system")?;
        // .map_err(|e| anyhow::anyhow!("Failed to initialize logging system: {}", e))?;

    // Clean up old log files
    if let Err(e) = logging::cleanup_old_logs(&settings.logging) {
        tracing::warn!("Failed to cleanup old logs: {}", e);
    }

    info!("🚀 Starting KAIRÓS Trading Core...");
    info!("⚡ Initializing Tokio Runtime");
    info!("🌍 Environment: {}", settings.environment);
    info!("📋 Configuration loaded successfully");

    // TODO: Initialize components
    // 1. Create broadcast channel for market data
    // 2. Start Feed Handler (The Feed Handler)
    // 3. Start Persistence Layer (The Logger)
    // 4. Start Strategies (The Sprinters)
    // 5. Create MPSC channel for orders
    // 6. Start Risk Engine (The Gatekeeper)
    // 7. Start Execution Engine (The Sniper)
    // 8. Start gRPC Server for external communication

    info!("✅ KAIRÓS Core initialized successfully");
    info!("📡 Listening for market data...");

    // Keep the main task alive
    tokio::signal::ctrl_c()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to listen for shutdown signal: {}", e))?;
    info!("🛑 Shutting down KAIRÓS Core...");

    Ok(())
}
