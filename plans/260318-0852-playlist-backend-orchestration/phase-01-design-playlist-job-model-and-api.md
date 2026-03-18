**Context Links**
- [plan.md](plan.md)
- [batch.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/batch.rs)
- [jobs.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/jobs.rs)
- [playlist-download-worker.ts](/home/khoa2807/working-sources/downloadtool/frontend/src/lib/playlist-download-worker.ts)

**Overview**
- Priority: high
- Status: proposed
- Brief: chốt contract trước khi code để tránh vá schema/API nhiều lần

**Key Insights**
- `/api/batch` hiện chỉ stream metadata playlist, chưa có orchestration server-side.
- `mux_jobs` hiện giải quyết tốt job đơn, progress, ticket, artifact, nhưng chưa có concept playlist tổng.
- KISS nhất là tạo `playlist_jobs` + `playlist_job_items`, không nhồi chung vào `mux_jobs` ngay.

**Requirements**
- Tạo playlist job từ 1 URL playlist.
- Track từng item: pending, queued, extracting, downloading, muxing, uploading, ready, failed, canceled.
- Support cancel toàn playlist hoặc từng item.
- Query status và progress realtime.
- Không đổi single download API hiện tại.

**Architecture**
- API mới:
1. `POST /api/playlist-jobs`
2. `GET /api/playlist-jobs/:id`
3. `GET /api/playlist-jobs/:id/events`
4. `POST /api/playlist-jobs/:id/cancel`
5. `POST /api/playlist-jobs/:id/items/:itemId/retry`
- DB mới:
1. `playlist_jobs`
2. `playlist_job_items`
- Redis/pubsub mới cho progress playlist và item.

**Related Code Files**
- Modify:
  - `crates/api/src/main.rs`
  - `crates/api/src/routes/batch.rs`
  - `crates/api/src/services/job_control_plane.rs`
  - `crates/job-system/src/*`
- Create:
  - `crates/api/src/routes/playlist_jobs.rs`
  - migrations cho `playlist_jobs`, `playlist_job_items`
  - playlist progress store/repo modules
- Delete:
  - none

**Implementation Steps**
1. Chốt schema tối thiểu cho playlist job và item.
2. Chốt dedupe key cho item artifact reuse.
3. Chốt payload progress SSE cho job tổng và item.
4. Chốt action matrix: cancel, retry item, retry whole job.

**Todo List**
- [ ] Define playlist job states
- [ ] Define playlist item states
- [ ] Define SSE event payload
- [ ] Define artifact reuse key

**Success Criteria**
- API contract đủ để frontend bỏ client orchestration.
- State machine không mơ hồ.

**Risk Assessment**
- Nếu scope phase này tràn sang implementation sẽ kéo dài.

**Security Considerations**
- Ownership phải theo session/user tương tự mux jobs hiện tại.
- Không leak playlist job của user khác.

**Next Steps**
- Sang Phase 2 tạo control plane backend.

**Unresolved questions**
- Nên gắn playlist ownership theo session hiện có hay tạo playlist session ID riêng?
