# Phase 4: Frontend Migration And Realtime Progress

## Context Links
- [phase-03-playlist-worker-and-item-execution.md](/home/khoa2807/working-sources/downloadtool/plans/260318-0852-playlist-backend-orchestration/phase-03-playlist-worker-and-item-execution.md)
- [\+page.svelte](/home/khoa2807/working-sources/downloadtool/frontend/src/routes/+page.svelte)
- [api.ts](/home/khoa2807/working-sources/downloadtool/frontend/src/lib/api.ts)
- [BatchProgress.svelte](/home/khoa2807/working-sources/downloadtool/frontend/src/components/BatchProgress.svelte)

## Overview
- Priority: P1
- Status: pending
- Mục tiêu: bỏ client orchestration, giữ UI playlist quen thuộc nhưng data source là backend playlist job.

## Key Insights
- UI hiện khá đầy đủ; vấn đề là source of truth nằm ở browser worker.
- `BatchProgress` có thể giữ lại concept nhưng đổi state source.

## Requirements
- Fetch playlist: tạo playlist job thay vì giữ staged queue local.
- Download playlist: chỉ gửi command start/resume.
- Progress realtime: SSE/WebSocket từ backend.
- Single `DownloadBtn` không đổi contract.

## Architecture
- Thêm playlist API client:
  - create job
  - subscribe events
  - pause/resume/cancel
  - request item file/download ticket
- Xóa/retire dần:
  - `playlist-download-worker.ts`
  - runtime limits chỉ dành cho client worker cũ

## Related Code Files
- Modify:
  - [frontend/src/routes/+page.svelte](/home/khoa2807/working-sources/downloadtool/frontend/src/routes/+page.svelte)
  - [frontend/src/lib/api.ts](/home/khoa2807/working-sources/downloadtool/frontend/src/lib/api.ts)
  - [frontend/src/components/BatchProgress.svelte](/home/khoa2807/working-sources/downloadtool/frontend/src/components/BatchProgress.svelte)
  - [frontend/src/stores/batch.ts](/home/khoa2807/working-sources/downloadtool/frontend/src/stores/batch.ts)
- Create:
  - `frontend/src/lib/playlist-job-client.ts`
  - `frontend/src/lib/playlist-job-store.ts`
- Delete later:
  - `frontend/src/lib/playlist-download-worker.ts` or reduce to compatibility shim

## Implementation Steps
1. Tạo playlist job client + store mới.
2. Refactor playlist fetch panel dùng backend snapshot.
3. Refactor progress UI theo parent/item events.
4. Thêm nút pause/resume/cancel.
5. Giữ single tab untouched.

## Todo List
- [ ] Create backend playlist client/store
- [ ] Replace enqueue/download orchestration
- [ ] Wire realtime progress
- [ ] Add pause/resume/cancel controls
- [ ] Remove dead client-worker logic

## Success Criteria
- Reload trang vẫn thấy playlist đang chạy.
- Không còn call `extract()` từ playlist frontend worker.
- Single download vẫn y như cũ.

## Risk Assessment
- UI regressions do state shape đổi mạnh.
- Legacy batch store và playlist store mới chồng chéo.

## Security Considerations
- Không render raw signed URLs trong long-lived frontend state.
- SSE phải owner-scoped.

## Next Steps
- Phase 5 test, admin, rollout.

Unresolved questions:
- UX V1 nên auto-trigger browser download từng item khi ready, hay để user bấm tải từng item/zip sau?
