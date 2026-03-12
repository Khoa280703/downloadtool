use anyhow::Context;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

use crate::repository::now_ms;

const JOB_PROGRESS_KEY_PREFIX: &str = "mux:progress:";
const JOB_PROGRESS_CHANNEL_PREFIX: &str = "mux:progress:events:";

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum JobProgressPhase {
    Starting,
    FetchingStreams,
    MuxingUploading,
    CompletingUpload,
    Ready,
    Failed,
    Retrying,
}

impl JobProgressPhase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Starting => "starting",
            Self::FetchingStreams => "fetching_streams",
            Self::MuxingUploading => "muxing_uploading",
            Self::CompletingUpload => "completing_upload",
            Self::Ready => "ready",
            Self::Failed => "failed",
            Self::Retrying => "retrying",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JobProgressSnapshot {
    pub job_id: String,
    pub phase: JobProgressPhase,
    pub percent: Option<f32>,
    pub uploaded_bytes: u64,
    pub total_bytes: Option<u64>,
    pub updated_at_ms: i64,
}

impl JobProgressSnapshot {
    pub fn new(
        job_id: impl Into<String>,
        phase: JobProgressPhase,
        percent: Option<f32>,
        uploaded_bytes: u64,
        total_bytes: Option<u64>,
    ) -> Self {
        Self {
            job_id: job_id.into(),
            phase,
            percent,
            uploaded_bytes,
            total_bytes,
            updated_at_ms: now_ms(),
        }
    }
}

#[derive(Clone)]
pub struct JobProgressStore {
    client: redis::Client,
}

impl JobProgressStore {
    pub fn new(redis_url: &str) -> anyhow::Result<Self> {
        Ok(Self {
            client: redis::Client::open(redis_url)
                .context("failed to create job progress redis client")?,
        })
    }

    async fn connection(&self) -> anyhow::Result<redis::aio::MultiplexedConnection> {
        self.client
            .get_multiplexed_async_connection()
            .await
            .context("failed to connect to redis for job progress")
    }

    pub async fn write_snapshot(
        &self,
        snapshot: &JobProgressSnapshot,
        ttl_secs: u64,
    ) -> anyhow::Result<()> {
        let mut conn = self.connection().await?;
        let key = progress_key(&snapshot.job_id);
        let channel = progress_channel(&snapshot.job_id);
        let payload =
            serde_json::to_string(snapshot).context("failed to serialize job progress snapshot")?;
        let _: () = conn
            .set_ex(&key, payload, ttl_secs)
            .await
            .with_context(|| format!("failed to write redis key {key}"))?;
        let _: i64 = conn
            .publish(&channel, snapshot_payload(snapshot)?)
            .await
            .with_context(|| format!("failed to publish redis channel {channel}"))?;
        Ok(())
    }

    pub async fn read_snapshot(&self, job_id: &str) -> anyhow::Result<Option<JobProgressSnapshot>> {
        let mut conn = self.connection().await?;
        let key = progress_key(job_id);
        let payload: Option<String> = conn
            .get(&key)
            .await
            .with_context(|| format!("failed to read redis key {key}"))?;
        payload
            .map(|value| {
                serde_json::from_str(&value).context("failed to parse job progress snapshot")
            })
            .transpose()
    }

    pub async fn subscribe(&self, job_id: &str) -> anyhow::Result<redis::aio::PubSub> {
        let mut pubsub = self
            .client
            .get_async_pubsub()
            .await
            .context("failed to create redis pubsub connection for job progress")?;
        let channel = progress_channel(job_id);
        pubsub
            .subscribe(&channel)
            .await
            .with_context(|| format!("failed to subscribe redis channel {channel}"))?;
        Ok(pubsub)
    }
}

fn progress_key(job_id: &str) -> String {
    format!("{JOB_PROGRESS_KEY_PREFIX}{job_id}")
}

fn progress_channel(job_id: &str) -> String {
    format!("{JOB_PROGRESS_CHANNEL_PREFIX}{job_id}")
}

fn snapshot_payload(snapshot: &JobProgressSnapshot) -> anyhow::Result<String> {
    serde_json::to_string(snapshot).context("failed to serialize job progress pubsub payload")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_phase_uses_snake_case_strings() {
        assert_eq!(
            JobProgressPhase::MuxingUploading.as_str(),
            "muxing_uploading"
        );
        assert_eq!(
            JobProgressPhase::CompletingUpload.as_str(),
            "completing_upload"
        );
    }

    #[test]
    fn progress_key_keeps_job_namespace() {
        assert_eq!(progress_key("job-123"), "mux:progress:job-123");
    }

    #[test]
    fn progress_channel_keeps_job_namespace() {
        assert_eq!(progress_channel("job-123"), "mux:progress:events:job-123");
    }
}
