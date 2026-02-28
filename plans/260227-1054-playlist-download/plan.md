---
title: "Playlist Downloading Feature"
description: "End-to-end playlist download (?list= URLs only): TypeScript InnerTube extractor → Rust SSE batch handler → SvelteKit IndexedDB queue with concurrency pool"
status: pending
priority: P1
effort: 8h
branch: main
tags: [playlist, batch, sse, innertube, indexeddb, fsaa]
created: 2026-02-27
---

# Playlist Download Feature

## Architecture

**"Frontend is Brain (Stateful), Backend is Muscle (Stateless)"**

```
User pastes playlist URL
  → BatchInput.svelte → GET /api/batch?url=...
  → Rust SSE streams {videoId, title, thumbnail} events  ← camelCase, serde rename
    → Rust calls extractors["youtube"].extractPlaylist(url)  ← passes full URL, TS parses ID
    → TS uses InnerTube browse API, fetches all pages, returns BatchVideoInfo[] array to Rust
  → Frontend stores each entry in IndexedDB (status: pending)
  → Concurrency pool (max 1) picks pending entries:
      POST /api/extract  → pick best format → GET /api/stream/muxed or /api/stream
      → File System Access API (or <a download> fallback)
  → On refresh: read IndexedDB, re-queue pending/failed
```

## Phases

| # | Phase | File | Status | Est |
|---|-------|------|--------|-----|
| 1 | TypeScript playlist extractor | `extractors/youtube-playlist.ts` | completed | 2h |
| 2 | Rust batch_handler real impl | `crates/api/src/routes/batch.rs` | completed | 2h |
| 3 | Frontend IndexedDB + FSAA + pool | `frontend/src/lib/playlist-*.ts` | completed | 2h |
| 4 | Frontend UI wiring | `BatchInput.svelte`, `BatchActiveState.svelte`, `batch.ts` | completed | 2h |

## Key Constraints
- No GPU, no new video format logic — reuse existing stream selection
- Files stay under 200 lines — split as needed
- YAGNI/KISS/DRY — no over-engineering
- SSE client disconnect → async_stream drops SSE loop immediately; extractor std::thread runs to completion (result discarded) — acceptable tradeoff, extraction finishes in seconds
- MAX_CONCURRENT = 1 (dev: keep low to avoid exhausting Vite proxy HTTP/1.1 pool)
- READY_QUEUE_MAX = 1 (dev: same reason)

## Links
- [Phase 1 - TypeScript Extractor](./phase-01-typescript-playlist-extractor.md)
- [Phase 2 - Rust batch_handler](./phase-02-rust-batch-handler.md)
- [Phase 3 - Frontend IndexedDB + FSAA](./phase-03-frontend-indexeddb-fsaa.md)
- [Phase 4 - Frontend UI Wiring](./phase-04-frontend-ui-wiring.md)
