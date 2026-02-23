---
title: "YouTube Download: Fix Timeout + N-Parameter Throttle"
description: "Fix download stopping mid-way (timeout bug) and extremely slow speeds (n-param throttle)"
status: completed
priority: P1
effort: 3h
branch: main
tags: [youtube, performance, download, bugfix]
created: 2026-02-23
---

# YouTube Download: Fix Timeout + N-Parameter Throttle

## Problem Summary

Two bugs cause YouTube downloads to fail or be extremely slow:

1. **Download stops mid-way** -- `Client::builder().timeout(30s)` in `anti_bot.rs` kills the entire HTTP transfer after 30s, not just connection establishment
2. **Download very slow (50-200 KB/s)** -- YouTube CDN throttles when `n` parameter in stream URLs is not transformed via player JS function

## Phase Overview

| Phase | Description | Effort | Status |
|-------|-------------|--------|--------|
| [Phase 1](phase-01-fix-timeout-bug.md) | Fix timeout: `.timeout()` -> `.connect_timeout()` | 30min | completed |
| [Phase 2](phase-02-implement-n-param-transform.md) | Implement n-parameter transform in TS extractor | 2.5h | completed |

## Key Files

- `crates/proxy/src/anti_bot.rs` -- line 99, `build_client()` timeout config
- `extractors/youtube-innertube.ts` -- InnerTube extractor, add n-param transform post-processing
- `extractors/youtube.ts` -- main YouTube extractor (HTML fallback also needs n-param)
- `crates/extractor/src/runtime.rs` -- deno_core runtime, domain whitelist needs update

## Dependencies

- Phase 2 depends on Phase 1 (test full pipeline after both fixes)
- Phase 2 requires adding `googlevideo.com` subdomains to fetch whitelist (already present in runtime.rs)

## Build & Test

```bash
# Rust build
~/.cargo/bin/cargo b

# JS bundle
cd extractors && npx esbuild youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=dist/youtube.js

# Manual test: download a YouTube video and verify full-speed transfer
```

## Success Criteria

- Downloads no longer timeout after 30s for large files
- YouTube CDN delivers at full speed (typically 5-50 MB/s) instead of throttled 50-200 KB/s
- No regressions in existing extraction flow
