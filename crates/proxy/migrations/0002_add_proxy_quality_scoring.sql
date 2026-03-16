ALTER TABLE proxies
    ADD COLUMN IF NOT EXISTS health_score INTEGER NOT NULL DEFAULT 100,
    ADD COLUMN IF NOT EXISTS auto_disabled_at TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS auto_disabled_reason TEXT;

ALTER TABLE proxies
    DROP CONSTRAINT IF EXISTS proxies_health_score_check;

ALTER TABLE proxies
    ADD CONSTRAINT proxies_health_score_check
        CHECK (health_score >= 0 AND health_score <= 100);

CREATE INDEX IF NOT EXISTS idx_proxies_health_score
    ON proxies(health_score ASC, updated_at DESC);
