# Remove IndexedDB — Simplify Playlist Download Worker

## Status: completed

## Context
Brainstorm session concluded: IndexedDB persistence for playlist queue is over-engineered for a quick-use tool. Users paste URL → download → done. Resume-across-sessions is a YAGNI feature. Removing it eliminates ~150 LOC, removes async overhead on every state change, and fixes a class of stale-state bugs (e.g. 'downloading' entries stuck in DB after F5 kill).

## Related files
- `frontend/src/lib/playlist-queue-db.ts` — DELETE
- `frontend/src/lib/playlist-download-worker.ts` — remove all DB calls
- `frontend/src/components/BatchInput.svelte` — remove resume UI

## Phases

| # | Phase | Status |
|---|-------|--------|
| 1 | [Remove IndexedDB & simplify worker + BatchInput](phase-01-remove-indexeddb.md) | completed |

## Success criteria
- `playlist-queue-db.ts` deleted
- No `upsertEntry` / `getPendingEntries` / `clearDoneEntries` calls anywhere
- No "Resume Downloads" UI in BatchInput
- `cancelAll()` is synchronous (no fire-and-forget DB writes)
- `QueueEntry` type simplified: `{ videoId, title, thumbnail? }`
- Batch download still works end-to-end
- Build passes (`pnpm build:web`)
