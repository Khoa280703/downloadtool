use anyhow::Context;

use crate::job_models::{ArtifactRecord, JobArtifactDownload, JobOwner, JobRecord};
use crate::repository::JobRepository;

impl JobRepository {
    pub async fn get_user_job(
        &self,
        job_id: &str,
        owner: &JobOwner,
    ) -> anyhow::Result<Option<JobRecord>> {
        let row = match (&owner.user_id, &owner.session_id) {
            (Some(user_id), _) => {
                sqlx::query(
                    r#"
                    SELECT
                        j.id, j.user_id, j.session_id, j.request_hash, j.dedupe_key, j.source_url,
                        j.video_url, j.audio_url, j.video_format_id, j.audio_format_id, j.title,
                        j.status, j.artifact_id, j.attempt_count, j.max_attempts, j.lease_owner,
                        j.lease_expires_at_ms, j.last_error, j.created_at_ms, j.updated_at_ms,
                        a.size_bytes AS file_size_bytes
                    FROM mux_jobs j
                    LEFT JOIN mux_artifacts a ON a.id = j.artifact_id
                    WHERE j.id = $1 AND j.user_id = $2
                    "#,
                )
                .bind(job_id)
                .bind(user_id)
                .fetch_optional(self.pool())
                .await
            }
            (None, Some(session_id)) => {
                sqlx::query(
                    r#"
                    SELECT
                        j.id, j.user_id, j.session_id, j.request_hash, j.dedupe_key, j.source_url,
                        j.video_url, j.audio_url, j.video_format_id, j.audio_format_id, j.title,
                        j.status, j.artifact_id, j.attempt_count, j.max_attempts, j.lease_owner,
                        j.lease_expires_at_ms, j.last_error, j.created_at_ms, j.updated_at_ms,
                        a.size_bytes AS file_size_bytes
                    FROM mux_jobs j
                    LEFT JOIN mux_artifacts a ON a.id = j.artifact_id
                    WHERE j.id = $1 AND j.session_id = $2
                    "#,
                )
                .bind(job_id)
                .bind(session_id)
                .fetch_optional(self.pool())
                .await
            }
            (None, None) => Ok(None),
        }
        .with_context(|| format!("failed to load user-scoped job {job_id}"))?;

        Ok(row.map(|row| Self::row_to_job(&row)))
    }

    pub async fn get_job_for_worker(&self, job_id: &str) -> anyhow::Result<Option<JobRecord>> {
        let row = sqlx::query(
            r#"
            SELECT
                j.id, j.user_id, j.session_id, j.request_hash, j.dedupe_key, j.source_url,
                j.video_url, j.audio_url, j.video_format_id, j.audio_format_id, j.title,
                j.status, j.artifact_id, j.attempt_count, j.max_attempts, j.lease_owner,
                j.lease_expires_at_ms, j.last_error, j.created_at_ms, j.updated_at_ms,
                a.size_bytes AS file_size_bytes
            FROM mux_jobs j
            LEFT JOIN mux_artifacts a ON a.id = j.artifact_id
            WHERE j.id = $1
            "#,
        )
        .bind(job_id)
        .fetch_optional(self.pool())
        .await
        .with_context(|| format!("failed to load worker job {job_id}"))?;

        Ok(row.map(|row| Self::row_to_job(&row)))
    }

