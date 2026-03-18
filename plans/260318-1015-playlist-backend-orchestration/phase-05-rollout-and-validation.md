# Phase 5: Rollout And Validation

## Context Links

- [Overview plan](./plan.md)
- [Existing job redesign plan](../260306-0927-job-control-plane-worker-storage-redesign/plan.md)

## Overview

- Priority: P1
- Status: pending
- Goal: ship safely without breaking single download flow.

## Key Insights

- Migration risk is in orchestration ownership, not low-level mux.
- Feature flag or guarded route switch is enough for rollout safety.

## Requirements

- Validate single download regression.
- Validate playlist survives refresh/tab close.
- Validate cancel/resume/retry.
- Validate proxy/extract pressure stays acceptable.

## Architecture

- Rollout behind feature flag or internal env guard.
- Keep old playlist flow removable but available during bring-up if needed.

## Related Code Files

- Modify:
  - runtime config or env config files
  - frontend route wiring
  - docs if flow changes materially

## Implementation Steps

1. Add rollout switch.
2. Test happy path playlist.
3. Test failed item retry.
4. Test cancel/resume.
5. Test single flow regression.
6. Remove old path once confidence is high.

## Todo List

- [ ] Validation checklist
- [ ] Rollout switch
- [ ] Regression checklist

## Success Criteria

- No single-download regressions.
- Playlist no longer depends on open tab.
- Rate-limit pressure on `/api/proxy/extract` becomes predictable.

## Risk Assessment

- Dual path maintenance if old flow remains too long.

## Security Considerations

- Ensure playlist endpoints obey same auth/session rules as current durable jobs.

## Next Steps

- Implementation can start from Phase 1 then Phase 2.

## Unresolved questions

- None beyond product UX choices already listed in overview plan.
