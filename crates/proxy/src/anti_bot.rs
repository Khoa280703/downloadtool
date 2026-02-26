//! Anti-bot detection evasion with full client implementation
//!
//! Provides proxy rotation, cookie persistence, header rotation,
//! and request throttling to avoid bot detection.

use crate::cookie_store::{CookieStore, Platform};
use futures::StreamExt;
use crate::header_builder::HeaderBuilder;
use crate::proxy_pool::ProxyPool;
use crate::throttle::DomainThrottle;
use bytes::Bytes;
use futures::Stream;
use reqwest::{Client, Proxy, StatusCode};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, error, warn};

/// Maximum number of retries on 403/429 errors
const MAX_RETRIES: u32 = 3;
/// Delay between retry attempts (200ms)
const RETRY_DELAY: Duration = Duration::from_millis(200);

/// Errors that can occur during anti-bot operations
#[derive(Debug, thiserror::Error)]
pub enum AntiBotError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    /// No healthy proxies available
    #[error("No healthy proxies available")]
    NoHealthyProxies,

    /// Max retries exceeded
    #[error("Max retries exceeded for URL: {0}")]
    MaxRetriesExceeded(String),

    /// Invalid URL
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
}

/// Full anti-bot client with all protection layers
pub struct AntiBotClient {
    proxy_pool: Arc<ProxyPool>,
    cookie_store: Arc<CookieStore>,
    header_builder: HeaderBuilder,
    throttle: Arc<DomainThrottle>,
    client: Client,
}

impl AntiBotClient {
    /// Create a new anti-bot client for the given platform
    pub fn new(platform: Platform) -> Result<Self, AntiBotError> {
        let proxy_pool = Arc::new(ProxyPool::from_env().unwrap_or_default());
        let cookie_store = Arc::new(CookieStore::new(platform));
        let header_builder = HeaderBuilder::new();
        let throttle = Arc::new(DomainThrottle::new());

        // Build client with cookie store
        let client = Self::build_client(&proxy_pool, &cookie_store)?;

        Ok(Self {
            proxy_pool,
            cookie_store,
            header_builder,
            throttle,
            client,
        })
    }

    /// Create a new anti-bot client with custom proxy pool
    pub fn with_proxy_pool(
        platform: Platform,
        proxy_pool: Arc<ProxyPool>,
    ) -> Result<Self, AntiBotError> {
        let cookie_store = Arc::new(CookieStore::new(platform));
        let header_builder = HeaderBuilder::new();
        let throttle = Arc::new(DomainThrottle::new());

        let client = Self::build_client(&proxy_pool, &cookie_store)?;

        Ok(Self {
            proxy_pool,
            cookie_store,
            header_builder,
            throttle,
            client,
        })
    }

    /// Build the HTTP client with cookie jar and optional proxy
    fn build_client(
        proxy_pool: &ProxyPool,
        _cookie_store: &CookieStore,
    ) -> Result<Client, AntiBotError> {
        let mut builder = Client::builder()
            .connect_timeout(Duration::from_secs(30))
            .pool_max_idle_per_host(10)
            .cookie_store(true)  // Use built-in cookie store
            .connection_verbose(false);

        // Add proxy if available
        if let Some(proxy_url) = proxy_pool.next() {
            debug!("Configuring client with proxy: {}", proxy_url);
            let proxy = Proxy::all(proxy_url)
                .map_err(|e| AntiBotError::InvalidUrl(e.to_string()))?;
            builder = builder.proxy(proxy);
        }

        builder.build().map_err(AntiBotError::RequestFailed)
    }

    /// Warm up cookies by fetching platform homepage
    pub async fn warm_up(&self) -> Result<(), AntiBotError> {
        self.cookie_store.warm_up(&self.client).await?;
        Ok(())
    }

    /// Make a GET request with full anti-bot protection
    pub async fn get(&self, url: &str) -> Result<reqwest::Response, AntiBotError> {
        self.request_with_retry(url, None).await
    }

    /// Make a GET request with specific range
    pub async fn get_with_range(
        &self,
        url: &str,
        range: Option<String>,
    ) -> Result<reqwest::Response, AntiBotError> {
        self.request_with_retry(url, range).await
    }

    /// Fetch a stream with anti-bot protection
    pub async fn fetch_stream(
        &self,
        url: &str,
        range: Option<String>,
    ) -> Result<impl Stream<Item = Result<Bytes, AntiBotError>>, AntiBotError> {
        let response = self.request_with_retry(url, range).await?;

        let stream = response
            .bytes_stream()
            .map(|result| result.map_err(AntiBotError::RequestFailed));

        Ok(stream)
    }

