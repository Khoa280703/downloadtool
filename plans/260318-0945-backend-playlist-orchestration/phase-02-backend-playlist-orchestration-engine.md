---
title: "Phase 2 - Backend Playlist Orchestration Engine"
status: complete
---

# Phase 2: Backend Playlist Orchestration Engine

## Context Links
- [plan.md](./plan.md)
- [crates/api/src/routes/batch.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/batch.rs)
- existing mux job worker/control plane

## Overview
- Priority: P0
- Status: complete
- Mục tiêu: backend tự discover playlist, xếp item, extract đúng lúc, và chuẩn bị download URL/artifact cho từng item.
- Completed: Background processor (discovery, sequential processing, mux integration), SSE events

## Key Insights
- `/api/batch` hiện tại chỉ nên tái dùng cho discovery logic, không nên là orchestration boundary cuối cùng.
- Extract đúng lúc cần vẫn là bắt buộc để tránh URL hết hạn.
- Có thể tận dụng mux job system hiện tại thay vì viết pipeline mới cho mux.

## Requirements
- Sau khi tạo playlist job, backend tự discover item list.
- Item processor xử lý tuần tự lúc đầu để giảm rủi ro proxy; có thể tăng concurrency sau.
- Với mỗi item:
  - extract metadata/streams ngay lúc sắp xử lý
  - chọn best stream theo requested quality/mode
  - direct stream nếu đủ
  - create mux job nếu cần video+audio
  - reuse artifact nếu có
- Retry item có kiểm soát, không làm sập cả playlist job.

## Architecture
- Discovery stage:
  - gọi extractor playlist hiện có
  - insert `playlist_job_items`
- Processing stage:
  - background loop hoặc worker poll item `pending`
  - sticky proxy policy: item nào extract bằng proxy nào thì ưu tiên proxy đó cho item đó trong cùng attempt chain
  - just-in-time extract per item
  - emit SSE event mỗi lần item đổi trạng thái
- States gợi ý:
  - playlist: `queued`, `discovering`, `processing`, `ready`, `completed`, `failed`, `cancelled`
  - item: `pending`, `extracting`, `queued_mux`, `muxing`, `ready`, `downloading`, `completed`, `failed`, `cancelled`

## Related Code Files
- Modify:
  - playlist extractor integration
  - mux job orchestration integration
  - audit log
  - admin query layer
- Create:
  - playlist job worker/service
  - playlist SSE event publisher

## Implementation Steps
1. Tách `playlist discovery` ra thành service có thể gọi nội bộ, không chỉ qua SSE route.
2. Tạo playlist job create handler.
3. Tạo processor loop cho playlist items.
4. Reuse logic `pickBestStreams` / degrade detection ở server side hoặc shared module rõ ràng.
5. Khi cần mux, tạo mux job hiện có và attach `mux_job_id` vào item.
6. Khi item sẵn sàng, emit `item_ready` với download endpoint an toàn.
7. Hỗ trợ cancel playlist job và cancel item còn pending.

## Todo List
- [x] Extract discovery service
- [x] Build playlist create API
- [x] Build worker loop
- [x] Attach mux job integration
- [x] Build item-ready event stream
- [x] Build cancel flow

## Success Criteria
- Playlist có thể chạy end-to-end dù frontend reload.
- Không còn frontend loop gọi `extract()` cho từng item.
- Progress item và playlist được backend phát realtime.

## Risk Assessment
- Nếu đưa stream selection logic sang server, cần tránh lệch behavior với single flow.
- Concurrency quá cao sẽ đập proxy; phase đầu nên giữ thấp.

## Security Considerations
- Rate limit ở backend cho create playlist job.
- Kiểm soát access để chỉ requester hợp lệ mới đọc được playlist job của họ nếu cần auth.

## Next Steps
- Sang Phase 3 để frontend bỏ orchestration cũ, chỉ consume job state.

## Unresolved Questions
- Có dùng chung worker process hiện tại hay thêm worker lane riêng cho playlist orchestration?
