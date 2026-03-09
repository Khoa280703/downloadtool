use anyhow::Context;
use sqlx::{PgPool, Row};

use crate::job_models::{
    ArtifactRecord, JobArtifactDownload, JobCreationResult, JobRecord, JobStatus, MuxJobRequest,
};

#[derive(Clone)]
pub struct JobRepository {
    pool: PgPool,
}

impl JobRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn row_to_job(row: &sqlx::postgres::PgRow) -> JobRecord {
        JobRecord {
            id: row.get("id"),
            user_id: row.get("user_id"),
            session_id: row.get("session_id"),
            request_hash: row.get("request_hash"),
            dedupe_key: row.get("dedupe_key"),
            request: MuxJobRequest {
                source_url: row.get("source_url"),
                video_url: row.get("video_url"),
                audio_url: row.get("audio_url"),
                video_format_id: row.get("video_format_id"),
                audio_format_id: row.get("audio_format_id"),
                title: row.get("title"),
            },
            status: JobStatus::from_str(&row.get::<String, _>("status")),
            artifact_id: row.get("artifact_id"),
            attempt_count: row.get("attempt_count"),
            max_attempts: row.get("max_attempts"),
            lease_owner: row.get("lease_owner"),
            lease_expires_at_ms: row.get("lease_expires_at_ms"),
            last_error: row.get("last_error"),
            created_at_ms: row.get("created_at_ms"),
            updated_at_ms: row.get("updated_at_ms"),
            file_size_bytes: row.get("file_size_bytes"),
        }
    }

    pub fn row_to_artifact(row: &sqlx::postgres::PgRow) -> ArtifactRecord {
        ArtifactRecord {
            id: row.get("artifact_id"),
            dedupe_key: row.get("artifact_dedupe_key"),
            backend: row.get("backend"),
            local_path: row.get("local_path"),
            storage_bucket: row.get("storage_bucket"),
            object_key: row.get("object_key"),
            status: row.get("artifact_status"),
            size_bytes: row.get("artifact_size_bytes"),
            content_type: row.get("content_type"),
            etag: row.get("etag"),
        }
    }

    pub fn row_to_download(row: &sqlx::postgres::PgRow) -> JobArtifactDownload {
        JobArtifactDownload {
            job: Self::row_to_job(row),
            artifact: Self::row_to_artifact(row),
        }
    }
}

pub fn now_ms() -> i64 {
    (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()) as i64
}

pub fn build_event_payload(
    job: &JobRecord,
    reused_existing: bool,
    reason: Option<&str>,
) -> anyhow::Result<serde_json::Value> {
    serde_json::to_value(JobCreationResult {
        job: job.clone(),
        reused_existing,
    })
    .map(|mut payload| {
        if let Some(reason) = reason {
            if let Some(object) = payload.as_object_mut() {
                object.insert(
                    "reason".to_string(),
                    serde_json::Value::String(reason.to_string()),
                );
            }
        }
        payload
    })
    .context("failed to serialize job event payload")
}
