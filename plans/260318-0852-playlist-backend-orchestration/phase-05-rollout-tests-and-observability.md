# Phase 5: Rollout, Tests And Observability

## Context Links
- [phase-04-frontend-migration-and-realtime-progress.md](/home/khoa2807/working-sources/downloadtool/plans/260318-0852-playlist-backend-orchestration/phase-04-frontend-migration-and-realtime-progress.md)
- [admin-dashboard.ts](/home/khoa2807/working-sources/downloadtool/frontend/src/lib/server/admin-dashboard.ts)
- [routes/(auth)/admin/jobs/+page.svelte](/home/khoa2807/working-sources/downloadtool/frontend/src/routes/(auth)/admin/jobs/+page.svelte)

## Overview
- Priority: P1
- Status: pending
- Mục tiêu: rollout an toàn, đo được, rollback được.

## Key Insights
- Admin hiện đã có jobs/activity/proxy; nên mở rộng thay vì tạo dashboard mới.
- Playlist orchestration sẽ tăng đáng kể lượng state/event; thiếu metrics là mù.

## Requirements
- Test unit/integration cho repository, control plane, worker runner.
- Admin thấy parent jobs + item summary.
- Audit log ghi create/start/pause/resume/cancel/fail.
- Có feature flag rollout.

## Architecture
- Feature flag: `PLAYLIST_BACKEND_ORCHESTRATION=true|false`
- Song song một thời gian:
  - `false`: flow cũ
  - `true`: flow mới

## Related Code Files
- Modify:
  - [frontend/src/lib/server/admin-dashboard.ts](/home/khoa2807/working-sources/downloadtool/frontend/src/lib/server/admin-dashboard.ts)
  - [frontend/src/routes/(auth)/admin/jobs/+page.svelte](/home/khoa2807/working-sources/downloadtool/frontend/src/routes/(auth)/admin/jobs/+page.svelte)
  - [crates/api/src/config.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/config.rs)
- Create:
  - playlist integration tests
  - worker execution tests

## Implementation Steps
1. Thêm feature flag config.
2. Thêm admin queries cho playlist parents/items.
3. Thêm tests:
   - create/discover
   - item retry
   - pause/resume/cancel
   - mux reuse
4. Rollout local -> prod hidden -> prod default.

## Todo List
- [ ] Feature flag
- [ ] Admin metrics
- [ ] Tests
- [ ] Rollout checklist

## Success Criteria
- Có rollback nhanh về flow cũ.
- Có visibility rõ playlist stuck ở đâu.
- Không regression single jobs.

## Risk Assessment
- Hai flow chạy song song dễ gây branch logic bẩn.
- Admin query nặng nếu join item events quá nhiều.

## Security Considerations
- Admin chỉ đọc aggregated operational data.
- Không expose secret URLs trong admin payload.

## Next Steps
- Sau rollout ổn, xóa flow client-worker cũ.

Unresolved questions:
- Có cần migration/backfill gì cho playlist jobs nếu đổi schema ở V2 không?
