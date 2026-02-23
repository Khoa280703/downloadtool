//! HTTP client wrapper for stream proxying
//!
//! Provides a high-level interface for fetching video streams with
//! anti-bot protection and range request support.

use crate::anti_bot::{AntiBotClient, AntiBotError};
use crate::cookie_store::Platform;
use bytes::Bytes;
use futures::{Stream, StreamExt};
use reqwest::header::HeaderMap;
use std::time::Duration;
use tracing::{debug, error, info};

/// HTTP range specification for partial content requests.
#[derive(Debug, Clone, Copy)]
pub struct Range {
    /// Start byte position (inclusive)
    pub start: u64,
    /// End byte position (inclusive, optional)
    pub end: Option<u64>,
}

impl Range {
    /// Create a new range from start byte.
    pub fn from(start: u64) -> Self {
        Self { start, end: None }
    }

    /// Create a new range with explicit start and end.
    pub fn new(start: u64, end: u64) -> Self {
        Self {
            start,
            end: Some(end),
        }
    }

    /// Convert to HTTP Range header value.
    pub fn to_header_value(&self) -> String {
        match self.end {
            Some(end) => format!("bytes={}-{}", self.start, end),
            None => format!("bytes={}-", self.start),
        }
    }
}

/// Errors that can occur during proxy operations.
#[derive(Debug, thiserror::Error)]
pub enum ProxyError {
    /// HTTP request failed.
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    /// Anti-bot error.
    #[error("Anti-bot protection failed: {0}")]
    AntiBotFailed(#[from] AntiBotError),

    /// Invalid URL.
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// URL not in allowlist.
    #[error("URL not allowed: {0}")]
    UrlNotAllowed(String),

    /// Stream interrupted.
    #[error("Stream interrupted")]
    StreamInterrupted,
}

/// HTTP client wrapper for proxying video streams.
pub struct ProxyClient {
    anti_bot: AntiBotClient,
}

impl ProxyClient {
    /// Create a new proxy client for the given platform.
    pub fn new(platform: Platform) -> Result<Self, ProxyError> {
        let anti_bot = AntiBotClient::new(platform)?;
        Ok(Self { anti_bot })
    }

    /// Create a new proxy client with warm-up.
    pub async fn with_warmup(platform: Platform) -> Result<Self, ProxyError> {
        let anti_bot = AntiBotClient::new(platform)?;
        anti_bot.warm_up().await?;
        Ok(Self { anti_bot })
    }

    /// Fetch a stream from the given URL with optional range support.
    ///
    /// # Arguments
    /// * `url` - The source URL to fetch
    /// * `range` - Optional byte range for partial content
    ///
    /// # Returns
    /// A stream of bytes from the source.
    pub async fn fetch_stream(
        &self,
        url: &str,
        range: Option<Range>,
    ) -> Result<impl Stream<Item = Result<Bytes, ProxyError>>, ProxyError> {
        info!("Fetching stream from: {}", url);

        let range_header = range.map(|r| r.to_header_value());
        let stream = self.anti_bot.fetch_stream(url, range_header).await?;
        let stream = stream.map(|r| r.map_err(ProxyError::from));

        debug!("Stream initiated for: {}", url);

        Ok(stream)
    }

    /// Fetch a stream with full response metadata.
    ///
    /// Returns the response headers along with the byte stream.
    pub async fn fetch_stream_with_headers(
        &self,
        url: &str,
        range: Option<Range>,
    ) -> Result<(HeaderMap, impl Stream<Item = Result<Bytes, ProxyError>>), ProxyError> {
        info!("Fetching stream with headers from: {}", url);

        let range_header = range.map(|r| r.to_header_value());
        let response = self.anti_bot.get_with_range(url, range_header).await?;

        let status = response.status();
        let headers = response.headers().clone();

        if !status.is_success() && status.as_u16() != 206 {
            error!("Failed to fetch stream: HTTP {}", status);
            return Err(ProxyError::RequestFailed(
                response.error_for_status().unwrap_err(),
            ));
        }

        debug!(
            "Stream response received: HTTP {}, content-length: {:?}",
            status,
            headers.get("content-length")
        );

        let stream = response
            .bytes_stream()
            .map(|result| result.map_err(ProxyError::RequestFailed));

        Ok((headers, stream))
    }

