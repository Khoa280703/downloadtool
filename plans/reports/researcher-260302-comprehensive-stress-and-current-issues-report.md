# Comprehensive Stress Test + Current Issues Report

Date: 2026-03-02  
Project: `downloadtool`  
Scope: `/api/extract`, `/api/stream`, `/api/stream/muxed`, `/api/batch`  
Report focus: tổng hợp toàn bộ kết quả test đã chạy + vấn đề tồn đọng hiện tại

## 1) Executive summary

- Kết quả stress đã xác nhận 3 trạng thái khác nhau theo giai đoạn:
  - **Giai đoạn baseline strong**: `/api/extract` ổn, `/api/stream` giảm theo tải, `/api/stream/muxed` rất thấp.
  - **Giai đoạn có backpressure**: lỗi chuyển nhiều sang `503` fail-fast (hệ thống tự bảo vệ tốt hơn).
  - **Giai đoạn refresh-auth retry mới nhất**: dưới tải rất cao, vẫn xuất hiện `502` nặng với body `401 Unauthorized` từ `googlevideo`.
- Fix retry/refresh đã hoạt động ở code path, nhưng **không đủ triệt tiêu** auth fail khi concurrency + volume vượt ngưỡng upstream.
- Nút thắt lớn nhất vẫn là upstream YouTube/CDN policy (IP/session/token/client context), đặc biệt cho luồng long-lived và muxed.

## 2) Kiến trúc hiện tại (phần liên quan tải)

### 2.1 `/api/extract`
- Input URL -> extractor -> metadata/formats.
- Có thể bị chặn bot-check từ upstream tại thời điểm extract (từng gặp 500 với message sign-in).

### 2.2 `/api/stream`
- Hỗ trợ range/single/chunked.
- Có retry refresh URL từ `source_url + format_id` khi lỗi auth upstream.
- Có max attempts qua env:
  - `STREAM_URL_REFRESH_MAX_ATTEMPTS` (default 3).

### 2.3 `/api/stream/muxed`
- Fetch song song video/audio -> remux fMP4 -> preflight chunk đầu.
- Có retry refresh URL ở 2 điểm:
  - khi `fetch_both` fail auth-like.
  - khi `preflight` fail auth-like.
- Env:
  - `MUX_URL_REFRESH_MAX_ATTEMPTS` (default 3).
  - `MUX_PREFLIGHT_TIMEOUT_SECS`.

### 2.4 Admission control
- Backpressure semaphore đang có trong route stream/muxed.
- Dưới overload có thể trả `503` sớm thay vì dồn vào upstream.

## 3) Thay đổi mới nhất đã implement

File: `crates/api/src/routes/stream.rs`

- Thêm config + state:
  - `STREAM_URL_REFRESH_MAX_ATTEMPTS_DEFAULT`
  - `MUX_URL_REFRESH_MAX_ATTEMPTS_DEFAULT`
  - OnceLock getters đọc env tương ứng.
- Thêm detector auth-like:
  - parse theo status nếu có.
  - fallback parse theo message (`401 Unauthorized`, `403 Forbidden`, ...).
- `/api/stream`:
  - `proxy_single_request` đổi từ refresh 1 lần -> refresh nhiều lần (max attempts).
  - `chunked_stream` đổi từ refresh 1 lần -> refresh nhiều lần (max attempts).
- `/api/stream/muxed`:
  - retry-refresh nhiều vòng cho `fetch_both`.
  - retry-refresh nhiều vòng cho lỗi preflight trước commit 200.

Validation:
- `cargo check -p api`: pass.
- `cargo test -p api -- --nocapture`: pass (`22 passed`).

## 4) Test methodology + artifacts

### 4.1 Baseline strong run (trước nhánh refresh mới)
- Artifact:
  - `/tmp/stress-rootcause-20260302-145152/summary.tsv`
  - `/tmp/stress-rootcause-20260302-145152/batch-summary.tsv`
- Profile:
  - `/api/extract`: c120 n12000
  - `/api/stream` 4KB: c150 n12000
  - `/api/stream` 1MB: c100 n5000
  - `/api/stream/muxed`: c40 n2000, c80 n2000

### 4.2 Post-refresh probe run (sau patch mới)
- Artifact:
  - `/tmp/stress-refresh-probe-20260302-152704/summary.tsv`
- Profile:
  - `/api/stream` 4KB: c160 n6000
  - `/api/stream/muxed`: c80 n1600

### 4.3 Probe phụ để loại trừ nhiễu
- `n300, c80` stream range 4KB:
  - `/tmp/stream-fail-sample-1772440819/code-counts.txt`
  - `/tmp/stream-fail-sample-1772440908/code-counts.txt`
- `n1000, c160` stream range 4KB:
  - `/tmp/stream-fail-c160-n1000-1772440953/code-counts.txt`
  - body fail: `/tmp/stream-fail-c160-n1000-1772440953/fail-bodies.txt`
- `n3000, c160` stream range 4KB:
  - `/tmp/stream-count-1772440836/code-counts.txt`

## 5) Kết quả chi tiết

### 5.1 Baseline strong (rootcause run)

Nguồn: `/tmp/stress-rootcause-20260302-145152/summary.tsv`

- `/api/extract` c120 n12000
  - `12000/12000` HTTP 200 (100%)
  - p95 `0.008s`
- `/api/stream` 4KB c150 n12000
  - `9817/12000` HTTP 200 (81.81%)
  - `2183` lỗi `5xx` (18.19%)
  - p95 `6.06s`
- `/api/stream` 1MB c100 n5000
  - `5000/5000` HTTP 200 (100%)
  - nhưng có `rc28=6`, `rc18=3` (timeout/partial transfer phía client)
  - p95 `6.25s`
