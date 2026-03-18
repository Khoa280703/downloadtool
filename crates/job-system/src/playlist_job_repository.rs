use anyhow::{Context, Result};
use serde_json::json;
use sqlx::PgPool;

use crate::playlist_job_models::{
    CreatePlaylistJobRequest, PlaylistItemStatus, PlaylistJobItemRecord, PlaylistJobRecord,
    PlaylistJobStatus,
};
use crate::repository::now_ms;

/// Repository for playlist job CRUD operations.
pub struct PlaylistJobRepository {
    pool: PgPool,
    sequence: std::sync::atomic::AtomicU64,
}

impl PlaylistJobRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            sequence: std::sync::atomic::AtomicU64::new(0),
        }
    }

    fn next_id(&self, prefix: &str) -> String {
        let seq = self
            .sequence
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        format!("{prefix}-{}-{seq}", now_ms())
    }

    /// Create a new playlist job. Returns the created record.
    pub async fn create_job(
        &self,
        request: &CreatePlaylistJobRequest,
        user_id: Option<&str>,
        session_id: Option<&str>,
        request_ip: Option<&str>,
    ) -> Result<PlaylistJobRecord> {
        let id = self.next_id("pl");
        let now = now_ms();
        let quality = request.requested_quality.as_deref().unwrap_or("best");
        let mode = request.requested_mode.as_deref().unwrap_or("auto");
        let status = PlaylistJobStatus::Queued.as_str();

        sqlx::query(
            r#"
            INSERT INTO playlist_jobs (id, source_url, status, requested_quality, requested_mode,
                                       user_id, session_id, request_ip, created_at_ms, updated_at_ms)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
        )
        .bind(&id)
        .bind(&request.source_url)
        .bind(status)
        .bind(quality)
        .bind(mode)
        .bind(user_id)
        .bind(session_id)
        .bind(request_ip)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await
        .context("failed to insert playlist job")?;

        Ok(PlaylistJobRecord {
            id,
            source_url: request.source_url.clone(),
            title: None,
            status: PlaylistJobStatus::Queued,
            total_items: 0,
            completed_items: 0,
            failed_items: 0,
            requested_quality: quality.to_string(),
            requested_mode: mode.to_string(),
            user_id: user_id.map(String::from),
            session_id: session_id.map(String::from),
            request_ip: request_ip.map(String::from),
            created_at_ms: now,
            updated_at_ms: now,
        })
    }

    /// Get a playlist job by ID, scoped to owner (user_id or session_id).
    pub async fn get_job(
        &self,
        job_id: &str,
        user_id: Option<&str>,
        session_id: Option<&str>,
    ) -> Result<Option<PlaylistJobRecord>> {
        let row: Option<PlaylistJobRow> = sqlx::query_as(
            r#"
            SELECT id, source_url, title, status, total_items, completed_items, failed_items,
                   requested_quality, requested_mode, user_id, session_id, request_ip,
                   created_at_ms, updated_at_ms
            FROM playlist_jobs
            WHERE id = $1
              AND (user_id = $2 OR session_id = $3 OR ($2 IS NULL AND $3 IS NULL))
            "#,
        )
        .bind(job_id)
        .bind(user_id)
        .bind(session_id)
        .fetch_optional(&self.pool)
        .await
        .context("failed to fetch playlist job")?;

        Ok(row.map(PlaylistJobRow::into_record))
    }

    /// Update playlist job status.
    pub async fn update_job_status(
        &self,
        job_id: &str,
        status: PlaylistJobStatus,
    ) -> Result<()> {
        sqlx::query(
            "UPDATE playlist_jobs SET status = $2, updated_at_ms = $3, updated_at = NOW() WHERE id = $1",
        )
        .bind(job_id)
        .bind(status.as_str())
        .bind(now_ms())
        .execute(&self.pool)
        .await
        .context("failed to update playlist job status")?;
        Ok(())
    }

    /// Set title and total_items after discovery, transition to Processing.
    pub async fn set_discovery_result(
        &self,
        job_id: &str,
        title: Option<&str>,
        total_items: i32,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE playlist_jobs
            SET title = COALESCE($2, title), total_items = $3, status = $4,
                updated_at_ms = $5, updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(job_id)
        .bind(title)
        .bind(total_items)
        .bind(PlaylistJobStatus::Ready.as_str())
        .bind(now_ms())
        .execute(&self.pool)
        .await
        .context("failed to set discovery result")?;
        Ok(())
    }

    /// Increment completed_items counter and return updated record.
    pub async fn increment_completed(&self, job_id: &str) -> Result<PlaylistJobRecord> {
        let row: PlaylistJobRow = sqlx::query_as(
            r#"
            UPDATE playlist_jobs
            SET completed_items = completed_items + 1, updated_at_ms = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING id, source_url, title, status, total_items, completed_items, failed_items,
                      requested_quality, requested_mode, user_id, session_id, request_ip,
                      created_at_ms, updated_at_ms
            "#,
        )
        .bind(job_id)
        .bind(now_ms())
        .fetch_one(&self.pool)
        .await
        .context("failed to increment completed items")?;
        Ok(row.into_record())
    }

    /// Increment failed_items counter and return updated record.
    pub async fn increment_failed(&self, job_id: &str) -> Result<PlaylistJobRecord> {
        let row: PlaylistJobRow = sqlx::query_as(
            r#"
            UPDATE playlist_jobs
            SET failed_items = failed_items + 1, updated_at_ms = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING id, source_url, title, status, total_items, completed_items, failed_items,
                      requested_quality, requested_mode, user_id, session_id, request_ip,
                      created_at_ms, updated_at_ms
            "#,
        )
        .bind(job_id)
        .bind(now_ms())
        .fetch_one(&self.pool)
        .await
        .context("failed to increment failed items")?;
        Ok(row.into_record())
    }

    /// Insert discovered items in bulk within a transaction.
    pub async fn insert_items(
        &self,
        job_id: &str,
        items: &[(String, String, Option<String>, i32)], // (video_id, title, thumbnail, ordinal)
    ) -> Result<Vec<PlaylistJobItemRecord>> {
        let now = now_ms();
        let mut records = Vec::with_capacity(items.len());
        let mut tx = self.pool.begin().await.context("failed to begin transaction")?;

        for (video_id, title, thumbnail, ordinal) in items {
            let id = self.next_id("pli");
            sqlx::query(
                r#"
                INSERT INTO playlist_job_items
                    (id, playlist_job_id, video_id, title, thumbnail, ordinal, status, selected_stream_meta, created_at_ms, updated_at_ms)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                "#,
            )
            .bind(&id)
            .bind(job_id)
            .bind(video_id)
            .bind(title)
            .bind(thumbnail.as_deref())
            .bind(ordinal)
            .bind(PlaylistItemStatus::Pending.as_str())
            .bind(json!({ "selected": true }))
            .bind(now)
            .bind(now)
            .execute(&mut *tx)
            .await
            .context("failed to insert playlist job item")?;

            records.push(PlaylistJobItemRecord {
                id,
                playlist_job_id: job_id.to_string(),
                video_id: video_id.clone(),
                title: Some(title.clone()),
                thumbnail: thumbnail.clone(),
                ordinal: *ordinal,
                status: PlaylistItemStatus::Pending,
                attempt_count: 0,
                last_error: None,
                selected_stream_meta: Some(json!({ "selected": true })),
                mux_job_id: None,
                artifact_key: None,
                download_url: None,
                created_at_ms: now,
                updated_at_ms: now,
            });
        }

        tx.commit().await.context("failed to commit item inserts")?;
        Ok(records)
    }

    /// Get all items for a playlist job ordered by ordinal.
    pub async fn get_items(&self, job_id: &str) -> Result<Vec<PlaylistJobItemRecord>> {
        let rows: Vec<PlaylistJobItemRow> = sqlx::query_as(
            r#"
            SELECT id, playlist_job_id, video_id, title, thumbnail, ordinal, status,
                   attempt_count, last_error, selected_stream_meta, mux_job_id,
                   artifact_key, download_url, created_at_ms, updated_at_ms
            FROM playlist_job_items
            WHERE playlist_job_id = $1
            ORDER BY ordinal ASC
            "#,
        )
        .bind(job_id)
        .fetch_all(&self.pool)
        .await
        .context("failed to fetch playlist job items")?;

        Ok(rows.into_iter().map(PlaylistJobItemRow::into_record).collect())
    }

    /// Claim next pending item for processing (SELECT FOR UPDATE SKIP LOCKED).
    pub async fn claim_next_pending_item(
        &self,
        job_id: &str,
    ) -> Result<Option<PlaylistJobItemRecord>> {
        let row: Option<PlaylistJobItemRow> = sqlx::query_as(
            r#"
            UPDATE playlist_job_items
            SET status = $3, attempt_count = attempt_count + 1, updated_at_ms = $4, updated_at = NOW()
            WHERE id = (
                SELECT id FROM playlist_job_items
                WHERE playlist_job_id = $1 AND status = $2
                ORDER BY ordinal ASC
                LIMIT 1
                FOR UPDATE SKIP LOCKED
            )
            RETURNING id, playlist_job_id, video_id, title, thumbnail, ordinal, status,
                      attempt_count, last_error, selected_stream_meta, mux_job_id,
                      artifact_key, download_url, created_at_ms, updated_at_ms
            "#,
        )
        .bind(job_id)
        .bind(PlaylistItemStatus::Pending.as_str())
        .bind(PlaylistItemStatus::Extracting.as_str())
        .bind(now_ms())
        .fetch_optional(&self.pool)
        .await
        .context("failed to claim next pending item")?;

        Ok(row.map(PlaylistJobItemRow::into_record))
    }

    /// Update item status with optional fields.
    pub async fn update_item_status(
        &self,
        item_id: &str,
        status: PlaylistItemStatus,
        last_error: Option<&str>,
        mux_job_id: Option<&str>,
        download_url: Option<&str>,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE playlist_job_items
            SET status = $2,
                last_error = COALESCE($3, last_error),
                mux_job_id = COALESCE($4, mux_job_id),
                download_url = COALESCE($5, download_url),
                updated_at_ms = $6, updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(item_id)
        .bind(status.as_str())
        .bind(last_error)
        .bind(mux_job_id)
        .bind(download_url)
        .bind(now_ms())
        .execute(&self.pool)
        .await
        .context("failed to update playlist job item")?;
        Ok(())
    }

    /// Find all non-terminal playlist jobs (queued, discovering, processing).
    /// Used on server startup to re-spawn orphaned processors.
    pub async fn find_active_jobs(&self) -> Result<Vec<PlaylistJobRecord>> {
        let rows: Vec<PlaylistJobRow> = sqlx::query_as(
            r#"
            SELECT id, source_url, title, status, total_items, completed_items, failed_items,
                   requested_quality, requested_mode, user_id, session_id, request_ip,
                   created_at_ms, updated_at_ms
            FROM playlist_jobs
            WHERE status IN ($1, $2, $3)
            ORDER BY created_at_ms ASC
            "#,
        )
        .bind(PlaylistJobStatus::Queued.as_str())
        .bind(PlaylistJobStatus::Discovering.as_str())
        .bind(PlaylistJobStatus::Processing.as_str())
        .fetch_all(&self.pool)
        .await
        .context("failed to fetch active playlist jobs")?;

        Ok(rows.into_iter().map(PlaylistJobRow::into_record).collect())
    }

    /// Reset in-progress items back to pending for a job (recovery after crash).
    pub async fn reset_extracting_items_to_pending(&self, job_id: &str) -> Result<i32> {
        let result = sqlx::query(
            r#"
            UPDATE playlist_job_items
            SET status = $3, updated_at_ms = $4, updated_at = NOW()
            WHERE playlist_job_id = $1 AND status = $2
            "#,
        )
        .bind(job_id)
        .bind(PlaylistItemStatus::Extracting.as_str())
        .bind(PlaylistItemStatus::Pending.as_str())
        .bind(now_ms())
        .execute(&self.pool)
        .await
        .context("failed to reset extracting items")?;
        Ok(result.rows_affected() as i32)
    }

    /// Cancel all pending items for a playlist job.
    pub async fn cancel_pending_items(&self, job_id: &str) -> Result<i32> {
        let result = sqlx::query(
            r#"
            UPDATE playlist_job_items
            SET status = $3, updated_at_ms = $4, updated_at = NOW()
            WHERE playlist_job_id = $1 AND status = $2
            "#,
        )
        .bind(job_id)
        .bind(PlaylistItemStatus::Pending.as_str())
        .bind(PlaylistItemStatus::Cancelled.as_str())
        .bind(now_ms())
        .execute(&self.pool)
        .await
        .context("failed to cancel pending items")?;
        Ok(result.rows_affected() as i32)
    }

    /// Persist selected items before processing starts.
    pub async fn prepare_items_for_start(
        &self,
        job_id: &str,
        selected_video_ids: &[String],
    ) -> Result<i32> {
        let result = sqlx::query(
            r#"
            UPDATE playlist_job_items
            SET selected_stream_meta = jsonb_build_object('selected', video_id = ANY($2::text[])),
                status = CASE
                    WHEN video_id = ANY($2::text[]) THEN $3
                    ELSE $4
                END,
                last_error = NULL,
                mux_job_id = NULL,
                download_url = NULL,
                updated_at_ms = $5,
                updated_at = NOW()
            WHERE playlist_job_id = $1
            "#,
        )
        .bind(job_id)
        .bind(selected_video_ids)
        .bind(PlaylistItemStatus::Pending.as_str())
        .bind(PlaylistItemStatus::Cancelled.as_str())
        .bind(now_ms())
        .execute(&self.pool)
        .await
        .context("failed to prepare playlist items for start")?;

        Ok(result.rows_affected() as i32)
    }

    /// Transition playlist job into processing with selected total.
    pub async fn start_processing(
        &self,
        job_id: &str,
        selected_total: i32,
        requested_quality: &str,
        requested_mode: &str,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE playlist_jobs
            SET status = $2,
                total_items = $3,
                completed_items = 0,
                failed_items = 0,
                requested_quality = $4,
                requested_mode = $5,
                updated_at_ms = $6,
                updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(job_id)
        .bind(PlaylistJobStatus::Processing.as_str())
        .bind(selected_total)
        .bind(requested_quality)
        .bind(requested_mode)
        .bind(now_ms())
        .execute(&self.pool)
        .await
        .context("failed to mark playlist job as processing")?;
        Ok(())
    }
}

