## Context Links
- `frontend/src/routes/+page.svelte`
- `frontend/src/lib/api.ts`
- `frontend/src/lib/playlist-download-worker.ts`
- `frontend/src/stores/batch.ts`
- `frontend/src/components/BatchProgress.svelte`

## Overview
Priority: high
Status: proposed
Mục tiêu phase này: frontend chuyển từ local queue sang thin client cho playlist jobs.

## Key Insights
- UI hiện tại đã có playlist panel, selection UI, progress list.
- Cần giữ UX quen thuộc, chỉ thay nguồn dữ liệu.
- Không nên xóa ngay UI cũ trước khi playlist job API ổn.

## Requirements
- Fetch playlist vẫn hiển thị item list để user chọn.
- Start download sẽ tạo playlist job trên backend, không enqueue local worker nữa.
- UI subscribe progress SSE theo playlist job id.
- Reload page khôi phục job đang chạy gần nhất nếu có.

## Architecture
- Giữ `fetch playlist metadata` riêng, hoặc chuyển `POST /api/playlist-jobs` trả luôn preview list.
- Thêm client API:
  - create playlist job
  - get playlist job
  - subscribe playlist job events
  - cancel/resume playlist job
- `BatchProgress` đọc từ server state thay vì local worker state.

## Related Code Files
- Modify:
  - `frontend/src/routes/+page.svelte`
  - `frontend/src/lib/api.ts`
  - `frontend/src/stores/batch.ts`
  - `frontend/src/components/BatchProgress.svelte`
- Delete later:
  - `frontend/src/lib/playlist-download-worker.ts` after migration complete

## Implementation Steps
1. Add playlist job client API and types.
2. Replace `enqueueDownload(...)` flow with `createPlaylistJob(...)`.
3. Replace local progress mutations by server-driven store updates.
4. Add cancel/resume buttons in playlist toolbar.
5. Add reload recovery using stored active `playlist_job_id`.
6. Keep single-video `DownloadBtn` untouched.

## Todo List
- [ ] Playlist job client API
- [ ] SSE subscription for playlist jobs
- [ ] UI actions: start/cancel/resume
- [ ] Reload recovery
- [ ] Legacy worker removed or feature-flagged

## Success Criteria
- Frontend no longer orchestrates playlist sequencing
- Reload does not orphan playlist UX
- Single download flow unchanged

## Risk Assessment
- UI state migration may break current selected-count/progress rendering
- If both legacy worker and new job flow coexist too lâu, bug surface doubles

## Security Considerations
- No hidden admin-only playlist controls leaked to public UI
- Job ids must remain owner-scoped

## Next Steps
- Testing + rollout + cleanup

## Unresolved Questions
- Có cần cho user chọn lại subset item sau khi job đã tạo, hay khóa selection snapshot từ lúc start?
