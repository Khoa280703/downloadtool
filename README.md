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

### Local isolation
- `pnpm dev:db` starts project-local containers: `downloadtool-postgres` and `downloadtool-redis`.
- `pnpm dev:be` and `pnpm dev:worker` are intended to use those local containers, not shared server services.
- Keep local `.env` aligned with `.env.example`:
```text
DATABASE_URL=postgres://downloadtool:...@127.0.0.1:5432/downloadtool
REDIS_URL=redis://127.0.0.1:6379
```
- Production uses the internal Compose service names `postgres` and `redis` inside the deployed stack.
- Do not point local `.env` to shared services like `server-redis`, otherwise dev and non-dev state becomes ambiguous.

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
pnpm dev:be    # run rust api on host (expects local postgres/redis from dev:db)
pnpm dev:worker # run mux worker on host (expects local postgres/redis from dev:db)
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
- Local dev target should stay isolated:
- `DATABASE_URL` -> `127.0.0.1:5432/downloadtool`
- `REDIS_URL` -> `127.0.0.1:6379`
- Production should keep using internal service names from `docker/docker-compose.server.yml`:
- `DATABASE_URL=postgres://downloadtool:...@postgres:5432/downloadtool`
- `REDIS_URL=redis://redis:6379`
- Proxy pool quarantine:
- Khi extractor gặp lỗi bot-check (`Sign in to confirm you're not a bot`), proxy đó sẽ bị loại khỏi vòng xoay ngay lập tức.
- Danh sách proxy bị loại được ghi riêng vào `PROXY_QUARANTINE_FILE` (mặc định: `/tmp/downloadtool-quarantined-proxies.txt`) để dễ thay proxy mới.
- Runtime limits are now managed in `config/runtime-limit-profiles.json` with `local` and `production` objects.
- Trong `runtime-limit-profiles.json`, đặt field = `null` để tắt guard tương ứng.
- Runtime profile variable notes (`config/runtime-limit-profiles.json`):
- `backend.extract_rate_limit_enabled`: Bật/tắt rate limit IP cho `/api/extract`.
- `backend.stream_max_concurrent`: Giới hạn đồng thời cho `/api/stream`. `null` = không giới hạn.
- `backend.stream_url_refresh_max_attempts`: Số lần refresh URL stream khi upstream auth fail. `null` = không giới hạn.
- `frontend.extract_max_retry_attempts`: Số lần retry tối đa cho extract API call. `null` = không giới hạn.
- `frontend.extract_retry_base_delay_ms`: Delay retry cơ sở (ms). `null` = fallback mặc định trong code.
- `frontend.extract_retry_max_delay_ms`: Delay retry tối đa (ms). `null` = fallback mặc định trong code.
- `frontend.batch_max_reconnect_attempts`: Số lần reconnect tối đa cho batch stream. `null` = không giới hạn.
- `frontend.batch_reconnect_base_delay_ms`: Delay reconnect cơ sở (ms). `null` = fallback mặc định trong code.
- `frontend.batch_reconnect_max_delay_ms`: Delay reconnect tối đa (ms). `null` = fallback mặc định trong code.
- `frontend.mux_job_poll_interval_ms`: Chu kỳ poll trạng thái mux job (ms). `null` = fallback mặc định trong code. (Lưu ý: Giờ dùng SSE `/api/proxy/jobs/{id}/events` thay vì polling)
- `frontend.mux_job_max_wait_ms`: Tổng thời gian poll mux job (ms). `null` = không giới hạn.
- `frontend.playlist_worker_max_concurrent`: Số worker playlist đồng thời. `null` = không giới hạn.
- `frontend.playlist_worker_ready_queue_max`: Độ dài ready queue playlist worker. `null` = không giới hạn.
- `frontend.playlist_worker_extract_jitter_min_ms`: Jitter tối thiểu trước mỗi lần extract (ms). `null` = 0.
- `frontend.playlist_worker_extract_jitter_range_ms`: Jitter random bổ sung (ms). `null` = 0.
- `frontend.playlist_worker_circuit_cooldown_ms`: Cooldown circuit breaker (ms). `null` = 0.

## Recent Changes (2026-03-16)

- **i18n Complete:** 24+ languages active via Paraglide JS
- **Dual Download Flow:** Direct browser + background mux job with SSE progress
- **New Job System:** Durable job pipeline (PostgreSQL + Redis + Worker)
- **Real-Time Progress:** SSE endpoint `/api/proxy/jobs/{id}/events` for 7-phase progress tracking
- **New Components:** DownloadBtn (unified), AppIcon (SVG icons + quality badges)
- **S3 Support:** S3 multipart upload for artifact storage
- For SEO audits: disable Cloudflare "Managed robots/content signals" for `robots.txt`, otherwise Cloudflare injects `Content-Signal` lines that Lighthouse flags as invalid robots directives.
