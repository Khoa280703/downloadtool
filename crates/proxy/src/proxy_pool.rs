//! Proxy pool with round-robin rotation and cooldown-based failover.
//!
//! Supports both:
//! - Full proxy URLs (`socks5h://user:pass@host:port`, `http://...`)
//! - Raw records (`host:port:user:pass`) commonly used by proxy providers.

use crate::proxy_quarantine::{
    append_quarantine_record, load_quarantined_proxies, proxy_quarantine_file_from_env,
};
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tracing::{debug, warn};

/// Maximum consecutive failures before marking proxy as unhealthy.
const MAX_FAILURES: usize = 3;
/// Default cooldown period for failed proxies.
const FAILURE_COOLDOWN: Duration = Duration::from_secs(1800);

/// Entry in the proxy pool with health tracking.
struct ProxyEntry {
    url: String,
    quarantined: AtomicBool,
    failed_count: AtomicUsize,
    last_failed: RwLock<Option<Instant>>,
}

impl ProxyEntry {
    fn new(url: String, quarantined: bool) -> Self {
        Self {
            url,
            quarantined: AtomicBool::new(quarantined),
            failed_count: AtomicUsize::new(0),
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
}

/// Pool of proxies with round-robin selection.
pub struct ProxyPool {
    proxies: Vec<Arc<ProxyEntry>>,
    current: AtomicUsize,
    quarantine_file: Option<PathBuf>,
}

impl ProxyPool {
    /// Create a new proxy pool from a list of fully qualified proxy URLs.
    pub fn new(urls: Vec<String>) -> Self {
        Self::new_with_quarantine(urls, None, HashSet::new())
    }

    fn new_with_quarantine(
        urls: Vec<String>,
        quarantine_file: Option<PathBuf>,
        quarantined_urls: HashSet<String>,
    ) -> Self {
        let proxies = urls
            .into_iter()
            .filter(|url| !url.trim().is_empty())
            .map(|url| {
                let is_quarantined = quarantined_urls.contains(&url);
                Arc::new(ProxyEntry::new(url, is_quarantined))
            })
            .collect();

        Self {
            proxies,
            current: AtomicUsize::new(0),
            quarantine_file,
        }
    }

    /// Create a pool from raw proxy records (`host:port:user:pass`), one per line.
    pub fn from_raw_list(raw_list: &str) -> Self {
        let urls = parse_proxy_tokens(raw_list);
        Self::new(urls)
    }

    /// Create a proxy pool from environment variable `PROXY_LIST`.
    ///
    /// Accepts comma/newline separated values in either of these formats:
    /// - `socks5h://user:pass@host:port`
    /// - `http://user:pass@host:port`
    /// - `host:port:user:pass` (auto-converted to `socks5h://...`)
    pub fn from_env() -> Option<Self> {
        let raw = std::env::var("PROXY_LIST").ok()?;
        let urls = parse_proxy_tokens(&raw);
        if urls.is_empty() {
            return None;
        }

        let quarantine_file = proxy_quarantine_file_from_env();
        let quarantined_urls = quarantine_file
            .as_ref()
            .map(|path| load_quarantined_proxies(path))
            .unwrap_or_default();
        let quarantined_count = urls
            .iter()
            .filter(|url| quarantined_urls.contains(*url))
            .count();

        debug!(
            "Loaded {} proxies from PROXY_LIST ({} quarantined)",
            urls.len(),
            quarantined_count
        );

        Some(Self::new_with_quarantine(
            urls,
            quarantine_file,
            quarantined_urls,
        ))
    }

    /// Get next healthy proxy by round-robin.
    pub fn next(&self) -> Option<&str> {
        if self.proxies.is_empty() {
            return None;
        }

        let start_idx = self.current.fetch_add(1, Ordering::Relaxed);
        let proxy_count = self.proxies.len();

        for i in 0..proxy_count {
            let idx = (start_idx + i) % proxy_count;
            let entry = &self.proxies[idx];
            if entry.is_healthy() {
                return Some(&entry.url);
            }
        }

        for i in 0..proxy_count {
            let idx = (start_idx + i) % proxy_count;
            let entry = &self.proxies[idx];
            if !entry.is_quarantined() {
                warn!("No healthy proxies available, falling back to non-quarantined proxy");
                return Some(&entry.url);
            }
        }

        warn!("No usable proxies available (all proxies quarantined)");
        None
    }

    /// Owned version of [`Self::next`], useful for async call sites.
    pub fn next_owned(&self) -> Option<String> {
        self.next().map(ToString::to_string)
    }

    /// Mark a specific proxy as failed.
    pub fn mark_failed(&self, proxy_url: &str) {
        for entry in &self.proxies {
            if entry.url == proxy_url {
                entry.mark_failed();
                return;
            }
        }
    }

    /// Mark a specific proxy as healthy again.
    pub fn mark_success(&self, proxy_url: &str) {
        for entry in &self.proxies {
            if entry.url == proxy_url {
                entry.mark_success();
                return;
            }
        }
    }

    /// Quarantine a proxy and persist it to a separate file for operator replacement.
    pub fn quarantine(&self, proxy_url: &str, reason: &str) {
        for entry in &self.proxies {
            if entry.url == proxy_url {
                if entry.quarantine() {
                    if let Some(path) = self.quarantine_file.as_ref() {
                        append_quarantine_record(path, proxy_url, reason);
                    }
                    warn!(
                        "Proxy {} quarantined and removed from rotation. reason={}",
                        proxy_url, reason
                    );
                }
                return;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.proxies.len()
    }

    pub fn is_empty(&self) -> bool {
        self.proxies.is_empty()
    }
}

impl Default for ProxyPool {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

fn parse_proxy_tokens(raw: &str) -> Vec<String> {
    raw.lines()
        .flat_map(|line| line.split(','))
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .filter_map(normalize_proxy_token)
        .collect()
}

fn normalize_proxy_token(token: &str) -> Option<String> {
    if token.contains("://") {
        return Some(token.to_string());
    }

    // Parse raw provider format: host:port:user:pass
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
    use super::*;
    use std::fs;

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

        let pool = ProxyPool::new_with_quarantine(
            vec!["http://proxy1:8080".to_string()],
            Some(path.clone()),
            HashSet::new(),
        );
        pool.quarantine("http://proxy1:8080", "unit-test");

        let content = fs::read_to_string(&path).expect("quarantine file should exist");
        assert!(content.contains("http://proxy1:8080"));
        assert!(content.contains("unit-test"));

        let _ = fs::remove_file(path);
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
