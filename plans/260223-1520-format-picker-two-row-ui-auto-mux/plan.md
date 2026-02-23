---
title: "FormatPicker Two-Row UI + Auto-Mux"
description: "Redesign quality picker with 2-row resolution/codec selection and transparent audio muxing for video-only streams"
status: pending
priority: P2
effort: 3h
branch: main
tags: [frontend, svelte, extractor, typescript, ux]
created: 2026-02-23
---

# FormatPicker Two-Row UI + Auto-Mux

## Problem Statement

1. **Duplicate options** -- YouTube returns both H.264 and AV1 at same resolution; current flat list shows "1080p MP4" twice
2. **Missing stream metadata** -- `hasAudio`, `isAudioOnly`, `codecLabel` not populated from InnerTube extractor
3. **No auto-muxing** -- DownloadBtn always calls `/api/stream`; 1080p+ video-only streams download without audio
4. **Broken field mapping** in `api.ts` -- uses `f.acodec`/`f.vcodec` (yt-dlp format) but backend returns InnerTube fields (`mimeType`, `codec`)

## Solution Overview

- Add `hasAudio`, `isAudioOnly`, `codecLabel` to extractor Stream type + populate in `parseFormat()`
- Redesign FormatPicker: Row 1 = resolution buttons, Row 2 = codec buttons for selected resolution
- DownloadBtn auto-detects video-only streams and calls `/api/stream/muxed` with best audio track
- Backend Rust unchanged -- `/api/stream/muxed` endpoint already exists

## Architecture

```
[InnerTube API] -> parseFormat() adds hasAudio/isAudioOnly/codecLabel
                          |
                   [Deno V8 extractor]
                          |
                   [Rust /api/extract] -> returns streams with new fields
                          |
                   [Frontend api.ts] -> maps to frontend Stream type
                          |
              [FormatPicker] -----------> [DownloadBtn]
              2-row UI:                   if video-only:
              - resolution row              find best audio stream
              - codec row                   call /api/stream/muxed
                                          else:
                                            call /api/stream (current)
```

## Phases

| # | Phase | Files | Status |
|---|-------|-------|--------|
| 1 | [Extractor stream fields](./phase-01-extractor-stream-fields.md) | `extractors/types.ts`, `extractors/youtube-innertube.ts`, esbuild rebuild | pending |
| 2 | [Frontend API + types](./phase-02-frontend-api-types.md) | `frontend/src/lib/types.ts`, `frontend/src/lib/api.ts` | pending |
| 3 | [FormatPicker redesign](./phase-03-format-picker-redesign.md) | `frontend/src/components/FormatPicker.svelte` | pending |
| 4 | [DownloadBtn auto-mux](./phase-04-download-btn-auto-mux.md) | `frontend/src/components/DownloadBtn.svelte`, `frontend/src/routes/+page.svelte` | pending |

## Dependencies

- Phase 1 must complete before Phase 2 (frontend needs new fields)
- Phase 2 must complete before Phases 3 & 4
- Phases 3 & 4 can run in parallel after Phase 2
- Backend Rust: NO changes needed

## Risk Assessment

- **InnerTube mimeType parsing** -- regex already exists in `parseFormat()`, just needs to extract codec info; low risk
- **esbuild bundle** -- existing build command; rebuild after TS changes
- **FormatPicker signature change** -- parent `+page.svelte` must adapt to new `onSelect(video, audio)` signature
- **Muxed endpoint params** -- must match Rust `MuxedStreamParams`: `video_url`, `audio_url`, `title`
