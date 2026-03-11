use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use anyhow::Result;
use job_system::{JobRepository, JobStatus};
use object_store::{StorageBackend, StoredArtifact};
use queue::{ClaimedQueueMessage, JobQueuePublisher, QueueJobMessage};
use tokio::sync::oneshot;
use tracing::{error, info, warn};

use crate::mux_pipeline::upload_muxed_artifact;
use crate::worker_config::WorkerConfig;

pub async fn run_claimed_job(
    repo: &JobRepository,
    queue: Arc<dyn JobQueuePublisher>,
    storage: Arc<dyn StorageBackend>,
    config: &WorkerConfig,
    claimed: ClaimedQueueMessage,
) -> Result<bool> {
    let Some(job) = repo
        .claim_job(
            &claimed.message.job_id,
            &config.worker_id,
            lease_expires_at_ms(config.lease_secs),
        )
        .await?
    else {
        return Ok(true);
    };

    if let Some(artifact) = repo
        .find_ready_artifact_by_dedupe_key(&job.dedupe_key)
        .await?
    {
        repo.attach_ready_artifact(&job.id, &artifact.id).await?;
        repo.record_event(
            &job.id,
            "artifact_reused",
            serde_json::json!({ "artifact_id": artifact.id }),
        )
        .await?;
        return Ok(true);
    }

    if !repo.try_claim_dedupe_lock(&job.dedupe_key).await? {
        warn!(job_id = job.id, "Dedupe lock busy, requeueing job");
        repo.mark_job_failed(
            &job.id,
            &config.worker_id,
            "dedupe_build_in_progress",
            false,
        )
        .await?;
        queue
            .publish(QueueJobMessage {
                job_id: job.id.clone(),
                dedupe_key: job.dedupe_key.clone(),
                requested_at_ms: claimed.message.requested_at_ms,
            })
            .await?;
        return Ok(true);
    }

    let processing_result = async {
        if let Some(artifact) = repo
            .find_ready_artifact_by_dedupe_key(&job.dedupe_key)
            .await?
        {
            repo.attach_ready_artifact(&job.id, &artifact.id).await?;
            return Ok::<_, anyhow::Error>(());
        }

        let artifact = repo
            .ensure_building_artifact(&job.dedupe_key, "video/mp4")
            .await?;
        info!(
            job_id = job.id,
            artifact_id = artifact.id,
            dedupe_key = job.dedupe_key,
            artifact_backend = config.artifact_backend,
            artifact_dir = %config.artifact_dir.display(),
            "Worker starting mux artifact build"
        );
        repo.mark_processing(
            &job.id,
            &config.worker_id,
            lease_expires_at_ms(config.lease_secs),
        )
        .await?;
        let (heartbeat_stop, heartbeat_task) = spawn_lease_heartbeat(
            repo.clone(),
            job.id.clone(),
            config.worker_id.clone(),
            config.lease_secs,
        );

        let upload_started_at = Instant::now();
        let upload_result =
            upload_muxed_artifact(&job.id, &job.request, storage.clone(), &job.dedupe_key).await;

        let _ = heartbeat_stop.send(());
        let _ = heartbeat_task.await;
        let stored = upload_result?;
        info!(
            job_id = job.id,
            artifact_id = artifact.id,
            backend = stored.backend,
            size_bytes = stored.size_bytes,
            local_path = stored.local_path.as_deref().unwrap_or(""),
            bucket = stored.storage_bucket.as_deref().unwrap_or(""),
            object_key = stored.object_key.as_deref().unwrap_or(""),
            elapsed_ms = upload_started_at.elapsed().as_millis() as u64,
            "Worker stored muxed artifact"
        );

        repo.mark_artifact_ready(
            &job.id,
            &artifact.id,
            &stored.backend,
            stored.local_path.as_deref(),
            stored.storage_bucket.as_deref(),
            stored.object_key.as_deref(),
            stored.size_bytes,
            stored.etag.as_deref(),
            config.artifact_ttl_secs,
        )
        .await?;

        repo.record_event(
            &job.id,
            "artifact_ready",
            serde_json::json!({
                "artifact_id": artifact.id,
                "backend": stored.backend,
                "size_bytes": stored.size_bytes
            }),
        )
        .await?;
        Ok(())
    }
    .await;

    repo.release_dedupe_lock(&job.dedupe_key).await?;

    match processing_result {
        Ok(()) => {
            info!(job_id = job.id, "Worker completed mux job");
            Ok(true)
        }
        Err(error) => {
            let final_failure = job.attempt_count >= config.max_attempts;
            let next_status = repo
                .mark_job_failed(
                    &job.id,
                    &config.worker_id,
                    &error.to_string(),
                    final_failure,
                )
                .await?;
            error!(
                job_id = job.id,
                err = %error,
                err_chain = %format!("{error:#}"),
                status = next_status.as_str(),
                "Worker failed mux job"
            );
            if next_status == JobStatus::Queued {
                queue
                    .publish(QueueJobMessage {
                        job_id: job.id,
                        dedupe_key: job.dedupe_key,
                        requested_at_ms: claimed.message.requested_at_ms,
                    })
                    .await?;
            }
            Ok(next_status != JobStatus::Queued)
        }
    }
}