- `/api/stream/muxed` c40 n2000
  - `71/2000` HTTP 200 (3.55%)
  - `1929` lỗi `5xx` (96.45%)
- `/api/stream/muxed` c80 n2000
  - `35/2000` HTTP 200 (1.75%)
  - `1965` lỗi `5xx` (98.25%)

### 5.2 Post-refresh probe (latest)

Nguồn: `/tmp/stress-refresh-probe-20260302-152704/summary.tsv`

- `/api/stream` 4KB c160 n6000
  - `202/6000` HTTP 200 (3.37%)
  - `5798` lỗi `5xx` (96.63%)
  - p95 `4.47s`
- `/api/stream/muxed` c80 n1600
  - `1600/1600` HTTP 200
  - nhưng `rc28=1528`, `rc18=65`, `rc0=7`
  - p95 `30.00s` (sát hard timeout)

### 5.3 Probe phụ (latest)

- `stream n300 c80` x2 lần: `200=300`, `502=0` (100% success ở tải vừa).
- `stream n1000 c160`: `200=341`, `502=659` (34.1% success).
- `stream n3000 c160`: `200=325`, `502=2675` (10.83% success).

Inference:
- Hệ thống có **ngưỡng gãy rõ theo load** (threshold behavior), không phải fail ngẫu nhiên tuyến tính.

## 6) Phân tích nguyên nhân gốc

### 6.1 Bằng chứng trực tiếp

- Body lỗi `502` của `/api/stream` ghi rõ upstream:
  - `HTTP status client error (401 Unauthorized)` từ `googlevideo`.
- Artifact:
  - `/tmp/stream-fail-c160-n1000-1772440953/fail-bodies.txt`

### 6.2 Vì sao refresh URL vẫn không cứu được dưới tải rất cao

- Refresh có chạy, nhưng concurrency/volume quá cao làm URL mới cũng nhanh chóng rơi vào auth fail.
- Đây là pattern của upstream policy theo tổ hợp:
  - IP/session consistency
  - token validity window
  - client fingerprint/profile
  - request burst pattern
- Khi vượt ngưỡng risk-score phía CDN, retry-refresh chỉ kéo dài được ngắn, không đảo ngược xu hướng fail.

### 6.3 Vì sao muxed nhìn thấy `200` nhưng UX vẫn fail

- HTTP status `200` chỉ nói request được accept và stream bắt đầu.
- Với muxed long-lived:
  - client timeout (`rc28`) hoặc transfer closed (`rc18`) xảy ra trong khi stream đang chạy.
  - nên success business không đồng nghĩa với success HTTP.

## 7) Vấn đề tồn đọng hiện tại

1. `/api/stream` vẫn tụt mạnh khi đi vào vùng tải rất cao (c160, n lớn), gốc do upstream `401`.
2. `/api/stream/muxed` chưa có reliability tốt cho long-lived under stress; timeout client rất cao.
3. Lớp observability chưa đủ chi tiết để nhìn real-time theo job/session key (IP/proxy/client-profile/token-age).
4. Chưa có cơ chế async job cho muxed; realtime mux làm tail latency và retry cost cao.
5. Chưa có externalized state/queue nếu scale ngang nhiều instance.

## 8) Tác động vận hành

- Ở tải vừa: có thể ổn (ví dụ n300 c80 stream đạt 100%).
- Ở tải cao burst: tỷ lệ fail tăng rất nhanh theo ngưỡng, đặc biệt stream small-range và muxed long-lived.
- Nếu không có retry orchestration ở client + queue ở server, UX production sẽ dao động mạnh theo traffic burst.

## 9) Khuyến nghị ưu tiên

### P0 (làm ngay)
1. Thêm metric bắt buộc theo endpoint:
   - `upstream_401_total`, `upstream_403_total`
   - `stream_refresh_attempt_total`, `stream_refresh_success_total`
   - `mux_preflight_fail_total`, `mux_client_timeout_total`
2. Tách SLO technical vs business success:
   - HTTP 200 rate
   - completion rate (download complete)
3. Client retry policy thống nhất cho `503/502/504` + tôn trọng `Retry-After`.

### P1
1. Chuyển `/api/stream/muxed` sang async job queue (202 + polling/SSE result) thay vì giữ realtime sync.
2. Admission control theo 2 tầng:
   - global concurrency
   - per-IP/per-session budget

### P2
1. Benchmark sticky proxy strategy (nếu dùng proxy): 1 job = 1 proxy = 1 session context.
2. External store cho rate/queue state khi scale multi-instance.

## 10) Trạng thái code và script liên quan report

- Code thay đổi chính:
  - `crates/api/src/routes/stream.rs`
- Script stress:
  - `scripts/stress-test-all-endpoints.sh`
  - `scripts/stress-test-refresh-probe.sh`

## 11) Kết luận cuối

- Đã có tiến bộ về defensive handling (retry/refresh logic phủ rộng hơn).
- Nhưng với stress rất cao, hệ thống hiện vẫn bị quyết định bởi upstream auth policy; `401` vẫn là dominant failure mode ở stream.
- `muxed` cần thay đổi kiến trúc (async/queue) nếu muốn throughput và reliability production-grade.

## Unresolved questions

1. Ngưỡng traffic mục tiêu production cụ thể (concurrent jobs, req/min) để chốt tuning cố định là bao nhiêu?
2. Có chấp nhận chuyển `muxed` sang async flow (không realtime) không?
3. Có triển khai sticky proxy pool + session pinning không, hay giữ single egress?
4. Mức retry budget tối đa chấp nhận cho UX hiện tại (latency vs success) là bao nhiêu?
