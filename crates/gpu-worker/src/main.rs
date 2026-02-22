//! GPU Worker Server - gRPC server for Home Server
//!
//! This is the main entry point for the Home Server deployment.
//! It provides gRPC endpoints for GPU-accelerated video transcoding.

use std::net::SocketAddr;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use gpu_worker::server::{GpuWorkerServer, ServerConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting GPU Worker Server");

    // Load configuration
    let config = ServerConfig::from_env()?;
    info!(
        "Configuration: bind={}, max_jobs={}, cuda_device={}",
        config.bind_addr, config.max_concurrent_jobs, config.cuda_device_id
    );

    // Parse bind address
    let bind_addr: SocketAddr = config.bind_addr.parse()?;

    // Create and run server
    let server = GpuWorkerServer::new(bind_addr, config);
    server.run().await?;

    Ok(())
}
