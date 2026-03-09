use anyhow::Context;

use crate::job_models::{JobCreationResult, JobOwner, JobRecord, JobStatus, MuxJobRequest};
use crate::repository::{build_event_payload, now_ms, JobRepository};

impl JobRepository {
    pub async fn create_or_reuse_job(
        &self,
        job_id: &str,
        owner: &JobOwner,
        request_hash: &str,
        dedupe_key: &str,
        request: &MuxJobRequest,
        max_attempts: i32,
    ) -> anyhow::Result<JobCreationResult> {
        if let Some(existing) = self.find_reusable_job(owner, request_hash).await? {
            self.record_event(
                &existing.id,
                "job_reused",
                build_event_payload(&existing, true, Some("request_hash_match"))?,
            )
            .await?;
            return Ok(JobCreationResult {
                job: existing,
                reused_existing: true,
            });
        }

        let created_at_ms = now_ms();
        let row = sqlx::query(
            r#"
            INSERT INTO mux_jobs (
                id, user_id, session_id, request_hash, dedupe_key, source_url, video_url, audio_url,
                video_format_id, audio_format_id, title, status, attempt_count, max_attempts,
                created_at_ms, updated_at_ms
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, 'queued', 0, $12, $13, $13)
            RETURNING
                id, user_id, session_id, request_hash, dedupe_key, source_url, video_url,
                audio_url, video_format_id, audio_format_id, title, status, artifact_id,
                attempt_count, max_attempts, lease_owner, lease_expires_at_ms, last_error,
                created_at_ms, updated_at_ms, NULL::bigint AS file_size_bytes
            "#,
        )
        .bind(job_id)
        .bind(owner.user_id.as_deref())
        .bind(owner.session_id.as_deref())
        .bind(request_hash)
        .bind(dedupe_key)
        .bind(request.source_url.as_deref())
        .bind(&request.video_url)
        .bind(&request.audio_url)
        .bind(request.video_format_id.as_deref())
        .bind(request.audio_format_id.as_deref())
        .bind(request.title.as_deref())
        .bind(max_attempts)
        .bind(created_at_ms)
        .fetch_one(self.pool())
        .await
        .with_context(|| format!("failed to create mux job {job_id}"))?;

        let job = Self::row_to_job(&row);
        self.record_event(
            &job.id,
            "job_created",
            build_event_payload(&job, false, None)?,
        )
        .await?;

        Ok(JobCreationResult {
            job,
            reused_existing: false,
        })
    }

    async fn find_reusable_job(&self, owner: &JobOwner, request_hash: &str) -> anyhow::Result<Option<JobRecord>> {
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
                    WHERE j.user_id = $1
                      AND j.request_hash = $2
                      AND j.status IN ('queued', 'leased', 'processing', 'ready')
                      AND (j.delete_after_at IS NULL OR j.delete_after_at > NOW())
                    ORDER BY j.created_at_ms DESC
                    LIMIT 1
                    "#,
                )
                .bind(user_id)
                .bind(request_hash)
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
                    WHERE j.session_id = $1
                      AND j.request_hash = $2
                      AND j.status IN ('queued', 'leased', 'processing', 'ready')
                      AND (j.delete_after_at IS NULL OR j.delete_after_at > NOW())
                    ORDER BY j.created_at_ms DESC
                    LIMIT 1
                    "#,
                )
                .bind(session_id)
                .bind(request_hash)
                .fetch_optional(self.pool())
                .await
            }
            (None, None) => Ok(None),
        }
        .context("failed to query reusable mux job")?;

        Ok(row.map(|row| Self::row_to_job(&row)))
    }

    pub async fn record_event(
        &self,
        job_id: &str,
        event_type: &str,
        payload_json: serde_json::Value,
    ) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO mux_job_events (job_id, event_type, payload_json)
            VALUES ($1, $2, $3::jsonb)
            "#,
        )
        .bind(job_id)
        .bind(event_type)
        .bind(payload_json)
        .execute(self.pool())
        .await
        .with_context(|| format!("failed to record event {event_type} for job {job_id}"))?;

        Ok(())
    }

    pub async fn mark_job_expired(&self, job_id: &str) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            UPDATE mux_jobs
            SET status = $2, updated_at_ms = $3, completed_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(job_id)
        .bind(JobStatus::Expired.as_str())
        .bind(now_ms())
        .execute(self.pool())
        .await
        .with_context(|| format!("failed to expire job {job_id}"))?;
        Ok(())
    }
}
