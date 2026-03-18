# Code Review: Playlist Backend Orchestration Phase 4 Follow-up

## Scope
- **Files reviewed**: ~20 files (backend + frontend)
- **LOC changed**: ~1,500+ (excluding deploy-log.md)
- **Focus**: New playlist job system (backend orchestration, crash recovery, admin dashboard, SSE, localStorage persistence)

## Overall Assessment
Solid implementation moving from client-side playlist processing to server-side orchestration. Good patterns: parameterized SQL, transactional bulk inserts, proper auth scoping, crash recovery. Several issues need attention before production.

---

## Critical Issues

### C1. Admin Playlists page references `data.overview` but page server doesn't return it
**File**: `frontend/src/routes/(auth)/admin/playlists/+page.server.ts` (line 5-9)
**Impact**: Page will crash at runtime. Template reads `data.overview.playlistActiveJobs` etc but load returns only `{ playlistJobs }`.

**Fix**:
```ts
import { loadAdminPlaylistJobs, loadAdminOverview } from '$lib/server/admin-dashboard';

export const load: PageServerLoad = async () => {
    const [playlistJobs, overview] = await Promise.all([
        loadAdminPlaylistJobs(),
        loadAdminOverview()
    ]);
    return { playlistJobs, overview };
};
```

### C2. SSE proxy route missing `signal: request.signal` -- resource leak
**File**: `frontend/src/routes/api/proxy/playlist-jobs/[jobId]/events/+server.ts` (line 12-16)
**Impact**: When client disconnects, upstream SSE connection stays open indefinitely. The polling loop in backend continues wasting DB queries.

**Fix**: Add `signal: request.signal` to the fetch call:
```ts
const upstream = await fetch(
    buildRustApiUrl(`/api/playlist-jobs/${encodeURIComponent(params.jobId)}/events`),
    {
        headers: await buildRustApiHeaders(request, false, downloadSessionId),
        signal: request.signal
    }
);
```

### C3. `get_job(id, None, None)` bypasses owner authorization
**Files**: `playlist_processor.rs` (lines 87, 143), `playlist_jobs.rs` (line 208)
**Impact**: The SQL condition `($2 IS NULL AND $3 IS NULL)` matches ANY job when both user_id and session_id are NULL. Internal server-side calls use `None, None` which is fine for the processor, but in the SSE polling loop (`playlist_jobs.rs:208`) this runs inside a user-facing handler. After the initial auth check, the poll loop uses `None, None`, meaning the SSE stream continues even if ownership changes or job is reassigned.

**Risk**: Low in practice (job IDs are unpredictable), but violates principle of least privilege. Consider adding a `get_job_internal()` method without the owner filter for server-side use, keeping `get_job()` always owner-scoped.

---

## High Priority

### H1. No concurrency limit on `tokio::spawn` for playlist processors
**File**: `playlist_processor.rs` (line 55), `playlist_jobs.rs` (line 125)
**Impact**: Each playlist job spawns an unbounded task. A burst of playlist job creates (or crash recovery with many orphaned jobs) could overwhelm the server with concurrent yt-dlp processes.

**Recommendation**: Use a `tokio::sync::Semaphore` to cap concurrent playlist processors (e.g., 3-5 max).

### H2. `new AbortController().signal` leaked per auto-save download
**File**: `+page.svelte` (line ~388)
```ts
void saveDownload(downloadUrl, `${item.title ?? item.video_id}.mp4`, new AbortController().signal)
```
**Impact**: Each download creates an AbortController that is never aborted on page unload/cancel. The AbortControllers are unreachable after creation. When user cancels playlist or navigates away, downloads continue.

**Fix**: Store controllers in a Map keyed by video_id, abort all on `resetPlaylistComposer()` or `onDestroy`.

### H3. `mapBackendItemStatus` maps 'ready' to 'completed' but auto-save triggers on 'ready'
**File**: `+page.svelte` (lines 366-390, 405-417)
**Impact**: When `handlePlaylistSnapshot` runs, it first calls `addBatchItem` with status 'completed' (mapped from 'ready'), then in the auto-save loop checks `item.status !== 'ready'`. This works but causes a visual flash: item shows "completed" before download actually starts, then gets overridden to "downloading" by `updateBatchItemByVideoId`.

**Recommendation**: Map 'ready' to 'downloading' instead of 'completed', since the auto-save hasn't happened yet.

### H4. `handlePlaylistSnapshot` calls `addBatchItem` on every SSE event for all items
**File**: `+page.svelte` (lines 362-377)
**Impact**: Every 2-second poll re-adds all items. `addBatchItem` likely deduplicates by videoId, but calling `updateBatchItemByVideoId` only for non-pending/completed/error statuses means pending items that transition to pending again (after crash recovery reset) won't re-render.

