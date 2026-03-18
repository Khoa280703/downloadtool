## Context Links
- `crates/api/src/routes/batch.rs`
- `crates/api/src/routes/jobs.rs`
- `crates/api/src/services/job_control_plane.rs`
- `frontend/src/routes/+page.svelte`
- `frontend/src/lib/playlist-download-worker.ts`

## Overview
Priority: high
Status: proposed
Mục tiêu phase này: thêm model + API cho playlist job, nhưng chưa thay toàn bộ frontend ngay.

## Key Insights
- `/api/batch` hiện chỉ enumerate playlist items; orchestration chưa ở server.
- `/api/jobs` đã có durable primitives tốt: owner, status, reuse, SSE progress.
- Cần playlist job riêng; không nên nhét playlist vào mux job cũ.

## Requirements
- Tạo playlist job từ playlist URL.
- Persist playlist items + selected mode/quality snapshot.
- Có status tổng và status per item.
- Hỗ trợ cancel, resume, get details.

## Architecture
- Thêm `playlist_jobs` và `playlist_job_items`.
- `playlist_jobs`: id, owner, source_url, download_mode, quality, status, counters, created/updated.
- `playlist_job_items`: playlist_job_id, video_id, title, thumbnail, ordinal, status, attempts, error, output refs.
- API mới:
1. `POST /api/playlist-jobs`
2. `GET /api/playlist-jobs/{id}`
3. `GET /api/playlist-jobs/{id}/events`
4. `POST /api/playlist-jobs/{id}/cancel`
5. `POST /api/playlist-jobs/{id}/resume`

## Related Code Files
- Modify:
  - `crates/api/src/main.rs`
  - `crates/api/src/routes/mod.rs`
  - `crates/api/src/routes/batch.rs`
- Create:
  - `crates/api/src/routes/playlist_jobs.rs`
  - `crates/api/src/services/playlist_job_control_plane.rs`
  - `crates/api/src/services/playlist_progress_store.rs`
  - `crates/job-system/*` or equivalent repository files for playlist domain
  - DB migration files
- Delete:
  - none

## Implementation Steps
1. Add schema + repository for playlist jobs/items.
2. Add create endpoint: validate playlist URL, enumerate item metadata only, persist job + items.
3. Emit initial progress snapshot after enumeration.
4. Add read/detail endpoint for page reload recovery.
5. Add cancel/resume endpoints with owner checks.

## Todo List
- [ ] Schema designed for playlist job + item state
- [ ] Migration added
- [ ] Repository CRUD ready
- [ ] REST + SSE contract written
- [ ] Auth/owner semantics aligned with current job routes

## Success Criteria
- Tạo playlist job trả về `playlist_job_id`
- Playlist items persist server-side
- Reload vẫn query lại được job và item list

## Risk Assessment
- Schema quá tham lam sẽ kéo dài refactor
- Reuse sai abstraction của mux jobs sẽ làm code rối

## Security Considerations
- Owner scoping giống `/api/jobs`
- Validate playlist URL chặt
- Không trust title/thumbnail từ client

## Next Steps
- Worker orchestration + event model

## Unresolved Questions
- Có cần save selection per item từ lần create đầu, hay chỉ save full list rồi patch selection sau?
