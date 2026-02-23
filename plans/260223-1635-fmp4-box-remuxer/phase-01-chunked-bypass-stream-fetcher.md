# Phase 1: Chunked CDN Bypass in stream_fetcher.rs

## Context
- [stream_fetcher.rs](../../crates/muxer/src/stream_fetcher.rs) -- current impl, single request only
- [stream.rs](../../crates/api/src/routes/stream.rs) -- has working `chunked_stream()` for single-file downloads
- [client.rs](../../crates/proxy/src/client.rs) -- `ProxyClient::fetch_stream_with_headers(&self, url, range)`

## Overview
- **Priority:** P1
- **Status:** pending
- **Description:** Add chunked range-request bypass to `StreamFetcher::fetch_both()` so muxed downloads get full CDN speed

## Key Insights
- `stream.rs` already implements `chunked_stream()` using `ProxyClient` with `Range` headers
- YouTube CDN throttles full-file requests to ~2 Mbps but serves sub-range chunks at line speed
- `clen` param in YouTube URL gives total content length
- We need to replicate the chunked logic inside `StreamFetcher` but return a `ByteStream` (AntiBotError-based)
- `StreamFetcher` currently uses `AntiBotClient::fetch_stream()` which accepts `Option<String>` range

## Requirements
### Functional
- Extract `clen` from URL query params
- If `clen` found: fetch via sequential 9.5MB range chunks
- If no `clen`: fallback to current single-request behavior
- Return same `ByteStream` type signature

### Non-functional
- No new dependencies needed
- Reuse `YOUTUBE_CHUNK_SIZE = 9_500_000` constant (define in stream_fetcher or share)

## Architecture
```
fetch_both(video_url, audio_url, platform)
  ├─ tokio::join!(
  │    fetch_stream_chunked(client, video_url),
  │    fetch_stream_chunked(client, audio_url)
  │  )
  └─ return (video_stream, audio_stream)

fetch_stream_chunked(client, url)
  ├─ extract clen from URL
  ├─ if clen found:
  │    spawn task: loop range chunks → mpsc channel
  │    return channel as ByteStream
  └─ else:
       client.fetch_stream(url, None) → ByteStream
```

## Related Code Files
- **Modify:** `crates/muxer/src/stream_fetcher.rs`
- **Reference:** `crates/api/src/routes/stream.rs` lines 211-273 (chunked_stream fn)
- **Reference:** `crates/proxy/src/client.rs` (Range struct, ProxyClient)

## Implementation Steps

### Step 1: Add helper `extract_clen_from_url`
```rust
/// Extract `clen` (content length) from YouTube CDN URL query params.
fn extract_clen_from_url(url: &str) -> Option<u64> {
    url::Url::parse(url)
        .ok()?
        .query_pairs()
        .find(|(k, _)| k == "clen")
        .and_then(|(_, v)| v.parse().ok())
}
```
Note: `url` crate is transitively available via `reqwest`. May need to add `url` to Cargo.toml or use `reqwest::Url`.

### Step 2: Add `YOUTUBE_CHUNK_SIZE` constant
```rust
const YOUTUBE_CHUNK_SIZE: u64 = 9_500_000;
```

### Step 3: Add `fetch_stream_chunked` method
```rust
async fn fetch_stream_chunked(
    client: &AntiBotClient,
    url: &str,
) -> Result<ByteStream, AntiBotError> {
    let total_size = match extract_clen_from_url(url) {
        Some(size) => size,
        None => {
            // Fallback: single request
            let stream = client.fetch_stream(url, None).await?;
            return Ok(Box::pin(stream));
        }
    };

    info!("Chunked fetch: {} bytes in ~{}MB chunks", total_size, YOUTUBE_CHUNK_SIZE / 1_000_000);

    let (tx, rx) = tokio::sync::mpsc::channel::<Result<Bytes, AntiBotError>>(8);
    let url_owned = url.to_string();

    // Need to clone/create new client for the spawned task
    // AntiBotClient may need to be cloneable or we create new one
    let platform = client.platform(); // need to check if this getter exists

    tokio::spawn(async move {
        // Create new client inside task (AntiBotClient is not Send across await in spawn)
        let task_client = match AntiBotClient::new(platform) {
            Ok(c) => c,
            Err(e) => { let _ = tx.send(Err(e)).await; return; }
        };

        let mut offset = 0u64;
        while offset < total_size {
            let end = (offset + YOUTUBE_CHUNK_SIZE - 1).min(total_size - 1);
            let range = format!("bytes={}-{}", offset, end);

            match task_client.fetch_stream(&url_owned, Some(range)).await {
                Ok(mut chunk_stream) => {
                    use futures::StreamExt;
                    while let Some(item) = chunk_stream.next().await {
                        if tx.send(item).await.is_err() { return; }
                    }
                }
                Err(e) => {
                    error!("Chunk fetch failed bytes={}-{}: {}", offset, end, e);
                    let _ = tx.send(Err(e)).await;
                    return;
                }
            }
            offset = end + 1;
        }
    });

    let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    Ok(Box::pin(stream))
}
```

### Step 4: Update `fetch_both()` to use chunked fetch
```rust
pub async fn fetch_both(
    video_url: &str,
    audio_url: &str,
    platform: Platform,
) -> Result<(ByteStream, ByteStream), AntiBotError> {
    let client = AntiBotClient::new(platform)?;

    let (video_result, audio_result) = tokio::join!(
        Self::fetch_stream_chunked(&client, video_url),
        Self::fetch_stream_chunked(&client, audio_url)
    );

    Ok((video_result?, audio_result?))
}
```

### Step 5: Check if `AntiBotClient` has a `platform()` getter
If not, store platform in `fetch_stream_chunked` params directly:
```rust
async fn fetch_stream_chunked(
    platform: Platform,
    url: &str,
) -> Result<ByteStream, AntiBotError> { ... }
```

## Todo
- [ ] Add `extract_clen_from_url` helper
- [ ] Add `YOUTUBE_CHUNK_SIZE` constant
- [ ] Implement `fetch_stream_chunked`
- [ ] Update `fetch_both` to use chunked fetch
- [ ] Verify AntiBotClient cloneability or create new instance in spawn
- [ ] Add unit test for `extract_clen_from_url`
- [ ] Integration test with mock URLs

## Success Criteria
- `fetch_both()` returns chunked streams for YouTube CDN URLs with `clen`
- Non-YouTube URLs fallback to single request
- No breaking changes to `ByteStream` type

## Risk
- `AntiBotClient` may not be `Send` or cloneable -- need to create new instance inside spawned task
- `url` crate may need explicit Cargo.toml dependency (check if re-exported by reqwest)

## Unresolved Questions
- Does `AntiBotClient` expose a `platform()` getter? If not, pass Platform as param.
