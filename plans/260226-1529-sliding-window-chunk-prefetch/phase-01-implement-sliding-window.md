---
title: "Phase 01 – Implement Sliding Window Pre-fetch"
status: completed
priority: P2
effort: 2h
---

# Phase 01: Implement Sliding Window Pre-fetch

## Context Links

- Source: `crates/muxer/src/stream_fetcher.rs`
- Plan: `./plan.md`

## Overview

Refactor `fetch_stream_chunked` to issue chunk N+1 HTTP request while chunk N
bytes are still being streamed, hiding the next-chunk RTT behind current chunk
download time.

## Key Insight: Current Flow vs Target Flow

```
CURRENT (sequential):
  chunk0: [request→RTT→download] → chunk1: [request→RTT→download] → ...
                                             ^^^^^^^^^^ dead time

TARGET (sliding window):
  chunk0: [request→RTT→download]
                          chunk1: [request→RTT→download]
                                   ^^^^^^ overlap: RTT hidden
```

## Architecture

### Approach: Prefetch Handle via `tokio::task::JoinHandle`

Inside the spawned task in `fetch_stream_chunked`, maintain an
`Option<JoinHandle<Result<ChunkStream, ...>>>` that holds the in-flight prefetch
request.

```
State per iteration:
  current_stream: already-connected response stream for chunk N
  prefetch_handle: Option<JoinHandle> for chunk N+1 request
```

Loop structure:

```
1. Compute next_chunk range (N+1)
2. If not last chunk → spawn prefetch task for chunk N+1 HTTP request only
   (connect + response headers, NOT streaming bytes)
3. Stream chunk N bytes → tx (retry logic unchanged)
4. When chunk N complete → await prefetch_handle to get chunk N+1 stream
5. If prefetch failed → fall into existing retry loop for N+1
6. Repeat
```

### Prefetch Task Responsibility

The prefetch task calls `task_client.fetch_stream(url, range)` and returns the
resulting stream (or error). It does **not** read any bytes — just establishes
the TCP connection and receives response headers.

This means: connection + TLS + HTTP headers for N+1 happen during N download.

### Type for Prefetch Result

```rust
type PrefetchResult = Result</* stream type from AntiBotClient */, AntiBotError>;
```

Use `tokio::task::JoinHandle<PrefetchResult>`.

## Implementation Steps

### Step 1 – Define helper types inside the spawned task scope

```rust
// Inside tokio::spawn closure, before the while loop:
let mut prefetch: Option<tokio::task::JoinHandle<Result<_, AntiBotError>>> = None;
```

### Step 2 – Compute lookahead range before streaming

At the top of each while-loop iteration:

```rust
let next_offset = chunk_end + 1;
let has_next_chunk = next_offset < total_size;

// Spawn prefetch for next chunk if applicable
if has_next_chunk && prefetch.is_none() {
    let next_end = (next_offset + YOUTUBE_CHUNK_SIZE - 1).min(total_size - 1);
    let next_range = format!("bytes={}-{}", next_offset, next_end);
    let pf_url = url_owned.clone();
    // Share one client instance across current chunk + prefetch task
    let pf_client = Arc::clone(&task_client);
    prefetch = Some(tokio::spawn(async move {
        pf_client.fetch_stream(&pf_url, Some(next_range)).await
    }));
}
```

### Step 3 – Stream chunk N bytes (retry logic unchanged)

Keep the existing `'retry` loop verbatim. The only addition: if `tx.send`
returns `Err` (receiver dropped), **abort the prefetch** before returning:

```rust
if tx.send(Ok(bytes)).await.is_err() {
    if let Some(h) = prefetch.take() { h.abort(); }
    return;
}
```

Also abort on terminal error:

```rust
// before every `return` in the retry loop:
if let Some(h) = prefetch.take() { h.abort(); }
return;
```

### Step 4 – Consume prefetch result when chunk N completes

After `break 'retry` (chunk N done), before advancing `offset`:

