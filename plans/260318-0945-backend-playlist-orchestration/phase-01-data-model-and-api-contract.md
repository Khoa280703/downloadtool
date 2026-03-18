---
title: "Phase 1 - Data Model and API Contract"
status: complete
---

# Phase 1: Data Model and API Contract

## Context Links
- [plan.md](./plan.md)
- [frontend/src/lib/playlist-download-worker.ts](/home/khoa2807/working-sources/downloadtool/frontend/src/lib/playlist-download-worker.ts)
- [crates/api/src/routes/batch.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/batch.rs)

## Overview
- Priority: P0
- Status: complete
- Mục tiêu: định nghĩa resource backend cho playlist job và item job trước khi sửa logic.
- Completed: DB tables (playlist_jobs, playlist_job_items), Rust models, API routes (CRUD + SSE)

## Key Insights
- Flow hiện tại đã đúng ở chỗ `just-in-time extract`, nhưng orchestration vẫn ở client.
- Playlist cần persistence riêng để survive refresh/reconnect và để admin theo dõi.
- Final save vẫn ở client, nên backend chỉ nên điều phối đến trạng thái `ready_to_download`.

## Requirements
- Tạo `playlist_job` và `playlist_job_item` persisted trong Postgres.
- Track đủ state tổng và state từng item.
- Có API create/status/events/cancel rõ ràng.
- Không làm gãy mux job schema hiện tại.

## Architecture
- `playlist_jobs`
  - `id`, `source_url`, `title`, `status`, `total_items`, `completed_items`, `failed_items`, `requested_quality`, `requested_mode`, `created_by_user_id`, `request_ip`, `created_at`, `updated_at`
- `playlist_job_items`
  - `id`, `playlist_job_id`, `video_id`, `title`, `ordinal`, `status`, `attempt_count`, `last_error`, `selected_stream_meta`, `mux_job_id`, `artifact_key`, `download_url`, `created_at`, `updated_at`
- Event stream:
  - `playlist.status`
  - `playlist.item_discovered`
  - `playlist.item_status`
  - `playlist.item_ready`
  - `playlist.done`
  - `playlist.error`

## Related Code Files
- Modify:
  - `crates/api/src/*` job control plane and route registry
  - frontend typed API layer for new playlist routes
- Create:
  - playlist job model/repository files
  - SQL migration files

## Implementation Steps
1. Định nghĩa state machine cho playlist job và playlist job item.
2. Tạo migration Postgres cho 2 bảng mới + index theo `playlist_job_id`, `status`, `video_id`.
3. Định nghĩa payload API:
   - `POST /api/proxy/playlist-jobs`
   - `GET /api/proxy/playlist-jobs/:id`
   - `GET /api/proxy/playlist-jobs/:id/events`
   - `POST /api/proxy/playlist-jobs/:id/cancel`
4. Định nghĩa response contract cho frontend typed client.
5. Chuẩn hóa audit fields để admin track actor/session/ip.

## Todo List
- [x] Chốt state machine
- [x] Tạo migration
- [x] Tạo typed models
- [x] Tạo route contract
- [x] Map audit payload

## Success Criteria
- API contract đủ để frontend không cần tự queue item.
- State item đủ chi tiết để render progress + retry + admin debug.

## Risk Assessment
- Schema quá chi tiết sẽ nặng migrate; giữ tối giản ở đợt đầu.
- Nếu nhồi cả browser-save state vào DB sẽ rối; nên tách server-ready và client-saved.

## Security Considerations
- Ghi `created_by_user_id` nếu có session, nếu không thì ghi IP/fingerprint tối thiểu.
- Không expose raw upstream URLs bừa bãi ngoài item ready flow.

## Next Steps
- Sang Phase 2 để backend tự chạy discovery + processing loop.

## Unresolved Questions
- Có cần lưu `client_saved_at` để biết item đã thực sự được browser lưu xong chưa?
