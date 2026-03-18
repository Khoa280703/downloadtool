# Playlist Backend Orchestration

Status: proposed
Priority: high
Goal: chuyển playlist từ client-orchestrated sang backend-orchestrated, vẫn just-in-time extract từng item, giữ nguyên single download flow

Phases
- [Phase 1](phase-01-playlist-job-domain-and-api.md) - playlist job domain + API surface
- [Phase 2](phase-02-playlist-worker-and-progress-pipeline.md) - backend worker orchestration + realtime progress
- [Phase 3](phase-03-frontend-migration-to-playlist-jobs.md) - frontend migration khỏi client queue
- [Phase 4](phase-04-testing-rollout-and-cleanup.md) - verify, rollout, cleanup legacy flow

Current state
- `/api/batch` chỉ trả playlist items qua SSE
- frontend đang giữ queue, selection, sequencing, extract-per-item
- muxing đã có durable job system + SSE progress riêng cho single/mux path
- playlist hiện là hybrid: backend-assisted, client-orchestrated

Target state
- browser chỉ tạo playlist job + subscribe progress
- backend giữ queue item, retry, cancel, resume, selection snapshot
- worker chỉ extract khi item chuẩn bị xử lý
- single download flow giữ nguyên routes/jobs hiện tại

Key constraints
- Không extract full stream URLs upfront cho cả playlist
- Không phá `/api/jobs` hiện tại
- Không làm admin/audit kém đi; ngược lại phải log tốt hơn
- Có thể rollout theo từng phase, không big-bang

Primary code areas
- `crates/api/src/routes/batch.rs`
- `crates/api/src/routes/jobs.rs`
- `crates/api/src/main.rs`
- `crates/api/src/services/job_control_plane.rs`
- `crates/worker/*`
- `frontend/src/routes/+page.svelte`
- `frontend/src/lib/playlist-download-worker.ts`
- `frontend/src/lib/api.ts`
- `frontend/src/stores/batch.ts`

Dependencies
- Reuse durable job primitives đang có
- Reuse Redis pubsub/SSE progress pattern đang dùng cho mux jobs
- Add new playlist tables / repository layer

Done when
- Playlist download không còn phụ thuộc client queue để chạy tiếp
- Reload/tab close vẫn không mất tiến trình playlist job trên server
- Progress playlist realtime theo job/item phase
- Item lỗi retry độc lập
- Cancel/resume hoạt động
- Single download hiện tại không regression
