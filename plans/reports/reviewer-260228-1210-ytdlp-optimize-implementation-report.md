# Implementation Report: Plan 260228-1052-ytdlp-optimize-semaphore-cache-ratelimit

- Plan: `plans/260228-1052-ytdlp-optimize-semaphore-cache-ratelimit/`
- Report time: 2026-02-28 12:10 (+07)
- Scope executed: extractor optimization + API rate limiting + runtime E2E validation
- Status: Completed and pushed to `main`

## 1) Mục tiêu plan

Plan đặt ra 4 nhóm tối ưu:
1. yt-dlp command args + binary path linh hoạt.
2. Giới hạn đồng thời bằng semaphore.
3. Cache in-memory chống gọi lặp/stampede.
4. Rate limiting theo IP cho endpoint `/api/extract`.

Ngoài ra có low-note cần sửa trong phase-03 (đánh số heading + bắt buộc `Clone` cho `ExtractionError`).

## 2) Những gì đã làm (theo code thực tế)

### 2.1 Sửa low-note trong plan

Đã sửa tại `phase-03-url-cache.md`:
- Heading numbering section đúng thứ tự.
- Nêu rõ `ExtractionError: Clone` là bắt buộc với flow `try_get_with + Arc<ExtractionError>`.

File: `plans/260228-1052-ytdlp-optimize-semaphore-cache-ratelimit/phase-03-url-cache.md`

### 2.2 Extractor: dependency + error clone

- Thêm `moka` để dùng async cache.
- Thêm `Clone` cho `ExtractionError`.

Files:
- `crates/extractor/Cargo.toml`
- `crates/extractor/src/types.rs`

### 2.3 Extractor: semaphore + cache + key normalization + binary path

Triển khai trong `crates/extractor/src/ytdlp.rs`:

1. Concurrency guard
- `MAX_CONCURRENT_YTDLP = 10`
- `YTDLP_SEMAPHORE: OnceLock<Arc<Semaphore>>`
- `get_semaphore()` khởi tạo lazy

2. URL cache
- `EXTRACT_CACHE: OnceLock<Cache<String, Arc<VideoInfo>>>`
- TTL 300s, max 500 entries
- Dùng `try_get_with(key, future)` để chống stampede

3. Cache key normalization
- Canonical key cho YouTube:
  - `watch?v=...`
  - `youtu.be/...`
  - `/shorts/...`
- Fallback raw URL cho URL ngoài pattern chuẩn

4. Binary resolution
- `resolve_ytdlp_binary()` dùng `YTDLP_PATH` nếu có, fallback `yt-dlp`

5. Refactor subprocess
- Tách `extract_subprocess(...)` để wrap bởi cache
- `extract_via_ytdlp(...)` trả `VideoInfo` từ `Arc<VideoInfo>`

6. Args update (initial implementation)
- `--socket-timeout 15`
- thêm extractor args YouTube

### 2.4 API: rate limit per IP trên `/api/extract`

Triển khai trong `crates/api/src/main.rs`:

1. Dependency
- `governor = "0.8"`

2. Limiter config
- `Quota::with_period(Duration::from_secs(6))`
- `allow_burst(5)`
- keyed theo `IpAddr`

3. IP extraction strategy
- Ưu tiên header `CF-Connecting-IP`
- fallback `ConnectInfo<SocketAddr>`
- không xác định được IP => 403 JSON

4. Middleware behavior
- Quá hạn mức => 429 JSON
- Trong hạn mức => pass-through

5. Wiring
- Chỉ apply middleware cho route `/api/extract`
- Không apply cho `/api/stream`, `/api/stream/muxed`, `/api/batch`
- Chuyển sang `into_make_service_with_connect_info::<SocketAddr>()`

## 3) Phát hiện trong test runtime và fix ngoài plan gốc

Trong test E2E thực tế, phát hiện lỗi nghiêm trọng:
- Với `player_client=android_embedded,web` thì `yt-dlp` trả lỗi:
  - `Requested format is not available`
  - hoặc `No video formats found`
- Hậu quả: `/api/extract` trả 500, kéo theo không download được.

Fix đã áp dụng:
1. Đổi profile primary sang:
- `youtube:player_client=android,web`

2. Thêm fallback retry nếu primary fail với lỗi format:
- Retry lần 2 không truyền `--extractor-args`
- Nếu fallback vẫn fail mới trả lỗi

