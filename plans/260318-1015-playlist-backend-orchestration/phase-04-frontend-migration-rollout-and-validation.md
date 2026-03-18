---
title: "Phase 4 - Frontend Migration Rollout And Validation"
status: pending
---

# Phase 4: Frontend Migration, Rollout, Validation

## Context Links
- Current playlist page: [\+page.svelte](/home/khoa2807/working-sources/downloadtool/frontend/src/routes/+page.svelte)
- Current playlist worker: [playlist-download-worker.ts](/home/khoa2807/working-sources/downloadtool/frontend/src/lib/playlist-download-worker.ts)
- Current API client: [api.ts](/home/khoa2807/working-sources/downloadtool/frontend/src/lib/api.ts)

## Overview
Priority: P1
Status: pending
Goal: remove browser-owned playlist queueing without breaking existing UI, and roll out safely behind a feature flag.

## Key Insights
- UI can stay mostly the same: discovery panel, selection list, progress table.
- Biggest frontend change is data source, not layout.
- Fast rollback is mandatory because playlist touches proxies, artifacts, and long-running flows.

## Requirements
- Replace client queue/worker with backend playlist job API calls.
- Keep selection UI and per-item progress rows.
- Keep download actions obvious for completed items.
- Roll back to current frontend worker path with one flag.

## Architecture
- New frontend flow:
  1. `createPlaylistJob(url)`
  2. subscribe to playlist SSE
  3. render items from backend snapshot
  4. `updatePlaylistSelection(...)`
  5. `startPlaylistJob(id)`
  6. for completed items, download backend-generated artifact URLs
- Feature flag:
  `PUBLIC_PLAYLIST_BACKEND_ORCHESTRATION=true|false`
  - `false` = current frontend worker path
  - `true` = new backend playlist path

## Related Code Files
- Modify:
  `frontend/src/routes/+page.svelte`
  `frontend/src/lib/api.ts`
  `frontend/src/lib/types.ts`
  `frontend/src/components/BatchProgress.svelte`
- Remove later:
  `frontend/src/lib/playlist-download-worker.ts` from active path, not deleted in first rollout

## Implementation Steps
1. Add playlist job client methods and types.
2. Swap playlist fetch/start handlers to backend endpoints behind feature flag.
3. Reuse existing progress UI with backend item statuses.
4. Keep old worker code dormant for rollback.
5. Add admin/audit fields for playlist jobs if useful.

## Todo List
- [ ] Add playlist API client
- [ ] Add playlist snapshot/item DTOs
- [ ] Wire feature flag
- [ ] Reuse BatchProgress with backend statuses
- [ ] Preserve current UX copy and controls

## Success Criteria
- User can refresh page and continue seeing real backend playlist state.
- Closing tab no longer kills playlist execution.
- Cloudflare rate limit can be tightened on `/api/proxy/extract` because browser no longer loops item extracts.

## Risk Assessment
- Risk: UI regression from changing progress source.
  Mitigation: keep current component shell, swap data provider only.
- Risk: rollout breaks download expectations.
  Mitigation: feature flag + side-by-side fallback path.

## Security Considerations
- Never expose internal artifact URLs without existing file-ticket/download authorization rules.
- Ensure playlist SSE only returns owner-scoped rows.

## Migration Strategy
- Ship backend tables + APIs first.
- Ship durable passthrough item jobs second.
- Enable new frontend path in local/dev.
- Enable in production behind flag for admin only, then full rollout.

## Rollback Strategy
- Flip `PUBLIC_PLAYLIST_BACKEND_ORCHESTRATION=false`.
- Old `/api/batch` + frontend worker path stays intact.
- New backend tables can remain unused; no destructive rollback needed.

## Validate Strategy
- `cargo check --workspace`
- backend tests for playlist repository + control plane + item dispatch
- `pnpm --filter frontend check`
- manual matrix:
  short playlist, long playlist, direct-stream item, mux item, partial failure, refresh mid-run, close tab mid-run, retry failed
- verify Cloudflare rules:
  do not aggressively limit playlist SSE
  tighten `/api/proxy/extract` after frontend worker path is disabled

## Unresolved Questions
- Whether completed playlist items should auto-download one by one or just expose a ready list for manual save.
