# Stress Test Report (Post P0 Backpressure + Refresh)

Date: 2026-03-02
Scope: `/api/extract`, `/api/stream`, `/api/stream/muxed`, `/api/batch`

## 1) Kiến trúc hiện tại (liên quan tải)

- `GET /api/stream`
  - Proxy stream theo `Range`.
  - Có `STREAM_SEMAPHORE` global để admission control.
  - Khi full slot: trả `503` + `Retry-After: 2`.
  - Có cơ chế refresh URL từ `source_url + format_id` khi upstream trả `401/403`.
- `GET /api/stream/muxed`
  - Fetch video+audio, remux fMP4.
  - Có `MUX_SEMAPHORE` global riêng.
  - Full slot => `503` + `Retry-After: 2`.
  - Có refresh URL bằng `source_url + video_format_id + audio_format_id` khi auth fail upstream.
  - Preflight timeout configurable qua `MUX_PREFLIGHT_TIMEOUT_SECS` (default 15s).
- `POST /api/extract`
  - Metadata/format extraction (yt-dlp/extractor path).
- `GET /api/batch`
  - SSE playlist enumeration/extract per item; HTTP layer vẫn có thể `200` dù trong stream có event `error`.

## 2) Test methodology

### Main full-load run
- Script: `scripts/stress-test-all-endpoints.sh`
- Command:
  - `API_BASE='http://127.0.0.1:3068' VIDEO_URL='https://www.youtube.com/watch?v=dQw4w9WgXcQ' PROFILE='strong' RUN_BATCH='1' scripts/stress-test-all-endpoints.sh`
- Output:
  - `summary.tsv`: `/tmp/stress-reusable-strong-20260302-113032/summary.tsv`
  - `batch-summary.tsv`: `/tmp/stress-reusable-strong-20260302-113032/batch-summary.tsv`

### Baseline đối chiếu (trước patch P0)
- `summary.tsv`: `/tmp/stress-reusable-strong-20260302-100756/summary.tsv`
- `batch-summary.tsv`: `/tmp/stress-reusable-strong-20260302-100756/batch-summary.tsv`

### Extra tuning sweep
- Raw dir: `/tmp/semaphore-sweep-20260302-113908`
- Sweep values:
  - `STREAM_MAX_CONCURRENT`: `96/128/160`
  - `MUX_MAX_CONCURRENT`: `12/20/30`

## 3) Kết quả chính (full-load)

### 3.1 Post-fix run (20260302-113032)

- `/api/extract` c120 n12000:
  - `12000/12000` HTTP 200 (100%)
  - p95: `0.007695s`
- `/api/stream` 4KB c150 n12000:
  - `5633/12000` HTTP 200 (46.94%)
  - `6367` lỗi `503` (0 lỗi 502/504/000)
  - p95: `4.275401s`
- `/api/stream` 1MB c100 n5000:
  - `3826/5000` HTTP 200 (76.52%)
  - `1174` lỗi `503` (0 lỗi 502/504/000)
  - p95: `3.969459s`
- `/api/stream/muxed` c40 n2000:
  - `12/2000` HTTP 200 (0.6%)
  - `1988` lỗi `503` (0 lỗi 502/504/000)
  - p95: `0.005090s` (đa số fail nhanh do backpressure)
- `/api/stream/muxed` c80 n2000:
  - `12/2000` HTTP 200 (0.6%)
  - `1988` lỗi `503`
  - p95: `0.004877s`
- `/api/batch`:
  - c20 n2000: `done_ok=1554/2000` (77.7%), HTTP 200 toàn bộ
  - c40 n2000: `done_ok=1189/2000` (59.45%), HTTP 200 toàn bộ

### 3.2 So sánh baseline vs post-fix

- `/api/extract`: giữ 100% ổn định.
- `/api/stream 1MB`: tăng mạnh `34.14% -> 76.52%`.
- `/api/stream 4KB`: giảm `85.82% -> 46.94%` do backpressure `503` chủ động.
- `/api/muxed`: vẫn rất thấp (khoảng 0.6-1.75% tùy run).
- `/api/batch`: tương đương baseline (phụ thuộc upstream extractor ở mức event).

