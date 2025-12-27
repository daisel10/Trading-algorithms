use axum::{Router, routing::get};
use tower_http::cors::CorsLayer;
use tracing::{info, error};

mod graphql;
mod clients;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    info!("🚀 Starting KAIRÓS API Gateway...");

    // TODO: Initialize clients
    // 1. Connect to DragonflyDB for reading market data
    // 2. Connect to gRPC server (kairos-core) for placing orders

    // Build GraphQL schema
    let schema = graphql::build_schema().await;

    // Build routes
    let app = Router::new()
        .route("/", get(|| async { "KAIRÓS API Gateway" }))
        .route("/graphql", get(graphql::graphql_playground).post(graphql::graphql_handler))
        .layer(CorsLayer::permissive())
        .with_state(schema);

    let addr = "0.0.0.0:4000";
    info!("🌐 GraphQL Playground available at http://{}/graphql", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
