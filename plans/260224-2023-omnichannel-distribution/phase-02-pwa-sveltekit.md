# Phase 02: PWA (apps/web — SvelteKit)

## Context Links
- Parent plan: [plan.md](./plan.md)
- Depends on: [phase-00-monorepo-setup-api-client.md](./phase-00-monorepo-setup-api-client.md)

## Overview

| Field | Value |
|-------|-------|
| Priority | P1 |
| Status | ✅ completed |
| Description | SvelteKit app with PWA features: Web Share Target, Background Fetch, clipboard auto-read, install prompt |
| Effort | 1.5d |
| Blocked by | Phase 0 complete |

## Key Insights

- `apps/web/` = new SvelteKit app (replaces/supersedes old `frontend/` if migrating, or is standalone)
- Web Share Target API (Android): register in `manifest.json` as POST share target — YouTube app "Share" button sends URL directly to PWA
- Background Fetch API: download continues even if user closes PWA (critical for mobile UX)
- `visibilitychange` clipboard auto-read: pre-fills URL input when user returns to tab
- `beforeinstallprompt`: capture event, show custom Install button
- File System Access API: deferred — Chrome-only, YAGNI
- Service worker: `src/service-worker.ts` (SvelteKit built-in, auto-registered)

## Requirements

**Functional:**
- Installable on Android (Chrome) and desktop (Chrome, Edge)
- Web Share Target: Android "Share" from YouTube → opens PWA with URL pre-filled
- Background Fetch: download survives PWA being closed/backgrounded
- Clipboard auto-read on tab focus → pre-fill URL input
- Offline fallback page
- "Install App" button when browser supports it

**Non-functional:**
- No new npm packages beyond SvelteKit + vite-plugin-pwa (for manifest/SW generation)
- Service worker < 120 LOC
- `apps/web/` is a standalone pnpm workspace package

## Architecture

```
apps/web/
  ├── src/
  │   ├── routes/
  │   │   ├── +layout.svelte       # SW register, install prompt, clipboard listener
  │   │   ├── +page.svelte         # Main download UI
  │   │   └── share-target/
  │   │       └── +server.ts       # POST /share-target → redirect with url param
  │   ├── service-worker.ts        # Background Fetch + offline cache
  │   └── app.html                 # manifest link, meta tags
  └── static/
      ├── manifest.json            # Web Share Target registration
      └── offline.html
```

## Related Code Files

- Create: `apps/web/` (full SvelteKit scaffold)
- Create: `apps/web/static/manifest.json`
- Create: `apps/web/static/offline.html`
- Create: `apps/web/src/service-worker.ts`
- Create: `apps/web/src/routes/share-target/+server.ts`
- Modify: `apps/web/src/app.html`
- Modify: `apps/web/src/routes/+layout.svelte`
- Modify: `apps/web/src/routes/+page.svelte`

## Implementation Steps

1. **Scaffold `apps/web/`**
   - `pnpm create svelte@latest apps/web` (skeleton, TypeScript, no extras)
   - Add `@downloadtool/api-client` as workspace dep
   - Update `apps/web/package.json` name to `@downloadtool/web`

2. **Create `apps/web/static/manifest.json`**
   - `name`, `short_name`, `start_url: "/"`, `display: "standalone"`
   - `share_target`:
     ```json
     {
       "action": "/share-target",
       "method": "POST",
       "enctype": "application/x-www-form-urlencoded",
       "params": { "text": "url" }
     }
     ```
   - `icons`: 192x192 and 512x512 PNGs

3. **Create `apps/web/src/routes/share-target/+server.ts`**
   - `POST /share-target`: read `url` from form body → `redirect(302, `/?url=${encodeURIComponent(url)}`)`
   - Handles Android YouTube "Share" → PWA flow
   - ⚠️ **CSRF fix required**: SvelteKit blocks POST from native Android app (origin=null → 403). Add to `apps/web/svelte.config.js`:
     ```js
     kit: { csrf: { checkOrigin: false } }
     ```
     Or handle selectively in `hooks.server.ts` for `/share-target` route only.

4. **Create `apps/web/src/service-worker.ts`**
   - On `install`: cache `/offline.html`
   - On `fetch`: network-first; serve `/offline.html` on navigation failure
   - Background Fetch: register fetch via `registration.backgroundFetch.fetch(...)` on download trigger
   - Listen for `backgroundfetchsuccess` → notify user, save file

5. **Modify `apps/web/src/app.html`**
   - Add `<link rel="manifest" href="/manifest.json">`
   - Add `<meta name="theme-color" content="#...">`

6. **Modify `apps/web/src/routes/+layout.svelte`**
   - `onMount`:
     - Register SW: `navigator.serviceWorker.register('/service-worker.js')`
     - Capture `beforeinstallprompt` → store, show Install button
     - Add `visibilitychange` listener → `navigator.clipboard.readText()` → detect YouTube URL → dispatch `yturl-detected`
   - Install button: `deferredPrompt.prompt()` on click

7. **Modify `apps/web/src/routes/+page.svelte`**
   - Read `?url=` query param on load (from share target redirect)
   - Listen for `yturl-detected` custom event
   - Pre-fill URL input from either source
   - Use `@downloadtool/api-client` for `POST /api/extract`
   - Trigger download via Background Fetch API (with fallback to `window.open`)

8. **Create `apps/web/static/offline.html`**
   - Minimal self-contained HTML: "You are offline."

## Todo List

- [ ] Scaffold `apps/web/` with SvelteKit
- [ ] Create `static/manifest.json` with Web Share Target
- [ ] Create `share-target/+server.ts` POST handler
- [ ] Create `service-worker.ts` (offline + Background Fetch)
- [ ] Update `app.html` with manifest link + meta tags
- [ ] Update `+layout.svelte` (SW register, install prompt, clipboard)
- [ ] Update `+page.svelte` (share target param, api-client usage)
- [ ] Test Web Share Target on Android Chrome
- [ ] Test Background Fetch (close PWA mid-download)
- [ ] Test install prompt in Chrome DevTools

## Success Criteria

- Lighthouse PWA score ≥ 90
- Android: "Share" from YouTube app → PWA opens with URL pre-filled
- Download continues after PWA is backgrounded/closed (Background Fetch)
- Clipboard YouTube URL auto-detected on tab focus
- Offline page shown when disconnected

## Risk Assessment

| Risk | Likelihood | Mitigation |
|------|-----------|------------|
| Background Fetch API browser support limited | Medium | Fallback to `window.open` for unsupported browsers; feature-detect |
| Web Share Target requires HTTPS + installed PWA | Low | Production is HTTPS; document install requirement |
| SW breaks SvelteKit HMR in dev | Medium | Register SW only in `import.meta.env.PROD` |

## Next Steps

- Phase 3: Extension (independent, parallel)
- Phase 4: UserScript (independent, parallel)
