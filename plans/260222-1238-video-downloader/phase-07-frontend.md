# Phase 07 — Frontend (Svelte + Cloudflare Pages)

## Context
- Plan: [plan.md](plan.md)
- Prev: [phase-03-stream-proxy.md](phase-03-stream-proxy.md) (API ready)
- Next: [phase-08-ad-integration.md](phase-08-ad-integration.md)

## Overview
- **Priority**: P1
- **Status**: pending
- **Effort**: 2d
- Minimal, fast Svelte SPA. Single URL input → download dialog. Mobile-first. Hosted free on Cloudflare Pages.

## Key Insights
- Svelte compiles to vanilla JS → smallest bundle (no virtual DOM overhead)
- Trigger browser download dialog: `Content-Disposition: attachment` from /api/stream → `<a href="/api/stream?...">` click
- No file save on server → no progress bar needed; loading spinner until first byte
- SEO matters for ad revenue → SSR or at minimum prerendered landing page

## Architecture

<!-- Updated: Validation Session 1 - Added SSE batch UI + browser download pool -->

```
frontend/
├── src/
│   ├── App.svelte
│   ├── components/
│   │   ├── UrlInput.svelte       # single URL + paste button
│   │   ├── BatchInput.svelte     # channel/playlist URL input  ← NEW
│   │   ├── FormatPicker.svelte   # quality/format selector
│   │   ├── DownloadBtn.svelte    # triggers /api/stream
│   │   ├── BatchProgress.svelte  # SSE progress bar + queue  ← NEW
│   │   └── AdBanner.svelte
│   ├── lib/
│   │   ├── api.ts                # typed API client
│   │   └── download-pool.ts     # browser download pool (3 concurrent)  ← NEW
│   └── stores/
│       ├── download.ts
│       └── batch.ts              # batch queue state  ← NEW
└── ...
```

**Batch download flow (browser-side):**
```
BatchInput → POST /api/batch → SSE stream of {url, title, index, total}
    → BatchProgress shows progress bar
    → download-pool.ts: maintains 3 concurrent <a download> triggers
    → each slot: wait for previous to start, trigger next
```

## Implementation Steps

1. **Init project**
   ```bash
   npm create svelte@latest frontend  # choose: SvelteKit minimal
   cd frontend && npm install
   ```

2. **SvelteKit config** — static adapter for Cloudflare Pages
   ```bash
   npm i -D @sveltejs/adapter-cloudflare
   ```
   ```js
   // svelte.config.js
   import adapter from '@sveltejs/adapter-cloudflare';
   export default { kit: { adapter: adapter() } };
   ```

3. **API client** (`src/lib/api.ts`)
   ```typescript
   const API_BASE = import.meta.env.VITE_API_URL;  // set in CF Pages env

   export async function extract(url: string): Promise<ExtractResult>
   export function buildStreamUrl(streamUrl: string, title: string, format: string): string
   // buildStreamUrl → `${API_BASE}/api/stream?url=...&title=...`
   ```

4. **UrlInput.svelte**
   - Text input + "Paste" button (navigator.clipboard.readText)
   - Validate: regex for youtube.com/youtu.be/tiktok.com URLs
   - On submit → call `extract()` → show FormatPicker

5. **FormatPicker.svelte**
   - Show available streams: quality badges (4K, 1080p, 720p, MP3)
   - Default: highest quality video + audio
   - Toggle: "Remove watermark" (TikTok), "Add branding" (GPU feature)

6. **DownloadBtn.svelte**
   - Creates `<a>` tag with `/api/stream?...` href, `download` attribute
   - `.click()` → browser opens download dialog immediately
   - Loading state: spinner while waiting for first byte (fetch HEAD first)
   - No progress bar (stream is zero-storage, progress unknowable for muxed)

7. **Landing page SEO** (`src/routes/+page.svelte`)
   - H1: "Download TikTok & YouTube Videos Free"
   - Meta description, OG tags
   - Structured data (WebApplication schema)
   - Prerendered: `export const prerender = true`

8. **Cloudflare Pages deployment**
   - Build command: `npm run build`
   - Output dir: `.svelte-kit/cloudflare`
   - Env var: `VITE_API_URL=https://api.yourdomain.com`
   - `_headers` file: `Content-Security-Policy`, `X-Frame-Options`

9. **Responsive design** — mobile-first
   - Single column layout, large input, big download button
   - Touch-friendly: min 44px tap targets
   - Dark mode via `prefers-color-scheme`

## Todo
- [ ] SvelteKit + Cloudflare adapter setup
- [ ] API client (api.ts) with typed responses
- [ ] UrlInput component with validation
- [ ] BatchInput component (channel/playlist URL)
- [ ] FormatPicker component
- [ ] DownloadBtn with download attribute trigger
- [ ] download-pool.ts (3 concurrent browser downloads, EventSource listener)
- [ ] BatchProgress component (SSE progress bar, queue display)
- [ ] batch.ts store (queue state management)
- [ ] Landing page with SEO meta tags
- [ ] Mobile-responsive CSS
- [ ] Cloudflare Pages deployment config
- [ ] E2E test: paste URL → see format options → click download → file starts
- [ ] E2E test: paste channel URL → SSE progress → 3 downloads start concurrently

## Success Criteria
- Lighthouse score ≥90 (performance, SEO, accessibility)
- LCP <1.5s on mobile 4G
- Download dialog opens within 500ms of button click
- Works on iOS Safari, Chrome Android, desktop browsers

## Risk Assessment
| Risk | Mitigation |
|---|---|
| CORS errors from CF Pages to API | API CORS headers allow CF Pages origin (Phase 03) |
| iOS Safari blocks programmatic download | Use direct `<a download>` link, not fetch |
| API URL changes per environment | VITE_API_URL env var in CF Pages dashboard |
