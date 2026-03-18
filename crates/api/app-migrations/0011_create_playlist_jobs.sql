-- Playlist job: backend-orchestrated playlist download
CREATE TABLE IF NOT EXISTS playlist_jobs (
    id                  TEXT PRIMARY KEY,
    source_url          TEXT NOT NULL,
    title               TEXT,
    status              TEXT NOT NULL DEFAULT 'queued',
    total_items         INTEGER NOT NULL DEFAULT 0,
    completed_items     INTEGER NOT NULL DEFAULT 0,
    failed_items        INTEGER NOT NULL DEFAULT 0,
    requested_quality   TEXT NOT NULL DEFAULT 'best',
    requested_mode      TEXT NOT NULL DEFAULT 'auto',
    user_id             TEXT REFERENCES "user"(id) ON DELETE SET NULL,
    session_id          TEXT,
    request_ip          TEXT,
    created_at_ms       BIGINT NOT NULL,
    updated_at_ms       BIGINT NOT NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_playlist_jobs_status
    ON playlist_jobs(status);

CREATE INDEX IF NOT EXISTS idx_playlist_jobs_user_id
    ON playlist_jobs(user_id)
    WHERE user_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_playlist_jobs_session_id
    ON playlist_jobs(session_id)
    WHERE session_id IS NOT NULL;

-- Playlist job item: individual video within a playlist job
CREATE TABLE IF NOT EXISTS playlist_job_items (
    id                   TEXT PRIMARY KEY,
    playlist_job_id      TEXT NOT NULL REFERENCES playlist_jobs(id) ON DELETE CASCADE,
    video_id             TEXT NOT NULL,
    title                TEXT,
    thumbnail            TEXT,
    ordinal              INTEGER NOT NULL DEFAULT 0,
    status               TEXT NOT NULL DEFAULT 'pending',
    attempt_count        INTEGER NOT NULL DEFAULT 0,
    last_error           TEXT,
    selected_stream_meta JSONB,
    mux_job_id           TEXT,
    artifact_key         TEXT,
    download_url         TEXT,
    created_at_ms        BIGINT NOT NULL,
    updated_at_ms        BIGINT NOT NULL,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_playlist_job_items_job_id
    ON playlist_job_items(playlist_job_id);

CREATE INDEX IF NOT EXISTS idx_playlist_job_items_status
    ON playlist_job_items(playlist_job_id, status);

CREATE INDEX IF NOT EXISTS idx_playlist_job_items_video_id
    ON playlist_job_items(playlist_job_id, video_id);
