# Phase 1: Data Model And API Contract

## Context Links
- [plan.md](/home/khoa2807/working-sources/downloadtool/plans/260318-0852-playlist-backend-orchestration/plan.md)
- [batch.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/batch.rs)
- [jobs.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/jobs.rs)
- [job_control_plane.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/services/job_control_plane.rs)

## Overview
- Priority: P1
- Status: pending
- Mục tiêu: thêm data model và API tối thiểu để playlist có parent job bền vững, item state rõ ràng.

## Key Insights
- `mux_jobs` đã giải quyết tốt single durable job; nên reuse, không viết queue mới từ đầu.
- `/api/batch` hiện chỉ trả playlist entries, chưa có parent persistence.
- Client worker hiện giữ queue trong memory, nên reload mất state.

## Requirements
- Có `playlist_jobs` và `playlist_job_items`.
- Item phải lưu đủ dữ liệu để late-extract:
  `video_id`, `source_url`, requested mode/quality, selected state.
- Parent phải có lifecycle: `queued`, `discovering`, `ready`, `processing`, `paused`, `completed`, `failed`, `cancelled`.

## Architecture
- `playlist_jobs`
  - owner/session, input_url, requested_mode, requested_quality, status, counters, error
- `playlist_job_items`
  - playlist_job_id, index, video_id, title, thumbnail, status, retry_count, error
  - execution_kind: `direct_stream` | `direct_copy` | `mux`
  - optional `mux_job_id`
  - optional resolved stream metadata for on-demand refresh

## Related Code Files
- Modify:
  - [crates/api/src/main.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/main.rs)
  - [crates/api/src/routes/mod.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/mod.rs)
  - [crates/api/src/routes/batch.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/batch.rs)
  - [crates/job-system/src/lib.rs](/home/khoa2807/working-sources/downloadtool/crates/job-system/src/lib.rs)
- Create:
  - `crates/api/migrations/0011_create_playlist_jobs.sql`
  - `crates/api/src/routes/playlist_jobs.rs`
  - `crates/api/src/services/playlist_control_plane.rs`
  - `crates/job-system/src/playlist_models.rs`
  - `crates/job-system/src/playlist_repository.rs`

## Implementation Steps
1. Thiết kế schema parent/item và index cho owner, status, updated_at.
2. Tạo repository read/write cho playlist jobs/items.
3. Định nghĩa API:
   - `POST /api/playlist-jobs`
   - `GET /api/playlist-jobs/{id}`
   - `GET /api/playlist-jobs/{id}/events`
   - `POST /api/playlist-jobs/{id}/pause`
   - `POST /api/playlist-jobs/{id}/resume`
   - `POST /api/playlist-jobs/{id}/cancel`
4. Chuẩn hóa response shape để frontend render chung được.

## Todo List
- [ ] Chốt schema parent/item
- [ ] Chốt enum status parent/item
- [ ] Chốt API surface V1
- [ ] Chốt owner semantics user/session

## Success Criteria
- DB model đủ cho resume sau reload/redeploy.
- API tạo/đọc playlist job được mà chưa cần worker chạy.

## Risk Assessment
- Over-design schema quá sớm.
- Dùng quá nhiều field stream-specific làm schema cồng kềnh.

## Security Considerations
- Reuse owner semantics giống `mux_jobs`.
- Không để user đọc playlist job của owner khác.

## Next Steps
- Sang Phase 2 để gắn lifecycle và publish orchestration.

Unresolved questions:
- Có cần lưu selected state per item ngay từ lúc discover xong, hay V1 mặc định tất cả selected?