---

## Medium Priority

### M1. `+page.svelte` is 1,622 lines -- well above 200-line guideline
**File**: `frontend/src/routes/+page.svelte`
**Impact**: Maintainability. The playlist job logic (handlePlaylistSnapshot, resumePlaylistJob, mapBackendItemStatus, persistence helpers) could be extracted to a dedicated module like `$lib/playlist-job-orchestrator.ts`.

### M2. `from_str` silently defaults to `Queued`/`Pending`
**Files**: `playlist_job_models.rs` (lines 26-35, 69-79)
**Impact**: Unknown status strings from DB are silently treated as Queued/Pending. Should log a warning for unexpected values.

### M3. Playlist overview SQL not resilient to missing `updated_at` column
**File**: `admin-dashboard.ts` (playlistOverviewQuery)
The query references `updated_at` for 24h filtering but the CREATE TABLE uses `updated_at_ms`. The `safe` wrapper catches table-not-found but won't catch column mismatch at this level.

### M4. `rand_jitter` is not cryptographically random
**File**: `playlist_processor.rs` (lines 408-413)
Uses `SystemTime` hash for jitter -- acceptable for delay jitter but worth noting it's deterministic within the same millisecond.

### M5. SEO_ORIGIN domain changed from `download.khoadangbui.online` to `snapvie.com`
**File**: `+page.svelte` -- this is a domain migration change bundled into the playlist PR. Should be a separate commit.

---

## Low Priority

### L1. `PlaylistJobRow` fields in `JobsApiError` made pub
**File**: `jobs.rs` (lines 27-29) -- `message`, `status`, `retry_after_secs` changed to `pub` for reuse by `playlist_jobs.rs`. Fine but consider extracting to shared module.

### L2. Admin playlist page pagination is client-side only
**File**: `admin/playlists/+page.svelte` -- loads all 50 jobs, paginates in browser. With LIMIT 50 in query, this is fine. But no server-side pagination for future growth.

---

## Edge Cases Found by Scout

1. **Crash during discovery phase**: If server crashes between `set_discovery_result` and `insert_items`, the job shows total_items > 0 but has 0 items in DB. Recovery re-spawns but `claim_next_pending_item` returns None immediately, marking job "completed" with 0 actual completions.

2. **Duplicate playlist job creation**: No deduplication check. Same URL can spawn multiple concurrent processors for the same playlist.

3. **Mux job timeout (5min) vs large playlists**: A playlist with many mux items processes sequentially. If each mux takes close to 5 minutes, a 50-item playlist could take ~4 hours. No overall playlist timeout exists.

4. **localStorage stale job ID**: If server DB is wiped but localStorage retains old job ID, `resumePlaylistJob` catches the error and clears storage -- good. But there's a small window where the UI briefly shows playlist mode before the fetch fails.

5. **SSE polling interval mismatch**: Backend polls DB every 2 seconds. For a playlist with 100 items, that's 100 items * 15 columns every 2 seconds. Consider sending only changed items or using a last_updated_at filter.

---

## Positive Observations

- Transactional bulk insert for playlist items (`insert_items`) -- good data integrity
- Proper `SELECT FOR UPDATE SKIP LOCKED` pattern for item claiming -- safe for concurrent processors
- `safeLoadPlaylistOverview()` with `42P01` error code check -- graceful table-not-exists handling
- Rate limiting applied to playlist job creation route -- prevents abuse
- localStorage persistence with try/catch -- handles private browsing
- Clean SSE auto-close on terminal status in both frontend and backend
- Cancel propagation: cancels pending items AND updates job status atomically

---

## Recommended Actions (Priority Order)

1. **[CRITICAL]** Fix admin playlists page server to return `overview` data
2. **[CRITICAL]** Add `signal: request.signal` to SSE proxy route
3. **[HIGH]** Add concurrency semaphore for playlist processors
4. **[HIGH]** Store AbortControllers for auto-save downloads, abort on cleanup
5. **[MEDIUM]** Extract playlist job logic from `+page.svelte` into module
6. **[MEDIUM]** Add warning log for unknown status values in `from_str`
7. **[LOW]** Consider deduplication for same-URL playlist jobs

---

## Unresolved Questions

1. Is there a DB migration file for `playlist_jobs` and `playlist_job_items` tables? Not seen in this diff -- were they created in a previous phase?
2. Does the `updated_at` column exist in the `playlist_jobs` table or only `updated_at_ms`? The admin query uses `updated_at` while repo uses both.
3. Should `Ready` items that fail to download on the client side trigger a server-side retry or re-extract?
