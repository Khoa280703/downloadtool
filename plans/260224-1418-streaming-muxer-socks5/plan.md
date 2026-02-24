---
title: "Production-Grade Streaming Muxer & SOCKS5 Routing"
description: "Refactor buffer-based muxer to streaming pipeline; add SOCKS5 selective routing for extraction"
status: pending
priority: P1
effort: 3-4 days
branch: main
tags: [muxer, streaming, socks5, performance, scalability]
created: 2026-02-24
---

# Production-Grade Streaming Muxer & SOCKS5 Routing

## Problem Summary

| Problem | Severity | Impact |
|---|---|---|
| Muxer buffers full video+audio in RAM before output | CRITICAL | OOM crash with 4-5 concurrent 4K users (~5GB/req) |
| YouTube extraction uses direct HTTP (no proxy routing) | MEDIUM | Rate-limited/blocked on home server IP |

## Phases

| # | Phase | Status | Effort |
|---|---|---|---|
| 1 | [Streaming Muxer Refactor](./phase-01-streaming-muxer-refactor.md) | pending | 2-3 days |
| 2 | [SOCKS5 Selective Routing](./phase-02-socks5-selective-routing.md) | pending | 0.5-1 day |

## Key Dependencies

- Phase 2 is independent from Phase 1 — can be implemented in parallel
- Phase 1 must keep `remux_streams()` signature unchanged (API stability)
- Existing `FragmentReader` + `merge_fragments()` + `merge_moov()` are reusable as-is

## Files Touched

**Phase 1 (new files):**
- `crates/muxer/src/atom_framer.rs` (new)
- `crates/muxer/src/fragment_aligner.rs` (new)
- `crates/muxer/src/fmp4_remuxer.rs` (replace body, keep pub API)
- `crates/muxer/src/lib.rs` (add new module declarations)

**Phase 2 (modify existing):**
- `crates/extractor/src/runtime.rs` (`op_fetch` fn)
- `crates/proxy/Cargo.toml` (add `reqwest/socks` feature)
