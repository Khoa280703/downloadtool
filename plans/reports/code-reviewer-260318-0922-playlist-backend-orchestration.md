# Code Review: Playlist Backend Orchestration

**Reviewer:** code-reviewer | **Date:** 2026-03-18
**Scope:** 16 files across Rust backend + SvelteKit frontend
**Focus:** Security, resource management, correctness, performance

---

## Overall Assessment

Solid implementation of backend-orchestrated playlist processing. Clean separation between repository, processor, API routes, and frontend proxy/client. All SQL uses parameterized queries (no injection risk). Rate limiting properly applied to create endpoint. The design follows existing mux job patterns well.

**Rating: Good** -- a few high-priority items need attention before production.

---

## Critical Issues

### C1. SSE Events Proxy Missing AbortSignal Forwarding
**File:** `frontend/src/routes/api/proxy/playlist-jobs/[jobId]/events/+server.ts`

The SSE proxy does NOT forward `request.signal` to the upstream fetch. When the client disconnects, the SvelteKit proxy will close its side, but the upstream connection to the Rust API stays open indefinitely. The Rust SSE handler (`playlist_job_events_handler`) will keep polling the DB every 2 seconds until the job reaches a terminal state.

**Impact:** Resource leak -- orphaned SSE connections and DB polls accumulate if users navigate away during long-running playlists.

**Fix:**
```typescript
const upstream = await fetch(
    buildRustApiUrl(`/api/playlist-jobs/${encodeURIComponent(params.jobId)}/events`),
    {
        headers: await buildRustApiHeaders(request, false, downloadSessionId),
        signal: request.signal  // <-- add this
    }
);
```

### C2. `insert_items` Not Wrapped in a Transaction
**File:** `crates/job-system/src/playlist_job_repository.rs:196-244`

Bulk item insertion runs N individual INSERT queries without a transaction. If the process crashes after inserting 5 of 20 items, the DB will have partial data with no way to know discovery is incomplete. The `set_discovery_result` already updated `total_items`, so the job will show "5/20 completed" but 15 items simply don't exist.

**Impact:** Data inconsistency on partial failure during discovery phase.

**Fix:** Wrap `insert_items` in a transaction, or batch them into a single multi-value INSERT.

---

## High Priority

### H1. Authorization Bypass in SSE Polling Loop
**File:** `crates/api/src/routes/playlist_jobs.rs:208`

Inside the SSE stream's polling loop, `get_job` is called with `(None, None)` for owner parameters:
```rust
let Some(current_job) = repo.get_job(&jid, None, None).await.ok().flatten()
```

The initial auth check validates ownership, but subsequent polls bypass it. This is functionally acceptable (the job_id is unguessable), but inconsistent with the auth model. The same pattern exists in `playlist_processor.rs:57` and `:113` which is expected since that's a server-side task.

**Risk:** Low in practice (IDs are `pl-{timestamp}-{seq}`) but violates principle of least privilege.

### H2. `get_job` Authorization Query Has Edge Case
**File:** `crates/job-system/src/playlist_job_repository.rs:91-98`

```sql
WHERE id = $1
  AND (user_id = $2 OR session_id = $3 OR ($2 IS NULL AND $3 IS NULL))
```

When both `$2` and `$3` are NULL, the `($2 IS NULL AND $3 IS NULL)` clause matches ANY job regardless of ownership. This means unauthenticated users without a session header could see any job. However, `resolve_playlist_owner` in the route handler returns `UNAUTHORIZED` if both are empty, so this SQL edge case is mitigated by the handler layer.

**Risk:** Low -- defense in depth exists, but the SQL should be self-protecting.

### H3. No Limit on Playlist Size
**Files:** `playlist_processor.rs`, `playlist_jobs.rs`

There's no cap on how many items a playlist can contain. A playlist with 500+ videos will:
- Create 500+ DB rows in `insert_items` (no batch INSERT)
- Run 500+ sequential extract + mux operations (each taking seconds to minutes)
- Keep a tokio task alive for potentially hours

**Fix:** Add a max items limit (e.g., 50-100) in `discover_playlist_items`.

### H4. No Concurrency Limit on Background Playlist Tasks
**File:** `crates/api/src/routes/playlist_jobs.rs:125`

Every `POST /api/playlist-jobs` spawns an unbounded `tokio::spawn`. There's no semaphore or queue limit. An attacker could create 100 playlist jobs, each spawning a long-running task consuming extractor pool slots and DB connections.

**Fix:** Add a semaphore or limit concurrent playlist processors (e.g., `Arc<Semaphore>` in AppState).

---

## Medium Priority

