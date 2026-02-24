//! Video stream proxy functionality
//!
//! Provides zero-copy streaming from source to client with
//! support for range requests and header forwarding.

use bytes::Bytes;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;
use tracing::{debug, error, info};

use crate::anti_bot::AntiBotGuard;
use crate::client::{ProxyError, Range};

/// Type alias for byte stream.
pub type ByteStream = Pin<Box<dyn Stream<Item = Result<Bytes, ProxyError>> + Send>>;

/// Stream proxy for relaying video content with zero-copy streaming.
pub struct StreamProxy {
    /// HTTP client for making requests
    client: Client,
    /// Anti-bot protection
    anti_bot: AntiBotGuard,
}

impl StreamProxy {
    /// Create a new stream proxy.
    pub fn new(client: Client) -> Self {
        Self {
            client,
            anti_bot: AntiBotGuard::new(),
        }
    }

    /// Proxy a video stream from the given URL.
    ///
    /// # Arguments
    /// * `url` - The source URL to proxy
    /// * `referer` - Optional referer header
    ///
    /// # Returns
    /// A stream of bytes from the proxied content.
    pub async fn proxy_stream(
        &mut self,
        url: &str,
        referer: Option<&str>,
    ) -> Result<ByteStream, ProxyError> {
        info!("Proxying stream from: {}", url);

        let headers = self.anti_bot.generate_headers(referer);

        let response = self.client.get(url).headers(headers).send().await?;

        if !response.status().is_success() {
            error!("Failed to fetch stream: HTTP {}", response.status());
            return Err(ProxyError::RequestFailed(
                response.error_for_status().unwrap_err(),
            ));
        }

        debug!(
            "Stream response received, content-length: {:?}",
            response.content_length()
        );

        let stream = response
            .bytes_stream()
            .map(|result| result.map_err(ProxyError::RequestFailed));

        Ok(Box::pin(stream))
    }

    /// Proxy a video stream with range request support.
    ///
    /// # Arguments
    /// * `url` - The source URL to proxy
    /// * `range` - Optional byte range for partial content
    /// * `referer` - Optional referer header
    ///
    /// # Returns
    /// A tuple of (status_code, headers, byte_stream) for the proxied content.
    pub async fn proxy_stream_with_range(
        &mut self,
        url: &str,
        range: Option<Range>,
        referer: Option<&str>,
    ) -> Result<(reqwest::StatusCode, reqwest::header::HeaderMap, ByteStream), ProxyError> {
        info!("Proxying stream with range from: {}", url);

        let mut request = self.client.get(url);

        // Add anti-bot headers
        let headers = self.anti_bot.generate_headers(referer);
        request = request.headers(headers);

        // Add range header if specified
        if let Some(ref r) = range {
            debug!("Adding Range header: {}", r.to_header_value());
            request = request.header("Range", r.to_header_value());
        }

        let response = request.send().await?;
        let status = response.status();

        if !status.is_success() && status.as_u16() != 206 {
            error!("Failed to fetch stream: HTTP {}", status);
            return Err(ProxyError::RequestFailed(
                response.error_for_status().unwrap_err(),
            ));
        }

        let response_headers = response.headers().clone();

        debug!(
            "Stream response received: HTTP {}, content-length: {:?}",
            status,
            response.content_length()
        );

        let stream = response
            .bytes_stream()
            .map(|result| result.map_err(ProxyError::RequestFailed));

        Ok((status, response_headers, Box::pin(stream)))
    }

    /// Get a reference to the HTTP client.
    pub fn client(&self) -> &Client {
        &self.client
    }
}

impl Default for StreamProxy {
    fn default() -> Self {
        Self::new(Client::new())
    }
}

/// Forward relevant headers from source response to client response.
///
/// Headers forwarded:
/// - Content-Type
/// - Content-Length
/// - Content-Range
/// - Accept-Ranges
/// - Last-Modified
/// - ETag
pub fn forward_stream_headers(
    source_headers: &reqwest::header::HeaderMap,
) -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();

    let headers_to_forward = [
        "content-type",
        "content-length",
        "content-range",
        "accept-ranges",
        "last-modified",
        "etag",
    ];

    for header_name in &headers_to_forward {
        if let Some(value) = source_headers.get(*header_name) {
            if let Ok(name) = reqwest::header::HeaderName::from_bytes(header_name.as_bytes()) {
                headers.insert(name, value.clone());
            }
        }
    }

    headers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_proxy_creation() {
        let client = Client::new();
        let proxy = StreamProxy::new(client);
        // Verify proxy wraps the client correctly
        let _ = proxy.client();
    }

    #[test]
    fn test_forward_stream_headers() {
        let mut source_headers = reqwest::header::HeaderMap::new();
        source_headers.insert("content-type", "video/mp4".parse().unwrap());
        source_headers.insert("content-length", "12345".parse().unwrap());
        source_headers.insert("accept-ranges", "bytes".parse().unwrap());
        source_headers.insert("x-custom-header", "should-not-forward".parse().unwrap());

        let forwarded = forward_stream_headers(&source_headers);

        assert_eq!(
            forwarded.get("content-type").unwrap(),
            "video/mp4"
        );
        assert_eq!(
            forwarded.get("content-length").unwrap(),
            "12345"
        );
        assert_eq!(
            forwarded.get("accept-ranges").unwrap(),
            "bytes"
        );
        assert!(forwarded.get("x-custom-header").is_none());
    }
}
