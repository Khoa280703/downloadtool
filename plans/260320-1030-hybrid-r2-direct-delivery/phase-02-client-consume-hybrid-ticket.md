# Phase 02: Client Consume Hybrid Ticket

## Context Links
- [plan.md](./plan.md)
- [Phase 01](./phase-01-backend-ticket-hybrid-delivery.md)
- Client API: `frontend/src/lib/api.ts`
- File saver: `frontend/src/lib/playlist-download-file-saver.ts`
- Playlist worker: `frontend/src/lib/playlist-download-worker.ts`
- Single download: `frontend/src/components/DownloadBtn.svelte`
- Playlist page download trigger: `frontend/src/routes/+page.svelte` (line 342)

## Overview
- **Priority**: P1
- **Status**: pending
- **Effort**: 2h

Update client-side code to handle the new ticket response format. When `ticket_delivery=direct`, the `download_url` is an absolute R2 URL. Client must handle this for both single video (DownloadBtn) and playlist (worker) flows, with fallback retry via proxy on failure.

## Key Insights

1. `toAbsoluteDownloadUrl()` in `api.ts:189-196` already returns absolute URLs as-is -- R2 URLs will pass through correctly
2. `fetchMuxJobFileTicket()` in `api.ts:635-649` returns `FileTicketApiResponse` with `download_url` -- needs `ticket_delivery` field added
3. `saveDownload()` in `playlist-download-file-saver.ts` uses `fetch(url)` internally -- works with any URL, but CORS must be configured for R2
4. `downloadViaAnchor()` sets `anchor.href = url` -- works with R2 URL, but cross-origin `download` attribute is ignored by browsers. R2 `Content-Disposition: attachment` header handles this.
5. Playlist worker `createReadyEntry()` gets download URL from `waitForMuxedDownloadJobReady()` which calls `fetchMuxJobFileTicket()` -- change propagates naturally

## Requirements

### Functional
- Client accepts `ticket_delivery` field in file-ticket response
- **CRITICAL: Skip `appendCacheBuster()` for absolute R2 URLs** — signed URL signature covers exact query params; appending `&t=123` invalidates signature → 403. Cache-buster only needed for relative proxy paths.
- Direct R2 URL used **only for FSAA path** (fetch + stream to file) — FSAA has JS error handling for auto-fallback
- **Anchor path always uses proxy URL** — cross-origin anchor `<a>` download provides no JS failure signal, so auto-fallback cannot trigger. Phase 1 must limit direct to FSAA-only.
- On direct download failure (FSAA path), client retries once via proxy URL
- Playlist worker: item-level fallback (failed direct -> retry proxy for that item only, FSAA only)

### Non-functional
- Progress tracking works with direct R2 (requires CORS `Content-Length` exposed -- Phase 3)
- No user-visible change in download UX

## Architecture

```
Single Video (DownloadBtn.svelte):
  1. waitForMuxedDownloadJobReady() returns download URL (may be R2 or proxy)
  2. saveDownload(url, ...) -- fetch or anchor
  3. If fetch fails with network/CORS error AND ticket_delivery was 'direct':
       Retry with proxy URL: /api/proxy/jobs/:id/file

Playlist (playlist-download-worker.ts):
  1. createReadyEntry() -> waitForMuxedDownloadJobReady() -> R2 or proxy URL
  2. saveReadyEntry() calls saveDownload()
  3. On failure, if was direct: retry with proxy fallback URL
```

## Related Code Files

### Modify
- `frontend/src/lib/api.ts` -- extend `FileTicketApiResponse` type, add `ticket_delivery` to return
- `frontend/src/lib/playlist-download-file-saver.ts` -- add fallback retry logic in `saveDownload()`
- `frontend/src/lib/playlist-download-worker.ts` -- pass fallback context through download chain

### No Change
- `frontend/src/components/DownloadBtn.svelte` -- uses `saveDownload()` which handles fallback internally
- `frontend/src/lib/playlist-job-api.ts` -- `resolvePlaylistItemDownloadUrl()` uses `toAbsoluteDownloadUrl()` which already works

## Implementation Steps

### Step 1: Extend `FileTicketApiResponse` in `api.ts`

```typescript
type FileTicketApiResponse = {
  job_id: string;
  download_url: string;
  ticket_delivery?: 'direct' | 'proxy';  // server's decision (client may override)
};
```

Update `pollForMuxedDownloadJobReady()` — **CRITICAL: skip cache-buster for signed R2 URLs**:

