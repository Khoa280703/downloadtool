CREATE TABLE IF NOT EXISTS audit_events (
    id                  TEXT PRIMARY KEY DEFAULT gen_random_uuid()::TEXT,
    scope               TEXT NOT NULL,
    event_type          TEXT NOT NULL,
    entity_id           TEXT,
    target_label        TEXT,
    route_path          TEXT,
    method              TEXT,
    status_code         INTEGER,
    outcome             TEXT NOT NULL DEFAULT 'info',
    actor_type          TEXT NOT NULL DEFAULT 'guest',
    user_id             TEXT,
    user_email          TEXT,
    auth_session_id     TEXT,
    download_session_id TEXT,
    client_ip           TEXT,
    user_agent          TEXT,
    detail              TEXT,
    payload_json        JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT audit_events_actor_type_check
        CHECK (actor_type IN ('guest', 'user', 'system'))
);

CREATE INDEX IF NOT EXISTS idx_audit_events_created_at
    ON audit_events(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_audit_events_scope_created_at
    ON audit_events(scope, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_audit_events_user_id_created_at
    ON audit_events(user_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_audit_events_download_session_created_at
    ON audit_events(download_session_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_audit_events_route_path_created_at
    ON audit_events(route_path, created_at DESC);
