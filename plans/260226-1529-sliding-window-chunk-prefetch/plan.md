---
title: "Sliding Window Pre-fetch for YouTube CDN Chunk Fetcher"
description: "Overlap chunk N download with chunk N+1 prefetch to eliminate RTT overhead (~40% speed for 4K)"
status: completed
priority: P2
effort: 3h
branch: main
tags: [rust, performance, youtube, streaming, prefetch]
created: 2026-02-26
---

# Sliding Window Pre-fetch

## Overview

Currently `fetch_stream_chunked` fetches YouTube CDN chunks **sequentially**:
finish chunk N → start request for chunk N+1. The RTT (round-trip time) for each
new HTTP request is dead time where no bytes flow to the consumer.

**Goal:** hide chunk N+1 RTT behind chunk N download time by issuing the next
request while the current chunk is still streaming.

## Scope

Single phase. One function to refactor: `fetch_stream_chunked` in
`crates/muxer/src/stream_fetcher.rs`.

No changes to public API, no new crates, no new files.

## Phases

| Phase | File | Status |
|-------|------|--------|
| [01 – Implement sliding window](./phase-01-implement-sliding-window.md) | `stream_fetcher.rs` | completed |

## Key Constraints

- Max 2 concurrent range requests per stream (current + prefetch only)
- Preserve retry logic (`CHUNK_MAX_RETRIES`, exponential backoff, `fetch_start` resume)
- Cancel prefetch on client disconnect (`tx.send` returns `Err`)
- Last chunk: no prefetch spawned
- Must not change `ByteStream` type or any public interface
