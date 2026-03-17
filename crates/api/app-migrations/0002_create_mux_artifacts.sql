CREATE TABLE IF NOT EXISTS mux_artifacts (
    id               TEXT PRIMARY KEY DEFAULT gen_random_uuid()::TEXT,
    artifact_key     TEXT NOT NULL UNIQUE,
    dedupe_key       TEXT,
    backend          TEXT NOT NULL DEFAULT 'localfs',
    local_path       TEXT,
    storage_bucket   TEXT,
    object_key       TEXT,
    content_type     TEXT NOT NULL DEFAULT 'video/mp4',
    status           TEXT NOT NULL DEFAULT 'building',
    size_bytes       BIGINT,
    etag             TEXT,
    sha256           TEXT,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ready_at         TIMESTAMPTZ,
    expires_at       TIMESTAMPTZ,
    last_accessed_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_mux_artifacts_dedupe_key
    ON mux_artifacts(dedupe_key);

CREATE INDEX IF NOT EXISTS idx_mux_artifacts_status
    ON mux_artifacts(status);

CREATE INDEX IF NOT EXISTS idx_mux_artifacts_expires_at
    ON mux_artifacts(expires_at);
