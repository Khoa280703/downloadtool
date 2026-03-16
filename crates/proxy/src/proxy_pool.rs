//! Proxy pool with round-robin rotation and shared runtime health.
//!
//! Supports both:
//! - Full proxy URLs (`socks5h://user:pass@host:port`, `http://...`)
//! - Raw records (`host:port:user:pass`) commonly used by proxy providers.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use sqlx::PgPool;
use tracing::{debug, warn};

use crate::proxy_health_store::{ProxyHealthStore, ProxyRuntimeHealth};
use crate::proxy_inventory_store::{ProxyInventoryRecord, ProxyInventoryStore};
use crate::proxy_quarantine::append_quarantine_record;

/// Maximum consecutive failures before marking proxy as unhealthy.
pub const MAX_FAILURES: usize = 3;
/// Default cooldown period for failed proxies.
pub const FAILURE_COOLDOWN: Duration = Duration::from_secs(1800);

struct ProxyEntry {
    url: String,
    quarantined: AtomicBool,
    failed_count: AtomicUsize,
    health_score: AtomicUsize,
    last_failed: RwLock<Option<Instant>>,
}

impl ProxyEntry {
    fn new(url: String, quarantined: bool, health_score: i32) -> Self {
        Self {
            url,
            quarantined: AtomicBool::new(quarantined),
            failed_count: AtomicUsize::new(0),
            health_score: AtomicUsize::new(health_score.clamp(0, 100) as usize),
            last_failed: RwLock::new(None),
        }
    }

    fn is_healthy(&self) -> bool {
        if self.is_quarantined() {
            return false;
        }

        let failures = self.failed_count.load(Ordering::Relaxed);
        if failures < MAX_FAILURES {
            return true;
        }

        if let Ok(last_failed) = self.last_failed.read() {
            if let Some(instant) = *last_failed {
                return instant.elapsed() > FAILURE_COOLDOWN;
            }
        }

        false
    }

    fn is_quarantined(&self) -> bool {
        self.quarantined.load(Ordering::Relaxed)
    }

    fn mark_failed(&self) {
        if self.is_quarantined() {
            return;
        }

        let count = self.failed_count.fetch_add(1, Ordering::Relaxed) + 1;
        if let Ok(mut last_failed) = self.last_failed.write() {
            *last_failed = Some(Instant::now());
        }
        warn!(
            "Proxy {} marked as failed (count: {}, cooldown: {}s)",
            self.url,
            count,
            FAILURE_COOLDOWN.as_secs()
        );
    }

    fn mark_success(&self) {
        if self.is_quarantined() {
            return;
        }

        let previous = self.failed_count.swap(0, Ordering::Relaxed);
        if previous > 0 {
            if let Ok(mut last_failed) = self.last_failed.write() {
                *last_failed = None;
            }
            debug!("Proxy {} recovered from failure state", self.url);
        }
    }

    fn quarantine(&self) -> bool {
        let was_quarantined = self.quarantined.swap(true, Ordering::Relaxed);
        if !was_quarantined {
            self.failed_count.store(MAX_FAILURES, Ordering::Relaxed);
            if let Ok(mut last_failed) = self.last_failed.write() {
                *last_failed = Some(Instant::now());
            }
            return true;
        }
        false
    }

    fn set_quarantined(&self, quarantined: bool) {
        self.quarantined.store(quarantined, Ordering::Relaxed);
        if !quarantined {
            self.failed_count.store(0, Ordering::Relaxed);
            if let Ok(mut last_failed) = self.last_failed.write() {
                *last_failed = None;
            }
        }
    }

    fn set_health_score(&self, health_score: i32) {
        self.health_score
            .store(health_score.clamp(0, 100) as usize, Ordering::Relaxed);
    }

    fn health_score(&self) -> usize {
        self.health_score.load(Ordering::Relaxed)
    }

    fn selection_weight(&self) -> usize {
        self.health_score().max(1)
    }

    fn apply_runtime_health(&self, health: &ProxyRuntimeHealth) {
        if self.is_quarantined() {
            return;
        }

        self.failed_count
            .store(health.fail_count.min(MAX_FAILURES), Ordering::Relaxed);

        if let Ok(mut last_failed) = self.last_failed.write() {
            *last_failed = if health.cooldown_active || health.fail_count > 0 {
                Some(Instant::now())
            } else {
                None
            };
        }
    }
}

