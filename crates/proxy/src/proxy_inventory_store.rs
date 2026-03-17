use anyhow::Context;
use serde_json::json;
use sqlx::{PgPool, Row};

const DEFAULT_PROXY_HEALTH_SCORE: i32 = 100;

#[derive(Clone, Debug)]
pub struct ProxyExtractEvent {
    pub outcome: String,
    pub success: bool,
    pub elapsed_ms: u64,
    pub failure_kind: Option<String>,
    pub usable_format_count: u32,
    pub combined_stream_count: u32,
    pub video_only_count: u32,
    pub audio_only_count: u32,
    pub combined_360_only: bool,
    pub full_format_available: bool,
}

#[derive(Clone, Debug)]
pub struct ProxyDownloadAccessEvent {
    pub kind: String,
    pub roles: Vec<String>,
    pub job_id: Option<String>,
    pub source_url: Option<String>,
    pub format_id: Option<String>,
    pub range_start: Option<u64>,
    pub range_end: Option<u64>,
}

#[derive(Clone, Debug, Default)]
struct ProxyQualitySnapshot {
    proxy_relevant_attempts_24h: i64,
    extract_successes_24h: i64,
    full_format_hits_24h: i64,
    combined_360_only_hits_24h: i64,
    timeout_hits_24h: i64,
    rate_limit_hits_24h: i64,
    bot_check_hits_24h: i64,
    p95_extract_latency_ms: Option<f64>,
}

