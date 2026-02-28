---
title: "UX — YouTube Thumbnail Prefetch During Extract"
description: "Show YouTube thumbnail immediately in skeleton loader while yt-dlp subprocess runs (~2s)"
status: pending
priority: P2
effort: 45min
branch: main
tags: [frontend, ux, svelte, performance]
created: 2026-02-28
---

# UX — YouTube Thumbnail Prefetch During Extract

## Context

After switching from Deno V8 in-process extractor to yt-dlp subprocess, first extract takes ~2s (Python cold start). Skeleton loader **already exists** in `+page.svelte` (lines ~391–418) with `animate-pulse`. The blank thumbnail slot is `bg-slate-200`.

Review note: implementation must avoid regressions in URL parsing behavior, add image-error fallback, and avoid adding more parsing logic into already-large `+page.svelte`.

## Scope

**2 files, 5 changes**:
1. Add strict YouTube video ID parser in `frontend/src/lib/api.ts` (shared helper, not in page)
2. Expand and align frontend URL validation with YouTube subdomains (`youtube.com`, `*.youtube.com`, `youtu.be`, `www.youtu.be`)
3. Add derived thumbnail URL state in `frontend/src/routes/+page.svelte`
4. Replace skeleton thumbnail block with image + spinner + robust fallback when image fails
5. Add explicit validation + manual test checklist in this plan

No backend/Rust changes.

## Implementation

### Step 1 — Expand validator/getPlatform in `frontend/src/lib/api.ts`

Update `isValidVideoUrl` and `getPlatform` to use URL parsing (instead of narrow regex), and introduce shared host helper:

```typescript
function isYouTubeHost(hostname: string): boolean {
  const host = hostname.toLowerCase().replace(/^www\./, '');
  return host === 'youtube.com' || host.endsWith('.youtube.com') || host === 'youtu.be';
}

export function isValidVideoUrl(url: string): boolean {
  try {
    const u = new URL(url.trim());
    if (!(u.protocol === 'http:' || u.protocol === 'https:')) return false;
    if (!isYouTubeHost(u.hostname)) return false;

    // Reject bare root URLs like https://youtube.com/
    const hasMeaningfulPath = u.pathname && u.pathname !== '/';
    const hasVideoOrPlaylistParam = u.searchParams.has('v') || u.searchParams.has('list');
    return hasMeaningfulPath || hasVideoOrPlaylistParam;
  } catch {
    return false;
  }
}

export function getPlatform(url: string): 'youtube' | 'unknown' {
  return isValidVideoUrl(url) ? 'youtube' : 'unknown';
}
```

Why:
- Unblocks `m.youtube.com` and other valid YouTube subdomains
- Prevents checklist/validation mismatch
- Keeps URL rules in one consistent path

### Step 1.1 — Add shared ID parser in `frontend/src/lib/api.ts`

Add exported helper near other URL helpers (reusing `isYouTubeHost` from Step 1):

```typescript
export function extractYouTubeVideoId(url: string): string | null {
  try {
    const u = new URL(url.trim());
    if (!isYouTubeHost(u.hostname)) return null;
    const host = u.hostname.toLowerCase().replace(/^www\./, '');

    const isValidId = (id: string | null): id is string => !!id && /^[A-Za-z0-9_-]{11}$/.test(id);

    if (host === 'youtu.be') {
      const firstSegment = u.pathname.split('/').filter(Boolean)[0] ?? null;
      return isValidId(firstSegment) ? firstSegment : null;
    }

    const v = u.searchParams.get('v');
    if (isValidId(v)) return v;

    const shortsMatch = u.pathname.match(/^\/shorts\/([A-Za-z0-9_-]{11})(?:[/?#]|$)/);
    if (shortsMatch) return shortsMatch[1];

    const embedMatch = u.pathname.match(/^\/embed\/([A-Za-z0-9_-]{11})(?:[/?#]|$)/);
    if (embedMatch) return embedMatch[1];

    return null;
  } catch {
    return null;
  }
}
```

