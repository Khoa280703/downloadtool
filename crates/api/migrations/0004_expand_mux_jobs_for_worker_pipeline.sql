CREATE TABLE IF NOT EXISTS mux_job_events (
    id           TEXT PRIMARY KEY DEFAULT gen_random_uuid()::TEXT,
    job_id       TEXT NOT NULL REFERENCES mux_jobs(id) ON DELETE CASCADE,
    event_type   TEXT NOT NULL,
    payload_json JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE mux_jobs
    ADD COLUMN IF NOT EXISTS lease_owner TEXT,
    ADD COLUMN IF NOT EXISTS lease_expires_at_ms BIGINT;

ALTER TABLE mux_artifacts
    ALTER COLUMN artifact_key DROP NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS idx_mux_artifacts_artifact_key_unique
    ON mux_artifacts(artifact_key)
    WHERE artifact_key IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_mux_jobs_lease_expires_at_ms
    ON mux_jobs(lease_expires_at_ms);

CREATE INDEX IF NOT EXISTS idx_mux_job_events_job_id_created_at
    ON mux_job_events(job_id, created_at DESC);
