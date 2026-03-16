use anyhow::Context;

use crate::job_models::{ArtifactRecord, JobStatus};
use crate::repository::{now_ms, JobRepository};

impl JobRepository {
    pub async fn ensure_building_artifact(
        &self,
        dedupe_key: &str,
        content_type: &str,
    ) -> anyhow::Result<ArtifactRecord> {
        let row = sqlx::query(
            r#"
            INSERT INTO mux_artifacts (dedupe_key, artifact_key, content_type, status)
            VALUES ($1, $1, $2, 'building')
            ON CONFLICT (artifact_key) DO UPDATE
            SET dedupe_key = EXCLUDED.dedupe_key
            RETURNING
                id AS artifact_id, dedupe_key AS artifact_dedupe_key, backend, local_path,
                storage_bucket, object_key, status AS artifact_status,
                size_bytes AS artifact_size_bytes, content_type, etag
            "#,
        )
        .bind(dedupe_key)
        .bind(content_type)
        .fetch_one(self.pool())
        .await
        .with_context(|| format!("failed to ensure building artifact for {dedupe_key}"))?;

        Ok(Self::row_to_artifact(&row))
    }

    pub async fn attach_ready_artifact(
        &self,
        job_id: &str,
        artifact_id: &str,
    ) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            UPDATE mux_jobs
            SET status = 'ready',
                artifact_id = $2,
                lease_owner = NULL,
                lease_expires_at_ms = NULL,
                last_error = NULL,
                updated_at_ms = $3,
                completed_at = NOW(),
                delete_after_at = (SELECT expires_at FROM mux_artifacts WHERE id = $2)
            WHERE id = $1
            "#,
        )
        .bind(job_id)
        .bind(artifact_id)
        .bind(now_ms())
        .execute(self.pool())
        .await
        .with_context(|| {
            format!("failed to attach ready artifact {artifact_id} to job {job_id}")
        })?;
        Ok(())
    }

    pub async fn mark_artifact_ready(
        &self,
        job_id: &str,
        artifact_id: &str,
        backend: &str,
        local_path: Option<&str>,
        storage_bucket: Option<&str>,
        object_key: Option<&str>,
        size_bytes: i64,
        etag: Option<&str>,
        ttl_secs: i64,
    ) -> anyhow::Result<()> {
        let mut tx = self
            .pool()
            .begin()
            .await
            .context("failed to begin artifact tx")?;
        sqlx::query(
            r#"
            UPDATE mux_artifacts
            SET backend = $2, local_path = $3, storage_bucket = $4, object_key = $5,
                status = 'ready', size_bytes = $6, etag = $7, ready_at = NOW(),
                expires_at = NOW() + ($8 || ' seconds')::interval, last_accessed_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(artifact_id)
        .bind(backend)
        .bind(local_path)
        .bind(storage_bucket)
        .bind(object_key)
        .bind(size_bytes)
        .bind(etag)
        .bind(ttl_secs)
        .execute(&mut *tx)
        .await
        .with_context(|| format!("failed to mark artifact {artifact_id} ready"))?;

        sqlx::query(
            r#"
            UPDATE mux_jobs
            SET status = 'ready', artifact_id = $2, lease_owner = NULL, lease_expires_at_ms = NULL,
                last_error = NULL, updated_at_ms = $3, completed_at = NOW(),
                delete_after_at = NOW() + ($4 || ' seconds')::interval
            WHERE id = $1
            "#,
        )
        .bind(job_id)
        .bind(artifact_id)
        .bind(now_ms())
        .bind(ttl_secs)
        .execute(&mut *tx)
        .await
        .with_context(|| format!("failed to mark job {job_id} ready"))?;

        tx.commit()
            .await
            .context("failed to commit ready artifact tx")?;
        Ok(())
    }

    pub async fn mark_job_failed(
        &self,
        job_id: &str,
        worker_id: &str,
        error_message: &str,
        final_failure: bool,
    ) -> anyhow::Result<JobStatus> {
        let next_status = if final_failure {
            JobStatus::Failed
        } else {
            JobStatus::Queued
        };
        let event_type = if next_status == JobStatus::Failed {
            "job_failed"
        } else {
            "job_requeued"
        };
        let mut tx = self
            .pool()
            .begin()
            .await
            .context("failed to begin job failure tx")?;
        sqlx::query(
            r#"
            UPDATE mux_jobs
            SET status = $3, lease_owner = NULL, lease_expires_at_ms = NULL, last_error = $4,
                updated_at_ms = $5,
                completed_at = CASE WHEN $3 = 'failed' THEN NOW() ELSE completed_at END
            WHERE id = $1 AND lease_owner = $2
            "#,
        )
        .bind(job_id)
        .bind(worker_id)
        .bind(next_status.as_str())
        .bind(error_message)
        .bind(now_ms())
        .execute(&mut *tx)
        .await
        .with_context(|| format!("failed to mark job {job_id} failed"))?;

        sqlx::query(
            r#"
            INSERT INTO mux_job_events (job_id, event_type, payload_json)
            VALUES ($1, $2, $3::jsonb)
            "#,
        )
        .bind(job_id)
        .bind(event_type)
        .bind(serde_json::json!({
            "worker_id": worker_id,
            "status": next_status.as_str(),
            "final_failure": final_failure,
            "error": error_message
        }))
        .execute(&mut *tx)
        .await
        .with_context(|| format!("failed to record {event_type} event for job {job_id}"))?;

        tx.commit()
            .await
            .context("failed to commit job failure tx")?;
        Ok(next_status)
    }

    pub async fn list_expired_artifacts(&self, limit: i64) -> anyhow::Result<Vec<ArtifactRecord>> {
        let rows = sqlx::query(
            r#"
            SELECT
                id AS artifact_id, dedupe_key AS artifact_dedupe_key, backend, local_path,
                storage_bucket, object_key, status AS artifact_status,
                size_bytes AS artifact_size_bytes, content_type, etag
            FROM mux_artifacts
            WHERE expires_at IS NOT NULL
              AND expires_at <= NOW()
            ORDER BY expires_at ASC
            LIMIT $1
            "#,
        )
        .bind(limit.max(1))
        .fetch_all(self.pool())
        .await
        .context("failed to list expired artifacts")?;

        Ok(rows.iter().map(Self::row_to_artifact).collect())
    }

    pub async fn finalize_expired_artifact(&self, artifact_id: &str) -> anyhow::Result<()> {
        let mut tx = self
            .pool()
            .begin()
            .await
            .context("failed to begin artifact expiration tx")?;

        sqlx::query(
            r#"
            UPDATE mux_jobs
            SET status = 'expired',
                artifact_id = NULL,
                updated_at_ms = $2,
                completed_at = COALESCE(completed_at, NOW())
            WHERE artifact_id = $1
              AND status IN ('ready', 'failed', 'expired')
            "#,
        )
        .bind(artifact_id)
        .bind(now_ms())
        .execute(&mut *tx)
        .await
        .with_context(|| format!("failed to expire jobs for artifact {artifact_id}"))?;

        sqlx::query("DELETE FROM mux_artifacts WHERE id = $1")
            .bind(artifact_id)
            .execute(&mut *tx)
            .await
            .with_context(|| format!("failed to delete expired artifact row {artifact_id}"))?;

        tx.commit()
            .await
            .context("failed to commit artifact expiration tx")?;
        Ok(())
    }
}
