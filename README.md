# downloadtool

## Local Dev (Debug Friendly)

### 1) Prerequisites
- `pnpm` (>= 9)
- `docker` + `docker compose`
- `rust` + `cargo`

### 2) Run local (recommended)
Open 3 terminals from project root:

Terminal A (DB):
```bash
pnpm dev:db
```

Terminal B (Backend API on host):
```bash
pnpm dev:be
```

Terminal C (Frontend on host):
```bash
pnpm dev:fe
```

Frontend URL:
```text
http://<server-ip>:5168
```

## Logs

### Recommended when debugging app flow
- Backend logs: watch Terminal B (`pnpm dev:be`) directly.
- Frontend logs: watch Terminal C (`pnpm dev:fe`) directly.

### If using Docker services
```bash
pnpm logs:db
pnpm logs:api
pnpm logs:fe
```

## Stop Everything

1. `Ctrl + C` in Terminal B and C
2. Stop Docker services:
```bash
pnpm dev:down
```

## Scripts Summary

```bash
pnpm dev:db    # start postgres container
pnpm dev:be    # run rust api on host
pnpm dev:fe    # run sveltekit on host (auto-load .env + auto-resolve DB container IP)
pnpm logs:db   # follow postgres container logs
pnpm logs:api  # follow api container logs
pnpm logs:fe   # follow frontend container logs
pnpm dev:down  # stop docker compose services
```

## Important Note

- `frontend` currently proxies to Rust API by `RUST_API_URL`.
- If you run full Docker stack (`api` + `frontend` containers), requests go to API container.
- If you run `pnpm dev:be` + `pnpm dev:fe`, requests go to host API and logs appear in Terminal B.
- Env source of truth is root `.env` (see `.env.example`) for secrets/runtime endpoints.
- Runtime limits are now managed in `config/runtime-limit-profiles.json` with `local` and `production` objects.
- Trong `runtime-limit-profiles.json`, đặt field = `null` để tắt guard tương ứng. Riêng `backend.mux_job_max_workers = null` thì auto scale theo CPU.
- Runtime profile variable notes (`config/runtime-limit-profiles.json`):
- `backend.extract_rate_limit_enabled`: Bật/tắt rate limit IP cho `/api/extract`.
- `backend.extract_cache_ttl_secs`: TTL cache metadata từ `yt-dlp` (giây). `null`/`<=0` fallback `300`.
- `backend.stream_proxy_cache_ttl_secs`: TTL cache pin `stream_url -> proxy` (giây). `null`/`<=0` fallback `1800`.
- `backend.stream_max_concurrent`: Giới hạn đồng thời cho `/api/stream`. `null` = không giới hạn.
- `backend.mux_max_concurrent`: Giới hạn đồng thời cho `/api/stream/muxed`. `null` = không giới hạn.
- `backend.mux_preflight_timeout_secs`: Timeout preflight chunk đầu mux (giây). `null` = tắt timeout preflight.
- `backend.stream_url_refresh_max_attempts`: Số lần refresh URL stream khi upstream auth fail. `null` = không giới hạn.
- `backend.mux_url_refresh_max_attempts`: Số lần refresh URL cho luồng mux khi upstream auth fail. `null` = không giới hạn.
- `backend.mux_job_max_workers`: Số worker xử lý mux job nền. `null` = auto theo CPU.
- `backend.mux_job_queue_capacity`: Sức chứa queue mux job nền. `null` = fallback mặc định trong code.
- `backend.mux_job_estimated_runtime_secs`: Thời gian ước tính 1 job để tính ETA/wait. `null` = fallback mặc định trong code.
- `backend.mux_job_max_estimated_wait_secs`: Ngưỡng wait tối đa trước khi từ chối job mới. `null` = không giới hạn.
- `backend.mux_job_timeout_secs`: Timeout cứng cho mỗi mux job. `null` = tắt timeout.
- `backend.mux_job_ttl_secs`: TTL metadata mux job sau khi hoàn thành/thất bại. `null` = fallback mặc định trong code.
- `backend.mux_job_temp_file_ttl_secs`: TTL file output tạm của mux job. `null` = fallback mặc định trong code.
- `backend.mux_job_cleanup_interval_secs`: Chu kỳ dọn dẹp job/file hết hạn. `null` = fallback mặc định trong code.
- `backend.mux_job_output_dir`: Thư mục lưu file output mux job. `null` = fallback `/tmp/downloadtool-mux-jobs`.
- `frontend.extract_max_retry_attempts`: Số lần retry tối đa cho extract API call. `null` = không giới hạn.
- `frontend.extract_retry_base_delay_ms`: Delay retry cơ sở (ms). `null` = fallback mặc định trong code.
- `frontend.extract_retry_max_delay_ms`: Delay retry tối đa (ms). `null` = fallback mặc định trong code.
- `frontend.batch_max_reconnect_attempts`: Số lần reconnect tối đa cho batch stream. `null` = không giới hạn.
- `frontend.batch_reconnect_base_delay_ms`: Delay reconnect cơ sở (ms). `null` = fallback mặc định trong code.
- `frontend.batch_reconnect_max_delay_ms`: Delay reconnect tối đa (ms). `null` = fallback mặc định trong code.
- `frontend.mux_job_poll_interval_ms`: Chu kỳ poll trạng thái mux job (ms). `null` = fallback mặc định trong code.
- `frontend.mux_job_max_wait_ms`: Tổng thời gian poll mux job (ms). `null` = không giới hạn.
- `frontend.mux_sync_active_hard_limit`: Hard limit số mux sync đồng thời. `null` = không giới hạn.
- `frontend.mux_sync_active_soft_limit`: Soft limit số mux sync đồng thời. `null` = không giới hạn.
- `frontend.mux_sync_duration_max_seconds`: Giới hạn duration cho sync mux path (s). `null` = không giới hạn.
- `frontend.mux_force_job_duration_seconds`: Ngưỡng duration để ép background mux job (s). `null` = tắt ngưỡng.
- `frontend.mux_sync_size_max_mb`: Giới hạn dung lượng cho sync mux path (MB). `null` = không giới hạn.
- `frontend.mux_force_job_size_max_mb`: Ngưỡng dung lượng để ép background mux job (MB). `null` = tắt ngưỡng.
- `frontend.mux_sync_resolution_max`: Giới hạn độ phân giải cho sync mux path. `null` = không giới hạn.
- `frontend.mux_force_job_resolution_min`: Ngưỡng độ phân giải để ép background mux job. `null` = tắt ngưỡng.
- `frontend.playlist_worker_max_concurrent`: Số worker playlist đồng thời. `null` = không giới hạn.
- `frontend.playlist_worker_ready_queue_max`: Độ dài ready queue playlist worker. `null` = không giới hạn.
- `frontend.playlist_worker_extract_jitter_min_ms`: Jitter tối thiểu trước mỗi lần extract (ms). `null` = 0.
- `frontend.playlist_worker_extract_jitter_range_ms`: Jitter random bổ sung (ms). `null` = 0.
- `frontend.playlist_worker_circuit_cooldown_ms`: Cooldown circuit breaker (ms). `null` = 0.
- For SEO audits: disable Cloudflare "Managed robots/content signals" for `robots.txt`, otherwise Cloudflare injects `Content-Signal` lines that Lighthouse flags as invalid robots directives.
