# Phase 04 — Clean Docker & Compose

## Overview
- **Priority:** P2
- **Status:** pending
- **ETA:** 15m
- **Depends on:** none (independent of Rust changes)

## Files to Modify / Delete

| File | Action |
|------|--------|
| `docker/Dockerfile.gpu-worker` | Delete entirely |
| `docker/docker-compose.server.yml` | Remove `gpu-worker` service, volumes, env vars |

---

## 1. Delete `docker/Dockerfile.gpu-worker`

```bash
rm docker/Dockerfile.gpu-worker
```

---

## 2. `docker/docker-compose.server.yml`

### Remove entire `gpu-worker` service block (lines 1–23):

```yaml
services:
  gpu-worker:          # ← remove this whole service
    build: ...
    container_name: downloadtool-gpu-worker
    environment:
      - GPU_WORKER_BIND=...
      - GPU_WORKER_MAX_JOBS=...
      - CUDA_DEVICE_ID=...
      - RUST_LOG=info
    volumes:
      - gpu-worker-data:/app/data
    restart: unless-stopped
    networks:
      - coolify
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: 1
              capabilities: [gpu]
```

### In `api` service — remove GPU env vars and depends_on:

Remove from `environment`:
```yaml
- GPU_WORKER_ADDR=http://downloadtool-gpu-worker:50051   # remove
- GPU_ENABLED=true                                        # remove
```

Remove `depends_on` block entirely (api no longer depends on gpu-worker):
```yaml
depends_on:            # remove
  - gpu-worker         # remove
```

### Remove orphan volume:

```yaml
volumes:
  gpu-worker-data:   # ← remove this entry
  api-data:          # keep
```

### Final `docker-compose.server.yml` structure:

```yaml
services:
  api:
    build:
      context: .
      dockerfile: docker/Dockerfile.api
    container_name: downloadtool-api
    environment:
      - PORT=3068
      - RUST_LOG=info
    expose:
      - "3068"
    volumes:
      - api-data:/app/data
    restart: unless-stopped
    networks:
      - coolify
    labels:
      - "traefik.docker.network=coolify"

  frontend:
    build:
      context: .
      dockerfile: docker/Dockerfile.frontend
      args:
        - VITE_API_URL=https://api-download.khoadangbui.online
    container_name: downloadtool-frontend
    environment:
      - NODE_ENV=production
      - PORT=5168
      - HOST=0.0.0.0
      - ORIGIN=https://download.khoadangbui.online
    expose:
      - "5168"
    restart: unless-stopped
    depends_on:
      - api
    networks:
      - coolify
    labels:
      - "traefik.docker.network=coolify"

volumes:
  api-data:

networks:
  coolify:
    external: true
```

## Success Criteria
- `docker/Dockerfile.gpu-worker` does not exist
- `docker-compose.server.yml` has no references to `gpu-worker`, `GPU_WORKER_ADDR`, `GPU_ENABLED`, `CUDA_*`, `NVIDIA_*`
- `docker compose -f docker/docker-compose.server.yml config` validates without errors
