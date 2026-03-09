use sqlx::PgPool;

const BETTER_AUTH_SCHEMA_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS public."user" (
    id TEXT NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    "emailVerified" BOOLEAN NOT NULL,
    image TEXT,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS public.account (
    id TEXT NOT NULL,
    "accountId" TEXT NOT NULL,
    "providerId" TEXT NOT NULL,
    "userId" TEXT NOT NULL,
    "accessToken" TEXT,
    "refreshToken" TEXT,
    "idToken" TEXT,
    "accessTokenExpiresAt" TIMESTAMPTZ,
    "refreshTokenExpiresAt" TIMESTAMPTZ,
    scope TEXT,
    password TEXT,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS public.session (
    id TEXT NOT NULL,
    "expiresAt" TIMESTAMPTZ NOT NULL,
    token TEXT NOT NULL,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL,
    "ipAddress" TEXT,
    "userAgent" TEXT,
    "userId" TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS public.verification (
    id TEXT NOT NULL,
    identifier TEXT NOT NULL,
    value TEXT NOT NULL,
    "expiresAt" TIMESTAMPTZ NOT NULL,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS public.jwks (
    id TEXT NOT NULL,
    "publicKey" TEXT NOT NULL,
    "privateKey" TEXT NOT NULL,
    "createdAt" TIMESTAMPTZ NOT NULL,
    "expiresAt" TIMESTAMPTZ
);

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'user_pkey'
    ) THEN
        ALTER TABLE ONLY public."user" ADD CONSTRAINT user_pkey PRIMARY KEY (id);
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'user_email_key'
    ) THEN
        ALTER TABLE ONLY public."user" ADD CONSTRAINT user_email_key UNIQUE (email);
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'account_pkey'
    ) THEN
        ALTER TABLE ONLY public.account ADD CONSTRAINT account_pkey PRIMARY KEY (id);
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'session_pkey'
    ) THEN
        ALTER TABLE ONLY public.session ADD CONSTRAINT session_pkey PRIMARY KEY (id);
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'session_token_key'
    ) THEN
        ALTER TABLE ONLY public.session ADD CONSTRAINT session_token_key UNIQUE (token);
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'verification_pkey'
    ) THEN
        ALTER TABLE ONLY public.verification ADD CONSTRAINT verification_pkey PRIMARY KEY (id);
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'jwks_pkey'
    ) THEN
        ALTER TABLE ONLY public.jwks ADD CONSTRAINT jwks_pkey PRIMARY KEY (id);
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'account_userId_fkey'
    ) THEN
        ALTER TABLE ONLY public.account
            ADD CONSTRAINT "account_userId_fkey"
            FOREIGN KEY ("userId") REFERENCES public."user"(id) ON DELETE CASCADE;
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'session_userId_fkey'
    ) THEN
        ALTER TABLE ONLY public.session
            ADD CONSTRAINT "session_userId_fkey"
            FOREIGN KEY ("userId") REFERENCES public."user"(id) ON DELETE CASCADE;
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS "account_userId_idx" ON public.account USING btree ("userId");
CREATE INDEX IF NOT EXISTS "session_userId_idx" ON public.session USING btree ("userId");
CREATE INDEX IF NOT EXISTS verification_identifier_idx ON public.verification USING btree (identifier);
"#;

pub async fn ensure_better_auth_schema(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::raw_sql(BETTER_AUTH_SCHEMA_SQL).execute(pool).await?;
    Ok(())
}
