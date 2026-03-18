# Phase 2: Backend Playlist Control Plane

## Context Links
- [phase-01-data-model-and-api-contract.md](/home/khoa2807/working-sources/downloadtool/plans/260318-0852-playlist-backend-orchestration/phase-01-data-model-and-api-contract.md)
- [job_control_plane.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/services/job_control_plane.rs)
- [routes/jobs.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/jobs.rs)

## Overview
- Priority: P1
- Status: pending
- Mục tiêu: làm parent control plane giống mux job control plane, nhưng cho playlist.

## Key Insights
- `create_or_reuse_job` pattern hiện có dùng tốt cho dedupe và artifact reuse.
- Playlist cần control plane riêng; không nên nhồi thẳng vào `mux_jobs`.

## Requirements
- Tạo playlist job từ 1 request duy nhất.
- Discover playlist items ở backend.
- Publish orchestration work sang queue/worker.
- Pause/resume/cancel cập nhật DB atomically.

## Architecture
- `PlaylistControlPlaneService`
  - create job
  - discover items
  - publish first runnable item or parent orchestration signal
  - read status aggregate
  - mutate lifecycle
- Parent progress tính từ item counters, không phụ thuộc UI memory.

## Related Code Files
- Modify:
  - [crates/api/src/main.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/main.rs)
  - [crates/api/src/routes/batch.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/batch.rs)
  - [crates/api/src/services/mod.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/services/mod.rs)
- Create:
  - `crates/api/src/services/playlist_progress_publisher.rs`
  - `crates/api/src/routes/playlist_jobs.rs`

## Implementation Steps
1. Tạo service tạo parent job + discover playlist entries bằng extractor hiện có.
2. Persist item list xong mới phát event `ready`.
3. Tạo aggregate counters: total, pending, processing, ready, completed, failed.
4. Thêm SSE endpoint phát snapshot parent + item delta.
5. Thêm pause/resume/cancel handlers.

## Todo List
- [ ] Tạo create/listen/mutate APIs
- [ ] Persist discovery result
- [ ] Aggregate counters
- [ ] SSE progress stream cho parent

## Success Criteria
- Browser có thể tạo playlist job rồi chỉ subscribe trạng thái.
- Refresh trang không mất orchestration state.

## Risk Assessment
- Discover playlist chậm làm request create bị treo lâu.
- Parent progress bị lệch nếu aggregate update không nhất quán.

## Security Considerations
- Audit log cho create/pause/resume/cancel.
- Giới hạn owner access giống job thường.

## Next Steps
- Phase 3: worker xử lý item execution thật.

Unresolved questions:
- Create API nên đồng bộ chờ discover xong hay trả `202` sớm rồi discover async?