### M1. Hardcoded `.mp4` Extension in Auto-Save
**File:** `frontend/src/routes/+page.svelte:356`

```typescript
void saveDownload(downloadUrl, `${item.title ?? item.video_id}.mp4`, ...)
```

Always uses `.mp4` regardless of whether the download is audio-only (e.g., `selectedDownloadMode === 'audio'`). Audio-only downloads should use `.m4a` or similar.

### M2. `EventSource` Does Not Auto-Close on Terminal State
**File:** `frontend/src/lib/playlist-job-api.ts:130-138`

The SSE client parses snapshots but never checks for terminal status to auto-close. It relies on the server to close the connection. If the server-side loop breaks but doesn't cleanly close the SSE, the `EventSource` will reconnect indefinitely (browser default behavior).

**Fix:** Check `snapshot.status` inside the event listener and call `es.close()` on terminal.

### M3. Race Between Cancel and Processing Loop
**Files:** `playlist_jobs.rs:256-266`, `playlist_processor.rs:56-63`

The cancel handler sets job status to `Cancelled` and cancels pending items. But the processor loop may have already claimed an item (status = `Extracting`) before the cancel takes effect. That in-flight item will complete/fail, then the next loop iteration will detect `Cancelled` and stop. This is acceptable behavior but should be documented.

### M4. `rand_jitter` Is Not Random
**File:** `crates/api/src/services/playlist_processor.rs:378-384`

Uses `SystemTime::now()` hashed as "randomness". Within a tight loop, successive calls may produce the same or very similar jitter values. Consider using `rand` crate or `fastrand`.

### M5. `from_str` Silently Defaults to First Variant
**File:** `crates/job-system/src/playlist_job_models.rs:26-34`

Unknown status strings silently map to `Queued`/`Pending`. If a future code change introduces a new status that gets persisted but not added to `from_str`, records will appear as `Queued` with no warning. Consider logging unknown values.

---

## Low Priority

### L1. Duplicate Thumbnail Rendering Logic
`BatchProgress.svelte` and `+page.svelte` both render playlist item thumbnails with fallback icons. Could be extracted to a shared component.

### L2. CSS-only Dark Mode Styling is Verbose
`BatchProgress.svelte` has 50+ lines of `:global(.page-root.theme-dark)` overrides. Consider using CSS custom properties for theming.

---

## Positive Observations

1. **SQL injection prevention**: All queries use parameterized bindings via sqlx -- no string interpolation in SQL
2. **`SELECT FOR UPDATE SKIP LOCKED`**: Proper concurrent item claiming pattern in `claim_next_pending_item`
3. **Rate limiting**: Shared with extract endpoint, properly gated behind feature flag
4. **Frontend proxy pattern**: Consistent with existing mux job proxies, proper session ID forwarding
5. **Ownership model**: Supports both authenticated users and anonymous sessions
6. **Status machine**: Clear terminal/non-terminal distinction via `is_terminal()`
7. **DB migration**: Proper indexes on status, user_id, session_id for query performance
8. **Frontend cleanup**: `onDestroy` closes SSE subscription, prevents leaks in SPA navigation
9. **`encodeURIComponent`**: Properly used in all frontend URL construction

---

## Recommended Actions (Priority Order)

1. **Add `signal: request.signal`** to SSE events proxy (C1)
2. **Wrap `insert_items` in a DB transaction** (C2)
3. **Add playlist size limit** (H3) -- suggest 50 items max
4. **Add concurrency semaphore** for playlist processors (H4)
5. **Auto-close EventSource on terminal state** (M2)
6. **Fix hardcoded `.mp4` extension** based on download mode (M1)

---

## DB Migration Review

**File:** `crates/api/app-migrations/0011_create_playlist_jobs.sql`

- Schema is clean, proper FK references with `ON DELETE CASCADE`
- Partial indexes on `user_id` and `session_id` (WHERE NOT NULL) -- efficient
- Composite index on `(playlist_job_id, status)` -- good for `claim_next_pending_item`
- `updated_at` column with `DEFAULT NOW()` exists but no trigger for auto-update -- relies on app code, consistent with existing pattern

---

## Unresolved Questions

1. Is there a cleanup/TTL mechanism planned for completed playlist jobs? Without one, the `playlist_jobs` and `playlist_job_items` tables will grow unbounded.
2. Should the `download_url` stored in items (e.g., `/api/jobs/{id}/file-ticket`) include auth tokens, or is the file-ticket endpoint separately authenticated?
3. What happens when the API server restarts mid-processing? Playlist jobs in `Processing` status will be orphaned with no recovery mechanism visible in this diff.