// -- Internal row types for sqlx FromRow mapping --

#[derive(sqlx::FromRow)]
struct PlaylistJobRow {
    id: String,
    source_url: String,
    title: Option<String>,
    status: String,
    total_items: i32,
    completed_items: i32,
    failed_items: i32,
    requested_quality: String,
    requested_mode: String,
    user_id: Option<String>,
    session_id: Option<String>,
    request_ip: Option<String>,
    created_at_ms: i64,
    updated_at_ms: i64,
}

impl PlaylistJobRow {
    fn into_record(self) -> PlaylistJobRecord {
        PlaylistJobRecord {
            id: self.id,
            source_url: self.source_url,
            title: self.title,
            status: PlaylistJobStatus::from_str(&self.status),
            total_items: self.total_items,
            completed_items: self.completed_items,
            failed_items: self.failed_items,
            requested_quality: self.requested_quality,
            requested_mode: self.requested_mode,
            user_id: self.user_id,
            session_id: self.session_id,
            request_ip: self.request_ip,
            created_at_ms: self.created_at_ms,
            updated_at_ms: self.updated_at_ms,
        }
    }
}

#[derive(sqlx::FromRow)]
struct PlaylistJobItemRow {
    id: String,
    playlist_job_id: String,
    video_id: String,
    title: Option<String>,
    thumbnail: Option<String>,
    ordinal: i32,
    status: String,
    attempt_count: i32,
    last_error: Option<String>,
    selected_stream_meta: Option<serde_json::Value>,
    mux_job_id: Option<String>,
    artifact_key: Option<String>,
    download_url: Option<String>,
    created_at_ms: i64,
    updated_at_ms: i64,
}

impl PlaylistJobItemRow {
    fn into_record(self) -> PlaylistJobItemRecord {
        PlaylistJobItemRecord {
            id: self.id,
            playlist_job_id: self.playlist_job_id,
            video_id: self.video_id,
            title: self.title,
            thumbnail: self.thumbnail,
            ordinal: self.ordinal,
            status: PlaylistItemStatus::from_str(&self.status),
            attempt_count: self.attempt_count,
            last_error: self.last_error,
            selected_stream_meta: self.selected_stream_meta,
            mux_job_id: self.mux_job_id,
            artifact_key: self.artifact_key,
            download_url: self.download_url,
            created_at_ms: self.created_at_ms,
            updated_at_ms: self.updated_at_ms,
        }
    }
}
