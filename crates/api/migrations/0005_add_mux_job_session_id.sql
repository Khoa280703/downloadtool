ALTER TABLE mux_jobs
    ADD COLUMN IF NOT EXISTS session_id TEXT;

CREATE INDEX IF NOT EXISTS idx_mux_jobs_session_id
    ON mux_jobs(session_id);

CREATE INDEX IF NOT EXISTS idx_mux_jobs_session_request_hash
    ON mux_jobs(session_id, request_hash)
    WHERE session_id IS NOT NULL;
