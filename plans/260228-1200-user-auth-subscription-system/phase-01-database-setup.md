---
title: "Phase 01 — Database Setup"
priority: P1
status: pending
effort: 2h
---

# Phase 01 — Database Setup (PostgreSQL + Migrations)

**Context:** `plans/reports/brainstorm-260228-1159-user-auth-subscription-system.md`
**Plan overview:** `plan.md`

## Overview

Provision PostgreSQL and create the `subscriptions` table (Rust-owned). Better Auth auto-creates its own tables (`user`, `session`, `account`, `verification`) on first run — no manual migration needed for those.

## Key Insights

- Better Auth uses its own migration system; do NOT create `user`/`session` tables manually
- `subscriptions` table is owned by Rust; use `sqlx` migrate for it
- Shared DB: both SvelteKit (Better Auth) and Rust API connect to same PostgreSQL instance
- `user_id` in `subscriptions` is a foreign key to Better Auth's `user.id` (TEXT/UUID)
- `stripe_customer_id` stored on user row — Better Auth supports custom user fields

## Requirements

### Functional
- PostgreSQL 16 instance running in Docker, same `coolify` network
- `subscriptions` table with columns matching brainstorm schema
- DB accessible to both `frontend` (SvelteKit) and `api` (Rust) containers
- Environment variable `DATABASE_URL` injected into both containers

### Non-functional
- Migration file versioned and idempotent (`CREATE TABLE IF NOT EXISTS`)
- Connection pool configured: max 5 connections per service (homeserver constraint)

## Architecture

```
docker-compose.server.yml
  postgres:
    image: postgres:16
    env: POSTGRES_DB, POSTGRES_USER, POSTGRES_PASSWORD
    volume: postgres-data
    network: coolify

  api:
    env: DATABASE_URL=postgres://...
    depends_on: postgres

  frontend:
    env: DATABASE_URL=postgres://...   ← Better Auth reads this
    depends_on: postgres
```

## Database Schema

```sql
-- Owned by Better Auth (auto-migrated, do NOT create manually):
-- user (id TEXT PK, name, email, emailVerified, image, createdAt, updatedAt)
-- session (id TEXT PK, expiresAt, token, createdAt, updatedAt, ipAddress, userAgent, userId)
-- account (id TEXT PK, accountId, providerId, userId, accessToken, refreshToken, ...)
-- verification (id TEXT PK, identifier, value, expiresAt, ...)

-- Owned by Rust API (sqlx migrate):
CREATE TABLE IF NOT EXISTS subscriptions (
    id                    TEXT PRIMARY KEY DEFAULT gen_random_uuid()::TEXT,
    user_id               TEXT NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    plan                  TEXT NOT NULL DEFAULT 'free',   -- 'free' | 'premium'
    status                TEXT NOT NULL DEFAULT 'active', -- 'active' | 'cancelled' | 'expired'
    current_period_end    TIMESTAMPTZ,
    stripe_subscription_id TEXT,
    stripe_customer_id    TEXT,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_subscriptions_user_id ON subscriptions(user_id);
CREATE INDEX IF NOT EXISTS idx_subscriptions_stripe_subscription_id ON subscriptions(stripe_subscription_id);
```

## Related Code Files

### Files to create
- `docker/Dockerfile.postgres` — optional, only if custom config needed (likely not needed, use stock image)
- `crates/api/migrations/0001_create_subscriptions.sql` — sqlx migration file

### Files to modify
- `docker/docker-compose.server.yml` — add `postgres` service, `DATABASE_URL` env vars
- `crates/api/Cargo.toml` — add `sqlx` with `postgres`, `runtime-tokio-rustls`, `uuid`, `time` features
- `Cargo.toml` (workspace) — add `sqlx` to workspace dependencies
- `crates/api/src/config.rs` — add `database_url: String` field

## Implementation Steps

1. **Add `postgres` service to `docker-compose.server.yml`**
   ```yaml
   postgres:
     image: postgres:16-alpine
     container_name: downloadtool-postgres
     environment:
       POSTGRES_DB: downloadtool
       POSTGRES_USER: downloadtool
       POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
     volumes:
       - postgres-data:/var/lib/postgresql/data
     networks:
       - coolify
     restart: unless-stopped
   ```

2. **Add `DATABASE_URL` to `api` and `frontend` services**
   ```yaml
   # api service env:
   DATABASE_URL: postgres://downloadtool:${POSTGRES_PASSWORD}@postgres:5432/downloadtool

   # frontend service env:
   DATABASE_URL: postgres://downloadtool:${POSTGRES_PASSWORD}@postgres:5432/downloadtool
   ```

3. **Add `postgres-data` volume**

4. **Add `sqlx` to workspace `Cargo.toml`**
   ```toml
   sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "postgres", "uuid", "time", "migrate"] }
   ```

5. **Add `sqlx` to `crates/api/Cargo.toml`**

6. **Update `crates/api/src/config.rs`** — add `database_url` field, read `DATABASE_URL` env var

7. **Create `crates/api/migrations/0001_create_subscriptions.sql`** — SQL from schema section above

8. **Add `sqlx::PgPool` to API `AppState`** in `main.rs`:
   - Connect pool on startup: `sqlx::PgPool::connect(&config.database_url)`
   - Run pending migrations: `sqlx::migrate!("./migrations").run(&pool)`
   - Add pool to router state

9. **Verify**: run `docker compose up postgres` + test connection with `psql`

## Todo

- [ ] Update `docker-compose.server.yml` with postgres service
- [ ] Add `DATABASE_URL` env vars to api + frontend services
- [ ] Add `postgres-data` volume declaration
- [ ] Add `sqlx` to workspace Cargo.toml
- [ ] Add `sqlx` to `crates/api/Cargo.toml`
- [ ] Add `database_url` to `config.rs`
- [ ] Create `crates/api/migrations/0001_create_subscriptions.sql`
- [ ] Update `main.rs` to init pool + run migrations + add to state
- [ ] Create `.env.example` with `POSTGRES_PASSWORD`, `DATABASE_URL`
- [ ] Verify connection + migration via psql

## Success Criteria

- `docker compose up` starts postgres without error
- `api` container connects to DB on startup (log: "Database pool connected")
- `subscriptions` table exists after API starts
- Both services can query DB

## Risk Assessment

- **Better Auth table naming:** Better Auth uses `user` (not `users`) — `subscriptions.user_id` FK must reference `"user"(id)`. Use quoted identifier in SQL.
- **Migration ordering:** sqlx migrate runs on Rust startup; Better Auth runs on SvelteKit startup. Race condition possible on first deploy — SvelteKit must start and run BA migrations before Rust inserts any data referencing `user.id`. In practice fine since Rust only writes to `subscriptions` from Stripe webhooks (post-user-creation).
- **Connection pool exhaustion:** Homeserver has limited memory. Cap pool at 5 connections per service via `PgPoolOptions::max_connections(5)`.

## Security Considerations

- `POSTGRES_PASSWORD` only in `.env` / Coolify secrets, never hardcoded
- DB not exposed to host port (no `ports:` mapping, only internal `coolify` network)
- `DATABASE_URL` contains credentials — treat as secret in all env configs

## Next Steps

→ Phase 02: Better Auth integration (needs `DATABASE_URL` available)