File fix:
- `crates/extractor/src/ytdlp.rs`

Đây là deviation hợp lý so với draft plan để đạt mục tiêu reliability thực tế.

## 4) Test đã chạy

### 4.1 Compile/Test cấp package và workspace

Đã chạy thành công:
- `cargo check -p extractor`
- `cargo test -p extractor` (20 passed)
- `cargo check -p api`
- `cargo build -p api`
- `cargo test -p api` (15 passed)
- `cargo check --workspace`
- `cargo test --workspace` (api/extractor/muxer/proxy pass)

### 4.2 Runtime test rate-limit

Trên server local (`PORT=3901`):
- Bắn nhanh 6 request `POST /api/extract` với body invalid (`{}`) để tránh chi phí extractor.
- Kết quả:
  - request #1..#5: 422 (deserialize error)
  - request #6: 429 `{"error":"Rate limit exceeded"}`
- Chờ ~6s bắn lại: về 422 => token refill đúng.
- Test `CF-Connecting-IP` khác nhau:
  - cùng IP: bị 429 ở request #6
  - đổi IP khác: không bị bucket cũ ảnh hưởng

### 4.3 Runtime test download thật (E2E)

Trên server local (`PORT=3902`) sau khi fix fallback:

Case 1: `https://www.youtube.com/watch?v=dQw4w9WgXcQ`
- `POST /api/extract` => 200 success
- `GET /api/stream?url=...` => 200, tải full thành công
- File tải `/tmp/e2e_stream_download.mp4`:
  - size ~11.8MB
  - `file` nhận diện MP4
  - `ffprobe` đọc được duration ~213s

Case 2: `https://www.youtube.com/watch?v=aqz-KE-bpKQ`
- `POST /api/extract` => 200
- `GET /api/stream` với `Range: bytes=0-2097151` => 200, tải 2MB đầu thành công

Kết luận: luồng extract + download hoạt động end-to-end.

## 5) Commit/Push

### Commit 1
- Hash: `01a9da4`
- Message: `feat(api,extractor): optimize ytdlp flow with cache and ip rate limit`
- Nội dung chính:
  - add governor/moka
  - implement limiter + cache + semaphore + key normalization
  - update plan phase-03 doc

### Commit 2
- Hash: `0328db8`
- Message: `fix(extractor): fallback yt-dlp args when youtube format lookup fails`
- Nội dung chính:
  - đổi primary client args
  - thêm retry fallback khi lỗi format

Push:
- `main -> origin/main` thành công cho cả 2 commit.

## 6) Danh sách file đã thay đổi trong phạm vi plan

1. `Cargo.lock`
2. `crates/extractor/Cargo.toml`
3. `crates/extractor/src/types.rs`
4. `crates/extractor/src/ytdlp.rs`
5. `crates/api/Cargo.toml`
6. `crates/api/src/main.rs`
7. `plans/260228-1052-ytdlp-optimize-semaphore-cache-ratelimit/phase-03-url-cache.md`

## 7) Kết quả so với mục tiêu plan

- Phase 1 (args + binary path): done, và có hardening thêm fallback runtime.
- Phase 2 (semaphore): done.
- Phase 3 (moka cache + stampede prevention): done.
- Phase 4 (rate limiting `/api/extract`): done.
- Verify build/test + runtime: done.

## 8) Rủi ro còn lại và khuyến nghị

1. `/api/extract` hiện có thể trả rất ít format (một số video chỉ còn progressive format 18) do behavior từ upstream YouTube/yt-dlp/profile client; tuy không block download, nhưng giảm lựa chọn chất lượng.
2. Rate limit state in-memory (governor keyed store) sẽ reset khi restart process; phù hợp hiện tại, nhưng cần external store nếu scale ngang nhiều instance.
3. `403` branch (không xác định IP) là defensive; trong wiring chuẩn có `ConnectInfo`, branch này hiếm khi kích hoạt local.

Khuyến nghị tiếp:
1. Bổ sung integration test tự động cho `/api/extract` + `/api/stream` bằng sample URL có độ ổn định cao.
2. Thêm metric log cho cache hit/miss và fallback retry count để quan sát production.
3. Nếu cần nhiều format hơn, benchmark thêm các client profile khác theo môi trường thực tế.

## 9) Unresolved questions

- Không còn unresolved blocker cho plan này.