```typescript
// In pollForMuxedDownloadJobReady, after fetching ticket:
const ticket = await fetchMuxJobFileTicket(jobPath, cacheBuster, signal);

// CRITICAL: Do NOT append cache-buster to absolute R2 URLs — signed URL
// signature covers exact query params. Adding &t=123 → 403 Forbidden.
// Cache-buster only needed for relative proxy paths to bypass browser cache.
const isAbsoluteUrl = /^https?:\/\//i.test(ticket.download_url);
const downloadUrl = isAbsoluteUrl
  ? ticket.download_url  // R2 signed URL — use as-is
  : toAbsoluteDownloadUrl(appendCacheBuster(ticket.download_url, cacheBuster));
```

**Simpler approach**: Since `waitForMuxedDownloadJobReady` returns a URL string (used everywhere), avoid changing its return type. Instead:

- For mux path: the file-ticket already decided direct vs proxy. If direct fails, client constructs proxy URL from jobId.
- For direct stream path: not affected (uses `/api/stream` which is backend-served).

The fallback logic goes in `saveDownload()`:

```typescript
// In saveDownload(), if fetch fails and URL is cross-origin:
// Derive proxy fallback URL from the current page context
```

### Step 2: Add fallback retry in `playlist-download-file-saver.ts` (FSAA path only)

Add a `fallbackUrl` option to `SaveDownloadOptions`:

```typescript
export interface SaveDownloadOptions {
  requireFsaa?: boolean;
  allowAnchorFallback?: boolean;
  fallbackUrl?: string;  // proxy URL to try if FSAA fetch fails with network/CORS error
  onProgress?: (update: { receivedBytes: number; totalBytes: number | null; percent: number | null }) => void;
}
```

**IMPORTANT**: Fallback retry ONLY works in the FSAA path (`saveWithDirectory()`), because:
- FSAA uses `fetch()` → JS gets error signal → can catch and retry with proxy URL
- Anchor path (`downloadViaAnchor()`) uses `<a href>` click → browser handles download → **no JS error signal** for cross-origin failures
- Therefore: when `ticket_delivery=direct` AND no FSAA, use proxy URL instead of R2 URL
- **Actual delivery mode** is determined by client, not server. Log `actual_delivery` for observability.

In `saveDownload()`, wrap the FSAA attempt with fallback:

```typescript
export async function saveDownload(
  url: string,
  filename: string,
  signal: AbortSignal,
  options: SaveDownloadOptions = {}
): Promise<void> {
  const hasFsaa = !!saveDirectoryHandle;
  const requestedDirect = isCrossOriginUrl(url);

  // CRITICAL: If no FSAA and URL is cross-origin (R2 direct), use fallback proxy URL
  // because anchor downloads can't detect/recover from cross-origin failures.
  const downgraded = !hasFsaa && options.fallbackUrl && requestedDirect;
  const effectiveUrl = downgraded ? options.fallbackUrl! : url;

  // Log actual delivery mode (differs from server's ticket_delivery when downgraded)
  const actualDelivery = downgraded ? 'proxy_downgrade' : (requestedDirect ? 'direct' : 'proxy');
  console.info('[downloadtool] saveDownload', {
    filename, actualDelivery, hasFsaa,
    ticketWasDirect: requestedDirect,
  });

  try {
    await saveDownloadInner(effectiveUrl, filename, signal, options);
  } catch (error) {
    if (isAbortError(error)) throw error;
    // Only fallback if FSAA path failed and we have a fallback URL
    if (hasFsaa && options.fallbackUrl && requestedDirect) {
      console.warn('[downloadtool] direct download failed, falling back to proxy', {
        filename, originalUrl: url, fallbackUrl: options.fallbackUrl, error
      });
      await saveDownloadInner(options.fallbackUrl, filename, signal, {
        ...options,
        fallbackUrl: undefined  // prevent infinite retry
      });
      return;
    }
    throw error;
  }
}

function isCrossOriginUrl(url: string): boolean {
  if (typeof window === 'undefined') return false;
  try {
    const parsed = new URL(url, window.location.origin);
    return parsed.origin !== window.location.origin;
  } catch {
    return false;
  }
}
```

### Step 3: Thread fallback URL through mux download flow

In `api.ts`, update `pollForMuxedDownloadJobReady()` to return both URLs:

```typescript
// New return type for internal use
type MuxDownloadResult = {
  downloadUrl: string;
  proxyFallbackUrl: string | null;
};
```

