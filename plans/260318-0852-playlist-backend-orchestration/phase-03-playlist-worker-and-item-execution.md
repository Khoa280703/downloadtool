# Phase 3: Playlist Worker And Item Execution

## Context Links
- [phase-02-backend-playlist-control-plane.md](/home/khoa2807/working-sources/downloadtool/plans/260318-0852-playlist-backend-orchestration/phase-02-backend-playlist-control-plane.md)
- [playlist-download-worker.ts](/home/khoa2807/working-sources/downloadtool/frontend/src/lib/playlist-download-worker.ts)
- [mux_pipeline.rs](/home/khoa2807/working-sources/downloadtool/crates/worker/src/mux_pipeline.rs)
- [routes/jobs.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/jobs.rs)

## Overview
- Priority: P1
- Status: pending
- Mục tiêu: chuyển queue, retry, just-in-time extract, execution path selection sang worker/backend.

## Key Insights
- Logic chọn stream hiện đang đúng chỗ nhưng nằm sai tầng: frontend.
- Worker mux hiện đã có late-extract refresh logic, nên có thể reuse tư duy đó cho playlist items.

## Requirements
- Per-item chỉ extract lúc sắp chạy.
- Retry per-item rõ ràng, không retry cả playlist.
- Cancel parent phải dừng item đang chạy.
- Resume parent phải tiếp tục từ item còn pending/failed-retriable.

## Architecture
- Playlist worker loop:
  1. claim next runnable item
  2. extract source video just-in-time
  3. chọn `execution_kind`
  4. nếu `mux`: create/reuse `mux_job`
  5. nếu `direct_copy`: thêm worker path tải thẳng 1 stream vào artifact storage
  6. nếu `direct_stream`: chỉ lưu metadata đủ để refresh-on-demand khi user bấm tải
- Parent concurrency V1: `1` item / playlist job.
- System concurrency tổng do worker config quyết định.

## Related Code Files
- Modify:
  - [crates/worker/src/main.rs](/home/khoa2807/working-sources/downloadtool/crates/worker/src/main.rs)
  - [crates/worker/src/job_runner.rs](/home/khoa2807/working-sources/downloadtool/crates/worker/src/job_runner.rs)
  - [crates/worker/src/mux_pipeline.rs](/home/khoa2807/working-sources/downloadtool/crates/worker/src/mux_pipeline.rs)
  - [crates/extractor/src/lib.rs](/home/khoa2807/working-sources/downloadtool/crates/extractor/src/lib.rs)
- Create:
  - `crates/worker/src/playlist_job_runner.rs`
  - `crates/worker/src/direct_copy_pipeline.rs`
  - `crates/api/src/services/playlist_download_ticket_service.rs`

## Implementation Steps
1. Port logic `pickBestStreams` sang backend/shared crate.
2. Thêm worker runner cho playlist item.
3. Reuse `mux_jobs` cho mux path.
4. Thêm direct-copy artifact path cho combined stream/audio-only để tránh lệ thuộc browser orchestration.
5. Với direct-stream V1, chỉ cho phép on-demand late refresh khi user click item.
6. Publish progress event mỗi lần item đổi phase/status.

## Todo List
- [ ] Move stream-selection logic server-side
- [ ] Add playlist item runner
- [ ] Reuse mux jobs
- [ ] Add direct-copy path
- [ ] Add parent/item progress events

## Success Criteria
- Playlist không cần frontend loop `extract()` nữa.
- Một item fail không kéo sập cả playlist.
- Resume hoạt động sau reload.

## Risk Assessment
- Scope direct-copy path lớn hơn dự kiến.
- Mapping item progress -> parent progress có thể nhiễu nếu nested job events không gọn.

## Security Considerations
- Stream URL chỉ sống nội bộ server hoặc ticket ngắn hạn.
- Không expose raw upstream signed URLs lâu dài cho client.

## Next Steps
- Phase 4 chuyển frontend sang subscribe playlist job.

Unresolved questions:
- V1 có nên bỏ hẳn `direct_stream` trong playlist để giảm scope, ép mọi item thành artifact?
