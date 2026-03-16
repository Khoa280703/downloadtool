use anyhow::Context;
use redis::AsyncCommands;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

use crate::proxy_pool::{FAILURE_COOLDOWN, MAX_FAILURES};

const FAIL_COUNT_TTL_SECS: u64 = FAILURE_COOLDOWN.as_secs();
const LAST_ERROR_TTL_SECS: u64 = FAILURE_COOLDOWN.as_secs() * 2;

#[derive(Clone, Debug, Default)]
pub struct ProxyRuntimeHealth {
    pub fail_count: usize,
    pub cooldown_active: bool,
}

#[derive(Clone)]
pub struct ProxyHealthStore {
    client: redis::Client,
}

impl ProxyHealthStore {
    pub fn new(redis_url: &str) -> anyhow::Result<Self> {
        Ok(Self {
            client: redis::Client::open(redis_url)
                .context("failed to create proxy redis client")?,
        })
    }

    async fn connection(&self) -> anyhow::Result<redis::aio::MultiplexedConnection> {
        self.client
            .get_multiplexed_async_connection()
            .await
            .context("failed to connect to redis for proxy health")
    }

    pub async fn load_snapshot(
        &self,
        proxy_urls: &[String],
    ) -> anyhow::Result<HashMap<String, ProxyRuntimeHealth>> {
        let mut conn = self.connection().await?;
        let mut snapshot = HashMap::with_capacity(proxy_urls.len());

        for proxy_url in proxy_urls {
            let fail_key = fail_count_key(proxy_url);
            let cooldown_key = cooldown_key(proxy_url);

            let fail_count: Option<usize> = conn
                .get(&fail_key)
                .await
                .with_context(|| format!("failed to read redis key {fail_key}"))?;
            let cooldown_active: bool = conn
                .exists(&cooldown_key)
                .await
                .with_context(|| format!("failed to read redis key {cooldown_key}"))?;

            snapshot.insert(
                proxy_url.clone(),
                ProxyRuntimeHealth {
                    fail_count: fail_count.unwrap_or_default(),
                    cooldown_active,
                },
            );
        }

        Ok(snapshot)
    }

    pub async fn mark_failed(&self, proxy_url: &str, reason: &str) -> anyhow::Result<()> {
        let mut conn = self.connection().await?;
        let fail_key = fail_count_key(proxy_url);
        let cooldown_key = cooldown_key(proxy_url);
        let last_error_key = last_error_key(proxy_url);

        let failures: i64 = conn
            .incr(&fail_key, 1)
            .await
            .with_context(|| format!("failed to increment redis key {fail_key}"))?;
        let _: bool = conn
            .expire(&fail_key, FAIL_COUNT_TTL_SECS as i64)
            .await
            .with_context(|| format!("failed to expire redis key {fail_key}"))?;
        let _: () = conn
            .set_ex(&last_error_key, reason, LAST_ERROR_TTL_SECS)
            .await
            .with_context(|| format!("failed to write redis key {last_error_key}"))?;

        if failures >= MAX_FAILURES as i64 {
            let _: () = conn
                .set_ex(&cooldown_key, "1", FAILURE_COOLDOWN.as_secs())
                .await
                .with_context(|| format!("failed to write redis key {cooldown_key}"))?;
        }

        Ok(())
    }

    /// Immediately put a proxy into cooldown without waiting for MAX_FAILURES.
    /// Use for hard transport failures (timeout, connection reset) where a single
    /// failure is enough to know the proxy is dead.
    pub async fn cooldown_now(
        &self,
        proxy_url: &str,
        reason: &str,
        ttl_secs: u64,
    ) -> anyhow::Result<()> {
        let mut conn = self.connection().await?;
        let cooldown_key = cooldown_key(proxy_url);
        let fail_key = fail_count_key(proxy_url);
        let last_error_key = last_error_key(proxy_url);

        let _: () = conn
            .set_ex(&cooldown_key, "1", ttl_secs)
            .await
            .with_context(|| format!("failed to write redis cooldown key {cooldown_key}"))?;
        let _: () = conn
            .set(&fail_key, MAX_FAILURES)
            .await
            .with_context(|| format!("failed to write redis fail count key {fail_key}"))?;
        let _: bool = conn
            .expire(&fail_key, ttl_secs as i64)
            .await
            .with_context(|| format!("failed to expire redis key {fail_key}"))?;
        let _: () = conn
            .set_ex(&last_error_key, reason, ttl_secs * 2)
            .await
            .with_context(|| format!("failed to write redis key {last_error_key}"))?;

        Ok(())
    }
}

fn proxy_key_suffix(proxy_url: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(proxy_url.as_bytes());
    hex::encode(hasher.finalize())
}

fn fail_count_key(proxy_url: &str) -> String {
    format!("proxy:fail-count:{}", proxy_key_suffix(proxy_url))
}

fn cooldown_key(proxy_url: &str) -> String {
    format!("proxy:cooldown:{}", proxy_key_suffix(proxy_url))
}

fn last_error_key(proxy_url: &str) -> String {
    format!("proxy:last-error:{}", proxy_key_suffix(proxy_url))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proxy_health_keys_are_hashed() {
        let key = fail_count_key("socks5h://user:pass@127.0.0.1:1080");
        assert!(key.starts_with("proxy:fail-count:"));
        assert!(!key.contains("user:pass"));
    }

    #[test]
    fn cooldown_matches_failure_window() {
        assert_eq!(FAIL_COUNT_TTL_SECS, FAILURE_COOLDOWN.as_secs());
    }
}
