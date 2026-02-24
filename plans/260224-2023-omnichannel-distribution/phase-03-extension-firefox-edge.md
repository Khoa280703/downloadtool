# Phase 03: Extension — Firefox + Edge (apps/extension)

## Context Links
- Parent plan: [plan.md](./plan.md)
- Depends on: [phase-00-monorepo-setup-api-client.md](./phase-00-monorepo-setup-api-client.md)
- API: `POST /api/extract`, `GET /api/stream/muxed`

## Overview

| Field | Value |
|-------|-------|
| Priority | P2 |
| Status | ✅ completed |
| Description | Vite + plain MV3 extension — content script with Shadow DOM injects Download button; background uses chrome.downloads; builds 2 zips (Firefox + Edge) |
| Effort | 2d |
| Blocked by | Phase 0 complete |

## Key Insights

- NO crxjs — too fragile, version-locked; use plain Vite with IIFE output for content script
- Shadow DOM is REQUIRED for content script CSS isolation on YouTube
- `chrome.downloads` API > `window.open` — native download dialog, no popup blocker issues
- Shared source, 2 manifests: `manifest-firefox.json` + `manifest-edge.json`
- Firefox: `background.scripts[]` array + `browser_specific_settings.gecko.id`
- Edge: `background.service_worker` string, no gecko settings
- `build-extension.sh` outputs 2 zips from same source
- Use `@downloadtool/api-client` for type-safe API calls

## Requirements

**Functional:**
- Content script injects Download button next to Like/Share on YouTube watch pages
- Shadow DOM wraps all injected UI (CSS isolation)
- Popup auto-detects YouTube tab, shows stream quality picker
- Download via `chrome.downloads.download()` (background service worker)
- Handles YouTube SPA navigation (MutationObserver)

**Non-functional:**
- Permissions: `activeTab`, `tabs`, `downloads`
- No crxjs — plain Vite build
- Build script outputs: `dist/youtube-downloader-firefox.zip` + `dist/youtube-downloader-edge.zip`

## Architecture

```
apps/extension/
  ├── src/
  │   ├── content-script.ts     # Injected into YouTube — Shadow DOM button
  │   ├── background.ts         # Service worker — chrome.downloads API
  │   ├── popup/
  │   │   ├── popup.html
  │   │   ├── popup.ts
  │   │   └── popup.css
  │   └── shared/
  │       └── stream-utils.ts   # Filter + URL builder (mirrors injector shared)
  ├── manifest-firefox.json
  ├── manifest-edge.json
  ├── icons/
  │   ├── icon-16.png
  │   ├── icon-48.png
  │   └── icon-128.png
  ├── vite.config.ts
  ├── build-extension.sh        # Outputs 2 zips
  └── package.json
```

## Related Code Files

- Create: `apps/extension/` (all files above)
- No backend changes needed (CORS already permissive)

## Implementation Steps

1. **Create `apps/extension/package.json`**
   - `name`: `@downloadtool/extension`
   - `devDependencies`: `vite`, `typescript`, `@downloadtool/api-client`
   - `scripts.build`: `vite build && bash build-extension.sh`

2. **Create `apps/extension/vite.config.ts`**
   - Multiple entry points: `content-script` → IIFE, `background` → IIFE, `popup/popup` → ES module
   - `build.outDir`: `dist/unpacked`
   - Copy manifest and icons to `dist/unpacked/` via `vite-plugin-static-copy` or manual copy in build script

3. **Create `apps/extension/src/content-script.ts`**
   - `MutationObserver` on `document.title` for SPA nav detection (debounced 500ms)
   - `injectButton()`: find YouTube actions container; create Shadow DOM host (`mode: 'open'`)
   - Inside shadow root: Download button + scoped CSS
   - On button click: message background via `chrome.runtime.sendMessage({type: 'EXTRACT', url})`

4. **Create `apps/extension/src/background.ts`**
   - Listen for `chrome.runtime.onMessage`
   - On `EXTRACT`: call `POST /api/extract` via api-client → send streams back to content script
   - On `DOWNLOAD`: call `chrome.downloads.download({url: muxedUrl, filename: title + '.mp4'})`

5. **Create `apps/extension/src/popup/popup.ts`**
   - Query active tab URL: `chrome.tabs.query({active: true, currentWindow: true})`
   - If YouTube watch URL: call `POST /api/extract` → render stream list
   - On stream select: send `DOWNLOAD` message to background
   - Handle loading/error states

6. **Create manifests**
   - `manifest-firefox.json`:
     - `background.scripts: ["background.js"]`
     - `browser_specific_settings.gecko.id`: `"ytdl@downloadtool"`
   - `manifest-edge.json`:
     - `background.service_worker: "background.js"`
     - No gecko settings

7. **Create `apps/extension/build-extension.sh`**
   - Copy `dist/unpacked/` to staging dir
   - Firefox zip: copy `manifest-firefox.json` as `manifest.json` → zip → `dist/youtube-downloader-firefox.zip`
   - Edge zip: copy `manifest-edge.json` as `manifest.json` → zip → `dist/youtube-downloader-edge.zip`

8. **Test Firefox**
   - `about:debugging` → Load Temporary Add-on → `dist/unpacked/manifest-firefox.json`
   - Navigate to YouTube watch → verify button in Shadow DOM

9. **Test Edge**
   - `edge://extensions/` → Developer mode → Load unpacked → `dist/unpacked/` (with `manifest-edge.json` renamed)
   - Verify identical behavior

## Todo List

- [ ] Create `apps/extension/package.json` + `vite.config.ts`
- [ ] Create `src/content-script.ts` (Shadow DOM injection + SPA nav)
- [ ] Create `src/background.ts` (chrome.downloads)
- [ ] Create `src/popup/popup.ts` + `popup.html` + `popup.css`
- [ ] Create `manifest-firefox.json` + `manifest-edge.json`
- [ ] Create `icons/` (16, 48, 128px PNGs)
- [ ] Create `build-extension.sh`
- [ ] Test in Firefox via `about:debugging`
- [ ] Test in Edge developer mode
- [ ] Run build script, verify 2 zips produced

## Success Criteria

- Content script injects Download button via Shadow DOM on YouTube watch pages
- Button persists through SPA navigation
- `chrome.downloads` triggers native download dialog
- `build-extension.sh` produces 2 valid, distinct zips
- Both Firefox and Edge load extension without errors

## Risk Assessment

| Risk | Likelihood | Mitigation |
|------|-----------|------------|
| YouTube DOM changes break content script selectors | Medium | Multiple fallback selectors; MutationObserver is stable |
| Firefox/Edge manifest format divergence grows | Low | Keep diffs minimal; document in comment block |
| `chrome.downloads` requires extra manifest permission | Low | Add `"downloads"` to permissions in both manifests |
| Vite IIFE for content script needs polyfill check | Low | Test in both browsers; avoid ESM-only APIs |

## Security Considerations

- `activeTab` + `tabs` + `downloads` only — no broad host permissions
- API base URL hardcoded (no user input) — prevents SSRF
- Shadow DOM prevents XSS via YouTube page CSS/JS

## Next Steps

- Package and submit Firefox zip to AMO
- Package and submit Edge zip to Edge Add-ons partner dashboard