    /// Internal method to make requests with retry logic
    async fn request_with_retry(
        &self,
        url: &str,
        range: Option<String>,
    ) -> Result<reqwest::Response, AntiBotError> {
        let parsed_url = reqwest::Url::parse(url)
            .map_err(|e| AntiBotError::InvalidUrl(e.to_string()))?;
        let domain = parsed_url
            .host_str()
            .ok_or_else(|| AntiBotError::InvalidUrl("Missing host".to_string()))?;

        let mut last_error = None;

        for attempt in 0..MAX_RETRIES {
            // Wait for throttle
            self.throttle.wait(domain).await;

            // Build request
            let mut request = self.client.get(url);

            // Add headers
            let headers = self.header_builder.build_headers(self.cookie_store.platform(), None);
            request = request.headers(headers);

            // Add range if specified
            if let Some(ref r) = range {
                request = request.header("Range", r.clone());
            }

            // Track current proxy for failure handling
            let current_proxy = self.proxy_pool.next().map(|s| s.to_string());

            debug!(
                "Making request to {} (attempt {}/{})",
                url,
                attempt + 1,
                MAX_RETRIES
            );

            match request.send().await {
                Ok(response) => {
                    let status = response.status();

                    // Check for bot detection responses
                    if status == StatusCode::FORBIDDEN || status == StatusCode::TOO_MANY_REQUESTS {
                        warn!(
                            "Received {} for {}, rotating proxy and retrying",
                            status,
                            url
                        );

                        // Mark proxy as failed
                        if let Some(ref proxy) = current_proxy {
                            self.proxy_pool.mark_failed(proxy);
                        }

                        // Clear cookies on 403 (might be session-related)
                        if status == StatusCode::FORBIDDEN {
                            self.cookie_store.clear();
                        }

                        last_error = Some(AntiBotError::RequestFailed(
                            response.error_for_status().unwrap_err(),
                        ));

                        // Wait before retry
                        sleep(RETRY_DELAY).await;
                        continue;
                    }

                    // Success - mark proxy as healthy
                    if let Some(ref proxy) = current_proxy {
                        self.proxy_pool.mark_success(proxy);
                    }

                    return Ok(response);
                }
                Err(e) => {
                    warn!("Request failed for {}: {}", url, e);

                    // Mark proxy as failed
                    if let Some(ref proxy) = current_proxy {
                        self.proxy_pool.mark_failed(proxy);
                    }

                    last_error = Some(AntiBotError::RequestFailed(e));

                    // Wait before retry
                    sleep(RETRY_DELAY).await;
                }
            }
        }

        // Max retries exceeded
        error!("Max retries exceeded for URL: {}", url);
        Err(last_error.unwrap_or_else(|| {
            AntiBotError::MaxRetriesExceeded(url.to_string())
        }))
    }

    /// Get the platform associated with this client
    pub fn platform(&self) -> Platform {
        self.cookie_store.platform()
    }

    /// Get a reference to the proxy pool
    pub fn proxy_pool(&self) -> &ProxyPool {
        &self.proxy_pool
    }

    /// Get a reference to the cookie store
    pub fn cookie_store(&self) -> &CookieStore {
        &self.cookie_store
    }

    /// Clear cookies and re-warm
    pub async fn reset_cookies(&self) -> Result<(), AntiBotError> {
        self.cookie_store.clear();
        self.warm_up().await
    }
}

/// Legacy anti-bot guard for backward compatibility
pub struct AntiBotGuard {
    header_builder: HeaderBuilder,
}

impl AntiBotGuard {
    /// Create a new anti-bot guard
    pub fn new() -> Self {
        Self {
            header_builder: HeaderBuilder::new(),
        }
    }

    /// Generate headers (for backward compatibility)
    pub fn generate_headers(&mut self, referer: Option<&str>) -> reqwest::header::HeaderMap {
        self.header_builder.build_generic_headers(referer)
    }
}

impl Default for AntiBotGuard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_antibot_client_creation() {
        let client = AntiBotClient::new(Platform::YouTube);
        assert!(client.is_ok());
    }

    #[test]
    fn test_antibot_guard() {
        let mut guard = AntiBotGuard::new();
        let headers = guard.generate_headers(Some("https://example.com"));
        assert!(headers.contains_key("user-agent"));
    }
}