## 4) Điểm quan trọng: bản chất lỗi đã đổi

- Trước fix: có nhiều failure từ upstream path (đặc biệt 502 liên quan auth/token churn).
- Sau fix: dưới tải nặng, lỗi chuyển thành `503` do admission control.
- Ý nghĩa:
  - Backend không còn “gãy ngẫu nhiên” theo kiểu upstream leak rõ trong mẫu mới.
  - Hệ thống hiện ưu tiên tự bảo vệ (shed load) thay vì cố xử lý rồi fail sâu.

## 5) Kết quả sweep ngưỡng semaphore

| case | ok200 | 503 | 502 | p95 |
|---|---:|---:|---:|---:|
| stream-4kb-s96 | 1465/3000 | 1535 | 0 | 4.54s |
| stream-4kb-s128 | 2477/3000 | 133 | 390 | 5.76s |
| stream-4kb-s160 | 589/3000 | 0 | 2411 | 5.75s |
| stream-1mb-s96 | 991/1200 | 209 | 0 | 4.53s |
| stream-1mb-s128 | 1200/1200 | 0 | 0 | 3.76s |
| stream-1mb-s160 | 1200/1200 | 0 | 0 | 4.04s |
| muxed-m12 | 12/400 | 388 | 0 | 0.009s |
| muxed-m20 | 20/400 | 380 | 0 | 0.81s |
| muxed-m30 | 30/400 | 370 | 0 | 30.00s |

Inference:
- `STREAM_MAX=128` cân bằng tốt nhất giữa 503 và 502.
- `STREAM_MAX=160` đẩy upstream fail (`502`) quá mạnh.
- `MUX_MAX=20` tốt hơn `12`; `30` đẩy tail latency lên sát timeout.

## 6) Thay đổi code đã chốt sau sweep

File: `crates/api/src/routes/stream.rs`

- `STREAM_MAX_CONCURRENT_DEFAULT`: `96 -> 128`
- `MUX_MAX_CONCURRENT_DEFAULT`: `12 -> 20`

`cargo check -p api`: pass.

## 7) Kết luận vận hành

- P0 đã đạt mục tiêu “defensive stability”: hệ thống phản hồi có kiểm soát dưới overload (503) thay vì fail ngẫu nhiên sâu.
- Tuy nhiên, với traffic burst không có retry/queue client-side, tỷ lệ success thô sẽ thấp do shed load.
- `/api/muxed` realtime vẫn là nút thắt kiến trúc lớn nhất; admission control chỉ giảm cháy lan, chưa tăng throughput thực.

## 8) Khuyến nghị tiếp theo (ưu tiên)

1. Thêm retry có backoff cho client khi nhận `503` + tôn trọng `Retry-After`.
2. Chuyển `/api/stream/muxed` sang async job queue (202 + polling/SSE) thay vì realtime synchronous.
3. Thêm metric production:
   - `admission_reject_total{endpoint}`
   - `upstream_401_403_total`
   - `url_refresh_success_total` / `url_refresh_failure_total`
   - `mux_preflight_timeout_total`
4. Tách SLO theo endpoint:
   - `/extract` latency SLO
   - `/stream` success+latency under bounded concurrency
   - `/muxed` queue wait + completion SLO

## 9) Post-report update (implemented)

- Frontend đã triển khai retry/backoff client-side cho luồng download:
  - Retry status: `429/502/503/504`.
  - Đọc và tôn trọng header `Retry-After` nếu có.
  - Có preflight trước anchor fallback để giảm xác suất hit `503` ngay lần click đầu.
- File thay đổi:
  - `frontend/src/lib/playlist-download-file-saver.ts`
  - `frontend/src/components/DownloadBtn.svelte`

## Unresolved questions

- Ngưỡng concurrency mục tiêu production theo hạ tầng thực tế (CPU/RAM/bandwidth) chưa được lock chính thức.
- Cần stress test lại end-to-end sau retry client-side để định lượng uplift thực tế ở UX (success-per-click).
- Có chấp nhận đổi UX `/api/stream/muxed` sang async job hay vẫn bắt buộc realtime.