/// Pool of proxies with round-robin selection.
pub struct ProxyPool {
    proxies: RwLock<Vec<Arc<ProxyEntry>>>,
    current: AtomicUsize,
    quarantine_file: Option<PathBuf>,
    inventory_pool: Option<PgPool>,
    health_store: Option<ProxyHealthStore>,
}

impl ProxyPool {
    /// Create a new proxy pool from a list of fully qualified proxy URLs.
    pub fn new(urls: Vec<String>) -> Self {
        let records = urls
            .into_iter()
            .map(|proxy_url| ProxyInventoryRecord {
                proxy_url,
                status: "active".to_string(),
                health_score: 100,
            })
            .collect();
        Self::new_with_runtime(records, None, None, None)
    }

    /// Create a proxy pool from a raw multi-format proxy string.
    /// Accepts comma/newline-separated entries in either full URL or `host:port:user:pass` format.
    pub fn from_raw_list(raw: &str) -> Self {
        Self::new(parse_proxy_tokens(raw))
    }

    pub fn new_with_runtime(
        inventory_records: Vec<ProxyInventoryRecord>,
        quarantine_file: Option<PathBuf>,
        inventory_pool: Option<PgPool>,
        health_store: Option<ProxyHealthStore>,
    ) -> Self {
        Self {
            proxies: RwLock::new(build_entries(inventory_records, None)),
            current: AtomicUsize::new(0),
            quarantine_file,
            inventory_pool,
            health_store,
        }
    }

    /// Get next healthy proxy by round-robin.
    pub fn next(&self) -> Option<String> {
        let start_idx = self.current.fetch_add(1, Ordering::Relaxed);
        let guard = self.proxies.read().ok()?;
        if guard.is_empty() {
            return None;
        }

        if let Some(proxy) = weighted_pick(&guard, start_idx, |entry| entry.is_healthy()) {
            return Some(proxy);
        }

        if let Some(proxy) = weighted_pick(&guard, start_idx, |entry| !entry.is_quarantined()) {
            warn!("No healthy proxies available, falling back to non-quarantined proxy");
            return Some(proxy);
        }

        warn!("No usable proxies available (all proxies quarantined)");
        None
    }

    /// Owned version of [`Self::next`], useful for async call sites.
    pub fn next_owned(&self) -> Option<String> {
        self.next()
    }

    /// Mark a specific proxy as failed.
    pub fn mark_failed(&self, proxy_url: &str) {
        if let Some(entry) = self.find_entry(proxy_url) {
            entry.mark_failed();
            if let Some(health_store) = self.health_store.clone() {
                let proxy_url = proxy_url.to_string();
                tokio::spawn(async move {
                    if let Err(error) = health_store.mark_failed(&proxy_url, "request-failed").await
                    {
                        warn!(err = %error, "Failed to persist proxy failure to redis");
                    }
                });
            }
        }
    }

    /// Mark a specific proxy as healthy again.
    pub fn mark_success(&self, proxy_url: &str) {
        if let Some(entry) = self.find_entry(proxy_url) {
            entry.mark_success();
        }
    }

    /// Quarantine a proxy and persist it for operator/admin review.
    pub fn quarantine(&self, proxy_url: &str, reason: &str) {
        if let Some(entry) = self.find_entry(proxy_url) {
            if entry.quarantine() {
                if let Some(path) = self.quarantine_file.as_ref() {
                    append_quarantine_record(path, proxy_url, reason);
                }
                if let Some(inventory_pool) = self.inventory_pool.clone() {
                    let proxy_url = proxy_url.to_string();
                    let reason = reason.to_string();
                    persist_quarantine_blocking(inventory_pool, proxy_url, reason);
                }
                warn!(
                    "Proxy {} quarantined and removed from rotation. reason={}",
                    proxy_url, reason
                );
            }
        }
    }

