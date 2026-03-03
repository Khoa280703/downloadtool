# Stress Test Report - Architecture, Methodology, Findings

Date: 2026-03-02  
Project: `downloadtool`  
Scope: `/api/extract`, `/api/stream`, `/api/stream/muxed`, `/api/batch`

## 1) Executive Summary

- Kết quả `30%` và `1%` là **thực**, đến từ run stress mạnh hợp lệ.
- `/api/extract` ổn định 100% ở tải cao.
- `/api/stream` giảm mạnh tỷ lệ thành công khi tăng kích thước range và concurrency.
- `/api/stream/muxed` gần như sập ở tải cao (khoảng 1.75% thành công).
- Root-cause chính: upstream YouTube/CDN trả `401 Unauthorized` dưới tải lớn; muxed chịu tác động nặng hơn vì cần 2 stream + preflight.

## 2) Current Architecture (as-is)

### 2.1 API server

- Framework: Axum
- Main entry: `crates/api/src/main.rs`
- Protected routes:
  - `POST /api/extract`
  - `GET /api/stream`
  - `GET /api/stream/muxed`
  - `GET /api/batch`
- Auth layer: JWT middleware gắn ở `protected_api`.
- Rate limit:
  - Chỉ áp dụng cho `/api/extract`.
  - Governor keyed by IP, quota `5 requests / 6s`, burst `5`.
  - File: `crates/api/src/main.rs` line ~46-116.

### 2.2 Extract flow

- `POST /api/extract`:
  - Validate URL YouTube.
  - Gọi extractor runtime.
  - Trả metadata + danh sách formats + `selected_stream_url`.
- File: `crates/api/src/routes/extract.rs`.

### 2.3 Stream flow

- `GET /api/stream`:
  - Nếu có `Range` từ browser: proxy single-range.
  - Nếu URL có `clen`: tải chunked 9.5MB để bypass throttle.
  - Nếu không: fallback single request.
- File: `crates/api/src/routes/stream.rs` line ~87-275.

### 2.4 Muxed flow

- `GET /api/stream/muxed`:
  - Validate URL.
  - Reject WebM input sớm (`422`) cho cả video/audio.
  - `StreamFetcher::fetch_both` tải song song video+audio.
  - `remux_streams` tạo output fMP4.
  - Preflight chunk đầu trước khi commit `200`.
  - Preflight timeout hiện tại: `8s`.
- File: `crates/api/src/routes/stream.rs` line ~277-395.

### 2.5 Proxy/Anti-bot layer

- AntiBotClient:
  - `MAX_RETRIES=3`, `RETRY_DELAY=200ms`.
  - `connect_timeout=30s`.
  - Với CDN URL `googlevideo.com`, gặp `403/429` thì fail fast (không rotate proxy).
- File: `crates/proxy/src/anti_bot.rs`.

### 2.6 StreamFetcher (chunked CDN)

- Chunk size: `9.5MB`.
- `CHUNK_MAX_RETRIES=3`.
- `CHUNK_REQUEST_TIMEOUT=30s`.
- `CHUNK_READ_IDLE_TIMEOUT=20s`.
- `PREFETCH_AWAIT_TIMEOUT=5s`.
- File: `crates/muxer/src/stream_fetcher.rs`.

## 3) Test Methodology

### 3.1 Reusable script

- Script mới: `scripts/stress-test-all-endpoints.sh`
- Profile: `PROFILE=strong`
- Endpoints covered:
  - `/api/extract`
  - `/api/stream` (4KB range, 1MB range)
  - `/api/stream/muxed` (video-only mp4 + audio-only m4a)
  - `/api/batch`
- Output:
  - `summary.tsv`
  - `batch-summary.tsv`

### 3.2 Workload matrix (strong)

- `extract-c120-n12000` timeout 12s
- `stream4k-c150-n12000` timeout 15s
- `stream1m-c100-n5000` timeout 30s
- `muxed-c40-n2000-t30`
- `muxed-c80-n2000-t30`
- `batch-c20-n2000-t120`
- `batch-c40-n2000-t120`

### 3.3 Important note about one invalid run

- Run `/tmp/stress-reusable-strong-20260302-100524` không hợp lệ do bug script (worker shell không nhận env URL) nên ra toàn `400`.
- Bug đã fix bằng `export API_BASE VIDEO_URL PLAYLIST_URL STREAM_URL VIDEO_ONLY_URL AUDIO_M4A_URL`.
- Run hợp lệ dùng để kết luận: `/tmp/stress-reusable-strong-20260302-100756`.

