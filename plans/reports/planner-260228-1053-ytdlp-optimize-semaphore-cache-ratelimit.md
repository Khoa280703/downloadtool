# Planner Report — yt-dlp Optimize: Semaphore + Cache + Rate Limit

**Date:** 2026-02-28
**Plan dir:** `plans/260228-1052-ytdlp-optimize-semaphore-cache-ratelimit/`

## Summary

4-phase plan covering 5 optimizations to improve performance, protect server resources, and make the binary portable. All changes additive and non-breaking.

## Phases

| Phase | File(s) | New deps | Effort |
|-------|---------|----------|--------|
| 01 — Command args + binary path | `ytdlp.rs` | none | 30m |
| 02 — Tokio semaphore (cap 10) | `ytdlp.rs` | none (tokio::sync already present) | 30m |
| 03 — moka URL cache TTL 5min | `ytdlp.rs`, `extractor/Cargo.toml` | `moka 0.12` | 45m |
| 04 — tower_governor rate limit | `main.rs`, `api/Cargo.toml` | `tower_governor 0.4` | 45m |

**Total estimated effort: ~2.5h**

## Execution Order

Phases 01 → 02 → 03 must be sequential (all touch `ytdlp.rs`). Phase 04 is fully independent and can run in parallel with 01-03.

## Key Design Decisions

- Cache check happens **before** semaphore acquire (hit = zero subprocess cost)
- `_permit` binding pattern guarantees semaphore release on all code paths (success + error)
- Rate limit via `.route_layer()` scoped to `/api/extract` only — stream routes unaffected
- `VideoInfo` requires `#[derive(Clone)]` for cache return — implementor must verify in `types.rs`

## Unresolved Questions

1. Is a reverse proxy (nginx/caddy) in front of the API? If yes, phase 04 needs `SmartIpKeyExtractor` instead of default `PeerIpKeyExtractor`.
2. Should `burst_size` be 5 or 10? Playlist-batch UI may fire multiple extract calls rapidly — needs clarification on expected concurrent use.
3. Does `VideoInfo` already derive `Clone`? (Quick check in `crates/extractor/src/types.rs` before implementing phase 03.)
