## Context Links
- `crates/api/src/routes/jobs.rs`
- `crates/api/src/services/job_control_plane.rs`
- `crates/worker/*`
- `frontend/src/lib/playlist-download-worker.ts`

## Overview
Priority: high
Status: proposed
Mục tiêu phase này: backend nắm sequencing, retry, progress; giữ just-in-time extract per item.

## Key Insights
- Vấn đề URL hết hạn là thật, nên worker chỉ extract khi item sắp chạy.
- Realtime progress đã có pattern tốt ở mux jobs: Redis pubsub + SSE.
- Không cần làm concurrent phức tạp ngay; phase đầu nên xử lý tuần tự per playlist job.

## Requirements
- Worker lấy playlist item `pending` tiếp theo.
- Với mỗi item:
  - extract just-in-time
  - chọn stream theo mode/quality snapshot
  - direct stream hoặc create mux job / direct pipeline
  - update progress + artifact refs
- Retry per item độc lập.
- Cancel/resume hợp lý.

## Architecture
- Playlist worker loop:
1. lease playlist job
2. pick next pending item
3. extract just-in-time
4. run single-item download flow
5. persist item status/result
6. emit aggregate progress
- Phase đầu: `playlist_worker_max_concurrent_per_job = 1`
- Phase sau mới xét concurrency > 1

## Related Code Files
- Modify:
  - `crates/worker/*`
  - `crates/api/src/services/job_control_plane.rs` if sharing reusable primitives
- Create:
  - `crates/worker/src/playlist_worker.rs`
  - `crates/worker/src/playlist_executor.rs`
  - playlist progress event types / repository helpers

## Implementation Steps
1. Create worker command path for playlist jobs.
2. Reuse current stream selection logic conceptually, nhưng move vào backend shared module.
3. Implement per-item retry policy:
   - extract retries
   - mux/download retries
   - terminal failure after max attempts
4. Emit progress:
   - job phase
   - current item index/total
   - item title/video_id
   - item phase
   - completed/failed counts
5. Implement cancel token checks before each item and within long waits.
6. Implement resume to continue from remaining pending/failed-retryable items.

## Todo List
- [ ] Shared stream selection logic on backend
- [ ] Playlist worker leasing
- [ ] Per-item retry policy
- [ ] Aggregate SSE payload shape
- [ ] Cancel/resume semantics locked

## Success Criteria
- Playlist keeps running if user closes tab
- Expired URLs avoided by just-in-time extract
- One failed item does not kill whole playlist unless policy says so

## Risk Assessment
- Reusing frontend selection logic naively may create divergence
- If each item still goes through browser save semantics, architecture remains half-done

## Security Considerations
- Validate upstream URLs before any mux/direct action
- Keep owner-bound access to generated artifacts

## Next Steps
- Frontend migration to playlist job UI

## Unresolved Questions
- Giai đoạn đầu có cần artifact zip/tải cả playlist, hay chỉ giữ per-item downloads?