## 4) Results (Valid Run)

Source files:
- `/tmp/stress-reusable-strong-20260302-100756/summary.tsv`
- `/tmp/stress-reusable-strong-20260302-100756/batch-summary.tsv`

### 4.1 HTTP endpoints

- `/api/extract` `c120 n12000`: `12000/12000` = `100%`.
- `/api/stream` 4KB `c150 n12000`: `10298/12000` = `85.82%`.
  - Fail: `502=1702`.
- `/api/stream` 1MB `c100 n5000`: `1707/5000` = `34.14%`.
  - Fail: `502=3293`.
- `/api/stream/muxed` `c40 n2000 t30`: `35/2000` = `1.75%`.
  - Fail: `502=1960`, `504=5`.
- `/api/stream/muxed` `c80 n2000 t30`: `35/2000` = `1.75%`.
  - Fail: `502=1965`.

### 4.2 Batch SSE

- `batch-c20-n2000`: HTTP `200` toàn bộ, nhưng `done_ok=1589/2000` = `79.45%`.
- `batch-c40-n2000`: HTTP `200` toàn bộ, `done_ok=1226/2000` = `61.30%`.

## 5) Failure Diagnostics

### 5.1 Stream failure probe

- Probe: concurrency 150, total 3000, 4KB range.
- File: `/tmp/stress-probe-stream-fail-1772422087.tsv`
- Fail sample: `502` body chứa
  - `Failed to fetch stream ... 401 Unauthorized ... googlevideo.com`
- Kết luận: fail chủ yếu do upstream CDN trả `401` dưới tải cao.

### 5.2 Muxed failure probe

- Probe: concurrency 80, total 600, timeout 30s.
- File: `/tmp/stress-probe-muxed-fail-1772422184.tsv`
- Distribution:
  - `502 = 519`
  - `504 = 8` (`Mux preflight timed out after 8s`)
  - `000 = 73` (client-side transfer abort/timeout signatures, có `curl(18)`).
- Fail sample `502`: chứa upstream `401 Unauthorized` từ `googlevideo.com`.

## 6) Why ~30% and ~1%

- `~30%` đến từ `/api/stream` 1MB case: `1707/5000 = 34.14%`.
- `~1%` đến từ `/api/stream/muxed`: `35/2000 = 1.75%`.
- Muxed thấp hơn stream vì:
  - Mỗi request mux cần 2 upstream streams.
  - Có preflight timeout `8s`.
  - Bị cộng gộp lỗi upstream (401) + timeout + stream interruption.

## 7) Key Problems (Current State)

- No global admission control/backpressure cho `/api/stream` và `/api/stream/muxed`.
- Không có queue/worker lane cho tải nặng (muxed/batch) nên burst đi thẳng vào upstream.
- Retry hiện tại chưa có cơ chế refresh stream URL khi gặp `401`.
- `MUX_PREFLIGHT_TIMEOUT=8s` quá chặt trong tải cao.
- Batch HTTP `200` không đồng nghĩa thành công nghiệp vụ (`done_ok` giảm theo tải).

## 8) Recommended Fix Path

### P0 (high impact, short cycle)

- Thêm global semaphore cho `/api/stream` và `/api/stream/muxed`.
- Khi quá tải, trả `503 + Retry-After` sớm.
- Tăng `MUX_PREFLIGHT_TIMEOUT` và tune retry window.
- Thêm fallback re-extract URL một lần khi gặp `401`/`403` từ CDN.
- Bổ sung metrics bắt buộc:
  - `upstream_401_total`
  - `upstream_403_total`
  - `mux_preflight_timeout_total`
  - `stream_5xx_total` by cause
  - success ratio theo endpoint.

### P1 (architecture)

- Tách lane:
  - Realtime lane: `/api/stream` (best-effort, guarded by semaphore).
  - Queue lane: `batch` và `muxed` nặng qua job queue + worker.
- Có thể dùng RabbitMQ cho queue lane nếu cần durability + retry semantics rõ.

## 9) Reproduce Commands

```bash
cd /home/khoa2807/working-sources/downloadtool
API_BASE='http://127.0.0.1:3068' PROFILE='strong' RUN_BATCH='1' scripts/stress-test-all-endpoints.sh
```

## 10) Unresolved Questions

- Ngưỡng concurrency tối ưu thực tế cho production target (SLO cụ thể) là bao nhiêu?
- Có chấp nhận chuyển `muxed` sang async job hoàn toàn (không realtime) không?
- Có triển khai proxy/IP pool production-grade hay giữ single egress IP?
