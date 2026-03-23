use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use job_system::JobRepository;
use proxy::init_global_proxy_pool;
use queue::redis_streams::RedisStreamsQueue;
use queue::{JobQueueConsumer, JobQueuePublisher};
use sqlx::postgres::PgPoolOptions;
use tokio::task::JoinSet;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

mod job_progress_publisher;
mod job_runner;
mod mux_pipeline;
mod storage_factory;
mod worker_config;

use job_runner::{cleanup_expired_artifacts, republish_reclaimed_jobs, run_claimed_job};
use job_system::JobProgressStore;
use storage_factory::{build_storage_backend, init_extractor_bundle};
use worker_config::WorkerConfig;

async fn run_app_migrations(pool: &sqlx::PgPool) -> Result<()> {
    let mut migrator = sqlx::migrate::Migrator::new(resolve_app_migrations_dir()).await?;
    migrator.set_ignore_missing(true);
    migrator.run(pool).await?;
    Ok(())
}

fn resolve_app_migrations_dir() -> PathBuf {
    let compile_time_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../api/app-migrations");
    if compile_time_dir.exists() {
        return compile_time_dir;
    }

    let runtime_dir = PathBuf::from("/app/app-migrations");
    if runtime_dir.exists() {
        return runtime_dir;
    }

    if let Ok(executable_path) = std::env::current_exe() {
        if let Some(bin_dir) = executable_path.parent() {
            let sibling_dir = bin_dir.join("app-migrations");
            if sibling_dir.exists() {
                return sibling_dir;
            }
        }
    }

    compile_time_dir
}

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
    run_app_migrations(&db_pool).await?;
    let proxy_db_pool = if config.proxy_database_url == config.database_url {
        db_pool.clone()
    } else {
        PgPoolOptions::new()
            .max_connections(3)
            .connect(&config.proxy_database_url)
            .await?
    };
    proxy::ensure_proxy_schema(&proxy_db_pool).await?;
    init_global_proxy_pool(
        proxy_db_pool,
        &config.proxy_redis_url,
        config.proxy_quarantine_ttl_secs,
    )
    .await?;

    let repo = Arc::new(JobRepository::new(db_pool));
    let queue = Arc::new(RedisStreamsQueue::new(
        &config.redis_url,
        &config.queue_stream,
        &config.queue_group,
        &config.worker_id,
    )?);
    let queue_publisher: Arc<dyn JobQueuePublisher> = queue.clone();
    let queue_consumer: Arc<dyn JobQueueConsumer> = queue.clone();
    queue_consumer.ensure_group().await?;
    let progress_store = Arc::new(JobProgressStore::new(&config.redis_url)?);

    let storage = build_storage_backend(&config).await?;

    info!(
        worker_id = config.worker_id,
        concurrency = config.concurrency,
        queue_stream = config.queue_stream,
        "Mux worker started"
    );
    let mut last_cleanup_at =
        std::time::Instant::now() - Duration::from_secs(config.cleanup_interval_secs.max(1) as u64);
    let mut in_flight = JoinSet::new();

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

        while in_flight.len() < config.concurrency {
            match queue_consumer.consume(250).await {
                Ok(Some(claimed)) => {
                    let repo = repo.clone();
                    let queue_publisher = queue_publisher.clone();
                    let queue_consumer = queue_consumer.clone();
                    let storage = storage.clone();
                    let progress_store = progress_store.clone();
                    let config = config.clone();

                    in_flight.spawn(async move {
                        let should_ack = run_claimed_job(
                            repo.as_ref(),
                            queue_publisher,
                            storage,
                            progress_store,
                            &config,
                            claimed.clone(),
                        )
                        .await?;
                        if should_ack {
                            queue_consumer.ack(&claimed.stream_id).await?;
                        }
                        Ok::<(), anyhow::Error>(())
                    });
                }
                Ok(None) => break,
                Err(error) => {
                    error!(err = %error, "Worker consume loop failed");
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    break;
                }
            }
        }

        if in_flight.is_empty() {
            tokio::time::sleep(Duration::from_millis(250)).await;
            continue;
        }

        if let Some(result) = in_flight.join_next().await {
            match result {
                Ok(Ok(())) => {}
                Ok(Err(error)) => return Err(error),
                Err(error) => return Err(error.into()),
            }
        }
    }
}
