**Context Links**
- [plan.md](plan.md)
- [phase-02-implement-backend-playlist-control-plane.md](phase-02-implement-backend-playlist-control-plane.md)
- [worker main](/home/khoa2807/working-sources/downloadtool/crates/worker/src/main.rs)
- [job_runner.rs](/home/khoa2807/working-sources/downloadtool/crates/worker/src/job_runner.rs)
- [mux_pipeline.rs](/home/khoa2807/working-sources/downloadtool/crates/worker/src/mux_pipeline.rs)

**Overview**
- Priority: critical
- Status: proposed
- Brief: biến playlist item thành server-managed execution thật sự

**Key Insights**
- Just-in-time extract phải xảy ra ở worker lúc item bắt đầu chạy.
- Playlist backend path nên luôn ra artifact để browser chỉ việc tải.
- Reuse artifact quan trọng hơn tối ưu fancy progress ở phase đầu.

**Requirements**
- Worker claim item pending.
- Extract formats đúng lúc cần.
- Nếu combined stream có audio: tải thẳng artifact.
- Nếu video/audio tách rời: spawn mux build hoặc thực hiện mux path dùng primitives hiện có.
- Retry per item với backoff và classify lỗi.
- Cập nhật progress item + aggregate playlist job.

**Architecture**
- Worker loop mới hoặc mở rộng worker hiện tại.
- Item executor:
1. resolve source/watch URL
2. extract formats just-in-time
3. chọn stream theo mode/quality
4. reuse artifact nếu đã có
5. direct-download-to-artifact hoặc mux-to-artifact
6. publish progress
- Failure policy:
  - temporary => retry item
  - permanent => failed item, playlist tiếp tục

**Related Code Files**
- Modify:
  - `crates/worker/src/main.rs`
  - `crates/worker/src/job_runner.rs`
  - `crates/worker/src/storage_factory.rs`
- Create:
  - `crates/worker/src/playlist_job_runner.rs`
  - `crates/worker/src/playlist_progress_publisher.rs`
  - `crates/worker/src/direct_stream_to_artifact.rs`
- Delete:
  - none

**Implementation Steps**
1. Add worker consume path cho playlist items.
2. Implement artifact-first direct stream pipeline.
3. Reuse existing mux pipeline khi cần mux.
4. Publish per-item and aggregate progress.
5. Add retry/backoff/error classification.

**Todo List**
- [ ] Add playlist item worker loop
- [ ] Add direct-to-artifact flow
- [ ] Add progress publisher
- [ ] Add retry policy

**Success Criteria**
- Playlist item chạy được không cần browser orchestration.
- URL hết hạn giảm vì extract xảy ra ngay lúc item chạy.

**Risk Assessment**
- Direct stream to artifact path mới dễ lỗi edge case với upstream refresh.

**Security Considerations**
- Validate source URL và proxy assignment.
- Không trust filename/input từ client.

**Next Steps**
- Sang frontend migration.

**Unresolved questions**
- Có nên reuse hẳn `mux_jobs` cho item cần mux hay giữ playlist item runner tự xử lý tất cả.
