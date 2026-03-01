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
