**Context Links**
- [plan.md](plan.md)
- [phase-01-design-playlist-job-model-and-api.md](phase-01-design-playlist-job-model-and-api.md)
- [job_control_plane.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/services/job_control_plane.rs)
- [jobs.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/jobs.rs)

**Overview**
- Priority: high
- Status: proposed
- Brief: thêm backend entrypoint để tạo và điều phối playlist jobs

**Key Insights**
- Control plane hiện đã có pattern tốt cho create/get/status/reuse của mux jobs.
- Nên reuse triết lý đó thay vì invent hệ mới.

**Requirements**
- `POST /api/playlist-jobs` nhận playlist URL + mode + quality.
- Backend extract playlist metadata một lần, tạo item rows.
- Trả `playlistJobId` và SSE/status URLs.
- Support cancel/retry an toàn idempotent.

**Architecture**
- API layer validate input.
- Service layer tạo playlist job + item rows trong transaction.
- Publisher enqueue item đầu tiên hoặc toàn bộ item với trạng thái chờ coordinator.
- Audit log ghi actor/session/ip và playlist URL đã sanitize.

**Related Code Files**
- Modify:
  - `crates/api/src/main.rs`
  - `frontend/src/lib/server/audit-log.ts`
- Create:
  - `crates/api/src/routes/playlist_jobs.rs`
  - `crates/api/src/services/playlist_job_control_plane.rs`
  - `crates/job-system/src/playlist_*`
  - migrations mới
- Delete:
  - none

**Implementation Steps**
1. Add schema + repository helpers.
2. Add create/status/events/cancel/retry routes.
3. Add Redis progress publisher/subscriber.
4. Add ownership checks tương tự jobs hiện tại.

**Todo List**
- [ ] Create migrations
- [ ] Create route handlers
- [ ] Create control plane service
- [ ] Hook audit log

**Success Criteria**
- Tạo được playlist job và xem được progress stream trống/ban đầu.
- Cancel không làm bể item state.

**Risk Assessment**
- Transaction tạo nhiều item có thể chậm với playlist lớn.

**Security Considerations**
- Limit playlist size hợp lý.
- Reject non-playlist URLs sớm.

**Next Steps**
- Sang Phase 3 để worker thực thi item jobs.

**Unresolved questions**
- Có cần hard cap số item mỗi playlist ở phase đầu không.
