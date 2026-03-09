use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use job_system::JobRepository;
use queue::redis_streams::RedisStreamsQueue;
use queue::{JobQueueConsumer, JobQueuePublisher};
use sqlx::postgres::PgPoolOptions;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

mod job_runner;
mod mux_pipeline;
mod storage_factory;
mod worker_config;

use job_runner::{cleanup_expired_artifacts, republish_reclaimed_jobs, run_claimed_job};
use storage_factory::{build_storage_backend, init_extractor_bundle};
use worker_config::WorkerConfig;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish(),
    )?;

    let config = WorkerConfig::from_env()?;
    init_extractor_bundle(&config.extractor_dir).await?;

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;
    sqlx::migrate!("../api/migrations").run(&db_pool).await?;

    let repo = JobRepository::new(db_pool);
    let queue = Arc::new(RedisStreamsQueue::new(
        &config.redis_url,
        &config.queue_stream,
        &config.queue_group,
        &config.worker_id,
    )?);
    let queue_publisher: Arc<dyn JobQueuePublisher> = queue.clone();
    let queue_consumer: Arc<dyn JobQueueConsumer> = queue.clone();
    queue_consumer.ensure_group().await?;

    let storage = build_storage_backend(&config).await?;
    tokio::fs::create_dir_all(Path::new(&config.artifact_dir)).await?;

    info!(
        worker_id = config.worker_id,
        artifact_backend = config.artifact_backend,
        queue_stream = config.queue_stream,
        "Mux worker started"
    );
    let mut last_cleanup_at =
        std::time::Instant::now() - Duration::from_secs(config.cleanup_interval_secs.max(1) as u64);

    loop {
        if let Err(error) = republish_reclaimed_jobs(&repo, queue_publisher.clone(), &config).await
        {
            error!(err = %error, "Failed to republish reclaimed jobs");
        }
        if last_cleanup_at.elapsed()
            >= Duration::from_secs(config.cleanup_interval_secs.max(1) as u64)
        {
            if let Err(error) =
                cleanup_expired_artifacts(&repo, storage.clone(), config.cleanup_batch_limit).await
            {
                error!(err = %error, "Failed to clean expired mux artifacts");
            }
            last_cleanup_at = std::time::Instant::now();
        }

        match queue_consumer.consume(1_000).await {
            Ok(Some(claimed)) => {
                let should_ack = run_claimed_job(
                    &repo,
                    queue_publisher.clone(),
                    storage.clone(),
                    &config,
                    claimed.clone(),
                )
                .await?;
                if should_ack {
                    queue_consumer.ack(&claimed.stream_id).await?;
                }
            }
            Ok(None) => tokio::time::sleep(Duration::from_millis(250)).await,
            Err(error) => {
                error!(err = %error, "Worker consume loop failed");
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    }
}
