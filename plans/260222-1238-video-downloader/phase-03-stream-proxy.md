# Phase 03 — Stream Proxy Core

## Context
- Plan: [plan.md](plan.md)
- Prev: [phase-02-extraction-layer.md](phase-02-extraction-layer.md)
- Next: [phase-04-antibot-layer.md](phase-04-antibot-layer.md), [phase-05-cpu-muxer.md](phase-05-cpu-muxer.md)

## Overview
- **Priority**: P0
- **Status**: completed
- **Effort**: 1.5d
- Zero-storage stream proxy: receive extracted URL → fetch from source → pipe chunked bytes to browser. No temp files, no disk writes.

## Key Insights
- axum `Body::from_stream()` handles chunked transfer automatically
- `reqwest` response body implements `Stream<Item=Bytes>` → direct pipe
- Content-Disposition header triggers browser download dialog
- Range requests (HTTP 206) needed for resume support
- Response headers from source (Content-Length, Content-Type) must be forwarded

## Architecture

<!-- Updated: Validation Session 1 - Added SSE batch endpoint -->

```
POST /api/extract       →  ExtractorPool  →  { streams, title }
GET  /api/stream?...    →  ProxyClient → Source CDN → axum chunked stream → browser
GET  /api/batch         →  SSE endpoint
         ├── accept channel/playlist URL
         ├── ExtractorPool.extract_channel(url) → stream of video links
         └── SSE events: { type: "link", url, title, index, total }
             browser receives all links → starts download pool (3 concurrent)
```

## Related Code Files
- `crates/api/src/main.rs` — axum router setup
- `crates/api/src/routes/extract.rs` — POST /api/extract handler
- `crates/api/src/routes/stream.rs` — GET /api/stream handler
- `crates/proxy/src/lib.rs` — ProxyClient wrapper
- `crates/proxy/src/client.rs` — reqwest client with headers/proxy

## Implementation Steps

1. **axum router** (`crates/api/src/main.rs`)
   ```rust
   let app = Router::new()
       .route("/api/extract", post(extract_handler))
       .route("/api/stream", get(stream_handler))
       .layer(CorsLayer::permissive())
       .layer(TraceLayer::new_for_http());
   ```

2. **Extract handler** (`routes/extract.rs`)
   ```rust
   // POST /api/extract  body: { url: string, options: { quality, format, watermark } }
   // Returns: { streams: [...], title, selected_stream_url }
   ```
   - Validate URL (youtube.com / tiktok.com only)
   - Call `extractor::extract(url, cookies)`
   - Return stream list + recommended stream

3. **Stream handler** (`routes/stream.rs`)
   ```rust
   // GET /api/stream?url=<encoded>&title=<encoded>&format=mp4
   // Pipes source CDN bytes → browser
   ```
   - Validate `url` param against allowlist (googlevideo.com, tiktokcdn.com)
   - Parse Range header for resume support (HTTP 206)
   - Build `ProxyClient::get(url).range(range).send()`
   - Forward: Content-Length, Content-Type, Accept-Ranges
   - Set: `Content-Disposition: attachment; filename="{title}.mp4"`
   - Return `axum::body::Body::from_stream(response.bytes_stream())`

4. **ProxyClient** (`crates/proxy/src/client.rs`)
   ```rust
   pub struct ProxyClient { inner: reqwest::Client }
   impl ProxyClient {
       pub fn new(proxy_url: Option<&str>) -> Self
       pub async fn get_stream(&self, url: &str, range: Option<RangeHeader>)
           -> Result<impl Stream<Item=Result<Bytes>>>
   }
   ```
   - Built with `reqwest::ClientBuilder`
   - Default headers: realistic browser UA, Accept, Accept-Language
   - Optional proxy: `Proxy::all(proxy_url)`
   - `timeout(Duration::from_secs(30))`
   - `connection_verbose(false)` — no debug noise

5. **Range request forwarding**
   - Parse `Range: bytes=X-Y` from incoming request
   - Forward as `Range` header to source
   - Return HTTP 206 with `Content-Range` if source supports it

6. **Security: URL allowlist validation**
   ```rust
   const ALLOWED_HOSTS: &[&str] = &[
       "googlevideo.com", "youtube.com",
       "tiktokcdn.com", "tiktok.com",
   ];
   fn validate_stream_url(url: &str) -> Result<Url>
   ```

## SSE Batch Endpoint (`routes/batch.rs`)
```rust
// GET /api/batch?url=<channel_url>
// Response: text/event-stream
// Events: data: {"type":"link","url":"...","title":"...","index":1,"total":50}
//         data: {"type":"done","total":50}
//         data: {"type":"error","message":"..."}
async fn batch_handler(Query(params): Query<BatchParams>) -> Sse<impl Stream<Item=Event>>
```
- Use `axum::response::Sse` with `keep_alive(Duration::from_secs(15))`
- Call `extractor::extract_channel(url)` → async stream of video metadata
- Map each video → SSE `Event::default().data(json!({...}))`

## Todo
- [x] axum router with CORS + tracing
- [x] POST /api/extract handler
- [x] GET /api/stream handler with chunked pipe
- [x] GET /api/batch SSE handler
- [x] ProxyClient (reqwest wrapper)
- [x] Range request support (HTTP 206)
- [x] URL allowlist validation
- [ ] Integration test: stream 10MB YouTube chunk end-to-end
- [ ] Integration test: SSE batch → receive 10 video links from mock channel

## Success Criteria
- First byte delivered to browser in <500ms
- 1GB video streams without OOM (zero buffering in app)
- Range resume works (pause/resume download)
- CORS allows frontend on Cloudflare Pages origin

## Risk Assessment
| Risk | Mitigation |
|---|---|
| YouTube CDN returns 403 on direct proxy | Anti-bot layer (Phase 04) adds proper headers/cookies |
| Large file causes memory spike | Use `bytes_stream()` not `.bytes()` — streaming not buffered |
| Stream URL expires (YouTube signed URLs ~6h) | Re-extract on 403; add TTL cache for extracted URLs |

## Security
- Never proxy arbitrary URLs — strict allowlist check
- Rate limit per IP: 10 req/min on /api/extract
