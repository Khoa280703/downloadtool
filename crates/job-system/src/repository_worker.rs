use anyhow::Context;
use sqlx::Row;

use crate::job_models::{JobRecord, JobStatus};
use crate::repository::{now_ms, JobRepository};

impl JobRepository {
    pub async fn reclaim_expired_leases(&self, limit: i64) -> anyhow::Result<Vec<String>> {
        let rows = sqlx::query(
            r#"
            UPDATE mux_jobs
            SET status = 'queued',
                lease_owner = NULL,
                lease_expires_at_ms = NULL,
                updated_at_ms = $1
            WHERE id IN (
                SELECT id
                FROM mux_jobs
                WHERE status IN ('leased', 'processing')
                  AND lease_expires_at_ms IS NOT NULL
                  AND lease_expires_at_ms < $1
                ORDER BY updated_at_ms ASC
                LIMIT $2
            )
            RETURNING id
            "#,
        )
        .bind(now_ms())
        .bind(limit)
        .fetch_all(self.pool())
        .await
        .context("failed to reclaim expired mux job leases")?;

        Ok(rows.into_iter().map(|row| row.get("id")).collect())
    }

    pub async fn claim_job(
        &self,
        job_id: &str,
        worker_id: &str,
        lease_expires_at_ms: i64,
    ) -> anyhow::Result<Option<JobRecord>> {
        let row = sqlx::query(
            r#"
            UPDATE mux_jobs
            SET status = 'leased',
                lease_owner = $2,
                lease_expires_at_ms = $3,
                attempt_count = attempt_count + 1,
                updated_at_ms = $4
            WHERE id = $1
              AND status = 'queued'
            RETURNING
                id, user_id, request_hash, dedupe_key, source_url, video_url, audio_url,
                video_format_id, audio_format_id, title, status, artifact_id, attempt_count,
                max_attempts, lease_owner, lease_expires_at_ms, last_error, created_at_ms,
                updated_at_ms, NULL::bigint AS file_size_bytes
            "#,
        )
        .bind(job_id)
        .bind(worker_id)
        .bind(lease_expires_at_ms)
        .bind(now_ms())
        .fetch_optional(self.pool())
        .await
        .with_context(|| format!("failed to claim job {job_id}"))?;

        Ok(row.map(|row| Self::row_to_job(&row)))
    }

    pub async fn mark_processing(
        &self,
        job_id: &str,
        worker_id: &str,
        lease_expires_at_ms: i64,
    ) -> anyhow::Result<()> {
        self.update_leased_status(
            job_id,
            worker_id,
            JobStatus::Processing,
            lease_expires_at_ms,
        )
        .await
    }

    pub async fn heartbeat_lease(
        &self,
        job_id: &str,
        worker_id: &str,
        lease_expires_at_ms: i64,
    ) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            UPDATE mux_jobs
            SET lease_expires_at_ms = $3, updated_at_ms = $4
            WHERE id = $1 AND lease_owner = $2 AND status IN ('leased', 'processing')
            "#,
        )
        .bind(job_id)
        .bind(worker_id)
        .bind(lease_expires_at_ms)
        .bind(now_ms())
        .execute(self.pool())
        .await
        .with_context(|| format!("failed to heartbeat job {job_id}"))?;
        Ok(())
    }

    async fn update_leased_status(
        &self,
        job_id: &str,
        worker_id: &str,
        status: JobStatus,
        lease_expires_at_ms: i64,
    ) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            UPDATE mux_jobs
            SET status = $3, lease_expires_at_ms = $4, updated_at_ms = $5
            WHERE id = $1 AND lease_owner = $2
            "#,
        )
        .bind(job_id)
        .bind(worker_id)
        .bind(status.as_str())
        .bind(lease_expires_at_ms)
        .bind(now_ms())
        .execute(self.pool())
        .await
        .with_context(|| format!("failed to update job {job_id} to {}", status.as_str()))?;
        Ok(())
    }

    pub async fn try_claim_dedupe_lock(&self, dedupe_key: &str) -> anyhow::Result<bool> {
        let row = sqlx::query_scalar::<_, bool>("SELECT pg_try_advisory_lock(hashtext($1))")
            .bind(dedupe_key)
            .fetch_one(self.pool())
            .await
            .with_context(|| format!("failed to acquire dedupe advisory lock for {dedupe_key}"))?;
        Ok(row)
    }

    pub async fn release_dedupe_lock(&self, dedupe_key: &str) -> anyhow::Result<()> {
        sqlx::query("SELECT pg_advisory_unlock(hashtext($1))")
            .bind(dedupe_key)
            .execute(self.pool())
            .await
            .with_context(|| format!("failed to release dedupe advisory lock for {dedupe_key}"))?;
        Ok(())
    }
}