pub async fn republish_reclaimed_jobs(
    repo: &JobRepository,
    queue: Arc<dyn JobQueuePublisher>,
    config: &WorkerConfig,
) -> Result<()> {
    let reclaimed = repo.reclaim_expired_leases(config.reclaim_limit).await?;
    for job_id in reclaimed {
        queue
            .publish(QueueJobMessage {
                job_id,
                dedupe_key: String::new(),
                requested_at_ms: job_system::repository::now_ms(),
            })
            .await?;
    }
    Ok(())
}

pub async fn cleanup_expired_artifacts(
    repo: &JobRepository,
    storage: Arc<dyn StorageBackend>,
    batch_limit: i64,
) -> Result<()> {
    let expired = repo.list_expired_artifacts(batch_limit).await?;
    for artifact in expired {
        let stored = StoredArtifact {
            backend: artifact.backend.clone(),
            local_path: artifact.local_path.clone(),
            storage_bucket: artifact.storage_bucket.clone(),
            object_key: artifact.object_key.clone(),
            size_bytes: artifact.size_bytes.unwrap_or_default(),
            etag: artifact.etag.clone(),
            content_type: artifact.content_type.clone(),
        };

        if let Err(error) = storage.delete(&stored).await {
            warn!(
                artifact_id = artifact.id,
                err = %error,
                "Failed to delete expired artifact from storage backend"
            );
            continue;
        }

        repo.finalize_expired_artifact(&artifact.id).await?;
        info!(artifact_id = artifact.id, "Deleted expired mux artifact");
    }
    Ok(())
}

fn lease_expires_at_ms(lease_secs: i64) -> i64 {
    job_system::repository::now_ms() + (lease_secs * 1000)
}

fn spawn_lease_heartbeat(
    repo: JobRepository,
    job_id: String,
    worker_id: String,
    lease_secs: i64,
) -> (oneshot::Sender<()>, tokio::task::JoinHandle<()>) {
    let (stop_tx, mut stop_rx) = oneshot::channel();
    let handle = tokio::spawn(async move {
        let interval_secs = heartbeat_interval_secs(lease_secs);
        loop {
            tokio::select! {
                _ = &mut stop_rx => break,
                _ = tokio::time::sleep(Duration::from_secs(interval_secs)) => {
                    if let Err(error) = repo
                        .heartbeat_lease(&job_id, &worker_id, lease_expires_at_ms(lease_secs))
                        .await
                    {
                        warn!(job_id, err = %error, "Failed to extend worker lease heartbeat");
                    }
                }
            }
        }
    });

    (stop_tx, handle)
}

fn heartbeat_interval_secs(lease_secs: i64) -> u64 {
    let lease_secs = lease_secs.max(15) as u64;
    (lease_secs / 3).max(5)
}