Why:
- Fix host mismatch (`www.youtu.be` case)
- Enforce strict 11-char YouTube ID regex
- Keep parsing logic out of `+page.svelte`
- Reuse host rules from a single helper (`isYouTubeHost`)

### Step 2 — Use helper in `frontend/src/routes/+page.svelte`

Import helper:

```typescript
import { extract, isValidVideoUrl, extractYouTubeVideoId } from '$lib/api';
```

Add derived + state below existing state declarations:

```typescript
let previewThumbnailId = $derived(
  isExtracting ? extractYouTubeVideoId(inputUrl) : null
);
let previewThumbnailLoadFailed = $state(false);
let previewThumbnailUrl = $derived(
  previewThumbnailId ? `https://i.ytimg.com/vi/${previewThumbnailId}/hqdefault.jpg` : null
);
```

Reset image error state at extract start:

```typescript
isExtracting = true;
previewThumbnailLoadFailed = false;
```

### Step 3 — Replace skeleton thumbnail block with robust fallback

Current skeleton thumbnail (line ~396):

```svelte
<div class="w-full aspect-video rounded-3xl bg-slate-200"></div>
```

Replace with:

```svelte
<div class="relative w-full aspect-video rounded-3xl overflow-hidden bg-slate-200">
  {#if previewThumbnailUrl && !previewThumbnailLoadFailed}
    <img
      class="absolute inset-0 w-full h-full object-cover"
      src={previewThumbnailUrl}
      alt="YouTube thumbnail preview"
      decoding="async"
      onerror={() => (previewThumbnailLoadFailed = true)}
    />
  {:else}
    <div class="absolute inset-0 grid place-items-center text-slate-400">
      <span class="material-symbols-outlined text-6xl">movie</span>
    </div>
  {/if}
  <div class="absolute inset-0 bg-gradient-to-t from-black/40 to-transparent"></div>
  <div class="absolute bottom-4 right-4 bg-black/60 backdrop-blur-md px-3 py-1.5 rounded-full text-xs font-bold text-white border border-white/20 flex items-center gap-1">
    <span class="material-symbols-outlined animate-spin text-sm">progress_activity</span>
    Fetching...
  </div>
</div>
```

### Step 4 — Validation and checks

Run:

```bash
pnpm --filter frontend check
```

Manual acceptance checklist:
- `https://www.youtube.com/watch?v=dQw4w9WgXcQ` shows thumbnail during extracting
- `https://m.youtube.com/watch?v=dQw4w9WgXcQ` shows thumbnail
- `https://music.youtube.com/watch?v=dQw4w9WgXcQ` shows thumbnail
- `https://youtu.be/dQw4w9WgXcQ` and `https://www.youtu.be/dQw4w9WgXcQ` show thumbnail
- `https://www.youtube.com/shorts/dQw4w9WgXcQ` shows thumbnail
- Invalid URL still shows existing validation error (no JS error)
- If thumbnail request fails, UI falls back to placeholder icon + spinner (no broken image icon)

### Step 5 — Non-goals / constraints

- Do not change backend extract behavior
- Do not change stream selection logic
- Do not add Rust changes

## Result

| | Before | After |
|--|--|--|
| First request UX | Blank skeleton 2s | Thumbnail appears instantly, spinner overlay |
| Broken image case | Possible broken image | Fallback placeholder icon |
| URL handling | Narrow regex + partial plan | Expanded subdomain support + strict ID regex |
| Code organization | Parsing in page | Parsing shared in `src/lib` |
| Code changes | — | 2 frontend files |
| Backend changes | None | None |

## Files

### Modify
- `frontend/src/lib/api.ts` — add `extractYouTubeVideoId` helper
- `frontend/src/routes/+page.svelte` — derive preview state + fallback UI in skeleton thumbnail

### Do NOT touch
- `frontend/src/lib/types.ts`
- Any Rust files