    /// Immediately cooldown a proxy without waiting for MAX_FAILURES.
    /// Used for hard transport failures (timeout, connection reset) where
    /// a single failure is enough to know the proxy is currently dead.
    pub fn cooldown_now(&self, proxy_url: &str, reason: &str) {
        if let Some(entry) = self.find_entry(proxy_url) {
            entry.failed_count.store(MAX_FAILURES, Ordering::Relaxed);
            if let Ok(mut last_failed) = entry.last_failed.write() {
                *last_failed = Some(Instant::now());
            }
            warn!(
                "Proxy {} immediately cooled down. reason={}",
                proxy_url, reason
            );
        }
        if let Some(health_store) = self.health_store.clone() {
            let proxy_url = proxy_url.to_string();
            let reason = reason.to_string();
            tokio::spawn(async move {
                if let Err(error) = health_store
                    .cooldown_now(&proxy_url, &reason, FAILURE_COOLDOWN.as_secs())
                    .await
                {
                    warn!(err = %error, "Failed to persist proxy immediate cooldown to redis");
                }
            });
        }
    }

    pub fn record_extract_result(&self, proxy_url: &str, event: crate::ProxyExtractEvent) {
        let Some(inventory_pool) = self.inventory_pool.clone() else {
            return;
        };

        let proxy_url = proxy_url.to_string();
        tokio::spawn(async move {
            let store = ProxyInventoryStore::new(inventory_pool);
            if let Err(error) = store.record_extract_result(&proxy_url, event).await {
                warn!(err = %error, proxy = %proxy_url, "Failed to persist proxy extract-result event");
            }
        });
    }

    pub async fn refresh_from_runtime(&self) -> anyhow::Result<()> {
        if let Some(inventory_pool) = self.inventory_pool.clone() {
            let store = ProxyInventoryStore::new(inventory_pool);
            let inventory_records = store.list_runtime_records().await?;
            self.replace_inventory(inventory_records);
        }

        if let Some(health_store) = self.health_store.clone() {
            let urls = self.all_urls();
            let snapshot = health_store.load_snapshot(&urls).await?;
            self.apply_runtime_snapshot(snapshot);
        }

        Ok(())
    }

    pub fn inventory_pool(&self) -> Option<PgPool> {
        self.inventory_pool.clone()
    }

