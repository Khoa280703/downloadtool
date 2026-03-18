# Phase 4: Hardening, Rate Limit, Cleanup

## Context Links

- `frontend/src/routes/api/proxy/extract/+server.ts`
- `config/runtime-limit-profiles.json`
- Cloudflare rate limiting discussions
- `frontend/src/lib/playlist-download-worker.ts`

## Overview

- Priority: P2
- Status: pending
- Brief: Clean old path, retune rate limiting, and verify operational behavior.

## Key Insights

- Once playlist item extract moves server-side, Cloudflare no longer sees one extract request per item from browser.
- Then `/api/proxy/extract` can be limited for single-video usage only, and playlist create endpoints can have their own gentler limits.
- Old client worker code becomes misleading technical debt if left half-alive.

## Requirements

- Keep backward compatibility during rollout.
- Provide clean cutover path and rollback.
- Re-tune Cloudflare and app-level rate limits based on new traffic shape.

## Architecture

- During rollout:
  - keep `/api/batch`
  - add playlist-job endpoints
  - gate new frontend path behind config or branch cutover
- After parity:
  - delete old client playlist worker orchestration
  - keep only browser file saver helpers

## Related Code Files

- Modify:
  - `config/runtime-limit-profiles.json`
  - `frontend/src/routes/api/proxy/extract/+server.ts`
  - admin dashboard files if playlist jobs are surfaced there
- Delete or retire:
  - `frontend/src/lib/playlist-download-worker.ts` orchestration logic

## Implementation Steps

1. Add operational logs and admin visibility for playlist jobs.
2. Re-tune Cloudflare rules:
   - limit playlist create endpoints, not per-item browser extracts
   - keep `/api/batch` relatively open
3. Add app-level quotas for playlist job creation if needed.
4. Remove dead client-worker code.
5. Update docs and run end-to-end verification.

## Todo List

- [ ] Add playlist admin observability
- [ ] Re-tune rate limits
- [ ] Remove obsolete client orchestration
- [ ] Update docs
- [ ] Run full test matrix

## Success Criteria

- Cloudflare no longer fights normal playlist downloads.
- Old and new paths are not both mutating playlist state in parallel.
- Admin can inspect playlist jobs and item failures.

## Risk Assessment

- Biggest rollout risk is half-cutover where frontend and backend both think they own sequencing.

## Security Considerations

- Apply rate limit by owner/session/IP at playlist job creation, not only by raw IP.

## Next Steps

- If stable, unify single-video and playlist observability in the same admin area.
