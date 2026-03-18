# Phase 3: Frontend Cutover To Playlist Jobs

## Context Links

- `frontend/src/routes/+page.svelte`
- `frontend/src/components/BatchProgress.svelte`
- `frontend/src/stores/batch.ts`
- `frontend/src/lib/api.ts`
- `frontend/src/lib/playlist-download-worker.ts`
- `frontend/src/lib/playlist-download-file-saver.ts`

## Overview

- Priority: P1
- Status: pending
- Brief: Remove browser-owned playlist orchestration, keep browser-owned file saving only.

## Key Insights

- UI already has good playlist preview/progress surface. Reuse it.
- The real cut is not visual; it is who owns state transitions.
- `playlist-download-file-saver.ts` should stay. It solves browser save behavior, not orchestration.

## Requirements

- Keep current preview UX from `/api/batch`.
- After user selects items and clicks download, create backend playlist job.
- Replace local worker progress with SSE-fed server progress.
- When an item becomes ready, browser downloads from playlist item file endpoint.
- Support resume after reload by polling/re-subscribing playlist job ID.

## Architecture

- Frontend does:
  1. preview playlist via `/api/batch`
  2. create playlist job
  3. subscribe `/api/proxy/playlist-jobs/{id}/events`
  4. trigger save when item status moves to `ready`
- Frontend no longer:
  - calls `extract()` per playlist item
  - owns `MAX_CONCURRENT`
  - decides direct-vs-mux path

## Related Code Files

- Modify:
  - `frontend/src/routes/+page.svelte`
  - `frontend/src/components/BatchProgress.svelte`
  - `frontend/src/stores/batch.ts`
  - `frontend/src/lib/api.ts`
  - `frontend/src/lib/types.ts`
- Delete or shrink:
  - `frontend/src/lib/playlist-download-worker.ts`
- Keep:
  - `frontend/src/lib/playlist-download-file-saver.ts`

## Implementation Steps

1. Add API client methods for playlist job create/status/events/file.
2. Convert batch store to server-fed progress model.
3. Replace `enqueueDownload()` loop with one playlist job create call.
4. Trigger save only when item transitions to ready.
5. Add reconnect/resume support using stored active playlist job ID.

## Todo List

- [ ] Add playlist job client helpers
- [ ] Rewire BatchProgress to server events
- [ ] Remove client orchestration loop
- [ ] Preserve folder-picker / anchor fallback behavior
- [ ] Support reload recovery

## Success Criteria

- Playlist can continue after tab reload.
- Batch UI reflects backend truth, not local guessed state.
- Browser still saves files to chosen location/fallback anchor path.

## Risk Assessment

- Auto-downloading many ready items can race browser gesture/download restrictions. Queue save triggers carefully.

## Security Considerations

- Playlist job SSE and file endpoints must remain owner-scoped.
- Avoid leaking raw stream URLs into persistent frontend state.

## Next Steps

- After frontend cutover, old client worker can be removed and rate limiting simplified.
