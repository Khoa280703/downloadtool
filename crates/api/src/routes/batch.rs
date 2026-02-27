//! Batch extraction handler with SSE streaming
//!
//! GET /api/batch - Server-Sent Events endpoint for batch video extraction

use axum::extract::Query;
use axum::http::{header, HeaderValue};
use axum::response::{sse::Event, IntoResponse, Response, Sse};
use futures::stream::{self, BoxStream, Stream};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::time::Duration;
use tracing::{info, warn};

const NO_STORE_CACHE_CONTROL: &str = "no-store, no-cache, must-revalidate";

/// Query parameters for batch extraction.
#[derive(Debug, Deserialize)]
pub struct BatchParams {
    /// URL of the channel/playlist to extract
    pub url: String,
}

/// SSE event types for batch extraction.
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum BatchEvent {
    /// A video link extracted from the channel/playlist
    #[serde(rename = "link")]
    Link {
        /// Video URL
        url: String,
        /// Video title
        title: String,
        /// Index in the batch
        index: usize,
        /// Total expected videos
        total: Option<usize>,
    },
    /// Batch extraction completed
    #[serde(rename = "done")]
    Done {
        /// Total videos extracted
        total: usize,
    },
    /// Error during extraction
    #[serde(rename = "error")]
    Error {
        /// Error message
        message: String,
    },
    /// Progress update
    #[serde(rename = "progress")]
    Progress {
        /// Current progress
        current: usize,
        /// Total expected
        total: Option<usize>,
    },
}

/// Batch extraction SSE endpoint.
///
/// GET /api/batch?url=<channel_url>
/// Response: text/event-stream
///
/// Events:
/// - data: {"type":"link","url":"...","title":"...","index":1,"total":50}
/// - data: {"type":"done","total":50}
/// - data: {"type":"error","message":"..."}
/// - data: {"type":"progress","current":10,"total":50}
pub async fn batch_handler(
    Query(params): Query<BatchParams>,
) -> Response {
    info!("Batch extraction request for URL: {}", params.url);

    let stream: BoxStream<'static, Result<Event, Infallible>> =
        if !is_valid_batch_url(&params.url) {
            warn!("Invalid batch URL: {}", params.url);
            let error_event = BatchEvent::Error {
                message: "Invalid URL. Only YouTube channels/playlists are supported.".to_string(),
            };
            Box::pin(stream::once(async move {
                Ok(Event::default().data(serde_json::to_string(&error_event).unwrap()))
            }))
        } else {
            Box::pin(create_batch_stream(&params.url))
        };

    let mut response = Sse::new(stream)
        .keep_alive(
            axum::response::sse::KeepAlive::new()
                .interval(Duration::from_secs(15)),
        )
        .into_response();
    response.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static(NO_STORE_CACHE_CONTROL),
    );
    response
}

/// Create a stream of SSE events for batch extraction.
///
/// This is a placeholder implementation that simulates batch extraction.
/// In the real implementation, this would:
/// 1. Call extractor::extract_channel(url) to get an async stream
/// 2. Map each video to a BatchEvent::Link
/// 3. Send BatchEvent::Progress updates periodically
/// 4. Send BatchEvent::Done when complete
fn create_batch_stream(
    url: &str,
) -> impl Stream<Item = Result<Event, Infallible>> {
    let url = url.to_string();

    async_stream::stream! {
        info!("Starting batch extraction for: {}", url);

        // Simulate extracting videos from a channel/playlist
        // In real implementation, this would call the extractor

        let total = 10; // Simulated total
        let mut extracted = 0;

        // Send initial progress
        yield Ok(Event::default().data(
            serde_json::to_string(&BatchEvent::Progress {
                current: 0,
                total: Some(total),
            }).unwrap()
        ));

        // Simulate extracting videos
        for i in 1..=total {
            tokio::time::sleep(Duration::from_millis(100)).await;

            // Simulate a video link
            let event = BatchEvent::Link {
                url: format!("https://example.com/video/{}", i),
                title: format!("Video {}", i),
                index: i,
                total: Some(total),
            };

            yield Ok(Event::default().data(
                serde_json::to_string(&event).unwrap()
            ));

            extracted += 1;

            // Send progress update every 3 videos
            if i % 3 == 0 {
                yield Ok(Event::default().data(
                    serde_json::to_string(&BatchEvent::Progress {
                        current: extracted,
                        total: Some(total),
                    }).unwrap()
                ));
            }
        }

        // Send completion event
        yield Ok(Event::default().data(
            serde_json::to_string(&BatchEvent::Done { total: extracted }).unwrap()
        ));

        info!("Batch extraction completed: {} videos", extracted);
    }
}

/// Check if URL is a valid YouTube batch extraction URL (channel/playlist).
fn is_valid_batch_url(url: &str) -> bool {
    let url_lower = url.to_lowercase();
    url_lower.contains("youtube.com/channel/")
        || url_lower.contains("youtube.com/c/")
        || url_lower.contains("youtube.com/user/")
        || url_lower.contains("youtube.com/playlist")
        || url_lower.contains("youtube.com/@")
        || (url_lower.contains("youtube.com") && url_lower.contains("list="))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_batch_url() {
        // YouTube channels
        assert!(is_valid_batch_url("https://youtube.com/channel/UCxxx"));
        assert!(is_valid_batch_url("https://youtube.com/c/ChannelName"));
        assert!(is_valid_batch_url("https://youtube.com/user/Username"));
        assert!(is_valid_batch_url("https://youtube.com/playlist?list=xxx"));
        assert!(is_valid_batch_url("https://youtube.com/@ChannelName"));

        // Invalid URLs
        assert!(!is_valid_batch_url("https://youtube.com/watch?v=xxx"));
        assert!(!is_valid_batch_url("https://example.com/playlist"));
    }

    #[test]
    fn test_batch_event_serialization() {
        let event = BatchEvent::Link {
            url: "https://example.com/video".to_string(),
            title: "Test Video".to_string(),
            index: 1,
            total: Some(10),
        };

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"type\":\"link\""));
        assert!(json.contains("\"title\":\"Test Video\""));

        let done_event = BatchEvent::Done { total: 10 };
        let json = serde_json::to_string(&done_event).unwrap();
        assert!(json.contains("\"type\":\"done\""));
    }
}
