# Phase 3: Worker Orchestration And Progress

## Context Links

- [Current batch extractor](../../crates/api/src/routes/batch.rs)
- [Current mux job control plane](../../crates/api/src/services/job_control_plane.rs)
- [Current client playlist worker](../../frontend/src/lib/playlist-download-worker.ts)
- [Overview plan](./plan.md)

## Overview

- Priority: P1
- Status: pending
- Goal: move sequencing, retry, and progress authority to backend worker.

## Key Insights

- Reusing current mux worker path is cheaper than inventing a second execution system.
- Playlist worker should operate on item intents, not raw signed stream URLs.
- Progress needs two layers:
  - playlist aggregate
  - active item detail

## Requirements

- Worker picks next runnable playlist item.
- Worker performs extract just-in-time.
- If direct-stream eligible, produce ready download artifact or direct ticket safely.
- If mux needed, create/reuse durable mux job and follow its result.
- Retry item according to per-item attempt budget.
- Publish progress snapshots to SSE/pubsub.

## Architecture

- Playlist coordinator in backend:
  - receives playlist job created
  - fills item table from playlist extractor
  - schedules pending items
- Item execution:
  - extract current video formats
  - choose stream using shared selection logic
  - direct-stream item: create item-ready ticket or artifact reference
  - mux item: create/reuse existing durable mux job
  - mirror mux progress into playlist item progress
- Aggregate updater:
  - updates completed/failed/running counts

## Related Code Files

- Modify:
  - `crates/api/src/services/job_control_plane.rs`
  - worker/job-system crates related to queue and progress
  - `frontend/src/lib/api.ts`
- Create:
  - playlist orchestration service
  - playlist progress publisher/subscriber modules

## Implementation Steps

1. Add playlist coordinator service.
2. Add worker loop for playlist items.
3. Bridge mux job events into playlist item progress.
4. Add cancel token checks and resume logic.
5. Add bounded retries per item.

## Todo List

- [ ] Worker state machine defined
- [ ] Retry policy defined
- [ ] Cancel semantics defined
- [ ] Progress payload examples defined

## Success Criteria

- Closing browser does not stop playlist.
- Failed item can retry without restarting whole playlist.
- Active progress feels realtime enough for UI.

## Risk Assessment

- Direct-stream items are harder than mux items because there may be no durable artifact.
- Playlist aggregate progress can drift if item updates are not transactional enough.

## Security Considerations

- Do not expose raw upstream stream URLs longer than necessary.
- Cancel/resume actions must not leak job existence across users.

## Next Steps

- Replace frontend orchestration with thin UI client in Phase 4.

## Unresolved questions

- For direct-stream playlist items, whether to keep ephemeral proxy download path or always materialize an artifact.
