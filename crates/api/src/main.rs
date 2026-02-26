//! API Server - Axum HTTP server for video downloader
//!
//! This is the main entry point for the API deployment.
//! It provides HTTP endpoints for video extraction and download.

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::path::Path;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod config;
mod routes;

use config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Load configuration
    let config = Config::from_env()?;
    let extractor_path = Path::new(&config.extractor_dir);
    if extractor_path.is_file() {
        info!(
            "Initializing extractor pool from external bundle {}",
            config.extractor_dir
        );
        extractor::init(Some(extractor_path)).await?;
    } else {
        info!(
            "Extractor path '{}' is not a file; using embedded extractor bundle",
            config.extractor_dir
        );
        extractor::init(None).await?;
    }

    info!("Starting API server on port {}", config.port);

    // Build router with all routes
    let app = Router::new()
        .route("/health", get(routes::health_check))
        .route("/openapi.json", get(routes::openapi_handler))
        .route("/bm.js", get(routes::bm_js_handler))
        .route("/userscript", get(routes::userscript_handler))
        .route("/api/extract", post(routes::extract_handler))
        .route("/api/stream", get(routes::stream_handler))
        .route("/api/stream/muxed", get(routes::muxed_stream_handler))
        .route("/api/batch", get(routes::batch_handler))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("API server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
