# Phase 04: UserScript (apps/injector)

## Context Links
- Parent plan: [plan.md](./plan.md)
- Depends on: [phase-01-bookmarklet-injector.md](./phase-01-bookmarklet-injector.md)
- Shared source: `apps/injector/src/shared/`

## Overview

| Field | Value |
|-------|-------|
| Priority | P2 |
| Status | ✅ completed |
| Description | UserScript for Tampermonkey/Greasemonkey — reuses apps/injector shared modules, adds GM_xmlhttpRequest and SPA nav |
| Effort | 1d |
| Blocked by | Phase 1 complete (shared modules must exist) |

## Key Insights

- DRY: reuses `apps/injector/src/shared/` (inject-button, quality-picker, stream-utils) from Phase 1
- Only new file: `apps/injector/src/userscript.ts` (entry point)
- `GM_xmlhttpRequest` bypasses CORS in userscript context — more reliable than fetch
- Vite builds IIFE → Vite plugin prepends `==UserScript==` header block
- Backend serves `dist/youtube-downloader.user.js` at `GET /userscript` (already planned in Phase 1's `static_files.rs`)
- Tampermonkey auto-update via `@updateURL` + `@downloadURL`

## Requirements

**Functional:**
- Injects Download button on `youtube.com/watch` pages (Shadow DOM)
- Handles YouTube SPA navigation (MutationObserver on title)
- `GM_xmlhttpRequest` for API calls
- Auto-update from `GET /userscript`

**Non-functional:**
- Single new entry point file: `apps/injector/src/userscript.ts`
- `==UserScript==` header prepended to output via Vite plugin or post-process script
- No code duplication with bookmarklet (shared modules)

## Architecture

```
apps/injector/
  └── src/
      ├── bookmarklet.ts       # Phase 1 (unchanged)
      ├── userscript.ts        # NEW — this phase
      └── shared/              # Phase 1 (unchanged, reused)
          ├── inject-button.ts
          ├── quality-picker.ts
          └── stream-utils.ts

dist/
  ├── bm.js                           # Phase 1 output
  └── youtube-downloader.user.js      # This phase output

Backend:
  GET /userscript → dist/youtube-downloader.user.js
```

## Related Code Files

- Create: `apps/injector/src/userscript.ts`
- Modify: `apps/injector/vite.config.ts` — add userscript entry + header plugin
- Verify: `backend/crates/api/src/routes/static_files.rs` — `/userscript` route (Phase 1)

## Implementation Steps

1. **Create `apps/injector/src/userscript.ts`**
   - Import shared modules: `injectButton`, `observeNavigation`
   - Replace `fetch` calls with `GM_xmlhttpRequest` wrapper:
     ```ts
     declare function GM_xmlhttpRequest(details: { method: string; url: string; headers: Record<string, string>; data: string; onload: (r: any) => void; onerror: (e: any) => void }): void
     ```
   - `observeNavigation()`: MutationObserver on `document.querySelector('title')`; debounce 500ms; re-run `injectButton()`
   - Call `injectButton()` on load + `observeNavigation()`

2. **Update `apps/injector/vite.config.ts`**
   - ⚠️ **Do NOT use `vite-plugin-banner`** — ESBuild strips all comments including UserScript header block.
   - Use **`vite-plugin-monkey`** instead: preserves header, auto-configures GM type declarations, supports HMR dev mode in browser:
     ```
     pnpm add -D vite-plugin-monkey
     ```
     Config:
     ```ts
     import monkey from 'vite-plugin-monkey';
     // In vite.config.ts plugins:
     monkey({
       entry: 'src/userscript.ts',
       userscript: {
         name: 'YouTube Downloader',
         namespace: 'https://yourdomain.com',
         version: '1.0.0',
         match: ['https://www.youtube.com/watch*', 'https://youtube.com/watch*'],
         grant: ['GM_xmlhttpRequest'],
         connect: ['yourdomain.com'],
         'run-at': 'document-idle',
         updateURL: 'https://yourdomain.com/userscript',
         downloadURL: 'https://yourdomain.com/userscript',
       },
       build: { fileName: 'youtube-downloader.user.js' },
     })
     ```

3. **Verify `backend/crates/api/src/routes/static_files.rs`**
   - Confirm `GET /userscript` serves `apps/injector/dist/youtube-downloader.user.js`
   - `Content-Type: application/javascript; charset=utf-8`
   - Add route in `main.rs` if Phase 1 left it as placeholder

4. **Add install link to frontend**
   - `apps/web/src/routes/+page.svelte`: "Install UserScript" link → `https://yourdomain.com/userscript`
   - Note: requires Tampermonkey or Greasemonkey

5. **Build and test**
   - `pnpm --filter @downloadtool/injector build`
   - Verify `dist/youtube-downloader.user.js` has `==UserScript==` header block
   - Install Tampermonkey → navigate to `/userscript` → one-click install
   - Test on YouTube watch page: button appears
   - Navigate to another video (SPA) → button re-injects
   - Test auto-update: bump `@version`, redeploy → Tampermonkey detects update

## Todo List

- [ ] Create `apps/injector/src/userscript.ts`
- [ ] Update `apps/injector/vite.config.ts` (userscript entry + header plugin)
- [ ] Verify `/userscript` backend route serves correct file
- [ ] Add "Install UserScript" link to `apps/web` homepage
- [ ] Build, verify `==UserScript==` header in output
- [ ] Test install via Tampermonkey (Chrome + Firefox)
- [ ] Test SPA navigation re-injection
- [ ] Test auto-update flow

## Success Criteria

- `dist/youtube-downloader.user.js` has valid `==UserScript==` header
- Tampermonkey one-click install from hosted URL
- Download button appears and persists through SPA navigation
- Quality picker shows correct streams (no WebM video-only)
- Auto-update works after version bump

## Risk Assessment

| Risk | Likelihood | Mitigation |
|------|-----------|------------|
| `GM_xmlhttpRequest` type declarations missing | Low | Add `declare function` in userscript.ts; or use `@types/greasemonkey` |
| Vite banner plugin incompatibility | Low | Use simple post-build script to prepend header if plugin fails |
| SPA nav observer fires too often | Low | 500ms debounce + idempotency guard in `injectButton()` (shared module) |

## Security Considerations

- `@connect yourdomain.com` limits GM_xmlhttpRequest to own domain only
- `@match` limits to `youtube.com/watch*`
- No user credentials stored

## Next Steps

- All phases complete → announce 4 distribution channels on project homepage
- Optional: submit to Greasy Fork for wider discovery
