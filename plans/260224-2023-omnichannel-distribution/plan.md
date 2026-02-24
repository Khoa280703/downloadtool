---
title: "Omnichannel Distribution - Monorepo Lean"
description: "pnpm workspaces + SvelteKit PWA + MV3 Extension + Vite Injector for YouTube downloader"
status: completed
priority: P2
effort: 4w
branch: main
tags: [monorepo, pnpm, sveltekit, pwa, extension, mv3, bookmarklet, userscript]
created: 2026-02-24
---

# Omnichannel Distribution - Monorepo Lean

## Overview

Expand YouTube downloader to multiple distribution channels using a pnpm monorepo. Backend API contracts unchanged.

**Backend already working:**
- `POST /api/extract` → JSON video info + stream list
- `GET /api/stream/muxed?video_url=...&audio_url=...&title=...` → fMP4 stream

## Monorepo Structure

```
downloadtool/
├── backend/                    # Rust workspace (no API changes)
│   └── crates/...
├── packages/
│   └── api-client/             # utoipa → openapi.json → openapi-ts generated TS types
└── apps/
    ├── web/                    # SvelteKit PWA
    ├── extension/              # Vite + plain MV3 (Firefox + Edge shared source)
    └── injector/               # Vite → bm.js + userscript.js
```

**Root tooling:** pnpm workspaces (Turborepo deferred until build time > 2min)

## Phases

| # | Phase | Status | Priority | Est. |
|---|-------|--------|----------|------|
| 0 | [Monorepo Setup & API Client](./phase-00-monorepo-setup-api-client.md) | ✅ completed | P1 | 2d |
| 1 | [Bookmarklet (apps/injector)](./phase-01-bookmarklet-injector.md) | ✅ completed | P1 | 1d |
| 2 | [PWA (apps/web SvelteKit)](./phase-02-pwa-sveltekit.md) | ✅ completed | P1 | 1.5d |
| 3 | [Extension Firefox+Edge (apps/extension)](./phase-03-extension-firefox-edge.md) | ✅ completed | P2 | 2d |
| 4 | [UserScript (apps/injector)](./phase-04-userscript.md) | ✅ completed | P2 | 1d |

## Key Dependencies

- Phase 0 must complete first — api-client package enables type-safe calls for all phases
- Phase 4 (UserScript) reuses `apps/injector/` from Phase 1 (DRY)
- Backend: add `GET /openapi.json`, `GET /bm.js`, `GET /userscript` routes

## Key Technical Decisions

1. pnpm workspaces over Turborepo — simpler, add caching later
2. NO crxjs — too fragile, version-locked; use plain MV3 Vite
3. Shadow DOM required for extension content script (CSS isolation on YouTube)
4. `chrome.downloads` > `window.open` for extension downloads
5. Web Share Target API — high value for Android (share from YouTube app → PWA)
6. Background Fetch API — high value for mobile (download survives app close)
7. File System Access API — deferred (Chrome-only, YAGNI)
8. shared-ui Svelte library — deferred (extract after 2 apps exist)
9. utoipa + openapi-ts — setup Week 1 (high value, low effort)
