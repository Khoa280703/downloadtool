---
title: "Phase 3 - Frontend Thin Client Migration"
status: complete
---

# Phase 3: Frontend Thin Client Migration

## Context Links
- [plan.md](./plan.md)
- [frontend/src/routes/+page.svelte](/home/khoa2807/working-sources/downloadtool/frontend/src/routes/+page.svelte)
- [frontend/src/lib/api.ts](/home/khoa2807/working-sources/downloadtool/frontend/src/lib/api.ts)
- [frontend/src/lib/playlist-download-worker.ts](/home/khoa2807/working-sources/downloadtool/frontend/src/lib/playlist-download-worker.ts)

## Overview
- Priority: P0
- Status: complete
- Mục tiêu: frontend bỏ vai trò điều phối playlist; chỉ tạo job, theo dõi SSE, và lưu file khi item ready.
- Completed: Frontend thin client (proxy routes, playlist-job-api.ts, page refactored to use backend SSE)

## Key Insights
- UI hiện tại đã có progress/list panel khá đầy đủ; có thể giữ UI, chỉ thay source state.
- `playlist-download-worker.ts` là trung tâm orchestration cũ; cần thu nhỏ hoặc thay thế.
- Browser save vẫn phải xử lý ở client vì đây là boundary của web platform.

## Requirements
- `Fetch playlist` -> tạo backend playlist job.
- UI subscribe SSE từ backend playlist job.
- Khi item ready:
  - nếu có FSAA: tự ghi file theo folder đã chọn
  - nếu anchor fallback: vẫn sequential, có thông báo rõ browser policy
- Giữ download progress dễ hiểu với user.

## Architecture
- Frontend state mới:
  - `playlistJobId`
  - `playlistStatus`
  - `items[]` từ SSE/backend snapshot
- Xóa hoặc thu nhỏ client queue:
  - không còn `pendingQueue`, `readyQueue`, `activeEntries` là nguồn sự thật
- Save layer:
  - giữ `saveDownload(...)`
  - thêm contract mới để mark client-side `download_started` / `download_finished` nếu cần

## Related Code Files
- Modify:
  - `frontend/src/routes/+page.svelte`
  - `frontend/src/lib/api.ts`
  - `frontend/src/stores/batch.ts`
  - `frontend/src/components/BatchProgress.svelte`
- Remove/refactor:
  - `frontend/src/lib/playlist-download-worker.ts`

## Implementation Steps
1. Tạo playlist job client API.
2. Tạo SSE subscriber cho playlist job events.
3. Refactor batch store để nhận item state từ backend events.
4. Thay `enqueueDownload(...)` bằng `create playlist job` + `listen`.
5. Khi item `ready`, gọi save layer theo mode FSAA/anchor.
6. Nếu cần, gửi ack về backend khi browser đã bắt đầu save để analytics đúng hơn.

## Todo List
- [x] Add playlist job client API
- [x] Add SSE subscriber
- [x] Refactor batch store
- [x] Replace worker queue orchestration
- [x] Keep FSAA path working
- [x] Test anchor fallback behavior

## Success Criteria
- Refresh trang có thể resume xem progress playlist job.
- UI không còn tự `extract()` per item.
- Download single không bị ảnh hưởng.

## Risk Assessment
- Anchor fallback nhiều file có thể bị browser block; cần UX warning rõ.
- Nếu vẫn cần local folder permission, phải giữ flow chọn folder hợp lệ.

## Security Considerations
- Không trust client cho item state; client chỉ render state backend gửi xuống.

## Next Steps
- Sang Phase 4 để rollout, rate limit, metrics, admin tracking.

## Unresolved Questions
- Có cần cho user vào lại job cũ qua URL `/playlist-jobs/:id` không?
