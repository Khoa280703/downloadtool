---
title: "Code Review Fixes — 6 Findings"
description: "Fix SSRF, playlist mode mismatch, CI gap, SSE polling, docs drift, and hotspot files"
status: pending
priority: P1
effort: 8h
branch: main
tags: [security, bugfix, ci, optimization, docs, tech-debt]
created: 2026-03-19
---

# Code Review Fixes — 6 Findings

## Phase Summary

| # | Phase | Priority | Effort | Status |
|---|-------|----------|--------|--------|
| 01 | [URL Validation SSRF Fix](phase-01-url-validation-ssrf-fix.md) | P0 Critical | 1h | pending |
| 02 | [Playlist Mode Backend](phase-02-playlist-mode-backend.md) | P1 High | 2h | pending |
| 03 | [CI Frontend Coverage](phase-03-ci-frontend-coverage.md) | P1 High | 30m | pending |
| 04 | [Playlist SSE Optimization](phase-04-playlist-sse-redis-pubsub.md) | P2 Medium | defer | **deferred** (mux progress regression risk) |
| 05 | [Docs Sync](phase-05-docs-sync.md) | P2 Low | 1h | pending |
| 06 | [Hotspot Modularization](phase-06-hotspot-modularization.md) | P3 Low | defer | deferred |

## Execution Order

1. **Phase 01** (SSRF) — security, ship immediately
2. **Phase 02** (playlist mode) — functional bug, next priority
3. **Phase 03** (CI) — quick win, can parallel with 01/02
4. **Phase 04** (SSE) — optimization, defer if under time pressure
5. **Phase 05** (docs) — housekeeping, do after functional fixes
6. **Phase 06** (hotspots) — tech debt, defer to natural refactoring

## Key Dependencies

- Phase 01: uses `reqwest::Url` (already available, no new dep needed)
- Phase 02: touches playlist_processor.rs; WebM filter must differ per mode (direct vs mux)
- Phase 03: fully independent, can run in parallel with any phase
- Phase 04: DEFERRED — "changed signal" approach would break mux progress display
- Phase 05: fully independent

## Risk Summary

- Phase 01: Low risk, additive validation change, no new dependencies
- Phase 02: Medium risk, changes stream selection logic; WebM must be allowed for direct download modes
- Phase 04: DEFERRED — high risk of mux progress regression if done naively
