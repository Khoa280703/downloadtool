## Context Links
- `frontend/src/routes/+page.svelte`
- `frontend/src/lib/api.ts`
- `crates/api/src/routes/jobs.rs`
- `crates/api/src/routes/batch.rs`
- `crates/worker/*`

## Overview
Priority: medium
Status: proposed
Mục tiêu phase này: rollout an toàn, không phá single flow, dọn legacy client worker.

## Key Insights
- Playlist là flow dài, cần test reload/cancel/resume thật.
- Regression lớn nhất là đụng single mux jobs hoặc progress SSE cũ.

## Requirements
- Test unit/repo/service cho playlist state machine.
- Test integration API + worker + SSE.
- Test manual UX cho fetch/start/reload/cancel/resume.
- Remove or gate legacy playlist worker path.

## Architecture
- Rollout theo flag:
  - off: legacy
  - on: backend playlist jobs
- Khi ổn định mới xóa legacy worker code.

## Related Code Files
- Modify:
  - test files across `crates/api`, `crates/worker`, `frontend`
  - runtime config / feature flag files if needed
- Delete:
  - `frontend/src/lib/playlist-download-worker.ts` when rollout done

## Implementation Steps
1. Add backend tests for create/cancel/resume/retry/progress.
2. Add frontend checks for SSE-driven playlist state.
3. Add feature flag for staged rollout if needed.
4. Run local + production smoke tests with real playlist.
5. Remove legacy client orchestration after confidence.

## Todo List
- [ ] Unit tests
- [ ] Integration tests
- [ ] Manual QA checklist
- [ ] Feature flag or safe rollout switch
- [ ] Legacy worker cleanup

## Success Criteria
- Playlist backend flow stable in production
- No regressions in single download
- No duplicate orchestration path left

## Risk Assessment
- Two orchestration systems song song sẽ gây drift
- Test coverage thiếu ở SSE/progress sẽ làm bug production khó debug

## Security Considerations
- Audit log cho create/cancel/resume/retry actions
- Rate limit backend endpoints theo owner/IP thay vì chỉ Cloudflare

## Next Steps
- Sau rollout ổn, tối ưu concurrency per playlist job nếu cần

## Unresolved Questions
- Có cần admin dashboard section riêng cho playlist jobs ngay phase đầu không?
