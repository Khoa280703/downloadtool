# Phase 01: Bookmarklet (apps/injector)

## Context Links
- Parent plan: [plan.md](./plan.md)
- Depends on: [phase-00-monorepo-setup-api-client.md](./phase-00-monorepo-setup-api-client.md)
- Reused by: [phase-04-userscript.md](./phase-04-userscript.md)

## Overview

| Field | Value |
|-------|-------|
| Priority | P1 |
| Status | ✅ completed |
| Description | Vite-bundled IIFE bookmarklet — injects Download button on YouTube, uses Shadow DOM for CSS isolation |
| Effort | 1d |
| Blocked by | Phase 0 complete |

## Key Insights

- `apps/injector/` is shared by both bookmarklet (Phase 1) and userscript (Phase 4) — DRY
- Vite builds IIFE (`bm.js`) with no external deps — tree-shakes api-client types
- Shadow DOM is required: YouTube CSS bleeds into injected UI without it
- Backend serves `dist/bm.js` at `GET /bm.js` via `include_str!` (compile-time embed)
- Loader bookmarklet stays tiny (< 200 chars): fetches and evals `bm.js`
- Vanilla TS only — no Svelte in injector

## Requirements

**Functional:**
- Bookmarklet detects `youtube.com/watch?v=` URL, injects Download button
- Shadow DOM wraps all injected UI
- Button click → `POST /api/extract` → quality picker → `/api/stream/muxed`
- Works in Chrome, Firefox, Safari, Edge

**Non-functional:**
- `bm.js` output: IIFE, no external runtime deps
- Loader bookmarklet < 200 chars
- Vanilla TS only (no Svelte)

## Architecture

```
apps/injector/
  ├── src/
  │   ├── bookmarklet.ts        # Entry point → bm.js
  │   ├── userscript.ts         # Entry point → userscript.js (Phase 4)
  │   └── shared/
  │       ├── inject-button.ts  # DOM injection + Shadow DOM setup
  │       ├── quality-picker.ts # Modal UI inside Shadow DOM
  │       └── stream-utils.ts   # Filter WebM-only, build muxed URL
  ├── vite.config.ts
  └── package.json

Backend serves:
  GET /bm.js → apps/injector/dist/bm.js (include_str! at compile time)
```

## Related Code Files

- Create: `apps/injector/package.json`
- Create: `apps/injector/vite.config.ts`
- Create: `apps/injector/src/bookmarklet.ts`
- Create: `apps/injector/src/shared/inject-button.ts`
- Create: `apps/injector/src/shared/quality-picker.ts`
- Create: `apps/injector/src/shared/stream-utils.ts`
- Create: `backend/crates/api/src/routes/static_files.rs`
- Modify: `backend/crates/api/src/main.rs` — add `/bm.js` route

## Implementation Steps

1. **Create `apps/injector/package.json`**
   - `name`: `@downloadtool/injector`
   - `scripts.build`: `vite build`
   - `devDependencies`: `vite`, `typescript`, `@downloadtool/api-client`

2. **Create `apps/injector/vite.config.ts`**
   - Two lib entries: `bookmarklet` → IIFE → `dist/bm.js`, `userscript` → IIFE → `dist/youtube-downloader.user.js`
   - `build.minify: true`, no external deps

3. **Create shared modules** (`apps/injector/src/shared/`)
   - `inject-button.ts`: find YouTube actions container (`#above-the-fold`, fallback selectors); create Shadow DOM host; idempotency guard
   - `quality-picker.ts`: modal rendered inside Shadow DOM; self-contained CSS (no external stylesheet)
   - `stream-utils.ts`: filter WebM video-only streams; build `/api/stream/muxed` URL params

4. **Create `apps/injector/src/bookmarklet.ts`**
   - Import shared modules
   - Guard: `if (!location.href.includes('youtube.com/watch')) return`
   - Call `injectButton()` → on click call `POST /api/extract` via api-client → `showQualityPicker(streams)`

5. **Create `backend/crates/api/src/routes/static_files.rs`**
   - `GET /bm.js`: `include_str!` path to `apps/injector/dist/bm.js`; `Content-Type: application/javascript`; `Cache-Control: public, max-age=3600`
   - `GET /userscript`: placeholder for Phase 4

6. **Update `backend/crates/api/src/main.rs`**
   - Register `/bm.js` and `/userscript` routes

7. **Add bookmarklet install UI to frontend**
   - Loader code (DOM injection — NOT eval, blocked by YouTube CSP):
     ```js
     javascript:(function(){var s=document.createElement('script');s.src='https://YOUR_DOMAIN/bm.js?t='+Date.now();document.body.appendChild(s);})()
     ```
   - ⚠️ **CSP caveat**: YouTube CSP `script-src` may also block external domain `<script src>` in Chrome. Test on real page. Fallback: inline all logic in bookmarklet (no external load).
   - Draggable `<a href="javascript:...">` link with install instructions

8. **Build and verify**
   - `pnpm --filter @downloadtool/injector build`
   - Check `dist/bm.js` is IIFE, minified
   - Start backend, navigate to YouTube, click bookmarklet

## Todo List

- [ ] Create `apps/injector/package.json` + `vite.config.ts`
- [ ] Create `apps/injector/src/shared/` modules (inject-button, quality-picker, stream-utils)
- [ ] Create `apps/injector/src/bookmarklet.ts`
- [ ] Create `backend/crates/api/src/routes/static_files.rs`
- [ ] Register `/bm.js` route in `backend/crates/api/src/main.rs`
- [ ] Add bookmarklet install UI to frontend homepage
- [ ] Build, test in Chrome + Firefox

## Success Criteria

- `pnpm build` in `apps/injector/` produces `dist/bm.js` (IIFE, minified)
- Backend serves `GET /bm.js` correctly
- Download button appears in Shadow DOM on YouTube watch pages
- Quality picker shows streams (no WebM video-only)
- Download initiates via `/api/stream/muxed`

## Risk Assessment

| Risk | Likelihood | Mitigation |
|------|-----------|------------|
| YouTube DOM changes break injection selector | Medium | Multiple fallback selectors in `inject-button.ts` |
| Shadow DOM CSS leakage from YouTube | Low | Shadow DOM with `mode: 'closed'` prevents bleed |
| `include_str!` path breaks on different build environments | Low | Use workspace-relative path, document in README |

## Next Steps

- Phase 4: UserScript reuses `apps/injector/src/shared/` + adds `userscript.ts` entry
