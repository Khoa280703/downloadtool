use anyhow::Context;
use serde_json::json;
use sqlx::{PgPool, Row};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProxyInventoryRecord {
    pub proxy_url: String,
    pub status: String,
}

#[derive(Clone)]
pub struct ProxyInventoryStore {
    pool: PgPool,
}

impl ProxyInventoryStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn count_all(&self) -> anyhow::Result<i64> {
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM proxies")
            .fetch_one(&self.pool)
            .await
            .context("failed to count proxies")?;
        Ok(count)
    }

    pub async fn list_runtime_records(&self) -> anyhow::Result<Vec<ProxyInventoryRecord>> {
        let rows = sqlx::query(
            r#"
            SELECT proxy_url, status
            FROM proxies
            WHERE status IN ('active', 'quarantined')
            ORDER BY created_at ASC, proxy_url ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .context("failed to load proxy inventory runtime records")?;

        Ok(rows
            .into_iter()
            .map(|row| ProxyInventoryRecord {
                proxy_url: row.get("proxy_url"),
                status: row.get("status"),
            })
            .collect())
    }

    pub async fn mark_urls_quarantined(
        &self,
        proxy_urls: &[String],
        reason: &str,
    ) -> anyhow::Result<u64> {
        let mut affected = 0;
        for proxy_url in proxy_urls {
            affected += self.mark_quarantined(proxy_url, reason).await? as u64;
        }
        Ok(affected)
    }

    pub async fn mark_quarantined(&self, proxy_url: &str, reason: &str) -> anyhow::Result<bool> {
        let maybe_proxy_id = sqlx::query(
            r#"
            UPDATE proxies
            SET status = 'quarantined',
                last_quarantined_at = NOW(),
                last_quarantine_reason = $2,
                updated_at = NOW()
            WHERE proxy_url = $1
              AND status <> 'quarantined'
            RETURNING id
            "#,
        )
        .bind(proxy_url)
        .bind(reason)
        .fetch_optional(&self.pool)
        .await
        .with_context(|| format!("failed to quarantine proxy {proxy_url}"))?;

        let Some(row) = maybe_proxy_id else {
            return Ok(false);
        };
        let proxy_id: String = row.get("id");

        sqlx::query(
            r#"
            INSERT INTO proxy_health_events (proxy_id, event_type, reason, payload_json)
            VALUES ($1, 'quarantined', $2, $3::jsonb)
            "#,
        )
        .bind(proxy_id)
        .bind(reason)
        .bind(json!({
            "proxy_url": proxy_url,
            "reason": reason,
        }))
        .execute(&self.pool)
        .await
        .with_context(|| format!("failed to record quarantine event for {proxy_url}"))?;

        Ok(true)
    }
}