    pub async fn get_ready_artifact_for_user_job(
        &self,
        job_id: &str,
        owner: &JobOwner,
    ) -> anyhow::Result<Option<JobArtifactDownload>> {
        let row = match (&owner.user_id, &owner.session_id) {
            (Some(user_id), _) => {
                sqlx::query(
                    r#"
                    SELECT
                        j.id, j.user_id, j.session_id, j.request_hash, j.dedupe_key, j.source_url,
                        j.video_url, j.audio_url, j.video_format_id, j.audio_format_id, j.title,
                        j.status, j.artifact_id, j.attempt_count, j.max_attempts, j.lease_owner,
                        j.lease_expires_at_ms, j.last_error, j.created_at_ms, j.updated_at_ms,
                        a.size_bytes AS file_size_bytes, a.id AS artifact_id,
                        a.dedupe_key AS artifact_dedupe_key, a.backend, a.local_path,
                        a.storage_bucket, a.object_key, a.status AS artifact_status,
                        a.size_bytes AS artifact_size_bytes, a.content_type, a.etag
                    FROM mux_jobs j
                    JOIN mux_artifacts a ON a.id = j.artifact_id
                    WHERE j.id = $1 AND j.user_id = $2 AND j.status = 'ready' AND a.status = 'ready'
                    "#,
                )
                .bind(job_id)
                .bind(user_id)
                .fetch_optional(self.pool())
                .await
            }
            (None, Some(session_id)) => {
                sqlx::query(
                    r#"
                    SELECT
                        j.id, j.user_id, j.session_id, j.request_hash, j.dedupe_key, j.source_url,
                        j.video_url, j.audio_url, j.video_format_id, j.audio_format_id, j.title,
                        j.status, j.artifact_id, j.attempt_count, j.max_attempts, j.lease_owner,
                        j.lease_expires_at_ms, j.last_error, j.created_at_ms, j.updated_at_ms,
                        a.size_bytes AS file_size_bytes, a.id AS artifact_id,
                        a.dedupe_key AS artifact_dedupe_key, a.backend, a.local_path,
                        a.storage_bucket, a.object_key, a.status AS artifact_status,
                        a.size_bytes AS artifact_size_bytes, a.content_type, a.etag
                    FROM mux_jobs j
                    JOIN mux_artifacts a ON a.id = j.artifact_id
                    WHERE j.id = $1 AND j.session_id = $2 AND j.status = 'ready' AND a.status = 'ready'
                    "#,
                )
                .bind(job_id)
                .bind(session_id)
                .fetch_optional(self.pool())
                .await
            }
            (None, None) => Ok(None),
        }
        .with_context(|| format!("failed to load ready artifact for job {job_id}"))?;

        Ok(row.map(|row| Self::row_to_download(&row)))
    }

    pub async fn find_ready_artifact_by_dedupe_key(
        &self,
        dedupe_key: &str,
    ) -> anyhow::Result<Option<ArtifactRecord>> {
        let row = sqlx::query(
            r#"
            SELECT
                a.id AS artifact_id, a.dedupe_key AS artifact_dedupe_key, a.backend, a.local_path,
                a.storage_bucket, a.object_key, a.status AS artifact_status,
                a.size_bytes AS artifact_size_bytes, a.content_type, a.etag
            FROM mux_artifacts a
            WHERE a.dedupe_key = $1 AND a.status = 'ready'
            ORDER BY a.ready_at DESC NULLS LAST
            LIMIT 1
            "#,
        )
        .bind(dedupe_key)
        .fetch_optional(self.pool())
        .await
        .with_context(|| format!("failed to load ready artifact for dedupe key {dedupe_key}"))?;

        Ok(row.map(|row| Self::row_to_artifact(&row)))
    }

    pub async fn touch_artifact_access_for_user_job(
        &self,
        job_id: &str,
        owner: &JobOwner,
    ) -> anyhow::Result<bool> {
        let updated_rows = match (&owner.user_id, &owner.session_id) {
            (Some(user_id), _) => {
                sqlx::query(
                    r#"
                    UPDATE mux_artifacts
                    SET last_accessed_at = NOW()
                    WHERE id = (
                        SELECT artifact_id
                        FROM mux_jobs
                        WHERE id = $1 AND user_id = $2
                    )
                    "#,
                )
                .bind(job_id)
                .bind(user_id)
                .execute(self.pool())
                .await
            }
            (None, Some(session_id)) => {
                sqlx::query(
                    r#"
                    UPDATE mux_artifacts
                    SET last_accessed_at = NOW()
                    WHERE id = (
                        SELECT artifact_id
                        FROM mux_jobs
                        WHERE id = $1 AND session_id = $2
                    )
                    "#,
                )
                .bind(job_id)
                .bind(session_id)
                .execute(self.pool())
                .await
            }
            (None, None) => return Ok(false),
        }
        .with_context(|| format!("failed to touch artifact access for job {job_id}"))?
        .rows_affected();

        Ok(updated_rows > 0)
    }
}