    pub fn len(&self) -> usize {
        self.proxies
            .read()
            .map(|guard| guard.len())
            .unwrap_or_default()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_proxy_usable(&self, proxy_url: &str) -> bool {
        self.find_entry(proxy_url)
            .map(|entry| entry.is_healthy())
            .unwrap_or(false)
    }

    fn all_urls(&self) -> Vec<String> {
        self.proxies
            .read()
            .map(|guard| guard.iter().map(|entry| entry.url.clone()).collect())
            .unwrap_or_default()
    }

    fn find_entry(&self, proxy_url: &str) -> Option<Arc<ProxyEntry>> {
        self.proxies
            .read()
            .ok()
            .and_then(|guard| guard.iter().find(|entry| entry.url == proxy_url).cloned())
    }

    fn replace_inventory(&self, inventory_records: Vec<ProxyInventoryRecord>) {
        let previous_entries = self
            .proxies
            .read()
            .map(|guard| {
                guard
                    .iter()
                    .map(|entry| (entry.url.clone(), Arc::clone(entry)))
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();

        let replacement = build_entries(inventory_records, Some(&previous_entries));
        if let Ok(mut guard) = self.proxies.write() {
            *guard = replacement;
        }
    }

    fn apply_runtime_snapshot(&self, snapshot: HashMap<String, ProxyRuntimeHealth>) {
        if let Ok(guard) = self.proxies.read() {
            for entry in guard.iter() {
                let health = snapshot.get(&entry.url).cloned().unwrap_or_default();
                entry.apply_runtime_health(&health);
            }
        }
    }
}

impl Default for ProxyPool {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

pub fn parse_proxy_tokens(raw: &str) -> Vec<String> {
    raw.lines()
        .flat_map(|line| line.split(','))
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .filter_map(normalize_proxy_token)
        .collect()
}

fn build_entries(
    inventory_records: Vec<ProxyInventoryRecord>,
    previous_entries: Option<&HashMap<String, Arc<ProxyEntry>>>,
) -> Vec<Arc<ProxyEntry>> {
    inventory_records
        .into_iter()
        .filter(|record| !record.proxy_url.trim().is_empty())
        .filter(|record| record.status != "disabled")
        .map(|record| {
            if let Some(existing) =
                previous_entries.and_then(|entries| entries.get(&record.proxy_url))
            {
                match record.status.as_str() {
                    "quarantined" => existing.set_quarantined(true),
                    "active" => {
                        if !existing.is_quarantined() {
                            existing.set_quarantined(false);
                        }
                    }
                    _ => {}
                }
                existing.set_health_score(record.health_score);
                return Arc::clone(existing);
            }

            Arc::new(ProxyEntry::new(
                record.proxy_url.clone(),
                record.status == "quarantined",
                record.health_score,
            ))
        })
        .collect()
}

fn weighted_pick(
    entries: &[Arc<ProxyEntry>],
    start_idx: usize,
    predicate: impl Fn(&ProxyEntry) -> bool,
) -> Option<String> {
    let rotated_entries = entries
        .iter()
        .cycle()
        .skip(start_idx % entries.len())
        .take(entries.len());

    let mut weighted_entries = Vec::with_capacity(entries.len());
    let mut total_weight = 0usize;

    for entry in rotated_entries {
        if predicate(entry) {
            let weight = entry.selection_weight();
            total_weight += weight;
            weighted_entries.push((entry, weight));
        }
    }

    if total_weight == 0 {
        return None;
    }

    let mut ticket = start_idx % total_weight;
    for (entry, weight) in weighted_entries {
        if ticket < weight {
            return Some(entry.url.clone());
        }
        ticket -= weight;
    }

    None
}

fn persist_quarantine_blocking(inventory_pool: PgPool, proxy_url: String, reason: String) {
    let store = ProxyInventoryStore::new(inventory_pool);
    if let Ok(handle) = tokio::runtime::Handle::try_current() {
        tokio::task::block_in_place(|| {
            if let Err(error) =
                handle.block_on(async move { store.mark_quarantined(&proxy_url, &reason).await })
            {
                warn!(err = %error, "Failed to persist proxy quarantine to postgres");
            }
        });
        return;
    }

    std::thread::spawn(move || {
        let runtime = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(runtime) => runtime,
            Err(error) => {
                warn!(err = %error, "Failed to build runtime for proxy quarantine persistence");
                return;
            }
        };
        if let Err(error) =
            runtime.block_on(async move { store.mark_quarantined(&proxy_url, &reason).await })
        {
            warn!(err = %error, "Failed to persist proxy quarantine to postgres");
        }
    });
}

fn normalize_proxy_token(token: &str) -> Option<String> {
    if token.contains("://") {
        return Some(token.to_string());
    }

    let mut parts = token.rsplitn(4, ':');
    let pass = parts.next()?;
    let user = parts.next()?;
    let port = parts.next()?;
    let host = parts.next()?;

    if host.is_empty() || port.is_empty() || user.is_empty() || pass.is_empty() {
        return None;
    }

    let host = if host.contains(':') && !host.starts_with('[') && !host.ends_with(']') {
        format!("[{}]", host)
    } else {
        host.to_string()
    };

    Some(format!("socks5h://{}:{}@{}:{}", user, pass, host, port))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_proxy_pool_round_robin() {
        let pool = ProxyPool::new(vec![
            "http://proxy1:8080".to_string(),
            "http://proxy2:8080".to_string(),
            "http://proxy3:8080".to_string(),
        ]);

        assert_eq!(pool.next().unwrap(), "http://proxy1:8080");
        assert_eq!(pool.next().unwrap(), "http://proxy2:8080");
        assert_eq!(pool.next().unwrap(), "http://proxy3:8080");
        assert_eq!(pool.next().unwrap(), "http://proxy1:8080");
    }

    #[test]
    fn test_proxy_pool_empty() {
        let pool = ProxyPool::new(vec![]);
        assert!(pool.next().is_none());
        assert!(pool.is_empty());
    }

    #[test]
    fn test_proxy_failure_tracking() {
        let pool = ProxyPool::new(vec![
            "http://proxy1:8080".to_string(),
            "http://proxy2:8080".to_string(),
        ]);

        for _ in 0..MAX_FAILURES {
            pool.mark_failed("http://proxy1:8080");
        }

        assert_eq!(pool.next().unwrap(), "http://proxy2:8080");
    }

    #[test]
    fn test_proxy_success_reset() {
        let pool = ProxyPool::new(vec!["http://proxy1:8080".to_string()]);
        pool.mark_failed("http://proxy1:8080");
        pool.mark_success("http://proxy1:8080");
        assert_eq!(pool.next().unwrap(), "http://proxy1:8080");
    }

    #[test]
    fn test_proxy_quarantine_removes_from_rotation() {
        let pool = ProxyPool::new(vec![
            "http://proxy1:8080".to_string(),
            "http://proxy2:8080".to_string(),
        ]);
        pool.quarantine("http://proxy1:8080", "test");

        for _ in 0..10 {
            assert_eq!(pool.next().unwrap(), "http://proxy2:8080");
        }
    }

    #[test]
    fn test_proxy_quarantine_persists_to_file() {
        let path = std::env::temp_dir().join(format!(
            "downloadtool-proxy-quarantine-{}.txt",
            std::process::id()
        ));
        let _ = fs::remove_file(&path);

        let pool = ProxyPool::new_with_runtime(
            vec![ProxyInventoryRecord {
                proxy_url: "http://proxy1:8080".to_string(),
                status: "active".to_string(),
                health_score: 100,
            }],
            Some(path.clone()),
            None,
            None,
        );
        pool.quarantine("http://proxy1:8080", "unit-test");

        let content = fs::read_to_string(&path).expect("quarantine file should exist");
        assert!(content.contains("http://proxy1:8080"));
        assert!(content.contains("unit-test"));

        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_reused_entry_keeps_local_quarantine_until_explicit_reenable() {
        let initial = build_entries(
            vec![ProxyInventoryRecord {
                proxy_url: "http://proxy1:8080".to_string(),
                status: "quarantined".to_string(),
                health_score: 100,
            }],
            None,
        );
        assert!(initial[0].is_quarantined());

        let previous = HashMap::from([("http://proxy1:8080".to_string(), initial[0].clone())]);
        let refreshed = build_entries(
            vec![ProxyInventoryRecord {
                proxy_url: "http://proxy1:8080".to_string(),
                status: "active".to_string(),
                health_score: 100,
            }],
            Some(&previous),
        );
        assert!(refreshed[0].is_quarantined());
    }

    #[test]
    fn test_disabled_inventory_record_is_removed_from_runtime() {
        let entries = build_entries(
            vec![ProxyInventoryRecord {
                proxy_url: "http://proxy1:8080".to_string(),
                status: "disabled".to_string(),
                health_score: 100,
            }],
            None,
        );
        assert!(entries.is_empty());
    }

    #[test]
    fn test_weighted_selection_prefers_higher_score() {
        let pool = ProxyPool::new_with_runtime(
            vec![
                ProxyInventoryRecord {
                    proxy_url: "http://proxy-high:8080".to_string(),
                    status: "active".to_string(),
                    health_score: 95,
                },
                ProxyInventoryRecord {
                    proxy_url: "http://proxy-low:8080".to_string(),
                    status: "active".to_string(),
                    health_score: 5,
                },
            ],
            None,
            None,
            None,
        );

        let mut high_hits = 0usize;
        let mut low_hits = 0usize;
        for _ in 0..100 {
            match pool.next().as_deref() {
                Some("http://proxy-high:8080") => high_hits += 1,
                Some("http://proxy-low:8080") => low_hits += 1,
                other => panic!("unexpected proxy selection: {:?}", other),
            }
        }

        assert!(high_hits > low_hits);
        assert!(high_hits >= 90);
    }

    #[test]
    fn test_parse_raw_proxy_line() {
        let raw = "203.0.113.10:1080:test-user:test-pass";
        let pool = ProxyPool::from_raw_list(raw);
        assert_eq!(
            pool.next().unwrap(),
            "socks5h://test-user:test-pass@203.0.113.10:1080"
        );
    }

    #[test]
    fn test_parse_mixed_env_proxy_list() {
        let raw = "socks5h://u:p@1.2.3.4:1080,203.0.113.10:1080:test-user:test-pass";
        let parsed = parse_proxy_tokens(raw);
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0], "socks5h://u:p@1.2.3.4:1080");
        assert_eq!(parsed[1], "socks5h://test-user:test-pass@203.0.113.10:1080");
    }
}
