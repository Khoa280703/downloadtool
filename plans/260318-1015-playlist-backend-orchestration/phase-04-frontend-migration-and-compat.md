# Phase 4: Frontend Migration And Compatibility

## Context Links

- [Home page playlist flow](../../frontend/src/routes/+page.svelte)
- [Current client playlist worker](../../frontend/src/lib/playlist-download-worker.ts)
- [Frontend API client](../../frontend/src/lib/api.ts)
- [Overview plan](./plan.md)

## Overview

- Priority: P1
- Status: pending
- Goal: frontend becomes thin control panel, not queue owner.

## Key Insights

- Most current UI can stay.
- Biggest change is data source and button semantics.
- Single download button logic should not be touched except shared helper reuse.

## Requirements

- Start playlist job.
- Subscribe to playlist SSE.
- Show aggregate and per-item progress.
- Cancel/resume playlist.
- Retry one item.
- Download completed items individually.

## Architecture

- Remove frontend queue worker ownership.
- Replace local pending/ready/active orchestration with server-backed store.
- Keep current visual components where possible.
- Preserve single download path in `DownloadBtn.svelte`.

## Related Code Files

- Modify:
  - `frontend/src/routes/+page.svelte`
  - `frontend/src/lib/api.ts`
  - `frontend/src/stores/batch.ts`
  - `frontend/src/components/BatchProgress.svelte`
- Delete:
  - `frontend/src/lib/playlist-download-worker.ts` after feature parity

## Implementation Steps

1. Add playlist API client methods.
2. Add playlist store backed by server events.
3. Rewire home page actions to start/cancel/resume/retry.
4. Keep old UI skeleton, swap data source under it.
5. Remove dead client orchestration code.

## Todo List

- [ ] API client methods
- [ ] Store contract
- [ ] Home page rewired
- [ ] Old worker removed

## Success Criteria

- Refresh page keeps showing server truth.
- Playlist can continue without tab staying open.
- Single flow unaffected.

## Risk Assessment

- UI regressions from changing store semantics.
- Mixed temporary states during migration if old and new flow coexist too long.

## Security Considerations

- Frontend should consume only server-scoped playlist identifiers.

## Next Steps

- Roll out with validation and fallback in Phase 5.

## Unresolved questions

- Whether completed playlist items should auto-download or require explicit click per item in first rollout.
