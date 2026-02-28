CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS subscriptions (
    id                 TEXT PRIMARY KEY DEFAULT gen_random_uuid()::TEXT,
    user_id            TEXT NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    plan               TEXT NOT NULL DEFAULT 'free',
    status             TEXT NOT NULL DEFAULT 'active',
    current_period_end TIMESTAMPTZ,
    whop_membership_id TEXT,
    whop_updated_at    TIMESTAMPTZ,
    created_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id)
);

CREATE INDEX IF NOT EXISTS idx_subscriptions_user_id ON subscriptions(user_id);
