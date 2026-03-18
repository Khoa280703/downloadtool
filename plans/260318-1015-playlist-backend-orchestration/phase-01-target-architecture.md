# Phase 1: Target Architecture

## Context Links

- [Current batch SSE route](../../crates/api/src/routes/batch.rs)
- [Current durable job route](../../crates/api/src/routes/jobs.rs)
- [Current client playlist worker](../../frontend/src/lib/playlist-download-worker.ts)
- [Overview plan](./plan.md)

## Overview

- Priority: P1
- Status: pending
- Goal: define the new runtime boundary clearly before touching schema or UI.

## Key Insights

- Current system already does just-in-time extract per item.
- Current gap is orchestration ownership, not extract timing.
- Existing durable mux job pipeline is strong enough to reuse as backbone.

## Requirements

- Playlist creation is one backend request.
- Backend persists playlist job and item jobs.
- Worker extracts each item right before processing.
- Progress available for playlist-level and item-level UI.
- Cancel/resume survives tab close.

## Architecture

- New entity: `playlist_job`
- New entity: `playlist_job_item`
- API creates playlist job from playlist URL + requested mode/quality.
- Backend extracts playlist membership only once at creation time.
- Worker consumes pending items one-by-one.
- Each item either:
  - direct stream path
  - durable mux job path
- Playlist SSE emits aggregate progress and item updates.

## Related Code Files

- Modify:
  - `crates/api/src/routes/batch.rs`
  - `crates/api/src/routes/jobs.rs`
  - `crates/api/src/services/job_control_plane.rs`
  - `frontend/src/lib/api.ts`
  - `frontend/src/routes/+page.svelte`
  - `frontend/src/stores/batch.ts`
- Create:
  - playlist routes/services/repository files under `crates/api/src`
  - playlist-facing frontend API/store files under `frontend/src/lib`
- Delete later:
  - `frontend/src/lib/playlist-download-worker.ts` once migration completes

## Implementation Steps

1. Define playlist job lifecycle and item lifecycle.
2. Decide what reuses existing mux job tables/services and what needs new schema.
3. Define playlist progress payload shape for SSE.
4. Define compatibility boundary so single flow does not change.

## Todo List

- [ ] Finalize playlist states
- [ ] Finalize item states
- [ ] Finalize SSE payload contract
- [ ] Finalize ownership boundary frontend vs backend

## Success Criteria

- No ambiguity on where queue state lives.
- No item processing logic required in browser.

## Risk Assessment

- Over-coupling playlist and single job models.
- Duplicating state machine logic across playlist item and mux job.

## Security Considerations

- Playlist owner access rules must match existing job owner model.
- SSE/status endpoints must not leak other users' playlist data.

## Next Steps

- Implement schema and API contracts in Phase 2.

## Unresolved questions

- Whether playlist items should directly embed mux job IDs or reference artifact records abstractly.
