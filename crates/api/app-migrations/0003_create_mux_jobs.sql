CREATE TABLE IF NOT EXISTS mux_jobs (
    id              TEXT PRIMARY KEY,
    user_id         TEXT REFERENCES "user"(id) ON DELETE SET NULL,
    request_hash    TEXT,
    dedupe_key      TEXT,
    source_url      TEXT,
    video_url       TEXT NOT NULL,
    audio_url       TEXT NOT NULL,
    video_format_id TEXT,
    audio_format_id TEXT,
    title           TEXT,
    status          TEXT NOT NULL,
    artifact_id     TEXT REFERENCES mux_artifacts(id) ON DELETE SET NULL,
    attempt_count   INTEGER NOT NULL DEFAULT 0,
    max_attempts    INTEGER NOT NULL DEFAULT 0,
    last_error      TEXT,
    created_at_ms   BIGINT NOT NULL,
    updated_at_ms   BIGINT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at    TIMESTAMPTZ,
    delete_after_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_mux_jobs_status
    ON mux_jobs(status);

CREATE INDEX IF NOT EXISTS idx_mux_jobs_artifact_id
    ON mux_jobs(artifact_id);

CREATE INDEX IF NOT EXISTS idx_mux_jobs_delete_after_at
    ON mux_jobs(delete_after_at);

CREATE INDEX IF NOT EXISTS idx_mux_jobs_dedupe_key
    ON mux_jobs(dedupe_key);