    /// Make a simple GET request.
    pub async fn get(&self, url: &str) -> Result<reqwest::Response, ProxyError> {
        let response = self.anti_bot.get(url).await?;
        Ok(response)
    }

    /// Get a reference to the anti-bot client.
    pub fn anti_bot(&self) -> &AntiBotClient {
        &self.anti_bot
    }

    /// Reset cookies and re-warm.
    pub async fn reset_cookies(&self) -> Result<(), ProxyError> {
        self.anti_bot.reset_cookies().await?;
        Ok(())
    }
}

/// Parse a Range header string into a Range struct.
///
/// Supports format: "bytes=start-end" or "bytes=start-"
pub fn parse_range_header(header: &str) -> Option<Range> {
    let header = header.trim();

    // Check for bytes= prefix
    if !header.to_lowercase().starts_with("bytes=") {
        return None;
    }

    let range_part = &header[6..]; // Skip "bytes="
    let parts: Vec<&str> = range_part.split('-').collect();

    if parts.len() != 2 {
        return None;
    }

    let start: u64 = parts[0].parse().ok()?;
    let end = if parts[1].is_empty() {
        None
    } else {
        parts[1].parse().ok()
    };

    Some(Range { start, end })
}

/// Validate that a URL is from an allowed host.
pub fn validate_stream_url(url: &str) -> Result<reqwest::Url, ProxyError> {
    const ALLOWED_STREAM_HOSTS: &[&str] = &[
        "googlevideo.com",
        "youtube.com",
        "youtu.be",
        "tiktokcdn.com",
        "tiktok.com",
        "vm.tiktok.com",
    ];

    let parsed = reqwest::Url::parse(url)
        .map_err(|e| ProxyError::InvalidUrl(e.to_string()))?;

    let host = parsed
        .host_str()
        .ok_or_else(|| ProxyError::InvalidUrl("Missing host".to_string()))?;

    let is_allowed = ALLOWED_STREAM_HOSTS.iter().any(|allowed| {
        host == *allowed || host.ends_with(&format!(".{}", allowed))
    });

    if !is_allowed {
        return Err(ProxyError::UrlNotAllowed(host.to_string()));
    }

    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_to_header_value() {
        let range = Range::new(0, 1023);
        assert_eq!(range.to_header_value(), "bytes=0-1023");

        let range = Range::from(1024);
        assert_eq!(range.to_header_value(), "bytes=1024-");
    }

    #[test]
    fn test_parse_range_header() {
        let range = parse_range_header("bytes=0-1023").unwrap();
        assert_eq!(range.start, 0);
        assert_eq!(range.end, Some(1023));

        let range = parse_range_header("bytes=1024-").unwrap();
        assert_eq!(range.start, 1024);
        assert_eq!(range.end, None);

        assert!(parse_range_header("invalid").is_none());
        assert!(parse_range_header("bytes=abc-def").is_none());
    }

    #[test]
    fn test_validate_stream_url() {
        // Valid URLs
        assert!(validate_stream_url("https://rr1---sn-abc.googlevideo.com/videoplayback").is_ok());
        assert!(validate_stream_url("https://youtube.com/watch?v=abc").is_ok());
        assert!(validate_stream_url("https://youtu.be/abc123").is_ok());
        assert!(validate_stream_url("https://v16-webapp.tiktokcdn.com/video").is_ok());
        assert!(validate_stream_url("https://tiktok.com/@user/video/123").is_ok());

        // Invalid URLs
        assert!(validate_stream_url("https://example.com/video").is_err());
        assert!(validate_stream_url("https://malicious.com").is_err());
        assert!(validate_stream_url("not-a-url").is_err());
    }

    #[test]
    fn test_proxy_client_creation() {
        let client = ProxyClient::new(Platform::YouTube);
        assert!(client.is_ok());
    }
}