But to minimize API surface change, keep `waitForMuxedDownloadJobReady` returning string. Instead, create a helper:

```typescript
export function buildMuxProxyFallbackUrl(jobId: string): string {
  return toAbsoluteDownloadUrl(`/api/proxy/jobs/${encodeURIComponent(jobId)}/file`);
}
```

Then in `DownloadBtn.svelte` and `playlist-download-worker.ts`, pass `fallbackUrl` to `saveDownload()`:

```typescript
// DownloadBtn.svelte (mux path):
const fallbackUrl = buildMuxProxyFallbackUrl(created.jobId);
await saveDownload(secureDownloadUrl, filename, controller.signal, {
  ...saveOpts,
  fallbackUrl
});

// playlist-download-worker.ts (saveReadyEntry):
// Need to pass jobId context -- add to ReadyEntry type
```

### Step 4: Update `ReadyEntry` in playlist worker

Add optional `muxJobId` to `ReadyEntry` for constructing fallback URL:

```typescript
type ReadyEntry = {
  entry: QueueEntry;
  downloadUrl: string;
  filename: string;
  muxJobId?: string;  // for proxy fallback
};
```

In `createReadyEntry()`, populate `muxJobId` when using mux path.

In `saveReadyEntry()`, construct fallback URL:

```typescript
async function saveReadyEntry(ready: ReadyEntry, signal: AbortSignal): Promise<void> {
  const fallbackUrl = ready.muxJobId
    ? buildMuxProxyFallbackUrl(ready.muxJobId)
    : undefined;

  await saveDownload(ready.downloadUrl, ready.filename, signal, {
    ...saveOptions,
    fallbackUrl
  });
}
```

### Step 5: Handle playlist backend job items

For playlist items that come with `download_url` from backend SSE (not client-side mux), the `download_url` may also be a direct R2 URL in future. The `+page.svelte` line 342 calls:

```typescript
void saveDownload(downloadUrl, filename, signal, { onProgress: ... });
```

Add fallback URL here too:

```typescript
const item = ...; // playlist item
const fallbackUrl = item.mux_job_id
  ? buildMuxProxyFallbackUrl(item.mux_job_id)
  : undefined;
void saveDownload(downloadUrl, filename, signal, { onProgress: ..., fallbackUrl });
```

## Todo List
- [ ] Extend `FileTicketApiResponse` with optional `ticket_delivery` field
- [ ] Add `fallbackUrl` to `SaveDownloadOptions`
- [ ] Implement fallback retry in `saveDownload()` for cross-origin failures
- [ ] Add `isCrossOriginUrl()` helper
- [ ] Add `buildMuxProxyFallbackUrl()` to `api.ts`
- [ ] Update `DownloadBtn.svelte` to pass `fallbackUrl` in mux path
- [ ] Add `muxJobId` to playlist worker `ReadyEntry`
- [ ] Update `saveReadyEntry()` to use fallback URL
- [ ] Update `+page.svelte` playlist item download to include fallback URL
- [ ] Console log when fallback triggered (for observability)

## Success Criteria
- Single video mux download works with R2 direct URL
- Single video mux download falls back to proxy on CORS/network failure
- Playlist mux downloads work with R2 direct URLs
- Playlist items individually fall back to proxy on failure
- Direct stream downloads (non-mux) unaffected
- No regression in FSAA progress tracking (depends on Phase 3 CORS)

## Risk Assessment
- **CORS not configured yet**: Until Phase 3, direct R2 fetch() will fail. But `DOWNLOAD_DELIVERY_MODE` defaults to `hybrid` and Rust backend `MUX_DIRECT_DOWNLOAD` must also be true. Safe to deploy code first.
- **Anchor path always uses proxy**: Cross-origin anchor downloads have no JS error signal. When `ticket_delivery=direct` but no FSAA, `saveDownload()` automatically uses `fallbackUrl` (proxy) instead. This eliminates anchor+direct risk entirely.
- **Double download on fallback**: Fallback only triggers on FSAA fetch failure (network/CORS error), not on successful-but-wrong response. No risk of double download.
- **Cache-buster on signed URLs**: `appendCacheBuster()` skipped for absolute R2 URLs. Verified on production: signed URL + `&t=123` → 403.

## Security Considerations
- Fallback URL is always same-origin proxy path -- no new attack surface
- R2 signed URL exposure in browser is acceptable (short TTL, read-only, single object)
