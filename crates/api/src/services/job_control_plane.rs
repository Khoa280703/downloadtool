use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use anyhow::Context;
use job_system::{
    JobArtifactDownload, JobOwner, JobRecord as DurableJobRecord,
    JobRepository as DurableJobRepository, JobStatus, MuxJobRequest,
};
use queue::{JobQueuePublisher, QueueJobMessage};

use super::job_identity::derive_job_identity;

#[derive(Debug, Clone)]
pub struct JobCreationResult {
    pub job_id: String,
    pub status: JobStatus,
    pub reused_existing: bool,
}

#[derive(Debug, Clone)]
pub struct JobStatusRecord {
    pub job_id: String,
    pub status: JobStatus,
    pub error: Option<String>,
    pub created_at_ms: u64,
    pub updated_at_ms: u64,
    pub file_size_bytes: Option<u64>,
    pub title: Option<String>,
    pub backend: Option<String>,
    pub local_path: Option<PathBuf>,
    pub object_key: Option<String>,
    pub storage_bucket: Option<String>,
    pub content_type: Option<String>,
    pub etag: Option<String>,
}

pub struct JobControlPlaneService {
    durable_repository: Arc<DurableJobRepository>,
    queue_publisher: Arc<dyn JobQueuePublisher>,
    max_attempts: i32,
    id_counter: AtomicU64,
}

impl JobControlPlaneService {
    pub fn new(
        durable_repository: Arc<DurableJobRepository>,
        queue_publisher: Arc<dyn JobQueuePublisher>,
        max_attempts: i32,
    ) -> Arc<Self> {
        Arc::new(Self {
            durable_repository,
            queue_publisher,
            max_attempts: max_attempts.max(1),
            id_counter: AtomicU64::new(1),
        })
    }

    pub async fn create_job(
        &self,
        owner: &JobOwner,
        request: MuxJobRequest,
    ) -> anyhow::Result<JobCreationResult> {
        let identity = derive_job_identity(owner, &request);
        let created = self
            .durable_repository
            .create_or_reuse_job(
                &self.next_durable_job_id(),
                owner,
                &identity.request_hash,
                &identity.dedupe_key,
                &request,
                self.max_attempts,
            )
            .await?;

        if created.job.status == JobStatus::Queued {
            self.publish_job(&created.job.id, &created.job.dedupe_key)
                .await?;
        }

        Ok(JobCreationResult {
            job_id: created.job.id,
            status: created.job.status,
            reused_existing: created.reused_existing,
        })
    }

    pub async fn get_job_for_user(
        &self,
        job_id: &str,
        owner: &JobOwner,
    ) -> anyhow::Result<Option<JobStatusRecord>> {
        let Some(job) = self.durable_repository.get_user_job(job_id, owner).await? else {
            return Ok(None);
        };

        if job.status == JobStatus::Ready {
            if let Some(download) = self
                .durable_repository
                .get_ready_artifact_for_user_job(job_id, owner)
                .await?
            {
                return Ok(Some(map_durable_download(download)));
            }
        }

        Ok(Some(map_durable_job(job)))
    }

    pub async fn touch_release(&self, job_id: &str, owner: &JobOwner) -> anyhow::Result<bool> {
        self.durable_repository
            .touch_artifact_access_for_user_job(job_id, owner)
            .await
    }

    async fn publish_job(&self, job_id: &str, dedupe_key: &str) -> anyhow::Result<()> {
        self.queue_publisher
            .publish(QueueJobMessage {
                job_id: job_id.to_string(),
                dedupe_key: dedupe_key.to_string(),
                requested_at_ms: job_system::repository::now_ms(),
            })
            .await
            .with_context(|| format!("failed to publish durable worker job {job_id}"))
    }

    fn next_durable_job_id(&self) -> String {
        let sequence = self.id_counter.fetch_add(1, Ordering::Relaxed);
        format!("job-{}-{sequence}", job_system::repository::now_ms())
    }
}

fn map_durable_job(job: DurableJobRecord) -> JobStatusRecord {
    JobStatusRecord {
        job_id: job.id,
        status: job.status,
        error: job.last_error,
        created_at_ms: job.created_at_ms as u64,
        updated_at_ms: job.updated_at_ms as u64,
        file_size_bytes: job.file_size_bytes.map(|value| value as u64),
        title: job.request.title,
        backend: None,
        local_path: None,
        object_key: None,
        storage_bucket: None,
        content_type: None,
        etag: None,
    }
}

fn map_durable_download(download: JobArtifactDownload) -> JobStatusRecord {
    JobStatusRecord {
        job_id: download.job.id,
        status: download.job.status,
        error: download.job.last_error,
        created_at_ms: download.job.created_at_ms as u64,
        updated_at_ms: download.job.updated_at_ms as u64,
        file_size_bytes: download.artifact.size_bytes.map(|value| value as u64),
        title: download.job.request.title,
        backend: Some(download.artifact.backend),
        local_path: download.artifact.local_path.map(PathBuf::from),
        object_key: download.artifact.object_key,
        storage_bucket: download.artifact.storage_bucket,
        content_type: Some(download.artifact.content_type),
        etag: download.artifact.etag,
    }
}
