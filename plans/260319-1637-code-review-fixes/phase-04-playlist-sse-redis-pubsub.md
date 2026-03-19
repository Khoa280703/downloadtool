# Phase 04 — Playlist SSE Optimization (DEFERRED)

## Context Links
- Mux job SSE (reference impl): `crates/api/src/routes/jobs.rs:190-223` — Redis pub/sub
- Playlist SSE (current): `crates/api/src/routes/playlist_jobs.rs:223-250` — DB polling every 500ms
- Mux progress in playlist: `crates/api/src/routes/playlist_jobs.rs:489-501` — reads `job_progress_store.read_snapshot(mux_job_id)`
- Job progress store: `crates/job-system/src/job_progress.rs` — `JobProgressStore` with `subscribe()` at line 120

## Overview
- **Priority:** P2 Medium (optimization)
- **Status:** **DEFERRED**
- **Effort:** 4h+ (originally estimated 2h, underestimated due to mux progress bridging)
- Playlist SSE polls DB every 500ms. Mux job SSE uses Redis pub/sub. Inconsistent but functional.
- **DEFERRED because:** Simple "changed signal" approach breaks mux progress display. Proper fix requires multi-channel subscription per active mux item — complexity not justified at current traffic level.

## Key Insights
- **Mux progress regression risk:** Playlist SSE currently polls DB AND reads `job_progress_store.read_snapshot(mux_job_id)` for items being muxed. This gives live phase/percent updates (fetching_streams → muxing 45% → 90% → ready). A playlist-level "changed" signal won't fire during mux (playlist DB doesn't change), **breaking real-time mux progress**.
- To properly replace polling, SSE handler must subscribe to **each active item's mux job channel** — dynamic fan-in from N channels
- Current polling works correctly and maintains full mux progress visibility
- DB load from polling is minimal at current traffic (~2 queries/s per connection)

## Why Deferred (Not Cancelled)
- Revisit when concurrent playlist downloads exceed ~50 simultaneous connections
- Or when DB query latency shows degradation from SSE polling
- The correct approach when revisiting: hybrid model — playlist signal + fan-in from active mux job channels

## Minimal Alternative (Optional, Low Risk)
If DB load becomes a concern before full refactor:
- Increase polling interval from 500ms to 1000ms
- Only poll mux progress for items in `processing` status (skip completed/failed items)
- ~2 lines of code change, no architecture change

## Todo List
- [ ] (DEFERRED) Revisit when traffic justifies refactor
- [ ] (OPTIONAL) Reduce polling interval if DB load is observed

## Risk Assessment
- **Current approach:** Low risk, works correctly
- **Full refactor:** High risk of mux progress regression, significant complexity
- **Minimal alternative:** Zero risk, trivial change