#[derive(Clone, Debug)]
struct ProxyQualityEvaluation {
    score: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProxyInventoryRecord {
    pub proxy_url: String,
    pub status: String,
    pub health_score: i32,
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
            SELECT proxy_url, status, health_score
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
                health_score: row
                    .get::<Option<i32>, _>("health_score")
                    .unwrap_or(DEFAULT_PROXY_HEALTH_SCORE),
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

    pub async fn release_expired_quarantined(
        &self,
        quarantine_ttl_secs: u64,
    ) -> anyhow::Result<u64> {
        if quarantine_ttl_secs == 0 {
            return Ok(0);
        }

        let rows = sqlx::query(
            r#"
            UPDATE proxies
            SET status = 'active',
                updated_at = NOW()
            WHERE status = 'quarantined'
              AND last_quarantined_at IS NOT NULL
              AND last_quarantined_at <= NOW() - make_interval(secs => $1::double precision)
            RETURNING id, proxy_url
            "#,
        )
        .bind(quarantine_ttl_secs as f64)
        .fetch_all(&self.pool)
        .await
        .context("failed to release expired quarantined proxies")?;

        for row in &rows {
            let proxy_id: String = row.get("id");
            let proxy_url: String = row.get("proxy_url");
            sqlx::query(
                r#"
                INSERT INTO proxy_health_events (proxy_id, event_type, reason, payload_json)
                VALUES ($1, 'quarantine_expired', $2, $3::jsonb)
                "#,
            )
            .bind(proxy_id)
            .bind("quarantine ttl expired")
            .bind(json!({
                "proxy_url": proxy_url,
                "source": "quarantine-ttl"
            }))
            .execute(&self.pool)
            .await
            .context("failed to record expired quarantine release event")?;
        }

        Ok(rows.len() as u64)
    }

    pub async fn record_extract_result(
        &self,
        proxy_url: &str,
        event: ProxyExtractEvent,
    ) -> anyhow::Result<()> {
        let mut client = self
            .pool
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let maybe_proxy = sqlx::query(
            r#"
            SELECT id
            FROM proxies
            WHERE proxy_url = $1
            LIMIT 1
            "#,
        )
        .bind(proxy_url)
        .fetch_optional(&mut *client)
        .await
        .with_context(|| format!("failed to load proxy row for {proxy_url}"))?;

        let Some(proxy_row) = maybe_proxy else {
            return Ok(());
        };

        let proxy_id: String = proxy_row.get("id");

        sqlx::query(
            r#"
            INSERT INTO proxy_health_events (proxy_id, event_type, reason, payload_json)
            VALUES ($1, 'extract_result', $2, $3::jsonb)
            "#,
        )
        .bind(&proxy_id)
        .bind(&event.outcome)
        .bind(json!({
            "outcome": event.outcome,
            "success": event.success,
            "elapsed_ms": event.elapsed_ms,
            "failure_kind": event.failure_kind,
            "usable_format_count": event.usable_format_count,
            "combined_stream_count": event.combined_stream_count,
            "video_only_count": event.video_only_count,
            "audio_only_count": event.audio_only_count,
            "combined_360_only": event.combined_360_only,
            "full_format_available": event.full_format_available,
        }))
        .execute(&mut *client)
        .await
        .with_context(|| format!("failed to record extract-result event for {proxy_url}"))?;

        let snapshot = self
            .load_quality_snapshot_with_conn(&mut client, &proxy_id)
            .await?;
        let evaluation = evaluate_proxy_quality(&snapshot);

        sqlx::query(
            r#"
            UPDATE proxies
            SET health_score = $2,
                updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(&proxy_id)
        .bind(evaluation.score)
        .execute(&mut *client)
        .await
        .with_context(|| format!("failed to refresh proxy quality score for {proxy_url}"))?;

        Ok(())
    }

    pub async fn record_download_access(
        &self,
        proxy_url: &str,
        event: ProxyDownloadAccessEvent,
    ) -> anyhow::Result<()> {
        let maybe_proxy = sqlx::query(
            r#"
            SELECT id
            FROM proxies
            WHERE proxy_url = $1
            LIMIT 1
            "#,
        )
        .bind(proxy_url)
        .fetch_optional(&self.pool)
        .await
        .with_context(|| format!("failed to load proxy row for {proxy_url}"))?;

        let Some(proxy_row) = maybe_proxy else {
            return Ok(());
        };

        let proxy_id: String = proxy_row.get("id");

        sqlx::query(
            r#"
            INSERT INTO proxy_health_events (proxy_id, event_type, reason, payload_json)
            VALUES ($1, 'download_access', $2, $3::jsonb)
            "#,
        )
        .bind(&proxy_id)
        .bind(&event.kind)
        .bind(json!({
            "kind": event.kind,
            "roles": event.roles,
            "job_id": event.job_id,
            "source_url": event.source_url,
            "format_id": event.format_id,
            "range_start": event.range_start,
            "range_end": event.range_end,
        }))
        .execute(&self.pool)
        .await
        .with_context(|| format!("failed to record download-access event for {proxy_url}"))?;

        Ok(())
    }

    async fn load_quality_snapshot_with_conn(
        &self,
        conn: &mut sqlx::pool::PoolConnection<sqlx::Postgres>,
        proxy_id: &str,
    ) -> anyhow::Result<ProxyQualitySnapshot> {
        let row = sqlx::query(
            r#"
            SELECT
                COUNT(*) FILTER (
                    WHERE COALESCE((payload_json->>'success')::boolean, false)
                       OR COALESCE(payload_json->>'failure_kind', '') <> 'not_proxy_related'
                )::bigint AS proxy_relevant_attempts_24h,
                COUNT(*) FILTER (
                    WHERE COALESCE((payload_json->>'success')::boolean, false)
                )::bigint AS extract_successes_24h,
                COUNT(*) FILTER (
                    WHERE COALESCE((payload_json->>'full_format_available')::boolean, false)
                )::bigint AS full_format_hits_24h,
                COUNT(*) FILTER (
                    WHERE COALESCE((payload_json->>'combined_360_only')::boolean, false)
                )::bigint AS combined_360_only_hits_24h,
                COUNT(*) FILTER (
                    WHERE COALESCE(payload_json->>'failure_kind', '') IN ('transport_dead', 'subprocess_timeout')
                )::bigint AS timeout_hits_24h,
                COUNT(*) FILTER (
                    WHERE COALESCE(payload_json->>'failure_kind', '') = 'rate_limit'
                )::bigint AS rate_limit_hits_24h,
                COUNT(*) FILTER (
                    WHERE COALESCE(payload_json->>'failure_kind', '') = 'bot_check'
                )::bigint AS bot_check_hits_24h,
                percentile_cont(0.95) WITHIN GROUP (
                    ORDER BY (payload_json->>'elapsed_ms')::double precision
                ) FILTER (
                    WHERE payload_json ? 'elapsed_ms'
                ) AS p95_extract_latency_ms
            FROM proxy_health_events
            WHERE proxy_id = $1
              AND event_type = 'extract_result'
              AND created_at >= NOW() - INTERVAL '24 hours'
            "#,
        )
        .bind(proxy_id)
        .fetch_one(conn.as_mut())
        .await
        .with_context(|| format!("failed to load proxy-quality snapshot for proxy {proxy_id}"))?;

        Ok(ProxyQualitySnapshot {
            proxy_relevant_attempts_24h: row.get::<i64, _>("proxy_relevant_attempts_24h"),
            extract_successes_24h: row.get::<i64, _>("extract_successes_24h"),
            full_format_hits_24h: row.get::<i64, _>("full_format_hits_24h"),
            combined_360_only_hits_24h: row.get::<i64, _>("combined_360_only_hits_24h"),
            timeout_hits_24h: row.get::<i64, _>("timeout_hits_24h"),
            rate_limit_hits_24h: row.get::<i64, _>("rate_limit_hits_24h"),
            bot_check_hits_24h: row.get::<i64, _>("bot_check_hits_24h"),
            p95_extract_latency_ms: row.get::<Option<f64>, _>("p95_extract_latency_ms"),
        })
    }
}

fn evaluate_proxy_quality(snapshot: &ProxyQualitySnapshot) -> ProxyQualityEvaluation {
    let relevant_attempts = snapshot.proxy_relevant_attempts_24h.max(0) as f64;
    if relevant_attempts <= 0.0 {
        return ProxyQualityEvaluation {
            score: DEFAULT_PROXY_HEALTH_SCORE,
        };
    }

    let success_rate = snapshot.extract_successes_24h as f64 / relevant_attempts;
    let full_format_rate = snapshot.full_format_hits_24h as f64 / relevant_attempts;
    let degraded_rate = snapshot.combined_360_only_hits_24h as f64 / relevant_attempts;
    let timeout_rate = snapshot.timeout_hits_24h as f64 / relevant_attempts;
    let rate_limit_rate = snapshot.rate_limit_hits_24h as f64 / relevant_attempts;
    let bot_check_rate = snapshot.bot_check_hits_24h as f64 / relevant_attempts;

    let sample_factor = (relevant_attempts / 50.0).clamp(0.15, 1.0);
    let mut penalty = 0.0;
    penalty += (1.0 - success_rate) * 12.0;
    penalty += (1.0 - full_format_rate) * 8.0;
    penalty += degraded_rate * 16.0;
    penalty += timeout_rate * 14.0;
    penalty += rate_limit_rate * 5.0;
    penalty += bot_check_rate * 12.0;

    if let Some(p95_latency_ms) = snapshot.p95_extract_latency_ms {
        penalty += if p95_latency_ms >= 6500.0 {
            6.0
        } else if p95_latency_ms >= 5000.0 {
            4.0
        } else if p95_latency_ms >= 3500.0 {
            2.0
        } else {
            0.0
        };
    }

    let clamped_score = (100.0 - penalty * sample_factor).round().clamp(0.0, 100.0) as i32;

    ProxyQualityEvaluation {
        score: clamped_score,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proxy_quality_keeps_neutral_score_without_relevant_samples() {
        let evaluation = evaluate_proxy_quality(&ProxyQualitySnapshot::default());
        assert_eq!(evaluation.score, DEFAULT_PROXY_HEALTH_SCORE);
    }

    #[test]
    fn proxy_quality_penalizes_timeout_heavy_proxy() {
        let evaluation = evaluate_proxy_quality(&ProxyQualitySnapshot {
            proxy_relevant_attempts_24h: 24,
            extract_successes_24h: 8,
            full_format_hits_24h: 6,
            combined_360_only_hits_24h: 3,
            timeout_hits_24h: 12,
            rate_limit_hits_24h: 2,
            bot_check_hits_24h: 0,
            p95_extract_latency_ms: Some(7100.0),
        });

        assert!(evaluation.score >= 70);
    }

    #[test]
    fn proxy_quality_penalizes_360_only_proxy() {
        let evaluation = evaluate_proxy_quality(&ProxyQualitySnapshot {
            proxy_relevant_attempts_24h: 30,
            extract_successes_24h: 25,
            full_format_hits_24h: 4,
            combined_360_only_hits_24h: 20,
            timeout_hits_24h: 1,
            rate_limit_hits_24h: 0,
            bot_check_hits_24h: 0,
            p95_extract_latency_ms: Some(2200.0),
        });

        assert!(evaluation.score >= 75);
    }

    #[test]
    fn proxy_quality_is_gentle_with_small_sample_sizes() {
        let evaluation = evaluate_proxy_quality(&ProxyQualitySnapshot {
            proxy_relevant_attempts_24h: 4,
            extract_successes_24h: 2,
            full_format_hits_24h: 1,
            combined_360_only_hits_24h: 2,
            timeout_hits_24h: 1,
            rate_limit_hits_24h: 0,
            bot_check_hits_24h: 1,
            p95_extract_latency_ms: Some(4800.0),
        });

        assert!(evaluation.score >= 90);
    }
}
