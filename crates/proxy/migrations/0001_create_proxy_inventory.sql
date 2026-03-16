CREATE TABLE IF NOT EXISTS proxies (
    id                     TEXT PRIMARY KEY DEFAULT gen_random_uuid()::TEXT,
    proxy_url              TEXT NOT NULL UNIQUE,
    display_name           TEXT,
    status                 TEXT NOT NULL DEFAULT 'active',
    source                 TEXT NOT NULL DEFAULT 'manual',
    notes                  TEXT,
    last_quarantined_at    TIMESTAMPTZ,
    last_quarantine_reason TEXT,
    created_at             TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at             TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT proxies_status_check
        CHECK (status IN ('active', 'disabled', 'quarantined'))
);

CREATE INDEX IF NOT EXISTS idx_proxies_status
    ON proxies(status);

CREATE INDEX IF NOT EXISTS idx_proxies_updated_at
    ON proxies(updated_at DESC);

CREATE TABLE IF NOT EXISTS proxy_health_events (
    id           TEXT PRIMARY KEY DEFAULT gen_random_uuid()::TEXT,
    proxy_id     TEXT NOT NULL REFERENCES proxies(id) ON DELETE CASCADE,
    event_type   TEXT NOT NULL,
    reason       TEXT,
    payload_json JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_proxy_health_events_proxy_id_created_at
    ON proxy_health_events(proxy_id, created_at DESC);
