use tracing::{error, info};
use tracing_subscriber;

mod adapters;
mod application;
mod config;
mod domain;

use config::Settings;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Load configuration
    let settings =
        Settings::new().map_err(|e| anyhow::anyhow!("Failed to load configuration: {}", e))?;

    // Initialize tracing with configured log level
    std::env::set_var("RUST_LOG", &settings.rust_log);
    std::env::set_var("RUST_BACKTRACE", &settings.rust_backtrace);

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    info!("🚀 Starting KAIRÓS Trading Core...");
    info!("⚡ Initializing Tokio Runtime");
    info!("🌍 Environment: {}", settings.environment);
    info!("📋 Configuration loaded successfully");
    info!("   gRPC Server: {}", settings.grpc_address());
    info!("   Exchange: {}", settings.exchange.okx_ws_public_url);
    info!(" okx_api_key: {:?}", settings.exchange.okx_api_key);
    // Ejemplo: Leer una variable directamente del .env (no de Settings)
    if let Ok(custom_var) = std::env::var("CUSTOM_VAR") {
        info!("📌 CUSTOM_VAR from .env: {}", custom_var);
    }

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
    tokio::signal::ctrl_c().await?;
    info!("🛑 Shutting down KAIRÓS Core...");

    Ok(())
}
