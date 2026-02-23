---
title: "fMP4 Box-Level Remuxer"
description: "Replace broken fmp4_muxer with copy-based box remuxer + add chunked CDN bypass to stream_fetcher"
status: pending
priority: P1
effort: 12h
branch: main
tags: [muxer, fmp4, performance, bugfix]
created: 2026-02-23
---

# fMP4 Box-Level Remuxer

## Problem
1. `/api/stream/muxed` slow -- no chunked CDN bypass in `StreamFetcher` (YouTube throttles to ~248 KB/s)
2. Downloaded muxed files unplayable -- `fmp4_muxer.rs` uses placeholder codec data (empty SPS/PPS) + hardcoded 1000ms timing

## Key Insight
YouTube adaptive streams are already valid fMP4/CMAF. No codec parsing needed. Just re-box at box level:
- Parse box boundaries from source streams
- Merge moov boxes (video trak_id=1, audio trak_id=2)
- Patch track_id in audio fragments' tfhd boxes
- Renumber mfhd.sequence_number

## Phases

| # | Phase | File | Status | Est |
|---|-------|------|--------|-----|
| 1 | [Chunked CDN bypass](./phase-01-chunked-bypass-stream-fetcher.md) | `stream_fetcher.rs` | pending | 2h |
| 2 | [Box parser](./phase-02-box-parser.md) | `box_parser.rs` (new) | pending | 3h |
| 3 | [Moov merger](./phase-03-moov-merger.md) | `moov_merger.rs` (new) | pending | 3h |
| 4 | [fMP4 remuxer](./phase-04-fmp4-remuxer.md) | `fmp4_remuxer.rs` (new) | pending | 4h |

## File Changes
```
crates/muxer/src/
  box_parser.rs          (NEW ~150 lines)
  moov_merger.rs         (NEW ~200 lines)
  fmp4_remuxer.rs        (NEW ~250 lines, replaces fmp4_muxer.rs)
  stream_fetcher.rs      (UPDATE: add chunked bypass)
  lib.rs                 (UPDATE: swap exports)
  fmp4_muxer.rs          (DELETE after phase 4)
  codec.rs               (KEEP but no longer used by remuxer)
  mux_router.rs          (NO CHANGE)

crates/api/src/routes/
  stream.rs              (UPDATE: call remux_streams, remove codec params)
```

## Dependencies
- Phase 2 blocks Phase 3 (moov_merger uses box_parser)
- Phase 2+3 block Phase 4 (remuxer uses both)
- Phase 1 is independent (can parallelize with 2)

## Risk
- YouTube may change fMP4 structure (unlikely, CMAF is standard)
- Extended box sizes (>4GB) need 8-byte size handling in box_parser
- Some streams may use version 1 tkhd (8-byte fields) -- handled in moov_merger
