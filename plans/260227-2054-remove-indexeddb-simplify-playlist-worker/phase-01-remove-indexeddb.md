# Phase 01 — Remove IndexedDB & Simplify Worker + BatchInput

## Status: completed
## Priority: high
## Effort: small (~2h)

## Context links
- Brainstorm: session 2/27/2026 — concluded IndexedDB is YAGNI for a quick-use download tool
- Worker: `frontend/src/lib/playlist-download-worker.ts`
- BatchInput: `frontend/src/components/BatchInput.svelte`
- DB file: `frontend/src/lib/playlist-queue-db.ts`

## Related code files

### Delete
- `frontend/src/lib/playlist-queue-db.ts`

### Modify
- `frontend/src/lib/playlist-download-worker.ts`
- `frontend/src/components/BatchInput.svelte`

## Key insights
- `QueueEntry` type currently defined in `playlist-queue-db.ts`. After deletion, move simplified version to `playlist-download-worker.ts` and export it.
- All `await upsertEntry(...)` calls in worker are blocking state transitions unnecessarily. Removing them makes the hot path synchronous.
- `cancelAll()` currently does `void upsertEntry(pending)` fire-and-forget — these writes often don't complete before F5 kills the JS context anyway (stale 'downloading' entries). Removal is safe.
- `BatchInput.svelte` imports `QueueEntry` from DB file — needs to import from worker instead.
- `clearDoneEntries()` in `handleSubmit` becomes unnecessary.

## Implementation steps

### Step 1 — Simplify `QueueEntry` type and move to worker
In `playlist-download-worker.ts`, replace the import:
```ts
// Remove:
import { upsertEntry, type QueueEntry } from './playlist-queue-db';

// Add at top of file (exported type):
export type QueueEntry = {
  videoId: string;
  title: string;
  thumbnail?: string;
};
```

### Step 2 — Remove all DB calls from `playlist-download-worker.ts`

Remove these imports (no longer needed):
```ts
import { upsertEntry, type QueueEntry } from './playlist-queue-db';
```

In `enqueueDownload()`:
```ts
// Remove:
await upsertEntry(normalized);
// Keep: pendingQueue.push(normalized) and queue triggers
```

In `cancelAll()`:
```ts
// Remove the entire loop that calls void upsertEntry(pending)
// Keep: abort controllers, clear queues, clear activeEntries, reset activeCount
```

In `fillPrefetchBuffer()` catch blocks:
```ts
// Remove all upsertEntry calls
// Keep: updateBatchItemByVideoId calls (UI store)
```

In `startDownload()`:
```ts
// Remove: await upsertEntry(downloading) before download
// Remove: await upsertEntry({ status: 'completed' }) on success
// Remove: await upsertEntry({ status: 'pending' }) on AbortError
// Remove: await upsertEntry({ status: 'error' }) on error
// Keep: all updateBatchItemByVideoId calls (UI store)
```

In `resetWorkerState()`:
```ts
// Remove: any DB-related resets if present
```

Also remove `QueueEntry` normalized object fields that were only for DB:
```ts
// In enqueueDownload, the normalized object can drop status/error/addedAt:
const normalized: QueueEntry = {
  videoId: entry.videoId,
  title: entry.title,
  thumbnail: entry.thumbnail,
};
```

### Step 3 — Update `BatchInput.svelte`

Remove imports:
```ts
// Remove:
import { clearDoneEntries, getPendingEntries, upsertEntry, type QueueEntry } from '$lib/playlist-queue-db';

// Add:
import { type QueueEntry } from '$lib/playlist-download-worker';
```

Remove state and functions:
```ts
// Remove:
let pendingResume = $state<QueueEntry[]>([]);
async function resumePendingQueue(): Promise<void> { ... }
async function handleResumeDownloads(): Promise<void> { ... }
```

Remove from `onMount`:
```ts
// Remove:
void resumePendingQueue();
```

Remove from `handleSubmit`:
```ts
// Remove:
void clearDoneEntries();
// Remove:
await upsertEntry(entry);
```

Remove resume UI block from template:
```svelte
<!-- Remove entire block: -->
{#if pendingResume.length > 0}
  <div class="resume-notice">...</div>
{/if}
```

Remove unused CSS:
```css
/* Remove: .resume-notice, .resume-btn styles */
```

Keep `upsertEntry` removal in `enqueueDownload` call — the `BatchInput` still calls `enqueueDownload(entry)`, which is fine (signature unchanged).

### Step 4 — Delete `playlist-queue-db.ts`
```bash
rm frontend/src/lib/playlist-queue-db.ts
```

### Step 5 — Verify build
```bash
pnpm build:web
```
Check for any remaining imports of `playlist-queue-db`.

## Todo list
- [x] Move + simplify `QueueEntry` type in worker (export from worker)
- [x] Remove all `upsertEntry` calls from worker
- [x] Simplify `cancelAll()` to synchronous
- [x] Simplify `enqueueDownload()` — drop status/error/addedAt fields
- [x] Update `BatchInput.svelte` imports
- [x] Remove resume state + functions + UI from BatchInput
- [x] Remove unused CSS from BatchInput
- [x] Delete `playlist-queue-db.ts`
- [x] Run `pnpm build:web` — confirm no errors
- [ ] Manual test: start playlist download, observe UI works, cancel works

## Success criteria
- `playlist-queue-db.ts` does not exist
- `grep -r "playlist-queue-db"` returns no results
- `grep -r "upsertEntry\|getPendingEntries\|clearDoneEntries"` returns no results
- Build passes
- Batch download flow works: paste URL → links stream in → download starts → status updates in UI

## Risk assessment
- **Low risk**: DB layer was purely additive. In-memory queues + batch store handle everything needed.
- **Edge case**: `QueueEntry` type used in BatchInput for creating entries — covered by re-exporting from worker.

## Next steps
- After this phase: commit, rebuild binary (`cargo build --workspace`), redeploy
