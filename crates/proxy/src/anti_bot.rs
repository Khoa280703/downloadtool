//! Anti-bot detection evasion with full client implementation
//!
//! Provides proxy rotation, header rotation,
//! and request throttling to avoid bot detection.

use crate::header_builder::HeaderBuilder;
use crate::platform::Platform;
use crate::proxy_pool::ProxyPool;
use crate::throttle::DomainThrottle;
use bytes::Bytes;
use futures::Stream;
use futures::StreamExt;
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
    platform: Platform,
    proxy_pool: Arc<ProxyPool>,
    active_proxy: Option<String>,
    header_builder: HeaderBuilder,
    throttle: Arc<DomainThrottle>,
    client: Client,
}

impl AntiBotClient {
    fn build_for_platform(
        platform: Platform,
        proxy_pool: Arc<ProxyPool>,
        forced_proxy: Option<String>,
    ) -> Result<Self, AntiBotError> {
        let header_builder = HeaderBuilder::new();
        let throttle = Arc::new(DomainThrottle::new());
        let active_proxy = forced_proxy
            .or_else(|| proxy_pool.next_owned())
            .ok_or(AntiBotError::NoHealthyProxies)?;
        let client = Self::build_client(Some(active_proxy.as_str()))?;

        Ok(Self {
            platform,
            proxy_pool,
            active_proxy: Some(active_proxy),
            header_builder,
            throttle,
            client,
        })
    }

    /// Create a new anti-bot client for the given platform
    pub fn new(platform: Platform) -> Result<Self, AntiBotError> {
        let proxy_pool = ProxyPool::global_or_env().unwrap_or_default();
        Self::build_for_platform(platform, proxy_pool, None)
    }

    /// Create a new anti-bot client pinned to a specific proxy URL.
    ///
    /// If `forced_proxy` is `None`, falls back to normal pool selection.
    pub fn new_with_proxy(
        platform: Platform,
        forced_proxy: Option<String>,
    ) -> Result<Self, AntiBotError> {
        let proxy_pool = ProxyPool::global_or_env().unwrap_or_default();
        Self::build_for_platform(platform, proxy_pool, forced_proxy)
    }

    /// Create a new anti-bot client with custom proxy pool
    pub fn with_proxy_pool(
        platform: Platform,
        proxy_pool: Arc<ProxyPool>,
    ) -> Result<Self, AntiBotError> {
        Self::build_for_platform(platform, proxy_pool, None)
    }

    /// Build the HTTP client with optional proxy.
    fn build_client(proxy_url: Option<&str>) -> Result<Client, AntiBotError> {
        let mut builder = Client::builder()
            .connect_timeout(Duration::from_secs(30))
            .pool_max_idle_per_host(10)
            .connection_verbose(false);

        // Add proxy if available
        if let Some(proxy_url) = proxy_url {
            debug!("Configuring client with proxy: {}", proxy_url);
            let proxy =
                Proxy::all(proxy_url).map_err(|e| AntiBotError::InvalidUrl(e.to_string()))?;
            builder = builder.proxy(proxy);
        }

        builder.build().map_err(AntiBotError::RequestFailed)
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
        let status = response.status();
        if !status.is_success() {
            return Err(AntiBotError::RequestFailed(
                response.error_for_status().unwrap_err(),
            ));
        }

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
        let parsed_url =
            reqwest::Url::parse(url).map_err(|e| AntiBotError::InvalidUrl(e.to_string()))?;
        let domain = parsed_url
            .host_str()
            .ok_or_else(|| AntiBotError::InvalidUrl("Missing host".to_string()))?;

        let mut last_error = None;

        for attempt in 0..MAX_RETRIES {
            // Wait for throttle
            self.throttle.wait(domain).await;

            // Build request
            let mut request = self.client.get(url);

            // Add headers — detects c=IOS URLs and uses iOS User-Agent automatically
            let headers = self
                .header_builder
                .build_headers_for_url(url, self.platform, None);
            request = request.headers(headers);

            // Add range if specified
            if let Some(ref r) = range {
                request = request.header("Range", r.clone());
            }

            // Track current proxy for failure handling
            let current_proxy = self.active_proxy.clone();

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
                        // YouTube CDN stream URLs (googlevideo.com) are IP-bound at extraction time.
                        // Rotating proxies on 403 is counterproductive — the URL will always 403
                        // from a different IP. Fail fast to surface the error without delay.
                        let is_cdn_url = domain.contains("googlevideo.com");

                        warn!(
                            "Received {} for {}{}",
                            status,
                            url,
                            if is_cdn_url {
                                " (CDN URL — not retrying)"
                            } else {
                                ", rotating proxy and retrying"
                            }
                        );

                        if is_cdn_url {
                            return Err(AntiBotError::RequestFailed(
                                response.error_for_status().unwrap_err(),
                            ));
                        }

                        // Mark proxy as failed (non-CDN only)
                        if let Some(ref proxy) = current_proxy {
                            self.proxy_pool.mark_failed(proxy);
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
        Err(last_error.unwrap_or_else(|| AntiBotError::MaxRetriesExceeded(url.to_string())))
    }

    /// Get the platform associated with this client
    pub fn platform(&self) -> Platform {
        self.platform
    }

    /// Get the currently pinned proxy URL for this client instance.
    pub fn active_proxy(&self) -> Option<&str> {
        self.active_proxy.as_deref()
    }

    /// Get a reference to the proxy pool
    pub fn proxy_pool(&self) -> &ProxyPool {
        &self.proxy_pool
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
    fn test_antibot_client_requires_proxy() {
        let client = AntiBotClient::new(Platform::YouTube);
        assert!(matches!(client, Err(AntiBotError::NoHealthyProxies)));
    }

    #[test]
    fn test_antibot_client_creation_with_forced_proxy() {
        let client = AntiBotClient::new_with_proxy(
            Platform::YouTube,
            Some("http://proxy.example:8080".to_string()),
        );
        assert!(client.is_ok());
    }

    #[test]
    fn test_antibot_guard() {
        let mut guard = AntiBotGuard::new();
        let headers = guard.generate_headers(Some("https://example.com"));
        assert!(headers.contains_key("user-agent"));
    }
}
