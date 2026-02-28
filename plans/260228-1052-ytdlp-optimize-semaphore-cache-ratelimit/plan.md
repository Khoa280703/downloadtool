---
title: "yt-dlp Performance & Reliability Optimizations"
description: "5 targeted optimizations: faster cmd args, semaphore, in-memory cache, rate limiting, and binary path detection"
status: pending
priority: P2
effort: 3h
branch: main
tags: [rust, ytdlp, performance, rate-limiting, cache]
created: 2026-02-28
---

# yt-dlp Optimize: Semaphore + Cache + Rate Limit

## Overview

5 targeted optimizations to `crates/extractor/src/ytdlp.rs` and `crates/api/` to improve performance, protect the server from abuse, and make the binary more portable.

Most changes are internal/additive. **API contract note:** Phase 04 introduces new HTTP responses: `429 Too Many Requests` (rate limit exceeded) and `403 Forbidden` (unidentifiable client IP). Clients should handle these.

## Phases

| # | Phase | Files | Status |
|---|-------|-------|--------|
| 1 | [yt-dlp Command Args + Binary Path](phase-01-ytdlp-command-args.md) | `ytdlp.rs` | pending |
| 2 | [Tokio Semaphore](phase-02-semaphore.md) | `ytdlp.rs` | pending |
| 3 | [In-memory URL Cache (moka)](phase-03-url-cache.md) | `ytdlp.rs`, `extractor/Cargo.toml` | pending |
| 4 | [Rate Limiting per IP](phase-04-rate-limiting.md) | `main.rs`, `api/Cargo.toml` | pending |

## Key Dependencies

- Phase 1 independent
- Phase 2 independent (add to `ytdlp.rs` after phase 1 done)
- Phase 3 depends on phase 2 (both modify `ytdlp.rs` — sequential to avoid conflicts)
- Phase 4 fully independent

## New Crate Dependencies

| Crate | Version | Location | Why |
|-------|---------|----------|-----|
| `moka` | `0.12` | `extractor/Cargo.toml` | async LRU cache with TTL |
| `governor` | `0.8` | `api/Cargo.toml` | token bucket rate limiter (axum-version-agnostic) |

## Risk

Low. All changes additive; no removal of existing logic. Semaphore + cache + rate limit have no effect on correctness if disabled.