```rust
offset = chunk_end + 1;

// If prefetch was spawned, await it to get the stream for next chunk
// If prefetch failed, clear it so the normal retry loop handles N+1
if let Some(handle) = prefetch.take() {
    match handle.await {
        Ok(Ok(stream)) => {
            // Store stream to use as the first attempt for chunk N+1
            // Pass via a local Option instead of going through retry loop
            next_stream = Some(stream);
        }
        Ok(Err(_)) | Err(_) => {
            // Prefetch failed or panicked: let retry loop handle N+1 normally
            // next_stream stays None
        }
    }
}
```

Modify the retry loop entry: if `next_stream.take()` is Some, skip the
`fetch_stream` call and use that stream directly on first attempt.

### Step 5 – Thread `next_stream` through retry loop

`fetch_stream` returns `impl Stream<Item = Result<Bytes, AntiBotError>>` (opaque type).
Must box it as `ByteStream` (`Pin<Box<dyn Stream...>>`):

Before the while loop:

```rust
let mut next_stream: Option<ByteStream> = None;
```

Inside `'retry` loop, at request phase:

```rust
let mut chunk_stream = if retry_count == 0 {
    if let Some(s) = next_stream.take() {
        s // use prefetched stream directly
    } else {
        match task_client.fetch_stream(&url_owned, Some(range)).await { ... }
    }
} else {
    match task_client.fetch_stream(&url_owned, Some(range)).await { ... }
};
```

## Related Code Files

| File | Change |
|------|--------|
| `crates/muxer/src/stream_fetcher.rs` | Refactor `fetch_stream_chunked` only |

## Todo List

- [x] Verified: `AntiBotClient` has NO Clone → share one client via `Arc<AntiBotClient>`
- [x] Verified: `fetch_stream` returns opaque type → box as `ByteStream`
- [x] Verified: `JoinHandle::abort()` drops TCP connection correctly
- [x] Add `prefetch: Option<JoinHandle>` before while loop
- [x] Add `next_stream: Option<stream>` before while loop
- [x] Spawn prefetch at start of each iteration (when `has_next_chunk`)
- [x] Abort prefetch on all `return` paths (disconnect + terminal error)
- [x] Await prefetch after `break 'retry`, populate `next_stream`
- [x] Thread `next_stream` into retry loop first-attempt branch
- [x] Run `cargo check -p muxer` to verify no compile errors
- [x] Run existing tests: `cargo test -p muxer`

## Success Criteria

- `cargo check -p muxer` passes with no errors/warnings
- Existing tests in `stream_fetcher.rs` pass unchanged
- For a 2-chunk stream: chunk 1 request is in-flight before chunk 0 finishes
- On disconnect mid-chunk: prefetch JoinHandle is aborted (no resource leak)
- On prefetch failure: chunk N+1 falls back to normal retry loop transparently

## Risk Assessment

| Risk | Mitigation |
|------|-----------|
| `AntiBotClient` not `Clone` | Share one client via `Arc<AntiBotClient>` between chunk stream and prefetch task |
| JoinHandle type complexity | Use concrete stream type alias or `Box<dyn ...>` if needed |
| Prefetch spawns while previous prefetch still pending | `prefetch.is_none()` guard prevents double-spawn |
| `fetch_start` resume broken | `next_stream` only used on `retry_count == 0`; mid-stream retry skips it |

## Verified Findings (2026-02-26)

| Question | Answer |
|----------|--------|
| `AntiBotClient` Clone? | ❌ No `#[derive(Clone)]`. Mitigation: share a single client via `Arc<AntiBotClient>` between current chunk and prefetch task. |
| Return type of `fetch_stream` | `impl Stream<Item = Result<Bytes, AntiBotError>>` (opaque). Must box as `ByteStream = Pin<Box<dyn Stream...>>`. One allocation, acceptable. |
| `JoinHandle::abort()` cancels HTTP? | ✅ Yes — drops the future at next `.await`, reqwest drops response → OS closes TCP connection. No lingering. |
