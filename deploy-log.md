2026-Feb-25 07:10:21.239553
Starting deployment of khoa280703/downloadtool:main-zoscg4oc04gkwkssg0kw8w8w to localhost.
2026-Feb-25 07:10:21.397064
Preparing container with helper image: ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Feb-25 07:10:21.489982
[CMD]: docker stop -t 30 zogw0c04g4okg8w4wokkgs4k
2026-Feb-25 07:10:21.489982
Error response from daemon: No such container: zogw0c04g4okg8w4wokkgs4k
2026-Feb-25 07:10:21.961873
[CMD]: docker run -d --network coolify --name zogw0c04g4okg8w4wokkgs4k  --rm -v /var/run/docker.sock:/var/run/docker.sock ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Feb-25 07:10:21.961873
30c8ac319e3231d9bcfe53629e2272212b8b9a3e755504d00254d82363b035f4
2026-Feb-25 07:10:23.108933
[CMD]: docker exec zogw0c04g4okg8w4wokkgs4k bash -c 'GIT_SSH_COMMAND="ssh -o ConnectTimeout=30 -p 22 -o Port=22 -o LogLevel=ERROR -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git ls-remote https://github.com/Khoa280703/downloadtool refs/heads/main'
2026-Feb-25 07:10:23.108933
6284ccef8da2b1bbb3aa00193368912f946f4bb3	refs/heads/main
2026-Feb-25 07:10:23.119663
----------------------------------------
2026-Feb-25 07:10:23.123619
Importing Khoa280703/downloadtool:main (commit sha 6284ccef8da2b1bbb3aa00193368912f946f4bb3) to /artifacts/zogw0c04g4okg8w4wokkgs4k.
2026-Feb-25 07:10:23.270570
[CMD]: docker exec zogw0c04g4okg8w4wokkgs4k bash -c 'git clone --depth=1 --recurse-submodules --shallow-submodules -b 'main' 'https://github.com/Khoa280703/downloadtool' '/artifacts/zogw0c04g4okg8w4wokkgs4k' && cd '/artifacts/zogw0c04g4okg8w4wokkgs4k' && if [ -f .gitmodules ]; then sed -i "s#git@\(.*\):#https://\1/#g" '/artifacts/zogw0c04g4okg8w4wokkgs4k'/.gitmodules || true && git submodule sync && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git submodule update --init --recursive --depth=1; fi && cd '/artifacts/zogw0c04g4okg8w4wokkgs4k' && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git lfs pull'
2026-Feb-25 07:10:23.270570
Cloning into '/artifacts/zogw0c04g4okg8w4wokkgs4k'...
2026-Feb-25 07:10:25.237081
[CMD]: docker exec zogw0c04g4okg8w4wokkgs4k bash -c 'cd /artifacts/zogw0c04g4okg8w4wokkgs4k && git log -1 6284ccef8da2b1bbb3aa00193368912f946f4bb3 --pretty=%B'
2026-Feb-25 07:10:25.237081
fix deploy
2026-Feb-25 07:10:28.496776
[CMD]: docker exec zogw0c04g4okg8w4wokkgs4k bash -c 'test -f /artifacts/zogw0c04g4okg8w4wokkgs4k/docker/Dockerfile.homeserver && echo 'exists' || echo 'not found''
2026-Feb-25 07:10:28.496776
exists
2026-Feb-25 07:10:28.636450
[CMD]: docker exec zogw0c04g4okg8w4wokkgs4k bash -c 'cat /artifacts/zogw0c04g4okg8w4wokkgs4k/docker/Dockerfile.homeserver'
2026-Feb-25 07:10:28.636450
# Dockerfile for Home Server deployment
2026-Feb-25 07:10:28.636450
# Builds the GPU worker with CUDA support for hardware transcoding
2026-Feb-25 07:10:28.636450
2026-Feb-25 07:10:28.636450
FROM nvidia/cuda:12.3.2-devel-ubuntu22.04 AS builder
2026-Feb-25 07:10:28.636450
2026-Feb-25 07:10:28.636450
WORKDIR /app
2026-Feb-25 07:10:28.636450
2026-Feb-25 07:10:28.636450
# Install dependencies
2026-Feb-25 07:10:28.636450
RUN apt-get update && apt-get install -y \
2026-Feb-25 07:10:28.636450
curl \
2026-Feb-25 07:10:28.636450
build-essential \
2026-Feb-25 07:10:28.636450
pkg-config \
2026-Feb-25 07:10:28.636450
libssl-dev \
2026-Feb-25 07:10:28.636450
protobuf-compiler \
2026-Feb-25 07:10:28.636450
ffmpeg \
2026-Feb-25 07:10:28.636450
libavcodec-dev \
2026-Feb-25 07:10:28.636450
libavformat-dev \
2026-Feb-25 07:10:28.636450
libavutil-dev \
2026-Feb-25 07:10:28.636450
libswscale-dev \
2026-Feb-25 07:10:28.636450
libavfilter-dev \
2026-Feb-25 07:10:28.636450
libavdevice-dev \
2026-Feb-25 07:10:28.636450
clang \
2026-Feb-25 07:10:28.636450
libclang-dev \
2026-Feb-25 07:10:28.636450
&& rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:10:28.636450
2026-Feb-25 07:10:28.636450
# Install Rust
2026-Feb-25 07:10:28.636450
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
2026-Feb-25 07:10:28.636450
ENV PATH="/root/.cargo/bin:${PATH}"
2026-Feb-25 07:10:28.636450
2026-Feb-25 07:10:28.636450
# Copy workspace configuration
2026-Feb-25 07:10:28.636450
COPY Cargo.toml ./
2026-Feb-25 07:10:28.636450
COPY Cargo.lock ./
2026-Feb-25 07:10:28.636450
2026-Feb-25 07:10:28.636450
# Copy all crates
2026-Feb-25 07:10:28.636450
COPY crates/ ./crates/
2026-Feb-25 07:10:28.636450
2026-Feb-25 07:10:28.636450
# Copy proto files
2026-Feb-25 07:10:28.636450
COPY proto/ ./proto/
2026-Feb-25 07:10:28.636450
2026-Feb-25 07:10:28.636450
# Build the GPU worker with CUDA support
2026-Feb-25 07:10:28.636450
# NOTE: --features gpu disabled until FFmpeg 7.x is available in the build environment
2026-Feb-25 07:10:28.636450
# Ubuntu 22.04 apt provides FFmpeg 4.4.x but ffmpeg-next = "7" requires FFmpeg 7.x
2026-Feb-25 07:10:28.636450
RUN cargo build --release --bin gpu-node
2026-Feb-25 07:10:28.636450
2026-Feb-25 07:10:28.636450
# Stage 2: Runtime
2026-Feb-25 07:10:28.636450
FROM nvidia/cuda:12.3.2-runtime-ubuntu22.04 AS runtime
2026-Feb-25 07:10:28.636450
2026-Feb-25 07:10:28.636450
WORKDIR /app
2026-Feb-25 07:10:28.636450
2026-Feb-25 07:10:28.636450
# Install runtime dependencies
2026-Feb-25 07:10:28.636450
RUN apt-get update && apt-get install -y \
2026-Feb-25 07:10:28.636450
ca-certificates \
2026-Feb-25 07:10:28.636450
libssl3 \
2026-Feb-25 07:10:28.636450
ffmpeg \
2026-Feb-25 07:10:28.636450
libavcodec58 \
2026-Feb-25 07:10:28.636450
libavformat58 \
2026-Feb-25 07:10:28.636450
libavutil56 \
2026-Feb-25 07:10:28.636450
libswscale5 \
2026-Feb-25 07:10:28.636450
libavfilter7 \
2026-Feb-25 07:10:28.636450
libavdevice58 \
2026-Feb-25 07:10:28.636450
&& rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:10:28.636450
2026-Feb-25 07:10:28.636450
# Create non-root user
2026-Feb-25 07:10:28.636450
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-25 07:10:28.636450
2026-Feb-25 07:10:28.636450
# Copy binary from builder
2026-Feb-25 07:10:28.636450
COPY --from=builder /app/target/release/gpu-node /usr/local/bin/
2026-Feb-25 07:10:28.636450
2026-Feb-25 07:10:28.636450
# Create directories
2026-Feb-25 07:10:28.636450
RUN mkdir -p /app/data && chown -R appuser:appuser /app
2026-Feb-25 07:10:28.636450
2026-Feb-25 07:10:28.636450
# Switch to non-root user
2026-Feb-25 07:10:28.636450
USER appuser
2026-Feb-25 07:10:28.636450
2026-Feb-25 07:10:28.636450
# Environment variables
2026-Feb-25 07:10:28.636450
ENV GPU_WORKER_BIND=0.0.0.0:50051
2026-Feb-25 07:10:28.636450
ENV GPU_WORKER_MAX_JOBS=4
2026-Feb-25 07:10:28.636450
ENV CUDA_DEVICE_ID=0
2026-Feb-25 07:10:28.636450
ENV RUST_LOG=info
2026-Feb-25 07:10:28.636450
2026-Feb-25 07:10:28.636450
# NVIDIA runtime configuration
2026-Feb-25 07:10:28.636450
ENV NVIDIA_VISIBLE_DEVICES=all
2026-Feb-25 07:10:28.636450
ENV NVIDIA_DRIVER_CAPABILITIES=compute,video,utility
2026-Feb-25 07:10:28.636450
2026-Feb-25 07:10:28.636450
# Expose gRPC port
2026-Feb-25 07:10:28.636450
EXPOSE 50051
2026-Feb-25 07:10:28.636450
2026-Feb-25 07:10:28.636450
# Health check
2026-Feb-25 07:10:28.636450
HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
2026-Feb-25 07:10:28.636450
CMD echo "health check placeholder" || exit 0
2026-Feb-25 07:10:28.636450
2026-Feb-25 07:10:28.636450
# Run the GPU worker
2026-Feb-25 07:10:28.636450
CMD ["gpu-node"]
2026-Feb-25 07:10:28.787800
Added 12 ARG declarations to Dockerfile for service gpu-worker (multi-stage build, added to 2 stages).
2026-Feb-25 07:10:28.931037
[CMD]: docker exec zogw0c04g4okg8w4wokkgs4k bash -c 'test -f /artifacts/zogw0c04g4okg8w4wokkgs4k/docker/Dockerfile.vps && echo 'exists' || echo 'not found''
2026-Feb-25 07:10:28.931037
exists
2026-Feb-25 07:10:29.074539
[CMD]: docker exec zogw0c04g4okg8w4wokkgs4k bash -c 'cat /artifacts/zogw0c04g4okg8w4wokkgs4k/docker/Dockerfile.vps'
2026-Feb-25 07:10:29.074539
# Dockerfile for VPS deployment
2026-Feb-25 07:10:29.074539
# Builds the API server and related components without GPU support
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
# Stage 0: Build injector JS (embedded into api crate via include_str! at compile time)
2026-Feb-25 07:10:29.074539
FROM node:22-alpine AS js-builder
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
WORKDIR /app
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
RUN npm install -g pnpm
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
# Copy workspace manifests for pnpm resolution
2026-Feb-25 07:10:29.074539
COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Feb-25 07:10:29.074539
COPY packages/api-client/package.json ./packages/api-client/
2026-Feb-25 07:10:29.074539
COPY apps/injector/package.json ./apps/injector/
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
# Copy injector source and shared packages
2026-Feb-25 07:10:29.074539
COPY apps/injector/ ./apps/injector/
2026-Feb-25 07:10:29.074539
COPY packages/ ./packages/
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
# Install deps and build injector (produces dist/bm.js and dist/youtube-downloader.user.js)
2026-Feb-25 07:10:29.074539
RUN pnpm install --frozen-lockfile
2026-Feb-25 07:10:29.074539
RUN pnpm --filter @downloadtool/injector build
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
# Stage 1: Rust builder
2026-Feb-25 07:10:29.074539
FROM rust:1.88-bookworm AS builder
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
WORKDIR /app
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
# Install dependencies
2026-Feb-25 07:10:29.074539
RUN apt-get update && apt-get install -y \
2026-Feb-25 07:10:29.074539
pkg-config \
2026-Feb-25 07:10:29.074539
libssl-dev \
2026-Feb-25 07:10:29.074539
protobuf-compiler \
2026-Feb-25 07:10:29.074539
&& rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
# Copy workspace configuration
2026-Feb-25 07:10:29.074539
COPY Cargo.toml ./
2026-Feb-25 07:10:29.074539
COPY Cargo.lock ./
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
# Copy all crates
2026-Feb-25 07:10:29.074539
COPY crates/ ./crates/
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
# Copy proto files
2026-Feb-25 07:10:29.074539
COPY proto/ ./proto/
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
# Copy injector dist (required by include_str! in crates/api/src/routes/static_files.rs)
2026-Feb-25 07:10:29.074539
COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
# Build the release binary
2026-Feb-25 07:10:29.074539
RUN cargo build --release --bin vps-gateway
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
# Stage 2: Runtime
2026-Feb-25 07:10:29.074539
FROM debian:bookworm-slim AS runtime
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
WORKDIR /app
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
# Install runtime dependencies
2026-Feb-25 07:10:29.074539
RUN apt-get update && apt-get install -y \
2026-Feb-25 07:10:29.074539
ca-certificates \
2026-Feb-25 07:10:29.074539
libssl3 \
2026-Feb-25 07:10:29.074539
&& rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
# Create non-root user
2026-Feb-25 07:10:29.074539
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
# Copy binary from builder
2026-Feb-25 07:10:29.074539
COPY --from=builder /app/target/release/vps-gateway /usr/local/bin/
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
# Create directories
2026-Feb-25 07:10:29.074539
RUN mkdir -p /app/extractors /app/data && chown -R appuser:appuser /app
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
# Switch to non-root user
2026-Feb-25 07:10:29.074539
USER appuser
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
# Environment variables
2026-Feb-25 07:10:29.074539
ENV PORT=3068
2026-Feb-25 07:10:29.074539
ENV EXTRACTOR_DIR=/app/extractors
2026-Feb-25 07:10:29.074539
ENV GPU_ENABLED=false
2026-Feb-25 07:10:29.074539
ENV RUST_LOG=info
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
# Expose port
2026-Feb-25 07:10:29.074539
EXPOSE 3068
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
# Health check
2026-Feb-25 07:10:29.074539
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Feb-25 07:10:29.074539
CMD curl -f http://localhost:3068/health || exit 1
2026-Feb-25 07:10:29.074539
2026-Feb-25 07:10:29.074539
# Run the server
2026-Feb-25 07:10:29.074539
CMD ["vps-gateway"]
2026-Feb-25 07:10:29.237075
Added 18 ARG declarations to Dockerfile for service api (multi-stage build, added to 3 stages).
2026-Feb-25 07:10:29.390572
[CMD]: docker exec zogw0c04g4okg8w4wokkgs4k bash -c 'test -f /artifacts/zogw0c04g4okg8w4wokkgs4k/docker/Dockerfile.frontend && echo 'exists' || echo 'not found''
2026-Feb-25 07:10:29.390572
exists
2026-Feb-25 07:10:29.545548
[CMD]: docker exec zogw0c04g4okg8w4wokkgs4k bash -c 'cat /artifacts/zogw0c04g4okg8w4wokkgs4k/docker/Dockerfile.frontend'
2026-Feb-25 07:10:29.545548
# Dockerfile for frontend (SvelteKit Node server)
2026-Feb-25 07:10:29.545548
# Copy ALL source files BEFORE npm install so svelte-kit sync (prepare script)
2026-Feb-25 07:10:29.545548
# can find svelte.config.js and generate .svelte-kit/ correctly.
2026-Feb-25 07:10:29.545548
2026-Feb-25 07:10:29.545548
FROM node:22-alpine AS builder
2026-Feb-25 07:10:29.545548
2026-Feb-25 07:10:29.545548
WORKDIR /app
2026-Feb-25 07:10:29.545548
2026-Feb-25 07:10:29.545548
# Copy all frontend source files first (node_modules excluded via .dockerignore)
2026-Feb-25 07:10:29.545548
COPY frontend/ ./
2026-Feb-25 07:10:29.545548
2026-Feb-25 07:10:29.545548
# Install — prepare script runs svelte-kit sync with svelte.config.js available
2026-Feb-25 07:10:29.545548
RUN npm install
2026-Feb-25 07:10:29.545548
2026-Feb-25 07:10:29.545548
# Build via programmatic API — bypasses vite.config.ts loading (esbuild issues on Alpine)
2026-Feb-25 07:10:29.545548
RUN node build-docker.mjs
2026-Feb-25 07:10:29.545548
2026-Feb-25 07:10:29.545548
# Runtime
2026-Feb-25 07:10:29.545548
FROM node:22-alpine AS runtime
2026-Feb-25 07:10:29.545548
2026-Feb-25 07:10:29.545548
WORKDIR /app
2026-Feb-25 07:10:29.545548
2026-Feb-25 07:10:29.545548
COPY --from=builder /app/build ./build
2026-Feb-25 07:10:29.545548
COPY --from=builder /app/package.json ./
2026-Feb-25 07:10:29.545548
2026-Feb-25 07:10:29.545548
ENV PORT=3000
2026-Feb-25 07:10:29.545548
ENV HOST=0.0.0.0
2026-Feb-25 07:10:29.545548
2026-Feb-25 07:10:29.545548
EXPOSE 3000
2026-Feb-25 07:10:29.545548
2026-Feb-25 07:10:29.545548
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Feb-25 07:10:29.545548
CMD wget -qO- http://localhost:3000 || exit 1
2026-Feb-25 07:10:29.545548
2026-Feb-25 07:10:29.545548
CMD ["node", "build"]
2026-Feb-25 07:10:29.698710
Added 12 ARG declarations to Dockerfile for service frontend (multi-stage build, added to 2 stages).
2026-Feb-25 07:10:29.702745
Pulling & building required images.
2026-Feb-25 07:10:29.709019
Creating build-time .env file in /artifacts (outside Docker context).
2026-Feb-25 07:10:29.860567
Adding build arguments to Docker Compose build command.
2026-Feb-25 07:10:30.121992
[CMD]: docker exec zogw0c04g4okg8w4wokkgs4k bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/build-time.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/zogw0c04g4okg8w4wokkgs4k -f /artifacts/zogw0c04g4okg8w4wokkgs4k/docker/docker-compose.homeserver.yml build --pull --build-arg COOLIFY_URL --build-arg COOLIFY_FQDN --build-arg SERVICE_URL_FRONTEND --build-arg SERVICE_FQDN_FRONTEND --build-arg SERVICE_URL_API --build-arg SERVICE_FQDN_API --build-arg COOLIFY_BUILD_SECRETS_HASH=016dc4226f6c53eea50a0f46c58a295ad358d7f449966f8f46fd81293803bce6'
2026-Feb-25 07:10:30.121992
#1 [internal] load local bake definitions
2026-Feb-25 07:10:30.330391
#1 reading from stdin 2.57kB done
2026-Feb-25 07:10:30.330391
#1 DONE 0.0s
2026-Feb-25 07:10:30.330391
2026-Feb-25 07:10:30.330391
#2 [api internal] load build definition from Dockerfile.vps
2026-Feb-25 07:10:30.330391
#2 transferring dockerfile: 2.63kB done
2026-Feb-25 07:10:30.330391
#2 DONE 0.0s
2026-Feb-25 07:10:30.330391
2026-Feb-25 07:10:30.330391
#3 [gpu-worker internal] load build definition from Dockerfile.homeserver
2026-Feb-25 07:10:30.330391
#3 transferring dockerfile: 2.46kB done
2026-Feb-25 07:10:30.330391
#3 DONE 0.0s
2026-Feb-25 07:10:30.330391
2026-Feb-25 07:10:30.330391
#4 [frontend internal] load build definition from Dockerfile.frontend
2026-Feb-25 07:10:30.330391
#4 transferring dockerfile: 1.18kB done
2026-Feb-25 07:10:30.330391
#4 DONE 0.0s
2026-Feb-25 07:10:30.330391
2026-Feb-25 07:10:30.330391
#5 [frontend internal] load metadata for docker.io/library/node:22-alpine
2026-Feb-25 07:10:31.278977
#5 ...
2026-Feb-25 07:10:31.278977
2026-Feb-25 07:10:31.278977
#6 [api internal] load metadata for docker.io/library/rust:1.88-bookworm
2026-Feb-25 07:10:31.278977
#6 DONE 1.1s
2026-Feb-25 07:10:31.380244
#7 [api internal] load metadata for docker.io/library/debian:bookworm-slim
2026-Feb-25 07:10:31.380244
#7 DONE 1.1s
2026-Feb-25 07:10:31.380244
2026-Feb-25 07:10:31.380244
#5 [api internal] load metadata for docker.io/library/node:22-alpine
2026-Feb-25 07:10:31.380244
#5 DONE 1.1s
2026-Feb-25 07:10:31.380244
2026-Feb-25 07:10:31.380244
#8 [gpu-worker internal] load metadata for docker.io/nvidia/cuda:12.3.2-runtime-ubuntu22.04
2026-Feb-25 07:10:31.380244
#8 DONE 1.1s
2026-Feb-25 07:10:31.380244
2026-Feb-25 07:10:31.380244
#9 [gpu-worker internal] load metadata for docker.io/nvidia/cuda:12.3.2-devel-ubuntu22.04
2026-Feb-25 07:10:31.380244
#9 DONE 1.1s
2026-Feb-25 07:10:31.380244
2026-Feb-25 07:10:31.380244
#10 [frontend internal] load .dockerignore
2026-Feb-25 07:10:31.380244
#10 transferring context: 341B done
2026-Feb-25 07:10:31.380244
#10 DONE 0.0s
2026-Feb-25 07:10:31.380244
2026-Feb-25 07:10:31.380244
#11 [api runtime 1/6] FROM docker.io/library/debian:bookworm-slim@sha256:74d56e3931e0d5a1dd51f8c8a2466d21de84a271cd3b5a733b803aa91abf4421
2026-Feb-25 07:10:31.380244
#11 resolve docker.io/library/debian:bookworm-slim@sha256:74d56e3931e0d5a1dd51f8c8a2466d21de84a271cd3b5a733b803aa91abf4421 0.0s done
2026-Feb-25 07:10:31.380244
#11 DONE 0.0s
2026-Feb-25 07:10:31.380244
2026-Feb-25 07:10:31.380244
#12 [api runtime 2/6] WORKDIR /app
2026-Feb-25 07:10:31.380244
#12 CACHED
2026-Feb-25 07:10:31.380244
2026-Feb-25 07:10:31.380244
#13 [api builder 1/9] FROM docker.io/library/rust:1.88-bookworm@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0
2026-Feb-25 07:10:31.380244
#13 resolve docker.io/library/rust:1.88-bookworm@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0 0.0s done
2026-Feb-25 07:10:31.380244
#13 DONE 0.0s
2026-Feb-25 07:10:31.380244
2026-Feb-25 07:10:31.380244
#14 [api builder 2/9] WORKDIR /app
2026-Feb-25 07:10:31.380244
#14 CACHED
2026-Feb-25 07:10:31.380244
2026-Feb-25 07:10:31.380244
#15 [gpu-worker builder 1/9] FROM docker.io/nvidia/cuda:12.3.2-devel-ubuntu22.04@sha256:6655d5fc2fb48580255a5021a81c379c325a457b74b77ac823ed67e4faa32aeb
2026-Feb-25 07:10:31.380244
#15 resolve docker.io/nvidia/cuda:12.3.2-devel-ubuntu22.04@sha256:6655d5fc2fb48580255a5021a81c379c325a457b74b77ac823ed67e4faa32aeb 0.0s done
2026-Feb-25 07:10:31.380244
#15 DONE 0.0s
2026-Feb-25 07:10:31.380244
2026-Feb-25 07:10:31.380244
#16 [gpu-worker builder 2/9] WORKDIR /app
2026-Feb-25 07:10:31.380244
#16 CACHED
2026-Feb-25 07:10:31.380244
2026-Feb-25 07:10:31.380244
#17 [api builder 1/5] FROM docker.io/library/node:22-alpine@sha256:e4bf2a82ad0a4037d28035ae71529873c069b13eb0455466ae0bc13363826e34
2026-Feb-25 07:10:31.380244
#17 resolve docker.io/library/node:22-alpine@sha256:e4bf2a82ad0a4037d28035ae71529873c069b13eb0455466ae0bc13363826e34 0.0s done
2026-Feb-25 07:10:31.380244
#17 DONE 0.0s
2026-Feb-25 07:10:31.380244
2026-Feb-25 07:10:31.380244
#18 [gpu-worker runtime 1/6] FROM docker.io/nvidia/cuda:12.3.2-runtime-ubuntu22.04@sha256:882b43fadd789693f08de95e6014ed5f0ea118c7b342150876e153b4340e1103
2026-Feb-25 07:10:31.380244
#18 resolve docker.io/nvidia/cuda:12.3.2-runtime-ubuntu22.04@sha256:882b43fadd789693f08de95e6014ed5f0ea118c7b342150876e153b4340e1103 0.0s done
2026-Feb-25 07:10:31.380244
#18 DONE 0.0s
2026-Feb-25 07:10:31.380244
2026-Feb-25 07:10:31.380244
#19 [api builder 2/5] WORKDIR /app
2026-Feb-25 07:10:31.380244
#19 CACHED
2026-Feb-25 07:10:31.380244
2026-Feb-25 07:10:31.380244
#20 [gpu-worker runtime 2/6] WORKDIR /app
2026-Feb-25 07:10:31.380244
#20 CACHED
2026-Feb-25 07:10:31.380244
2026-Feb-25 07:10:31.380244
#21 [gpu-worker internal] load build context
2026-Feb-25 07:10:31.380244
#21 transferring context: 475.95kB 0.0s done
2026-Feb-25 07:10:31.380244
#21 DONE 0.0s
2026-Feb-25 07:10:31.380244
2026-Feb-25 07:10:31.380244
#22 [api internal] load build context
2026-Feb-25 07:10:31.380244
#22 transferring context: 600.49kB 0.0s done
2026-Feb-25 07:10:31.380244
#22 DONE 0.0s
2026-Feb-25 07:10:31.380244
2026-Feb-25 07:10:31.380244
#23 [frontend internal] load build context
2026-Feb-25 07:10:31.380244
#23 transferring context: 197.18kB done
2026-Feb-25 07:10:31.380244
#23 DONE 0.0s
2026-Feb-25 07:10:31.380244
2026-Feb-25 07:10:31.380244
#24 [gpu-worker runtime 3/6] RUN apt-get update && apt-get install -y     ca-certificates     libssl3     ffmpeg     libavcodec58     libavformat58     libavutil56     libswscale5     libavfilter7     libavdevice58     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:10:31.533713
#24 ...
2026-Feb-25 07:10:31.533713
2026-Feb-25 07:10:31.533713
#25 [frontend builder 3/5] COPY frontend/ ./
2026-Feb-25 07:10:31.533713
#25 CACHED
2026-Feb-25 07:10:31.683730
#26 [frontend builder 4/5] RUN npm install
2026-Feb-25 07:10:33.057622
#26 ...
2026-Feb-25 07:10:33.057622
2026-Feb-25 07:10:33.057622
#27 [api js-builder  3/10] RUN npm install -g pnpm
2026-Feb-25 07:10:33.057622
#27 1.605
2026-Feb-25 07:10:33.057622
#27 1.605 added 1 package in 1s
2026-Feb-25 07:10:33.057622
#27 1.605
2026-Feb-25 07:10:33.057622
#27 1.605 1 package is looking for funding
2026-Feb-25 07:10:33.057622
#27 1.605   run `npm fund` for details
2026-Feb-25 07:10:33.057622
#27 1.606 npm notice
2026-Feb-25 07:10:33.057622
#27 1.606 npm notice New major version of npm available! 10.9.4 -> 11.10.1
2026-Feb-25 07:10:33.057622
#27 1.606 npm notice Changelog: https://github.com/npm/cli/releases/tag/v11.10.1
2026-Feb-25 07:10:33.057622
#27 1.606 npm notice To update run: npm install -g npm@11.10.1
2026-Feb-25 07:10:33.057622
#27 1.606 npm notice
2026-Feb-25 07:10:33.057622
#27 DONE 1.7s
2026-Feb-25 07:10:33.164839
#28 [api js-builder  4/10] COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Feb-25 07:10:33.164839
#28 DONE 0.0s
2026-Feb-25 07:10:33.164839
2026-Feb-25 07:10:33.164839
#29 [api js-builder  5/10] COPY packages/api-client/package.json ./packages/api-client/
2026-Feb-25 07:10:33.164839
#29 DONE 0.0s
2026-Feb-25 07:10:33.164839
2026-Feb-25 07:10:33.164839
#30 [api js-builder  6/10] COPY apps/injector/package.json ./apps/injector/
2026-Feb-25 07:10:33.164839
#30 DONE 0.0s
2026-Feb-25 07:10:33.164839
2026-Feb-25 07:10:33.164839
#31 [api js-builder  7/10] COPY apps/injector/ ./apps/injector/
2026-Feb-25 07:10:33.164839
#31 DONE 0.0s
2026-Feb-25 07:10:33.164839
2026-Feb-25 07:10:33.164839
#32 [api js-builder  8/10] COPY packages/ ./packages/
2026-Feb-25 07:10:33.164839
#32 DONE 0.0s
2026-Feb-25 07:10:33.164839
2026-Feb-25 07:10:33.164839
#33 [api builder 3/9] RUN apt-get update && apt-get install -y     pkg-config     libssl-dev     protobuf-compiler     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:10:33.164839
#33 0.664 Get:1 http://deb.debian.org/debian bookworm InRelease [151 kB]
2026-Feb-25 07:10:33.164839
#33 1.228 Get:2 http://deb.debian.org/debian bookworm-updates InRelease [55.4 kB]
2026-Feb-25 07:10:33.164839
#33 1.431 Get:3 http://deb.debian.org/debian-security bookworm-security InRelease [48.0 kB]
2026-Feb-25 07:10:33.164839
#33 1.630 Get:4 http://deb.debian.org/debian bookworm/main amd64 Packages [8792 kB]
2026-Feb-25 07:10:34.581608
#33 3.253 Get:5 http://deb.debian.org/debian bookworm-updates/main amd64 Packages [6924 B]
2026-Feb-25 07:10:34.755685
#33 3.427 Get:6 http://deb.debian.org/debian-security bookworm-security/main amd64 Packages [297 kB]
2026-Feb-25 07:10:35.397435
#33 4.017 Fetched 9350 kB in 4s (2478 kB/s)
2026-Feb-25 07:10:35.397435
#33 4.017 Reading package lists...
2026-Feb-25 07:10:35.517693
#33 ...
2026-Feb-25 07:10:35.517693
2026-Feb-25 07:10:35.517693
#34 [api js-builder  9/10] RUN pnpm install --frozen-lockfile
2026-Feb-25 07:10:35.517693
#34 0.547 Scope: all 3 workspace projects
2026-Feb-25 07:10:35.517693
#34 0.625 Lockfile is up to date, resolution step is skipped
2026-Feb-25 07:10:35.517693
#34 0.655 Progress: resolved 1, reused 0, downloaded 0, added 0
2026-Feb-25 07:10:35.517693
#34 0.697 Packages: +102
2026-Feb-25 07:10:35.517693
#34 0.697 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
2026-Feb-25 07:10:35.517693
#34 1.656 Progress: resolved 102, reused 0, downloaded 86, added 86
2026-Feb-25 07:10:35.517693
#34 1.963 Progress: resolved 102, reused 0, downloaded 102, added 102, done
2026-Feb-25 07:10:35.517693
#34 2.027 .../esbuild@0.25.12/node_modules/esbuild postinstall$ node install.js
2026-Feb-25 07:10:35.517693
#34 2.092 .../esbuild@0.25.12/node_modules/esbuild postinstall: Done
2026-Feb-25 07:10:35.517693
#34 2.153
2026-Feb-25 07:10:35.517693
#34 2.253 Done in 2s using pnpm v10.30.2
2026-Feb-25 07:10:35.517693
#34 DONE 2.4s
2026-Feb-25 07:10:35.668712
#35 [api js-builder 10/10] RUN pnpm --filter @downloadtool/injector build
2026-Feb-25 07:10:36.086084
#35 0.568
2026-Feb-25 07:10:36.086084
#35 0.568 > @downloadtool/injector@0.0.1 build /app/apps/injector
2026-Feb-25 07:10:36.086084
#35 0.568 > vite build && vite build --config vite.userscript.config.ts
2026-Feb-25 07:10:36.086084
#35 0.568
2026-Feb-25 07:10:36.432484
#35 0.809 vite v6.4.1 building for production...
2026-Feb-25 07:10:36.432484
#35 0.862 transforming...
2026-Feb-25 07:10:36.432484
#35 0.914 ✓ 4 modules transformed.
2026-Feb-25 07:10:36.567982
#35 0.936 rendering chunks...
2026-Feb-25 07:10:36.567982
#35 0.939 computing gzip size...
2026-Feb-25 07:10:36.567982
#35 0.943 dist/bm.js  6.00 kB │ gzip: 2.27 kB
2026-Feb-25 07:10:36.567982
#35 0.944 ✓ built in 98ms
2026-Feb-25 07:10:36.809312
#35 1.212 vite v6.4.1 building for production...
2026-Feb-25 07:10:36.809312
#35 1.246 transforming...
2026-Feb-25 07:10:36.809312
#35 1.291 ✓ 4 modules transformed.
2026-Feb-25 07:10:36.930488
#35 1.307 rendering chunks...
2026-Feb-25 07:10:36.930488
#35 1.359 computing gzip size...
2026-Feb-25 07:10:36.930488
#35 1.361 dist/youtube-downloader.user.js  10.03 kB │ gzip: 3.09 kB
2026-Feb-25 07:10:36.930488
#35 1.361 ✓ built in 138ms
2026-Feb-25 07:10:36.930488
#35 DONE 1.4s
2026-Feb-25 07:10:36.930488
2026-Feb-25 07:10:36.930488
#33 [api builder 3/9] RUN apt-get update && apt-get install -y     pkg-config     libssl-dev     protobuf-compiler     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:10:36.930488
#33 4.017 Reading package lists...
2026-Feb-25 07:10:36.930488
#33 4.515 Reading package lists...
2026-Feb-25 07:10:36.930488
#33 4.991 Building dependency tree...
2026-Feb-25 07:10:36.930488
#33 5.143 Reading state information...
2026-Feb-25 07:10:36.930488
#33 5.283 pkg-config is already the newest version (1.8.1-1).
2026-Feb-25 07:10:36.930488
#33 5.283 pkg-config set to manually installed.
2026-Feb-25 07:10:36.930488
#33 5.283 The following additional packages will be installed:
2026-Feb-25 07:10:36.930488
#33 5.284   libprotobuf-dev libprotobuf-lite32 libprotobuf32 libprotoc32 libssl3 openssl
2026-Feb-25 07:10:36.930488
#33 5.285 Suggested packages:
2026-Feb-25 07:10:36.930488
#33 5.285   libssl-doc protobuf-mode-el
2026-Feb-25 07:10:36.930488
#33 5.345 The following NEW packages will be installed:
2026-Feb-25 07:10:36.930488
#33 5.345   libprotobuf-dev libprotobuf-lite32 libprotobuf32 libprotoc32
2026-Feb-25 07:10:36.930488
#33 5.345   protobuf-compiler
2026-Feb-25 07:10:36.930488
#33 5.346 The following packages will be upgraded:
2026-Feb-25 07:10:36.930488
#33 5.346   libssl-dev libssl3 openssl
2026-Feb-25 07:10:37.203671
#33 5.757 3 upgraded, 5 newly installed, 0 to remove and 105 not upgraded.
2026-Feb-25 07:10:37.203671
#33 5.757 Need to get 9295 kB of archives.
2026-Feb-25 07:10:37.203671
#33 5.757 After this operation, 19.7 MB of additional disk space will be used.
2026-Feb-25 07:10:37.203671
#33 5.757 Get:1 http://deb.debian.org/debian bookworm/main amd64 libprotobuf32 amd64 3.21.12-3 [932 kB]
2026-Feb-25 07:10:38.462628
#33 7.134 Get:2 http://deb.debian.org/debian bookworm/main amd64 libprotobuf-lite32 amd64 3.21.12-3 [261 kB]
2026-Feb-25 07:10:38.588506
#33 7.173 Get:3 http://deb.debian.org/debian bookworm/main amd64 libprotobuf-dev amd64 3.21.12-3 [1283 kB]
2026-Feb-25 07:10:39.302617
#33 7.929 Get:4 http://deb.debian.org/debian bookworm/main amd64 libprotoc32 amd64 3.21.12-3 [829 kB]
2026-Feb-25 07:10:39.302617
#33 7.936 Get:5 http://deb.debian.org/debian-security bookworm-security/main amd64 libssl-dev amd64 3.0.18-1~deb12u2 [2444 kB]
2026-Feb-25 07:10:39.940133
#33 8.461 Get:6 http://deb.debian.org/debian-security bookworm-security/main amd64 libssl3 amd64 3.0.18-1~deb12u2 [2030 kB]
2026-Feb-25 07:10:40.507510
#33 9.017 Get:7 http://deb.debian.org/debian-security bookworm-security/main amd64 openssl amd64 3.0.18-1~deb12u2 [1433 kB]
2026-Feb-25 07:10:40.846301
#33 9.433 Get:8 http://deb.debian.org/debian bookworm/main amd64 protobuf-compiler amd64 3.21.12-3 [83.9 kB]
2026-Feb-25 07:10:40.982149
#33 ...
2026-Feb-25 07:10:40.982149
2026-Feb-25 07:10:40.982149
#36 [api runtime 3/6] RUN apt-get update && apt-get install -y     ca-certificates     libssl3     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:10:40.982149
#36 0.635 Get:1 http://deb.debian.org/debian bookworm InRelease [151 kB]
2026-Feb-25 07:10:40.982149
#36 1.217 Get:2 http://deb.debian.org/debian bookworm-updates InRelease [55.4 kB]
2026-Feb-25 07:10:40.982149
#36 1.429 Get:3 http://deb.debian.org/debian-security bookworm-security InRelease [48.0 kB]
2026-Feb-25 07:10:40.982149
#36 1.637 Get:4 http://deb.debian.org/debian bookworm/main amd64 Packages [8792 kB]
2026-Feb-25 07:10:40.982149
#36 3.339 Get:5 http://deb.debian.org/debian bookworm-updates/main amd64 Packages [6924 B]
2026-Feb-25 07:10:40.982149
#36 3.522 Get:6 http://deb.debian.org/debian-security bookworm-security/main amd64 Packages [297 kB]
2026-Feb-25 07:10:40.982149
#36 4.075 Fetched 9350 kB in 4s (2450 kB/s)
2026-Feb-25 07:10:40.982149
#36 4.075 Reading package lists...
2026-Feb-25 07:10:40.982149
#36 4.556 Reading package lists...
2026-Feb-25 07:10:40.982149
#36 5.045 Building dependency tree...
2026-Feb-25 07:10:40.982149
#36 5.172 Reading state information...
2026-Feb-25 07:10:40.982149
#36 5.320 The following additional packages will be installed:
2026-Feb-25 07:10:40.982149
#36 5.320   openssl
2026-Feb-25 07:10:40.982149
#36 5.341 The following NEW packages will be installed:
2026-Feb-25 07:10:40.982149
#36 5.342   ca-certificates libssl3 openssl
2026-Feb-25 07:10:40.982149
#36 5.720 0 upgraded, 3 newly installed, 0 to remove and 0 not upgraded.
2026-Feb-25 07:10:40.982149
#36 5.720 Need to get 3617 kB of archives.
2026-Feb-25 07:10:40.982149
#36 5.720 After this operation, 8928 kB of additional disk space will be used.
2026-Feb-25 07:10:40.982149
#36 5.720 Get:1 http://deb.debian.org/debian-security bookworm-security/main amd64 libssl3 amd64 3.0.18-1~deb12u2 [2030 kB]
2026-Feb-25 07:10:40.985686
#36 7.266 Get:2 http://deb.debian.org/debian-security bookworm-security/main amd64 openssl amd64 3.0.18-1~deb12u2 [1433 kB]
2026-Feb-25 07:10:40.985686
#36 7.454 Get:3 http://deb.debian.org/debian bookworm/main amd64 ca-certificates all 20230311+deb12u1 [155 kB]
2026-Feb-25 07:10:40.985686
#36 7.580 debconf: delaying package configuration, since apt-utils is not installed
2026-Feb-25 07:10:40.985686
#36 7.615 Fetched 3617 kB in 2s (1713 kB/s)
2026-Feb-25 07:10:40.985686
#36 7.632 Selecting previously unselected package libssl3:amd64.
2026-Feb-25 07:10:40.985686
#36 7.632 (Reading database ... 
(Reading database ... 5%
(Reading database ... 10%
(Reading database ... 15%
(Reading database ... 20%
(Reading database ... 25%
(Reading database ... 30%
(Reading database ... 35%
(Reading database ... 40%
(Reading database ... 45%
(Reading database ... 50%
(Reading database ... 55%
(Reading database ... 60%
(Reading database ... 65%
(Reading database ... 70%
(Reading database ... 75%
(Reading database ... 80%
(Reading database ... 85%
(Reading database ... 90%
(Reading database ... 95%
(Reading database ... 100%
(Reading database ... 6096 files and directories currently installed.)
2026-Feb-25 07:10:40.985686
#36 7.637 Preparing to unpack .../libssl3_3.0.18-1~deb12u2_amd64.deb ...
2026-Feb-25 07:10:40.985686
#36 7.641 Unpacking libssl3:amd64 (3.0.18-1~deb12u2) ...
2026-Feb-25 07:10:40.985686
#36 7.763 Selecting previously unselected package openssl.
2026-Feb-25 07:10:40.985686
#36 7.764 Preparing to unpack .../openssl_3.0.18-1~deb12u2_amd64.deb ...
2026-Feb-25 07:10:40.985686
#36 7.765 Unpacking openssl (3.0.18-1~deb12u2) ...
2026-Feb-25 07:10:40.985686
#36 7.858 Selecting previously unselected package ca-certificates.
2026-Feb-25 07:10:40.985686
#36 7.859 Preparing to unpack .../ca-certificates_20230311+deb12u1_all.deb ...
2026-Feb-25 07:10:40.985686
#36 7.860 Unpacking ca-certificates (20230311+deb12u1) ...
2026-Feb-25 07:10:40.985686
#36 7.903 Setting up libssl3:amd64 (3.0.18-1~deb12u2) ...
2026-Feb-25 07:10:40.985686
#36 7.906 Setting up openssl (3.0.18-1~deb12u2) ...
2026-Feb-25 07:10:40.985686
#36 7.912 Setting up ca-certificates (20230311+deb12u1) ...
2026-Feb-25 07:10:40.985686
#36 7.980 debconf: unable to initialize frontend: Dialog
2026-Feb-25 07:10:40.985686
#36 7.980 debconf: (TERM is not set, so the dialog frontend is not usable.)
2026-Feb-25 07:10:40.985686
#36 7.980 debconf: falling back to frontend: Readline
2026-Feb-25 07:10:40.985686
#36 7.980 debconf: unable to initialize frontend: Readline
2026-Feb-25 07:10:40.985686
#36 7.980 debconf: (Can't locate Term/ReadLine.pm in @INC (you may need to install the Term::ReadLine module) (@INC contains: /etc/perl /usr/local/lib/x86_64-linux-gnu/perl/5.36.0 /usr/local/share/perl/5.36.0 /usr/lib/x86_64-linux-gnu/perl5/5.36 /usr/share/perl5 /usr/lib/x86_64-linux-gnu/perl-base /usr/lib/x86_64-linux-gnu/perl/5.36 /usr/share/perl/5.36 /usr/local/lib/site_perl) at /usr/share/perl5/Debconf/FrontEnd/Readline.pm line 7.)
2026-Feb-25 07:10:40.985686
#36 7.980 debconf: falling back to frontend: Teletype
2026-Feb-25 07:10:40.985686
#36 8.383 Updating certificates in /etc/ssl/certs...
2026-Feb-25 07:10:40.985686
#36 8.939 142 added, 0 removed; done.
2026-Feb-25 07:10:40.985686
#36 8.954 Processing triggers for libc-bin (2.36-9+deb12u13) ...
2026-Feb-25 07:10:40.985686
#36 8.962 Processing triggers for ca-certificates (20230311+deb12u1) ...
2026-Feb-25 07:10:40.985686
#36 8.966 Updating certificates in /etc/ssl/certs...
2026-Feb-25 07:10:40.985686
#36 9.391 0 added, 0 removed; done.
2026-Feb-25 07:10:40.985686
#36 9.391 Running hooks in /etc/ca-certificates/update.d...
2026-Feb-25 07:10:40.985686
#36 9.392 done.
2026-Feb-25 07:10:40.985686
#36 DONE 9.7s
2026-Feb-25 07:10:40.985686
2026-Feb-25 07:10:40.985686
#37 [gpu-worker builder 3/9] RUN apt-get update && apt-get install -y     curl     build-essential     pkg-config     libssl-dev     protobuf-compiler     ffmpeg     libavcodec-dev     libavformat-dev     libavutil-dev     libswscale-dev     libavfilter-dev     libavdevice-dev     clang     libclang-dev     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:10:40.985686
#37 0.813 Get:1 http://archive.ubuntu.com/ubuntu jammy InRelease [270 kB]
2026-Feb-25 07:10:40.985686
#37 1.049 Get:2 http://security.ubuntu.com/ubuntu jammy-security InRelease [129 kB]
2026-Feb-25 07:10:40.985686
#37 2.303 Get:3 http://security.ubuntu.com/ubuntu jammy-security/universe amd64 Packages [1302 kB]
2026-Feb-25 07:10:40.985686
#37 2.348 Get:4 http://archive.ubuntu.com/ubuntu jammy-updates InRelease [128 kB]
2026-Feb-25 07:10:40.985686
#37 2.711 Get:5 http://archive.ubuntu.com/ubuntu jammy-backports InRelease [127 kB]
2026-Feb-25 07:10:40.985686
#37 3.074 Get:6 http://archive.ubuntu.com/ubuntu jammy/universe amd64 Packages [17.5 MB]
2026-Feb-25 07:10:40.985686
#37 3.471 Get:7 https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2204/x86_64  InRelease [1581 B]
2026-Feb-25 07:10:40.985686
#37 3.952 Get:8 http://security.ubuntu.com/ubuntu jammy-security/main amd64 Packages [3767 kB]
2026-Feb-25 07:10:40.985686
#37 4.757 Get:9 http://security.ubuntu.com/ubuntu jammy-security/restricted amd64 Packages [6626 kB]
2026-Feb-25 07:10:40.985686
#37 5.334 Get:10 http://security.ubuntu.com/ubuntu jammy-security/multiverse amd64 Packages [62.6 kB]
2026-Feb-25 07:10:40.985686
#37 5.657 Get:11 http://archive.ubuntu.com/ubuntu jammy/restricted amd64 Packages [164 kB]
2026-Feb-25 07:10:40.985686
#37 5.659 Get:12 http://archive.ubuntu.com/ubuntu jammy/main amd64 Packages [1792 kB]
2026-Feb-25 07:10:40.985686
#37 5.699 Get:13 http://archive.ubuntu.com/ubuntu jammy/multiverse amd64 Packages [266 kB]
2026-Feb-25 07:10:40.985686
#37 5.704 Get:14 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 Packages [4107 kB]
2026-Feb-25 07:10:40.985686
#37 5.873 Get:15 http://archive.ubuntu.com/ubuntu jammy-updates/restricted amd64 Packages [6861 kB]
2026-Feb-25 07:10:40.985686
#37 6.023 Get:16 http://archive.ubuntu.com/ubuntu jammy-updates/multiverse amd64 Packages [70.9 kB]
2026-Feb-25 07:10:40.985686
#37 6.023 Get:17 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 Packages [1613 kB]
2026-Feb-25 07:10:40.985686
#37 6.131 Get:18 http://archive.ubuntu.com/ubuntu jammy-backports/universe amd64 Packages [37.2 kB]
2026-Feb-25 07:10:40.985686
#37 6.132 Get:19 http://archive.ubuntu.com/ubuntu jammy-backports/main amd64 Packages [83.9 kB]
2026-Feb-25 07:10:40.985686
#37 6.577 Get:20 https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2204/x86_64  Packages [2383 kB]
2026-Feb-25 07:10:40.985686
#37 6.711 Fetched 47.3 MB in 6s (7386 kB/s)
2026-Feb-25 07:10:40.985686
#37 6.711 Reading package lists...
2026-Feb-25 07:10:40.985686
#37 7.794 Reading package lists...
2026-Feb-25 07:10:40.985686
#37 8.795 Building dependency tree...
2026-Feb-25 07:10:40.985686
#37 9.027 Reading state information...
2026-Feb-25 07:10:40.985686
#37 9.234 build-essential is already the newest version (12.9ubuntu3).
2026-Feb-25 07:10:40.985686
#37 9.234 build-essential set to manually installed.
2026-Feb-25 07:10:40.985686
#37 9.234 The following additional packages will be installed:
2026-Feb-25 07:10:40.985686
#37 9.234   alsa-topology-conf alsa-ucm-conf binfmt-support clang-14 cpp-11 dbus
2026-Feb-25 07:10:40.985686
#37 9.234   fontconfig fontconfig-config fonts-dejavu-core g++-11 gcc-11 gcc-11-base
2026-Feb-25 07:10:40.985686
#37 9.234   gcc-12-base i965-va-driver icu-devtools intel-media-va-driver lib32gcc-s1
2026-Feb-25 07:10:40.985686
#37 9.234   lib32stdc++6 libaacs0 libaom3 libapparmor1 libasan6 libasound2
2026-Feb-25 07:10:40.985686
#37 9.234   libasound2-data libass9 libasyncns0 libatomic1 libavc1394-0 libavcodec58
2026-Feb-25 07:10:40.985686
#37 9.234   libavdevice58 libavfilter7 libavformat58 libavutil56 libbdplus0 libblas3
2026-Feb-25 07:10:40.985686
#37 9.234   libbluray2 libbrotli1 libbs2b0 libbsd0 libc-dev-bin libc6 libc6-dev
2026-Feb-25 07:10:40.985686
#37 9.234   libc6-i386 libcaca0 libcairo-gobject2 libcairo2 libcc1-0 libcdio-cdda2
2026-Feb-25 07:10:40.985686
#37 9.234   libcdio-paranoia2 libcdio19 libchromaprint1 libclang-14-dev
2026-Feb-25 07:10:40.985686
#37 9.234   libclang-common-14-dev libclang-cpp14 libclang1-14 libcodec2-1.0 libcurl4
2026-Feb-25 07:10:40.985686
#37 9.234   libdatrie1 libdav1d5 libdbus-1-3 libdc1394-25 libdecor-0-0
2026-Feb-25 07:10:40.985686
#37 9.234   libdecor-0-plugin-1-cairo libdeflate0 libdrm-amdgpu1 libdrm-common
2026-Feb-25 07:10:40.985686
#37 9.234   libdrm-intel1 libdrm-nouveau2 libdrm-radeon1 libdrm2 libedit2 libelf1
2026-Feb-25 07:10:40.985686
#37 9.234   libexpat1 libffi-dev libflac8 libflite1 libfontconfig1 libfreetype6
2026-Feb-25 07:10:40.985686
#37 9.234   libfribidi0 libgbm1 libgc1 libgcc-11-dev libgcc-s1 libgdk-pixbuf-2.0-0
2026-Feb-25 07:10:40.985686
#37 9.234   libgdk-pixbuf2.0-bin libgdk-pixbuf2.0-common libgfortran5 libgl1
2026-Feb-25 07:10:40.985686
#37 9.234   libgl1-amber-dri libgl1-mesa-dri libglapi-mesa libglib2.0-0 libglib2.0-data
2026-Feb-25 07:10:40.985686
#37 9.234   libglvnd0 libglx-mesa0 libglx0 libgme0 libgomp1 libgraphite2-3 libgsm1
2026-Feb-25 07:10:40.985686
#37 9.234   libharfbuzz0b libicu-dev libicu70 libiec61883-0 libigdgmm12 libitm1
2026-Feb-25 07:10:40.985686
#37 9.234   libjack-jackd2-0 libjbig0 libjpeg-turbo8 libjpeg8 liblapack3 liblilv-0-0
2026-Feb-25 07:10:40.985686
#37 9.234   libllvm14 libllvm15 liblsan0 libmd0 libmfx1 libmp3lame0 libmpdec3
2026-Feb-25 07:10:40.985686
#37 9.234   libmpg123-0 libmysofa1 libncurses-dev libnghttp2-14 libnorm1 libnuma1
2026-Feb-25 07:10:40.985686
#37 9.234   libobjc-11-dev libobjc4 libogg0 libopenal-data libopenal1 libopenjp2-7
2026-Feb-25 07:10:40.985686
#37 9.234   libopenmpt0 libopus0 libpango-1.0-0 libpangocairo-1.0-0 libpangoft2-1.0-0
2026-Feb-25 07:10:40.985686
#37 9.234   libpciaccess0 libpfm4 libpgm-5.3-0 libpipeline1 libpixman-1-0 libpng16-16
2026-Feb-25 07:10:40.985686
#37 9.234   libpocketsphinx3 libpostproc-dev libpostproc55 libprotobuf-dev
2026-Feb-25 07:10:40.985686
#37 9.234   libprotobuf-lite23 libprotobuf23 libprotoc23 libpsl5 libpulse0
2026-Feb-25 07:10:40.985686
#37 9.234   libpython3-stdlib libpython3.10-minimal libpython3.10-stdlib libquadmath0
2026-Feb-25 07:10:40.985686
#37 9.234   librabbitmq4 libraw1394-11 librsvg2-2 librsvg2-common librtmp1
2026-Feb-25 07:10:40.985686
#37 9.234   librubberband2 libsamplerate0 libsdl2-2.0-0 libsensors-config libsensors5
2026-Feb-25 07:10:40.985686
#37 9.234   libserd-0-0 libshine3 libslang2 libsnappy1v5 libsndfile1 libsndio7.0
2026-Feb-25 07:10:40.985686
#37 9.234   libsodium23 libsord-0-0 libsoxr0 libspeex1 libsphinxbase3 libsratom-0-0
2026-Feb-25 07:10:40.985686
#37 9.234   libsrt1.4-gnutls libssh-4 libssh-gcrypt-4 libssl3 libstdc++-11-dev
2026-Feb-25 07:10:40.985686
#37 9.235   libstdc++6 libswresample-dev libswresample3 libswscale5 libthai-data
2026-Feb-25 07:10:40.985686
#37 9.235   libthai0 libtheora0 libtiff5 libtinfo-dev libtsan0 libtwolame0 libubsan1
2026-Feb-25 07:10:40.985686
#37 9.235   libudfread0 libusb-1.0-0 libva-drm2 libva-x11-2 libva2 libvdpau1
2026-Feb-25 07:10:40.985686
#37 9.235   libvidstab1.1 libvorbis0a libvorbisenc2 libvorbisfile3 libvpx7
2026-Feb-25 07:10:40.985686
#37 9.235   libwayland-client0 libwayland-cursor0 libwayland-egl1 libwayland-server0
2026-Feb-25 07:10:40.985686
#37 9.235   libwebp7 libwebpmux3 libx11-6 libx11-data libx11-xcb1 libx264-163
2026-Feb-25 07:10:40.985686
#37 9.235   libx265-199 libxau6 libxcb-dri2-0 libxcb-dri3-0 libxcb-glx0 libxcb-present0
2026-Feb-25 07:10:40.985686
#37 9.235   libxcb-randr0 libxcb-render0 libxcb-shape0 libxcb-shm0 libxcb-sync1
2026-Feb-25 07:10:40.985686
#37 9.235   libxcb-xfixes0 libxcb1 libxcursor1 libxdmcp6 libxext6 libxfixes3 libxi6
2026-Feb-25 07:10:40.985686
#37 9.235   libxinerama1 libxkbcommon0 libxml2 libxml2-dev libxrandr2 libxrender1
2026-Feb-25 07:10:40.985686
#37 9.235   libxshmfence1 libxss1 libxv1 libxvidcore4 libxxf86vm1 libyaml-0-2 libz3-4
2026-Feb-25 07:10:40.985686
#37 9.235   libz3-dev libzimg2 libzmq5 libzvbi-common libzvbi0 llvm-14 llvm-14-dev
2026-Feb-25 07:10:40.985686
#37 9.235   llvm-14-linker-tools llvm-14-runtime llvm-14-tools media-types
2026-Feb-25 07:10:40.985686
#37 9.235   mesa-va-drivers mesa-vdpau-drivers ocl-icd-libopencl1 pocketsphinx-en-us
2026-Feb-25 07:10:40.985686
#37 9.236   publicsuffix python3 python3-minimal python3-pkg-resources python3-pygments
2026-Feb-25 07:10:40.985686
#37 9.236   python3-yaml python3.10 python3.10-minimal shared-mime-info ucf
2026-Feb-25 07:10:40.985686
#37 9.236   va-driver-all vdpau-driver-all x11-common xdg-user-dirs xkb-data zlib1g-dev
2026-Feb-25 07:10:40.985686
#37 9.238 Suggested packages:
2026-Feb-25 07:10:40.985686
#37 9.238   clang-14-doc gcc-11-locales default-dbus-session-bus | dbus-session-bus
2026-Feb-25 07:10:40.985686
#37 9.238   ffmpeg-doc g++-11-multilib gcc-11-doc gcc-11-multilib i965-va-driver-shaders
2026-Feb-25 07:10:40.985686
#37 9.238   libasound2-plugins alsa-utils libcuda1 libnvcuvid1 libnvidia-encode1
2026-Feb-25 07:10:40.985686
#37 9.238   libbluray-bdj glibc-doc locales manpages-dev icu-doc jackd2 ncurses-doc
2026-Feb-25 07:10:40.985686
#37 9.238   libportaudio2 opus-tools pciutils pulseaudio libraw1394-doc librsvg2-bin
2026-Feb-25 07:10:40.985686
#37 9.238   xdg-utils lm-sensors serdi sndiod sordi speex libssl-doc libstdc++-11-doc
2026-Feb-25 07:10:40.985686
#37 9.238   llvm-14-doc opencl-icd protobuf-mode-el python3-doc python3-tk python3-venv
2026-Feb-25 07:10:40.985686
#37 9.238   python3-setuptools python-pygments-doc ttf-bitstream-vera python3.10-venv
2026-Feb-25 07:10:40.985686
#37 9.238   python3.10-doc libvdpau-va-gl1
2026-Feb-25 07:10:40.985686
#37 9.238 Recommended packages:
2026-Feb-25 07:10:40.985686
#37 9.238   manpages manpages-dev libc-devtools libnss-nis libnss-nisplus
2026-Feb-25 07:10:41.093334
#37 ...
2026-Feb-25 07:10:41.093334
2026-Feb-25 07:10:41.093334
#33 [api builder 3/9] RUN apt-get update && apt-get install -y     pkg-config     libssl-dev     protobuf-compiler     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:10:41.093334
#33 9.715 debconf: delaying package configuration, since apt-utils is not installed
2026-Feb-25 07:10:41.093334
#33 9.743 Fetched 9295 kB in 4s (2277 kB/s)
2026-Feb-25 07:10:41.093334
#33 9.764 Selecting previously unselected package libprotobuf32:amd64.
2026-Feb-25 07:10:41.093334
#33 9.764 (Reading database ...
2026-Feb-25 07:10:41.194469
#33 ...
2026-Feb-25 07:10:41.194469
2026-Feb-25 07:10:41.194469
#38 [api runtime 4/6] RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-25 07:10:41.194469
#38 DONE 0.2s
2026-Feb-25 07:10:41.194469
2026-Feb-25 07:10:41.194469
#33 [api builder 3/9] RUN apt-get update && apt-get install -y     pkg-config     libssl-dev     protobuf-compiler     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:10:41.194469
#33 9.764 (Reading database ... 
(Reading database ... 5%
(Reading database ... 10%
(Reading database ... 15%
(Reading database ... 20%
(Reading database ... 25%
(Reading database ... 30%
(Reading database ... 35%
(Reading database ... 40%
(Reading database ... 45%
(Reading database ... 50%
(Reading database ... 55%
(Reading database ... 60%
(Reading database ... 65%
(Reading database ... 70%
(Reading database ... 75%
(Reading database ... 80%
(Reading database ... 85%
(Reading database ... 90%
(Reading database ... 95%
(Reading database ... 100%
(Reading database ... 23250 files and directories currently installed.)
2026-Feb-25 07:10:41.194469
#33 9.782 Preparing to unpack .../0-libprotobuf32_3.21.12-3_amd64.deb ...
2026-Feb-25 07:10:41.194469
#33 9.784 Unpacking libprotobuf32:amd64 (3.21.12-3) ...
2026-Feb-25 07:10:41.194469
#33 9.863 Selecting previously unselected package libprotobuf-lite32:amd64.
2026-Feb-25 07:10:41.194469
#33 9.866 Preparing to unpack .../1-libprotobuf-lite32_3.21.12-3_amd64.deb ...
2026-Feb-25 07:10:41.341890
#33 9.867 Unpacking libprotobuf-lite32:amd64 (3.21.12-3) ...
2026-Feb-25 07:10:41.341890
#33 9.901 Selecting previously unselected package libprotobuf-dev:amd64.
2026-Feb-25 07:10:41.341890
#33 9.904 Preparing to unpack .../2-libprotobuf-dev_3.21.12-3_amd64.deb ...
2026-Feb-25 07:10:41.341890
#33 9.905 Unpacking libprotobuf-dev:amd64 (3.21.12-3) ...
2026-Feb-25 07:10:41.341890
#33 10.01 Selecting previously unselected package libprotoc32:amd64.
2026-Feb-25 07:10:41.507842
#33 ...
2026-Feb-25 07:10:41.507842
2026-Feb-25 07:10:41.507842
#26 [frontend builder 4/5] RUN npm install
2026-Feb-25 07:10:41.507842
#26 9.148
2026-Feb-25 07:10:41.507842
#26 9.148 > frontend@0.0.1 prepare
2026-Feb-25 07:10:41.507842
#26 9.148 > svelte-kit sync || echo ''
2026-Feb-25 07:10:41.507842
#26 9.148
2026-Feb-25 07:10:41.507842
#26 9.998
2026-Feb-25 07:10:41.507842
#26 9.998 added 110 packages, and audited 111 packages in 10s
2026-Feb-25 07:10:41.507842
#26 9.998
2026-Feb-25 07:10:41.507842
#26 9.998 20 packages are looking for funding
2026-Feb-25 07:10:41.507842
#26 9.998   run `npm fund` for details
2026-Feb-25 07:10:41.507842
#26 10.01
2026-Feb-25 07:10:41.507842
#26 10.01 5 low severity vulnerabilities
2026-Feb-25 07:10:41.507842
#26 10.01
2026-Feb-25 07:10:41.507842
#26 10.01 To address all issues (including breaking changes), run:
2026-Feb-25 07:10:41.507842
#26 10.01   npm audit fix --force
2026-Feb-25 07:10:41.507842
#26 10.01
2026-Feb-25 07:10:41.507842
#26 10.01 Run `npm audit` for details.
2026-Feb-25 07:10:41.507842
#26 10.01 npm notice
2026-Feb-25 07:10:41.507842
#26 10.01 npm notice New major version of npm available! 10.9.4 -> 11.10.1
2026-Feb-25 07:10:41.507842
#26 10.01 npm notice Changelog: https://github.com/npm/cli/releases/tag/v11.10.1
2026-Feb-25 07:10:41.507842
#26 10.01 npm notice To update run: npm install -g npm@11.10.1
2026-Feb-25 07:10:41.507842
#26 10.01 npm notice
2026-Feb-25 07:10:41.507842
#26 DONE 10.1s
2026-Feb-25 07:10:41.507842
2026-Feb-25 07:10:41.507842
#33 [api builder 3/9] RUN apt-get update && apt-get install -y     pkg-config     libssl-dev     protobuf-compiler     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:10:41.507842
#33 10.02 Preparing to unpack .../3-libprotoc32_3.21.12-3_amd64.deb ...
2026-Feb-25 07:10:41.507842
#33 10.02 Unpacking libprotoc32:amd64 (3.21.12-3) ...
2026-Feb-25 07:10:41.507842
#33 10.09 Preparing to unpack .../4-libssl-dev_3.0.18-1~deb12u2_amd64.deb ...
2026-Feb-25 07:10:41.507842
#33 10.09 Unpacking libssl-dev:amd64 (3.0.18-1~deb12u2) over (3.0.16-1~deb12u1) ...
2026-Feb-25 07:10:41.739584
#33 10.27 Preparing to unpack .../5-libssl3_3.0.18-1~deb12u2_amd64.deb ...
2026-Feb-25 07:10:41.739584
#33 10.27 Unpacking libssl3:amd64 (3.0.18-1~deb12u2) over (3.0.16-1~deb12u1) ...
2026-Feb-25 07:10:41.906825
#33 10.42 Preparing to unpack .../6-openssl_3.0.18-1~deb12u2_amd64.deb ...
2026-Feb-25 07:10:41.906825
#33 10.43 Unpacking openssl (3.0.18-1~deb12u2) over (3.0.16-1~deb12u1) ...
2026-Feb-25 07:10:41.970095
#33 10.64 Selecting previously unselected package protobuf-compiler.
2026-Feb-25 07:10:42.104323
#33 10.64 Preparing to unpack .../7-protobuf-compiler_3.21.12-3_amd64.deb ...
2026-Feb-25 07:10:42.104323
#33 10.65 Unpacking protobuf-compiler (3.21.12-3) ...
2026-Feb-25 07:10:42.104323
#33 10.68 Setting up libssl3:amd64 (3.0.18-1~deb12u2) ...
2026-Feb-25 07:10:42.104323
#33 10.68 Setting up libssl-dev:amd64 (3.0.18-1~deb12u2) ...
2026-Feb-25 07:10:42.104323
#33 10.69 Setting up libprotobuf32:amd64 (3.21.12-3) ...
2026-Feb-25 07:10:42.104323
#33 10.69 Setting up libprotobuf-lite32:amd64 (3.21.12-3) ...
2026-Feb-25 07:10:42.104323
#33 10.69 Setting up openssl (3.0.18-1~deb12u2) ...
2026-Feb-25 07:10:42.104323
#33 10.70 Setting up libprotoc32:amd64 (3.21.12-3) ...
2026-Feb-25 07:10:42.104323
#33 10.70 Setting up protobuf-compiler (3.21.12-3) ...
2026-Feb-25 07:10:42.104323
#33 10.70 Setting up libprotobuf-dev:amd64 (3.21.12-3) ...
2026-Feb-25 07:10:42.104323
#33 10.71 Processing triggers for libc-bin (2.36-9+deb12u10) ...
2026-Feb-25 07:10:42.104323
#33 DONE 10.8s
2026-Feb-25 07:10:42.104323
2026-Feb-25 07:10:42.104323
#37 [gpu-worker builder 3/9] RUN apt-get update && apt-get install -y     curl     build-essential     pkg-config     libssl-dev     protobuf-compiler     ffmpeg     libavcodec-dev     libavformat-dev     libavutil-dev     libswscale-dev     libavfilter-dev     libavdevice-dev     clang     libclang-dev     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:10:42.104323
#37 9.765 The following NEW packages will be installed:
2026-Feb-25 07:10:42.104323
#37 9.765   alsa-topology-conf alsa-ucm-conf binfmt-support clang clang-14 curl dbus
2026-Feb-25 07:10:42.104323
#37 9.765   ffmpeg fontconfig fontconfig-config fonts-dejavu-core i965-va-driver
2026-Feb-25 07:10:42.104323
#37 9.765   icu-devtools intel-media-va-driver lib32gcc-s1 lib32stdc++6 libaacs0 libaom3
2026-Feb-25 07:10:42.104323
#37 9.765   libapparmor1 libasound2 libasound2-data libass9 libasyncns0 libavc1394-0
2026-Feb-25 07:10:42.104323
#37 9.765   libavcodec-dev libavcodec58 libavdevice-dev libavdevice58 libavfilter-dev
2026-Feb-25 07:10:42.104323
#37 9.765   libavfilter7 libavformat-dev libavformat58 libavutil-dev libavutil56
2026-Feb-25 07:10:42.104323
#37 9.765   libbdplus0 libblas3 libbluray2 libbrotli1 libbs2b0 libbsd0 libc6-i386
2026-Feb-25 07:10:42.104323
#37 9.765   libcaca0 libcairo-gobject2 libcairo2 libcdio-cdda2 libcdio-paranoia2
2026-Feb-25 07:10:42.104323
#37 9.765   libcdio19 libchromaprint1 libclang-14-dev libclang-common-14-dev
2026-Feb-25 07:10:42.104323
#37 9.765   libclang-cpp14 libclang-dev libclang1-14 libcodec2-1.0 libcurl4 libdatrie1
2026-Feb-25 07:10:42.104323
#37 9.765   libdav1d5 libdbus-1-3 libdc1394-25 libdecor-0-0 libdecor-0-plugin-1-cairo
2026-Feb-25 07:10:42.104323
#37 9.765   libdeflate0 libdrm-amdgpu1 libdrm-common libdrm-intel1 libdrm-nouveau2
2026-Feb-25 07:10:42.104323
#37 9.765   libdrm-radeon1 libdrm2 libedit2 libelf1 libexpat1 libffi-dev libflac8
2026-Feb-25 07:10:42.104323
#37 9.765   libflite1 libfontconfig1 libfreetype6 libfribidi0 libgbm1 libgc1
2026-Feb-25 07:10:42.104323
#37 9.765   libgdk-pixbuf-2.0-0 libgdk-pixbuf2.0-bin libgdk-pixbuf2.0-common
2026-Feb-25 07:10:42.104323
#37 9.765   libgfortran5 libgl1 libgl1-amber-dri libgl1-mesa-dri libglapi-mesa
2026-Feb-25 07:10:42.104323
#37 9.765   libglib2.0-0 libglib2.0-data libglvnd0 libglx-mesa0 libglx0 libgme0
2026-Feb-25 07:10:42.104323
#37 9.765   libgraphite2-3 libgsm1 libharfbuzz0b libicu-dev libicu70 libiec61883-0
2026-Feb-25 07:10:42.104323
#37 9.765   libigdgmm12 libjack-jackd2-0 libjbig0 libjpeg-turbo8 libjpeg8 liblapack3
2026-Feb-25 07:10:42.104323
#37 9.765   liblilv-0-0 libllvm14 libllvm15 libmd0 libmfx1 libmp3lame0 libmpdec3
2026-Feb-25 07:10:42.104323
#37 9.765   libmpg123-0 libmysofa1 libncurses-dev libnghttp2-14 libnorm1 libnuma1
2026-Feb-25 07:10:42.104323
#37 9.765   libobjc-11-dev libobjc4 libogg0 libopenal-data libopenal1 libopenjp2-7
2026-Feb-25 07:10:42.104323
#37 9.765   libopenmpt0 libopus0 libpango-1.0-0 libpangocairo-1.0-0 libpangoft2-1.0-0
2026-Feb-25 07:10:42.104323
#37 9.765   libpciaccess0 libpfm4 libpgm-5.3-0 libpipeline1 libpixman-1-0 libpng16-16
2026-Feb-25 07:10:42.104323
#37 9.765   libpocketsphinx3 libpostproc-dev libpostproc55 libprotobuf-dev
2026-Feb-25 07:10:42.104323
#37 9.765   libprotobuf-lite23 libprotobuf23 libprotoc23 libpsl5 libpulse0
2026-Feb-25 07:10:42.104323
#37 9.765   libpython3-stdlib libpython3.10-minimal libpython3.10-stdlib librabbitmq4
2026-Feb-25 07:10:42.104323
#37 9.765   libraw1394-11 librsvg2-2 librsvg2-common librtmp1 librubberband2
2026-Feb-25 07:10:42.104323
#37 9.765   libsamplerate0 libsdl2-2.0-0 libsensors-config libsensors5 libserd-0-0
2026-Feb-25 07:10:42.104323
#37 9.765   libshine3 libslang2 libsnappy1v5 libsndfile1 libsndio7.0 libsodium23
2026-Feb-25 07:10:42.104323
#37 9.765   libsord-0-0 libsoxr0 libspeex1 libsphinxbase3 libsratom-0-0 libsrt1.4-gnutls
2026-Feb-25 07:10:42.104323
#37 9.765   libssh-4 libssh-gcrypt-4 libssl-dev libswresample-dev libswresample3
2026-Feb-25 07:10:42.104323
#37 9.765   libswscale-dev libswscale5 libthai-data libthai0 libtheora0 libtiff5
2026-Feb-25 07:10:42.104323
#37 9.765   libtinfo-dev libtwolame0 libudfread0 libusb-1.0-0 libva-drm2 libva-x11-2
2026-Feb-25 07:10:42.104323
#37 9.766   libva2 libvdpau1 libvidstab1.1 libvorbis0a libvorbisenc2 libvorbisfile3
2026-Feb-25 07:10:42.104323
#37 9.766   libvpx7 libwayland-client0 libwayland-cursor0 libwayland-egl1
2026-Feb-25 07:10:42.104323
#37 9.766   libwayland-server0 libwebp7 libwebpmux3 libx11-6 libx11-data libx11-xcb1
2026-Feb-25 07:10:42.104323
#37 9.766   libx264-163 libx265-199 libxau6 libxcb-dri2-0 libxcb-dri3-0 libxcb-glx0
2026-Feb-25 07:10:42.104323
#37 9.766   libxcb-present0 libxcb-randr0 libxcb-render0 libxcb-shape0 libxcb-shm0
2026-Feb-25 07:10:42.104323
#37 9.766   libxcb-sync1 libxcb-xfixes0 libxcb1 libxcursor1 libxdmcp6 libxext6
2026-Feb-25 07:10:42.104323
#37 9.766   libxfixes3 libxi6 libxinerama1 libxkbcommon0 libxml2 libxml2-dev libxrandr2
2026-Feb-25 07:10:42.104323
#37 9.766   libxrender1 libxshmfence1 libxss1 libxv1 libxvidcore4 libxxf86vm1
2026-Feb-25 07:10:42.104323
#37 9.766   libyaml-0-2 libz3-4 libz3-dev libzimg2 libzmq5 libzvbi-common libzvbi0
2026-Feb-25 07:10:42.104323
#37 9.766   llvm-14 llvm-14-dev llvm-14-linker-tools llvm-14-runtime llvm-14-tools
2026-Feb-25 07:10:42.104323
#37 9.766   media-types mesa-va-drivers mesa-vdpau-drivers ocl-icd-libopencl1 pkg-config
2026-Feb-25 07:10:42.104323
#37 9.766   pocketsphinx-en-us protobuf-compiler publicsuffix python3 python3-minimal
2026-Feb-25 07:10:42.104323
#37 9.766   python3-pkg-resources python3-pygments python3-yaml python3.10
2026-Feb-25 07:10:42.104323
#37 9.766   python3.10-minimal shared-mime-info ucf va-driver-all vdpau-driver-all
2026-Feb-25 07:10:42.104323
#37 9.766   x11-common xdg-user-dirs xkb-data zlib1g-dev
2026-Feb-25 07:10:42.104323
#37 9.767 The following packages will be upgraded:
2026-Feb-25 07:10:42.104323
#37 9.767   cpp-11 g++-11 gcc-11 gcc-11-base gcc-12-base libasan6 libatomic1
2026-Feb-25 07:10:42.104323
#37 9.767   libc-dev-bin libc6 libc6-dev libcc1-0 libgcc-11-dev libgcc-s1 libgomp1
2026-Feb-25 07:10:42.104323
#37 9.768   libitm1 liblsan0 libquadmath0 libssl3 libstdc++-11-dev libstdc++6 libtsan0
2026-Feb-25 07:10:42.104323
#37 9.768   libubsan1
2026-Feb-25 07:10:42.104323
#37 10.41 22 upgraded, 268 newly installed, 0 to remove and 69 not upgraded.
2026-Feb-25 07:10:42.104323
#37 10.41 Need to get 392 MB of archives.
2026-Feb-25 07:10:42.104323
#37 10.41 After this operation, 1540 MB of additional disk space will be used.
2026-Feb-25 07:10:42.104323
#37 10.41 Get:1 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libc6-dev amd64 2.35-0ubuntu3.13 [2101 kB]
2026-Feb-25 07:10:42.221945
#37 ...
2026-Feb-25 07:10:42.221945
2026-Feb-25 07:10:42.221945
#39 [api builder 4/9] COPY Cargo.toml ./
2026-Feb-25 07:10:42.221945
#39 DONE 0.0s
2026-Feb-25 07:10:42.221945
2026-Feb-25 07:10:42.221945
#40 [api builder 5/9] COPY Cargo.lock ./
2026-Feb-25 07:10:42.221945
#40 DONE 0.0s
2026-Feb-25 07:10:42.221945
2026-Feb-25 07:10:42.221945
#41 [api builder 6/9] COPY crates/ ./crates/
2026-Feb-25 07:10:42.221945
#41 DONE 0.0s
2026-Feb-25 07:10:42.221945
2026-Feb-25 07:10:42.221945
#42 [api builder 7/9] COPY proto/ ./proto/
2026-Feb-25 07:10:42.221945
#42 DONE 0.0s
2026-Feb-25 07:10:42.221945
2026-Feb-25 07:10:42.221945
#43 [api builder 8/9] COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Feb-25 07:10:42.221945
#43 DONE 0.0s
2026-Feb-25 07:10:42.221945
2026-Feb-25 07:10:42.221945
#24 [gpu-worker runtime 3/6] RUN apt-get update && apt-get install -y     ca-certificates     libssl3     ffmpeg     libavcodec58     libavformat58     libavutil56     libswscale5     libavfilter7     libavdevice58     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:10:42.221945
#24 0.837 Get:1 http://security.ubuntu.com/ubuntu jammy-security InRelease [129 kB]
2026-Feb-25 07:10:42.221945
#24 0.871 Get:2 http://archive.ubuntu.com/ubuntu jammy InRelease [270 kB]
2026-Feb-25 07:10:42.221945
#24 2.109 Get:3 http://security.ubuntu.com/ubuntu jammy-security/main amd64 Packages [3767 kB]
2026-Feb-25 07:10:42.221945
#24 2.538 Get:4 http://archive.ubuntu.com/ubuntu jammy-updates InRelease [128 kB]
2026-Feb-25 07:10:42.221945
#24 2.931 Get:5 http://archive.ubuntu.com/ubuntu jammy-backports InRelease [127 kB]
2026-Feb-25 07:10:42.221945
#24 3.325 Get:6 http://archive.ubuntu.com/ubuntu jammy/restricted amd64 Packages [164 kB]
2026-Feb-25 07:10:42.221945
#24 3.485 Get:7 http://archive.ubuntu.com/ubuntu jammy/universe amd64 Packages [17.5 MB]
2026-Feb-25 07:10:42.221945
#24 4.486 Get:8 http://security.ubuntu.com/ubuntu jammy-security/multiverse amd64 Packages [62.6 kB]
2026-Feb-25 07:10:42.221945
#24 4.491 Get:9 http://security.ubuntu.com/ubuntu jammy-security/restricted amd64 Packages [6626 kB]
2026-Feb-25 07:10:42.221945
#24 5.171 Get:10 http://security.ubuntu.com/ubuntu jammy-security/universe amd64 Packages [1302 kB]
2026-Feb-25 07:10:42.221945
#24 5.222 Get:11 https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2204/x86_64  InRelease [1581 B]
2026-Feb-25 07:10:42.221945
#24 6.139 Get:12 http://archive.ubuntu.com/ubuntu jammy/multiverse amd64 Packages [266 kB]
2026-Feb-25 07:10:42.221945
#24 6.142 Get:13 http://archive.ubuntu.com/ubuntu jammy/main amd64 Packages [1792 kB]
2026-Feb-25 07:10:42.221945
#24 6.185 Get:14 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 Packages [1613 kB]
2026-Feb-25 07:10:42.221945
#24 6.224 Get:15 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 Packages [4107 kB]
2026-Feb-25 07:10:42.221945
#24 6.406 Get:16 http://archive.ubuntu.com/ubuntu jammy-updates/restricted amd64 Packages [6861 kB]
2026-Feb-25 07:10:42.221945
#24 6.649 Get:17 http://archive.ubuntu.com/ubuntu jammy-updates/multiverse amd64 Packages [70.9 kB]
2026-Feb-25 07:10:42.221945
#24 6.650 Get:18 http://archive.ubuntu.com/ubuntu jammy-backports/universe amd64 Packages [37.2 kB]
2026-Feb-25 07:10:42.221945
#24 6.650 Get:19 http://archive.ubuntu.com/ubuntu jammy-backports/main amd64 Packages [83.9 kB]
2026-Feb-25 07:10:42.221945
#24 9.500 Get:20 https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2204/x86_64  Packages [2383 kB]
2026-Feb-25 07:10:42.221945
#24 9.659 Fetched 47.3 MB in 9s (5061 kB/s)
2026-Feb-25 07:10:42.221945
#24 9.659 Reading package lists...
2026-Feb-25 07:10:42.221945
#24 10.72 Reading package lists...
2026-Feb-25 07:10:42.373375
#24 ...
2026-Feb-25 07:10:42.373375
2026-Feb-25 07:10:42.373375
#44 [api builder 9/9] RUN cargo build --release --bin vps-gateway
2026-Feb-25 07:10:42.451711
#44 0.229     Updating crates.io index
2026-Feb-25 07:10:46.029953
#44 ...
2026-Feb-25 07:10:46.029953
2026-Feb-25 07:10:46.029953
#45 [frontend builder 5/5] RUN node build-docker.mjs
2026-Feb-25 07:10:46.029953
#45 1.164 The following Vite config options will be overridden by SvelteKit:
2026-Feb-25 07:10:46.029953
#45 1.164   - build.outDir
2026-Feb-25 07:10:46.029953
#45 1.194 vite v6.4.1 building SSR bundle for production...
2026-Feb-25 07:10:46.029953
#45 1.217 transforming...
2026-Feb-25 07:10:46.029953
#45 1.652 7:10:43 AM [vite-plugin-svelte] src/components/InterstitialAd.svelte:28:20 This reference only captures the initial value of `countdownSeconds`. Did you mean to reference it inside a closure instead?
2026-Feb-25 07:10:46.029953
#45 1.652 https://svelte.dev/e/state_referenced_locally
2026-Feb-25 07:10:46.029953
#45 1.653 26:
2026-Feb-25 07:10:46.029953
#45 1.653 27:   /** Current countdown value */
2026-Feb-25 07:10:46.029953
#45 1.653 28:   let count = $state(countdownSeconds);
2026-Feb-25 07:10:46.029953
#45 1.653                                          ^
2026-Feb-25 07:10:46.029953
#45 1.653 29:
2026-Feb-25 07:10:46.029953
#45 1.653 30:   /** Whether countdown is complete */
2026-Feb-25 07:10:46.029953
#45 2.357 ✓ 219 modules transformed.
2026-Feb-25 07:10:46.029953
#45 2.444 rendering chunks...
2026-Feb-25 07:10:46.029953
#45 2.899 vite v6.4.1 building for production...
2026-Feb-25 07:10:46.029953
#45 2.901 transforming...
2026-Feb-25 07:10:46.029953
#45 3.161 7:10:44 AM [vite-plugin-svelte] src/components/InterstitialAd.svelte:28:20 This reference only captures the initial value of `countdownSeconds`. Did you mean to reference it inside a closure instead?
2026-Feb-25 07:10:46.029953
#45 3.161 https://svelte.dev/e/state_referenced_locally
2026-Feb-25 07:10:46.029953
#45 3.161 26:
2026-Feb-25 07:10:46.029953
#45 3.161 27:   /** Current countdown value */
2026-Feb-25 07:10:46.029953
#45 3.161 28:   let count = $state(countdownSeconds);
2026-Feb-25 07:10:46.029953
#45 3.161                                          ^
2026-Feb-25 07:10:46.029953
#45 3.161 29:
2026-Feb-25 07:10:46.029953
#45 3.161 30:   /** Whether countdown is complete */
2026-Feb-25 07:10:46.029953
#45 3.655 ✓ 180 modules transformed.
2026-Feb-25 07:10:46.029953
#45 3.705 rendering chunks...
2026-Feb-25 07:10:46.029953
#45 3.726 computing gzip size...
2026-Feb-25 07:10:46.029953
#45 3.732 .svelte-kit/output/client/_app/version.json                          0.03 kB │ gzip:  0.05 kB
2026-Feb-25 07:10:46.029953
#45 3.732 .svelte-kit/output/client/.vite/manifest.json                        4.25 kB │ gzip:  0.71 kB
2026-Feb-25 07:10:46.029953
#45 3.732 .svelte-kit/output/client/_app/immutable/assets/index.BFOS5JwN.css   1.33 kB │ gzip:  0.58 kB
2026-Feb-25 07:10:46.029953
#45 3.732 .svelte-kit/output/client/_app/immutable/assets/3.ClnXt1Pa.css       1.37 kB │ gzip:  0.47 kB
2026-Feb-25 07:10:46.029953
#45 3.732 .svelte-kit/output/client/_app/immutable/assets/0.DQ2purO0.css       5.77 kB │ gzip:  1.51 kB
2026-Feb-25 07:10:46.029953
#45 3.732 .svelte-kit/output/client/_app/immutable/assets/2.DwCm9Uta.css      16.23 kB │ gzip:  3.14 kB
2026-Feb-25 07:10:46.029953
#45 3.732 .svelte-kit/output/client/_app/immutable/entry/start.BniDo-92.js     0.08 kB │ gzip:  0.09 kB
2026-Feb-25 07:10:46.029953
#45 3.732 .svelte-kit/output/client/_app/immutable/chunks/OqcRVUO6.js          0.52 kB │ gzip:  0.33 kB
2026-Feb-25 07:10:46.029953
#45 3.733 .svelte-kit/output/client/_app/immutable/nodes/1.CM2rFgRD.js         0.59 kB │ gzip:  0.36 kB
2026-Feb-25 07:10:46.029953
#45 3.733 .svelte-kit/output/client/_app/immutable/chunks/CsSG9ez8.js          1.13 kB │ gzip:  0.64 kB
2026-Feb-25 07:10:46.029953
#45 3.733 .svelte-kit/output/client/_app/immutable/chunks/DrfBFmL_.js          1.24 kB │ gzip:  0.62 kB
2026-Feb-25 07:10:46.029953
#45 3.733 .svelte-kit/output/client/_app/immutable/chunks/D9RlerWX.js          2.11 kB │ gzip:  1.05 kB
2026-Feb-25 07:10:46.029953
#45 3.733 .svelte-kit/output/client/_app/immutable/chunks/DKFxp2lO.js          4.43 kB │ gzip:  2.06 kB
2026-Feb-25 07:10:46.029953
#45 3.733 .svelte-kit/output/client/_app/immutable/entry/app.8nv43NgL.js       6.43 kB │ gzip:  2.96 kB
2026-Feb-25 07:10:46.029953
#45 3.733 .svelte-kit/output/client/_app/immutable/nodes/0.146buU3j.js         7.40 kB │ gzip:  3.24 kB
2026-Feb-25 07:10:46.029953
#45 3.733 .svelte-kit/output/client/_app/immutable/chunks/BnxzqLTh.js          8.24 kB │ gzip:  3.51 kB
2026-Feb-25 07:10:46.029953
#45 3.733 .svelte-kit/output/client/_app/immutable/nodes/3.YOLfrf5y.js         9.02 kB │ gzip:  2.72 kB
2026-Feb-25 07:10:46.029953
#45 3.733 .svelte-kit/output/client/_app/immutable/chunks/CFeK1F_f.js         11.11 kB │ gzip:  4.99 kB
2026-Feb-25 07:10:46.029953
#45 3.733 .svelte-kit/output/client/_app/immutable/chunks/Corhg2Cp.js         24.57 kB │ gzip:  9.65 kB
2026-Feb-25 07:10:46.029953
#45 3.733 .svelte-kit/output/client/_app/immutable/chunks/DUdA-PQf.js         27.51 kB │ gzip: 10.62 kB
2026-Feb-25 07:10:46.029953
#45 3.733 .svelte-kit/output/client/_app/immutable/nodes/2.zkL_iJL3.js        33.30 kB │ gzip: 12.51 kB
2026-Feb-25 07:10:46.029953
#45 3.733 ✓ built in 834ms
2026-Feb-25 07:10:46.029953
#45 3.908 vite v6.4.1 building for production...
2026-Feb-25 07:10:46.029953
#45 3.911 transforming...
2026-Feb-25 07:10:46.029953
#45 3.914 ✓ 2 modules transformed.
2026-Feb-25 07:10:46.029953
#45 3.916 rendering chunks...
2026-Feb-25 07:10:46.029953
#45 3.916 computing gzip size...
2026-Feb-25 07:10:46.029953
#45 3.917 .svelte-kit/output/client/service-worker.mjs  2.22 kB │ gzip: 0.95 kB
2026-Feb-25 07:10:46.029953
#45 3.917 ✓ built in 9ms
2026-Feb-25 07:10:46.029953
#45 3.917 .svelte-kit/output/server/.vite/manifest.json                             4.00 kB
2026-Feb-25 07:10:46.029953
#45 3.917 .svelte-kit/output/server/_app/immutable/assets/AdBanner.BFOS5JwN.css     1.33 kB
2026-Feb-25 07:10:46.029953
#45 3.917 .svelte-kit/output/server/_app/immutable/assets/_page.ClnXt1Pa.css        1.37 kB
2026-Feb-25 07:10:46.029953
#45 3.917 .svelte-kit/output/server/_app/immutable/assets/_layout.DQ2purO0.css      5.77 kB
2026-Feb-25 07:10:46.029953
#45 3.917 .svelte-kit/output/server/_app/immutable/assets/_page.C3wyMcah.css       12.18 kB
2026-Feb-25 07:10:46.029953
#45 3.917 .svelte-kit/output/server/entries/pages/_page.ts.js                       0.05 kB
2026-Feb-25 07:10:46.029953
#45 3.917 .svelte-kit/output/server/chunks/false.js                                 0.05 kB
2026-Feb-25 07:10:46.029953
#45 3.917 .svelte-kit/output/server/entries/endpoints/share-target/_server.ts.js    0.25 kB
2026-Feb-25 07:10:46.029953
#45 3.917 .svelte-kit/output/server/internal.js                                     0.35 kB
2026-Feb-25 07:10:46.029953
#45 3.917 .svelte-kit/output/server/chunks/environment.js                           0.62 kB
2026-Feb-25 07:10:46.029953
#45 3.917 .svelte-kit/output/server/chunks/utils.js                                 1.15 kB
2026-Feb-25 07:10:46.029953
#45 3.917 .svelte-kit/output/server/entries/fallbacks/error.svelte.js               1.38 kB
2026-Feb-25 07:10:46.029953
#45 3.917 .svelte-kit/output/server/chunks/AdBanner.js                              1.88 kB
2026-Feb-25 07:10:46.029953
#45 3.917 .svelte-kit/output/server/chunks/index.js                                 2.65 kB
2026-Feb-25 07:10:46.029953
#45 3.917 .svelte-kit/output/server/chunks/internal.js                              3.26 kB
2026-Feb-25 07:10:46.029953
#45 3.917 .svelte-kit/output/server/chunks/exports.js                               5.54 kB
2026-Feb-25 07:10:46.029953
#45 3.917 .svelte-kit/output/server/entries/pages/_layout.svelte.js                 5.95 kB
2026-Feb-25 07:10:46.029953
#45 3.917 .svelte-kit/output/server/entries/pages/privacy/_page.svelte.js           9.01 kB
2026-Feb-25 07:10:46.029953
#45 3.917 .svelte-kit/output/server/remote-entry.js                                19.00 kB
2026-Feb-25 07:10:46.029953
#45 3.917 .svelte-kit/output/server/entries/pages/_page.svelte.js                  20.21 kB
2026-Feb-25 07:10:46.029953
#45 3.917 .svelte-kit/output/server/chunks/shared.js                               26.42 kB
2026-Feb-25 07:10:46.029953
#45 3.917 .svelte-kit/output/server/chunks/root.js                                 27.99 kB
2026-Feb-25 07:10:46.029953
#45 3.917 .svelte-kit/output/server/chunks/index2.js                               88.65 kB
2026-Feb-25 07:10:46.029953
#45 3.917 .svelte-kit/output/server/index.js                                      123.11 kB
2026-Feb-25 07:10:46.029953
#45 3.917 ✓ built in 2.72s
2026-Feb-25 07:10:46.029953
#45 3.917
2026-Feb-25 07:10:46.029953
#45 3.917 Run npm run preview to preview your production build locally.
2026-Feb-25 07:10:46.029953
#45 3.919
2026-Feb-25 07:10:46.029953
#45 3.919 > Using @sveltejs/adapter-node
2026-Feb-25 07:10:46.029953
#45 4.400   ✔ done
2026-Feb-25 07:10:46.029953
#45 DONE 4.5s
2026-Feb-25 07:10:46.182045
#46 [frontend runtime 3/4] COPY --from=builder /app/build ./build
2026-Feb-25 07:10:46.182045
#46 DONE 0.0s
2026-Feb-25 07:10:46.182045
2026-Feb-25 07:10:46.182045
#24 [gpu-worker runtime 3/6] RUN apt-get update && apt-get install -y     ca-certificates     libssl3     ffmpeg     libavcodec58     libavformat58     libavutil56     libswscale5     libavfilter7     libavdevice58     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:10:46.182045
#24 10.72 Reading package lists...
2026-Feb-25 07:10:46.182045
#24 11.72 Building dependency tree...
2026-Feb-25 07:10:46.182045
#24 11.99 Reading state information...
2026-Feb-25 07:10:46.182045
#24 12.23 The following additional packages will be installed:
2026-Feb-25 07:10:46.182045
#24 12.23   alsa-topology-conf alsa-ucm-conf dbus fontconfig fontconfig-config
2026-Feb-25 07:10:46.182045
#24 12.23   fonts-dejavu-core gcc-12-base i965-va-driver intel-media-va-driver libaacs0
2026-Feb-25 07:10:46.182045
#24 12.23   libaom3 libapparmor1 libasound2 libasound2-data libass9 libasyncns0
2026-Feb-25 07:10:46.182045
#24 12.23   libavc1394-0 libbdplus0 libblas3 libbluray2 libbrotli1 libbs2b0 libbsd0
2026-Feb-25 07:10:46.182045
#24 12.23   libcaca0 libcairo-gobject2 libcairo2 libcdio-cdda2 libcdio-paranoia2
2026-Feb-25 07:10:46.182045
#24 12.23   libcdio19 libchromaprint1 libcodec2-1.0 libdatrie1 libdav1d5 libdbus-1-3
2026-Feb-25 07:10:46.182045
#24 12.23   libdc1394-25 libdecor-0-0 libdecor-0-plugin-1-cairo libdeflate0
2026-Feb-25 07:10:46.182045
#24 12.23   libdrm-amdgpu1 libdrm-common libdrm-intel1 libdrm-nouveau2 libdrm-radeon1
2026-Feb-25 07:10:46.182045
#24 12.23   libdrm2 libedit2 libelf1 libexpat1 libflac8 libflite1 libfontconfig1
2026-Feb-25 07:10:46.182045
#24 12.23   libfreetype6 libfribidi0 libgbm1 libgcc-s1 libgdk-pixbuf-2.0-0
2026-Feb-25 07:10:46.182045
#24 12.23   libgdk-pixbuf2.0-bin libgdk-pixbuf2.0-common libgfortran5 libgl1
2026-Feb-25 07:10:46.182045
#24 12.23   libgl1-amber-dri libgl1-mesa-dri libglapi-mesa libglib2.0-0 libglib2.0-data
2026-Feb-25 07:10:46.182045
#24 12.23   libglvnd0 libglx-mesa0 libglx0 libgme0 libgomp1 libgraphite2-3 libgsm1
2026-Feb-25 07:10:46.182045
#24 12.23   libharfbuzz0b libicu70 libiec61883-0 libigdgmm12 libjack-jackd2-0 libjbig0
2026-Feb-25 07:10:46.182045
#24 12.23   libjpeg-turbo8 libjpeg8 liblapack3 liblilv-0-0 libllvm15 libmd0 libmfx1
2026-Feb-25 07:10:46.182045
#24 12.23   libmp3lame0 libmpg123-0 libmysofa1 libnorm1 libnuma1 libogg0 libopenal-data
2026-Feb-25 07:10:46.182045
#24 12.23   libopenal1 libopenjp2-7 libopenmpt0 libopus0 libpango-1.0-0
2026-Feb-25 07:10:46.182045
#24 12.23   libpangocairo-1.0-0 libpangoft2-1.0-0 libpciaccess0 libpgm-5.3-0
2026-Feb-25 07:10:46.182045
#24 12.23   libpixman-1-0 libpng16-16 libpocketsphinx3 libpostproc55 libpulse0
2026-Feb-25 07:10:46.182045
#24 12.23   libquadmath0 librabbitmq4 libraw1394-11 librsvg2-2 librsvg2-common
2026-Feb-25 07:10:46.182045
#24 12.23   librubberband2 libsamplerate0 libsdl2-2.0-0 libsensors-config libsensors5
2026-Feb-25 07:10:46.182045
#24 12.23   libserd-0-0 libshine3 libslang2 libsnappy1v5 libsndfile1 libsndio7.0
2026-Feb-25 07:10:46.182045
#24 12.23   libsodium23 libsord-0-0 libsoxr0 libspeex1 libsphinxbase3 libsratom-0-0
2026-Feb-25 07:10:46.182045
#24 12.23   libsrt1.4-gnutls libssh-gcrypt-4 libstdc++6 libswresample3 libthai-data
2026-Feb-25 07:10:46.182045
#24 12.23   libthai0 libtheora0 libtiff5 libtwolame0 libudfread0 libusb-1.0-0 libva-drm2
2026-Feb-25 07:10:46.182045
#24 12.23   libva-x11-2 libva2 libvdpau1 libvidstab1.1 libvorbis0a libvorbisenc2
2026-Feb-25 07:10:46.182045
#24 12.23   libvorbisfile3 libvpx7 libwayland-client0 libwayland-cursor0 libwayland-egl1
2026-Feb-25 07:10:46.182045
#24 12.23   libwayland-server0 libwebp7 libwebpmux3 libx11-6 libx11-data libx11-xcb1
2026-Feb-25 07:10:46.182045
#24 12.23   libx264-163 libx265-199 libxau6 libxcb-dri2-0 libxcb-dri3-0 libxcb-glx0
2026-Feb-25 07:10:46.182045
#24 12.23   libxcb-present0 libxcb-randr0 libxcb-render0 libxcb-shape0 libxcb-shm0
2026-Feb-25 07:10:46.182045
#24 12.23   libxcb-sync1 libxcb-xfixes0 libxcb1 libxcursor1 libxdmcp6 libxext6
2026-Feb-25 07:10:46.182045
#24 12.23   libxfixes3 libxi6 libxinerama1 libxkbcommon0 libxml2 libxrandr2 libxrender1
2026-Feb-25 07:10:46.182045
#24 12.23   libxshmfence1 libxss1 libxv1 libxvidcore4 libxxf86vm1 libzimg2 libzmq5
2026-Feb-25 07:10:46.182045
#24 12.23   libzvbi-common libzvbi0 mesa-va-drivers mesa-vdpau-drivers
2026-Feb-25 07:10:46.182045
#24 12.23   ocl-icd-libopencl1 pocketsphinx-en-us shared-mime-info ucf va-driver-all
2026-Feb-25 07:10:46.182045
#24 12.23   vdpau-driver-all x11-common xdg-user-dirs xkb-data
2026-Feb-25 07:10:46.182045
#24 12.23 Suggested packages:
2026-Feb-25 07:10:46.182045
#24 12.23   default-dbus-session-bus | dbus-session-bus ffmpeg-doc
2026-Feb-25 07:10:46.182045
#24 12.23   i965-va-driver-shaders libasound2-plugins alsa-utils libcuda1 libnvcuvid1
2026-Feb-25 07:10:46.182045
#24 12.23   libnvidia-encode1 libbluray-bdj jackd2 libportaudio2 opus-tools pciutils
2026-Feb-25 07:10:46.182045
#24 12.23   pulseaudio libraw1394-doc librsvg2-bin xdg-utils lm-sensors serdi sndiod
2026-Feb-25 07:10:46.182045
#24 12.23   sordi speex opencl-icd libvdpau-va-gl1
2026-Feb-25 07:10:46.182045
#24 12.60 The following NEW packages will be installed:
2026-Feb-25 07:10:46.182045
#24 12.60   alsa-topology-conf alsa-ucm-conf dbus ffmpeg fontconfig fontconfig-config
2026-Feb-25 07:10:46.182045
#24 12.60   fonts-dejavu-core i965-va-driver intel-media-va-driver libaacs0 libaom3
2026-Feb-25 07:10:46.182045
#24 12.60   libapparmor1 libasound2 libasound2-data libass9 libasyncns0 libavc1394-0
2026-Feb-25 07:10:46.182045
#24 12.60   libavcodec58 libavdevice58 libavfilter7 libavformat58 libavutil56 libbdplus0
2026-Feb-25 07:10:46.182045
#24 12.60   libblas3 libbluray2 libbrotli1 libbs2b0 libbsd0 libcaca0 libcairo-gobject2
2026-Feb-25 07:10:46.182045
#24 12.60   libcairo2 libcdio-cdda2 libcdio-paranoia2 libcdio19 libchromaprint1
2026-Feb-25 07:10:46.182045
#24 12.60   libcodec2-1.0 libdatrie1 libdav1d5 libdbus-1-3 libdc1394-25 libdecor-0-0
2026-Feb-25 07:10:46.182045
#24 12.60   libdecor-0-plugin-1-cairo libdeflate0 libdrm-amdgpu1 libdrm-common
2026-Feb-25 07:10:46.182045
#24 12.60   libdrm-intel1 libdrm-nouveau2 libdrm-radeon1 libdrm2 libedit2 libelf1
2026-Feb-25 07:10:46.182045
#24 12.60   libexpat1 libflac8 libflite1 libfontconfig1 libfreetype6 libfribidi0 libgbm1
2026-Feb-25 07:10:46.182045
#24 12.60   libgdk-pixbuf-2.0-0 libgdk-pixbuf2.0-bin libgdk-pixbuf2.0-common
2026-Feb-25 07:10:46.182045
#24 12.60   libgfortran5 libgl1 libgl1-amber-dri libgl1-mesa-dri libglapi-mesa
2026-Feb-25 07:10:46.182045
#24 12.60   libglib2.0-0 libglib2.0-data libglvnd0 libglx-mesa0 libglx0 libgme0 libgomp1
2026-Feb-25 07:10:46.182045
#24 12.60   libgraphite2-3 libgsm1 libharfbuzz0b libicu70 libiec61883-0 libigdgmm12
2026-Feb-25 07:10:46.182045
#24 12.60   libjack-jackd2-0 libjbig0 libjpeg-turbo8 libjpeg8 liblapack3 liblilv-0-0
2026-Feb-25 07:10:46.182045
#24 12.60   libllvm15 libmd0 libmfx1 libmp3lame0 libmpg123-0 libmysofa1 libnorm1
2026-Feb-25 07:10:46.182045
#24 12.60   libnuma1 libogg0 libopenal-data libopenal1 libopenjp2-7 libopenmpt0 libopus0
2026-Feb-25 07:10:46.182045
#24 12.60   libpango-1.0-0 libpangocairo-1.0-0 libpangoft2-1.0-0 libpciaccess0
2026-Feb-25 07:10:46.182045
#24 12.60   libpgm-5.3-0 libpixman-1-0 libpng16-16 libpocketsphinx3 libpostproc55
2026-Feb-25 07:10:46.182045
#24 12.60   libpulse0 libquadmath0 librabbitmq4 libraw1394-11 librsvg2-2 librsvg2-common
2026-Feb-25 07:10:46.182045
#24 12.60   librubberband2 libsamplerate0 libsdl2-2.0-0 libsensors-config libsensors5
2026-Feb-25 07:10:46.182045
#24 12.60   libserd-0-0 libshine3 libslang2 libsnappy1v5 libsndfile1 libsndio7.0
2026-Feb-25 07:10:46.182045
#24 12.60   libsodium23 libsord-0-0 libsoxr0 libspeex1 libsphinxbase3 libsratom-0-0
2026-Feb-25 07:10:46.182045
#24 12.60   libsrt1.4-gnutls libssh-gcrypt-4 libswresample3 libswscale5 libthai-data
2026-Feb-25 07:10:46.182045
#24 12.60   libthai0 libtheora0 libtiff5 libtwolame0 libudfread0 libusb-1.0-0 libva-drm2
2026-Feb-25 07:10:46.182045
#24 12.60   libva-x11-2 libva2 libvdpau1 libvidstab1.1 libvorbis0a libvorbisenc2
2026-Feb-25 07:10:46.182045
#24 12.60   libvorbisfile3 libvpx7 libwayland-client0 libwayland-cursor0 libwayland-egl1
2026-Feb-25 07:10:46.182045
#24 12.60   libwayland-server0 libwebp7 libwebpmux3 libx11-6 libx11-data libx11-xcb1
2026-Feb-25 07:10:46.182045
#24 12.60   libx264-163 libx265-199 libxau6 libxcb-dri2-0 libxcb-dri3-0 libxcb-glx0
2026-Feb-25 07:10:46.182045
#24 12.60   libxcb-present0 libxcb-randr0 libxcb-render0 libxcb-shape0 libxcb-shm0
2026-Feb-25 07:10:46.182045
#24 12.60   libxcb-sync1 libxcb-xfixes0 libxcb1 libxcursor1 libxdmcp6 libxext6
2026-Feb-25 07:10:46.182045
#24 12.60   libxfixes3 libxi6 libxinerama1 libxkbcommon0 libxml2 libxrandr2 libxrender1
2026-Feb-25 07:10:46.182045
#24 12.60   libxshmfence1 libxss1 libxv1 libxvidcore4 libxxf86vm1 libzimg2 libzmq5
2026-Feb-25 07:10:46.182045
#24 12.60   libzvbi-common libzvbi0 mesa-va-drivers mesa-vdpau-drivers
2026-Feb-25 07:10:46.182045
#24 12.60   ocl-icd-libopencl1 pocketsphinx-en-us shared-mime-info ucf va-driver-all
2026-Feb-25 07:10:46.182045
#24 12.60   vdpau-driver-all x11-common xdg-user-dirs xkb-data
2026-Feb-25 07:10:46.182045
#24 12.60 The following packages will be upgraded:
2026-Feb-25 07:10:46.182045
#24 12.60   ca-certificates gcc-12-base libgcc-s1 libssl3 libstdc++6
2026-Feb-25 07:10:46.182045
#24 13.23 5 upgraded, 204 newly installed, 0 to remove and 56 not upgraded.
2026-Feb-25 07:10:46.182045
#24 13.23 Need to get 161 MB of archives.
2026-Feb-25 07:10:46.182045
#24 13.23 After this operation, 495 MB of additional disk space will be used.
2026-Feb-25 07:10:46.182045
#24 13.23 Get:1 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 gcc-12-base amd64 12.3.0-1ubuntu1~22.04.3 [216 kB]
2026-Feb-25 07:10:46.182045
#24 14.84 Get:2 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libstdc++6 amd64 12.3.0-1ubuntu1~22.04.3 [699 kB]
2026-Feb-25 07:10:46.285895
#24 ...
2026-Feb-25 07:10:46.285895
2026-Feb-25 07:10:46.285895
#47 [frontend runtime 4/4] COPY --from=builder /app/package.json ./
2026-Feb-25 07:10:46.285895
#47 DONE 0.1s
2026-Feb-25 07:10:46.402439
#48 [frontend] exporting to image
2026-Feb-25 07:10:46.402439
#48 exporting layers 0.1s done
2026-Feb-25 07:10:46.402439
#48 exporting manifest sha256:af59e93e2d76249fe8e48f3b78a4c2996a0c8ec14950095b41355eacd04169f9 done
2026-Feb-25 07:10:46.402439
#48 exporting config sha256:9b744fd2ea03750445160c48f7ef79e7360b74c71b03aa041d864d27bc695e5b done
2026-Feb-25 07:10:46.402439
#48 exporting attestation manifest sha256:ea61d86cbee5e90f459525090e470a805b7a7c650012eedf21d8826b1247231f done
2026-Feb-25 07:10:46.402439
#48 exporting manifest list sha256:a61b8ced14a87c74b1def0b33020450a018bd8f78fb0c63ec8ebab4372d6df23 done
2026-Feb-25 07:10:46.402439
#48 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:6284ccef8da2b1bbb3aa00193368912f946f4bb3 done
2026-Feb-25 07:10:46.402439
#48 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:6284ccef8da2b1bbb3aa00193368912f946f4bb3 0.0s done
2026-Feb-25 07:10:46.402439
#48 DONE 0.2s
2026-Feb-25 07:10:46.402439
2026-Feb-25 07:10:46.402439
#49 [frontend] resolving provenance for metadata file
2026-Feb-25 07:10:46.558029
#49 DONE 0.0s
2026-Feb-25 07:10:46.558029
2026-Feb-25 07:10:46.558029
#37 [gpu-worker builder 3/9] RUN apt-get update && apt-get install -y     curl     build-essential     pkg-config     libssl-dev     protobuf-compiler     ffmpeg     libavcodec-dev     libavformat-dev     libavutil-dev     libswscale-dev     libavfilter-dev     libavdevice-dev     clang     libclang-dev     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:10:46.558029
#37 14.09 Get:2 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libc-dev-bin amd64 2.35-0ubuntu3.13 [20.3 kB]
2026-Feb-25 07:10:46.558029
#37 14.70 Get:3 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libatomic1 amd64 12.3.0-1ubuntu1~22.04.3 [10.5 kB]
2026-Feb-25 07:10:46.558029
#37 15.05 Get:4 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libubsan1 amd64 12.3.0-1ubuntu1~22.04.3 [976 kB]
2026-Feb-25 07:10:48.598364
#37 17.27 Get:5 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libquadmath0 amd64 12.3.0-1ubuntu1~22.04.3 [154 kB]
2026-Feb-25 07:10:48.932031
#37 17.60 Get:6 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 liblsan0 amd64 12.3.0-1ubuntu1~22.04.3 [1069 kB]
2026-Feb-25 07:10:49.641067
#37 18.31 Get:7 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libitm1 amd64 12.3.0-1ubuntu1~22.04.3 [30.2 kB]
2026-Feb-25 07:10:49.932176
#37 18.60 Get:8 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgomp1 amd64 12.3.0-1ubuntu1~22.04.3 [127 kB]
2026-Feb-25 07:10:50.388389
#37 18.91 Get:9 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 gcc-12-base amd64 12.3.0-1ubuntu1~22.04.3 [216 kB]
2026-Feb-25 07:10:50.697601
#37 19.23 Get:10 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgcc-s1 amd64 12.3.0-1ubuntu1~22.04.3 [53.9 kB]
2026-Feb-25 07:10:50.914227
#37 19.52 Get:11 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libcc1-0 amd64 12.3.0-1ubuntu1~22.04.3 [48.3 kB]
2026-Feb-25 07:10:50.914227
#37 ...
2026-Feb-25 07:10:50.914227
2026-Feb-25 07:10:50.914227
#44 [api builder 9/9] RUN cargo build --release --bin vps-gateway
2026-Feb-25 07:10:50.914227
#44 7.387  Downloading crates ...
2026-Feb-25 07:10:50.914227
#44 7.965   Downloaded adler v1.0.2
2026-Feb-25 07:10:50.914227
#44 8.000   Downloaded adler2 v2.0.1
2026-Feb-25 07:10:50.914227
#44 8.327   Downloaded block-buffer v0.10.4
2026-Feb-25 07:10:50.914227
#44 8.330   Downloaded radium v0.7.0
2026-Feb-25 07:10:50.914227
#44 8.481   Downloaded quote v1.0.44
2026-Feb-25 07:10:50.914227
#44 8.494   Downloaded alloc-stdlib v0.2.2
2026-Feb-25 07:10:50.914227
#44 8.511   Downloaded async-stream v0.3.6
2026-Feb-25 07:10:50.914227
#44 8.527   Downloaded cfg-if v1.0.4
2026-Feb-25 07:10:50.914227
#44 8.563   Downloaded gzip-header v1.0.0
2026-Feb-25 07:10:50.914227
#44 8.565   Downloaded async-stream-impl v0.3.6
2026-Feb-25 07:10:50.914227
#44 8.597   Downloaded dirs-sys v0.4.1
2026-Feb-25 07:10:50.914227
#44 8.674   Downloaded const-random v0.1.18
2026-Feb-25 07:10:50.914227
#44 8.682   Downloaded errno v0.3.14
2026-Feb-25 07:10:50.914227
#44 8.691   Downloaded equivalent v1.0.2
2026-Feb-25 07:10:51.031665
#44 8.698   Downloaded futures-sink v0.3.32
2026-Feb-25 07:10:51.031665
#44 8.744   Downloaded cooked-waker v5.0.0
2026-Feb-25 07:10:51.031665
#44 8.809   Downloaded semver-parser v0.7.0
2026-Feb-25 07:10:51.135465
#44 8.830   Downloaded crunchy v0.2.4
2026-Feb-25 07:10:51.135465
#44 8.833   Downloaded rustc-hash v1.1.0
2026-Feb-25 07:10:51.135465
#44 8.847   Downloaded futures-core v0.3.32
2026-Feb-25 07:10:51.135465
#44 8.850   Downloaded rustc-hash v2.1.1
2026-Feb-25 07:10:51.135465
#44 8.853   Downloaded fastrand v2.3.0
2026-Feb-25 07:10:51.135465
#44 8.856   Downloaded form_urlencoded v1.2.2
2026-Feb-25 07:10:51.135465
#44 8.868   Downloaded fslock v0.2.1
2026-Feb-25 07:10:51.135465
#44 8.872   Downloaded same-file v1.0.6
2026-Feb-25 07:10:51.135465
#44 8.883   Downloaded futures-task v0.3.32
2026-Feb-25 07:10:51.135465
#44 8.887   Downloaded fnv v1.0.7
2026-Feb-25 07:10:51.135465
#44 8.897   Downloaded cpufeatures v0.2.17
2026-Feb-25 07:10:51.135465
#44 8.900   Downloaded foreign-types-shared v0.1.1
2026-Feb-25 07:10:51.135465
#44 8.913   Downloaded compression-core v0.4.31
2026-Feb-25 07:10:51.252028
#44 8.925   Downloaded dirs v5.0.1
2026-Feb-25 07:10:51.252028
#44 8.928   Downloaded cfg_aliases v0.2.1
2026-Feb-25 07:10:51.252028
#44 9.029   Downloaded serde_urlencoded v0.7.1
2026-Feb-25 07:10:51.375517
#44 9.032   Downloaded atomic-waker v1.1.2
2026-Feb-25 07:10:51.375517
#44 9.053   Downloaded crypto-common v0.1.7
2026-Feb-25 07:10:51.375517
#44 9.064   Downloaded document-features v0.2.12
2026-Feb-25 07:10:51.375517
#44 9.073   Downloaded futures-io v0.3.32
2026-Feb-25 07:10:51.375517
#44 9.084   Downloaded bit-set v0.5.3
2026-Feb-25 07:10:51.375517
#44 9.129   Downloaded tower-layer v0.3.3
2026-Feb-25 07:10:51.375517
#44 9.153   Downloaded rustc_version v0.2.3
2026-Feb-25 07:10:51.481236
#44 9.160   Downloaded simd-abstraction v0.7.1
2026-Feb-25 07:10:51.481236
#44 9.171   Downloaded stable_deref_trait v1.2.1
2026-Feb-25 07:10:51.481236
#44 9.174   Downloaded serde_spanned v0.6.9
2026-Feb-25 07:10:51.481236
#44 9.186   Downloaded base64-simd v0.7.0
2026-Feb-25 07:10:51.481236
#44 9.196   Downloaded generic-array v0.14.7
2026-Feb-25 07:10:51.481236
#44 9.200   Downloaded ahash v0.8.12
2026-Feb-25 07:10:51.481236
#44 9.205   Downloaded funty v2.0.0
2026-Feb-25 07:10:51.481236
#44 9.206   Downloaded const-random-macro v0.1.16
2026-Feb-25 07:10:51.481236
#44 9.213   Downloaded futures-macro v0.3.32
2026-Feb-25 07:10:51.481236
#44 9.215   Downloaded foreign-types v0.3.2
2026-Feb-25 07:10:51.481236
#44 9.217   Downloaded debugid v0.8.0
2026-Feb-25 07:10:51.481236
#44 9.227   Downloaded tap v1.0.1
2026-Feb-25 07:10:51.481236
#44 9.235   Downloaded option-ext v0.2.0
2026-Feb-25 07:10:51.481236
#44 9.237   Downloaded pin-utils v0.1.0
2026-Feb-25 07:10:51.481236
#44 9.240   Downloaded proc-macro-rules-macros v0.4.0
2026-Feb-25 07:10:51.481236
#44 9.242   Downloaded proc-macro-rules v0.4.0
2026-Feb-25 07:10:51.481236
#44 9.245   Downloaded alloc-no-stdlib v2.0.4
2026-Feb-25 07:10:51.481236
#44 9.258   Downloaded cexpr v0.6.0
2026-Feb-25 07:10:51.590172
#44 9.262   Downloaded filetime v0.2.27
2026-Feb-25 07:10:51.590172
#44 9.265   Downloaded compression-codecs v0.4.37
2026-Feb-25 07:10:51.590172
#44 9.302   Downloaded sync_wrapper v1.0.2
2026-Feb-25 07:10:51.590172
#44 9.341   Downloaded time-core v0.1.8
2026-Feb-25 07:10:51.590172
#44 9.343   Downloaded strum v0.25.0
2026-Feb-25 07:10:51.590172
#44 9.352   Downloaded try-lock v0.2.5
2026-Feb-25 07:10:51.590172
#44 9.357   Downloaded tinyvec_macros v0.1.1
2026-Feb-25 07:10:51.703411
#44 9.380   Downloaded digest v0.10.7
2026-Feb-25 07:10:51.703411
#44 9.383   Downloaded deranged v0.5.8
2026-Feb-25 07:10:51.703411
#44 9.385   Downloaded glob v0.3.3
2026-Feb-25 07:10:51.703411
#44 9.388   Downloaded axum-core v0.4.5
2026-Feb-25 07:10:51.703411
#44 9.391   Downloaded either v1.15.0
2026-Feb-25 07:10:51.703411
#44 9.393   Downloaded dlv-list v0.5.2
2026-Feb-25 07:10:51.703411
#44 9.397   Downloaded dashmap v6.1.0
2026-Feb-25 07:10:51.703411
#44 9.401   Downloaded data-encoding v2.10.0
2026-Feb-25 07:10:51.703411
#44 9.402   Downloaded convert_case v0.6.0
2026-Feb-25 07:10:51.703411
#44 9.405   Downloaded bit-vec v0.6.3
2026-Feb-25 07:10:51.703411
#44 9.408   Downloaded want v0.3.1
2026-Feb-25 07:10:51.703411
#44 9.410   Downloaded deno_unsync v0.4.4
2026-Feb-25 07:10:51.703411
#44 9.414   Downloaded autocfg v1.5.0
2026-Feb-25 07:10:51.703411
#44 9.417   Downloaded axum-core v0.5.6
2026-Feb-25 07:10:51.703411
#44 9.421   Downloaded arraydeque v0.5.1
2026-Feb-25 07:10:51.703411
#44 9.423   Downloaded find-msvc-tools v0.1.9
2026-Feb-25 07:10:51.703411
#44 9.427   Downloaded utf8_iter v1.0.4
2026-Feb-25 07:10:51.703411
#44 9.429   Downloaded fixedbitset v0.5.7
2026-Feb-25 07:10:51.703411
#44 9.464   Downloaded ppv-lite86 v0.2.21
2026-Feb-25 07:10:51.703411
#44 9.481   Downloaded rand_chacha v0.3.1
2026-Feb-25 07:10:51.805654
#44 9.483   Downloaded rand_core v0.9.5
2026-Feb-25 07:10:51.805654
#44 9.490   Downloaded subtle v2.6.1
2026-Feb-25 07:10:51.805654
#44 9.492   Downloaded toml_datetime v0.6.11
2026-Feb-25 07:10:51.805654
#44 9.508   Downloaded tower-service v0.3.3
2026-Feb-25 07:10:51.805654
#44 9.511   Downloaded http-body v1.0.1
2026-Feb-25 07:10:51.805654
#44 9.513   Downloaded zerofrom-derive v0.1.6
2026-Feb-25 07:10:51.805654
#44 9.515   Downloaded if_chain v1.0.3
2026-Feb-25 07:10:51.805654
#44 9.517   Downloaded rand_core v0.6.4
2026-Feb-25 07:10:51.805654
#44 9.519   Downloaded rust-ini v0.20.0
2026-Feb-25 07:10:51.805654
#44 9.522   Downloaded semver v0.9.0
2026-Feb-25 07:10:51.805654
#44 9.524   Downloaded rand_chacha v0.9.0
2026-Feb-25 07:10:51.805654
#44 9.526   Downloaded scopeguard v1.2.0
2026-Feb-25 07:10:51.805654
#44 9.528   Downloaded rustversion v1.0.22
2026-Feb-25 07:10:51.805654
#44 9.550   Downloaded anyhow v1.0.102
2026-Feb-25 07:10:51.805654
#44 9.556   Downloaded cookie_store v0.22.1
2026-Feb-25 07:10:51.805654
#44 9.559   Downloaded clang-sys v1.8.1
2026-Feb-25 07:10:51.805654
#44 9.563   Downloaded async-trait v0.1.89
2026-Feb-25 07:10:51.805654
#44 9.569   Downloaded getrandom v0.2.17
2026-Feb-25 07:10:51.805654
#44 9.575   Downloaded futures-executor v0.3.32
2026-Feb-25 07:10:51.805654
#44 9.577   Downloaded bincode v1.3.3
2026-Feb-25 07:10:51.805654
#44 9.581   Downloaded lazycell v1.3.0
2026-Feb-25 07:10:51.805654
#44 9.583   Downloaded inotify-sys v0.1.5
2026-Feb-25 07:10:51.907504
#44 9.585   Downloaded crossbeam-utils v0.8.21
2026-Feb-25 07:10:51.907504
#44 9.590   Downloaded prost-build v0.13.5
2026-Feb-25 07:10:51.907504
#44 9.595   Downloaded yoke v0.8.1
2026-Feb-25 07:10:51.907504
#44 9.598   Downloaded multimap v0.10.1
2026-Feb-25 07:10:51.907504
#44 9.601   Downloaded zerovec-derive v0.11.2
2026-Feb-25 07:10:51.907504
#44 9.603   Downloaded getrandom v0.4.1
2026-Feb-25 07:10:51.907504
#44 9.609   Downloaded getrandom v0.3.4
2026-Feb-25 07:10:51.907504
#44 9.615   Downloaded futures v0.3.32
2026-Feb-25 07:10:51.907504
#44 9.623   Downloaded dotenvy v0.15.7
2026-Feb-25 07:10:51.907504
#44 9.628   Downloaded futures-channel v0.3.32
2026-Feb-25 07:10:51.907504
#44 9.631   Downloaded bitflags v2.11.0
2026-Feb-25 07:10:51.907504
#44 9.638   Downloaded displaydoc v0.2.5
2026-Feb-25 07:10:51.907504
#44 9.643   Downloaded cookie v0.18.1
2026-Feb-25 07:10:51.907504
#44 9.648   Downloaded thread_local v1.1.9
2026-Feb-25 07:10:51.907504
#44 9.651   Downloaded serde_path_to_error v0.1.20
2026-Feb-25 07:10:51.907504
#44 9.654   Downloaded bitflags v1.3.2
2026-Feb-25 07:10:51.907504
#44 9.659   Downloaded tokio-macros v2.6.0
2026-Feb-25 07:10:51.907504
#44 9.661   Downloaded signal-hook-registry v1.4.8
2026-Feb-25 07:10:51.907504
#44 9.664   Downloaded tonic-build v0.12.3
2026-Feb-25 07:10:51.907504
#44 9.667   Downloaded time-macros v0.2.27
2026-Feb-25 07:10:51.907504
#44 9.670   Downloaded slab v0.4.12
2026-Feb-25 07:10:51.907504
#44 9.673   Downloaded sha2 v0.10.9
2026-Feb-25 07:10:51.907504
#44 9.677   Downloaded toml_write v0.1.2
2026-Feb-25 07:10:51.907504
#44 9.680   Downloaded thiserror-impl v1.0.69
2026-Feb-25 07:10:51.907504
#44 9.682   Downloaded tracing-log v0.2.0
2026-Feb-25 07:10:51.907504
#44 9.685   Downloaded thiserror-impl v2.0.18
2026-Feb-25 07:10:52.010698
#44 9.687   Downloaded strum_macros v0.25.3
2026-Feb-25 07:10:52.010698
#44 9.691   Downloaded simd-adler32 v0.3.8
2026-Feb-25 07:10:52.010698
#44 9.695   Downloaded tokio-native-tls v0.3.1
2026-Feb-25 07:10:52.010698
#44 9.699   Downloaded static_assertions v1.1.0
2026-Feb-25 07:10:52.010698
#44 9.702   Downloaded tinystr v0.8.2
2026-Feb-25 07:10:52.010698
#44 9.726   Downloaded pathdiff v0.2.3
2026-Feb-25 07:10:52.010698
#44 9.734   Downloaded allocator-api2 v0.2.21
2026-Feb-25 07:10:52.010698
#44 9.739   Downloaded crc32fast v1.5.0
2026-Feb-25 07:10:52.010698
#44 9.748   Downloaded hyper-timeout v0.5.2
2026-Feb-25 07:10:52.010698
#44 9.752   Downloaded base64 v0.22.1
2026-Feb-25 07:10:52.010698
#44 9.759   Downloaded deno_ops v0.176.0
2026-Feb-25 07:10:52.010698
#44 9.776   Downloaded ordered-multimap v0.7.3
2026-Feb-25 07:10:52.010698
#44 9.780   Downloaded zerofrom v0.1.6
2026-Feb-25 07:10:52.010698
#44 9.781   Downloaded base64 v0.21.7
2026-Feb-25 07:10:52.010698
#44 9.788   Downloaded num-integer v0.1.46
2026-Feb-25 07:10:52.112768
#44 9.790   Downloaded flate2 v1.1.9
2026-Feb-25 07:10:52.112768
#44 9.798   Downloaded httpdate v1.0.3
2026-Feb-25 07:10:52.112768
#44 9.800   Downloaded synstructure v0.13.2
2026-Feb-25 07:10:52.112768
#44 9.802   Downloaded writeable v0.6.2
2026-Feb-25 07:10:52.112768
#44 9.806   Downloaded hashlink v0.8.4
2026-Feb-25 07:10:52.112768
#44 9.809   Downloaded walkdir v2.5.0
2026-Feb-25 07:10:52.112768
#44 9.812   Downloaded tokio-stream v0.1.18
2026-Feb-25 07:10:52.112768
#44 9.820   Downloaded rustls-pki-types v1.14.0
2026-Feb-25 07:10:52.112768
#44 9.824   Downloaded toml v0.8.23
2026-Feb-25 07:10:52.112768
#44 9.827   Downloaded tempfile v3.25.0
2026-Feb-25 07:10:52.112768
#44 9.831   Downloaded unicode-id-start v1.4.0
2026-Feb-25 07:10:52.112768
#44 9.836   Downloaded ucd-trie v0.1.7
2026-Feb-25 07:10:52.112768
#44 9.838   Downloaded smallvec v1.15.1
2026-Feb-25 07:10:52.112768
#44 9.841   Downloaded tokio-rustls v0.26.4
2026-Feb-25 07:10:52.112768
#44 9.844   Downloaded tiny-keccak v2.0.2
2026-Feb-25 07:10:52.112768
#44 9.848   Downloaded thiserror v2.0.18
2026-Feb-25 07:10:52.112768
#44 9.857   Downloaded openssl-probe v0.2.1
2026-Feb-25 07:10:52.112768
#44 9.859   Downloaded pkg-config v0.3.32
2026-Feb-25 07:10:52.112768
#44 9.862   Downloaded ryu v1.0.23
2026-Feb-25 07:10:52.112768
#44 9.866   Downloaded sharded-slab v0.1.7
2026-Feb-25 07:10:52.112768
#44 9.871   Downloaded unicode-ident v1.0.24
2026-Feb-25 07:10:52.112768
#44 9.875   Downloaded tracing-attributes v0.1.31
2026-Feb-25 07:10:52.112768
#44 9.879   Downloaded socket2 v0.5.10
2026-Feb-25 07:10:52.112768
#44 9.881   Downloaded ron v0.8.1
2026-Feb-25 07:10:52.112768
#44 9.890   Downloaded thiserror v1.0.69
2026-Feb-25 07:10:52.214213
#44 9.899   Downloaded sourcemap v8.0.1
2026-Feb-25 07:10:52.214213
#44 9.903   Downloaded shlex v1.3.0
2026-Feb-25 07:10:52.214213
#44 9.905   Downloaded publicsuffix v2.3.0
2026-Feb-25 07:10:52.214213
#44 9.909   Downloaded itoa v1.0.17
2026-Feb-25 07:10:52.214213
#44 9.911   Downloaded memoffset v0.9.1
2026-Feb-25 07:10:52.214213
#44 9.913   Downloaded async-compression v0.4.40
2026-Feb-25 07:10:52.214213
#44 9.924   Downloaded mime v0.3.17
2026-Feb-25 07:10:52.214213
#44 9.927   Downloaded httparse v1.10.1
2026-Feb-25 07:10:52.214213
#44 9.930   Downloaded untrusted v0.9.0
2026-Feb-25 07:10:52.214213
#44 9.933   Downloaded psl-types v2.0.11
2026-Feb-25 07:10:52.214213
#44 9.935   Downloaded icu_locale_core v2.1.1
2026-Feb-25 07:10:52.214213
#44 9.946   Downloaded matchers v0.2.0
2026-Feb-25 07:10:52.214213
#44 9.949   Downloaded lru-slab v0.1.2
2026-Feb-25 07:10:52.214213
#44 9.950   Downloaded lazy_static v1.5.0
2026-Feb-25 07:10:52.214213
#44 9.953   Downloaded http-body-util v0.1.3
2026-Feb-25 07:10:52.214213
#44 9.956   Downloaded zeroize v1.8.2
2026-Feb-25 07:10:52.214213
#44 9.958   Downloaded prost v0.13.5
2026-Feb-25 07:10:52.214213
#44 9.961   Downloaded http v1.4.0
2026-Feb-25 07:10:52.214213
#44 9.966   Downloaded lock_api v0.4.14
2026-Feb-25 07:10:52.214213
#44 9.968   Downloaded memchr v2.8.0
2026-Feb-25 07:10:52.214213
#44 9.975   Downloaded proc-macro-error v1.0.4
2026-Feb-25 07:10:52.214213
#44 9.980   Downloaded prost-derive v0.13.5
2026-Feb-25 07:10:52.214213
#44 9.982   Downloaded serde_core v1.0.228
2026-Feb-25 07:10:52.214213
#44 9.986   Downloaded tinyvec v1.10.0
2026-Feb-25 07:10:52.214213
#44 9.989   Downloaded socket2 v0.6.2
2026-Feb-25 07:10:52.214213
#44 9.991   Downloaded log v0.4.29
2026-Feb-25 07:10:52.316284
#44 9.995   Downloaded parking_lot_core v0.9.12
2026-Feb-25 07:10:52.316284
#44 9.998   Downloaded crossbeam-channel v0.5.15
2026-Feb-25 07:10:52.316284
#44 10.00   Downloaded cc v1.2.56
2026-Feb-25 07:10:52.316284
#44 10.01   Downloaded typenum v1.19.0
2026-Feb-25 07:10:52.316284
#44 10.01   Downloaded quinn-udp v0.5.14
2026-Feb-25 07:10:52.316284
#44 10.02   Downloaded version_check v0.9.5
2026-Feb-25 07:10:52.316284
#44 10.02   Downloaded powerfmt v0.2.0
2026-Feb-25 07:10:52.316284
#44 10.02   Downloaded rand v0.8.5
2026-Feb-25 07:10:52.316284
#44 10.02   Downloaded tracing-core v0.1.36
2026-Feb-25 07:10:52.316284
#44 10.03   Downloaded serde_derive v1.0.228
2026-Feb-25 07:10:52.316284
#44 10.03   Downloaded serde v1.0.228
2026-Feb-25 07:10:52.316284
#44 10.04   Downloaded json5 v0.4.1
2026-Feb-25 07:10:52.316284
#44 10.04   Downloaded idna_adapter v1.2.1
2026-Feb-25 07:10:52.316284
#44 10.04   Downloaded heck v0.4.1
2026-Feb-25 07:10:52.316284
#44 10.05   Downloaded serde_v8 v0.209.0
2026-Feb-25 07:10:52.316284
#44 10.05   Downloaded matchit v0.8.4
2026-Feb-25 07:10:52.316284
#44 10.05   Downloaded config v0.14.1
2026-Feb-25 07:10:52.316284
#44 10.06   Downloaded bytes v1.11.1
2026-Feb-25 07:10:52.316284
#44 10.08   Downloaded prost-types v0.13.5
2026-Feb-25 07:10:52.316284
#44 10.08   Downloaded h2 v0.4.13
2026-Feb-25 07:10:52.316284
#44 10.09   Downloaded pin-project-internal v1.1.10
2026-Feb-25 07:10:52.316284
#44 10.09   Downloaded parking_lot v0.12.5
2026-Feb-25 07:10:52.422151
#44 10.10   Downloaded icu_properties v2.1.2
2026-Feb-25 07:10:52.422151
#44 10.10   Downloaded axum v0.7.9
2026-Feb-25 07:10:52.422151
#44 10.11   Downloaded yoke-derive v0.8.1
2026-Feb-25 07:10:52.422151
#44 10.12   Downloaded icu_properties_data v2.1.2
2026-Feb-25 07:10:52.422151
#44 10.13   Downloaded paste v1.0.15
2026-Feb-25 07:10:52.422151
#44 10.14   Downloaded hyper-tls v0.6.0
2026-Feb-25 07:10:52.422151
#44 10.14   Downloaded quinn-proto v0.11.13
2026-Feb-25 07:10:52.422151
#44 10.16   Downloaded heck v0.5.0
2026-Feb-25 07:10:52.422151
#44 10.16   Downloaded num_cpus v1.17.0
2026-Feb-25 07:10:52.422151
#44 10.16   Downloaded home v0.5.12
2026-Feb-25 07:10:52.422151
#44 10.17   Downloaded tower v0.4.13
2026-Feb-25 07:10:52.422151
#44 10.18   Downloaded which v4.4.2
2026-Feb-25 07:10:52.422151
#44 10.18   Downloaded proc-macro-error-attr v1.0.4
2026-Feb-25 07:10:52.422151
#44 10.18   Downloaded libloading v0.8.9
2026-Feb-25 07:10:52.422151
#44 10.19   Downloaded indexmap v1.9.3
2026-Feb-25 07:10:52.422151
#44 10.19   Downloaded mio v1.1.1
2026-Feb-25 07:10:52.422151
#44 10.20   Downloaded num-conv v0.2.0
2026-Feb-25 07:10:52.535300
#44 10.20   Downloaded hyper-rustls v0.27.7
2026-Feb-25 07:10:52.535300
#44 10.20   Downloaded tower-http v0.6.8
2026-Feb-25 07:10:52.535300
#44 10.21   Downloaded once_cell v1.21.3
2026-Feb-25 07:10:52.535300
#44 10.22   Downloaded regex v1.12.3
2026-Feb-25 07:10:52.535300
#44 10.22   Downloaded icu_provider v2.1.1
2026-Feb-25 07:10:52.535300
#44 10.23   Downloaded tokio-util v0.7.18
2026-Feb-25 07:10:52.535300
#44 10.24   Downloaded pest v2.8.6
2026-Feb-25 07:10:52.535300
#44 10.24   Downloaded icu_normalizer v2.1.1
2026-Feb-25 07:10:52.535300
#44 10.25   Downloaded percent-encoding v2.3.2
2026-Feb-25 07:10:52.535300
#44 10.25   Downloaded pest_derive v2.8.6
2026-Feb-25 07:10:52.535300
#44 10.25   Downloaded unicode-segmentation v1.12.0
2026-Feb-25 07:10:52.535300
#44 10.25   Downloaded litemap v0.8.1
2026-Feb-25 07:10:52.535300
#44 10.26   Downloaded litrs v1.0.0
2026-Feb-25 07:10:52.535300
#44 10.26   Downloaded deno_core v0.300.0
2026-Feb-25 07:10:52.535300
#44 10.27   Downloaded utoipa-gen v4.3.1
2026-Feb-25 07:10:52.535300
#44 10.28   Downloaded ipnet v2.11.0
2026-Feb-25 07:10:52.535300
#44 10.28   Downloaded pin-project-lite v0.2.16
2026-Feb-25 07:10:52.535300
#44 10.29   Downloaded uuid v1.21.0
2026-Feb-25 07:10:52.535300
#44 10.30   Downloaded openssl v0.10.75
2026-Feb-25 07:10:52.535300
#44 10.31   Downloaded zerocopy v0.8.39
2026-Feb-25 07:10:52.639042
#44 10.34   Downloaded pest_generator v2.8.6
2026-Feb-25 07:10:52.639042
#44 10.35   Downloaded serde_json v1.0.149
2026-Feb-25 07:10:52.639042
#44 10.36   Downloaded tracing-subscriber v0.3.22
2026-Feb-25 07:10:52.639042
#44 10.37   Downloaded openssl-sys v0.9.111
2026-Feb-25 07:10:52.639042
#44 10.38   Downloaded petgraph v0.7.1
2026-Feb-25 07:10:52.639042
#44 10.40   Downloaded syn v2.0.117
2026-Feb-25 07:10:52.639042
#44 10.41   Downloaded minimal-lexical v0.2.1
2026-Feb-25 07:10:52.639042
#44 10.42   Downloaded idna v1.1.0
2026-Feb-25 07:10:52.748086
#44 10.42   Downloaded syn v1.0.109
2026-Feb-25 07:10:52.748086
#44 10.43   Downloaded quinn v0.11.9
2026-Feb-25 07:10:52.748086
#44 10.44   Downloaded zerotrie v0.2.3
2026-Feb-25 07:10:52.748086
#44 10.44   Downloaded brotli v8.0.2
2026-Feb-25 07:10:52.748086
#44 10.46   Downloaded time v0.3.47
2026-Feb-25 07:10:52.748086
#44 10.48   Downloaded reqwest v0.12.28
2026-Feb-25 07:10:52.748086
#44 10.48   Downloaded hyper v1.8.1
2026-Feb-25 07:10:52.748086
#44 10.49   Downloaded rustls v0.23.36
2026-Feb-25 07:10:52.748086
#44 10.51   Downloaded tracing v0.1.44
2026-Feb-25 07:10:52.748086
#44 10.53   Downloaded regex-automata v0.4.14
2026-Feb-25 07:10:52.853205
#44 10.55   Downloaded vcpkg v0.2.15
2026-Feb-25 07:10:52.853205
#44 10.62   Downloaded zerovec v0.11.5
2026-Feb-25 07:10:52.853205
#44 10.63   Downloaded tokio v1.49.0
2026-Feb-25 07:10:52.988023
#44 10.69   Downloaded hyper-util v0.1.20
2026-Feb-25 07:10:52.988023
#44 10.70   Downloaded itertools v0.12.1
2026-Feb-25 07:10:52.988023
#44 10.70   Downloaded url v2.5.8
2026-Feb-25 07:10:52.988023
#44 10.71   Downloaded mio v0.8.11
2026-Feb-25 07:10:52.988023
#44 10.72   Downloaded ring v0.17.14
2026-Feb-25 07:10:52.988023
#44 10.77   Downloaded encoding_rs v0.8.35
2026-Feb-25 07:10:53.089487
#44 10.79   Downloaded rustix v1.1.3
2026-Feb-25 07:10:53.089487
#44 10.82   Downloaded rustix v0.38.44
2026-Feb-25 07:10:53.089487
#44 10.86   Downloaded regex-syntax v0.8.9
2026-Feb-25 07:10:53.089487
#44 10.86   Downloaded utoipa v4.2.3
2026-Feb-25 07:10:53.089487
#44 10.87   Downloaded pest_meta v2.8.6
2026-Feb-25 07:10:53.194831
#44 10.87   Downloaded hashbrown v0.12.3
2026-Feb-25 07:10:53.194831
#44 10.88   Downloaded native-tls v0.2.18
2026-Feb-25 07:10:53.194831
#44 10.88   Downloaded icu_collections v2.1.1
2026-Feb-25 07:10:53.194831
#44 10.89   Downloaded prettyplease v0.2.37
2026-Feb-25 07:10:53.194831
#44 10.89   Downloaded hashbrown v0.14.5
2026-Feb-25 07:10:53.194831
#44 10.90   Downloaded nom v7.1.3
2026-Feb-25 07:10:53.194831
#44 10.90   Downloaded indexmap v2.13.0
2026-Feb-25 07:10:53.194831
#44 10.91   Downloaded yaml-rust2 v0.8.1
2026-Feb-25 07:10:53.194831
#44 10.95   Downloaded iri-string v0.7.10
2026-Feb-25 07:10:53.194831
#44 10.96   Downloaded hashbrown v0.16.1
2026-Feb-25 07:10:53.194831
#44 10.96   Downloaded webpki-roots v1.0.6
2026-Feb-25 07:10:53.194831
#44 10.97   Downloaded linux-raw-sys v0.4.15
2026-Feb-25 07:10:53.336152
#44 11.02   Downloaded num-bigint v0.4.6
2026-Feb-25 07:10:53.336152
#44 11.02   Downloaded pin-project v1.1.10
2026-Feb-25 07:10:53.336152
#44 11.04   Downloaded miniz_oxide v0.8.9
2026-Feb-25 07:10:53.336152
#44 11.04   Downloaded tonic v0.12.3
2026-Feb-25 07:10:53.336152
#44 11.05   Downloaded rustls-webpki v0.103.9
2026-Feb-25 07:10:53.336152
#44 11.05   Downloaded potential_utf v0.1.4
2026-Feb-25 07:10:53.336152
#44 11.06   Downloaded linux-raw-sys v0.11.0
2026-Feb-25 07:10:53.336152
#44 11.11   Downloaded matchit v0.7.3
2026-Feb-25 07:10:53.437462
#44 11.12   Downloaded iana-time-zone v0.1.65
2026-Feb-25 07:10:53.437462
#44 11.12   Downloaded zmij v1.0.21
2026-Feb-25 07:10:53.437462
#44 11.12   Downloaded num-traits v0.2.19
2026-Feb-25 07:10:53.437462
#44 11.12   Downloaded tower v0.5.3
2026-Feb-25 07:10:53.437462
#44 11.14   Downloaded rand v0.9.2
2026-Feb-25 07:10:53.437462
#44 11.14   Downloaded miniz_oxide v0.7.4
2026-Feb-25 07:10:53.437462
#44 11.15   Downloaded openssl-macros v0.1.1
2026-Feb-25 07:10:53.437462
#44 11.15   Downloaded toml_edit v0.22.27
2026-Feb-25 07:10:53.437462
#44 11.15   Downloaded chrono v0.4.43
2026-Feb-25 07:10:53.437462
#44 11.16   Downloaded icu_normalizer_data v2.1.1
2026-Feb-25 07:10:53.437462
#44 11.16   Downloaded bitvec v1.0.1
2026-Feb-25 07:10:53.437462
#44 11.18   Downloaded bindgen v0.69.5
2026-Feb-25 07:10:53.437462
#44 11.19   Downloaded outref v0.1.0
2026-Feb-25 07:10:53.437462
#44 11.19   Downloaded inotify v0.9.6
2026-Feb-25 07:10:53.437462
#44 11.19   Downloaded itertools v0.14.0
2026-Feb-25 07:10:53.437462
#44 11.20   Downloaded winnow v0.7.14
2026-Feb-25 07:10:53.437462
#44 11.21   Downloaded wyz v0.5.1
2026-Feb-25 07:10:53.544582
#44 11.22   Downloaded which v6.0.3
2026-Feb-25 07:10:53.544582
#44 11.22   Downloaded proc-macro2 v1.0.106
2026-Feb-25 07:10:53.544582
#44 11.22   Downloaded brotli-decompressor v5.0.0
2026-Feb-25 07:10:53.544582
#44 11.23   Downloaded nu-ansi-term v0.50.3
2026-Feb-25 07:10:53.544582
#44 11.23   Downloaded libc v0.2.182
2026-Feb-25 07:10:53.544582
#44 11.28   Downloaded notify v6.1.1
2026-Feb-25 07:10:53.544582
#44 11.28   Downloaded futures-util v0.3.32
2026-Feb-25 07:10:53.544582
#44 11.30   Downloaded axum v0.8.8
2026-Feb-25 07:10:53.544582
#44 11.31   Downloaded aho-corasick v1.1.4
2026-Feb-25 07:10:53.544582
#44 11.32   Downloaded deno_core_icudata v0.0.73
2026-Feb-25 07:10:55.922987
#44 13.70   Downloaded v8 v0.101.0
2026-Feb-25 07:10:57.007259
#44 14.78    Compiling proc-macro2 v1.0.106
2026-Feb-25 07:10:57.007259
#44 14.78    Compiling unicode-ident v1.0.24
2026-Feb-25 07:10:57.108280
#44 14.78    Compiling quote v1.0.44
2026-Feb-25 07:10:57.108280
#44 14.78    Compiling libc v0.2.182
2026-Feb-25 07:10:57.108280
#44 14.78    Compiling cfg-if v1.0.4
2026-Feb-25 07:10:57.108280
#44 14.78    Compiling serde_core v1.0.228
2026-Feb-25 07:10:57.108280
#44 14.80    Compiling once_cell v1.21.3
2026-Feb-25 07:10:57.108280
#44 14.81    Compiling smallvec v1.15.1
2026-Feb-25 07:10:57.108280
#44 14.81    Compiling pin-project-lite v0.2.16
2026-Feb-25 07:10:57.108280
#44 14.83    Compiling shlex v1.3.0
2026-Feb-25 07:10:57.108280
#44 14.83    Compiling version_check v0.9.5
2026-Feb-25 07:10:57.108280
#44 14.83    Compiling memchr v2.8.0
2026-Feb-25 07:10:57.108280
#44 14.85    Compiling itoa v1.0.17
2026-Feb-25 07:10:57.108280
#44 14.85    Compiling parking_lot_core v0.9.12
2026-Feb-25 07:10:57.108280
#44 14.85    Compiling bytes v1.11.1
2026-Feb-25 07:10:57.108280
#44 14.87    Compiling futures-core v0.3.32
2026-Feb-25 07:10:57.108280
#44 14.87    Compiling scopeguard v1.2.0
2026-Feb-25 07:10:57.108280
#44 14.88    Compiling serde v1.0.228
2026-Feb-25 07:10:57.108280
#44 14.89    Compiling find-msvc-tools v0.1.9
2026-Feb-25 07:10:57.212971
#44 14.89    Compiling log v0.4.29
2026-Feb-25 07:10:57.212971
#44 14.91    Compiling stable_deref_trait v1.2.1
2026-Feb-25 07:10:57.212971
#44 14.91    Compiling futures-sink v0.3.32
2026-Feb-25 07:10:57.212971
#44 14.92    Compiling zerocopy v0.8.39
2026-Feb-25 07:10:57.212971
#44 14.92    Compiling equivalent v1.0.2
2026-Feb-25 07:10:57.212971
#44 14.92    Compiling hashbrown v0.16.1
2026-Feb-25 07:10:57.212971
#44 14.92    Compiling slab v0.4.12
2026-Feb-25 07:10:57.212971
#44 14.92    Compiling crc32fast v1.5.0
2026-Feb-25 07:10:57.212971
#44 14.92    Compiling autocfg v1.5.0
2026-Feb-25 07:10:57.212971
#44 14.92    Compiling futures-io v0.3.32
2026-Feb-25 07:10:57.212971
#44 14.92    Compiling futures-task v0.3.32
2026-Feb-25 07:10:57.212971
#44 14.92    Compiling writeable v0.6.2
2026-Feb-25 07:10:57.212971
#44 14.92    Compiling litemap v0.8.1
2026-Feb-25 07:10:57.212971
#44 14.93    Compiling lock_api v0.4.14
2026-Feb-25 07:10:57.212971
#44 ...
2026-Feb-25 07:10:57.212971
2026-Feb-25 07:10:57.212971
#24 [gpu-worker runtime 3/6] RUN apt-get update && apt-get install -y     ca-certificates     libssl3     ffmpeg     libavcodec58     libavformat58     libavutil56     libswscale5     libavfilter7     libavdevice58     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:10:57.212971
#24 15.77 Get:3 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgcc-s1 amd64 12.3.0-1ubuntu1~22.04.3 [53.9 kB]
2026-Feb-25 07:10:57.212971
#24 15.80 Get:4 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libssl3 amd64 3.0.2-0ubuntu1.21 [1905 kB]
2026-Feb-25 07:10:57.212971
#24 16.68 Get:5 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 ca-certificates all 20240203~22.04.1 [162 kB]
2026-Feb-25 07:10:57.212971
#24 16.70 Get:6 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libapparmor1 amd64 3.0.4-2ubuntu2.5 [39.6 kB]
2026-Feb-25 07:10:57.212971
#24 16.71 Get:7 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libdbus-1-3 amd64 1.12.20-2ubuntu4.1 [189 kB]
2026-Feb-25 07:10:57.212971
#24 16.74 Get:8 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libexpat1 amd64 2.4.7-1ubuntu0.7 [92.1 kB]
2026-Feb-25 07:10:57.212971
#24 16.76 Get:9 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 dbus amd64 1.12.20-2ubuntu4.1 [158 kB]
2026-Feb-25 07:10:57.212971
#24 16.78 Get:10 http://archive.ubuntu.com/ubuntu jammy/main amd64 libmd0 amd64 1.0.4-1build1 [23.0 kB]
2026-Feb-25 07:10:57.212971
#24 16.79 Get:11 http://archive.ubuntu.com/ubuntu jammy/main amd64 libbsd0 amd64 0.11.5-1 [44.8 kB]
2026-Feb-25 07:10:57.212971
#24 16.79 Get:12 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libelf1 amd64 0.186-1ubuntu0.1 [51.1 kB]
2026-Feb-25 07:10:57.212971
#24 16.89 Get:13 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libfribidi0 amd64 1.0.8-2ubuntu3.1 [26.1 kB]
2026-Feb-25 07:10:57.212971
#24 16.97 Get:14 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libglib2.0-0 amd64 2.72.4-0ubuntu2.9 [1467 kB]
2026-Feb-25 07:10:57.212971
#24 17.28 Get:15 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libglib2.0-data all 2.72.4-0ubuntu2.9 [5088 B]
2026-Feb-25 07:10:57.212971
#24 17.28 Get:16 http://archive.ubuntu.com/ubuntu jammy/main amd64 libicu70 amd64 70.1-2 [10.6 MB]
2026-Feb-25 07:10:57.212971
#24 18.18 Get:17 http://archive.ubuntu.com/ubuntu jammy/main amd64 libslang2 amd64 2.3.2-5build4 [468 kB]
2026-Feb-25 07:10:57.212971
#24 18.19 Get:18 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libxml2 amd64 2.9.13+dfsg-1ubuntu0.11 [765 kB]
2026-Feb-25 07:10:57.212971
#24 18.22 Get:19 http://archive.ubuntu.com/ubuntu jammy/main amd64 shared-mime-info amd64 2.1-2 [454 kB]
2026-Feb-25 07:10:57.212971
#24 18.24 Get:20 http://archive.ubuntu.com/ubuntu jammy/main amd64 ucf all 3.0043 [56.1 kB]
2026-Feb-25 07:10:57.212971
#24 18.24 Get:21 http://archive.ubuntu.com/ubuntu jammy/main amd64 xdg-user-dirs amd64 0.17-2ubuntu4 [53.9 kB]
2026-Feb-25 07:10:57.212971
#24 18.25 Get:22 http://archive.ubuntu.com/ubuntu jammy/main amd64 xkb-data all 2.33-1 [394 kB]
2026-Feb-25 07:10:57.212971
#24 18.26 Get:23 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libdrm-common all 2.4.113-2~ubuntu0.22.04.1 [5450 B]
2026-Feb-25 07:10:57.212971
#24 18.26 Get:24 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libdrm2 amd64 2.4.113-2~ubuntu0.22.04.1 [38.1 kB]
2026-Feb-25 07:10:57.212971
#24 18.26 Get:25 http://archive.ubuntu.com/ubuntu jammy/main amd64 libedit2 amd64 3.1-20210910-1build1 [96.8 kB]
2026-Feb-25 07:10:57.212971
#24 18.47 Get:26 http://archive.ubuntu.com/ubuntu jammy/main amd64 libnuma1 amd64 2.0.14-3ubuntu2 [22.5 kB]
2026-Feb-25 07:10:57.212971
#24 18.76 Get:27 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libpng16-16 amd64 1.6.37-3ubuntu0.4 [192 kB]
2026-Feb-25 07:10:57.212971
#24 18.77 Get:28 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libusb-1.0-0 amd64 2:1.0.25-1ubuntu2 [52.7 kB]
2026-Feb-25 07:10:57.212971
#24 18.77 Get:29 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxau6 amd64 1:1.0.9-1build5 [7634 B]
2026-Feb-25 07:10:57.212971
#24 18.77 Get:30 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxdmcp6 amd64 1:1.1.3-0ubuntu5 [10.9 kB]
2026-Feb-25 07:10:57.212971
#24 18.77 Get:31 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb1 amd64 1.14-3ubuntu3 [49.0 kB]
2026-Feb-25 07:10:57.212971
#24 18.77 Get:32 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libx11-data all 2:1.7.5-1ubuntu0.3 [120 kB]
2026-Feb-25 07:10:57.212971
#24 18.77 Get:33 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libx11-6 amd64 2:1.7.5-1ubuntu0.3 [667 kB]
2026-Feb-25 07:10:57.212971
#24 18.79 Get:34 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxext6 amd64 2:1.3.4-1build1 [31.8 kB]
2026-Feb-25 07:10:57.212971
#24 18.79 Get:35 http://archive.ubuntu.com/ubuntu jammy/main amd64 alsa-topology-conf all 1.2.5.1-2 [15.5 kB]
2026-Feb-25 07:10:57.212971
#24 19.06 Get:36 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libasound2-data all 1.2.6.1-1ubuntu1.1 [19.3 kB]
2026-Feb-25 07:10:57.212971
#24 19.35 Get:37 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libasound2 amd64 1.2.6.1-1ubuntu1.1 [391 kB]
2026-Feb-25 07:10:57.212971
#24 19.36 Get:38 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 alsa-ucm-conf all 1.2.6.3-1ubuntu1.12 [43.5 kB]
2026-Feb-25 07:10:57.212971
#24 19.36 Get:39 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libaom3 amd64 3.3.0-1ubuntu0.1 [1748 kB]
2026-Feb-25 07:10:57.212971
#24 19.40 Get:40 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libva2 amd64 2.14.0-1 [65.0 kB]
2026-Feb-25 07:10:57.212971
#24 19.40 Get:41 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libmfx1 amd64 22.3.0-1 [3105 kB]
2026-Feb-25 07:10:57.212971
#24 19.46 Get:42 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libva-drm2 amd64 2.14.0-1 [7502 B]
2026-Feb-25 07:10:57.212971
#24 19.46 Get:43 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxfixes3 amd64 1:6.0.0-1 [11.7 kB]
2026-Feb-25 07:10:57.212971
#24 19.46 Get:44 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libva-x11-2 amd64 2.14.0-1 [12.6 kB]
2026-Feb-25 07:10:57.212971
#24 19.46 Get:45 http://archive.ubuntu.com/ubuntu jammy/main amd64 libvdpau1 amd64 1.4-3build2 [27.0 kB]
2026-Feb-25 07:10:57.212971
#24 19.65 Get:46 http://archive.ubuntu.com/ubuntu jammy/universe amd64 ocl-icd-libopencl1 amd64 2.2.14-3 [39.1 kB]
2026-Feb-25 07:10:57.212971
#24 19.76 Get:47 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libavutil56 amd64 7:4.4.2-0ubuntu0.22.04.1 [290 kB]
2026-Feb-25 07:10:57.212971
#24 19.76 Get:48 http://archive.ubuntu.com/ubuntu jammy/main amd64 libbrotli1 amd64 1.0.9-2build6 [315 kB]
2026-Feb-25 07:10:57.212971
#24 19.77 Get:49 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libfreetype6 amd64 2.11.1+dfsg-1ubuntu0.3 [388 kB]
2026-Feb-25 07:10:57.212971
#24 19.78 Get:50 http://archive.ubuntu.com/ubuntu jammy/main amd64 fonts-dejavu-core all 2.37-2build1 [1041 kB]
2026-Feb-25 07:10:57.212971
#24 19.80 Get:51 http://archive.ubuntu.com/ubuntu jammy/main amd64 fontconfig-config all 2.13.1-4.2ubuntu5 [29.1 kB]
2026-Feb-25 07:10:57.212971
#24 19.80 Get:52 http://archive.ubuntu.com/ubuntu jammy/main amd64 libfontconfig1 amd64 2.13.1-4.2ubuntu5 [131 kB]
2026-Feb-25 07:10:57.212971
#24 19.80 Get:53 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libpixman-1-0 amd64 0.40.0-1ubuntu0.22.04.1 [264 kB]
2026-Feb-25 07:10:57.212971
#24 19.81 Get:54 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-render0 amd64 1.14-3ubuntu3 [16.4 kB]
2026-Feb-25 07:10:57.212971
#24 19.94 Get:55 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-shm0 amd64 1.14-3ubuntu3 [5780 B]
2026-Feb-25 07:10:57.212971
#24 20.23 Get:56 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxrender1 amd64 1:0.9.10-1build4 [19.7 kB]
2026-Feb-25 07:10:57.212971
#24 20.23 Get:57 http://archive.ubuntu.com/ubuntu jammy/main amd64 libcairo2 amd64 1.16.0-5ubuntu2 [628 kB]
2026-Feb-25 07:10:57.212971
#24 20.24 Get:58 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libcodec2-1.0 amd64 1.0.1-3 [8435 kB]
2026-Feb-25 07:10:57.212971
#24 20.41 Get:59 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libdav1d5 amd64 0.9.2-1 [463 kB]
2026-Feb-25 07:10:57.212971
#24 20.41 Get:60 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libgsm1 amd64 1.0.19-1 [27.7 kB]
2026-Feb-25 07:10:57.212971
#24 20.41 Get:61 http://archive.ubuntu.com/ubuntu jammy/main amd64 libmp3lame0 amd64 3.100-3build2 [141 kB]
2026-Feb-25 07:10:57.212971
#24 20.42 Get:62 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libopenjp2-7 amd64 2.4.0-6ubuntu0.4 [158 kB]
2026-Feb-25 07:10:57.212971
#24 20.42 Get:63 http://archive.ubuntu.com/ubuntu jammy/main amd64 libopus0 amd64 1.3.1-0.1build2 [203 kB]
2026-Feb-25 07:10:57.212971
#24 20.42 Get:64 http://archive.ubuntu.com/ubuntu jammy/main amd64 libcairo-gobject2 amd64 1.16.0-5ubuntu2 [19.4 kB]
2026-Feb-25 07:10:57.212971
#24 20.53 Get:65 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgdk-pixbuf2.0-common all 2.42.8+dfsg-1ubuntu0.4 [5546 B]
2026-Feb-25 07:10:57.212971
#24 20.82 Get:66 http://archive.ubuntu.com/ubuntu jammy/main amd64 libjpeg-turbo8 amd64 2.1.2-0ubuntu1 [134 kB]
2026-Feb-25 07:10:57.212971
#24 20.82 Get:67 http://archive.ubuntu.com/ubuntu jammy/main amd64 libjpeg8 amd64 8c-2ubuntu10 [2264 B]
2026-Feb-25 07:10:57.212971
#24 20.82 Get:68 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdeflate0 amd64 1.10-2 [70.9 kB]
2026-Feb-25 07:10:57.212971
#24 20.82 Get:69 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libjbig0 amd64 2.1-3.1ubuntu0.22.04.1 [29.2 kB]
2026-Feb-25 07:10:57.212971
#24 20.82 Get:70 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libwebp7 amd64 1.2.2-2ubuntu0.22.04.2 [206 kB]
2026-Feb-25 07:10:57.212971
#24 20.83 Get:71 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libtiff5 amd64 4.3.0-6ubuntu0.12 [185 kB]
2026-Feb-25 07:10:57.212971
#24 20.83 Get:72 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgdk-pixbuf-2.0-0 amd64 2.42.8+dfsg-1ubuntu0.4 [148 kB]
2026-Feb-25 07:10:57.212971
#24 20.83 Get:73 http://archive.ubuntu.com/ubuntu jammy/main amd64 fontconfig amd64 2.13.1-4.2ubuntu5 [177 kB]
2026-Feb-25 07:10:57.212971
#24 20.83 Get:74 http://archive.ubuntu.com/ubuntu jammy/main amd64 libgraphite2-3 amd64 1.3.14-1build2 [71.3 kB]
2026-Feb-25 07:10:57.212971
#24 21.11 Get:75 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libharfbuzz0b amd64 2.7.4-1ubuntu3.2 [353 kB]
2026-Feb-25 07:10:57.212971
#24 21.41 Get:76 http://archive.ubuntu.com/ubuntu jammy/main amd64 libthai-data all 0.1.29-1build1 [162 kB]
2026-Feb-25 07:10:57.212971
#24 21.41 Get:77 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdatrie1 amd64 0.2.13-2 [19.9 kB]
2026-Feb-25 07:10:57.212971
#24 21.41 Get:78 http://archive.ubuntu.com/ubuntu jammy/main amd64 libthai0 amd64 0.1.29-1build1 [19.2 kB]
2026-Feb-25 07:10:57.212971
#24 21.41 Get:79 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libpango-1.0-0 amd64 1.50.6+ds-2ubuntu1 [230 kB]
2026-Feb-25 07:10:57.212971
#24 21.41 Get:80 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libpangoft2-1.0-0 amd64 1.50.6+ds-2ubuntu1 [54.0 kB]
2026-Feb-25 07:10:57.212971
#24 21.41 Get:81 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libpangocairo-1.0-0 amd64 1.50.6+ds-2ubuntu1 [39.8 kB]
2026-Feb-25 07:10:57.212971
#24 21.41 Get:82 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 librsvg2-2 amd64 2.52.5+dfsg-3ubuntu0.2 [2974 kB]
2026-Feb-25 07:10:57.212971
#24 21.83 Get:83 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libshine3 amd64 3.1.1-2 [23.2 kB]
2026-Feb-25 07:10:57.212971
#24 21.83 Get:84 http://archive.ubuntu.com/ubuntu jammy/main amd64 libsnappy1v5 amd64 1.1.8-1build3 [17.5 kB]
2026-Feb-25 07:10:57.212971
#24 21.99 Get:85 http://archive.ubuntu.com/ubuntu jammy/main amd64 libspeex1 amd64 1.2~rc1.2-1.1ubuntu3 [57.9 kB]
2026-Feb-25 07:10:57.212971
#24 21.99 Get:86 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgomp1 amd64 12.3.0-1ubuntu1~22.04.3 [127 kB]
2026-Feb-25 07:10:57.212971
#24 22.12 Get:87 http://archive.ubuntu.com/ubuntu jammy/main amd64 libsoxr0 amd64 0.1.3-4build2 [79.8 kB]
2026-Feb-25 07:10:57.212971
#24 22.29 Get:88 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libswresample3 amd64 7:4.4.2-0ubuntu0.22.04.1 [62.2 kB]
2026-Feb-25 07:10:57.212971
#24 22.29 Get:89 http://archive.ubuntu.com/ubuntu jammy/main amd64 libogg0 amd64 1.3.5-0ubuntu3 [22.9 kB]
2026-Feb-25 07:10:57.212971
#24 22.29 Get:90 http://archive.ubuntu.com/ubuntu jammy/main amd64 libtheora0 amd64 1.1.1+dfsg.1-15ubuntu4 [209 kB]
2026-Feb-25 07:10:57.212971
#24 22.29 Get:91 http://archive.ubuntu.com/ubuntu jammy/main amd64 libtwolame0 amd64 0.4.0-2build2 [52.5 kB]
2026-Feb-25 07:10:57.212971
#24 22.29 Get:92 http://archive.ubuntu.com/ubuntu jammy/main amd64 libvorbis0a amd64 1.3.7-1build2 [99.2 kB]
2026-Feb-25 07:10:57.212971
#24 22.29 Get:93 http://archive.ubuntu.com/ubuntu jammy/main amd64 libvorbisenc2 amd64 1.3.7-1build2 [82.6 kB]
2026-Feb-25 07:10:57.212971
#24 22.58 Get:94 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libvpx7 amd64 1.11.0-2ubuntu2.5 [1078 kB]
2026-Feb-25 07:10:57.212971
#24 22.60 Get:95 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libwebpmux3 amd64 1.2.2-2ubuntu0.22.04.2 [20.5 kB]
2026-Feb-25 07:10:57.212971
#24 22.60 Get:96 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libx264-163 amd64 2:0.163.3060+git5db6aa6-2build1 [591 kB]
2026-Feb-25 07:10:57.212971
#24 23.00 Get:97 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libx265-199 amd64 3.5-2 [1170 kB]
2026-Feb-25 07:10:57.212971
#24 23.02 Get:98 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libxvidcore4 amd64 2:1.3.7-1 [201 kB]
2026-Feb-25 07:10:57.212971
#24 23.02 Get:99 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libzvbi-common all 0.2.35-19 [35.5 kB]
2026-Feb-25 07:10:57.212971
#24 23.02 Get:100 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libzvbi0 amd64 0.2.35-19 [262 kB]
2026-Feb-25 07:10:57.212971
#24 23.02 Get:101 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libavcodec58 amd64 7:4.4.2-0ubuntu0.22.04.1 [5567 kB]
2026-Feb-25 07:10:57.212971
#24 23.94 Get:102 http://archive.ubuntu.com/ubuntu jammy/main amd64 libraw1394-11 amd64 2.1.2-2build2 [27.0 kB]
2026-Feb-25 07:10:57.212971
#24 24.61 Get:103 http://archive.ubuntu.com/ubuntu jammy/main amd64 libavc1394-0 amd64 0.5.4-5build2 [17.0 kB]
2026-Feb-25 07:10:57.212971
#24 24.72 Get:104 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libass9 amd64 1:0.15.2-1 [97.5 kB]
2026-Feb-25 07:10:57.212971
#24 25.28 Get:105 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libudfread0 amd64 1.1.2-1 [16.2 kB]
2026-Feb-25 07:10:57.212971
#24 25.32 Get:106 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libbluray2 amd64 1:1.3.1-1 [159 kB]
2026-Feb-25 07:10:57.212971
#24 25.83 Get:107 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libchromaprint1 amd64 1.5.1-2 [28.4 kB]
2026-Feb-25 07:10:57.212971
#24 25.87 Get:108 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libgme0 amd64 0.6.3-2 [127 kB]
2026-Feb-25 07:10:57.324379
#24 ...
2026-Feb-25 07:10:57.324379
2026-Feb-25 07:10:57.324379
#44 [api builder 9/9] RUN cargo build --release --bin vps-gateway
2026-Feb-25 07:10:57.324379
#44 15.01    Compiling tracing-core v0.1.36
2026-Feb-25 07:10:57.324379
#44 15.08    Compiling cc v1.2.56
2026-Feb-25 07:10:57.324379
#44 15.08    Compiling futures-channel v0.3.32
2026-Feb-25 07:10:57.324379
#44 15.10    Compiling percent-encoding v2.3.2
2026-Feb-25 07:10:57.433607
#44 15.10    Compiling icu_normalizer_data v2.1.1
2026-Feb-25 07:10:57.433607
#44 15.10    Compiling icu_properties_data v2.1.2
2026-Feb-25 07:10:57.433607
#44 15.11    Compiling vcpkg v0.2.15
2026-Feb-25 07:10:57.433607
#44 15.12    Compiling zmij v1.0.21
2026-Feb-25 07:10:57.433607
#44 15.12    Compiling pkg-config v0.3.32
2026-Feb-25 07:10:57.433607
#44 15.12    Compiling regex-syntax v0.8.9
2026-Feb-25 07:10:57.433607
#44 15.16    Compiling glob v0.3.3
2026-Feb-25 07:10:57.433607
#44 15.18    Compiling serde_json v1.0.149
2026-Feb-25 07:10:57.433607
#44 15.21    Compiling tower-service v0.3.3
2026-Feb-25 07:10:57.553785
#44 15.21    Compiling rustix v0.38.44
2026-Feb-25 07:10:57.553785
#44 15.24    Compiling ahash v0.8.12
2026-Feb-25 07:10:57.553785
#44 15.24    Compiling prettyplease v0.2.37
2026-Feb-25 07:10:57.553785
#44 15.24    Compiling bitflags v2.11.0
2026-Feb-25 07:10:57.553785
#44 15.24    Compiling linux-raw-sys v0.4.15
2026-Feb-25 07:10:57.553785
#44 15.25    Compiling either v1.15.0
2026-Feb-25 07:10:57.553785
#44 15.26    Compiling httparse v1.10.1
2026-Feb-25 07:10:57.553785
#44 15.33    Compiling aho-corasick v1.1.4
2026-Feb-25 07:10:57.671308
#44 15.33    Compiling http v1.4.0
2026-Feb-25 07:10:57.671308
#44 15.33    Compiling num-traits v0.2.19
2026-Feb-25 07:10:57.671308
#44 15.34    Compiling form_urlencoded v1.2.2
2026-Feb-25 07:10:57.671308
#44 15.37    Compiling minimal-lexical v0.2.1
2026-Feb-25 07:10:57.671308
#44 15.40    Compiling alloc-no-stdlib v2.0.4
2026-Feb-25 07:10:57.671308
#44 15.45    Compiling fnv v1.0.7
2026-Feb-25 07:10:57.671308
#44 15.45    Compiling try-lock v0.2.5
2026-Feb-25 07:10:57.794942
#44 15.51    Compiling utf8_iter v1.0.4
2026-Feb-25 07:10:57.794942
#44 15.51    Compiling thiserror v1.0.69
2026-Feb-25 07:10:57.794942
#44 15.54    Compiling clang-sys v1.8.1
2026-Feb-25 07:10:57.794942
#44 15.54    Compiling atomic-waker v1.1.2
2026-Feb-25 07:10:57.794942
#44 15.54    Compiling home v0.5.12
2026-Feb-25 07:10:57.794942
#44 15.54    Compiling zeroize v1.8.2
2026-Feb-25 07:10:57.794942
#44 15.55    Compiling alloc-stdlib v0.2.2
2026-Feb-25 07:10:57.794942
#44 15.57    Compiling want v0.3.1
2026-Feb-25 07:10:57.794942
#44 15.57    Compiling nom v7.1.3
2026-Feb-25 07:10:57.901159
#44 15.62    Compiling libloading v0.8.9
2026-Feb-25 07:10:57.901159
#44 15.62    Compiling bindgen v0.69.5
2026-Feb-25 07:10:57.901159
#44 15.63    Compiling tower-layer v0.3.3
2026-Feb-25 07:10:57.901159
#44 15.66    Compiling openssl v0.10.75
2026-Feb-25 07:10:57.901159
#44 15.66    Compiling foreign-types-shared v0.1.1
2026-Feb-25 07:10:57.901159
#44 15.68    Compiling rustls-pki-types v1.14.0
2026-Feb-25 07:10:58.016016
#44 15.72    Compiling adler2 v2.0.1
2026-Feb-25 07:10:58.016016
#44 15.72    Compiling allocator-api2 v0.2.21
2026-Feb-25 07:10:58.016016
#44 15.73    Compiling simd-adler32 v0.3.8
2026-Feb-25 07:10:58.016016
#44 15.73    Compiling pin-utils v0.1.0
2026-Feb-25 07:10:58.016016
#44 15.74    Compiling crunchy v0.2.4
2026-Feb-25 07:10:58.016016
#44 15.76    Compiling httpdate v1.0.3
2026-Feb-25 07:10:58.016016
#44 15.76    Compiling untrusted v0.9.0
2026-Feb-25 07:10:58.016016
#44 15.76    Compiling brotli-decompressor v5.0.0
2026-Feb-25 07:10:58.016016
#44 15.77    Compiling foreign-types v0.3.2
2026-Feb-25 07:10:58.016016
#44 15.79    Compiling itertools v0.12.1
2026-Feb-25 07:10:58.127785
#44 15.81    Compiling sync_wrapper v1.0.2
2026-Feb-25 07:10:58.127785
#44 15.83    Compiling native-tls v0.2.18
2026-Feb-25 07:10:58.127785
#44 15.85    Compiling syn v2.0.117
2026-Feb-25 07:10:58.127785
#44 15.87    Compiling miniz_oxide v0.8.9
2026-Feb-25 07:10:58.127785
#44 15.88    Compiling num-conv v0.2.0
2026-Feb-25 07:10:58.127785
#44 15.88    Compiling rustc-hash v1.1.0
2026-Feb-25 07:10:58.127785
#44 15.90    Compiling powerfmt v0.2.0
2026-Feb-25 07:10:58.239663
#44 15.90    Compiling time-core v0.1.8
2026-Feb-25 07:10:58.239663
#44 15.90    Compiling lazycell v1.3.0
2026-Feb-25 07:10:58.239663
#44 15.91    Compiling rustversion v1.0.22
2026-Feb-25 07:10:58.239663
#44 15.95    Compiling crossbeam-utils v0.8.21
2026-Feb-25 07:10:58.239663
#44 15.96    Compiling lazy_static v1.5.0
2026-Feb-25 07:10:58.239663
#44 15.97    Compiling rustls v0.23.36
2026-Feb-25 07:10:58.239663
#44 15.99    Compiling base64 v0.22.1
2026-Feb-25 07:10:58.239663
#44 16.02    Compiling ipnet v2.11.0
2026-Feb-25 07:10:58.239663
#44 16.02    Compiling adler v1.0.2
2026-Feb-25 07:10:58.352325
#44 16.02    Compiling tiny-keccak v2.0.2
2026-Feb-25 07:10:58.352325
#44 16.03    Compiling gzip-header v1.0.0
2026-Feb-25 07:10:58.352325
#44 16.05    Compiling http-body v1.0.1
2026-Feb-25 07:10:58.352325
#44 16.07    Compiling deranged v0.5.8
2026-Feb-25 07:10:58.352325
#44 16.09    Compiling time-macros v0.2.27
2026-Feb-25 07:10:58.352325
#44 16.10    Compiling cookie v0.18.1
2026-Feb-25 07:10:58.352325
#44 16.13    Compiling
2026-Feb-25 07:10:58.465930
errno v0.3.14
2026-Feb-25 07:10:58.465930
#44 16.13    Compiling mio v1.1.1
2026-Feb-25 07:10:58.465930
#44 16.14    Compiling socket2 v0.6.2
2026-Feb-25 07:10:58.465930
#44 16.17    Compiling getrandom v0.2.17
2026-Feb-25 07:10:58.465930
#44 16.18    Compiling http-body-util v0.1.3
2026-Feb-25 07:10:58.465930
#44 16.18    Compiling miniz_oxide v0.7.4
2026-Feb-25 07:10:58.465930
#44 16.19    Compiling paste v1.0.15
2026-Feb-25 07:10:58.584443
#44 16.25    Compiling signal-hook-registry v1.4.8
2026-Feb-25 07:10:58.584443
#44 16.26    Compiling openssl-probe v0.2.1
2026-Feb-25 07:10:58.584443
#44 16.27    Compiling radium v0.7.0
2026-Feb-25 07:10:58.584443
#44 16.29    Compiling openssl-sys v0.9.111
2026-Feb-25 07:10:58.584443
#44 16.29    Compiling ring v0.17.14
2026-Feb-25 07:10:58.584443
#44 16.31    Compiling parking_lot v0.12.5
2026-Feb-25 07:10:58.584443
#44 16.31    Compiling rand_core v0.6.4
2026-Feb-25 07:10:58.584443
#44 16.36    Compiling flate2 v1.1.9
2026-Feb-25 07:10:58.692437
#44 16.38    Compiling fslock v0.2.1
2026-Feb-25 07:10:58.692437
#44 16.40    Compiling compression-core v0.4.31
2026-Feb-25 07:10:58.692437
#44 16.47    Compiling subtle v2.6.1
2026-Feb-25 07:10:58.932690
#44 16.60    Compiling encoding_rs v0.8.35
2026-Feb-25 07:10:58.932690
#44 16.61    Compiling anyhow v1.0.102
2026-Feb-25 07:10:58.932690
#44 16.62    Compiling outref v0.1.0
2026-Feb-25 07:10:58.932690
#44 16.62    Compiling litrs v1.0.0
2026-Feb-25 07:10:58.932690
#44 16.62    Compiling mime v0.3.17
2026-Feb-25 07:10:58.932690
#44 16.65    Compiling ucd-trie v0.1.7
2026-Feb-25 07:10:58.932690
#44 16.66    Compiling heck v0.4.1
2026-Feb-25 07:10:58.932690
#44 16.68    Compiling psl-types v2.0.11
2026-Feb-25 07:10:58.932690
#44 16.68    Compiling tap v1.0.1
2026-Feb-25 07:10:58.932690
#44 16.68    Compiling ryu v1.0.23
2026-Feb-25 07:10:58.932690
#44 16.71    Compiling simd-abstraction v0.7.1
2026-Feb-25 07:10:59.042604
#44 16.77    Compiling brotli v8.0.2
2026-Feb-25 07:10:59.042604
#44 16.79    Compiling regex-automata v0.4.14
2026-Feb-25 07:10:59.042604
#44 16.79    Compiling const-random-macro v0.1.16
2026-Feb-25 07:10:59.042604
#44 16.79    Compiling wyz v0.5.1
2026-Feb-25 07:10:59.042604
#44 16.82    Compiling pest v2.8.6
2026-Feb-25 07:10:59.168249
#44 16.87    Compiling num-integer v0.1.46
2026-Feb-25 07:10:59.168249
#44 16.87    Compiling webpki-roots v1.0.6
2026-Feb-25 07:10:59.168249
#44 16.91    Compiling indexmap v1.9.3
2026-Feb-25 07:10:59.168249
#44 16.91    Compiling memoffset v0.9.1
2026-Feb-25 07:10:59.168249
#44 16.94    Compiling proc-macro-error-attr v1.0.4
2026-Feb-25 07:10:59.274525
#44 16.96    Compiling iri-string v0.7.10
2026-Feb-25 07:10:59.274525
#44 16.97    Compiling funty v2.0.0
2026-Feb-25 07:10:59.274525
#44 17.01    Compiling syn v1.0.109
2026-Feb-25 07:10:59.274525
#44 17.02    Compiling uuid v1.21.0
2026-Feb-25 07:10:59.274525
#44 17.05    Compiling
2026-Feb-25 07:10:59.401014
base64-simd v0.7.0
2026-Feb-25 07:10:59.401014
#44 17.06    Compiling inotify-sys v0.1.5
2026-Feb-25 07:10:59.401014
#44 17.06    Compiling proc-macro-error v1.0.4
2026-Feb-25 07:10:59.401014
#44 17.10    Compiling which v4.4.2
2026-Feb-25 07:10:59.401014
#44 17.12    Compiling cexpr v0.6.0
2026-Feb-25 07:10:59.401014
#44 17.13    Compiling which v6.0.3
2026-Feb-25 07:10:59.401014
#44 17.13    Compiling document-features v0.2.12
2026-Feb-25 07:10:59.401014
#44 17.18    Compiling const-random v0.1.18
2026-Feb-25 07:10:59.501593
#44 17.23    Compiling data-encoding v2.10.0
2026-Feb-25 07:10:59.501593
#44 17.25    Compiling option-ext v0.2.0
2026-Feb-25 07:10:59.510112
#44 17.25    Compiling bit-vec v0.6.3
2026-Feb-25 07:10:59.510112
#44 17.28    Compiling bitflags v1.3.2
2026-Feb-25 07:10:59.610704
#44 17.28    Compiling if_chain v1.0.3
2026-Feb-25 07:10:59.610704
#44 17.31    Compiling unicode-id-start v1.4.0
2026-Feb-25 07:10:59.610704
#44 17.32    Compiling same-file v1.0.6
2026-Feb-25 07:10:59.610704
#44 17.35    Compiling hashbrown v0.12.3
2026-Feb-25 07:10:59.610704
#44 17.36    Compiling inotify v0.9.6
2026-Feb-25 07:10:59.610704
#44 17.37    Compiling dirs-sys v0.4.1
2026-Feb-25 07:10:59.610704
#44 17.38    Compiling bitvec v1.0.1
2026-Feb-25 07:10:59.610704
#44 17.39    Compiling time v0.3.47
2026-Feb-25 07:10:59.716155
#44 17.43    Compiling walkdir v2.5.0
2026-Feb-25 07:10:59.716155
#44 17.43    Compiling dlv-list v0.5.2
2026-Feb-25 07:10:59.716155
#44 17.44    Compiling crossbeam-channel v0.5.15
2026-Feb-25 07:10:59.716155
#44 17.47    Compiling filetime v0.2.27
2026-Feb-25 07:10:59.716155
#44 17.47    Compiling bit-set v0.5.3
2026-Feb-25 07:10:59.716155
#44 17.49    Compiling mio v0.8.11
2026-Feb-25 07:10:59.866869
#44 17.50    Compiling winnow v0.7.14
2026-Feb-25 07:10:59.866869
#44 17.52    Compiling matchit v0.8.4
2026-Feb-25 07:10:59.866869
#44 17.53    Compiling toml_write v0.1.2
2026-Feb-25 07:10:59.866869
#44 17.64    Compiling
2026-Feb-25 07:10:59.984561
indexmap v2.13.0
2026-Feb-25 07:10:59.984561
#44 17.66    Compiling serde_path_to_error v0.1.20
2026-Feb-25 07:10:59.984561
#44 17.70    Compiling iana-time-zone v0.1.65
2026-Feb-25 07:10:59.984561
#44 17.71    Compiling deno_core_icudata v0.0.73
2026-Feb-25 07:10:59.984561
#44 17.74    Compiling extractor v0.1.0 (/app/crates/extractor)
2026-Feb-25 07:10:59.984561
#44 17.74    Compiling cooked-waker v5.0.0
2026-Feb-25 07:10:59.984561
#44 17.76    Compiling static_assertions v1.1.0
2026-Feb-25 07:11:00.108765
#44 17.83 warning: function `strip_esm_exports` is never used
2026-Feb-25 07:11:00.108765
#44 17.83    --> crates/extractor/build.rs:132:4
2026-Feb-25 07:11:00.108765
#44 17.83     |
2026-Feb-25 07:11:00.108765
#44 17.83 132 | fn strip_esm_exports(content: &str) -> String {
2026-Feb-25 07:11:00.108765
#44 17.83     |    ^^^^^^^^^^^^^^^^^
2026-Feb-25 07:11:00.108765
#44 17.83     |
2026-Feb-25 07:11:00.108765
#44 17.83     = note: `#[warn(dead_code)]` on by default
2026-Feb-25 07:11:00.108765
#44 17.83
2026-Feb-25 07:11:00.108765
#44 17.83 warning: function `create_fallback_bundle` is never used
2026-Feb-25 07:11:00.108765
#44 17.83    --> crates/extractor/build.rs:152:4
2026-Feb-25 07:11:00.108765
#44 17.83     |
2026-Feb-25 07:11:00.108765
#44 17.83 152 | fn create_fallback_bundle(extractors_dir: &Path) -> String {
2026-Feb-25 07:11:00.108765
#44 17.83     |    ^^^^^^^^^^^^^^^^^^^^^^
2026-Feb-25 07:11:00.108765
#44 17.83
2026-Feb-25 07:11:00.108765
#44 17.84    Compiling chrono v0.4.43
2026-Feb-25 07:11:00.108765
#44 17.88    Compiling dirs v5.0.1
2026-Feb-25 07:11:00.237071
#44 17.90    Compiling num_cpus v1.17.0
2026-Feb-25 07:11:00.237071
#44 17.92    Compiling unicode-segmentation v1.12.0
2026-Feb-25 07:11:00.237071
#44 17.96    Compiling notify v6.1.1
2026-Feb-25 07:11:00.237071
#44 18.01    Compiling arraydeque v0.5.1
2026-Feb-25 07:11:00.386731
#44 18.03    Compiling base64 v0.21.7
2026-Feb-25 07:11:00.386731
#44 18.09 warning: `extractor` (build script) generated 2 warnings
2026-Feb-25 07:11:00.386731
#44 18.09    Compiling pest_meta v2.8.6
2026-Feb-25 07:11:00.386731
#44 18.16    Compiling tracing-log v0.2.0
2026-Feb-25 07:11:00.492044
#44 18.17    Compiling thread_local v1.1.9
2026-Feb-25 07:11:00.492044
#44 18.18    Compiling nu-ansi-term v0.50.3
2026-Feb-25 07:11:00.492044
#44 18.21    Compiling sharded-slab v0.1.7
2026-Feb-25 07:11:00.492044
#44 18.25    Compiling pathdiff v0.2.3
2026-Feb-25 07:11:00.597182
#44 18.27    Compiling dotenvy v0.15.7
2026-Feb-25 07:11:00.721937
#44 18.43    Compiling convert_case v0.6.0
2026-Feb-25 07:11:01.017526
#44 18.75    Compiling regex v1.12.3
2026-Feb-25 07:11:01.175856
#44 ...
2026-Feb-25 07:11:01.175856
2026-Feb-25 07:11:01.175856
#37 [gpu-worker builder 3/9] RUN apt-get update && apt-get install -y     curl     build-essential     pkg-config     libssl-dev     protobuf-compiler     ffmpeg     libavcodec-dev     libavformat-dev     libavutil-dev     libswscale-dev     libavfilter-dev     libavdevice-dev     clang     libclang-dev     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:11:01.175856
#37 19.82 Get:12 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libstdc++6 amd64 12.3.0-1ubuntu1~22.04.3 [699 kB]
2026-Feb-25 07:11:01.175856
#37 20.22 Get:13 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libc6 amd64 2.35-0ubuntu3.13 [3235 kB]
2026-Feb-25 07:11:01.175856
#37 21.13 Get:14 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libssl3 amd64 3.0.2-0ubuntu1.21 [1905 kB]
2026-Feb-25 07:11:01.175856
#37 21.53 Get:15 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libpython3.10-minimal amd64 3.10.12-1~22.04.14 [816 kB]
2026-Feb-25 07:11:01.175856
#37 21.86 Get:16 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libexpat1 amd64 2.4.7-1ubuntu0.7 [92.1 kB]
2026-Feb-25 07:11:01.175856
#37 22.16 Get:17 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 python3.10-minimal amd64 3.10.12-1~22.04.14 [2275 kB]
2026-Feb-25 07:11:01.175856
#37 22.58 Get:18 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 python3-minimal amd64 3.10.6-1~22.04.1 [24.3 kB]
2026-Feb-25 07:11:01.175856
#37 22.87 Get:19 http://archive.ubuntu.com/ubuntu jammy/main amd64 media-types all 7.0.0 [25.5 kB]
2026-Feb-25 07:11:01.175856
#37 23.16 Get:20 http://archive.ubuntu.com/ubuntu jammy/main amd64 libmpdec3 amd64 2.5.1-2build2 [86.8 kB]
2026-Feb-25 07:11:01.175856
#37 23.45 Get:21 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libpython3.10-stdlib amd64 3.10.12-1~22.04.14 [1850 kB]
2026-Feb-25 07:11:01.175856
#37 23.85 Get:22 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 python3.10 amd64 3.10.12-1~22.04.14 [509 kB]
2026-Feb-25 07:11:01.175856
#37 24.16 Get:23 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libpython3-stdlib amd64 3.10.6-1~22.04.1 [6812 B]
2026-Feb-25 07:11:01.175856
#37 24.45 Get:24 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 python3 amd64 3.10.6-1~22.04.1 [22.8 kB]
2026-Feb-25 07:11:01.175856
#37 24.74 Get:25 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libapparmor1 amd64 3.0.4-2ubuntu2.5 [39.6 kB]
2026-Feb-25 07:11:01.175856
#37 25.03 Get:26 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libdbus-1-3 amd64 1.12.20-2ubuntu4.1 [189 kB]
2026-Feb-25 07:11:01.175856
#37 25.32 Get:27 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 dbus amd64 1.12.20-2ubuntu4.1 [158 kB]
2026-Feb-25 07:11:01.175856
#37 25.95 Get:28 http://archive.ubuntu.com/ubuntu jammy/main amd64 libmd0 amd64 1.0.4-1build1 [23.0 kB]
2026-Feb-25 07:11:01.175856
#37 26.24 Get:29 http://archive.ubuntu.com/ubuntu jammy/main amd64 libbsd0 amd64 0.11.5-1 [44.8 kB]
2026-Feb-25 07:11:01.175856
#37 26.52 Get:30 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libelf1 amd64 0.186-1ubuntu0.1 [51.1 kB]
2026-Feb-25 07:11:01.175856
#37 26.82 Get:31 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libfribidi0 amd64 1.0.8-2ubuntu3.1 [26.1 kB]
2026-Feb-25 07:11:01.175856
#37 27.10 Get:32 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libglib2.0-0 amd64 2.72.4-0ubuntu2.9 [1467 kB]
2026-Feb-25 07:11:01.175856
#37 27.48 Get:33 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libglib2.0-data all 2.72.4-0ubuntu2.9 [5088 B]
2026-Feb-25 07:11:01.175856
#37 27.77 Get:34 http://archive.ubuntu.com/ubuntu jammy/main amd64 libicu70 amd64 70.1-2 [10.6 MB]
2026-Feb-25 07:11:01.175856
#37 28.67 Get:35 http://archive.ubuntu.com/ubuntu jammy/main amd64 libslang2 amd64 2.3.2-5build4 [468 kB]
2026-Feb-25 07:11:01.175856
#37 28.96 Get:36 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libxml2 amd64 2.9.13+dfsg-1ubuntu0.11 [765 kB]
2026-Feb-25 07:11:01.175856
#37 29.27 Get:37 http://archive.ubuntu.com/ubuntu jammy/main amd64 libyaml-0-2 amd64 0.2.2-1build2 [51.6 kB]
2026-Feb-25 07:11:01.175856
#37 29.55 Get:38 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 python3-pkg-resources all 59.6.0-1.2ubuntu0.22.04.3 [133 kB]
2026-Feb-25 07:11:01.175856
#37 29.84 Get:39 http://archive.ubuntu.com/ubuntu jammy/main amd64 python3-yaml amd64 5.4.1-1ubuntu1 [129 kB]
2026-Feb-25 07:11:01.308137
#37 ...
2026-Feb-25 07:11:01.308137
2026-Feb-25 07:11:01.308137
#24 [gpu-worker runtime 3/6] RUN apt-get update && apt-get install -y     ca-certificates     libssl3     ffmpeg     libavcodec58     libavformat58     libavutil56     libswscale5     libavfilter7     libavdevice58     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:11:01.308137
#24 26.12 Get:109 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libmpg123-0 amd64 1.29.3-1ubuntu0.1 [172 kB]
2026-Feb-25 07:11:01.308137
#24 26.26 Get:110 http://archive.ubuntu.com/ubuntu jammy/main amd64 libvorbisfile3 amd64 1.3.7-1build2 [17.1 kB]
2026-Feb-25 07:11:01.308137
#24 26.30 Get:111 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libopenmpt0 amd64 0.6.1-1 [592 kB]
2026-Feb-25 07:11:01.308137
#24 26.82 Get:112 http://archive.ubuntu.com/ubuntu jammy/main amd64 librabbitmq4 amd64 0.10.0-1ubuntu2 [39.3 kB]
2026-Feb-25 07:11:01.308137
#24 26.83 Get:113 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libsrt1.4-gnutls amd64 1.4.4-4 [309 kB]
2026-Feb-25 07:11:01.308137
#24 27.06 Get:114 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libssh-gcrypt-4 amd64 0.9.6-2ubuntu0.22.04.6 [225 kB]
2026-Feb-25 07:11:01.308137
#24 27.12 Get:115 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libnorm1 amd64 1.5.9+dfsg-2 [221 kB]
2026-Feb-25 07:11:01.308137
#24 27.29 Get:116 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libpgm-5.3-0 amd64 5.3.128~dfsg-2 [161 kB]
2026-Feb-25 07:11:01.308137
#24 27.33 Get:117 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libsodium23 amd64 1.0.18-1ubuntu0.22.04.1 [164 kB]
2026-Feb-25 07:11:01.308137
#24 27.37 Get:118 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libzmq5 amd64 4.3.4-2 [256 kB]
2026-Feb-25 07:11:01.315988
#24 27.42 Get:119 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libavformat58 amd64 7:4.4.2-0ubuntu0.22.04.1 [1103 kB]
2026-Feb-25 07:11:01.315988
#24 27.71 Get:120 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libbs2b0 amd64 3.1.0+dfsg-2.2build1 [10.2 kB]
2026-Feb-25 07:11:01.315988
#24 27.71 Get:121 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libflite1 amd64 2.2-3 [13.7 MB]
2026-Feb-25 07:11:01.315988
#24 29.08 Get:122 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libserd-0-0 amd64 0.30.10-2 [40.8 kB]
2026-Feb-25 07:11:01.315988
#24 29.08 Get:123 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libsord-0-0 amd64 0.16.8-2 [21.2 kB]
2026-Feb-25 07:11:01.315988
#24 29.08 Get:124 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libsratom-0-0 amd64 0.6.8-1 [17.0 kB]
2026-Feb-25 07:11:01.315988
#24 29.08 Get:125 http://archive.ubuntu.com/ubuntu jammy/universe amd64 liblilv-0-0 amd64 0.24.12-2 [42.8 kB]
2026-Feb-25 07:11:01.315988
#24 29.08 Get:126 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libmysofa1 amd64 1.2.1~dfsg0-1 [1157 kB]
2026-Feb-25 07:11:01.315988
#24 29.11 Get:127 http://archive.ubuntu.com/ubuntu jammy/main amd64 libblas3 amd64 3.10.0-2ubuntu1 [228 kB]
2026-Feb-25 07:11:01.315988
#24 29.12 Get:128 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libquadmath0 amd64 12.3.0-1ubuntu1~22.04.3 [154 kB]
2026-Feb-25 07:11:01.315988
#24 29.12 Get:129 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgfortran5 amd64 12.3.0-1ubuntu1~22.04.3 [879 kB]
2026-Feb-25 07:11:01.315988
#24 29.15 Get:130 http://archive.ubuntu.com/ubuntu jammy/main amd64 liblapack3 amd64 3.10.0-2ubuntu1 [2504 kB]
2026-Feb-25 07:11:01.315988
#24 29.38 Get:131 http://archive.ubuntu.com/ubuntu jammy/main amd64 libasyncns0 amd64 0.8-6build2 [12.8 kB]
2026-Feb-25 07:11:01.315988
#24 29.38 Get:132 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libflac8 amd64 1.3.3-2ubuntu0.2 [111 kB]
2026-Feb-25 07:11:01.315988
#24 29.67 Get:133 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libsndfile1 amd64 1.0.31-2ubuntu0.2 [196 kB]
2026-Feb-25 07:11:01.315988
#24 29.67 Get:134 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libx11-xcb1 amd64 2:1.7.5-1ubuntu0.3 [7802 B]
2026-Feb-25 07:11:01.315988
#24 29.67 Get:135 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libpulse0 amd64 1:15.99.1+dfsg1-1ubuntu2.2 [298 kB]
2026-Feb-25 07:11:01.315988
#24 29.68 Get:136 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libsphinxbase3 amd64 0.8+5prealpha+1-13build1 [126 kB]
2026-Feb-25 07:11:01.315988
#24 29.68 Get:137 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libpocketsphinx3 amd64 0.8.0+real5prealpha+1-14ubuntu1 [132 kB]
2026-Feb-25 07:11:01.315988
#24 29.68 Get:138 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libpostproc55 amd64 7:4.4.2-0ubuntu0.22.04.1 [60.1 kB]
2026-Feb-25 07:11:01.315988
#24 29.68 Get:139 http://archive.ubuntu.com/ubuntu jammy/main amd64 libsamplerate0 amd64 0.2.2-1build1 [1359 kB]
2026-Feb-25 07:11:01.315988
#24 29.70 Get:140 http://archive.ubuntu.com/ubuntu jammy/universe amd64 librubberband2 amd64 2.0.0-2 [90.0 kB]
2026-Feb-25 07:11:01.315988
#24 29.96 Get:141 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libswscale5 amd64 7:4.4.2-0ubuntu0.22.04.1 [180 kB]
2026-Feb-25 07:11:01.460252
#24 29.96 Get:142 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libvidstab1.1 amd64 1.1.0-2 [35.0 kB]
2026-Feb-25 07:11:01.602751
#24 30.26 Get:143 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libzimg2 amd64 3.0.3+ds1-1 [241 kB]
2026-Feb-25 07:11:01.735037
#24 30.26 Get:144 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libavfilter7 amd64 7:4.4.2-0ubuntu0.22.04.1 [1496 kB]
2026-Feb-25 07:11:01.735037
#24 30.28 Get:145 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libcaca0 amd64 0.99.beta19-2.2ubuntu4.1 [224 kB]
2026-Feb-25 07:11:01.735037
#24 30.29 Get:146 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libcdio19 amd64 2.1.0-3ubuntu0.2 [63.6 kB]
2026-Feb-25 07:11:01.735037
#24 30.29 Get:147 http://archive.ubuntu.com/ubuntu jammy/main amd64 libcdio-cdda2 amd64 10.2+2.0.0-1build3 [16.7 kB]
2026-Feb-25 07:11:01.735037
#24 30.29 Get:148 http://archive.ubuntu.com/ubuntu jammy/main amd64 libcdio-paranoia2 amd64 10.2+2.0.0-1build3 [15.9 kB]
2026-Feb-25 07:11:01.735037
#24 30.29 Get:149 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libdc1394-25 amd64 2.2.6-4 [88.8 kB]
2026-Feb-25 07:11:01.743335
#24 30.29 Get:150 http://archive.ubuntu.com/ubuntu jammy/main amd64 libglvnd0 amd64 1.4.0-1 [73.6 kB]
2026-Feb-25 07:11:01.950675
#24 30.55 Get:151 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libglapi-mesa amd64 23.2.1-1ubuntu3.1~22.04.3 [35.4 kB]
2026-Feb-25 07:11:01.950675
#24 30.55 Get:152 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-dri2-0 amd64 1.14-3ubuntu3 [7206 B]
2026-Feb-25 07:11:02.338620
#24 30.84 Get:153 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-dri3-0 amd64 1.14-3ubuntu3 [6968 B]
2026-Feb-25 07:11:02.338620
#24 30.84 Get:154 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-glx0 amd64 1.14-3ubuntu3 [25.9 kB]
2026-Feb-25 07:11:02.338620
#24 30.84 Get:155 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-present0 amd64 1.14-3ubuntu3 [5734 B]
2026-Feb-25 07:11:02.338620
#24 30.84 Get:156 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-randr0 amd64 1.14-3ubuntu3 [18.3 kB]
2026-Feb-25 07:11:02.338620
#24 30.84 Get:157 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-sync1 amd64 1.14-3ubuntu3 [9416 B]
2026-Feb-25 07:11:02.338620
#24 30.85 Get:158 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-xfixes0 amd64 1.14-3ubuntu3 [9996 B]
2026-Feb-25 07:11:02.338620
#24 30.85 Get:159 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxshmfence1 amd64 1.3-1build4 [5394 B]
2026-Feb-25 07:11:02.338620
#24 30.85 Get:160 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxxf86vm1 amd64 1:1.1.4-1build3 [10.4 kB]
2026-Feb-25 07:11:02.483018
#24 31.14 Get:161 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libdrm-amdgpu1 amd64 2.4.113-2~ubuntu0.22.04.1 [19.9 kB]
2026-Feb-25 07:11:02.632567
#24 31.14 Get:162 http://archive.ubuntu.com/ubuntu jammy/main amd64 libpciaccess0 amd64 0.16-3 [19.1 kB]
2026-Feb-25 07:11:02.776110
#24 31.43 Get:163 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libdrm-intel1 amd64 2.4.113-2~ubuntu0.22.04.1 [66.7 kB]
2026-Feb-25 07:11:02.885064
#24 31.43 Get:164 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libdrm-nouveau2 amd64 2.4.113-2~ubuntu0.22.04.1 [17.5 kB]
2026-Feb-25 07:11:02.885064
#24 31.43 Get:165 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libdrm-radeon1 amd64 2.4.113-2~ubuntu0.22.04.1 [21.6 kB]
2026-Feb-25 07:11:02.885064
#24 31.43 Get:166 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libllvm15 amd64 1:15.0.7-0ubuntu0.22.04.3 [25.4 MB]
2026-Feb-25 07:11:03.497550
#24 32.05 Get:167 http://archive.ubuntu.com/ubuntu jammy/main amd64 libsensors-config all 1:3.6.0-7ubuntu1 [5274 B]
2026-Feb-25 07:11:03.497550
#24 32.05 Get:168 http://archive.ubuntu.com/ubuntu jammy/main amd64 libsensors5 amd64 1:3.6.0-7ubuntu1 [26.3 kB]
2026-Feb-25 07:11:03.497550
#24 32.05 Get:169 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgl1-mesa-dri amd64 23.2.1-1ubuntu3.1~22.04.3 [8860 kB]
2026-Feb-25 07:11:03.497550
#24 32.12 Get:170 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libglx-mesa0 amd64 23.2.1-1ubuntu3.1~22.04.3 [158 kB]
2026-Feb-25 07:11:03.497550
#24 32.12 Get:171 http://archive.ubuntu.com/ubuntu jammy/main amd64 libglx0 amd64 1.4.0-1 [41.0 kB]
2026-Feb-25 07:11:03.497550
#24 32.12 Get:172 http://archive.ubuntu.com/ubuntu jammy/main amd64 libgl1 amd64 1.4.0-1 [110 kB]
2026-Feb-25 07:11:03.497550
#24 32.12 Get:173 http://archive.ubuntu.com/ubuntu jammy/main amd64 libiec61883-0 amd64 1.2.0-4build3 [25.9 kB]
2026-Feb-25 07:11:03.497550
#24 32.13 Get:174 http://archive.ubuntu.com/ubuntu jammy/main amd64 libjack-jackd2-0 amd64 1.9.20~dfsg-1 [293 kB]
2026-Feb-25 07:11:03.497550
#24 32.13 Get:175 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libopenal-data all 1:1.19.1-2build3 [164 kB]
2026-Feb-25 07:11:03.687771
#24 32.34 Get:176 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libsndio7.0 amd64 1.8.1-1.1 [29.3 kB]
2026-Feb-25 07:11:03.687771
#24 32.34 Get:177 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libopenal1 amd64 1:1.19.1-2build3 [535 kB]
2026-Feb-25 07:11:03.980295
#24 32.63 Get:178 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libwayland-client0 amd64 1.20.0-1ubuntu0.1 [25.9 kB]
2026-Feb-25 07:11:04.223501
#24 32.63 Get:179 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdecor-0-0 amd64 0.1.0-3build1 [15.1 kB]
2026-Feb-25 07:11:04.223501
#24 32.63 Get:180 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libwayland-server0 amd64 1.20.0-1ubuntu0.1 [34.3 kB]
2026-Feb-25 07:11:04.223501
#24 32.63 Get:181 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgbm1 amd64 23.2.1-1ubuntu3.1~22.04.3 [33.5 kB]
2026-Feb-25 07:11:04.223501
#24 32.64 Get:182 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libwayland-cursor0 amd64 1.20.0-1ubuntu0.1 [10.7 kB]
2026-Feb-25 07:11:04.223501
#24 32.64 Get:183 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libwayland-egl1 amd64 1.20.0-1ubuntu0.1 [5582 B]
2026-Feb-25 07:11:04.223501
#24 32.64 Get:184 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcursor1 amd64 1:1.2.0-2build4 [20.9 kB]
2026-Feb-25 07:11:04.223501
#24 32.64 Get:185 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxi6 amd64 2:1.8-1build1 [32.6 kB]
2026-Feb-25 07:11:04.273235
#24 32.93 Get:186 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxinerama1 amd64 2:1.1.4-3 [7382 B]
2026-Feb-25 07:11:04.511702
#24 32.93 Get:187 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxkbcommon0 amd64 1.4.0-1 [125 kB]
2026-Feb-25 07:11:04.650933
#24 33.22 Get:188 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxrandr2 amd64 2:1.5.2-1build1 [20.4 kB]
2026-Feb-25 07:11:04.650933
#24 33.22 Get:189 http://archive.ubuntu.com/ubuntu jammy/main amd64 x11-common all 1:7.7+23ubuntu2 [23.4 kB]
2026-Feb-25 07:11:04.650933
#24 33.22 Get:190 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxss1 amd64 1:1.2.3-1build2 [8476 B]
2026-Feb-25 07:11:04.650933
#24 33.22 Get:191 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libsdl2-2.0-0 amd64 2.0.20+dfsg-2ubuntu1.22.04.1 [582 kB]
2026-Feb-25 07:11:04.650933
#24 33.23 Get:192 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-shape0 amd64 1.14-3ubuntu3 [6158 B]
2026-Feb-25 07:11:04.650933
#24 33.23 Get:193 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxv1 amd64 2:1.0.11-1build2 [11.2 kB]
2026-Feb-25 07:11:04.650933
#24 33.23 Get:194 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libavdevice58 amd64 7:4.4.2-0ubuntu0.22.04.1 [87.5 kB]
2026-Feb-25 07:11:04.650933
#24 33.23 Get:195 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 ffmpeg amd64 7:4.4.2-0ubuntu0.22.04.1 [1696 kB]
2026-Feb-25 07:11:04.939258
#24 33.51 Get:196 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libigdgmm12 amd64 22.1.2+ds1-1 [139 kB]
2026-Feb-25 07:11:04.939258
#24 33.52 Get:197 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 intel-media-va-driver amd64 22.3.1+dfsg1-1ubuntu2 [2283 kB]
2026-Feb-25 07:11:05.047867
#24 ...
2026-Feb-25 07:11:05.047867
2026-Feb-25 07:11:05.047867
#44 [api builder 9/9] RUN cargo build --release --bin vps-gateway
2026-Feb-25 07:11:05.047867
#44 19.30    Compiling synstructure v0.13.2
2026-Feb-25 07:11:05.047867
#44 19.30    Compiling pest_generator v2.8.6
2026-Feb-25 07:11:05.047867
#44 19.51    Compiling ppv-lite86 v0.2.21
2026-Feb-25 07:11:05.047867
#44 19.63    Compiling hashbrown v0.14.5
2026-Feb-25 07:11:05.047867
#44 19.68    Compiling serde_derive v1.0.228
2026-Feb-25 07:11:05.047867
#44 19.68    Compiling zerofrom-derive v0.1.6
2026-Feb-25 07:11:05.047867
#44 19.68    Compiling tokio-macros v2.6.0
2026-Feb-25 07:11:05.047867
#44 19.68    Compiling yoke-derive v0.8.1
2026-Feb-25 07:11:05.047867
#44 19.68    Compiling zerovec-derive v0.11.2
2026-Feb-25 07:11:05.047867
#44 19.69    Compiling displaydoc v0.2.5
2026-Feb-25 07:11:05.047867
#44 19.69    Compiling tracing-attributes v0.1.31
2026-Feb-25 07:11:05.047867
#44 19.70    Compiling futures-macro v0.3.32
2026-Feb-25 07:11:05.047867
#44 19.72    Compiling thiserror-impl v1.0.69
2026-Feb-25 07:11:05.047867
#44 19.73    Compiling openssl-macros v0.1.1
2026-Feb-25 07:11:05.047867
#44 19.74    Compiling pin-project-internal v1.1.10
2026-Feb-25 07:11:05.047867
#44 19.74    Compiling proc-macro-rules-macros v0.4.0
2026-Feb-25 07:11:05.047867
#44 19.74    Compiling strum_macros v0.25.3
2026-Feb-25 07:11:05.047867
#44 19.75    Compiling async-stream-impl v0.3.6
2026-Feb-25 07:11:05.047867
#44 19.76    Compiling utoipa-gen v4.3.1
2026-Feb-25 07:11:05.047867
#44 19.79    Compiling async-trait v0.1.89
2026-Feb-25 07:11:05.047867
#44 19.86    Compiling rand_chacha v0.3.1
2026-Feb-25 07:11:05.047867
#44 19.96    Compiling pest_derive v2.8.6
2026-Feb-25 07:11:05.047867
#44 20.19    Compiling rand v0.8.5
2026-Feb-25 07:11:05.047867
#44 20.33    Compiling hashlink v0.8.4
2026-Feb-25 07:11:05.047867
#44 20.34    Compiling ordered-multimap v0.7.3
2026-Feb-25 07:11:05.047867
#44 20.36    Compiling dashmap v6.1.0
2026-Feb-25 07:11:05.047867
#44 20.45    Compiling compression-codecs v0.4.37
2026-Feb-25 07:11:05.047867
#44 20.47    Compiling tokio v1.49.0
2026-Feb-25 07:11:05.047867
#44 20.58    Compiling async-stream v0.3.6
2026-Feb-25 07:11:05.047867
#44 20.66    Compiling rust-ini v0.20.0
2026-Feb-25 07:11:05.047867
#44 20.68    Compiling yaml-rust2 v0.8.1
2026-Feb-25 07:11:05.047867
#44 20.73    Compiling proc-macro-rules v0.4.0
2026-Feb-25 07:11:05.047867
#44 20.76    Compiling futures-util v0.3.32
2026-Feb-25 07:11:05.047867
#44 20.91    Compiling num-bigint v0.4.6
2026-Feb-25 07:11:05.047867
#44 20.99    Compiling zerofrom v0.1.6
2026-Feb-25 07:11:05.047867
#44 21.01    Compiling pin-project v1.1.10
2026-Feb-25 07:11:05.047867
#44 21.06    Compiling yoke v0.8.1
2026-Feb-25 07:11:05.047867
#44 21.12    Compiling tracing v0.1.44
2026-Feb-25 07:11:05.047867
#44 21.14    Compiling matchers v0.2.0
2026-Feb-25 07:11:05.047867
#44 21.17    Compiling zerovec v0.11.5
2026-Feb-25 07:11:05.047867
#44 21.17    Compiling zerotrie v0.2.3
2026-Feb-25 07:11:05.047867
#44 21.19    Compiling strum v0.25.0
2026-Feb-25 07:11:05.047867
#44 21.25    Compiling deno_ops v0.176.0
2026-Feb-25 07:11:05.047867
#44 21.28    Compiling axum-core v0.5.6
2026-Feb-25 07:11:05.047867
#44 21.28    Compiling tracing-subscriber v0.3.22
2026-Feb-25 07:11:05.047867
#44 21.63    Compiling tinystr v0.8.2
2026-Feb-25 07:11:05.047867
#44 21.63    Compiling potential_utf v0.1.4
2026-Feb-25 07:11:05.047867
#44 21.68    Compiling icu_collections v2.1.1
2026-Feb-25 07:11:05.047867
#44 21.77    Compiling icu_locale_core v2.1.1
2026-Feb-25 07:11:05.047867
#44 22.31    Compiling icu_provider v2.1.1
2026-Feb-25 07:11:05.047867
#44 22.39    Compiling serde_urlencoded v0.7.1
2026-Feb-25 07:11:05.047867
#44 22.39    Compiling debugid v0.8.0
2026-Feb-25 07:11:05.047867
#44 22.39    Compiling toml_datetime v0.6.11
2026-Feb-25 07:11:05.047867
#44 22.39    Compiling bincode v1.3.3
2026-Feb-25 07:11:05.047867
#44 22.39    Compiling serde_spanned v0.6.9
2026-Feb-25 07:11:05.047867
#44 22.39    Compiling json5 v0.4.1
2026-Feb-25 07:11:05.047867
#44 22.39    Compiling ron v0.8.1
2026-Feb-25 07:11:05.047867
#44 22.48    Compiling icu_normalizer v2.1.1
2026-Feb-25 07:11:05.047867
#44 22.48    Compiling icu_properties v2.1.2
2026-Feb-25 07:11:05.047867
#44 22.59    Compiling toml_edit v0.22.27
2026-Feb-25 07:11:05.047867
#44 22.82    Compiling utoipa v4.2.3
2026-Feb-25 07:11:05.153025
#44 22.83    Compiling futures-executor v0.3.32
2026-Feb-25 07:11:05.310243
#44 22.95    Compiling futures v0.3.32
2026-Feb-25 07:11:05.310243
#44 23.09    Compiling v8 v0.101.0
2026-Feb-25 07:11:05.480835
#44 23.26    Compiling idna_adapter v1.2.1
2026-Feb-25 07:11:05.682807
#44 23.31    Compiling idna v1.1.0
2026-Feb-25 07:11:05.765198
#44 23.54    Compiling url v2.5.8
2026-Feb-25 07:11:05.952882
#44 23.54    Compiling publicsuffix v2.3.0
2026-Feb-25 07:11:06.235040
#44 23.96    Compiling tokio-util v0.7.18
2026-Feb-25 07:11:06.235040
#44 23.96    Compiling tower v0.5.3
2026-Feb-25 07:11:06.235040
#44 23.96    Compiling tokio-native-tls v0.3.1
2026-Feb-25 07:11:06.235040
#44 23.96    Compiling async-compression v0.4.40
2026-Feb-25 07:11:06.235040
#44 23.96    Compiling deno_unsync v0.4.4
2026-Feb-25 07:11:06.235040
#44 23.97    Compiling tokio-stream v0.1.18
2026-Feb-25 07:11:06.235040
#44 23.99    Compiling toml v0.8.23
2026-Feb-25 07:11:06.235040
#44 24.00    Compiling cookie_store v0.22.1
2026-Feb-25 07:11:06.235040
#44 24.01    Compiling sourcemap v8.0.1
2026-Feb-25 07:11:06.528895
#44 24.31    Compiling h2 v0.4.13
2026-Feb-25 07:11:06.528895
#44 24.31    Compiling tower-http v0.6.8
2026-Feb-25 07:11:06.707951
#44 24.31    Compiling tower v0.4.13
2026-Feb-25 07:11:06.707951
#44 24.35    Compiling config v0.14.1
2026-Feb-25 07:11:07.169331
#44 24.95    Compiling rustls-webpki v0.103.9
2026-Feb-25 07:11:09.205312
#44 26.98    Compiling hyper v1.8.1
2026-Feb-25 07:11:09.929115
#44 27.71    Compiling tokio-rustls v0.26.4
2026-Feb-25 07:11:10.154593
#44 27.93    Compiling hyper-util v0.1.20
2026-Feb-25 07:11:11.348076
#44 28.99    Compiling hyper-tls v0.6.0
2026-Feb-25 07:11:11.348076
#44 28.99    Compiling hyper-rustls v0.27.7
2026-Feb-25 07:11:11.348076
#44 28.99    Compiling axum v0.8.8
2026-Feb-25 07:11:11.348076
#44 29.13    Compiling reqwest v0.12.28
2026-Feb-25 07:11:11.498517
#44 ...
2026-Feb-25 07:11:11.498517
2026-Feb-25 07:11:11.498517
#37 [gpu-worker builder 3/9] RUN apt-get update && apt-get install -y     curl     build-essential     pkg-config     libssl-dev     protobuf-compiler     ffmpeg     libavcodec-dev     libavformat-dev     libavutil-dev     libswscale-dev     libavfilter-dev     libavdevice-dev     clang     libclang-dev     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:11:11.498517
#37 30.13 Get:40 http://archive.ubuntu.com/ubuntu jammy/main amd64 shared-mime-info amd64 2.1-2 [454 kB]
2026-Feb-25 07:11:11.498517
#37 30.43 Get:41 http://archive.ubuntu.com/ubuntu jammy/main amd64 ucf all 3.0043 [56.1 kB]
2026-Feb-25 07:11:11.498517
#37 30.72 Get:42 http://archive.ubuntu.com/ubuntu jammy/main amd64 xdg-user-dirs amd64 0.17-2ubuntu4 [53.9 kB]
2026-Feb-25 07:11:11.498517
#37 31.01 Get:43 http://archive.ubuntu.com/ubuntu jammy/main amd64 xkb-data all 2.33-1 [394 kB]
2026-Feb-25 07:11:11.498517
#37 31.30 Get:44 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libdrm-common all 2.4.113-2~ubuntu0.22.04.1 [5450 B]
2026-Feb-25 07:11:11.498517
#37 31.59 Get:45 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libdrm2 amd64 2.4.113-2~ubuntu0.22.04.1 [38.1 kB]
2026-Feb-25 07:11:11.498517
#37 31.88 Get:46 http://archive.ubuntu.com/ubuntu jammy/main amd64 libedit2 amd64 3.1-20210910-1build1 [96.8 kB]
2026-Feb-25 07:11:11.498517
#37 32.17 Get:47 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libnghttp2-14 amd64 1.43.0-1ubuntu0.2 [76.9 kB]
2026-Feb-25 07:11:11.498517
#37 32.45 Get:48 http://archive.ubuntu.com/ubuntu jammy/main amd64 libnuma1 amd64 2.0.14-3ubuntu2 [22.5 kB]
2026-Feb-25 07:11:11.498517
#37 32.74 Get:49 http://archive.ubuntu.com/ubuntu jammy/main amd64 libpipeline1 amd64 1.5.5-1 [23.5 kB]
2026-Feb-25 07:11:11.498517
#37 33.03 Get:50 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libpng16-16 amd64 1.6.37-3ubuntu0.4 [192 kB]
2026-Feb-25 07:11:11.498517
#37 33.32 Get:51 http://archive.ubuntu.com/ubuntu jammy/main amd64 libpsl5 amd64 0.21.0-1.2build2 [58.4 kB]
2026-Feb-25 07:11:11.498517
#37 33.61 Get:52 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libusb-1.0-0 amd64 2:1.0.25-1ubuntu2 [52.7 kB]
2026-Feb-25 07:11:11.498517
#37 33.90 Get:53 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxau6 amd64 1:1.0.9-1build5 [7634 B]
2026-Feb-25 07:11:11.498517
#37 34.18 Get:54 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxdmcp6 amd64 1:1.1.3-0ubuntu5 [10.9 kB]
2026-Feb-25 07:11:11.498517
#37 34.80 Get:55 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb1 amd64 1.14-3ubuntu3 [49.0 kB]
2026-Feb-25 07:11:11.498517
#37 35.09 Get:56 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libx11-data all 2:1.7.5-1ubuntu0.3 [120 kB]
2026-Feb-25 07:11:11.498517
#37 35.38 Get:57 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libx11-6 amd64 2:1.7.5-1ubuntu0.3 [667 kB]
2026-Feb-25 07:11:11.498517
#37 35.68 Get:58 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxext6 amd64 2:1.3.4-1build1 [31.8 kB]
2026-Feb-25 07:11:11.498517
#37 35.97 Get:59 http://archive.ubuntu.com/ubuntu jammy/main amd64 publicsuffix all 20211207.1025-1 [129 kB]
2026-Feb-25 07:11:11.498517
#37 36.26 Get:60 http://archive.ubuntu.com/ubuntu jammy/main amd64 alsa-topology-conf all 1.2.5.1-2 [15.5 kB]
2026-Feb-25 07:11:11.498517
#37 36.54 Get:61 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libasound2-data all 1.2.6.1-1ubuntu1.1 [19.3 kB]
2026-Feb-25 07:11:11.498517
#37 36.83 Get:62 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libasound2 amd64 1.2.6.1-1ubuntu1.1 [391 kB]
2026-Feb-25 07:11:11.498517
#37 37.13 Get:63 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 alsa-ucm-conf all 1.2.6.3-1ubuntu1.12 [43.5 kB]
2026-Feb-25 07:11:11.498517
#37 37.41 Get:64 http://archive.ubuntu.com/ubuntu jammy/main amd64 binfmt-support amd64 2.2.1-2 [55.8 kB]
2026-Feb-25 07:11:11.498517
#37 37.70 Get:65 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libllvm14 amd64 1:14.0.0-1ubuntu1.1 [24.0 MB]
2026-Feb-25 07:11:11.498517
#37 38.63 Get:66 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libclang-cpp14 amd64 1:14.0.0-1ubuntu1.1 [12.1 MB]
2026-Feb-25 07:11:11.498517
#37 39.03 Get:67 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 g++-11 amd64 11.4.0-1ubuntu1~22.04.3 [11.4 MB]
2026-Feb-25 07:11:11.498517
#37 39.80 Get:68 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 gcc-11 amd64 11.4.0-1ubuntu1~22.04.3 [20.1 MB]
2026-Feb-25 07:11:12.010819
#37 40.68 Get:69 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 cpp-11 amd64 11.4.0-1ubuntu1~22.04.3 [10.0 MB]
2026-Feb-25 07:11:12.843665
#37 ...
2026-Feb-25 07:11:12.843665
2026-Feb-25 07:11:12.843665
#24 [gpu-worker runtime 3/6] RUN apt-get update && apt-get install -y     ca-certificates     libssl3     ffmpeg     libavcodec58     libavformat58     libavutil56     libswscale5     libavfilter7     libavdevice58     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:11:12.843665
#24 33.81 Get:198 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libaacs0 amd64 0.11.1-1 [64.1 kB]
2026-Feb-25 07:11:12.843665
#24 33.81 Get:199 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libbdplus0 amd64 0.2.0-1 [52.2 kB]
2026-Feb-25 07:11:12.843665
#24 33.81 Get:200 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdecor-0-plugin-1-cairo amd64 0.1.0-3build1 [20.4 kB]
2026-Feb-25 07:11:12.843665
#24 33.81 Get:201 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgdk-pixbuf2.0-bin amd64 2.42.8+dfsg-1ubuntu0.4 [14.1 kB]
2026-Feb-25 07:11:12.843665
#24 33.81 Get:202 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgl1-amber-dri amd64 21.3.9-0ubuntu1~22.04.1 [4218 kB]
2026-Feb-25 07:11:12.843665
#24 34.46 Get:203 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 librsvg2-common amd64 2.52.5+dfsg-3ubuntu0.2 [17.7 kB]
2026-Feb-25 07:11:12.843665
#24 35.08 Get:204 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 mesa-va-drivers amd64 23.2.1-1ubuntu3.1~22.04.3 [4100 kB]
2026-Feb-25 07:11:12.843665
#24 38.45 Get:205 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 mesa-vdpau-drivers amd64 23.2.1-1ubuntu3.1~22.04.3 [3820 kB]
2026-Feb-25 07:11:12.843665
#24 38.91 Get:206 http://archive.ubuntu.com/ubuntu jammy/universe amd64 i965-va-driver amd64 2.4.1+dfsg1-1 [302 kB]
2026-Feb-25 07:11:12.843665
#24 39.05 Get:207 http://archive.ubuntu.com/ubuntu jammy/universe amd64 va-driver-all amd64 2.14.0-1 [3984 B]
2026-Feb-25 07:11:12.843665
#24 39.05 Get:208 http://archive.ubuntu.com/ubuntu jammy/main amd64 vdpau-driver-all amd64 1.4-3build2 [4510 B]
2026-Feb-25 07:11:12.843665
#24 39.05 Get:209 http://archive.ubuntu.com/ubuntu jammy/universe amd64 pocketsphinx-en-us all 0.8.0+real5prealpha+1-14ubuntu1 [27.6 MB]
2026-Feb-25 07:11:12.843665
#24 41.38 debconf: delaying package configuration, since apt-utils is not installed
2026-Feb-25 07:11:12.843665
#24 41.42 Fetched 161 MB in 29s (5606 kB/s)
2026-Feb-25 07:11:12.843665
#24 41.44 (Reading database ... 
(Reading database ... 5%
(Reading database ... 10%
(Reading database ... 15%
(Reading database ... 20%
(Reading database ... 25%
(Reading database ... 30%
(Reading database ... 35%
(Reading database ... 40%
(Reading database ... 45%
(Reading database ... 50%
(Reading database ... 55%
(Reading database ... 60%
(Reading database ... 65%
(Reading database ... 70%
(Reading database ... 75%
(Reading database ... 80%
(Reading database ... 85%
(Reading database ... 90%
(Reading database ... 95%
(Reading database ... 100%
(Reading database ... 5309 files and directories currently installed.)
2026-Feb-25 07:11:12.843665
#24 41.44 Preparing to unpack .../gcc-12-base_12.3.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:11:12.843665
#24 41.45 Unpacking gcc-12-base:amd64 (12.3.0-1ubuntu1~22.04.3) over (12.3.0-1ubuntu1~22.04) ...
2026-Feb-25 07:11:12.843665
#24 41.47 Setting up gcc-12-base:amd64 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:11:12.843665
#24 41.50 (Reading database ...
2026-Feb-25 07:11:12.958210
(Reading database ... 5%
(Reading database ... 10%
(Reading database ... 15%
(Reading database ... 20%
(Reading database ... 25%
(Reading database ... 30%
(Reading database ... 35%
(Reading database ... 40%
(Reading database ... 45%
(Reading database ... 50%
(Reading database ... 55%
(Reading database ... 60%
(Reading database ... 65%
(Reading database ... 70%
(Reading database ... 75%
(Reading database ... 80%
(Reading database ... 85%
(Reading database ... 90%
(Reading database ... 95%
(Reading database ... 100%
(Reading database ... 5309 files and directories currently installed.)
2026-Feb-25 07:11:12.958210
#24 41.50 Preparing to unpack .../libstdc++6_12.3.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:11:12.958210
#24 41.52 Unpacking libstdc++6:amd64 (12.3.0-1ubuntu1~22.04.3) over (12.3.0-1ubuntu1~22.04) ...
2026-Feb-25 07:11:12.958210
#24 41.56 Setting up libstdc++6:amd64 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:11:12.958210
#24 41.58 (Reading database ... 
(Reading database ... 5%
(Reading database ... 10%
(Reading database ... 15%
(Reading database ... 20%
(Reading database ... 25%
(Reading database ... 30%
(Reading database ... 35%
(Reading database ... 40%
(Reading database ... 45%
(Reading database ... 50%
(Reading database ... 55%
(Reading database ... 60%
(Reading database ... 65%
(Reading database ... 70%
(Reading database ... 75%
(Reading database ... 80%
(Reading database ... 85%
(Reading database ... 90%
(Reading database ... 95%
(Reading database ... 100%
(Reading database ... 5309 files and directories currently installed.)
2026-Feb-25 07:11:12.958210
#24 41.58 Preparing to unpack .../libgcc-s1_12.3.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:11:12.958210
#24 41.59 Unpacking libgcc-s1:amd64 (12.3.0-1ubuntu1~22.04.3) over (12.3.0-1ubuntu1~22.04) ...
2026-Feb-25 07:11:12.958210
#24 41.61 Setting up libgcc-s1:amd64 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:11:13.098203
#24 41.64 (Reading database ... 
(Reading database ... 5%
(Reading database ... 10%
(Reading database ... 15%
(Reading database ... 20%
(Reading database ... 25%
(Reading database ... 30%
(Reading database ... 35%
(Reading database ... 40%
(Reading database ... 45%
(Reading database ... 50%
(Reading database ... 55%
(Reading database ... 60%
(Reading database ... 65%
(Reading database ... 70%
(Reading database ... 75%
(Reading database ... 80%
(Reading database ... 85%
(Reading database ... 90%
(Reading database ... 95%
(Reading database ... 100%
(Reading database ... 5309 files and directories currently installed.)
2026-Feb-25 07:11:13.098203
#24 41.64 Preparing to unpack .../libssl3_3.0.2-0ubuntu1.21_amd64.deb ...
2026-Feb-25 07:11:13.098203
#24 41.64 Unpacking libssl3:amd64 (3.0.2-0ubuntu1.21) over (3.0.2-0ubuntu1.14) ...
2026-Feb-25 07:11:13.098203
#24 41.70 Setting up libssl3:amd64 (3.0.2-0ubuntu1.21) ...
2026-Feb-25 07:11:13.098203
#24 41.75 debconf: unable to initialize frontend: Dialog
2026-Feb-25 07:11:13.098203
#24 41.75 debconf: (TERM is not set, so the dialog frontend is not usable.)
2026-Feb-25 07:11:13.098203
#24 41.75 debconf: falling back to frontend: Readline
2026-Feb-25 07:11:13.098203
#24 ...
2026-Feb-25 07:11:13.098203
2026-Feb-25 07:11:13.098203
#37 [gpu-worker builder 3/9] RUN apt-get update && apt-get install -y     curl     build-essential     pkg-config     libssl-dev     protobuf-compiler     ffmpeg     libavcodec-dev     libavformat-dev     libavutil-dev     libswscale-dev     libavfilter-dev     libavdevice-dev     clang     libclang-dev     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:11:13.098203
#37 41.73 Get:70 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libasan6 amd64 11.4.0-1ubuntu1~22.04.3 [2283 kB]
2026-Feb-25 07:11:13.499621
#37 42.07 Get:71 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libstdc++-11-dev amd64 11.4.0-1ubuntu1~22.04.3 [2101 kB]
2026-Feb-25 07:11:13.830908
#37 42.41 Get:72 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgcc-11-dev amd64 11.4.0-1ubuntu1~22.04.3 [2517 kB]
2026-Feb-25 07:11:14.155097
#37 42.76 Get:73 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libtsan0 amd64 11.4.0-1ubuntu1~22.04.3 [2260 kB]
2026-Feb-25 07:11:14.493512
#37 43.11 Get:74 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 gcc-11-base amd64 11.4.0-1ubuntu1~22.04.3 [216 kB]
2026-Feb-25 07:11:14.772855
#37 43.40 Get:75 http://archive.ubuntu.com/ubuntu jammy/main amd64 libgc1 amd64 1:8.0.6-1.1build1 [96.8 kB]
2026-Feb-25 07:11:15.111757
#37 43.69 Get:76 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libobjc4 amd64 12.3.0-1ubuntu1~22.04.3 [48.7 kB]
2026-Feb-25 07:11:15.358359
#37 43.98 Get:77 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libobjc-11-dev amd64 11.4.0-1ubuntu1~22.04.3 [196 kB]
2026-Feb-25 07:11:15.710030
#37 44.27 Get:78 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libc6-i386 amd64 2.35-0ubuntu3.13 [2837 kB]
2026-Feb-25 07:11:15.961605
#37 44.63 Get:79 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 lib32gcc-s1 amd64 12.3.0-1ubuntu1~22.04.3 [63.9 kB]
2026-Feb-25 07:11:16.400375
#37 44.92 Get:80 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 lib32stdc++6 amd64 12.3.0-1ubuntu1~22.04.3 [739 kB]
2026-Feb-25 07:11:16.959920
#37 45.55 Get:81 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libclang-common-14-dev amd64 1:14.0.0-1ubuntu1.1 [5975 kB]
2026-Feb-25 07:11:17.384904
#37 45.99 Get:82 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 llvm-14-linker-tools amd64 1:14.0.0-1ubuntu1.1 [1355 kB]
2026-Feb-25 07:11:17.645597
#37 46.32 Get:83 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libclang1-14 amd64 1:14.0.0-1ubuntu1.1 [6792 kB]
2026-Feb-25 07:11:18.107508
#37 46.78 Get:84 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 clang-14 amd64 1:14.0.0-1ubuntu1.1 [81.2 kB]
2026-Feb-25 07:11:18.213865
#37 ...
2026-Feb-25 07:11:18.213865
2026-Feb-25 07:11:18.213865
#24 [gpu-worker runtime 3/6] RUN apt-get update && apt-get install -y     ca-certificates     libssl3     ffmpeg     libavcodec58     libavformat58     libavutil56     libswscale5     libavfilter7     libavdevice58     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:11:18.213865
#24 41.75 debconf: unable to initialize frontend: Readline
2026-Feb-25 07:11:18.213865
#24 41.75 debconf: (Can't locate Term/ReadLine.pm in @INC (you may need to install the Term::ReadLine module) (@INC contains: /etc/perl /usr/local/lib/x86_64-linux-gnu/perl/5.34.0 /usr/local/share/perl/5.34.0 /usr/lib/x86_64-linux-gnu/perl5/5.34 /usr/share/perl5 /usr/lib/x86_64-linux-gnu/perl-base /usr/lib/x86_64-linux-gnu/perl/5.34 /usr/share/perl/5.34 /usr/local/lib/site_perl) at /usr/share/perl5/Debconf/FrontEnd/Readline.pm line 7.)
2026-Feb-25 07:11:18.213865
#24 41.75 debconf: falling back to frontend: Teletype
2026-Feb-25 07:11:18.213865
#24 41.80 (Reading database ... 
(Reading database ... 5%
(Reading database ... 10%
(Reading database ... 15%
(Reading database ... 20%
(Reading database ... 25%
(Reading database ... 30%
(Reading database ... 35%
(Reading database ... 40%
(Reading database ... 45%
(Reading database ... 50%
(Reading database ... 55%
(Reading database ... 60%
(Reading database ... 65%
(Reading database ... 70%
(Reading database ... 75%
(Reading database ... 80%
(Reading database ... 85%
(Reading database ... 90%
(Reading database ... 95%
(Reading database ... 100%
(Reading database ... 5309 files and directories currently installed.)
2026-Feb-25 07:11:18.213865
#24 41.81 Preparing to unpack .../000-ca-certificates_20240203~22.04.1_all.deb ...
2026-Feb-25 07:11:18.213865
#24 41.81 Unpacking ca-certificates (20240203~22.04.1) over (20230311ubuntu0.22.04.1) ...
2026-Feb-25 07:11:18.213865
#24 41.94 Selecting previously unselected package libapparmor1:amd64.
2026-Feb-25 07:11:18.213865
#24 41.94 Preparing to unpack .../001-libapparmor1_3.0.4-2ubuntu2.5_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 41.94 Unpacking libapparmor1:amd64 (3.0.4-2ubuntu2.5) ...
2026-Feb-25 07:11:18.213865
#24 41.96 Selecting previously unselected package libdbus-1-3:amd64.
2026-Feb-25 07:11:18.213865
#24 41.96 Preparing to unpack .../002-libdbus-1-3_1.12.20-2ubuntu4.1_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 41.96 Unpacking libdbus-1-3:amd64 (1.12.20-2ubuntu4.1) ...
2026-Feb-25 07:11:18.213865
#24 41.98 Selecting previously unselected package libexpat1:amd64.
2026-Feb-25 07:11:18.213865
#24 41.98 Preparing to unpack .../003-libexpat1_2.4.7-1ubuntu0.7_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 41.98 Unpacking libexpat1:amd64 (2.4.7-1ubuntu0.7) ...
2026-Feb-25 07:11:18.213865
#24 42.00 Selecting previously unselected package dbus.
2026-Feb-25 07:11:18.213865
#24 42.00 Preparing to unpack .../004-dbus_1.12.20-2ubuntu4.1_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 42.01 Unpacking dbus (1.12.20-2ubuntu4.1) ...
2026-Feb-25 07:11:18.213865
#24 42.04 Selecting previously unselected package libmd0:amd64.
2026-Feb-25 07:11:18.213865
#24 42.04 Preparing to unpack .../005-libmd0_1.0.4-1build1_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 42.04 Unpacking libmd0:amd64 (1.0.4-1build1) ...
2026-Feb-25 07:11:18.213865
#24 42.05 Selecting previously unselected package libbsd0:amd64.
2026-Feb-25 07:11:18.213865
#24 42.05 Preparing to unpack .../006-libbsd0_0.11.5-1_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 42.05 Unpacking libbsd0:amd64 (0.11.5-1) ...
2026-Feb-25 07:11:18.213865
#24 42.07 Selecting previously unselected package libelf1:amd64.
2026-Feb-25 07:11:18.213865
#24 42.07 Preparing to unpack .../007-libelf1_0.186-1ubuntu0.1_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 42.07 Unpacking libelf1:amd64 (0.186-1ubuntu0.1) ...
2026-Feb-25 07:11:18.213865
#24 42.09 Selecting previously unselected package libfribidi0:amd64.
2026-Feb-25 07:11:18.213865
#24 42.09 Preparing to unpack .../008-libfribidi0_1.0.8-2ubuntu3.1_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 42.09 Unpacking libfribidi0:amd64 (1.0.8-2ubuntu3.1) ...
2026-Feb-25 07:11:18.213865
#24 42.11 Selecting previously unselected package libglib2.0-0:amd64.
2026-Feb-25 07:11:18.213865
#24 42.11 Preparing to unpack .../009-libglib2.0-0_2.72.4-0ubuntu2.9_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 42.11 Unpacking libglib2.0-0:amd64 (2.72.4-0ubuntu2.9) ...
2026-Feb-25 07:11:18.213865
#24 42.14 Selecting previously unselected package libglib2.0-data.
2026-Feb-25 07:11:18.213865
#24 42.14 Preparing to unpack .../010-libglib2.0-data_2.72.4-0ubuntu2.9_all.deb ...
2026-Feb-25 07:11:18.213865
#24 42.14 Unpacking libglib2.0-data (2.72.4-0ubuntu2.9) ...
2026-Feb-25 07:11:18.213865
#24 42.15 Selecting previously unselected package libicu70:amd64.
2026-Feb-25 07:11:18.213865
#24 42.15 Preparing to unpack .../011-libicu70_70.1-2_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 42.16 Unpacking libicu70:amd64 (70.1-2) ...
2026-Feb-25 07:11:18.213865
#24 42.26 Selecting previously unselected package libslang2:amd64.
2026-Feb-25 07:11:18.213865
#24 42.27 Preparing to unpack .../012-libslang2_2.3.2-5build4_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 42.27 Unpacking libslang2:amd64 (2.3.2-5build4) ...
2026-Feb-25 07:11:18.213865
#24 42.29 Selecting previously unselected package libxml2:amd64.
2026-Feb-25 07:11:18.213865
#24 42.29 Preparing to unpack .../013-libxml2_2.9.13+dfsg-1ubuntu0.11_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 42.29 Unpacking libxml2:amd64 (2.9.13+dfsg-1ubuntu0.11) ...
2026-Feb-25 07:11:18.213865
#24 42.31 Selecting previously unselected package shared-mime-info.
2026-Feb-25 07:11:18.213865
#24 42.31 Preparing to unpack .../014-shared-mime-info_2.1-2_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 42.31 Unpacking shared-mime-info (2.1-2) ...
2026-Feb-25 07:11:18.213865
#24 42.34 Selecting previously unselected package ucf.
2026-Feb-25 07:11:18.213865
#24 42.34 Preparing to unpack .../015-ucf_3.0043_all.deb ...
2026-Feb-25 07:11:18.213865
#24 42.34 Moving old data out of the way
2026-Feb-25 07:11:18.213865
#24 42.34 Unpacking ucf (3.0043) ...
2026-Feb-25 07:11:18.213865
#24 42.36 Selecting previously unselected package xdg-user-dirs.
2026-Feb-25 07:11:18.213865
#24 42.37 Preparing to unpack .../016-xdg-user-dirs_0.17-2ubuntu4_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 42.37 Unpacking xdg-user-dirs (0.17-2ubuntu4) ...
2026-Feb-25 07:11:18.213865
#24 42.42 Selecting previously unselected package xkb-data.
2026-Feb-25 07:11:18.213865
#24 42.42 Preparing to unpack .../017-xkb-data_2.33-1_all.deb ...
2026-Feb-25 07:11:18.213865
#24 42.42 Unpacking xkb-data (2.33-1) ...
2026-Feb-25 07:11:18.213865
#24 42.48 Selecting previously unselected package libdrm-common.
2026-Feb-25 07:11:18.213865
#24 42.49 Preparing to unpack .../018-libdrm-common_2.4.113-2~ubuntu0.22.04.1_all.deb ...
2026-Feb-25 07:11:18.213865
#24 42.49 Unpacking libdrm-common (2.4.113-2~ubuntu0.22.04.1) ...
2026-Feb-25 07:11:18.213865
#24 42.50 Selecting previously unselected package libdrm2:amd64.
2026-Feb-25 07:11:18.213865
#24 42.50 Preparing to unpack .../019-libdrm2_2.4.113-2~ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 42.51 Unpacking libdrm2:amd64 (2.4.113-2~ubuntu0.22.04.1) ...
2026-Feb-25 07:11:18.213865
#24 42.52 Selecting previously unselected package libedit2:amd64.
2026-Feb-25 07:11:18.213865
#24 42.52 Preparing to unpack .../020-libedit2_3.1-20210910-1build1_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 42.52 Unpacking libedit2:amd64 (3.1-20210910-1build1) ...
2026-Feb-25 07:11:18.213865
#24 42.54 Selecting previously unselected package libnuma1:amd64.
2026-Feb-25 07:11:18.213865
#24 42.54 Preparing to unpack .../021-libnuma1_2.0.14-3ubuntu2_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 42.54 Unpacking libnuma1:amd64 (2.0.14-3ubuntu2) ...
2026-Feb-25 07:11:18.213865
#24 42.56 Selecting previously unselected package libpng16-16:amd64.
2026-Feb-25 07:11:18.213865
#24 42.56 Preparing to unpack .../022-libpng16-16_1.6.37-3ubuntu0.4_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 42.56 Unpacking libpng16-16:amd64 (1.6.37-3ubuntu0.4) ...
2026-Feb-25 07:11:18.213865
#24 42.58 Selecting previously unselected package libusb-1.0-0:amd64.
2026-Feb-25 07:11:18.213865
#24 42.58 Preparing to unpack .../023-libusb-1.0-0_2%3a1.0.25-1ubuntu2_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 42.58 Unpacking libusb-1.0-0:amd64 (2:1.0.25-1ubuntu2) ...
2026-Feb-25 07:11:18.213865
#24 42.60 Selecting previously unselected package libxau6:amd64.
2026-Feb-25 07:11:18.213865
#24 42.60 Preparing to unpack .../024-libxau6_1%3a1.0.9-1build5_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 42.60 Unpacking libxau6:amd64 (1:1.0.9-1build5) ...
2026-Feb-25 07:11:18.213865
#24 42.61 Selecting previously unselected package libxdmcp6:amd64.
2026-Feb-25 07:11:18.213865
#24 42.62 Preparing to unpack .../025-libxdmcp6_1%3a1.1.3-0ubuntu5_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 42.62 Unpacking libxdmcp6:amd64 (1:1.1.3-0ubuntu5) ...
2026-Feb-25 07:11:18.213865
#24 42.63 Selecting previously unselected package libxcb1:amd64.
2026-Feb-25 07:11:18.213865
#24 42.63 Preparing to unpack .../026-libxcb1_1.14-3ubuntu3_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 42.64 Unpacking libxcb1:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:11:18.213865
#24 42.65 Selecting previously unselected package libx11-data.
2026-Feb-25 07:11:18.213865
#24 42.65 Preparing to unpack .../027-libx11-data_2%3a1.7.5-1ubuntu0.3_all.deb ...
2026-Feb-25 07:11:18.213865
#24 42.65 Unpacking libx11-data (2:1.7.5-1ubuntu0.3) ...
2026-Feb-25 07:11:18.213865
#24 42.70 Selecting previously unselected package libx11-6:amd64.
2026-Feb-25 07:11:18.213865
#24 42.70 Preparing to unpack .../028-libx11-6_2%3a1.7.5-1ubuntu0.3_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 42.70 Unpacking libx11-6:amd64 (2:1.7.5-1ubuntu0.3) ...
2026-Feb-25 07:11:18.213865
#24 42.72 Selecting previously unselected package libxext6:amd64.
2026-Feb-25 07:11:18.213865
#24 42.72 Preparing to unpack .../029-libxext6_2%3a1.3.4-1build1_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 42.73 Unpacking libxext6:amd64 (2:1.3.4-1build1) ...
2026-Feb-25 07:11:18.213865
#24 42.74 Selecting previously unselected package alsa-topology-conf.
2026-Feb-25 07:11:18.213865
#24 42.74 Preparing to unpack .../030-alsa-topology-conf_1.2.5.1-2_all.deb ...
2026-Feb-25 07:11:18.213865
#24 42.74 Unpacking alsa-topology-conf (1.2.5.1-2) ...
2026-Feb-25 07:11:18.213865
#24 42.76 Selecting previously unselected package libasound2-data.
2026-Feb-25 07:11:18.213865
#24 42.76 Preparing to unpack .../031-libasound2-data_1.2.6.1-1ubuntu1.1_all.deb ...
2026-Feb-25 07:11:18.213865
#24 42.76 Unpacking libasound2-data (1.2.6.1-1ubuntu1.1) ...
2026-Feb-25 07:11:18.213865
#24 42.79 Selecting previously unselected package libasound2:amd64.
2026-Feb-25 07:11:18.213865
#24 42.79 Preparing to unpack .../032-libasound2_1.2.6.1-1ubuntu1.1_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 42.79 Unpacking libasound2:amd64 (1.2.6.1-1ubuntu1.1) ...
2026-Feb-25 07:11:18.213865
#24 42.81 Selecting previously unselected package alsa-ucm-conf.
2026-Feb-25 07:11:18.213865
#24 42.81 Preparing to unpack .../033-alsa-ucm-conf_1.2.6.3-1ubuntu1.12_all.deb ...
2026-Feb-25 07:11:18.213865
#24 42.81 Unpacking alsa-ucm-conf (1.2.6.3-1ubuntu1.12) ...
2026-Feb-25 07:11:18.213865
#24 42.88 Selecting previously unselected package libaom3:amd64.
2026-Feb-25 07:11:18.213865
#24 42.89 Preparing to unpack .../034-libaom3_3.3.0-1ubuntu0.1_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 42.89 Unpacking libaom3:amd64 (3.3.0-1ubuntu0.1) ...
2026-Feb-25 07:11:18.213865
#24 42.92 Selecting previously unselected package libva2:amd64.
2026-Feb-25 07:11:18.213865
#24 42.92 Preparing to unpack .../035-libva2_2.14.0-1_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 42.92 Unpacking libva2:amd64 (2.14.0-1) ...
2026-Feb-25 07:11:18.213865
#24 42.94 Selecting previously unselected package libmfx1:amd64.
2026-Feb-25 07:11:18.213865
#24 42.94 Preparing to unpack .../036-libmfx1_22.3.0-1_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 42.94 Unpacking libmfx1:amd64 (22.3.0-1) ...
2026-Feb-25 07:11:18.213865
#24 43.02 Selecting previously unselected package libva-drm2:amd64.
2026-Feb-25 07:11:18.213865
#24 43.02 Preparing to unpack .../037-libva-drm2_2.14.0-1_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 43.02 Unpacking libva-drm2:amd64 (2.14.0-1) ...
2026-Feb-25 07:11:18.213865
#24 43.04 Selecting previously unselected package libxfixes3:amd64.
2026-Feb-25 07:11:18.213865
#24 43.04 Preparing to unpack .../038-libxfixes3_1%3a6.0.0-1_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 43.04 Unpacking libxfixes3:amd64 (1:6.0.0-1) ...
2026-Feb-25 07:11:18.213865
#24 43.06 Selecting previously unselected package libva-x11-2:amd64.
2026-Feb-25 07:11:18.213865
#24 43.06 Preparing to unpack .../039-libva-x11-2_2.14.0-1_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 43.06 Unpacking libva-x11-2:amd64 (2.14.0-1) ...
2026-Feb-25 07:11:18.213865
#24 43.08 Selecting previously unselected package libvdpau1:amd64.
2026-Feb-25 07:11:18.213865
#24 43.08 Preparing to unpack .../040-libvdpau1_1.4-3build2_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 43.08 Unpacking libvdpau1:amd64 (1.4-3build2) ...
2026-Feb-25 07:11:18.213865
#24 43.16 Selecting previously unselected package ocl-icd-libopencl1:amd64.
2026-Feb-25 07:11:18.213865
#24 43.16 Preparing to unpack .../041-ocl-icd-libopencl1_2.2.14-3_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 43.16 Unpacking ocl-icd-libopencl1:amd64 (2.2.14-3) ...
2026-Feb-25 07:11:18.213865
#24 43.18 Selecting previously unselected package libavutil56:amd64.
2026-Feb-25 07:11:18.213865
#24 43.18 Preparing to unpack .../042-libavutil56_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 43.18 Unpacking libavutil56:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:11:18.213865
#24 43.20 Selecting previously unselected package libbrotli1:amd64.
2026-Feb-25 07:11:18.213865
#24 43.20 Preparing to unpack .../043-libbrotli1_1.0.9-2build6_amd64.deb ...
2026-Feb-25 07:11:18.213865
#24 43.20 Unpacking libbrotli1:amd64 (1.0.9-2build6) ...
2026-Feb-25 07:11:18.221543
#24 43.22 Selecting previously unselected package libfreetype6:amd64.
2026-Feb-25 07:11:18.221543
#24 43.22 Preparing to unpack .../044-libfreetype6_2.11.1+dfsg-1ubuntu0.3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.22 Unpacking libfreetype6:amd64 (2.11.1+dfsg-1ubuntu0.3) ...
2026-Feb-25 07:11:18.221543
#24 43.24 Selecting previously unselected package fonts-dejavu-core.
2026-Feb-25 07:11:18.221543
#24 43.24 Preparing to unpack .../045-fonts-dejavu-core_2.37-2build1_all.deb ...
2026-Feb-25 07:11:18.221543
#24 43.24 Unpacking fonts-dejavu-core (2.37-2build1) ...
2026-Feb-25 07:11:18.221543
#24 43.32 Selecting previously unselected package fontconfig-config.
2026-Feb-25 07:11:18.221543
#24 43.32 Preparing to unpack .../046-fontconfig-config_2.13.1-4.2ubuntu5_all.deb ...
2026-Feb-25 07:11:18.221543
#24 43.32 Unpacking fontconfig-config (2.13.1-4.2ubuntu5) ...
2026-Feb-25 07:11:18.221543
#24 43.35 Selecting previously unselected package libfontconfig1:amd64.
2026-Feb-25 07:11:18.221543
#24 43.35 Preparing to unpack .../047-libfontconfig1_2.13.1-4.2ubuntu5_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.35 Unpacking libfontconfig1:amd64 (2.13.1-4.2ubuntu5) ...
2026-Feb-25 07:11:18.221543
#24 43.37 Selecting previously unselected package libpixman-1-0:amd64.
2026-Feb-25 07:11:18.221543
#24 43.37 Preparing to unpack .../048-libpixman-1-0_0.40.0-1ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.37 Unpacking libpixman-1-0:amd64 (0.40.0-1ubuntu0.22.04.1) ...
2026-Feb-25 07:11:18.221543
#24 43.39 Selecting previously unselected package libxcb-render0:amd64.
2026-Feb-25 07:11:18.221543
#24 43.39 Preparing to unpack .../049-libxcb-render0_1.14-3ubuntu3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.39 Unpacking libxcb-render0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:11:18.221543
#24 43.41 Selecting previously unselected package libxcb-shm0:amd64.
2026-Feb-25 07:11:18.221543
#24 43.41 Preparing to unpack .../050-libxcb-shm0_1.14-3ubuntu3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.41 Unpacking libxcb-shm0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:11:18.221543
#24 43.43 Selecting previously unselected package libxrender1:amd64.
2026-Feb-25 07:11:18.221543
#24 43.43 Preparing to unpack .../051-libxrender1_1%3a0.9.10-1build4_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.43 Unpacking libxrender1:amd64 (1:0.9.10-1build4) ...
2026-Feb-25 07:11:18.221543
#24 43.45 Selecting previously unselected package libcairo2:amd64.
2026-Feb-25 07:11:18.221543
#24 43.45 Preparing to unpack .../052-libcairo2_1.16.0-5ubuntu2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.45 Unpacking libcairo2:amd64 (1.16.0-5ubuntu2) ...
2026-Feb-25 07:11:18.221543
#24 43.47 Selecting previously unselected package libcodec2-1.0:amd64.
2026-Feb-25 07:11:18.221543
#24 43.47 Preparing to unpack .../053-libcodec2-1.0_1.0.1-3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.47 Unpacking libcodec2-1.0:amd64 (1.0.1-3) ...
2026-Feb-25 07:11:18.221543
#24 43.52 Selecting previously unselected package libdav1d5:amd64.
2026-Feb-25 07:11:18.221543
#24 43.52 Preparing to unpack .../054-libdav1d5_0.9.2-1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.53 Unpacking libdav1d5:amd64 (0.9.2-1) ...
2026-Feb-25 07:11:18.221543
#24 43.55 Selecting previously unselected package libgsm1:amd64.
2026-Feb-25 07:11:18.221543
#24 43.55 Preparing to unpack .../055-libgsm1_1.0.19-1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.55 Unpacking libgsm1:amd64 (1.0.19-1) ...
2026-Feb-25 07:11:18.221543
#24 43.56 Selecting previously unselected package libmp3lame0:amd64.
2026-Feb-25 07:11:18.221543
#24 43.56 Preparing to unpack .../056-libmp3lame0_3.100-3build2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.57 Unpacking libmp3lame0:amd64 (3.100-3build2) ...
2026-Feb-25 07:11:18.221543
#24 43.58 Selecting previously unselected package libopenjp2-7:amd64.
2026-Feb-25 07:11:18.221543
#24 43.58 Preparing to unpack .../057-libopenjp2-7_2.4.0-6ubuntu0.4_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.59 Unpacking libopenjp2-7:amd64 (2.4.0-6ubuntu0.4) ...
2026-Feb-25 07:11:18.221543
#24 43.60 Selecting previously unselected package libopus0:amd64.
2026-Feb-25 07:11:18.221543
#24 43.60 Preparing to unpack .../058-libopus0_1.3.1-0.1build2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.60 Unpacking libopus0:amd64 (1.3.1-0.1build2) ...
2026-Feb-25 07:11:18.221543
#24 43.62 Selecting previously unselected package libcairo-gobject2:amd64.
2026-Feb-25 07:11:18.221543
#24 43.62 Preparing to unpack .../059-libcairo-gobject2_1.16.0-5ubuntu2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.62 Unpacking libcairo-gobject2:amd64 (1.16.0-5ubuntu2) ...
2026-Feb-25 07:11:18.221543
#24 43.64 Selecting previously unselected package libgdk-pixbuf2.0-common.
2026-Feb-25 07:11:18.221543
#24 43.64 Preparing to unpack .../060-libgdk-pixbuf2.0-common_2.42.8+dfsg-1ubuntu0.4_all.deb ...
2026-Feb-25 07:11:18.221543
#24 43.64 Unpacking libgdk-pixbuf2.0-common (2.42.8+dfsg-1ubuntu0.4) ...
2026-Feb-25 07:11:18.221543
#24 43.65 Selecting previously unselected package libjpeg-turbo8:amd64.
2026-Feb-25 07:11:18.221543
#24 43.66 Preparing to unpack .../061-libjpeg-turbo8_2.1.2-0ubuntu1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.66 Unpacking libjpeg-turbo8:amd64 (2.1.2-0ubuntu1) ...
2026-Feb-25 07:11:18.221543
#24 43.67 Selecting previously unselected package libjpeg8:amd64.
2026-Feb-25 07:11:18.221543
#24 43.67 Preparing to unpack .../062-libjpeg8_8c-2ubuntu10_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.68 Unpacking libjpeg8:amd64 (8c-2ubuntu10) ...
2026-Feb-25 07:11:18.221543
#24 43.69 Selecting previously unselected package libdeflate0:amd64.
2026-Feb-25 07:11:18.221543
#24 43.69 Preparing to unpack .../063-libdeflate0_1.10-2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.69 Unpacking libdeflate0:amd64 (1.10-2) ...
2026-Feb-25 07:11:18.221543
#24 43.71 Selecting previously unselected package libjbig0:amd64.
2026-Feb-25 07:11:18.221543
#24 43.71 Preparing to unpack .../064-libjbig0_2.1-3.1ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.71 Unpacking libjbig0:amd64 (2.1-3.1ubuntu0.22.04.1) ...
2026-Feb-25 07:11:18.221543
#24 43.72 Selecting previously unselected package libwebp7:amd64.
2026-Feb-25 07:11:18.221543
#24 43.73 Preparing to unpack .../065-libwebp7_1.2.2-2ubuntu0.22.04.2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.73 Unpacking libwebp7:amd64 (1.2.2-2ubuntu0.22.04.2) ...
2026-Feb-25 07:11:18.221543
#24 43.74 Selecting previously unselected package libtiff5:amd64.
2026-Feb-25 07:11:18.221543
#24 43.74 Preparing to unpack .../066-libtiff5_4.3.0-6ubuntu0.12_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.75 Unpacking libtiff5:amd64 (4.3.0-6ubuntu0.12) ...
2026-Feb-25 07:11:18.221543
#24 43.77 Selecting previously unselected package libgdk-pixbuf-2.0-0:amd64.
2026-Feb-25 07:11:18.221543
#24 43.77 Preparing to unpack .../067-libgdk-pixbuf-2.0-0_2.42.8+dfsg-1ubuntu0.4_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.77 Unpacking libgdk-pixbuf-2.0-0:amd64 (2.42.8+dfsg-1ubuntu0.4) ...
2026-Feb-25 07:11:18.221543
#24 43.79 Selecting previously unselected package fontconfig.
2026-Feb-25 07:11:18.221543
#24 43.79 Preparing to unpack .../068-fontconfig_2.13.1-4.2ubuntu5_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.79 Unpacking fontconfig (2.13.1-4.2ubuntu5) ...
2026-Feb-25 07:11:18.221543
#24 43.81 Selecting previously unselected package libgraphite2-3:amd64.
2026-Feb-25 07:11:18.221543
#24 43.81 Preparing to unpack .../069-libgraphite2-3_1.3.14-1build2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.81 Unpacking libgraphite2-3:amd64 (1.3.14-1build2) ...
2026-Feb-25 07:11:18.221543
#24 43.83 Selecting previously unselected package libharfbuzz0b:amd64.
2026-Feb-25 07:11:18.221543
#24 43.83 Preparing to unpack .../070-libharfbuzz0b_2.7.4-1ubuntu3.2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.83 Unpacking libharfbuzz0b:amd64 (2.7.4-1ubuntu3.2) ...
2026-Feb-25 07:11:18.221543
#24 43.85 Selecting previously unselected package libthai-data.
2026-Feb-25 07:11:18.221543
#24 43.85 Preparing to unpack .../071-libthai-data_0.1.29-1build1_all.deb ...
2026-Feb-25 07:11:18.221543
#24 43.85 Unpacking libthai-data (0.1.29-1build1) ...
2026-Feb-25 07:11:18.221543
#24 43.87 Selecting previously unselected package libdatrie1:amd64.
2026-Feb-25 07:11:18.221543
#24 43.87 Preparing to unpack .../072-libdatrie1_0.2.13-2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.87 Unpacking libdatrie1:amd64 (0.2.13-2) ...
2026-Feb-25 07:11:18.221543
#24 43.88 Selecting previously unselected package libthai0:amd64.
2026-Feb-25 07:11:18.221543
#24 43.89 Preparing to unpack .../073-libthai0_0.1.29-1build1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.89 Unpacking libthai0:amd64 (0.1.29-1build1) ...
2026-Feb-25 07:11:18.221543
#24 43.90 Selecting previously unselected package libpango-1.0-0:amd64.
2026-Feb-25 07:11:18.221543
#24 43.90 Preparing to unpack .../074-libpango-1.0-0_1.50.6+ds-2ubuntu1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.90 Unpacking libpango-1.0-0:amd64 (1.50.6+ds-2ubuntu1) ...
2026-Feb-25 07:11:18.221543
#24 43.92 Selecting previously unselected package libpangoft2-1.0-0:amd64.
2026-Feb-25 07:11:18.221543
#24 43.92 Preparing to unpack .../075-libpangoft2-1.0-0_1.50.6+ds-2ubuntu1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.92 Unpacking libpangoft2-1.0-0:amd64 (1.50.6+ds-2ubuntu1) ...
2026-Feb-25 07:11:18.221543
#24 43.94 Selecting previously unselected package libpangocairo-1.0-0:amd64.
2026-Feb-25 07:11:18.221543
#24 43.94 Preparing to unpack .../076-libpangocairo-1.0-0_1.50.6+ds-2ubuntu1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.94 Unpacking libpangocairo-1.0-0:amd64 (1.50.6+ds-2ubuntu1) ...
2026-Feb-25 07:11:18.221543
#24 43.96 Selecting previously unselected package librsvg2-2:amd64.
2026-Feb-25 07:11:18.221543
#24 43.96 Preparing to unpack .../077-librsvg2-2_2.52.5+dfsg-3ubuntu0.2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 43.96 Unpacking librsvg2-2:amd64 (2.52.5+dfsg-3ubuntu0.2) ...
2026-Feb-25 07:11:18.221543
#24 44.01 Selecting previously unselected package libshine3:amd64.
2026-Feb-25 07:11:18.221543
#24 44.01 Preparing to unpack .../078-libshine3_3.1.1-2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.01 Unpacking libshine3:amd64 (3.1.1-2) ...
2026-Feb-25 07:11:18.221543
#24 44.03 Selecting previously unselected package libsnappy1v5:amd64.
2026-Feb-25 07:11:18.221543
#24 44.03 Preparing to unpack .../079-libsnappy1v5_1.1.8-1build3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.03 Unpacking libsnappy1v5:amd64 (1.1.8-1build3) ...
2026-Feb-25 07:11:18.221543
#24 44.05 Selecting previously unselected package libspeex1:amd64.
2026-Feb-25 07:11:18.221543
#24 44.05 Preparing to unpack .../080-libspeex1_1.2~rc1.2-1.1ubuntu3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.05 Unpacking libspeex1:amd64 (1.2~rc1.2-1.1ubuntu3) ...
2026-Feb-25 07:11:18.221543
#24 44.07 Selecting previously unselected package libgomp1:amd64.
2026-Feb-25 07:11:18.221543
#24 44.07 Preparing to unpack .../081-libgomp1_12.3.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.07 Unpacking libgomp1:amd64 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:11:18.221543
#24 44.09 Selecting previously unselected package libsoxr0:amd64.
2026-Feb-25 07:11:18.221543
#24 44.09 Preparing to unpack .../082-libsoxr0_0.1.3-4build2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.09 Unpacking libsoxr0:amd64 (0.1.3-4build2) ...
2026-Feb-25 07:11:18.221543
#24 44.12 Selecting previously unselected package libswresample3:amd64.
2026-Feb-25 07:11:18.221543
#24 44.12 Preparing to unpack .../083-libswresample3_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.12 Unpacking libswresample3:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:11:18.221543
#24 44.14 Selecting previously unselected package libogg0:amd64.
2026-Feb-25 07:11:18.221543
#24 44.14 Preparing to unpack .../084-libogg0_1.3.5-0ubuntu3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.14 Unpacking libogg0:amd64 (1.3.5-0ubuntu3) ...
2026-Feb-25 07:11:18.221543
#24 44.16 Selecting previously unselected package libtheora0:amd64.
2026-Feb-25 07:11:18.221543
#24 44.16 Preparing to unpack .../085-libtheora0_1.1.1+dfsg.1-15ubuntu4_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.16 Unpacking libtheora0:amd64 (1.1.1+dfsg.1-15ubuntu4) ...
2026-Feb-25 07:11:18.221543
#24 44.18 Selecting previously unselected package libtwolame0:amd64.
2026-Feb-25 07:11:18.221543
#24 44.18 Preparing to unpack .../086-libtwolame0_0.4.0-2build2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.18 Unpacking libtwolame0:amd64 (0.4.0-2build2) ...
2026-Feb-25 07:11:18.221543
#24 44.20 Selecting previously unselected package libvorbis0a:amd64.
2026-Feb-25 07:11:18.221543
#24 44.20 Preparing to unpack .../087-libvorbis0a_1.3.7-1build2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.20 Unpacking libvorbis0a:amd64 (1.3.7-1build2) ...
2026-Feb-25 07:11:18.221543
#24 44.22 Selecting previously unselected package libvorbisenc2:amd64.
2026-Feb-25 07:11:18.221543
#24 44.22 Preparing to unpack .../088-libvorbisenc2_1.3.7-1build2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.22 Unpacking libvorbisenc2:amd64 (1.3.7-1build2) ...
2026-Feb-25 07:11:18.221543
#24 44.24 Selecting previously unselected package libvpx7:amd64.
2026-Feb-25 07:11:18.221543
#24 44.24 Preparing to unpack .../089-libvpx7_1.11.0-2ubuntu2.5_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.24 Unpacking libvpx7:amd64 (1.11.0-2ubuntu2.5) ...
2026-Feb-25 07:11:18.221543
#24 44.27 Selecting previously unselected package libwebpmux3:amd64.
2026-Feb-25 07:11:18.221543
#24 44.27 Preparing to unpack .../090-libwebpmux3_1.2.2-2ubuntu0.22.04.2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.27 Unpacking libwebpmux3:amd64 (1.2.2-2ubuntu0.22.04.2) ...
2026-Feb-25 07:11:18.221543
#24 44.29 Selecting previously unselected package libx264-163:amd64.
2026-Feb-25 07:11:18.221543
#24 44.29 Preparing to unpack .../091-libx264-163_2%3a0.163.3060+git5db6aa6-2build1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.29 Unpacking libx264-163:amd64 (2:0.163.3060+git5db6aa6-2build1) ...
2026-Feb-25 07:11:18.221543
#24 44.31 Selecting previously unselected package libx265-199:amd64.
2026-Feb-25 07:11:18.221543
#24 44.31 Preparing to unpack .../092-libx265-199_3.5-2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.31 Unpacking libx265-199:amd64 (3.5-2) ...
2026-Feb-25 07:11:18.221543
#24 44.36 Selecting previously unselected package libxvidcore4:amd64.
2026-Feb-25 07:11:18.221543
#24 44.37 Preparing to unpack .../093-libxvidcore4_2%3a1.3.7-1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.37 Unpacking libxvidcore4:amd64 (2:1.3.7-1) ...
2026-Feb-25 07:11:18.221543
#24 44.39 Selecting previously unselected package libzvbi-common.
2026-Feb-25 07:11:18.221543
#24 44.39 Preparing to unpack .../094-libzvbi-common_0.2.35-19_all.deb ...
2026-Feb-25 07:11:18.221543
#24 44.40 Unpacking libzvbi-common (0.2.35-19) ...
2026-Feb-25 07:11:18.221543
#24 44.41 Selecting previously unselected package libzvbi0:amd64.
2026-Feb-25 07:11:18.221543
#24 44.41 Preparing to unpack .../095-libzvbi0_0.2.35-19_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.42 Unpacking libzvbi0:amd64 (0.2.35-19) ...
2026-Feb-25 07:11:18.221543
#24 44.43 Selecting previously unselected package libavcodec58:amd64.
2026-Feb-25 07:11:18.221543
#24 44.44 Preparing to unpack .../096-libavcodec58_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.44 Unpacking libavcodec58:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:11:18.221543
#24 44.50 Selecting previously unselected package libraw1394-11:amd64.
2026-Feb-25 07:11:18.221543
#24 44.50 Preparing to unpack .../097-libraw1394-11_2.1.2-2build2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.51 Unpacking libraw1394-11:amd64 (2.1.2-2build2) ...
2026-Feb-25 07:11:18.221543
#24 44.52 Selecting previously unselected package libavc1394-0:amd64.
2026-Feb-25 07:11:18.221543
#24 44.52 Preparing to unpack .../098-libavc1394-0_0.5.4-5build2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.52 Unpacking libavc1394-0:amd64 (0.5.4-5build2) ...
2026-Feb-25 07:11:18.221543
#24 44.54 Selecting previously unselected package libass9:amd64.
2026-Feb-25 07:11:18.221543
#24 44.54 Preparing to unpack .../099-libass9_1%3a0.15.2-1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.54 Unpacking libass9:amd64 (1:0.15.2-1) ...
2026-Feb-25 07:11:18.221543
#24 44.56 Selecting previously unselected package libudfread0:amd64.
2026-Feb-25 07:11:18.221543
#24 44.56 Preparing to unpack .../100-libudfread0_1.1.2-1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.56 Unpacking libudfread0:amd64 (1.1.2-1) ...
2026-Feb-25 07:11:18.221543
#24 44.58 Selecting previously unselected package libbluray2:amd64.
2026-Feb-25 07:11:18.221543
#24 44.58 Preparing to unpack .../101-libbluray2_1%3a1.3.1-1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.58 Unpacking libbluray2:amd64 (1:1.3.1-1) ...
2026-Feb-25 07:11:18.221543
#24 44.60 Selecting previously unselected package libchromaprint1:amd64.
2026-Feb-25 07:11:18.221543
#24 44.60 Preparing to unpack .../102-libchromaprint1_1.5.1-2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.60 Unpacking libchromaprint1:amd64 (1.5.1-2) ...
2026-Feb-25 07:11:18.221543
#24 44.62 Selecting previously unselected package libgme0:amd64.
2026-Feb-25 07:11:18.221543
#24 44.62 Preparing to unpack .../103-libgme0_0.6.3-2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.62 Unpacking libgme0:amd64 (0.6.3-2) ...
2026-Feb-25 07:11:18.221543
#24 44.64 Selecting previously unselected package libmpg123-0:amd64.
2026-Feb-25 07:11:18.221543
#24 44.64 Preparing to unpack .../104-libmpg123-0_1.29.3-1ubuntu0.1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.65 Unpacking libmpg123-0:amd64 (1.29.3-1ubuntu0.1) ...
2026-Feb-25 07:11:18.221543
#24 44.66 Selecting previously unselected package libvorbisfile3:amd64.
2026-Feb-25 07:11:18.221543
#24 44.66 Preparing to unpack .../105-libvorbisfile3_1.3.7-1build2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.67 Unpacking libvorbisfile3:amd64 (1.3.7-1build2) ...
2026-Feb-25 07:11:18.221543
#24 44.68 Selecting previously unselected package libopenmpt0:amd64.
2026-Feb-25 07:11:18.221543
#24 44.68 Preparing to unpack .../106-libopenmpt0_0.6.1-1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.68 Unpacking libopenmpt0:amd64 (0.6.1-1) ...
2026-Feb-25 07:11:18.221543
#24 44.71 Selecting previously unselected package librabbitmq4:amd64.
2026-Feb-25 07:11:18.221543
#24 44.71 Preparing to unpack .../107-librabbitmq4_0.10.0-1ubuntu2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.71 Unpacking librabbitmq4:amd64 (0.10.0-1ubuntu2) ...
2026-Feb-25 07:11:18.221543
#24 44.73 Selecting previously unselected package libsrt1.4-gnutls:amd64.
2026-Feb-25 07:11:18.221543
#24 44.73 Preparing to unpack .../108-libsrt1.4-gnutls_1.4.4-4_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.73 Unpacking libsrt1.4-gnutls:amd64 (1.4.4-4) ...
2026-Feb-25 07:11:18.221543
#24 44.75 Selecting previously unselected package libssh-gcrypt-4:amd64.
2026-Feb-25 07:11:18.221543
#24 44.75 Preparing to unpack .../109-libssh-gcrypt-4_0.9.6-2ubuntu0.22.04.6_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.75 Unpacking libssh-gcrypt-4:amd64 (0.9.6-2ubuntu0.22.04.6) ...
2026-Feb-25 07:11:18.221543
#24 44.77 Selecting previously unselected package libnorm1:amd64.
2026-Feb-25 07:11:18.221543
#24 44.77 Preparing to unpack .../110-libnorm1_1.5.9+dfsg-2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.77 Unpacking libnorm1:amd64 (1.5.9+dfsg-2) ...
2026-Feb-25 07:11:18.221543
#24 44.80 Selecting previously unselected package libpgm-5.3-0:amd64.
2026-Feb-25 07:11:18.221543
#24 44.80 Preparing to unpack .../111-libpgm-5.3-0_5.3.128~dfsg-2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.80 Unpacking libpgm-5.3-0:amd64 (5.3.128~dfsg-2) ...
2026-Feb-25 07:11:18.221543
#24 44.83 Selecting previously unselected package libsodium23:amd64.
2026-Feb-25 07:11:18.221543
#24 44.83 Preparing to unpack .../112-libsodium23_1.0.18-1ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.83 Unpacking libsodium23:amd64 (1.0.18-1ubuntu0.22.04.1) ...
2026-Feb-25 07:11:18.221543
#24 44.85 Selecting previously unselected package libzmq5:amd64.
2026-Feb-25 07:11:18.221543
#24 44.85 Preparing to unpack .../113-libzmq5_4.3.4-2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.85 Unpacking libzmq5:amd64 (4.3.4-2) ...
2026-Feb-25 07:11:18.221543
#24 44.87 Selecting previously unselected package libavformat58:amd64.
2026-Feb-25 07:11:18.221543
#24 44.87 Preparing to unpack .../114-libavformat58_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.88 Unpacking libavformat58:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:11:18.221543
#24 44.90 Selecting previously unselected package libbs2b0:amd64.
2026-Feb-25 07:11:18.221543
#24 44.90 Preparing to unpack .../115-libbs2b0_3.1.0+dfsg-2.2build1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 44.90 Unpacking libbs2b0:amd64 (3.1.0+dfsg-2.2build1) ...
2026-Feb-25 07:11:18.221543
#24 45.06 Selecting previously unselected package libflite1:amd64.
2026-Feb-25 07:11:18.221543
#24 45.06 Preparing to unpack .../116-libflite1_2.2-3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.06 Unpacking libflite1:amd64 (2.2-3) ...
2026-Feb-25 07:11:18.221543
#24 45.16 Selecting previously unselected package libserd-0-0:amd64.
2026-Feb-25 07:11:18.221543
#24 45.16 Preparing to unpack .../117-libserd-0-0_0.30.10-2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.16 Unpacking libserd-0-0:amd64 (0.30.10-2) ...
2026-Feb-25 07:11:18.221543
#24 45.18 Selecting previously unselected package libsord-0-0:amd64.
2026-Feb-25 07:11:18.221543
#24 45.18 Preparing to unpack .../118-libsord-0-0_0.16.8-2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.18 Unpacking libsord-0-0:amd64 (0.16.8-2) ...
2026-Feb-25 07:11:18.221543
#24 45.20 Selecting previously unselected package libsratom-0-0:amd64.
2026-Feb-25 07:11:18.221543
#24 45.20 Preparing to unpack .../119-libsratom-0-0_0.6.8-1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.20 Unpacking libsratom-0-0:amd64 (0.6.8-1) ...
2026-Feb-25 07:11:18.221543
#24 45.22 Selecting previously unselected package liblilv-0-0:amd64.
2026-Feb-25 07:11:18.221543
#24 45.22 Preparing to unpack .../120-liblilv-0-0_0.24.12-2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.22 Unpacking liblilv-0-0:amd64 (0.24.12-2) ...
2026-Feb-25 07:11:18.221543
#24 45.24 Selecting previously unselected package libmysofa1:amd64.
2026-Feb-25 07:11:18.221543
#24 45.24 Preparing to unpack .../121-libmysofa1_1.2.1~dfsg0-1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.24 Unpacking libmysofa1:amd64 (1.2.1~dfsg0-1) ...
2026-Feb-25 07:11:18.221543
#24 45.26 Selecting previously unselected package libblas3:amd64.
2026-Feb-25 07:11:18.221543
#24 45.26 Preparing to unpack .../122-libblas3_3.10.0-2ubuntu1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.27 Unpacking libblas3:amd64 (3.10.0-2ubuntu1) ...
2026-Feb-25 07:11:18.221543
#24 45.29 Selecting previously unselected package libquadmath0:amd64.
2026-Feb-25 07:11:18.221543
#24 45.29 Preparing to unpack .../123-libquadmath0_12.3.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.29 Unpacking libquadmath0:amd64 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:11:18.221543
#24 45.31 Selecting previously unselected package libgfortran5:amd64.
2026-Feb-25 07:11:18.221543
#24 45.31 Preparing to unpack .../124-libgfortran5_12.3.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.31 Unpacking libgfortran5:amd64 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:11:18.221543
#24 45.34 Selecting previously unselected package liblapack3:amd64.
2026-Feb-25 07:11:18.221543
#24 45.34 Preparing to unpack .../125-liblapack3_3.10.0-2ubuntu1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.34 Unpacking liblapack3:amd64 (3.10.0-2ubuntu1) ...
2026-Feb-25 07:11:18.221543
#24 45.38 Selecting previously unselected package libasyncns0:amd64.
2026-Feb-25 07:11:18.221543
#24 45.38 Preparing to unpack .../126-libasyncns0_0.8-6build2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.38 Unpacking libasyncns0:amd64 (0.8-6build2) ...
2026-Feb-25 07:11:18.221543
#24 45.40 Selecting previously unselected package libflac8:amd64.
2026-Feb-25 07:11:18.221543
#24 45.40 Preparing to unpack .../127-libflac8_1.3.3-2ubuntu0.2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.41 Unpacking libflac8:amd64 (1.3.3-2ubuntu0.2) ...
2026-Feb-25 07:11:18.221543
#24 45.42 Selecting previously unselected package libsndfile1:amd64.
2026-Feb-25 07:11:18.221543
#24 45.42 Preparing to unpack .../128-libsndfile1_1.0.31-2ubuntu0.2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.43 Unpacking libsndfile1:amd64 (1.0.31-2ubuntu0.2) ...
2026-Feb-25 07:11:18.221543
#24 45.44 Selecting previously unselected package libx11-xcb1:amd64.
2026-Feb-25 07:11:18.221543
#24 45.44 Preparing to unpack .../129-libx11-xcb1_2%3a1.7.5-1ubuntu0.3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.45 Unpacking libx11-xcb1:amd64 (2:1.7.5-1ubuntu0.3) ...
2026-Feb-25 07:11:18.221543
#24 45.46 Selecting previously unselected package libpulse0:amd64.
2026-Feb-25 07:11:18.221543
#24 45.46 Preparing to unpack .../130-libpulse0_1%3a15.99.1+dfsg1-1ubuntu2.2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.47 Unpacking libpulse0:amd64 (1:15.99.1+dfsg1-1ubuntu2.2) ...
2026-Feb-25 07:11:18.221543
#24 45.49 Selecting previously unselected package libsphinxbase3:amd64.
2026-Feb-25 07:11:18.221543
#24 45.49 Preparing to unpack .../131-libsphinxbase3_0.8+5prealpha+1-13build1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.49 Unpacking libsphinxbase3:amd64 (0.8+5prealpha+1-13build1) ...
2026-Feb-25 07:11:18.221543
#24 45.51 Selecting previously unselected package libpocketsphinx3:amd64.
2026-Feb-25 07:11:18.221543
#24 45.51 Preparing to unpack .../132-libpocketsphinx3_0.8.0+real5prealpha+1-14ubuntu1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.51 Unpacking libpocketsphinx3:amd64 (0.8.0+real5prealpha+1-14ubuntu1) ...
2026-Feb-25 07:11:18.221543
#24 45.53 Selecting previously unselected package libpostproc55:amd64.
2026-Feb-25 07:11:18.221543
#24 45.53 Preparing to unpack .../133-libpostproc55_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.53 Unpacking libpostproc55:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:11:18.221543
#24 45.55 Selecting previously unselected package libsamplerate0:amd64.
2026-Feb-25 07:11:18.221543
#24 45.55 Preparing to unpack .../134-libsamplerate0_0.2.2-1build1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.55 Unpacking libsamplerate0:amd64 (0.2.2-1build1) ...
2026-Feb-25 07:11:18.221543
#24 45.57 Selecting previously unselected package librubberband2:amd64.
2026-Feb-25 07:11:18.221543
#24 45.58 Preparing to unpack .../135-librubberband2_2.0.0-2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.58 Unpacking librubberband2:amd64 (2.0.0-2) ...
2026-Feb-25 07:11:18.221543
#24 45.59 Selecting previously unselected package libswscale5:amd64.
2026-Feb-25 07:11:18.221543
#24 45.60 Preparing to unpack .../136-libswscale5_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.60 Unpacking libswscale5:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:11:18.221543
#24 45.61 Selecting previously unselected package libvidstab1.1:amd64.
2026-Feb-25 07:11:18.221543
#24 45.62 Preparing to unpack .../137-libvidstab1.1_1.1.0-2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.62 Unpacking libvidstab1.1:amd64 (1.1.0-2) ...
2026-Feb-25 07:11:18.221543
#24 45.63 Selecting previously unselected package libzimg2:amd64.
2026-Feb-25 07:11:18.221543
#24 45.64 Preparing to unpack .../138-libzimg2_3.0.3+ds1-1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.64 Unpacking libzimg2:amd64 (3.0.3+ds1-1) ...
2026-Feb-25 07:11:18.221543
#24 45.66 Selecting previously unselected package libavfilter7:amd64.
2026-Feb-25 07:11:18.221543
#24 45.66 Preparing to unpack .../139-libavfilter7_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.66 Unpacking libavfilter7:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:11:18.221543
#24 45.69 Selecting previously unselected package libcaca0:amd64.
2026-Feb-25 07:11:18.221543
#24 45.69 Preparing to unpack .../140-libcaca0_0.99.beta19-2.2ubuntu4.1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.70 Unpacking libcaca0:amd64 (0.99.beta19-2.2ubuntu4.1) ...
2026-Feb-25 07:11:18.221543
#24 45.71 Selecting previously unselected package libcdio19:amd64.
2026-Feb-25 07:11:18.221543
#24 45.72 Preparing to unpack .../141-libcdio19_2.1.0-3ubuntu0.2_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.72 Unpacking libcdio19:amd64 (2.1.0-3ubuntu0.2) ...
2026-Feb-25 07:11:18.221543
#24 45.73 Selecting previously unselected package libcdio-cdda2:amd64.
2026-Feb-25 07:11:18.221543
#24 45.74 Preparing to unpack .../142-libcdio-cdda2_10.2+2.0.0-1build3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.74 Unpacking libcdio-cdda2:amd64 (10.2+2.0.0-1build3) ...
2026-Feb-25 07:11:18.221543
#24 45.75 Selecting previously unselected package libcdio-paranoia2:amd64.
2026-Feb-25 07:11:18.221543
#24 45.75 Preparing to unpack .../143-libcdio-paranoia2_10.2+2.0.0-1build3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.75 Unpacking libcdio-paranoia2:amd64 (10.2+2.0.0-1build3) ...
2026-Feb-25 07:11:18.221543
#24 45.77 Selecting previously unselected package libdc1394-25:amd64.
2026-Feb-25 07:11:18.221543
#24 45.77 Preparing to unpack .../144-libdc1394-25_2.2.6-4_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.77 Unpacking libdc1394-25:amd64 (2.2.6-4) ...
2026-Feb-25 07:11:18.221543
#24 45.79 Selecting previously unselected package libglvnd0:amd64.
2026-Feb-25 07:11:18.221543
#24 45.79 Preparing to unpack .../145-libglvnd0_1.4.0-1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.79 Unpacking libglvnd0:amd64 (1.4.0-1) ...
2026-Feb-25 07:11:18.221543
#24 45.81 Selecting previously unselected package libglapi-mesa:amd64.
2026-Feb-25 07:11:18.221543
#24 45.81 Preparing to unpack .../146-libglapi-mesa_23.2.1-1ubuntu3.1~22.04.3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.82 Unpacking libglapi-mesa:amd64 (23.2.1-1ubuntu3.1~22.04.3) ...
2026-Feb-25 07:11:18.221543
#24 45.83 Selecting previously unselected package libxcb-dri2-0:amd64.
2026-Feb-25 07:11:18.221543
#24 45.83 Preparing to unpack .../147-libxcb-dri2-0_1.14-3ubuntu3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.83 Unpacking libxcb-dri2-0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:11:18.221543
#24 45.85 Selecting previously unselected package libxcb-dri3-0:amd64.
2026-Feb-25 07:11:18.221543
#24 45.85 Preparing to unpack .../148-libxcb-dri3-0_1.14-3ubuntu3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.85 Unpacking libxcb-dri3-0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:11:18.221543
#24 45.87 Selecting previously unselected package libxcb-glx0:amd64.
2026-Feb-25 07:11:18.221543
#24 45.87 Preparing to unpack .../149-libxcb-glx0_1.14-3ubuntu3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.87 Unpacking libxcb-glx0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:11:18.221543
#24 45.89 Selecting previously unselected package libxcb-present0:amd64.
2026-Feb-25 07:11:18.221543
#24 45.89 Preparing to unpack .../150-libxcb-present0_1.14-3ubuntu3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.89 Unpacking libxcb-present0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:11:18.221543
#24 45.91 Selecting previously unselected package libxcb-randr0:amd64.
2026-Feb-25 07:11:18.221543
#24 45.91 Preparing to unpack .../151-libxcb-randr0_1.14-3ubuntu3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.91 Unpacking libxcb-randr0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:11:18.221543
#24 45.93 Selecting previously unselected package libxcb-sync1:amd64.
2026-Feb-25 07:11:18.221543
#24 45.93 Preparing to unpack .../152-libxcb-sync1_1.14-3ubuntu3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.93 Unpacking libxcb-sync1:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:11:18.221543
#24 45.95 Selecting previously unselected package libxcb-xfixes0:amd64.
2026-Feb-25 07:11:18.221543
#24 45.95 Preparing to unpack .../153-libxcb-xfixes0_1.14-3ubuntu3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.95 Unpacking libxcb-xfixes0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:11:18.221543
#24 45.96 Selecting previously unselected package libxshmfence1:amd64.
2026-Feb-25 07:11:18.221543
#24 45.97 Preparing to unpack .../154-libxshmfence1_1.3-1build4_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.97 Unpacking libxshmfence1:amd64 (1.3-1build4) ...
2026-Feb-25 07:11:18.221543
#24 45.98 Selecting previously unselected package libxxf86vm1:amd64.
2026-Feb-25 07:11:18.221543
#24 45.99 Preparing to unpack .../155-libxxf86vm1_1%3a1.1.4-1build3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 45.99 Unpacking libxxf86vm1:amd64 (1:1.1.4-1build3) ...
2026-Feb-25 07:11:18.221543
#24 46.00 Selecting previously unselected package libdrm-amdgpu1:amd64.
2026-Feb-25 07:11:18.221543
#24 46.00 Preparing to unpack .../156-libdrm-amdgpu1_2.4.113-2~ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 46.00 Unpacking libdrm-amdgpu1:amd64 (2.4.113-2~ubuntu0.22.04.1) ...
2026-Feb-25 07:11:18.221543
#24 46.02 Selecting previously unselected package libpciaccess0:amd64.
2026-Feb-25 07:11:18.221543
#24 46.02 Preparing to unpack .../157-libpciaccess0_0.16-3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 46.02 Unpacking libpciaccess0:amd64 (0.16-3) ...
2026-Feb-25 07:11:18.221543
#24 46.04 Selecting previously unselected package libdrm-intel1:amd64.
2026-Feb-25 07:11:18.221543
#24 46.04 Preparing to unpack .../158-libdrm-intel1_2.4.113-2~ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 46.04 Unpacking libdrm-intel1:amd64 (2.4.113-2~ubuntu0.22.04.1) ...
2026-Feb-25 07:11:18.221543
#24 46.06 Selecting previously unselected package libdrm-nouveau2:amd64.
2026-Feb-25 07:11:18.221543
#24 46.06 Preparing to unpack .../159-libdrm-nouveau2_2.4.113-2~ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 46.06 Unpacking libdrm-nouveau2:amd64 (2.4.113-2~ubuntu0.22.04.1) ...
2026-Feb-25 07:11:18.221543
#24 46.08 Selecting previously unselected package libdrm-radeon1:amd64.
2026-Feb-25 07:11:18.221543
#24 46.08 Preparing to unpack .../160-libdrm-radeon1_2.4.113-2~ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 46.08 Unpacking libdrm-radeon1:amd64 (2.4.113-2~ubuntu0.22.04.1) ...
2026-Feb-25 07:11:18.221543
#24 46.10 Selecting previously unselected package libllvm15:amd64.
2026-Feb-25 07:11:18.221543
#24 46.10 Preparing to unpack .../161-libllvm15_1%3a15.0.7-0ubuntu0.22.04.3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 46.10 Unpacking libllvm15:amd64 (1:15.0.7-0ubuntu0.22.04.3) ...
2026-Feb-25 07:11:18.221543
#24 46.65 Selecting previously unselected package libsensors-config.
2026-Feb-25 07:11:18.221543
#24 46.65 Preparing to unpack .../162-libsensors-config_1%3a3.6.0-7ubuntu1_all.deb ...
2026-Feb-25 07:11:18.221543
#24 46.65 Unpacking libsensors-config (1:3.6.0-7ubuntu1) ...
2026-Feb-25 07:11:18.221543
#24 46.67 Selecting previously unselected package libsensors5:amd64.
2026-Feb-25 07:11:18.221543
#24 46.67 Preparing to unpack .../163-libsensors5_1%3a3.6.0-7ubuntu1_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 46.70 Unpacking libsensors5:amd64 (1:3.6.0-7ubuntu1) ...
2026-Feb-25 07:11:18.221543
#24 46.71 Selecting previously unselected package libgl1-mesa-dri:amd64.
2026-Feb-25 07:11:18.221543
#24 46.72 Preparing to unpack .../164-libgl1-mesa-dri_23.2.1-1ubuntu3.1~22.04.3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 46.72 Unpacking libgl1-mesa-dri:amd64 (23.2.1-1ubuntu3.1~22.04.3) ...
2026-Feb-25 07:11:18.221543
#24 46.85 Selecting previously unselected package libglx-mesa0:amd64.
2026-Feb-25 07:11:18.221543
#24 46.85 Preparing to unpack .../165-libglx-mesa0_23.2.1-1ubuntu3.1~22.04.3_amd64.deb ...
2026-Feb-25 07:11:18.221543
#24 46.85 Unpacking libglx-mesa0:amd64 (23.2.1-1ubuntu3.1~22.04.3) ...
2026-Feb-25 07:11:18.221543
#24 46.87 Selecting previously unselected package libglx0:amd64.
2026-Feb-25 07:11:18.322921
#24 46.87 Preparing to unpack .../166-libglx0_1.4.0-1_amd64.deb ...
2026-Feb-25 07:11:18.322921
#24 46.88 Unpacking libglx0:amd64 (1.4.0-1) ...
2026-Feb-25 07:11:18.322921
#24 46.90 Selecting previously unselected package libgl1:amd64.
2026-Feb-25 07:11:18.322921
#24 46.90 Preparing to unpack .../167-libgl1_1.4.0-1_amd64.deb ...
2026-Feb-25 07:11:18.322921
#24 46.90 Unpacking libgl1:amd64 (1.4.0-1) ...
2026-Feb-25 07:11:18.322921
#24 46.92 Selecting previously unselected package libiec61883-0:amd64.
2026-Feb-25 07:11:18.322921
#24 46.92 Preparing to unpack .../168-libiec61883-0_1.2.0-4build3_amd64.deb ...
2026-Feb-25 07:11:18.322921
#24 46.92 Unpacking libiec61883-0:amd64 (1.2.0-4build3) ...
2026-Feb-25 07:11:18.322921
#24 46.93 Selecting previously unselected package libjack-jackd2-0:amd64.
2026-Feb-25 07:11:18.322921
#24 46.94 Preparing to unpack .../169-libjack-jackd2-0_1.9.20~dfsg-1_amd64.deb ...
2026-Feb-25 07:11:18.322921
#24 46.94 Unpacking libjack-jackd2-0:amd64 (1.9.20~dfsg-1) ...
2026-Feb-25 07:11:18.322921
#24 46.96 Selecting previously unselected package libopenal-data.
2026-Feb-25 07:11:18.322921
#24 46.96 Preparing to unpack .../170-libopenal-data_1%3a1.19.1-2build3_all.deb ...
2026-Feb-25 07:11:18.322921
#24 46.96 Unpacking libopenal-data (1:1.19.1-2build3) ...
2026-Feb-25 07:11:18.322921
#24 46.98 Selecting previously unselected package libsndio7.0:amd64.
2026-Feb-25 07:11:18.423410
#24 46.98 Preparing to unpack .../171-libsndio7.0_1.8.1-1.1_amd64.deb ...
2026-Feb-25 07:11:18.423410
#24 46.98 Unpacking libsndio7.0:amd64 (1.8.1-1.1) ...
2026-Feb-25 07:11:18.423410
#24 47.00 Selecting previously unselected package libopenal1:amd64.
2026-Feb-25 07:11:18.423410
#24 47.00 Preparing to unpack .../172-libopenal1_1%3a1.19.1-2build3_amd64.deb ...
2026-Feb-25 07:11:18.423410
#24 47.00 Unpacking libopenal1:amd64 (1:1.19.1-2build3) ...
2026-Feb-25 07:11:18.423410
#24 47.02 Selecting previously unselected package libwayland-client0:amd64.
2026-Feb-25 07:11:18.423410
#24 47.02 Preparing to unpack .../173-libwayland-client0_1.20.0-1ubuntu0.1_amd64.deb ...
2026-Feb-25 07:11:18.423410
#24 47.02 Unpacking libwayland-client0:amd64 (1.20.0-1ubuntu0.1) ...
2026-Feb-25 07:11:18.423410
#24 47.04 Selecting previously unselected package libdecor-0-0:amd64.
2026-Feb-25 07:11:18.423410
#24 47.04 Preparing to unpack .../174-libdecor-0-0_0.1.0-3build1_amd64.deb ...
2026-Feb-25 07:11:18.423410
#24 47.04 Unpacking libdecor-0-0:amd64 (0.1.0-3build1) ...
2026-Feb-25 07:11:18.423410
#24 47.06 Selecting previously unselected package libwayland-server0:amd64.
2026-Feb-25 07:11:18.423410
#24 47.06 Preparing to unpack .../175-libwayland-server0_1.20.0-1ubuntu0.1_amd64.deb ...
2026-Feb-25 07:11:18.423410
#24 47.06 Unpacking libwayland-server0:amd64 (1.20.0-1ubuntu0.1) ...
2026-Feb-25 07:11:18.423410
#24 47.08 Selecting previously unselected package libgbm1:amd64.
2026-Feb-25 07:11:18.533513
#24 47.08 Preparing to unpack .../176-libgbm1_23.2.1-1ubuntu3.1~22.04.3_amd64.deb ...
2026-Feb-25 07:11:18.533513
#24 47.08 Unpacking libgbm1:amd64 (23.2.1-1ubuntu3.1~22.04.3) ...
2026-Feb-25 07:11:18.533513
#24 47.10 Selecting previously unselected package libwayland-cursor0:amd64.
2026-Feb-25 07:11:18.533513
#24 47.10 Preparing to unpack .../177-libwayland-cursor0_1.20.0-1ubuntu0.1_amd64.deb ...
2026-Feb-25 07:11:18.533513
#24 47.10 Unpacking libwayland-cursor0:amd64 (1.20.0-1ubuntu0.1) ...
2026-Feb-25 07:11:18.533513
#24 47.12 Selecting previously unselected package libwayland-egl1:amd64.
2026-Feb-25 07:11:18.533513
#24 47.12 Preparing to unpack .../178-libwayland-egl1_1.20.0-1ubuntu0.1_amd64.deb ...
2026-Feb-25 07:11:18.533513
#24 47.12 Unpacking libwayland-egl1:amd64 (1.20.0-1ubuntu0.1) ...
2026-Feb-25 07:11:18.533513
#24 47.13 Selecting previously unselected package libxcursor1:amd64.
2026-Feb-25 07:11:18.533513
#24 47.13 Preparing to unpack .../179-libxcursor1_1%3a1.2.0-2build4_amd64.deb ...
2026-Feb-25 07:11:18.533513
#24 47.14 Unpacking libxcursor1:amd64 (1:1.2.0-2build4) ...
2026-Feb-25 07:11:18.533513
#24 47.15 Selecting previously unselected package libxi6:amd64.
2026-Feb-25 07:11:18.533513
#24 47.15 Preparing to unpack .../180-libxi6_2%3a1.8-1build1_amd64.deb ...
2026-Feb-25 07:11:18.533513
#24 47.15 Unpacking libxi6:amd64 (2:1.8-1build1) ...
2026-Feb-25 07:11:18.533513
#24 47.17 Selecting previously unselected package libxinerama1:amd64.
2026-Feb-25 07:11:18.533513
#24 47.17 Preparing to unpack .../181-libxinerama1_2%3a1.1.4-3_amd64.deb ...
2026-Feb-25 07:11:18.533513
#24 47.17 Unpacking libxinerama1:amd64 (2:1.1.4-3) ...
2026-Feb-25 07:11:18.533513
#24 47.19 Selecting previously unselected package libxkbcommon0:amd64.
2026-Feb-25 07:11:18.644231
#24 47.19 Preparing to unpack .../182-libxkbcommon0_1.4.0-1_amd64.deb ...
2026-Feb-25 07:11:18.644231
#24 47.19 Unpacking libxkbcommon0:amd64 (1.4.0-1) ...
2026-Feb-25 07:11:18.644231
#24 47.21 Selecting previously unselected package libxrandr2:amd64.
2026-Feb-25 07:11:18.644231
#24 47.21 Preparing to unpack .../183-libxrandr2_2%3a1.5.2-1build1_amd64.deb ...
2026-Feb-25 07:11:18.644231
#24 47.21 Unpacking libxrandr2:amd64 (2:1.5.2-1build1) ...
2026-Feb-25 07:11:18.644231
#24 47.23 Selecting previously unselected package x11-common.
2026-Feb-25 07:11:18.644231
#24 47.23 Preparing to unpack .../184-x11-common_1%3a7.7+23ubuntu2_all.deb ...
2026-Feb-25 07:11:18.644231
#24 47.23 Unpacking x11-common (1:7.7+23ubuntu2) ...
2026-Feb-25 07:11:18.644231
#24 47.25 Selecting previously unselected package libxss1:amd64.
2026-Feb-25 07:11:18.644231
#24 47.25 Preparing to unpack .../185-libxss1_1%3a1.2.3-1build2_amd64.deb ...
2026-Feb-25 07:11:18.644231
#24 47.25 Unpacking libxss1:amd64 (1:1.2.3-1build2) ...
2026-Feb-25 07:11:18.644231
#24 47.27 Selecting previously unselected package libsdl2-2.0-0:amd64.
2026-Feb-25 07:11:18.644231
#24 47.27 Preparing to unpack .../186-libsdl2-2.0-0_2.0.20+dfsg-2ubuntu1.22.04.1_amd64.deb ...
2026-Feb-25 07:11:18.644231
#24 47.27 Unpacking libsdl2-2.0-0:amd64 (2.0.20+dfsg-2ubuntu1.22.04.1) ...
2026-Feb-25 07:11:18.644231
#24 47.30 Selecting previously unselected package libxcb-shape0:amd64.
2026-Feb-25 07:11:18.749073
#24 47.30 Preparing to unpack .../187-libxcb-shape0_1.14-3ubuntu3_amd64.deb ...
2026-Feb-25 07:11:18.749073
#24 47.30 Unpacking libxcb-shape0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:11:18.749073
#24 47.32 Selecting previously unselected package libxv1:amd64.
2026-Feb-25 07:11:18.749073
#24 47.32 Preparing to unpack .../188-libxv1_2%3a1.0.11-1build2_amd64.deb ...
2026-Feb-25 07:11:18.749073
#24 47.32 Unpacking libxv1:amd64 (2:1.0.11-1build2) ...
2026-Feb-25 07:11:18.749073
#24 47.34 Selecting previously unselected package libavdevice58:amd64.
2026-Feb-25 07:11:18.749073
#24 47.34 Preparing to unpack .../189-libavdevice58_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:11:18.749073
#24 47.34 Unpacking libavdevice58:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:11:18.749073
#24 47.36 Selecting previously unselected package ffmpeg.
2026-Feb-25 07:11:18.749073
#24 47.36 Preparing to unpack .../190-ffmpeg_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:11:18.749073
#24 47.36 Unpacking ffmpeg (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:11:18.749073
#24 47.38 Selecting previously unselected package libigdgmm12:amd64.
2026-Feb-25 07:11:18.749073
#24 47.39 Preparing to unpack .../191-libigdgmm12_22.1.2+ds1-1_amd64.deb ...
2026-Feb-25 07:11:18.749073
#24 47.39 Unpacking libigdgmm12:amd64 (22.1.2+ds1-1) ...
2026-Feb-25 07:11:18.749073
#24 47.40 Selecting previously unselected package intel-media-va-driver:amd64.
2026-Feb-25 07:11:18.850659
#24 47.40 Preparing to unpack .../192-intel-media-va-driver_22.3.1+dfsg1-1ubuntu2_amd64.deb ...
2026-Feb-25 07:11:18.850659
#24 47.41 Unpacking intel-media-va-driver:amd64 (22.3.1+dfsg1-1ubuntu2) ...
2026-Feb-25 07:11:18.850659
#24 47.45 Selecting previously unselected package libaacs0:amd64.
2026-Feb-25 07:11:18.850659
#24 47.45 Preparing to unpack .../193-libaacs0_0.11.1-1_amd64.deb ...
2026-Feb-25 07:11:18.850659
#24 47.45 Unpacking libaacs0:amd64 (0.11.1-1) ...
2026-Feb-25 07:11:18.850659
#24 47.47 Selecting previously unselected package libbdplus0:amd64.
2026-Feb-25 07:11:18.850659
#24 47.47 Preparing to unpack .../194-libbdplus0_0.2.0-1_amd64.deb ...
2026-Feb-25 07:11:18.850659
#24 47.47 Unpacking libbdplus0:amd64 (0.2.0-1) ...
2026-Feb-25 07:11:18.850659
#24 47.49 Selecting previously unselected package libdecor-0-plugin-1-cairo:amd64.
2026-Feb-25 07:11:18.850659
#24 47.49 Preparing to unpack .../195-libdecor-0-plugin-1-cairo_0.1.0-3build1_amd64.deb ...
2026-Feb-25 07:11:18.850659
#24 47.49 Unpacking libdecor-0-plugin-1-cairo:amd64 (0.1.0-3build1) ...
2026-Feb-25 07:11:18.850659
#24 47.50 Selecting previously unselected package libgdk-pixbuf2.0-bin.
2026-Feb-25 07:11:18.954709
#24 47.51 Preparing to unpack .../196-libgdk-pixbuf2.0-bin_2.42.8+dfsg-1ubuntu0.4_amd64.deb ...
2026-Feb-25 07:11:18.954709
#24 47.51 Unpacking libgdk-pixbuf2.0-bin (2.42.8+dfsg-1ubuntu0.4) ...
2026-Feb-25 07:11:18.954709
#24 47.52 Selecting previously unselected package libgl1-amber-dri:amd64.
2026-Feb-25 07:11:18.954709
#24 47.52 Preparing to unpack .../197-libgl1-amber-dri_21.3.9-0ubuntu1~22.04.1_amd64.deb ...
2026-Feb-25 07:11:18.954709
#24 47.53 Unpacking libgl1-amber-dri:amd64 (21.3.9-0ubuntu1~22.04.1) ...
2026-Feb-25 07:11:18.954709
#24 47.59 Selecting previously unselected package librsvg2-common:amd64.
2026-Feb-25 07:11:18.954709
#24 47.59 Preparing to unpack .../198-librsvg2-common_2.52.5+dfsg-3ubuntu0.2_amd64.deb ...
2026-Feb-25 07:11:18.954709
#24 47.59 Unpacking librsvg2-common:amd64 (2.52.5+dfsg-3ubuntu0.2) ...
2026-Feb-25 07:11:18.954709
#24 47.61 Selecting previously unselected package mesa-va-drivers:amd64.
2026-Feb-25 07:11:19.080825
#24 47.61 Preparing to unpack .../199-mesa-va-drivers_23.2.1-1ubuntu3.1~22.04.3_amd64.deb ...
2026-Feb-25 07:11:19.080825
#24 47.61 Unpacking mesa-va-drivers:amd64 (23.2.1-1ubuntu3.1~22.04.3) ...
2026-Feb-25 07:11:19.080825
#24 47.67 Selecting previously unselected package mesa-vdpau-drivers:amd64.
2026-Feb-25 07:11:19.080825
#24 47.67 Preparing to unpack .../200-mesa-vdpau-drivers_23.2.1-1ubuntu3.1~22.04.3_amd64.deb ...
2026-Feb-25 07:11:19.080825
#24 47.68 Unpacking mesa-vdpau-drivers:amd64 (23.2.1-1ubuntu3.1~22.04.3) ...
2026-Feb-25 07:11:19.080825
#24 47.73 Selecting previously unselected package i965-va-driver:amd64.
2026-Feb-25 07:11:19.277326
#24 47.74 Preparing to unpack .../201-i965-va-driver_2.4.1+dfsg1-1_amd64.deb ...
2026-Feb-25 07:11:19.277326
#24 47.74 Unpacking i965-va-driver:amd64 (2.4.1+dfsg1-1) ...
2026-Feb-25 07:11:19.277326
#24 47.78 Selecting previously unselected package va-driver-all:amd64.
2026-Feb-25 07:11:19.277326
#24 47.78 Preparing to unpack .../202-va-driver-all_2.14.0-1_amd64.deb ...
2026-Feb-25 07:11:19.277326
#24 47.78 Unpacking va-driver-all:amd64 (2.14.0-1) ...
2026-Feb-25 07:11:19.277326
#24 47.79 Selecting previously unselected package vdpau-driver-all:amd64.
2026-Feb-25 07:11:19.277326
#24 47.79 Preparing to unpack .../203-vdpau-driver-all_1.4-3build2_amd64.deb ...
2026-Feb-25 07:11:19.277326
#24 47.79 Unpacking vdpau-driver-all:amd64 (1.4-3build2) ...
2026-Feb-25 07:11:19.277326
#24 47.81 Selecting previously unselected package pocketsphinx-en-us.
2026-Feb-25 07:11:19.277326
#24 47.81 Preparing to unpack .../204-pocketsphinx-en-us_0.8.0+real5prealpha+1-14ubuntu1_all.deb ...
2026-Feb-25 07:11:19.277326
#24 47.81 Unpacking pocketsphinx-en-us (0.8.0+real5prealpha+1-14ubuntu1) ...
2026-Feb-25 07:11:19.379903
#24 47.97 Setting up libgme0:amd64 (0.6.3-2) ...
2026-Feb-25 07:11:19.379903
#24 47.97 Setting up libssh-gcrypt-4:amd64 (0.9.6-2ubuntu0.22.04.6) ...
2026-Feb-25 07:11:19.379903
#24 47.98 Setting up libexpat1:amd64 (2.4.7-1ubuntu0.7) ...
2026-Feb-25 07:11:19.379903
#24 47.98 Setting up libgraphite2-3:amd64 (1.3.14-1build2) ...
2026-Feb-25 07:11:19.379903
#24 47.98 Setting up libsrt1.4-gnutls:amd64 (1.4.4-4) ...
2026-Feb-25 07:11:19.379903
#24 47.99 Setting up libpixman-1-0:amd64 (0.40.0-1ubuntu0.22.04.1) ...
2026-Feb-25 07:11:19.379903
#24 47.99 Setting up libudfread0:amd64 (1.1.2-1) ...
2026-Feb-25 07:11:19.379903
#24 47.99 Setting up libwayland-server0:amd64 (1.20.0-1ubuntu0.1) ...
2026-Feb-25 07:11:19.379903
#24 47.99 Setting up libaom3:amd64 (3.3.0-1ubuntu0.1) ...
2026-Feb-25 07:11:19.379903
#24 48.00 Setting up libpciaccess0:amd64 (0.16-3) ...
2026-Feb-25 07:11:19.379903
#24 48.00 Setting up librabbitmq4:amd64 (0.10.0-1ubuntu2) ...
2026-Feb-25 07:11:19.379903
#24 48.00 Setting up libxau6:amd64 (1:1.0.9-1build5) ...
2026-Feb-25 07:11:19.379903
#24 48.01 Setting up libraw1394-11:amd64 (2.1.2-2build2) ...
2026-Feb-25 07:11:19.379903
#24 48.01 Setting up libapparmor1:amd64 (3.0.4-2ubuntu2.5) ...
2026-Feb-25 07:11:19.379903
#24 48.01 Setting up libcodec2-1.0:amd64 (1.0.1-3) ...
2026-Feb-25 07:11:19.379903
#24 48.02 Setting up libsodium23:amd64 (1.0.18-1ubuntu0.22.04.1) ...
2026-Feb-25 07:11:19.379903
#24 48.02 Setting up libmpg123-0:amd64 (1.29.3-1ubuntu0.1) ...
2026-Feb-25 07:11:19.379903
#24 48.02 Setting up libogg0:amd64 (1.3.5-0ubuntu3) ...
2026-Feb-25 07:11:19.379903
#24 48.03 Setting up libspeex1:amd64 (1.2~rc1.2-1.1ubuntu3) ...
2026-Feb-25 07:11:19.379903
#24 48.03 Setting up libshine3:amd64 (3.1.1-2) ...
2026-Feb-25 07:11:19.379903
#24 48.03 Setting up libtwolame0:amd64 (0.4.0-2build2) ...
2026-Feb-25 07:11:19.379903
#24 48.03 Setting up libdatrie1:amd64 (0.2.13-2) ...
2026-Feb-25 07:11:19.497207
#24 48.04 Setting up xdg-user-dirs (0.17-2ubuntu4) ...
2026-Feb-25 07:11:19.497207
#24 48.05 Setting up libgsm1:amd64 (1.0.19-1) ...
2026-Feb-25 07:11:19.497207
#24 48.05 Setting up libglib2.0-0:amd64 (2.72.4-0ubuntu2.9) ...
2026-Feb-25 07:11:19.497207
#24 48.06 No schema files found: doing nothing.
2026-Feb-25 07:11:19.497207
#24 48.06 Setting up libglvnd0:amd64 (1.4.0-1) ...
2026-Feb-25 07:11:19.497207
#24 48.06 Setting up libpgm-5.3-0:amd64 (5.3.128~dfsg-2) ...
2026-Feb-25 07:11:19.497207
#24 48.07 Setting up libbrotli1:amd64 (1.0.9-2build6) ...
2026-Feb-25 07:11:19.497207
#24 48.07 Setting up libgdk-pixbuf2.0-common (2.42.8+dfsg-1ubuntu0.4) ...
2026-Feb-25 07:11:19.497207
#24 48.07 Setting up libnorm1:amd64 (1.5.9+dfsg-2) ...
2026-Feb-25 07:11:19.497207
#24 48.08 Setting up libmysofa1:amd64 (1.2.1~dfsg0-1) ...
2026-Feb-25 07:11:19.497207
#24 48.08 Setting up x11-common (1:7.7+23ubuntu2) ...
2026-Feb-25 07:11:19.497207
#24 48.15 debconf: unable to initialize frontend: Dialog
2026-Feb-25 07:11:19.497207
#24 48.15 debconf: (TERM is not set, so the dialog frontend is not usable.)
2026-Feb-25 07:11:19.497207
#24 48.15 debconf: falling back to frontend: Readline
2026-Feb-25 07:11:19.621035
#24 48.15 debconf: unable to initialize frontend: Readline
2026-Feb-25 07:11:19.621035
#24 48.15 debconf: (Can't locate Term/ReadLine.pm in @INC (you may need to install the Term::ReadLine module) (@INC contains: /etc/perl /usr/local/lib/x86_64-linux-gnu/perl/5.34.0 /usr/local/share/perl/5.34.0 /usr/lib/x86_64-linux-gnu/perl5/5.34 /usr/share/perl5 /usr/lib/x86_64-linux-gnu/perl-base /usr/lib/x86_64-linux-gnu/perl/5.34 /usr/share/perl/5.34 /usr/local/lib/site_perl) at /usr/share/perl5/Debconf/FrontEnd/Readline.pm line 7.)
2026-Feb-25 07:11:19.621035
#24 48.15 debconf: falling back to frontend: Teletype
2026-Feb-25 07:11:19.621035
#24 48.17 invoke-rc.d: could not determine current runlevel
2026-Feb-25 07:11:19.621035
#24 48.17 invoke-rc.d: policy-rc.d denied execution of start.
2026-Feb-25 07:11:19.621035
#24 48.18 Setting up libsensors-config (1:3.6.0-7ubuntu1) ...
2026-Feb-25 07:11:19.621035
#24 48.18 Setting up libdeflate0:amd64 (1.10-2) ...
2026-Feb-25 07:11:19.621035
#24 48.19 Setting up xkb-data (2.33-1) ...
2026-Feb-25 07:11:19.621035
#24 48.19 Setting up libigdgmm12:amd64 (22.1.2+ds1-1) ...
2026-Feb-25 07:11:19.621035
#24 48.19 Setting up libgomp1:amd64 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:11:19.621035
#24 48.19 Setting up libcdio19:amd64 (2.1.0-3ubuntu0.2) ...
2026-Feb-25 07:11:19.621035
#24 48.20 Setting up libxvidcore4:amd64 (2:1.3.7-1) ...
2026-Feb-25 07:11:19.621035
#24 48.20 Setting up libjbig0:amd64 (2.1-3.1ubuntu0.22.04.1) ...
2026-Feb-25 07:11:19.621035
#24 48.20 Setting up libsnappy1v5:amd64 (1.1.8-1build3) ...
2026-Feb-25 07:11:19.621035
#24 48.21 Setting up libflac8:amd64 (1.3.3-2ubuntu0.2) ...
2026-Feb-25 07:11:19.621035
#24 48.21 Setting up libasound2-data (1.2.6.1-1ubuntu1.1) ...
2026-Feb-25 07:11:19.621035
#24 48.21 Setting up ca-certificates (20240203~22.04.1) ...
2026-Feb-25 07:11:19.621035
#24 48.27 debconf: unable to initialize frontend: Dialog
2026-Feb-25 07:11:19.621035
#24 48.27 debconf: (TERM is not set, so the dialog frontend is not usable.)
2026-Feb-25 07:11:19.621035
#24 48.27 debconf: falling back to frontend: Readline
2026-Feb-25 07:11:19.771442
#24 48.28 debconf: unable to initialize frontend: Readline
2026-Feb-25 07:11:19.771442
#24 48.28 debconf: (Can't locate Term/ReadLine.pm in @INC (you may need to install the Term::ReadLine module) (@INC contains: /etc/perl /usr/local/lib/x86_64-linux-gnu/perl/5.34.0 /usr/local/share/perl/5.34.0 /usr/lib/x86_64-linux-gnu/perl5/5.34 /usr/share/perl5 /usr/lib/x86_64-linux-gnu/perl-base /usr/lib/x86_64-linux-gnu/perl/5.34 /usr/share/perl/5.34 /usr/local/lib/site_perl) at /usr/share/perl5/Debconf/FrontEnd/Readline.pm line 7.)
2026-Feb-25 07:11:19.771442
#24 48.28 debconf: falling back to frontend: Teletype
2026-Feb-25 07:11:20.880682
#24 49.53 Updating certificates in /etc/ssl/certs...
2026-Feb-25 07:11:21.623168
#24 50.28 rehash: warning: skipping ca-certificates.crt,it does not contain exactly one certificate or CRL
2026-Feb-25 07:11:21.775437
#24 50.30 14 added, 5 removed; done.
2026-Feb-25 07:11:21.775437
#24 50.32 Setting up libblas3:amd64 (3.10.0-2ubuntu1) ...
2026-Feb-25 07:11:21.775437
#24 50.32 update-alternatives: using /usr/lib/x86_64-linux-gnu/blas/libblas.so.3 to provide /usr/lib/x86_64-linux-gnu/libblas.so.3 (libblas.so.3-x86_64-linux-gnu) in auto mode
2026-Feb-25 07:11:21.775437
#24 50.32 Setting up libglib2.0-data (2.72.4-0ubuntu2.9) ...
2026-Feb-25 07:11:21.775437
#24 50.33 Setting up libslang2:amd64 (2.3.2-5build4) ...
2026-Feb-25 07:11:21.775437
#24 50.33 Setting up libva2:amd64 (2.14.0-1) ...
2026-Feb-25 07:11:21.775437
#24 50.33 Setting up libx11-data (2:1.7.5-1ubuntu0.3) ...
2026-Feb-25 07:11:21.775437
#24 50.33 Setting up libx264-163:amd64 (2:0.163.3060+git5db6aa6-2build1) ...
2026-Feb-25 07:11:21.775437
#24 50.34 Setting up libdbus-1-3:amd64 (1.12.20-2ubuntu4.1) ...
2026-Feb-25 07:11:21.775437
#24 50.34 Setting up dbus (1.12.20-2ubuntu4.1) ...
2026-Feb-25 07:11:21.775437
#24 50.43 Setting up libfribidi0:amd64 (1.0.8-2ubuntu3.1) ...
2026-Feb-25 07:11:21.895918
#24 50.44 Setting up libopus0:amd64 (1.3.1-0.1build2) ...
2026-Feb-25 07:11:21.895918
#24 50.45 Setting up libquadmath0:amd64 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:11:21.895918
#24 50.46 Setting up intel-media-va-driver:amd64 (22.3.1+dfsg1-1ubuntu2) ...
2026-Feb-25 07:11:21.895918
#24 50.46 Setting up libpng16-16:amd64 (1.6.37-3ubuntu0.4) ...
2026-Feb-25 07:11:21.895918
#24 50.47 Setting up libvorbis0a:amd64 (1.3.7-1build2) ...
2026-Feb-25 07:11:21.895918
#24 50.47 Setting up fonts-dejavu-core (2.37-2build1) ...
2026-Feb-25 07:11:21.895918
#24 50.49 Setting up ucf (3.0043) ...
2026-Feb-25 07:11:21.895918
#24 50.55 debconf: unable to initialize frontend: Dialog
2026-Feb-25 07:11:21.895918
#24 50.55 debconf: (TERM is not set, so the dialog frontend is not usable.)
2026-Feb-25 07:11:21.895918
#24 50.55 debconf: falling back to frontend: Readline
2026-Feb-25 07:11:21.997923
#24 50.55 debconf: unable to initialize frontend: Readline
2026-Feb-25 07:11:21.997923
#24 50.55 debconf: (Can't locate Term/ReadLine.pm in @INC (you may need to install the Term::ReadLine module) (@INC contains: /etc/perl /usr/local/lib/x86_64-linux-gnu/perl/5.34.0 /usr/local/share/perl/5.34.0 /usr/lib/x86_64-linux-gnu/perl5/5.34 /usr/share/perl5 /usr/lib/x86_64-linux-gnu/perl-base /usr/lib/x86_64-linux-gnu/perl/5.34 /usr/share/perl/5.34 /usr/local/lib/site_perl) at /usr/share/perl5/Debconf/FrontEnd/Readline.pm line 7.)
2026-Feb-25 07:11:21.997923
#24 50.55 debconf: falling back to frontend: Teletype
2026-Feb-25 07:11:21.997923
#24 50.57 Setting up libsensors5:amd64 (1:3.6.0-7ubuntu1) ...
2026-Feb-25 07:11:21.997923
#24 50.57 Setting up libaacs0:amd64 (0.11.1-1) ...
2026-Feb-25 07:11:21.997923
#24 50.58 Setting up libjpeg-turbo8:amd64 (2.1.2-0ubuntu1) ...
2026-Feb-25 07:11:21.997923
#24 50.58 Setting up pocketsphinx-en-us (0.8.0+real5prealpha+1-14ubuntu1) ...
2026-Feb-25 07:11:21.997923
#24 50.58 Setting up libglapi-mesa:amd64 (23.2.1-1ubuntu3.1~22.04.3) ...
2026-Feb-25 07:11:21.997923
#24 50.59 Setting up libgfortran5:amd64 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:11:21.997923
#24 50.59 Setting up libwebp7:amd64 (1.2.2-2ubuntu0.22.04.2) ...
2026-Feb-25 07:11:21.997923
#24 50.59 Setting up libbdplus0:amd64 (0.2.0-1) ...
2026-Feb-25 07:11:21.997923
#24 50.60 Setting up libnuma1:amd64 (2.0.14-3ubuntu2) ...
2026-Feb-25 07:11:21.997923
#24 50.60 Setting up libvidstab1.1:amd64 (1.1.0-2) ...
2026-Feb-25 07:11:21.997923
#24 50.60 Setting up libmd0:amd64 (1.0.4-1build1) ...
2026-Feb-25 07:11:21.997923
#24 50.61 Setting up alsa-topology-conf (1.2.5.1-2) ...
2026-Feb-25 07:11:21.997923
#24 50.62 Setting up ocl-icd-libopencl1:amd64 (2.2.14-3) ...
2026-Feb-25 07:11:21.997923
#24 50.62 Setting up libasyncns0:amd64 (0.8-6build2) ...
2026-Feb-25 07:11:21.997923
#24 50.62 Setting up libxshmfence1:amd64 (1.3-1build4) ...
2026-Feb-25 07:11:21.997923
#24 50.63 Setting up libbs2b0:amd64 (3.1.0+dfsg-2.2build1) ...
2026-Feb-25 07:11:21.997923
#24 50.63 Setting up libasound2:amd64 (1.2.6.1-1ubuntu1.1) ...
2026-Feb-25 07:11:21.997923
#24 50.63 Setting up libzimg2:amd64 (3.0.3+ds1-1) ...
2026-Feb-25 07:11:21.997923
#24 50.64 Setting up libopenjp2-7:amd64 (2.4.0-6ubuntu0.4) ...
2026-Feb-25 07:11:21.997923
#24 50.64 Setting up libopenal-data (1:1.19.1-2build3) ...
2026-Feb-25 07:11:21.997923
#24 50.64 Setting up libthai-data (0.1.29-1build1) ...
2026-Feb-25 07:11:21.997923
#24 50.65 Setting up libvpx7:amd64 (1.11.0-2ubuntu2.5) ...
2026-Feb-25 07:11:21.997923
#24 50.65 Setting up libwayland-egl1:amd64 (1.20.0-1ubuntu0.1) ...
2026-Feb-25 07:11:21.997923
#24 50.65 Setting up libusb-1.0-0:amd64 (2:1.0.25-1ubuntu2) ...
2026-Feb-25 07:11:22.137451
#24 50.65 Setting up libdav1d5:amd64 (0.9.2-1) ...
2026-Feb-25 07:11:22.137451
#24 50.66 Setting up libmfx1:amd64 (22.3.0-1) ...
2026-Feb-25 07:11:22.137451
#24 50.66 Setting up libsamplerate0:amd64 (0.2.2-1build1) ...
2026-Feb-25 07:11:22.137451
#24 50.66 Setting up libwebpmux3:amd64 (1.2.2-2ubuntu0.22.04.2) ...
2026-Feb-25 07:11:22.137451
#24 50.67 Setting up libbsd0:amd64 (0.11.5-1) ...
2026-Feb-25 07:11:22.137451
#24 50.67 Setting up libdrm-common (2.4.113-2~ubuntu0.22.04.1) ...
2026-Feb-25 07:11:22.137451
#24 50.67 Setting up libelf1:amd64 (0.186-1ubuntu0.1) ...
2026-Feb-25 07:11:22.137451
#24 50.68 Setting up libzvbi-common (0.2.35-19) ...
2026-Feb-25 07:11:22.137451
#24 50.68 Setting up libmp3lame0:amd64 (3.100-3build2) ...
2026-Feb-25 07:11:22.137451
#24 50.68 Setting up libvorbisenc2:amd64 (1.3.7-1build2) ...
2026-Feb-25 07:11:22.137451
#24 50.68 Setting up libicu70:amd64 (70.1-2) ...
2026-Feb-25 07:11:22.137451
#24 50.69 Setting up libiec61883-0:amd64 (1.2.0-4build3) ...
2026-Feb-25 07:11:22.137451
#24 50.69 Setting up libserd-0-0:amd64 (0.30.10-2) ...
2026-Feb-25 07:11:22.137451
#24 50.69 Setting up libxkbcommon0:amd64 (1.4.0-1) ...
2026-Feb-25 07:11:22.137451
#24 50.70 Setting up libwayland-client0:amd64 (1.20.0-1ubuntu0.1) ...
2026-Feb-25 07:11:22.137451
#24 50.70 Setting up libjpeg8:amd64 (8c-2ubuntu10) ...
2026-Feb-25 07:11:22.137451
#24 50.70 Setting up libavc1394-0:amd64 (0.5.4-5build2) ...
2026-Feb-25 07:11:22.137451
#24 50.71 Setting up libzvbi0:amd64 (0.2.35-19) ...
2026-Feb-25 07:11:22.137451
#24 50.71 Setting up libxdmcp6:amd64 (1:1.1.3-0ubuntu5) ...
2026-Feb-25 07:11:22.137451
#24 50.71 Setting up liblapack3:amd64 (3.10.0-2ubuntu1) ...
2026-Feb-25 07:11:22.137451
#24 50.72 update-alternatives: using /usr/lib/x86_64-linux-gnu/lapack/liblapack.so.3 to provide /usr/lib/x86_64-linux-gnu/liblapack.so.3 (liblapack.so.3-x86_64-linux-gnu) in auto mode
2026-Feb-25 07:11:22.137451
#24 50.72 Setting up libxcb1:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:11:22.137451
#24 50.72 Setting up libxcb-xfixes0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:11:22.137451
#24 50.72 Setting up libzmq5:amd64 (4.3.4-2) ...
2026-Feb-25 07:11:22.137451
#24 50.73 Setting up libcaca0:amd64 (0.99.beta19-2.2ubuntu4.1) ...
2026-Feb-25 07:11:22.137451
#24 50.73 Setting up alsa-ucm-conf (1.2.6.3-1ubuntu1.12) ...
2026-Feb-25 07:11:22.137451
#24 50.73 Setting up libxcb-render0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:11:22.137451
#24 50.74 Setting up libsoxr0:amd64 (0.1.3-4build2) ...
2026-Feb-25 07:11:22.137451
#24 50.74 Setting up libcdio-cdda2:amd64 (10.2+2.0.0-1build3) ...
2026-Feb-25 07:11:22.137451
#24 50.74 Setting up fontconfig-config (2.13.1-4.2ubuntu5) ...
2026-Feb-25 07:11:22.137451
#24 50.79 Setting up libcdio-paranoia2:amd64 (10.2+2.0.0-1build3) ...
2026-Feb-25 07:11:22.239281
#24 50.79 Setting up libxcb-glx0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:11:22.239281
#24 50.80 Setting up libedit2:amd64 (3.1-20210910-1build1) ...
2026-Feb-25 07:11:22.239281
#24 50.80 Setting up libxcb-shape0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:11:22.239281
#24 50.80 Setting up libxcb-shm0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:11:22.239281
#24 50.81 Setting up libxcb-present0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:11:22.239281
#24 50.81 Setting up libthai0:amd64 (0.1.29-1build1) ...
2026-Feb-25 07:11:22.239281
#24 50.81 Setting up libvorbisfile3:amd64 (1.3.7-1build2) ...
2026-Feb-25 07:11:22.239281
#24 50.82 Setting up libfreetype6:amd64 (2.11.1+dfsg-1ubuntu0.3) ...
2026-Feb-25 07:11:22.239281
#24 50.82 Setting up libxcb-sync1:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:11:22.239281
#24 50.82 Setting up libdc1394-25:amd64 (2.2.6-4) ...
2026-Feb-25 07:11:22.239281
#24 50.83 Setting up libx265-199:amd64 (3.5-2) ...
2026-Feb-25 07:11:22.239281
#24 50.83 Setting up librubberband2:amd64 (2.0.0-2) ...
2026-Feb-25 07:11:22.239281
#24 50.83 Setting up libsndio7.0:amd64 (1.8.1-1.1) ...
2026-Feb-25 07:11:22.239281
#24 50.83 Setting up libxcb-dri2-0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:11:22.239281
#24 50.84 Setting up libjack-jackd2-0:amd64 (1.9.20~dfsg-1) ...
2026-Feb-25 07:11:22.239281
#24 50.84 Setting up libdrm2:amd64 (2.4.113-2~ubuntu0.22.04.1) ...
2026-Feb-25 07:11:22.239281
#24 50.84 Setting up libflite1:amd64 (2.2-3) ...
2026-Feb-25 07:11:22.239281
#24 50.85 Setting up libva-drm2:amd64 (2.14.0-1) ...
2026-Feb-25 07:11:22.239281
#24 50.85 Setting up libsord-0-0:amd64 (0.16.8-2) ...
2026-Feb-25 07:11:22.239281
#24 50.85 Setting up libwayland-cursor0:amd64 (1.20.0-1ubuntu0.1) ...
2026-Feb-25 07:11:22.247344
#24 50.86 Setting up libxcb-randr0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:11:22.247344
#24 50.86 Setting up libsratom-0-0:amd64 (0.6.8-1) ...
2026-Feb-25 07:11:22.247344
#24 50.86 Setting up libdecor-0-0:amd64 (0.1.0-3build1) ...
2026-Feb-25 07:11:22.247344
#24 50.86 Setting up libx11-6:amd64 (2:1.7.5-1ubuntu0.3) ...
2026-Feb-25 07:11:22.247344
#24 50.87 Setting up libharfbuzz0b:amd64 (2.7.4-1ubuntu3.2) ...
2026-Feb-25 07:11:22.247344
#24 50.87 Setting up libtiff5:amd64 (4.3.0-6ubuntu0.12) ...
2026-Feb-25 07:11:22.247344
#24 50.87 Setting up libfontconfig1:amd64 (2.13.1-4.2ubuntu5) ...
2026-Feb-25 07:11:22.247344
#24 50.88 Setting up libsndfile1:amd64 (1.0.31-2ubuntu0.2) ...
2026-Feb-25 07:11:22.247344
#24 50.89 Setting up liblilv-0-0:amd64 (0.24.12-2) ...
2026-Feb-25 07:11:22.247344
#24 50.89 Setting up libxml2:amd64 (2.9.13+dfsg-1ubuntu0.11) ...
2026-Feb-25 07:11:22.400840
#24 50.90 Setting up libopenmpt0:amd64 (0.6.1-1) ...
2026-Feb-25 07:11:22.400840
#24 50.90 Setting up libdrm-amdgpu1:amd64 (2.4.113-2~ubuntu0.22.04.1) ...
2026-Feb-25 07:11:22.400840
#24 50.90 Setting up libxcb-dri3-0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:11:22.400840
#24 50.91 Setting up libx11-xcb1:amd64 (2:1.7.5-1ubuntu0.3) ...
2026-Feb-25 07:11:22.400840
#24 50.91 Setting up fontconfig (2.13.1-4.2ubuntu5) ...
2026-Feb-25 07:11:22.400840
#24 50.91 Regenerating fonts cache...
2026-Feb-25 07:11:23.756041
#24 ...
2026-Feb-25 07:11:23.756041
2026-Feb-25 07:11:23.756041
#44 [api builder 9/9] RUN cargo build --release --bin vps-gateway
2026-Feb-25 07:11:23.756041
#44 31.40    Compiling proxy v0.1.0 (/app/crates/proxy)
2026-Feb-25 07:11:23.756041
#44 31.47 warning: unused import: `info`
2026-Feb-25 07:11:23.756041
#44 31.47   --> crates/proxy/src/anti_bot.rs:17:29
2026-Feb-25 07:11:23.756041
#44 31.47    |
2026-Feb-25 07:11:23.756041
#44 31.47 17 | use tracing::{debug, error, info, warn};
2026-Feb-25 07:11:23.756041
#44 31.47    |                             ^^^^
2026-Feb-25 07:11:23.756041
#44 31.47    |
2026-Feb-25 07:11:23.756041
#44 31.47    = note: `#[warn(unused_imports)]` on by default
2026-Feb-25 07:11:23.756041
#44 31.47
2026-Feb-25 07:11:23.756041
#44 31.47 warning: unused import: `std::time::Duration`
2026-Feb-25 07:11:23.756041
#44 31.47   --> crates/proxy/src/client.rs:11:5
2026-Feb-25 07:11:23.756041
#44 31.47    |
2026-Feb-25 07:11:23.756041
#44 31.47 11 | use std::time::Duration;
2026-Feb-25 07:11:23.756041
#44 31.47    |     ^^^^^^^^^^^^^^^^^^^
2026-Feb-25 07:11:23.756041
#44 31.47
2026-Feb-25 07:11:23.756041
#44 31.48 warning: unused import: `std::sync::Arc`
2026-Feb-25 07:11:23.756041
#44 31.48  --> crates/proxy/src/cookie_store.rs:8:5
2026-Feb-25 07:11:23.756041
#44 31.48   |
2026-Feb-25 07:11:23.756041
#44 31.48 8 | use std::sync::Arc;
2026-Feb-25 07:11:23.756041
#44 31.48   |     ^^^^^^^^^^^^^^
2026-Feb-25 07:11:23.756041
#44 31.48
2026-Feb-25 07:11:23.756041
#44 31.53 warning: variable `last_proxy` is assigned to, but never used
2026-Feb-25 07:11:23.756041
#44 31.53    --> crates/proxy/src/anti_bot.rs:163:17
2026-Feb-25 07:11:23.756041
#44 31.53     |
2026-Feb-25 07:11:23.756041
#44 31.53 163 |         let mut last_proxy: Option<String> = None;
2026-Feb-25 07:11:23.756041
#44 31.53     |                 ^^^^^^^^^^
2026-Feb-25 07:11:23.756041
#44 31.53     |
2026-Feb-25 07:11:23.756041
#44 31.53     = note: consider using `_last_proxy` instead
2026-Feb-25 07:11:23.756041
#44 31.53     = note: `#[warn(unused_variables)]` on by default
2026-Feb-25 07:11:23.756041
#44 31.53
2026-Feb-25 07:11:23.756041
#44 31.53 warning: value assigned to `last_proxy` is never read
2026-Feb-25 07:11:23.756041
#44 31.53    --> crates/proxy/src/anti_bot.rs:216:25
2026-Feb-25 07:11:23.756041
#44 31.53     |
2026-Feb-25 07:11:23.756041
#44 31.53 216 |                         last_proxy = current_proxy;
2026-Feb-25 07:11:23.756041
#44 31.53     |                         ^^^^^^^^^^
2026-Feb-25 07:11:23.756041
#44 31.53     |
2026-Feb-25 07:11:23.756041
#44 31.53     = help: maybe it is overwritten before being read?
2026-Feb-25 07:11:23.756041
#44 31.53     = note: `#[warn(unused_assignments)]` on by default
2026-Feb-25 07:11:23.756041
#44 31.53
2026-Feb-25 07:11:23.756041
#44 31.53 warning: value assigned to `last_proxy` is never read
2026-Feb-25 07:11:23.756041
#44 31.53    --> crates/proxy/src/anti_bot.rs:239:21
2026-Feb-25 07:11:23.756041
#44 31.53     |
2026-Feb-25 07:11:23.756041
#44 31.53 239 |                     last_proxy = current_proxy;
2026-Feb-25 07:11:23.756041
#44 31.53     |                     ^^^^^^^^^^
2026-Feb-25 07:11:23.756041
#44 31.53     |
2026-Feb-25 07:11:23.756041
#44 31.53     = help: maybe it is overwritten before being read?
2026-Feb-25 07:11:23.756041
#44 31.53
2026-Feb-25 07:11:23.756041
#44 31.63 warning: unused variable: `now`
2026-Feb-25 07:11:23.756041
#44 31.63   --> crates/proxy/src/throttle.rs:41:13
2026-Feb-25 07:11:23.756041
#44 31.63    |
2026-Feb-25 07:11:23.756041
#44 31.63 41 |         let now = Instant::now();
2026-Feb-25 07:11:23.756041
#44 31.63    |             ^^^ help: if this is intentional, prefix it with an underscore: `_now`
2026-Feb-25 07:11:23.756041
#44 31.63
2026-Feb-25 07:11:23.756041
#44 32.27 warning: `proxy` (lib) generated 7 warnings (run `cargo fix --lib -p proxy` to apply 3 suggestions)
2026-Feb-25 07:11:23.756041
#44 37.05    Compiling serde_v8 v0.209.0
2026-Feb-25 07:11:23.756041
#44 37.34    Compiling deno_core v0.300.0
2026-Feb-25 07:11:23.756041
#44 40.20 warning: unused imports: `error` and `warn`
2026-Feb-25 07:11:23.756041
#44 40.20   --> crates/extractor/src/lib.rs:21:22
2026-Feb-25 07:11:23.756041
#44 40.20    |
2026-Feb-25 07:11:23.756041
#44 40.20 21 | use tracing::{debug, error, info, warn};
2026-Feb-25 07:11:23.756041
#44 40.20    |                      ^^^^^        ^^^^
2026-Feb-25 07:11:23.756041
#44 40.20    |
2026-Feb-25 07:11:23.756041
#44 40.20    = note: `#[warn(unused_imports)]` on by default
2026-Feb-25 07:11:23.756041
#44 40.20
2026-Feb-25 07:11:23.756041
#44 40.20 warning: unused import: `debug`
2026-Feb-25 07:11:23.756041
#44 40.20   --> crates/extractor/src/hot_reload.rs:10:15
2026-Feb-25 07:11:23.756041
#44 40.20    |
2026-Feb-25 07:11:23.756041
#44 40.20 10 | use tracing::{debug, error, info, warn};
2026-Feb-25 07:11:23.756041
#44 40.20    |               ^^^^^
2026-Feb-25 07:11:23.756041
#44 40.20
2026-Feb-25 07:11:23.756041
#44 40.20 warning: unused import: `SemaphorePermit`
2026-Feb-25 07:11:23.756041
#44 40.20  --> crates/extractor/src/pool.rs:9:30
2026-Feb-25 07:11:23.756041
#44 40.20   |
2026-Feb-25 07:11:23.756041
#44 40.20 9 | use tokio::sync::{Semaphore, SemaphorePermit};
2026-Feb-25 07:11:23.756041
#44 40.20   |                              ^^^^^^^^^^^^^^^
2026-Feb-25 07:11:23.765715
#44 40.20
2026-Feb-25 07:11:23.765715
#44 40.20 warning: unused imports: `error` and `warn`
2026-Feb-25 07:11:23.765715
#44 40.20   --> crates/extractor/src/pool.rs:10:22
2026-Feb-25 07:11:23.765715
#44 40.20    |
2026-Feb-25 07:11:23.765715
#44 40.20 10 | use tracing::{debug, error, info, warn};
2026-Feb-25 07:11:23.765715
#44 40.20    |                      ^^^^^        ^^^^
2026-Feb-25 07:11:23.765715
#44 40.20
2026-Feb-25 07:11:23.765715
#44 40.66    Compiling muxer v0.1.0 (/app/crates/muxer)
2026-Feb-25 07:11:23.765715
#44 41.00 warning: fields `video_buffer` and `audio_buffer` are never read
2026-Feb-25 07:11:23.765715
#44 41.00    --> crates/muxer/src/stream_fetcher.rs:200:5
2026-Feb-25 07:11:23.765715
#44 41.00     |
2026-Feb-25 07:11:23.765715
#44 41.00 197 | pub struct CombinedStream {
2026-Feb-25 07:11:23.765715
#44 41.00     |            -------------- fields in this struct
2026-Feb-25 07:11:23.765715
#44 41.00 ...
2026-Feb-25 07:11:23.765715
#44 41.00 200 |     video_buffer: Option<Bytes>,
2026-Feb-25 07:11:23.765715
#44 41.00     |     ^^^^^^^^^^^^
2026-Feb-25 07:11:23.765715
#44 41.00 201 |     audio_buffer: Option<Bytes>,
2026-Feb-25 07:11:23.765715
#44 41.00     |     ^^^^^^^^^^^^
2026-Feb-25 07:11:23.765715
#44 41.00     |
2026-Feb-25 07:11:23.765715
#44 41.00     = note: `#[warn(dead_code)]` on by default
2026-Feb-25 07:11:23.765715
#44 41.00
2026-Feb-25 07:11:23.765715
#44 41.27 warning: `muxer` (lib) generated 1 warning
2026-Feb-25 07:11:24.081956
#44 41.86 warning: `extractor` (lib) generated 4 warnings (run `cargo fix --lib -p extractor` to apply 4 suggestions)
2026-Feb-25 07:11:24.269647
#44 ...
2026-Feb-25 07:11:24.269647
2026-Feb-25 07:11:24.269647
#24 [gpu-worker runtime 3/6] RUN apt-get update && apt-get install -y     ca-certificates     libssl3     ffmpeg     libavcodec58     libavformat58     libavutil56     libswscale5     libavfilter7     libavdevice58     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:11:24.269647
#24 50.91 Regenerating fonts cache... done.
2026-Feb-25 07:11:24.471730
#24 52.93 Setting up libdrm-nouveau2:amd64 (2.4.113-2~ubuntu0.22.04.1) ...
2026-Feb-25 07:11:24.471730
#24 52.93 Setting up libxrender1:amd64 (1:0.9.10-1build4) ...
2026-Feb-25 07:11:24.471730
#24 52.94 Setting up libgbm1:amd64 (23.2.1-1ubuntu3.1~22.04.3) ...
2026-Feb-25 07:11:24.471730
#24 52.94 Setting up libpulse0:amd64 (1:15.99.1+dfsg1-1ubuntu2.2) ...
2026-Feb-25 07:11:24.471730
#24 52.95 Setting up libdrm-radeon1:amd64 (2.4.113-2~ubuntu0.22.04.1) ...
2026-Feb-25 07:11:24.471730
#24 52.95 Setting up libpango-1.0-0:amd64 (1.50.6+ds-2ubuntu1) ...
2026-Feb-25 07:11:24.471730
#24 52.95 Setting up libdrm-intel1:amd64 (2.4.113-2~ubuntu0.22.04.1) ...
2026-Feb-25 07:11:24.471730
#24 52.96 Setting up libxext6:amd64 (2:1.3.4-1build1) ...
2026-Feb-25 07:11:24.471730
#24 52.96 Setting up libopenal1:amd64 (1:1.19.1-2build3) ...
2026-Feb-25 07:11:24.471730
#24 52.96 Setting up libcairo2:amd64 (1.16.0-5ubuntu2) ...
2026-Feb-25 07:11:24.471730
#24 52.97 Setting up libxxf86vm1:amd64 (1:1.1.4-1build3) ...
2026-Feb-25 07:11:24.471730
#24 52.97 Setting up libass9:amd64 (1:0.15.2-1) ...
2026-Feb-25 07:11:24.471730
#24 52.97 Setting up libxfixes3:amd64 (1:6.0.0-1) ...
2026-Feb-25 07:11:24.471730
#24 52.98 Setting up shared-mime-info (2.1-2) ...
2026-Feb-25 07:11:24.953999
#24 53.61 Setting up libxinerama1:amd64 (2:1.1.4-3) ...
2026-Feb-25 07:11:25.056203
#24 53.61 Setting up libxv1:amd64 (2:1.0.11-1build2) ...
2026-Feb-25 07:11:25.056203
#24 53.61 Setting up libxrandr2:amd64 (2:1.5.2-1build1) ...
2026-Feb-25 07:11:25.056203
#24 53.62 Setting up libvdpau1:amd64 (1.4-3build2) ...
2026-Feb-25 07:11:25.056203
#24 53.62 Setting up libllvm15:amd64 (1:15.0.7-0ubuntu0.22.04.3) ...
2026-Feb-25 07:11:25.056203
#24 53.62 Setting up libtheora0:amd64 (1.1.1+dfsg.1-15ubuntu4) ...
2026-Feb-25 07:11:25.056203
#24 53.63 Setting up libgdk-pixbuf-2.0-0:amd64 (2.42.8+dfsg-1ubuntu0.4) ...
2026-Feb-25 07:11:25.056203
#24 53.65 Setting up libcairo-gobject2:amd64 (1.16.0-5ubuntu2) ...
2026-Feb-25 07:11:25.056203
#24 53.65 Setting up libxss1:amd64 (1:1.2.3-1build2) ...
2026-Feb-25 07:11:25.056203
#24 53.65 Setting up mesa-va-drivers:amd64 (23.2.1-1ubuntu3.1~22.04.3) ...
2026-Feb-25 07:11:25.056203
#24 53.66 Setting up libpangoft2-1.0-0:amd64 (1.50.6+ds-2ubuntu1) ...
2026-Feb-25 07:11:25.056203
#24 53.66 Setting up libbluray2:amd64 (1:1.3.1-1) ...
2026-Feb-25 07:11:25.056203
#24 53.66 Setting up libva-x11-2:amd64 (2.14.0-1) ...
2026-Feb-25 07:11:25.056203
#24 53.66 Setting up i965-va-driver:amd64 (2.4.1+dfsg1-1) ...
2026-Feb-25 07:11:25.056203
#24 53.67 Setting up libpangocairo-1.0-0:amd64 (1.50.6+ds-2ubuntu1) ...
2026-Feb-25 07:11:25.056203
#24 53.67 Setting up libgl1-amber-dri:amd64 (21.3.9-0ubuntu1~22.04.1) ...
2026-Feb-25 07:11:25.056203
#24 53.67 Setting up mesa-vdpau-drivers:amd64 (23.2.1-1ubuntu3.1~22.04.3) ...
2026-Feb-25 07:11:25.056203
#24 53.68 Setting up libxi6:amd64 (2:1.8-1build1) ...
2026-Feb-25 07:11:25.056203
#24 53.68 Setting up libsphinxbase3:amd64 (0.8+5prealpha+1-13build1) ...
2026-Feb-25 07:11:25.056203
#24 53.68 Setting up libxcursor1:amd64 (1:1.2.0-2build4) ...
2026-Feb-25 07:11:25.056203
#24 53.69 Setting up libgl1-mesa-dri:amd64 (23.2.1-1ubuntu3.1~22.04.3) ...
2026-Feb-25 07:11:25.056203
#24 53.69 Setting up libavutil56:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:11:25.056203
#24 53.70 Setting up librsvg2-2:amd64 (2.52.5+dfsg-3ubuntu0.2) ...
2026-Feb-25 07:11:25.056203
#24 53.70 Setting up libpocketsphinx3:amd64 (0.8.0+real5prealpha+1-14ubuntu1) ...
2026-Feb-25 07:11:25.056203
#24 53.70 Setting up va-driver-all:amd64 (2.14.0-1) ...
2026-Feb-25 07:11:25.056203
#24 53.71 Setting up libdecor-0-plugin-1-cairo:amd64 (0.1.0-3build1) ...
2026-Feb-25 07:11:25.056203
#24 53.71 Setting up libpostproc55:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:11:25.294110
#24 53.71 Setting up librsvg2-common:amd64 (2.52.5+dfsg-3ubuntu0.2) ...
2026-Feb-25 07:11:25.294110
#24 53.72 Setting up vdpau-driver-all:amd64 (1.4-3build2) ...
2026-Feb-25 07:11:25.294110
#24 53.72 Setting up libgdk-pixbuf2.0-bin (2.42.8+dfsg-1ubuntu0.4) ...
2026-Feb-25 07:11:25.294110
#24 53.73 Setting up libswscale5:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:11:25.294110
#24 53.73 Setting up libsdl2-2.0-0:amd64 (2.0.20+dfsg-2ubuntu1.22.04.1) ...
2026-Feb-25 07:11:25.294110
#24 53.73 Setting up libglx-mesa0:amd64 (23.2.1-1ubuntu3.1~22.04.3) ...
2026-Feb-25 07:11:25.294110
#24 53.74 Setting up libglx0:amd64 (1.4.0-1) ...
2026-Feb-25 07:11:25.294110
#24 53.74 Setting up libswresample3:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:11:25.294110
#24 53.74 Setting up libgl1:amd64 (1.4.0-1) ...
2026-Feb-25 07:11:25.294110
#24 53.75 Setting up libavcodec58:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:11:25.294110
#24 53.75 Setting up libchromaprint1:amd64 (1.5.1-2) ...
2026-Feb-25 07:11:25.294110
#24 53.75 Setting up libavformat58:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:11:25.294110
#24 53.75 Setting up libavfilter7:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:11:25.294110
#24 53.76 Setting up libavdevice58:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:11:25.294110
#24 53.76 Setting up ffmpeg (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:11:25.294110
#24 53.76 Processing triggers for libc-bin (2.35-0ubuntu3.6) ...
2026-Feb-25 07:11:25.294110
#24 53.79 Processing triggers for ca-certificates (20240203~22.04.1) ...
2026-Feb-25 07:11:25.294110
#24 53.80 Updating certificates in /etc/ssl/certs...
2026-Feb-25 07:11:25.625031
#24 54.28 0 added, 0 removed; done.
2026-Feb-25 07:11:25.625031
#24 54.28 Running hooks in /etc/ca-certificates/update.d...
2026-Feb-25 07:11:25.780683
#24 54.28 done.
2026-Feb-25 07:11:25.780683
#24 54.28 Processing triggers for libgdk-pixbuf-2.0-0:amd64 (2.42.8+dfsg-1ubuntu0.4) ...
2026-Feb-25 07:11:25.942056
#24 DONE 54.6s
2026-Feb-25 07:11:25.942056
2026-Feb-25 07:11:25.942056
#37 [gpu-worker builder 3/9] RUN apt-get update && apt-get install -y     curl     build-essential     pkg-config     libssl-dev     protobuf-compiler     ffmpeg     libavcodec-dev     libavformat-dev     libavutil-dev     libswscale-dev     libavfilter-dev     libavdevice-dev     clang     libclang-dev     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:11:25.942056
#37 47.07 Get:85 http://archive.ubuntu.com/ubuntu jammy/universe amd64 clang amd64 1:14.0-55~exp2 [3558 B]
2026-Feb-25 07:11:25.942056
#37 47.35 Get:86 http://archive.ubuntu.com/ubuntu jammy/main amd64 libbrotli1 amd64 1.0.9-2build6 [315 kB]
2026-Feb-25 07:11:25.942056
#37 48.18 Get:87 http://archive.ubuntu.com/ubuntu jammy/main amd64 librtmp1 amd64 2.4+20151223.gitfa8646d.1-2build4 [58.2 kB]
2026-Feb-25 07:11:25.942056
#37 49.10 Get:88 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libssh-4 amd64 0.9.6-2ubuntu0.22.04.6 [187 kB]
2026-Feb-25 07:11:25.942056
#37 49.98 Get:89 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libcurl4 amd64 7.81.0-1ubuntu1.22 [290 kB]
2026-Feb-25 07:11:25.942056
#37 50.67 Get:90 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 curl amd64 7.81.0-1ubuntu1.22 [194 kB]
2026-Feb-25 07:11:25.942056
#37 51.07 Get:91 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libaom3 amd64 3.3.0-1ubuntu0.1 [1748 kB]
2026-Feb-25 07:11:25.942056
#37 52.28 Get:92 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libva2 amd64 2.14.0-1 [65.0 kB]
2026-Feb-25 07:11:25.942056
#37 52.56 Get:93 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libmfx1 amd64 22.3.0-1 [3105 kB]
2026-Feb-25 07:11:25.942056
#37 53.45 Get:94 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libva-drm2 amd64 2.14.0-1 [7502 B]
2026-Feb-25 07:11:25.942056
#37 54.19 Get:95 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxfixes3 amd64 1:6.0.0-1 [11.7 kB]
2026-Feb-25 07:11:25.942056
#37 54.47 Get:96 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libva-x11-2 amd64 2.14.0-1 [12.6 kB]
2026-Feb-25 07:11:26.093600
#37 ...
2026-Feb-25 07:11:26.093600
2026-Feb-25 07:11:26.093600
#50 [gpu-worker runtime 4/6] RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-25 07:11:26.149674
#50 DONE 0.2s
2026-Feb-25 07:11:26.149674
2026-Feb-25 07:11:26.149674
#37 [gpu-worker builder 3/9] RUN apt-get update && apt-get install -y     curl     build-essential     pkg-config     libssl-dev     protobuf-compiler     ffmpeg     libavcodec-dev     libavformat-dev     libavutil-dev     libswscale-dev     libavfilter-dev     libavdevice-dev     clang     libclang-dev     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:11:26.314087
#37 54.83 Get:97 http://archive.ubuntu.com/ubuntu jammy/main amd64 libvdpau1 amd64 1.4-3build2 [27.0 kB]
2026-Feb-25 07:11:26.622606
#37 55.29 Get:98 http://archive.ubuntu.com/ubuntu jammy/universe amd64 ocl-icd-libopencl1 amd64 2.2.14-3 [39.1 kB]
2026-Feb-25 07:11:27.117374
#37 55.79 Get:99 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libavutil56 amd64 7:4.4.2-0ubuntu0.22.04.1 [290 kB]
2026-Feb-25 07:11:28.265806
#37 56.94 Get:100 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libfreetype6 amd64 2.11.1+dfsg-1ubuntu0.3 [388 kB]
2026-Feb-25 07:11:28.971843
#37 57.64 Get:101 http://archive.ubuntu.com/ubuntu jammy/main amd64 fonts-dejavu-core all 2.37-2build1 [1041 kB]
2026-Feb-25 07:11:29.877352
#37 58.55 Get:102 http://archive.ubuntu.com/ubuntu jammy/main amd64 fontconfig-config all 2.13.1-4.2ubuntu5 [29.1 kB]
2026-Feb-25 07:11:30.310025
#37 58.83 Get:103 http://archive.ubuntu.com/ubuntu jammy/main amd64 libfontconfig1 amd64 2.13.1-4.2ubuntu5 [131 kB]
2026-Feb-25 07:11:30.460600
#37 59.13 Get:104 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libpixman-1-0 amd64 0.40.0-1ubuntu0.22.04.1 [264 kB]
2026-Feb-25 07:11:30.785393
#37 59.46 Get:105 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-render0 amd64 1.14-3ubuntu3 [16.4 kB]
2026-Feb-25 07:11:31.065286
#37 59.74 Get:106 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-shm0 amd64 1.14-3ubuntu3 [5780 B]
2026-Feb-25 07:11:31.343579
#37 60.01 Get:107 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxrender1 amd64 1:0.9.10-1build4 [19.7 kB]
2026-Feb-25 07:11:31.494109
#37 ...
2026-Feb-25 07:11:31.494109
2026-Feb-25 07:11:31.494109
#44 [api builder 9/9] RUN cargo build --release --bin vps-gateway
2026-Feb-25 07:11:31.494109
#44 47.82    Compiling api v0.1.0 (/app/crates/api)
2026-Feb-25 07:11:31.494109
#44 47.93 warning: unused import: `StreamExt`
2026-Feb-25 07:11:31.494109
#44 47.93  --> crates/api/src/routes/batch.rs:7:48
2026-Feb-25 07:11:31.494109
#44 47.93   |
2026-Feb-25 07:11:31.494109
#44 47.93 7 | use futures::stream::{self, BoxStream, Stream, StreamExt};
2026-Feb-25 07:11:31.494109
#44 47.93   |                                                ^^^^^^^^^
2026-Feb-25 07:11:31.494109
#44 47.93   |
2026-Feb-25 07:11:31.494109
#44 47.93   = note: `#[warn(unused_imports)]` on by default
2026-Feb-25 07:11:31.494109
#44 47.93
2026-Feb-25 07:11:31.494109
#44 47.93 warning: unused import: `error`
2026-Feb-25 07:11:31.494109
#44 47.93   --> crates/api/src/routes/batch.rs:11:15
2026-Feb-25 07:11:31.494109
#44 47.93    |
2026-Feb-25 07:11:31.494109
#44 47.93 11 | use tracing::{error, info, warn};
2026-Feb-25 07:11:31.494109
#44 47.93    |               ^^^^^
2026-Feb-25 07:11:31.494109
#44 47.93
2026-Feb-25 07:11:31.494109
#44 47.93 warning: unused import: `VideoInfo`
2026-Feb-25 07:11:31.494109
#44 47.93  --> crates/api/src/routes/extract.rs:9:30
2026-Feb-25 07:11:31.494109
#44 47.93   |
2026-Feb-25 07:11:31.494109
#44 47.93 9 | use extractor::{VideoFormat, VideoInfo};
2026-Feb-25 07:11:31.494109
#44 47.93   |                              ^^^^^^^^^
2026-Feb-25 07:11:31.494109
#44 47.93
2026-Feb-25 07:11:31.494109
#44 47.94 warning: unused imports: `MuxRouter` and `StreamSource`
2026-Feb-25 07:11:31.494109
#44 47.94   --> crates/api/src/routes/stream.rs:20:25
2026-Feb-25 07:11:31.494109
#44 47.94    |
2026-Feb-25 07:11:31.494109
#44 47.94 20 | use muxer::mux_router::{MuxRouter, StreamSource};
2026-Feb-25 07:11:31.494109
#44 47.94    |                         ^^^^^^^^^  ^^^^^^^^^^^^
2026-Feb-25 07:11:31.494109
#44 47.94
2026-Feb-25 07:11:31.494109
#44 47.94 warning: unused import: `bytes::Bytes`
2026-Feb-25 07:11:31.494109
#44 47.94  --> crates/api/src/routes/transcode.rs:9:5
2026-Feb-25 07:11:31.494109
#44 47.94   |
2026-Feb-25 07:11:31.494109
#44 47.94 9 | use bytes::Bytes;
2026-Feb-25 07:11:31.494109
#44 47.94   |     ^^^^^^^^^^^^
2026-Feb-25 07:11:31.494109
#44 47.94
2026-Feb-25 07:11:31.494109
#44 47.94 warning: unused imports: `debug` and `error`
2026-Feb-25 07:11:31.494109
#44 47.94   --> crates/api/src/routes/transcode.rs:12:15
2026-Feb-25 07:11:31.494109
#44 47.94    |
2026-Feb-25 07:11:31.494109
#44 47.94 12 | use tracing::{debug, error, info, warn};
2026-Feb-25 07:11:31.494109
#44 47.94    |               ^^^^^  ^^^^^
2026-Feb-25 07:11:31.494109
#44 47.94
2026-Feb-25 07:11:31.494109
#44 48.13 warning: unused import: `futures::StreamExt`
2026-Feb-25 07:11:31.494109
#44 48.13   --> crates/api/src/routes/transcode.rs:10:5
2026-Feb-25 07:11:31.494109
#44 48.13    |
2026-Feb-25 07:11:31.494109
#44 48.13 10 | use futures::StreamExt;
2026-Feb-25 07:11:31.494109
#44 48.13    |     ^^^^^^^^^^^^^^^^^^
2026-Feb-25 07:11:31.494109
#44 48.13
2026-Feb-25 07:11:31.494109
#44 48.22 warning: fields `extractor_dir`, `gpu_worker_addr`, and `gpu_enabled` are never read
2026-Feb-25 07:11:31.494109
#44 48.22   --> crates/api/src/config.rs:13:9
2026-Feb-25 07:11:31.494109
#44 48.22    |
2026-Feb-25 07:11:31.494109
#44 48.22 9  | pub struct Config {
2026-Feb-25 07:11:31.494109
#44 48.22    |            ------ fields in this struct
2026-Feb-25 07:11:31.494109
#44 48.22 ...
2026-Feb-25 07:11:31.494109
#44 48.22 13 |     pub extractor_dir: String,
2026-Feb-25 07:11:31.494109
#44 48.22    |         ^^^^^^^^^^^^^
2026-Feb-25 07:11:31.494109
#44 48.22 14 |     /// GPU worker gRPC address (e.g., "10.0.0.2:50051")
2026-Feb-25 07:11:31.494109
#44 48.22 15 |     pub gpu_worker_addr: String,
2026-Feb-25 07:11:31.494109
#44 48.22    |         ^^^^^^^^^^^^^^^
2026-Feb-25 07:11:31.494109
#44 48.22 16 |     /// Whether GPU transcoding is enabled
2026-Feb-25 07:11:31.494109
#44 48.22 17 |     pub gpu_enabled: bool,
2026-Feb-25 07:11:31.494109
#44 48.22    |         ^^^^^^^^^^^
2026-Feb-25 07:11:31.494109
#44 48.22    |
2026-Feb-25 07:11:31.494109
#44 48.22    = note: `Config` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis
2026-Feb-25 07:11:31.494109
#44 48.22    = note: `#[warn(dead_code)]` on by default
2026-Feb-25 07:11:31.494109
#44 48.22
2026-Feb-25 07:11:31.494109
#44 48.22 warning: field `format` is never read
2026-Feb-25 07:11:31.494109
#44 48.22   --> crates/api/src/routes/extract.rs:19:9
2026-Feb-25 07:11:31.494109
#44 48.22    |
2026-Feb-25 07:11:31.494109
#44 48.22 13 | pub struct ExtractRequest {
2026-Feb-25 07:11:31.494109
#44 48.22    |            -------------- field in this struct
2026-Feb-25 07:11:31.494109
#44 48.22 ...
2026-Feb-25 07:11:31.494109
#44 48.22 19 |     pub format: Option<String>,
2026-Feb-25 07:11:31.494109
#44 48.22    |         ^^^^^^
2026-Feb-25 07:11:31.494109
#44 48.22    |
2026-Feb-25 07:11:31.494109
#44 48.22    = note: `ExtractRequest` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
2026-Feb-25 07:11:31.494109
#44 48.22
2026-Feb-25 07:11:31.494109
#44 48.22 warning: fields `video_codec` and `audio_codec` are never read
2026-Feb-25 07:11:31.494109
#44 48.22   --> crates/api/src/routes/stream.rs:50:9
2026-Feb-25 07:11:31.494109
#44 48.22    |
2026-Feb-25 07:11:31.494109
#44 48.22 44 | pub struct MuxedStreamParams {
2026-Feb-25 07:11:31.494109
#44 48.22    |            ----------------- fields in this struct
2026-Feb-25 07:11:31.494109
#44 48.22 ...
2026-Feb-25 07:11:31.494109
#44 48.22 50 |     pub video_codec: Option<String>,
2026-Feb-25 07:11:31.494109
#44 48.22    |         ^^^^^^^^^^^
2026-Feb-25 07:11:31.494109
#44 48.22 51 |     /// Audio codec (e.g., "aac", "opus")
2026-Feb-25 07:11:31.494109
#44 48.22 52 |     pub audio_codec: Option<String>,
2026-Feb-25 07:11:31.494109
#44 48.22    |         ^^^^^^^^^^^
2026-Feb-25 07:11:31.494109
#44 48.22    |
2026-Feb-25 07:11:31.494109
#44 48.22    = note: `MuxedStreamParams` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
2026-Feb-25 07:11:31.494109
#44 48.22
2026-Feb-25 07:11:31.494109
#44 48.22 warning: field `options` is never read
2026-Feb-25 07:11:31.494109
#44 48.22   --> crates/api/src/routes/transcode.rs:22:9
2026-Feb-25 07:11:31.494109
#44 48.22    |
2026-Feb-25 07:11:31.494109
#44 48.22 16 | pub struct TranscodeRequest {
2026-Feb-25 07:11:31.494109
#44 48.22    |            ---------------- field in this struct
2026-Feb-25 07:11:31.494109
#44 48.22 ...
2026-Feb-25 07:11:31.494109
#44 48.22 22 |     pub options: Option<TranscodeOptions>,
2026-Feb-25 07:11:31.494109
#44 48.22    |         ^^^^^^^
2026-Feb-25 07:11:31.494109
#44 48.22    |
2026-Feb-25 07:11:31.494109
#44 48.22    = note: `TranscodeRequest` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
2026-Feb-25 07:11:31.494109
#44 48.22
2026-Feb-25 07:11:31.494109
#44 48.22 warning: fields `resolution`, `bitrate_kbps`, and `format` are never read
2026-Feb-25 07:11:31.494109
#44 48.22   --> crates/api/src/routes/transcode.rs:41:9
2026-Feb-25 07:11:31.494109
#44 48.22    |
2026-Feb-25 07:11:31.494109
#44 48.22 39 | pub struct TranscodeOptions {
2026-Feb-25 07:11:31.494109
#44 48.22    |            ---------------- fields in this struct
2026-Feb-25 07:11:31.494109
#44 48.22 40 |     /// Target resolution (e.g., "1080p", "720p")
2026-Feb-25 07:11:31.494109
#44 48.22 41 |     pub resolution: Option<String>,
2026-Feb-25 07:11:31.494109
#44 48.22    |         ^^^^^^^^^^
2026-Feb-25 07:11:31.494109
#44 48.22 42 |     /// Target bitrate in kbps
2026-Feb-25 07:11:31.494109
#44 48.22 43 |     pub bitrate_kbps: Option<u32>,
2026-Feb-25 07:11:31.494109
#44 48.22    |         ^^^^^^^^^^^^
2026-Feb-25 07:11:31.494109
#44 48.22 44 |     /// Output format (e.g., "mp4", "webm")
2026-Feb-25 07:11:31.494109
#44 48.22 45 |     pub format: Option<String>,
2026-Feb-25 07:11:31.494109
#44 48.22    |         ^^^^^^
2026-Feb-25 07:11:31.494109
#44 48.22    |
2026-Feb-25 07:11:31.494109
#44 48.22    = note: `TranscodeOptions` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
2026-Feb-25 07:11:31.494109
#44 48.22
2026-Feb-25 07:11:31.494109
#44 48.22 warning: function `build_response_headers` is never used
2026-Feb-25 07:11:31.494109
#44 48.22    --> crates/api/src/routes/transcode.rs:155:4
2026-Feb-25 07:11:31.494109
#44 48.22     |
2026-Feb-25 07:11:31.494109
#44 48.22 155 | fn build_response_headers(
2026-Feb-25 07:11:31.494109
#44 48.22     |    ^^^^^^^^^^^^^^^^^^^^^^
2026-Feb-25 07:11:31.494109
#44 48.22
2026-Feb-25 07:11:31.494109
#44 48.22 warning: function `sanitize_filename` is never used
2026-Feb-25 07:11:31.494109
#44 48.22    --> crates/api/src/routes/transcode.rs:188:4
2026-Feb-25 07:11:31.494109
#44 48.22     |
2026-Feb-25 07:11:31.494109
#44 48.22 188 | fn sanitize_filename(name: &str) -> String {
2026-Feb-25 07:11:31.494109
#44 48.22     |    ^^^^^^^^^^^^^^^^^
2026-Feb-25 07:11:31.494109
#44 48.22
2026-Feb-25 07:11:31.503335
2026-Feb-25 07:11:41.654848
#44 ...
2026-Feb-25 07:11:41.654848
2026-Feb-25 07:11:41.654848
#37 [gpu-worker builder 3/9] RUN apt-get update && apt-get install -y     curl     build-essential     pkg-config     libssl-dev     protobuf-compiler     ffmpeg     libavcodec-dev     libavformat-dev     libavutil-dev     libswscale-dev     libavfilter-dev     libavdevice-dev     clang     libclang-dev     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:11:41.654848
#37 60.29 Get:108 http://archive.ubuntu.com/ubuntu jammy/main amd64 libcairo2 amd64 1.16.0-5ubuntu2 [628 kB]
2026-Feb-25 07:11:41.654848
#37 60.68 Get:109 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libcodec2-1.0 amd64 1.0.1-3 [8435 kB]
2026-Feb-25 07:11:41.654848
#37 62.18 Get:110 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libdav1d5 amd64 0.9.2-1 [463 kB]
2026-Feb-25 07:11:41.654848
#37 62.47 Get:111 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libgsm1 amd64 1.0.19-1 [27.7 kB]
2026-Feb-25 07:11:41.654848
#37 62.75 Get:112 http://archive.ubuntu.com/ubuntu jammy/main amd64 libmp3lame0 amd64 3.100-3build2 [141 kB]
2026-Feb-25 07:11:41.654848
#37 63.03 Get:113 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libopenjp2-7 amd64 2.4.0-6ubuntu0.4 [158 kB]
2026-Feb-25 07:11:41.654848
#37 63.31 Get:114 http://archive.ubuntu.com/ubuntu jammy/main amd64 libopus0 amd64 1.3.1-0.1build2 [203 kB]
2026-Feb-25 07:11:41.654848
#37 63.60 Get:115 http://archive.ubuntu.com/ubuntu jammy/main amd64 libcairo-gobject2 amd64 1.16.0-5ubuntu2 [19.4 kB]
2026-Feb-25 07:11:41.654848
#37 63.88 Get:116 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgdk-pixbuf2.0-common all 2.42.8+dfsg-1ubuntu0.4 [5546 B]
2026-Feb-25 07:11:41.654848
#37 64.15 Get:117 http://archive.ubuntu.com/ubuntu jammy/main amd64 libjpeg-turbo8 amd64 2.1.2-0ubuntu1 [134 kB]
2026-Feb-25 07:11:41.654848
#37 64.44 Get:118 http://archive.ubuntu.com/ubuntu jammy/main amd64 libjpeg8 amd64 8c-2ubuntu10 [2264 B]
2026-Feb-25 07:11:41.654848
#37 64.71 Get:119 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdeflate0 amd64 1.10-2 [70.9 kB]
2026-Feb-25 07:11:41.654848
#37 64.99 Get:120 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libjbig0 amd64 2.1-3.1ubuntu0.22.04.1 [29.2 kB]
2026-Feb-25 07:11:41.654848
#37 65.59 Get:121 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libwebp7 amd64 1.2.2-2ubuntu0.22.04.2 [206 kB]
2026-Feb-25 07:11:41.654848
#37 65.88 Get:122 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libtiff5 amd64 4.3.0-6ubuntu0.12 [185 kB]
2026-Feb-25 07:11:41.654848
#37 66.16 Get:123 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgdk-pixbuf-2.0-0 amd64 2.42.8+dfsg-1ubuntu0.4 [148 kB]
2026-Feb-25 07:11:41.654848
#37 66.44 Get:124 http://archive.ubuntu.com/ubuntu jammy/main amd64 fontconfig amd64 2.13.1-4.2ubuntu5 [177 kB]
2026-Feb-25 07:11:41.654848
#37 67.16 Get:125 http://archive.ubuntu.com/ubuntu jammy/main amd64 libgraphite2-3 amd64 1.3.14-1build2 [71.3 kB]
2026-Feb-25 07:11:41.654848
#37 68.07 Get:126 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libharfbuzz0b amd64 2.7.4-1ubuntu3.2 [353 kB]
2026-Feb-25 07:11:41.715611
#37 70.39 Get:127 http://archive.ubuntu.com/ubuntu jammy/main amd64 libthai-data all 0.1.29-1build1 [162 kB]
2026-Feb-25 07:11:42.894101
#37 71.56 Get:128 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdatrie1 amd64 0.2.13-2 [19.9 kB]
2026-Feb-25 07:11:43.156267
#37 71.83 Get:129 http://archive.ubuntu.com/ubuntu jammy/main amd64 libthai0 amd64 0.1.29-1build1 [19.2 kB]
2026-Feb-25 07:11:43.412619
#37 72.08 Get:130 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libpango-1.0-0 amd64 1.50.6+ds-2ubuntu1 [230 kB]
2026-Feb-25 07:11:44.004244
#37 72.67 Get:131 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libpangoft2-1.0-0 amd64 1.50.6+ds-2ubuntu1 [54.0 kB]
2026-Feb-25 07:11:44.280964
#37 72.95 Get:132 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libpangocairo-1.0-0 amd64 1.50.6+ds-2ubuntu1 [39.8 kB]
2026-Feb-25 07:11:44.546515
#37 73.22 Get:133 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 librsvg2-2 amd64 2.52.5+dfsg-3ubuntu0.2 [2974 kB]
2026-Feb-25 07:11:46.044085
#37 74.71 Get:134 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libshine3 amd64 3.1.1-2 [23.2 kB]
2026-Feb-25 07:11:46.285748
#37 74.96 Get:135 http://archive.ubuntu.com/ubuntu jammy/main amd64 libsnappy1v5 amd64 1.1.8-1build3 [17.5 kB]
2026-Feb-25 07:11:46.527703
#37 75.20 Get:136 http://archive.ubuntu.com/ubuntu jammy/main amd64 libspeex1 amd64 1.2~rc1.2-1.1ubuntu3 [57.9 kB]
2026-Feb-25 07:11:46.770794
#37 75.44 Get:137 http://archive.ubuntu.com/ubuntu jammy/main amd64 libsoxr0 amd64 0.1.3-4build2 [79.8 kB]
2026-Feb-25 07:11:47.017293
#37 75.69 Get:138 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libswresample3 amd64 7:4.4.2-0ubuntu0.22.04.1 [62.2 kB]
2026-Feb-25 07:11:47.261164
#37 75.93 Get:139 http://archive.ubuntu.com/ubuntu jammy/main amd64 libogg0 amd64 1.3.5-0ubuntu3 [22.9 kB]
2026-Feb-25 07:11:47.502058
#37 76.17 Get:140 http://archive.ubuntu.com/ubuntu jammy/main amd64 libtheora0 amd64 1.1.1+dfsg.1-15ubuntu4 [209 kB]
2026-Feb-25 07:11:47.759326
#37 76.43 Get:141 http://archive.ubuntu.com/ubuntu jammy/main amd64 libtwolame0 amd64 0.4.0-2build2 [52.5 kB]
2026-Feb-25 07:11:48.002964
#37 76.67 Get:142 http://archive.ubuntu.com/ubuntu jammy/main amd64 libvorbis0a amd64 1.3.7-1build2 [99.2 kB]
2026-Feb-25 07:11:48.251507
#37 76.92 Get:143 http://archive.ubuntu.com/ubuntu jammy/main amd64 libvorbisenc2 amd64 1.3.7-1build2 [82.6 kB]
2026-Feb-25 07:11:48.496776
#37 77.17 Get:144 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libvpx7 amd64 1.11.0-2ubuntu2.5 [1078 kB]
2026-Feb-25 07:11:48.835251
#37 77.51 Get:145 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libwebpmux3 amd64 1.2.2-2ubuntu0.22.04.2 [20.5 kB]
2026-Feb-25 07:11:49.076204
#37 77.75 Get:146 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libx264-163 amd64 2:0.163.3060+git5db6aa6-2build1 [591 kB]
2026-Feb-25 07:11:49.362853
#37 78.03 Get:147 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libx265-199 amd64 3.5-2 [1170 kB]
2026-Feb-25 07:11:49.696324
#37 78.37 Get:148 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libxvidcore4 amd64 2:1.3.7-1 [201 kB]
2026-Feb-25 07:11:49.951688
#37 78.62 Get:149 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libzvbi-common all 0.2.35-19 [35.5 kB]
2026-Feb-25 07:11:50.195295
#37 78.87 Get:150 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libzvbi0 amd64 0.2.35-19 [262 kB]
2026-Feb-25 07:11:50.456962
#37 79.13 Get:151 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libavcodec58 amd64 7:4.4.2-0ubuntu0.22.04.1 [5567 kB]
2026-Feb-25 07:11:51.211724
#37 79.88 Get:152 http://archive.ubuntu.com/ubuntu jammy/main amd64 libraw1394-11 amd64 2.1.2-2build2 [27.0 kB]
2026-Feb-25 07:11:51.734661
#37 80.40 Get:153 http://archive.ubuntu.com/ubuntu jammy/main amd64 libavc1394-0 amd64 0.5.4-5build2 [17.0 kB]
2026-Feb-25 07:11:51.975513
#37 80.65 Get:154 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libass9 amd64 1:0.15.2-1 [97.5 kB]
2026-Feb-25 07:11:52.217967
#37 80.89 Get:155 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libudfread0 amd64 1.1.2-1 [16.2 kB]
2026-Feb-25 07:11:52.459580
#37 81.13 Get:156 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libbluray2 amd64 1:1.3.1-1 [159 kB]
2026-Feb-25 07:11:52.703949
#37 81.37 Get:157 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libchromaprint1 amd64 1.5.1-2 [28.4 kB]
2026-Feb-25 07:11:52.945906
#37 81.62 Get:158 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libgme0 amd64 0.6.3-2 [127 kB]
2026-Feb-25 07:11:53.189738
#37 81.86 Get:159 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libmpg123-0 amd64 1.29.3-1ubuntu0.1 [172 kB]
2026-Feb-25 07:11:53.435333
#37 82.11 Get:160 http://archive.ubuntu.com/ubuntu jammy/main amd64 libvorbisfile3 amd64 1.3.7-1build2 [17.1 kB]
2026-Feb-25 07:11:53.676022
#37 82.35 Get:161 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libopenmpt0 amd64 0.6.1-1 [592 kB]
2026-Feb-25 07:11:53.933617
#37 82.60 Get:162 http://archive.ubuntu.com/ubuntu jammy/main amd64 librabbitmq4 amd64 0.10.0-1ubuntu2 [39.3 kB]
2026-Feb-25 07:11:54.173711
#37 82.84 Get:163 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libsrt1.4-gnutls amd64 1.4.4-4 [309 kB]
2026-Feb-25 07:11:54.423922
#37 83.09 Get:164 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libssh-gcrypt-4 amd64 0.9.6-2ubuntu0.22.04.6 [225 kB]
2026-Feb-25 07:11:54.670550
#37 83.34 Get:165 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libnorm1 amd64 1.5.9+dfsg-2 [221 kB]
2026-Feb-25 07:11:54.917830
#37 83.59 Get:166 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libpgm-5.3-0 amd64 5.3.128~dfsg-2 [161 kB]
2026-Feb-25 07:11:55.161759
#37 83.83 Get:167 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libsodium23 amd64 1.0.18-1ubuntu0.22.04.1 [164 kB]
2026-Feb-25 07:11:55.407665
#37 84.08 Get:168 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libzmq5 amd64 4.3.4-2 [256 kB]
2026-Feb-25 07:11:55.654001
#37 84.32 Get:169 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libavformat58 amd64 7:4.4.2-0ubuntu0.22.04.1 [1103 kB]
2026-Feb-25 07:11:55.927298
#37 84.60 Get:170 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libbs2b0 amd64 3.1.0+dfsg-2.2build1 [10.2 kB]
2026-Feb-25 07:11:56.166522
#37 84.84 Get:171 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libflite1 amd64 2.2-3 [13.7 MB]
2026-Feb-25 07:11:56.970943
#37 85.64 Get:172 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libserd-0-0 amd64 0.30.10-2 [40.8 kB]
2026-Feb-25 07:11:57.210578
#37 85.88 Get:173 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libsord-0-0 amd64 0.16.8-2 [21.2 kB]
2026-Feb-25 07:11:57.450482
#37 86.12 Get:174 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libsratom-0-0 amd64 0.6.8-1 [17.0 kB]
2026-Feb-25 07:11:57.694157
#37 86.36 Get:175 http://archive.ubuntu.com/ubuntu jammy/universe amd64 liblilv-0-0 amd64 0.24.12-2 [42.8 kB]
2026-Feb-25 07:11:59.743466
#37 88.41 Get:176 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libmysofa1 amd64 1.2.1~dfsg0-1 [1157 kB]
2026-Feb-25 07:12:02.898697
#37 91.57 Get:177 http://archive.ubuntu.com/ubuntu jammy/main amd64 libblas3 amd64 3.10.0-2ubuntu1 [228 kB]
2026-Feb-25 07:12:03.435296
#37 92.11 Get:178 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgfortran5 amd64 12.3.0-1ubuntu1~22.04.3 [879 kB]
2026-Feb-25 07:12:04.577175
#37 93.25 Get:179 http://archive.ubuntu.com/ubuntu jammy/main amd64 liblapack3 amd64 3.10.0-2ubuntu1 [2504 kB]
2026-Feb-25 07:12:06.521042
#37 95.19 Get:180 http://archive.ubuntu.com/ubuntu jammy/main amd64 libasyncns0 amd64 0.8-6build2 [12.8 kB]
2026-Feb-25 07:12:06.767066
#37 95.44 Get:181 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libflac8 amd64 1.3.3-2ubuntu0.2 [111 kB]
2026-Feb-25 07:12:07.052447
#37 95.72 Get:182 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libsndfile1 amd64 1.0.31-2ubuntu0.2 [196 kB]
2026-Feb-25 07:12:07.374843
#37 96.04 Get:183 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libx11-xcb1 amd64 2:1.7.5-1ubuntu0.3 [7802 B]
2026-Feb-25 07:12:07.618743
#37 96.29 Get:184 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libpulse0 amd64 1:15.99.1+dfsg1-1ubuntu2.2 [298 kB]
2026-Feb-25 07:12:07.983329
#37 96.65 Get:185 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libsphinxbase3 amd64 0.8+5prealpha+1-13build1 [126 kB]
2026-Feb-25 07:12:08.276465
#37 96.95 Get:186 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libpocketsphinx3 amd64 0.8.0+real5prealpha+1-14ubuntu1 [132 kB]
2026-Feb-25 07:12:08.571014
#37 97.24 Get:187 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libpostproc55 amd64 7:4.4.2-0ubuntu0.22.04.1 [60.1 kB]
2026-Feb-25 07:12:08.836110
#37 97.51 Get:188 http://archive.ubuntu.com/ubuntu jammy/main amd64 libsamplerate0 amd64 0.2.2-1build1 [1359 kB]
2026-Feb-25 07:12:09.579373
#37 98.25 Get:189 http://archive.ubuntu.com/ubuntu jammy/universe amd64 librubberband2 amd64 2.0.0-2 [90.0 kB]
2026-Feb-25 07:12:09.850073
#37 98.52 Get:190 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libswscale5 amd64 7:4.4.2-0ubuntu0.22.04.1 [180 kB]
2026-Feb-25 07:12:10.150243
#37 98.82 Get:191 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libvidstab1.1 amd64 1.1.0-2 [35.0 kB]
2026-Feb-25 07:12:10.401963
#37 99.07 Get:192 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libzimg2 amd64 3.0.3+ds1-1 [241 kB]
2026-Feb-25 07:12:10.722327
#37 99.39 Get:193 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libavfilter7 amd64 7:4.4.2-0ubuntu0.22.04.1 [1496 kB]
2026-Feb-25 07:12:11.415065
#37 100.1 Get:194 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libcaca0 amd64 0.99.beta19-2.2ubuntu4.1 [224 kB]
2026-Feb-25 07:12:11.718862
#37 100.4 Get:195 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libcdio19 amd64 2.1.0-3ubuntu0.2 [63.6 kB]
2026-Feb-25 07:12:11.976745
#37 100.6 Get:196 http://archive.ubuntu.com/ubuntu jammy/main amd64 libcdio-cdda2 amd64 10.2+2.0.0-1build3 [16.7 kB]
2026-Feb-25 07:12:12.222027
#37 100.9 Get:197 http://archive.ubuntu.com/ubuntu jammy/main amd64 libcdio-paranoia2 amd64 10.2+2.0.0-1build3 [15.9 kB]
2026-Feb-25 07:12:12.465759
#37 101.1 Get:198 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libdc1394-25 amd64 2.2.6-4 [88.8 kB]
2026-Feb-25 07:12:12.732012
#37 101.4 Get:199 http://archive.ubuntu.com/ubuntu jammy/main amd64 libglvnd0 amd64 1.4.0-1 [73.6 kB]
2026-Feb-25 07:12:12.993564
#37 101.7 Get:200 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libglapi-mesa amd64 23.2.1-1ubuntu3.1~22.04.3 [35.4 kB]
2026-Feb-25 07:12:13.243426
#37 101.9 Get:201 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-dri2-0 amd64 1.14-3ubuntu3 [7206 B]
2026-Feb-25 07:12:13.484266
#37 102.2 Get:202 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-dri3-0 amd64 1.14-3ubuntu3 [6968 B]
2026-Feb-25 07:12:13.726726
#37 102.4 Get:203 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-glx0 amd64 1.14-3ubuntu3 [25.9 kB]
2026-Feb-25 07:12:13.973181
#37 102.6 Get:204 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-present0 amd64 1.14-3ubuntu3 [5734 B]
2026-Feb-25 07:12:14.213621
#37 102.9 Get:205 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-randr0 amd64 1.14-3ubuntu3 [18.3 kB]
2026-Feb-25 07:12:14.743482
#37 103.4 Get:206 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-sync1 amd64 1.14-3ubuntu3 [9416 B]
2026-Feb-25 07:12:14.987506
#37 103.7 Get:207 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-xfixes0 amd64 1.14-3ubuntu3 [9996 B]
2026-Feb-25 07:12:15.232235
#37 103.9 Get:208 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxshmfence1 amd64 1.3-1build4 [5394 B]
2026-Feb-25 07:12:15.483685
#37 104.2 Get:209 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxxf86vm1 amd64 1:1.1.4-1build3 [10.4 kB]
2026-Feb-25 07:12:15.726978
#37 104.4 Get:210 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libdrm-amdgpu1 amd64 2.4.113-2~ubuntu0.22.04.1 [19.9 kB]
2026-Feb-25 07:12:15.977693
#37 104.6 Get:211 http://archive.ubuntu.com/ubuntu jammy/main amd64 libpciaccess0 amd64 0.16-3 [19.1 kB]
2026-Feb-25 07:12:16.222403
#37 104.9 Get:212 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libdrm-intel1 amd64 2.4.113-2~ubuntu0.22.04.1 [66.7 kB]
2026-Feb-25 07:12:16.482839
#37 105.2 Get:213 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libdrm-nouveau2 amd64 2.4.113-2~ubuntu0.22.04.1 [17.5 kB]
2026-Feb-25 07:12:16.732592
#37 105.4 Get:214 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libdrm-radeon1 amd64 2.4.113-2~ubuntu0.22.04.1 [21.6 kB]
2026-Feb-25 07:12:16.978030
#37 105.6 Get:215 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libllvm15 amd64 1:15.0.7-0ubuntu0.22.04.3 [25.4 MB]
2026-Feb-25 07:12:20.059840
#37 108.7 Get:216 http://archive.ubuntu.com/ubuntu jammy/main amd64 libsensors-config all 1:3.6.0-7ubuntu1 [5274 B]
2026-Feb-25 07:12:20.229864
#37 ...
2026-Feb-25 07:12:20.229864
2026-Feb-25 07:12:20.229864
#44 [api builder 9/9] RUN cargo build --release --bin vps-gateway
2026-Feb-25 07:12:20.229864
#44 97.86 warning: `api` (bin "vps-gateway") generated 14 warnings (run `cargo fix --bin "vps-gateway"` to apply 6 suggestions)
2026-Feb-25 07:12:20.229864
#44 97.86     Finished `release` profile [optimized] target(s) in 1m 37s
2026-Feb-25 07:12:20.242256
#44 DONE 98.0s
2026-Feb-25 07:12:20.242256
2026-Feb-25 07:12:20.242256
#37 [gpu-worker builder 3/9] RUN apt-get update && apt-get install -y     curl     build-essential     pkg-config     libssl-dev     protobuf-compiler     ffmpeg     libavcodec-dev     libavformat-dev     libavutil-dev     libswscale-dev     libavfilter-dev     libavdevice-dev     clang     libclang-dev     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:12:20.365328
#37 ...
2026-Feb-25 07:12:20.365328
2026-Feb-25 07:12:20.365328
#51 [api runtime 5/6] COPY --from=builder /app/target/release/vps-gateway /usr/local/bin/
2026-Feb-25 07:12:20.365328
#51 DONE 0.0s
2026-Feb-25 07:12:20.365328
2026-Feb-25 07:12:20.365328
#37 [gpu-worker builder 3/9] RUN apt-get update && apt-get install -y     curl     build-essential     pkg-config     libssl-dev     protobuf-compiler     ffmpeg     libavcodec-dev     libavformat-dev     libavutil-dev     libswscale-dev     libavfilter-dev     libavdevice-dev     clang     libclang-dev     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:12:20.365328
#37 109.0 Get:217 http://archive.ubuntu.com/ubuntu jammy/main amd64 libsensors5 amd64 1:3.6.0-7ubuntu1 [26.3 kB]
2026-Feb-25 07:12:20.520870
#37 ...
2026-Feb-25 07:12:20.520870
2026-Feb-25 07:12:20.520870
#52 [api runtime 6/6] RUN mkdir -p /app/extractors /app/data && chown -R appuser:appuser /app
2026-Feb-25 07:12:20.520870
#52 DONE 0.2s
2026-Feb-25 07:12:20.689969
#53 [api] exporting to image
2026-Feb-25 07:12:20.689969
#53 exporting layers
2026-Feb-25 07:12:22.454492
#53 exporting layers 1.8s done
2026-Feb-25 07:12:22.454492
#53 exporting manifest sha256:807e72a1d2039a43b058f99d5ce2aa8377504fa73231a165d3e9f06e309bf18a done
2026-Feb-25 07:12:22.454492
#53 exporting config sha256:df444ec535dcffc05b00ad425a1b6ba5949a8b7e69addb8957f8e6925ec59f3b done
2026-Feb-25 07:12:22.454492
#53 exporting attestation manifest sha256:3a536cad38be087c7ab4838679ab742952d5c5d4b639e8e3f226c960b4e40a38 done
2026-Feb-25 07:12:22.454492
#53 exporting manifest list sha256:dbb5e2056fbef1f3ee6121c036d4923d2238cbc93a84b2f3673675013541c3b1 done
2026-Feb-25 07:12:22.454492
#53 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_api:6284ccef8da2b1bbb3aa00193368912f946f4bb3 done
2026-Feb-25 07:12:22.454492
#53 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_api:6284ccef8da2b1bbb3aa00193368912f946f4bb3
2026-Feb-25 07:12:22.861198
#53 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_api:6284ccef8da2b1bbb3aa00193368912f946f4bb3 0.5s done
2026-Feb-25 07:12:22.861198
#53 DONE 2.3s
2026-Feb-25 07:12:22.861198
2026-Feb-25 07:12:22.861198
#54 [api] resolving provenance for metadata file
2026-Feb-25 07:12:22.971166
#54 DONE 0.0s
2026-Feb-25 07:12:22.971166
2026-Feb-25 07:12:22.971166
#37 [gpu-worker builder 3/9] RUN apt-get update && apt-get install -y     curl     build-essential     pkg-config     libssl-dev     protobuf-compiler     ffmpeg     libavcodec-dev     libavformat-dev     libavutil-dev     libswscale-dev     libavfilter-dev     libavdevice-dev     clang     libclang-dev     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 07:12:22.971166
#37 109.2 Get:218 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgl1-mesa-dri amd64 23.2.1-1ubuntu3.1~22.04.3 [8860 kB]
2026-Feb-25 07:12:22.971166
#37 109.9 Get:219 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libglx-mesa0 amd64 23.2.1-1ubuntu3.1~22.04.3 [158 kB]
2026-Feb-25 07:12:22.971166
#37 110.2 Get:220 http://archive.ubuntu.com/ubuntu jammy/main amd64 libglx0 amd64 1.4.0-1 [41.0 kB]
2026-Feb-25 07:12:22.971166
#37 110.4 Get:221 http://archive.ubuntu.com/ubuntu jammy/main amd64 libgl1 amd64 1.4.0-1 [110 kB]
2026-Feb-25 07:12:22.971166
#37 110.6 Get:222 http://archive.ubuntu.com/ubuntu jammy/main amd64 libiec61883-0 amd64 1.2.0-4build3 [25.9 kB]
2026-Feb-25 07:12:22.971166
#37 110.9 Get:223 http://archive.ubuntu.com/ubuntu jammy/main amd64 libjack-jackd2-0 amd64 1.9.20~dfsg-1 [293 kB]
2026-Feb-25 07:12:22.971166
#37 111.1 Get:224 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libopenal-data all 1:1.19.1-2build3 [164 kB]
2026-Feb-25 07:12:22.971166
#37 111.4 Get:225 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libsndio7.0 amd64 1.8.1-1.1 [29.3 kB]
2026-Feb-25 07:12:22.971166
#37 111.6 Get:226 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libopenal1 amd64 1:1.19.1-2build3 [535 kB]
2026-Feb-25 07:12:23.236197
#37 111.9 Get:227 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libwayland-client0 amd64 1.20.0-1ubuntu0.1 [25.9 kB]
2026-Feb-25 07:12:24.179901
#37 112.8 Get:228 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdecor-0-0 amd64 0.1.0-3build1 [15.1 kB]
2026-Feb-25 07:12:24.675693
#37 113.3 Get:229 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libwayland-server0 amd64 1.20.0-1ubuntu0.1 [34.3 kB]
2026-Feb-25 07:12:25.178564
#37 113.8 Get:230 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgbm1 amd64 23.2.1-1ubuntu3.1~22.04.3 [33.5 kB]
2026-Feb-25 07:12:25.520621
#37 114.2 Get:231 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libwayland-cursor0 amd64 1.20.0-1ubuntu0.1 [10.7 kB]
2026-Feb-25 07:12:25.787410
#37 114.5 Get:232 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libwayland-egl1 amd64 1.20.0-1ubuntu0.1 [5582 B]
2026-Feb-25 07:12:26.048193
#37 114.7 Get:233 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcursor1 amd64 1:1.2.0-2build4 [20.9 kB]
2026-Feb-25 07:12:26.346319
#37 115.0 Get:234 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxi6 amd64 2:1.8-1build1 [32.6 kB]
2026-Feb-25 07:12:26.674728
#37 115.3 Get:235 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxinerama1 amd64 2:1.1.4-3 [7382 B]
2026-Feb-25 07:12:26.938609
#37 115.6 Get:236 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxkbcommon0 amd64 1.4.0-1 [125 kB]
2026-Feb-25 07:12:27.557819
#37 116.2 Get:237 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxrandr2 amd64 2:1.5.2-1build1 [20.4 kB]
2026-Feb-25 07:12:27.824746
#37 116.5 Get:238 http://archive.ubuntu.com/ubuntu jammy/main amd64 x11-common all 1:7.7+23ubuntu2 [23.4 kB]
2026-Feb-25 07:12:28.097573
#37 116.8 Get:239 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxss1 amd64 1:1.2.3-1build2 [8476 B]
2026-Feb-25 07:12:28.354151
#37 117.0 Get:240 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libsdl2-2.0-0 amd64 2.0.20+dfsg-2ubuntu1.22.04.1 [582 kB]
2026-Feb-25 07:12:29.240638
#37 117.9 Get:241 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-shape0 amd64 1.14-3ubuntu3 [6158 B]
2026-Feb-25 07:12:29.487836
#37 118.2 Get:242 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxv1 amd64 2:1.0.11-1build2 [11.2 kB]
2026-Feb-25 07:12:30.738757
#37 119.4 Get:243 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libavdevice58 amd64 7:4.4.2-0ubuntu0.22.04.1 [87.5 kB]
2026-Feb-25 07:12:31.998136
#37 120.7 Get:244 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 ffmpeg amd64 7:4.4.2-0ubuntu0.22.04.1 [1696 kB]
2026-Feb-25 07:12:33.336060
#37 122.0 Get:245 http://archive.ubuntu.com/ubuntu jammy/main amd64 icu-devtools amd64 70.1-2 [197 kB]
2026-Feb-25 07:12:33.686170
#37 122.4 Get:246 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libigdgmm12 amd64 22.1.2+ds1-1 [139 kB]
2026-Feb-25 07:12:34.004581
#37 122.7 Get:247 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 intel-media-va-driver amd64 22.3.1+dfsg1-1ubuntu2 [2283 kB]
2026-Feb-25 07:12:35.181833
#37 123.9 Get:248 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libaacs0 amd64 0.11.1-1 [64.1 kB]
2026-Feb-25 07:12:35.448265
#37 124.1 Get:249 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libavutil-dev amd64 7:4.4.2-0ubuntu0.22.04.1 [427 kB]
2026-Feb-25 07:12:35.846492
#37 124.5 Get:250 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libswresample-dev amd64 7:4.4.2-0ubuntu0.22.04.1 [78.0 kB]
2026-Feb-25 07:12:36.119965
#37 124.8 Get:251 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libavcodec-dev amd64 7:4.4.2-0ubuntu0.22.04.1 [6221 kB]
2026-Feb-25 07:12:37.781928
#37 126.5 Get:252 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libavformat-dev amd64 7:4.4.2-0ubuntu0.22.04.1 [1347 kB]
2026-Feb-25 07:12:38.255286
#37 126.9 Get:253 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libpostproc-dev amd64 7:4.4.2-0ubuntu0.22.04.1 [60.9 kB]
2026-Feb-25 07:12:38.801384
#37 127.5 Get:254 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libswscale-dev amd64 7:4.4.2-0ubuntu0.22.04.1 [206 kB]
2026-Feb-25 07:12:39.082830
#37 127.8 Get:255 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libavfilter-dev amd64 7:4.4.2-0ubuntu0.22.04.1 [1732 kB]
2026-Feb-25 07:12:39.613734
#37 128.3 Get:256 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libavdevice-dev amd64 7:4.4.2-0ubuntu0.22.04.1 [97.3 kB]
2026-Feb-25 07:12:39.874699
#37 128.5 Get:257 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libbdplus0 amd64 0.2.0-1 [52.2 kB]
2026-Feb-25 07:12:40.134418
#37 128.8 Get:258 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libclang-14-dev amd64 1:14.0.0-1ubuntu1.1 [25.2 MB]
2026-Feb-25 07:12:43.059084
#37 131.7 Get:259 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libclang-dev amd64 1:14.0-55~exp2 [3138 B]
2026-Feb-25 07:12:43.305474
#37 132.0 Get:260 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdecor-0-plugin-1-cairo amd64 0.1.0-3build1 [20.4 kB]
2026-Feb-25 07:12:43.553151
#37 132.2 Get:261 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgdk-pixbuf2.0-bin amd64 2.42.8+dfsg-1ubuntu0.4 [14.1 kB]
2026-Feb-25 07:12:43.801514
#37 132.5 Get:262 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgl1-amber-dri amd64 21.3.9-0ubuntu1~22.04.1 [4218 kB]
2026-Feb-25 07:12:44.285108
#37 133.0 Get:263 http://archive.ubuntu.com/ubuntu jammy/main amd64 libicu-dev amd64 70.1-2 [11.6 MB]
2026-Feb-25 07:12:45.098253
#37 133.8 Get:264 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libncurses-dev amd64 6.3-2ubuntu0.1 [381 kB]
2026-Feb-25 07:12:45.362418
#37 134.0 Get:265 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libprotobuf-lite23 amd64 3.12.4-1ubuntu7.22.04.4 [209 kB]
2026-Feb-25 07:12:45.619537
#37 134.3 Get:266 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libprotobuf23 amd64 3.12.4-1ubuntu7.22.04.4 [878 kB]
2026-Feb-25 07:12:45.904858
#37 134.6 Get:267 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libprotoc23 amd64 3.12.4-1ubuntu7.22.04.4 [662 kB]
2026-Feb-25 07:12:46.181590
#37 134.9 Get:268 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 librsvg2-common amd64 2.52.5+dfsg-3ubuntu0.2 [17.7 kB]
2026-Feb-25 07:12:46.429665
#37 135.1 Get:269 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libssl-dev amd64 3.0.2-0ubuntu1.21 [2375 kB]
2026-Feb-25 07:12:46.782302
#37 135.5 Get:270 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libtinfo-dev amd64 6.3-2ubuntu0.1 [780 B]
2026-Feb-25 07:12:47.030392
#37 135.7 Get:271 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libxml2-dev amd64 2.9.13+dfsg-1ubuntu0.11 [805 kB]
2026-Feb-25 07:12:47.313799
#37 136.0 Get:272 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 llvm-14-runtime amd64 1:14.0.0-1ubuntu1.1 [484 kB]
2026-Feb-25 07:12:47.582339
#37 136.3 Get:273 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libpfm4 amd64 4.11.1+git32-gd0b85fb-1ubuntu0.1 [345 kB]
2026-Feb-25 07:12:47.845550
#37 136.5 Get:274 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 llvm-14 amd64 1:14.0.0-1ubuntu1.1 [12.7 MB]
2026-Feb-25 07:12:48.602827
#37 137.3 Get:275 http://archive.ubuntu.com/ubuntu jammy/main amd64 libffi-dev amd64 3.4.2-4 [63.7 kB]
2026-Feb-25 07:12:48.850998
#37 137.5 Get:276 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 python3-pygments all 2.11.2+dfsg-2ubuntu0.1 [750 kB]
2026-Feb-25 07:12:49.125305
#37 137.8 Get:277 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 llvm-14-tools amd64 1:14.0.0-1ubuntu1.1 [404 kB]
2026-Feb-25 07:12:49.385859
#37 138.1 Get:278 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libz3-4 amd64 4.8.12-1 [5766 kB]
2026-Feb-25 07:12:49.841782
#37 138.5 Get:279 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libz3-dev amd64 4.8.12-1 [72.2 kB]
2026-Feb-25 07:12:50.379350
#37 139.0 Get:280 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 llvm-14-dev amd64 1:14.0.0-1ubuntu1.1 [37.8 MB]
2026-Feb-25 07:12:51.662997
#37 140.3 Get:281 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 mesa-va-drivers amd64 23.2.1-1ubuntu3.1~22.04.3 [4100 kB]
2026-Feb-25 07:12:53.843012
#37 142.5 Get:282 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 mesa-vdpau-drivers amd64 23.2.1-1ubuntu3.1~22.04.3 [3820 kB]
2026-Feb-25 07:13:02.136575
#37 150.8 Get:283 http://archive.ubuntu.com/ubuntu jammy/main amd64 pkg-config amd64 0.29.2-1ubuntu3 [48.2 kB]
2026-Feb-25 07:13:02.928919
#37 151.6 Get:284 http://archive.ubuntu.com/ubuntu jammy/universe amd64 i965-va-driver amd64 2.4.1+dfsg1-1 [302 kB]
2026-Feb-25 07:13:03.989629
#37 152.7 Get:285 http://archive.ubuntu.com/ubuntu jammy/universe amd64 va-driver-all amd64 2.14.0-1 [3984 B]
2026-Feb-25 07:13:04.246823
#37 152.9 Get:286 http://archive.ubuntu.com/ubuntu jammy/main amd64 vdpau-driver-all amd64 1.4-3build2 [4510 B]
2026-Feb-25 07:13:04.504297
#37 153.2 Get:287 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 zlib1g-dev amd64 1:1.2.11.dfsg-2ubuntu9.2 [164 kB]
2026-Feb-25 07:13:04.870536
#37 153.5 Get:288 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libprotobuf-dev amd64 3.12.4-1ubuntu7.22.04.4 [1347 kB]
2026-Feb-25 07:13:05.963940
#37 154.6 Get:289 http://archive.ubuntu.com/ubuntu jammy/universe amd64 pocketsphinx-en-us all 0.8.0+real5prealpha+1-14ubuntu1 [27.6 MB]
2026-Feb-25 07:13:08.146360
#37 156.8 Get:290 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 protobuf-compiler amd64 3.12.4-1ubuntu7.22.04.4 [29.2 kB]
2026-Feb-25 07:13:08.297720
#37 157.0 debconf: delaying package configuration, since apt-utils is not installed
2026-Feb-25 07:13:08.520997
#37 157.0 Fetched 392 MB in 2min 27s (2670 kB/s)
2026-Feb-25 07:13:08.520997
#37 157.0 (Reading database ... 
(Reading database ... 5%
(Reading database ... 10%
(Reading database ... 15%
(Reading database ... 20%
(Reading database ... 25%
(Reading database ... 30%
(Reading database ... 35%
(Reading database ... 40%
(Reading database ... 45%
(Reading database ... 50%
(Reading database ... 55%
(Reading database ... 60%
(Reading database ... 65%
(Reading database ... 70%
(Reading database ... 75%
(Reading database ... 80%
(Reading database ... 85%
(Reading database ... 90%
(Reading database ... 95%
(Reading database ... 100%
(Reading database ... 14803 files and directories currently installed.)
2026-Feb-25 07:13:08.520997
#37 157.0 Preparing to unpack .../0-libc6-dev_2.35-0ubuntu3.13_amd64.deb ...
2026-Feb-25 07:13:08.520997
#37 157.0 Unpacking libc6-dev:amd64 (2.35-0ubuntu3.13) over (2.35-0ubuntu3.6) ...
2026-Feb-25 07:13:08.817716
#37 157.5 Preparing to unpack .../1-libc-dev-bin_2.35-0ubuntu3.13_amd64.deb ...
2026-Feb-25 07:13:08.924448
#37 157.5 Unpacking libc-dev-bin (2.35-0ubuntu3.13) over (2.35-0ubuntu3.6) ...
2026-Feb-25 07:13:08.924448
#37 157.5 Preparing to unpack .../2-libatomic1_12.3.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:08.924448
#37 157.5 Unpacking libatomic1:amd64 (12.3.0-1ubuntu1~22.04.3) over (12.3.0-1ubuntu1~22.04) ...
2026-Feb-25 07:13:08.924448
#37 157.5 Preparing to unpack .../3-libubsan1_12.3.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:08.924448
#37 157.5 Unpacking libubsan1:amd64 (12.3.0-1ubuntu1~22.04.3) over (12.3.0-1ubuntu1~22.04) ...
2026-Feb-25 07:13:08.924448
#37 157.6 Preparing to unpack .../4-libquadmath0_12.3.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:08.924448
#37 157.6 Unpacking libquadmath0:amd64 (12.3.0-1ubuntu1~22.04.3) over (12.3.0-1ubuntu1~22.04) ...
2026-Feb-25 07:13:08.924448
#37 157.6 Preparing to unpack .../5-liblsan0_12.3.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:09.036334
#37 157.6 Unpacking liblsan0:amd64 (12.3.0-1ubuntu1~22.04.3) over (12.3.0-1ubuntu1~22.04) ...
2026-Feb-25 07:13:09.036334
#37 157.6 Preparing to unpack .../6-libitm1_12.3.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:09.036334
#37 157.6 Unpacking libitm1:amd64 (12.3.0-1ubuntu1~22.04.3) over (12.3.0-1ubuntu1~22.04) ...
2026-Feb-25 07:13:09.036334
#37 157.7 Preparing to unpack .../7-libgomp1_12.3.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:09.036334
#37 157.7 Unpacking libgomp1:amd64 (12.3.0-1ubuntu1~22.04.3) over (12.3.0-1ubuntu1~22.04) ...
2026-Feb-25 07:13:09.036334
#37 157.7 Preparing to unpack .../8-gcc-12-base_12.3.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:09.036334
#37 157.7 Unpacking gcc-12-base:amd64 (12.3.0-1ubuntu1~22.04.3) over (12.3.0-1ubuntu1~22.04) ...
2026-Feb-25 07:13:09.036334
#37 157.7 Setting up gcc-12-base:amd64 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:09.149345
#37 157.7 (Reading database ... 
(Reading database ... 5%
(Reading database ... 10%
(Reading database ... 15%
(Reading database ... 20%
(Reading database ... 25%
(Reading database ... 30%
(Reading database ... 35%
(Reading database ... 40%
(Reading database ... 45%
(Reading database ... 50%
(Reading database ... 55%
(Reading database ... 60%
(Reading database ... 65%
(Reading database ... 70%
(Reading database ... 75%
(Reading database ... 80%
(Reading database ... 85%
(Reading database ... 90%
(Reading database ... 95%
(Reading database ... 100%
(Reading database ... 14803 files and directories currently installed.)
2026-Feb-25 07:13:09.149345
#37 157.7 Preparing to unpack .../libgcc-s1_12.3.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:09.149345
#37 157.7 Unpacking libgcc-s1:amd64 (12.3.0-1ubuntu1~22.04.3) over (12.3.0-1ubuntu1~22.04) ...
2026-Feb-25 07:13:09.149345
#37 157.8 Setting up libgcc-s1:amd64 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:09.149345
#37 157.8 (Reading database ... 
(Reading database ... 5%
(Reading database ... 10%
(Reading database ... 15%
(Reading database ... 20%
(Reading database ... 25%
(Reading database ... 30%
(Reading database ... 35%
(Reading database ... 40%
(Reading database ... 45%
(Reading database ... 50%
(Reading database ... 55%
(Reading database ... 60%
(Reading database ... 65%
(Reading database ... 70%
(Reading database ... 75%
(Reading database ... 80%
(Reading database ... 85%
(Reading database ... 90%
(Reading database ... 95%
(Reading database ... 100%
(Reading database ... 14803 files and directories currently installed.)
2026-Feb-25 07:13:09.149345
#37 157.8 Preparing to unpack .../libcc1-0_12.3.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:09.149345
#37 157.8 Unpacking libcc1-0:amd64 (12.3.0-1ubuntu1~22.04.3) over (12.3.0-1ubuntu1~22.04) ...
2026-Feb-25 07:13:09.149345
#37 157.8 Preparing to unpack .../libstdc++6_12.3.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:09.307999
#37 157.8 Unpacking libstdc++6:amd64 (12.3.0-1ubuntu1~22.04.3) over (12.3.0-1ubuntu1~22.04) ...
2026-Feb-25 07:13:09.307999
#37 157.9 Setting up libstdc++6:amd64 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:09.307999
#37 157.9 (Reading database ... 
(Reading database ... 5%
(Reading database ... 10%
(Reading database ... 15%
(Reading database ... 20%
(Reading database ... 25%
(Reading database ... 30%
(Reading database ... 35%
(Reading database ... 40%
(Reading database ... 45%
(Reading database ... 50%
(Reading database ... 55%
(Reading database ... 60%
(Reading database ... 65%
(Reading database ... 70%
(Reading database ... 75%
(Reading database ... 80%
(Reading database ... 85%
(Reading database ... 90%
(Reading database ... 95%
(Reading database ... 100%
(Reading database ... 14803 files and directories currently installed.)
2026-Feb-25 07:13:09.307999
#37 157.9 Preparing to unpack .../libc6_2.35-0ubuntu3.13_amd64.deb ...
2026-Feb-25 07:13:09.307999
#37 158.0 debconf: unable to initialize frontend: Dialog
2026-Feb-25 07:13:09.307999
#37 158.0 debconf: (TERM is not set, so the dialog frontend is not usable.)
2026-Feb-25 07:13:09.307999
#37 158.0 debconf: falling back to frontend: Readline
2026-Feb-25 07:13:09.504495
#37 158.0 Unpacking libc6:amd64 (2.35-0ubuntu3.13) over (2.35-0ubuntu3.6) ...
2026-Feb-25 07:13:09.683602
#37 158.4 Setting up libc6:amd64 (2.35-0ubuntu3.13) ...
2026-Feb-25 07:13:09.896324
#37 158.4 debconf: unable to initialize frontend: Dialog
2026-Feb-25 07:13:09.896324
#37 158.4 debconf: (TERM is not set, so the dialog frontend is not usable.)
2026-Feb-25 07:13:09.896324
#37 158.4 debconf: falling back to frontend: Readline
2026-Feb-25 07:13:10.808744
#37 159.5 (Reading database ...
2026-Feb-25 07:13:10.928921
(Reading database ... 5%
(Reading database ... 10%
(Reading database ... 15%
(Reading database ... 20%
(Reading database ... 25%
(Reading database ... 30%
(Reading database ... 35%
(Reading database ... 40%
(Reading database ... 45%
(Reading database ... 50%
(Reading database ... 55%
(Reading database ... 60%
(Reading database ... 65%
(Reading database ... 70%
(Reading database ... 75%
(Reading database ... 80%
(Reading database ... 85%
(Reading database ... 90%
(Reading database ... 95%
(Reading database ... 100%
(Reading database ... 14803 files and directories currently installed.)
2026-Feb-25 07:13:10.928921
#37 159.5 Preparing to unpack .../libssl3_3.0.2-0ubuntu1.21_amd64.deb ...
2026-Feb-25 07:13:10.928921
#37 159.5 Unpacking libssl3:amd64 (3.0.2-0ubuntu1.21) over (3.0.2-0ubuntu1.14) ...
2026-Feb-25 07:13:10.928921
#37 159.5 Setting up libssl3:amd64 (3.0.2-0ubuntu1.21) ...
2026-Feb-25 07:13:10.928921
#37 159.6 debconf: unable to initialize frontend: Dialog
2026-Feb-25 07:13:10.928921
#37 159.6 debconf: (TERM is not set, so the dialog frontend is not usable.)
2026-Feb-25 07:13:10.928921
#37 159.6 debconf: falling back to frontend: Readline
2026-Feb-25 07:13:11.047238
#37 159.6 Selecting previously unselected package libpython3.10-minimal:amd64.
2026-Feb-25 07:13:11.047238
#37 159.6 (Reading database ... 
(Reading database ... 5%
(Reading database ... 10%
(Reading database ... 15%
(Reading database ... 20%
(Reading database ... 25%
(Reading database ... 30%
(Reading database ... 35%
(Reading database ... 40%
(Reading database ... 45%
(Reading database ... 50%
(Reading database ... 55%
(Reading database ... 60%
(Reading database ... 65%
(Reading database ... 70%
(Reading database ... 75%
(Reading database ... 80%
(Reading database ... 85%
(Reading database ... 90%
(Reading database ... 95%
(Reading database ... 100%
(Reading database ... 14803 files and directories currently installed.)
2026-Feb-25 07:13:11.047238
#37 159.7 Preparing to unpack .../libpython3.10-minimal_3.10.12-1~22.04.14_amd64.deb ...
2026-Feb-25 07:13:11.047238
#37 159.7 Unpacking libpython3.10-minimal:amd64 (3.10.12-1~22.04.14) ...
2026-Feb-25 07:13:11.047238
#37 159.7 Selecting previously unselected package libexpat1:amd64.
2026-Feb-25 07:13:11.281410
#37 159.7 Preparing to unpack .../libexpat1_2.4.7-1ubuntu0.7_amd64.deb ...
2026-Feb-25 07:13:11.281410
#37 159.7 Unpacking libexpat1:amd64 (2.4.7-1ubuntu0.7) ...
2026-Feb-25 07:13:11.281410
#37 159.7 Selecting previously unselected package python3.10-minimal.
2026-Feb-25 07:13:11.281410
#37 159.7 Preparing to unpack .../python3.10-minimal_3.10.12-1~22.04.14_amd64.deb ...
2026-Feb-25 07:13:11.281410
#37 159.7 Unpacking python3.10-minimal (3.10.12-1~22.04.14) ...
2026-Feb-25 07:13:11.281410
#37 159.8 Setting up libpython3.10-minimal:amd64 (3.10.12-1~22.04.14) ...
2026-Feb-25 07:13:11.281410
#37 159.8 Setting up libexpat1:amd64 (2.4.7-1ubuntu0.7) ...
2026-Feb-25 07:13:11.281410
#37 159.8 Setting up python3.10-minimal (3.10.12-1~22.04.14) ...
2026-Feb-25 07:13:11.657336
#37 160.3 Selecting previously unselected package python3-minimal.
2026-Feb-25 07:13:11.657336
#37 160.3 (Reading database ...
2026-Feb-25 07:13:11.799927
(Reading database ... 5%
(Reading database ... 10%
(Reading database ... 15%
(Reading database ... 20%
(Reading database ... 25%
(Reading database ... 30%
(Reading database ... 35%
(Reading database ... 40%
(Reading database ... 45%
(Reading database ... 50%
(Reading database ... 55%
(Reading database ... 60%
(Reading database ... 65%
(Reading database ... 70%
(Reading database ... 75%
(Reading database ... 80%
(Reading database ... 85%
(Reading database ... 90%
(Reading database ... 95%
(Reading database ... 100%
(Reading database ... 15107 files and directories currently installed.)
2026-Feb-25 07:13:11.799927
#37 160.3 Preparing to unpack .../0-python3-minimal_3.10.6-1~22.04.1_amd64.deb ...
2026-Feb-25 07:13:11.799927
#37 160.3 Unpacking python3-minimal (3.10.6-1~22.04.1) ...
2026-Feb-25 07:13:11.799927
#37 160.4 Selecting previously unselected package media-types.
2026-Feb-25 07:13:11.799927
#37 160.4 Preparing to unpack .../1-media-types_7.0.0_all.deb ...
2026-Feb-25 07:13:11.799927
#37 160.4 Unpacking media-types (7.0.0) ...
2026-Feb-25 07:13:11.799927
#37 160.4 Selecting previously unselected package libmpdec3:amd64.
2026-Feb-25 07:13:11.799927
#37 160.4 Preparing to unpack .../2-libmpdec3_2.5.1-2build2_amd64.deb ...
2026-Feb-25 07:13:11.799927
#37 160.4 Unpacking libmpdec3:amd64 (2.5.1-2build2) ...
2026-Feb-25 07:13:11.799927
#37 160.4 Selecting previously unselected package libpython3.10-stdlib:amd64.
2026-Feb-25 07:13:11.799927
#37 160.4 Preparing to unpack .../3-libpython3.10-stdlib_3.10.12-1~22.04.14_amd64.deb ...
2026-Feb-25 07:13:11.799927
#37 160.4 Unpacking libpython3.10-stdlib:amd64 (3.10.12-1~22.04.14) ...
2026-Feb-25 07:13:11.799927
#37 160.5 Selecting previously unselected package python3.10.
2026-Feb-25 07:13:11.985408
#37 160.5 Preparing to unpack .../4-python3.10_3.10.12-1~22.04.14_amd64.deb ...
2026-Feb-25 07:13:11.985408
#37 160.5 Unpacking python3.10 (3.10.12-1~22.04.14) ...
2026-Feb-25 07:13:11.985408
#37 160.5 Selecting previously unselected package libpython3-stdlib:amd64.
2026-Feb-25 07:13:11.985408
#37 160.5 Preparing to unpack .../5-libpython3-stdlib_3.10.6-1~22.04.1_amd64.deb ...
2026-Feb-25 07:13:11.985408
#37 160.5 Unpacking libpython3-stdlib:amd64 (3.10.6-1~22.04.1) ...
2026-Feb-25 07:13:11.985408
#37 160.5 Setting up python3-minimal (3.10.6-1~22.04.1) ...
2026-Feb-25 07:13:11.985408
#37 160.7 Selecting previously unselected package python3.
2026-Feb-25 07:13:11.985408
#37 160.7 (Reading database ...
2026-Feb-25 07:13:12.094313
(Reading database ... 5%
(Reading database ... 10%
(Reading database ... 15%
(Reading database ... 20%
(Reading database ... 25%
(Reading database ... 30%
(Reading database ... 35%
(Reading database ... 40%
(Reading database ... 45%
(Reading database ... 50%
(Reading database ... 55%
(Reading database ... 60%
(Reading database ... 65%
(Reading database ... 70%
(Reading database ... 75%
(Reading database ... 80%
(Reading database ... 85%
(Reading database ... 90%
(Reading database ... 95%
(Reading database ... 100%
(Reading database ... 15508 files and directories currently installed.)
2026-Feb-25 07:13:12.094313
#37 160.7 Preparing to unpack .../000-python3_3.10.6-1~22.04.1_amd64.deb ...
2026-Feb-25 07:13:12.094313
#37 160.7 Unpacking python3 (3.10.6-1~22.04.1) ...
2026-Feb-25 07:13:12.094313
#37 160.7 Selecting previously unselected package libapparmor1:amd64.
2026-Feb-25 07:13:12.094313
#37 160.7 Preparing to unpack .../001-libapparmor1_3.0.4-2ubuntu2.5_amd64.deb ...
2026-Feb-25 07:13:12.094313
#37 160.7 Unpacking libapparmor1:amd64 (3.0.4-2ubuntu2.5) ...
2026-Feb-25 07:13:12.094313
#37 160.7 Selecting previously unselected package libdbus-1-3:amd64.
2026-Feb-25 07:13:12.094313
#37 160.7 Preparing to unpack .../002-libdbus-1-3_1.12.20-2ubuntu4.1_amd64.deb ...
2026-Feb-25 07:13:12.094313
#37 160.7 Unpacking libdbus-1-3:amd64 (1.12.20-2ubuntu4.1) ...
2026-Feb-25 07:13:12.094313
#37 160.7 Selecting previously unselected package dbus.
2026-Feb-25 07:13:12.094313
#37 160.7 Preparing to unpack .../003-dbus_1.12.20-2ubuntu4.1_amd64.deb ...
2026-Feb-25 07:13:12.094313
#37 160.7 Unpacking dbus (1.12.20-2ubuntu4.1) ...
2026-Feb-25 07:13:12.094313
#37 160.8 Selecting previously unselected package libmd0:amd64.
2026-Feb-25 07:13:12.206392
#37 160.8 Preparing to unpack .../004-libmd0_1.0.4-1build1_amd64.deb ...
2026-Feb-25 07:13:12.206392
#37 160.8 Unpacking libmd0:amd64 (1.0.4-1build1) ...
2026-Feb-25 07:13:12.206392
#37 160.8 Selecting previously unselected package libbsd0:amd64.
2026-Feb-25 07:13:12.206392
#37 160.8 Preparing to unpack .../005-libbsd0_0.11.5-1_amd64.deb ...
2026-Feb-25 07:13:12.206392
#37 160.8 Unpacking libbsd0:amd64 (0.11.5-1) ...
2026-Feb-25 07:13:12.206392
#37 160.8 Selecting previously unselected package libelf1:amd64.
2026-Feb-25 07:13:12.206392
#37 160.8 Preparing to unpack .../006-libelf1_0.186-1ubuntu0.1_amd64.deb ...
2026-Feb-25 07:13:12.206392
#37 160.8 Unpacking libelf1:amd64 (0.186-1ubuntu0.1) ...
2026-Feb-25 07:13:12.206392
#37 160.8 Selecting previously unselected package libfribidi0:amd64.
2026-Feb-25 07:13:12.206392
#37 160.8 Preparing to unpack .../007-libfribidi0_1.0.8-2ubuntu3.1_amd64.deb ...
2026-Feb-25 07:13:12.206392
#37 160.8 Unpacking libfribidi0:amd64 (1.0.8-2ubuntu3.1) ...
2026-Feb-25 07:13:12.206392
#37 160.8 Selecting previously unselected package libglib2.0-0:amd64.
2026-Feb-25 07:13:12.206392
#37 160.8 Preparing to unpack .../008-libglib2.0-0_2.72.4-0ubuntu2.9_amd64.deb ...
2026-Feb-25 07:13:12.206392
#37 160.8 Unpacking libglib2.0-0:amd64 (2.72.4-0ubuntu2.9) ...
2026-Feb-25 07:13:12.206392
#37 160.9 Selecting previously unselected package libglib2.0-data.
2026-Feb-25 07:13:12.356203
#37 160.9 Preparing to unpack .../009-libglib2.0-data_2.72.4-0ubuntu2.9_all.deb ...
2026-Feb-25 07:13:12.356203
#37 160.9 Unpacking libglib2.0-data (2.72.4-0ubuntu2.9) ...
2026-Feb-25 07:13:12.356203
#37 160.9 Selecting previously unselected package libicu70:amd64.
2026-Feb-25 07:13:12.356203
#37 160.9 Preparing to unpack .../010-libicu70_70.1-2_amd64.deb ...
2026-Feb-25 07:13:12.356203
#37 160.9 Unpacking libicu70:amd64 (70.1-2) ...
2026-Feb-25 07:13:12.356203
#37 161.0 Selecting previously unselected package libslang2:amd64.
2026-Feb-25 07:13:12.470235
#37 161.0 Preparing to unpack .../011-libslang2_2.3.2-5build4_amd64.deb ...
2026-Feb-25 07:13:12.470235
#37 161.0 Unpacking libslang2:amd64 (2.3.2-5build4) ...
2026-Feb-25 07:13:12.470235
#37 161.0 Selecting previously unselected package libxml2:amd64.
2026-Feb-25 07:13:12.470235
#37 161.1 Preparing to unpack .../012-libxml2_2.9.13+dfsg-1ubuntu0.11_amd64.deb ...
2026-Feb-25 07:13:12.470235
#37 161.1 Unpacking libxml2:amd64 (2.9.13+dfsg-1ubuntu0.11) ...
2026-Feb-25 07:13:12.470235
#37 161.1 Selecting previously unselected package libyaml-0-2:amd64.
2026-Feb-25 07:13:12.470235
#37 161.1 Preparing to unpack .../013-libyaml-0-2_0.2.2-1build2_amd64.deb ...
2026-Feb-25 07:13:12.470235
#37 161.1 Unpacking libyaml-0-2:amd64 (0.2.2-1build2) ...
2026-Feb-25 07:13:12.470235
#37 161.1 Selecting previously unselected package python3-pkg-resources.
2026-Feb-25 07:13:12.470235
#37 161.1 Preparing to unpack .../014-python3-pkg-resources_59.6.0-1.2ubuntu0.22.04.3_all.deb ...
2026-Feb-25 07:13:12.470235
#37 161.1 Unpacking python3-pkg-resources (59.6.0-1.2ubuntu0.22.04.3) ...
2026-Feb-25 07:13:12.470235
#37 161.1 Selecting previously unselected package python3-yaml.
2026-Feb-25 07:13:12.470235
#37 161.1 Preparing to unpack .../015-python3-yaml_5.4.1-1ubuntu1_amd64.deb ...
2026-Feb-25 07:13:12.470235
#37 161.1 Unpacking python3-yaml (5.4.1-1ubuntu1) ...
2026-Feb-25 07:13:12.470235
#37 161.1 Selecting previously unselected package shared-mime-info.
2026-Feb-25 07:13:12.583599
#37 161.1 Preparing to unpack .../016-shared-mime-info_2.1-2_amd64.deb ...
2026-Feb-25 07:13:12.583599
#37 161.1 Unpacking shared-mime-info (2.1-2) ...
2026-Feb-25 07:13:12.583599
#37 161.2 Selecting previously unselected package ucf.
2026-Feb-25 07:13:12.583599
#37 161.2 Preparing to unpack .../017-ucf_3.0043_all.deb ...
2026-Feb-25 07:13:12.583599
#37 161.2 Moving old data out of the way
2026-Feb-25 07:13:12.583599
#37 161.2 Unpacking ucf (3.0043) ...
2026-Feb-25 07:13:12.583599
#37 161.2 Selecting previously unselected package xdg-user-dirs.
2026-Feb-25 07:13:12.583599
#37 161.2 Preparing to unpack .../018-xdg-user-dirs_0.17-2ubuntu4_amd64.deb ...
2026-Feb-25 07:13:12.583599
#37 161.2 Unpacking xdg-user-dirs (0.17-2ubuntu4) ...
2026-Feb-25 07:13:12.583599
#37 161.3 Selecting previously unselected package xkb-data.
2026-Feb-25 07:13:12.695374
#37 161.3 Preparing to unpack .../019-xkb-data_2.33-1_all.deb ...
2026-Feb-25 07:13:12.695374
#37 161.3 Unpacking xkb-data (2.33-1) ...
2026-Feb-25 07:13:12.695374
#37 161.3 Selecting previously unselected package libdrm-common.
2026-Feb-25 07:13:12.695374
#37 161.3 Preparing to unpack .../020-libdrm-common_2.4.113-2~ubuntu0.22.04.1_all.deb ...
2026-Feb-25 07:13:12.695374
#37 161.3 Unpacking libdrm-common (2.4.113-2~ubuntu0.22.04.1) ...
2026-Feb-25 07:13:12.695374
#37 161.3 Selecting previously unselected package libdrm2:amd64.
2026-Feb-25 07:13:12.695374
#37 161.3 Preparing to unpack .../021-libdrm2_2.4.113-2~ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:13:12.695374
#37 161.3 Unpacking libdrm2:amd64 (2.4.113-2~ubuntu0.22.04.1) ...
2026-Feb-25 07:13:12.695374
#37 161.4 Selecting previously unselected package libedit2:amd64.
2026-Feb-25 07:13:12.796806
#37 161.4 Preparing to unpack .../022-libedit2_3.1-20210910-1build1_amd64.deb ...
2026-Feb-25 07:13:12.796806
#37 161.4 Unpacking libedit2:amd64 (3.1-20210910-1build1) ...
2026-Feb-25 07:13:12.796806
#37 161.4 Selecting previously unselected package libnghttp2-14:amd64.
2026-Feb-25 07:13:12.796806
#37 161.4 Preparing to unpack .../023-libnghttp2-14_1.43.0-1ubuntu0.2_amd64.deb ...
2026-Feb-25 07:13:12.796806
#37 161.4 Unpacking libnghttp2-14:amd64 (1.43.0-1ubuntu0.2) ...
2026-Feb-25 07:13:12.796806
#37 161.4 Selecting previously unselected package libnuma1:amd64.
2026-Feb-25 07:13:12.796806
#37 161.4 Preparing to unpack .../024-libnuma1_2.0.14-3ubuntu2_amd64.deb ...
2026-Feb-25 07:13:12.796806
#37 161.4 Unpacking libnuma1:amd64 (2.0.14-3ubuntu2) ...
2026-Feb-25 07:13:12.796806
#37 161.4 Selecting previously unselected package libpipeline1:amd64.
2026-Feb-25 07:13:12.796806
#37 161.4 Preparing to unpack .../025-libpipeline1_1.5.5-1_amd64.deb ...
2026-Feb-25 07:13:12.796806
#37 161.4 Unpacking libpipeline1:amd64 (1.5.5-1) ...
2026-Feb-25 07:13:12.796806
#37 161.4 Selecting previously unselected package libpng16-16:amd64.
2026-Feb-25 07:13:12.796806
#37 161.4 Preparing to unpack .../026-libpng16-16_1.6.37-3ubuntu0.4_amd64.deb ...
2026-Feb-25 07:13:12.796806
#37 161.4 Unpacking libpng16-16:amd64 (1.6.37-3ubuntu0.4) ...
2026-Feb-25 07:13:12.796806
#37 161.5 Selecting previously unselected package libpsl5:amd64.
2026-Feb-25 07:13:12.796806
#37 161.5 Preparing to unpack .../027-libpsl5_0.21.0-1.2build2_amd64.deb ...
2026-Feb-25 07:13:12.942691
#37 161.5 Unpacking libpsl5:amd64 (0.21.0-1.2build2) ...
2026-Feb-25 07:13:12.942691
#37 161.5 Selecting previously unselected package libusb-1.0-0:amd64.
2026-Feb-25 07:13:12.942691
#37 161.5 Preparing to unpack .../028-libusb-1.0-0_2%3a1.0.25-1ubuntu2_amd64.deb ...
2026-Feb-25 07:13:12.942691
#37 161.5 Unpacking libusb-1.0-0:amd64 (2:1.0.25-1ubuntu2) ...
2026-Feb-25 07:13:12.942691
#37 161.5 Selecting previously unselected package libxau6:amd64.
2026-Feb-25 07:13:12.942691
#37 161.5 Preparing to unpack .../029-libxau6_1%3a1.0.9-1build5_amd64.deb ...
2026-Feb-25 07:13:12.942691
#37 161.5 Unpacking libxau6:amd64 (1:1.0.9-1build5) ...
2026-Feb-25 07:13:12.942691
#37 161.5 Selecting previously unselected package libxdmcp6:amd64.
2026-Feb-25 07:13:12.942691
#37 161.5 Preparing to unpack .../030-libxdmcp6_1%3a1.1.3-0ubuntu5_amd64.deb ...
2026-Feb-25 07:13:12.942691
#37 161.5 Unpacking libxdmcp6:amd64 (1:1.1.3-0ubuntu5) ...
2026-Feb-25 07:13:12.942691
#37 161.5 Selecting previously unselected package libxcb1:amd64.
2026-Feb-25 07:13:12.942691
#37 161.5 Preparing to unpack .../031-libxcb1_1.14-3ubuntu3_amd64.deb ...
2026-Feb-25 07:13:12.942691
#37 161.5 Unpacking libxcb1:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:13:12.942691
#37 161.6 Selecting previously unselected package libx11-data.
2026-Feb-25 07:13:12.942691
#37 161.6 Preparing to unpack .../032-libx11-data_2%3a1.7.5-1ubuntu0.3_all.deb ...
2026-Feb-25 07:13:12.942691
#37 161.6 Unpacking libx11-data (2:1.7.5-1ubuntu0.3) ...
2026-Feb-25 07:13:12.942691
#37 161.6 Selecting previously unselected package libx11-6:amd64.
2026-Feb-25 07:13:13.053803
#37 161.6 Preparing to unpack .../033-libx11-6_2%3a1.7.5-1ubuntu0.3_amd64.deb ...
2026-Feb-25 07:13:13.053803
#37 161.6 Unpacking libx11-6:amd64 (2:1.7.5-1ubuntu0.3) ...
2026-Feb-25 07:13:13.053803
#37 161.6 Selecting previously unselected package libxext6:amd64.
2026-Feb-25 07:13:13.053803
#37 161.6 Preparing to unpack .../034-libxext6_2%3a1.3.4-1build1_amd64.deb ...
2026-Feb-25 07:13:13.053803
#37 161.6 Unpacking libxext6:amd64 (2:1.3.4-1build1) ...
2026-Feb-25 07:13:13.053803
#37 161.7 Selecting previously unselected package publicsuffix.
2026-Feb-25 07:13:13.053803
#37 161.7 Preparing to unpack .../035-publicsuffix_20211207.1025-1_all.deb ...
2026-Feb-25 07:13:13.053803
#37 161.7 Unpacking publicsuffix (20211207.1025-1) ...
2026-Feb-25 07:13:13.053803
#37 161.7 Selecting previously unselected package alsa-topology-conf.
2026-Feb-25 07:13:13.053803
#37 161.7 Preparing to unpack .../036-alsa-topology-conf_1.2.5.1-2_all.deb ...
2026-Feb-25 07:13:13.053803
#37 161.7 Unpacking alsa-topology-conf (1.2.5.1-2) ...
2026-Feb-25 07:13:13.053803
#37 161.7 Selecting previously unselected package libasound2-data.
2026-Feb-25 07:13:13.053803
#37 161.7 Preparing to unpack .../037-libasound2-data_1.2.6.1-1ubuntu1.1_all.deb ...
2026-Feb-25 07:13:13.053803
#37 161.7 Unpacking libasound2-data (1.2.6.1-1ubuntu1.1) ...
2026-Feb-25 07:13:13.053803
#37 161.7 Selecting previously unselected package libasound2:amd64.
2026-Feb-25 07:13:13.159680
#37 161.7 Preparing to unpack .../038-libasound2_1.2.6.1-1ubuntu1.1_amd64.deb ...
2026-Feb-25 07:13:13.159680
#37 161.7 Unpacking libasound2:amd64 (1.2.6.1-1ubuntu1.1) ...
2026-Feb-25 07:13:13.159680
#37 161.7 Selecting previously unselected package alsa-ucm-conf.
2026-Feb-25 07:13:13.159680
#37 161.7 Preparing to unpack .../039-alsa-ucm-conf_1.2.6.3-1ubuntu1.12_all.deb ...
2026-Feb-25 07:13:13.159680
#37 161.7 Unpacking alsa-ucm-conf (1.2.6.3-1ubuntu1.12) ...
2026-Feb-25 07:13:13.159680
#37 161.8 Selecting previously unselected package binfmt-support.
2026-Feb-25 07:13:13.335749
#37 161.8 Preparing to unpack .../040-binfmt-support_2.2.1-2_amd64.deb ...
2026-Feb-25 07:13:13.335749
#37 161.8 Unpacking binfmt-support (2.2.1-2) ...
2026-Feb-25 07:13:13.335749
#37 161.9 Selecting previously unselected package libllvm14:amd64.
2026-Feb-25 07:13:13.335749
#37 161.9 Preparing to unpack .../041-libllvm14_1%3a14.0.0-1ubuntu1.1_amd64.deb ...
2026-Feb-25 07:13:13.335749
#37 161.9 Unpacking libllvm14:amd64 (1:14.0.0-1ubuntu1.1) ...
2026-Feb-25 07:13:13.528914
#37 162.2 Selecting previously unselected package libclang-cpp14.
2026-Feb-25 07:13:13.683585
#37 162.2 Preparing to unpack .../042-libclang-cpp14_1%3a14.0.0-1ubuntu1.1_amd64.deb ...
2026-Feb-25 07:13:13.683585
#37 162.2 Unpacking libclang-cpp14 (1:14.0.0-1ubuntu1.1) ...
2026-Feb-25 07:13:13.712442
#37 162.4 Preparing to unpack .../043-g++-11_11.4.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:13.848412
#37 162.4 Unpacking g++-11 (11.4.0-1ubuntu1~22.04.3) over (11.4.0-1ubuntu1~22.04) ...
2026-Feb-25 07:13:13.848412
#37 162.5 Preparing to unpack .../044-gcc-11_11.4.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:14.001362
#37 162.5 Unpacking gcc-11 (11.4.0-1ubuntu1~22.04.3) over (11.4.0-1ubuntu1~22.04) ...
2026-Feb-25 07:13:14.102498
#37 162.8 Preparing to unpack .../045-cpp-11_11.4.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:14.244174
#37 162.8 Unpacking cpp-11 (11.4.0-1ubuntu1~22.04.3) over (11.4.0-1ubuntu1~22.04) ...
2026-Feb-25 07:13:14.244174
#37 162.9 Preparing to unpack .../046-libasan6_11.4.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:14.451283
#37 162.9 Unpacking libasan6:amd64 (11.4.0-1ubuntu1~22.04.3) over (11.4.0-1ubuntu1~22.04) ...
2026-Feb-25 07:13:14.451283
#37 163.0 Preparing to unpack .../047-libstdc++-11-dev_11.4.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:14.451283
#37 163.0 Unpacking libstdc++-11-dev:amd64 (11.4.0-1ubuntu1~22.04.3) over (11.4.0-1ubuntu1~22.04) ...
2026-Feb-25 07:13:14.980264
#37 163.7 Preparing to unpack .../048-libgcc-11-dev_11.4.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:15.133931
#37 163.7 Unpacking libgcc-11-dev:amd64 (11.4.0-1ubuntu1~22.04.3) over (11.4.0-1ubuntu1~22.04) ...
2026-Feb-25 07:13:15.155034
#37 163.8 Preparing to unpack .../049-libtsan0_11.4.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:15.269270
#37 163.8 Unpacking libtsan0:amd64 (11.4.0-1ubuntu1~22.04.3) over (11.4.0-1ubuntu1~22.04) ...
2026-Feb-25 07:13:15.269270
#37 163.9 Preparing to unpack .../050-gcc-11-base_11.4.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:15.269270
#37 163.9 Unpacking gcc-11-base:amd64 (11.4.0-1ubuntu1~22.04.3) over (11.4.0-1ubuntu1~22.04) ...
2026-Feb-25 07:13:15.269270
#37 163.9 Selecting previously unselected package libgc1:amd64.
2026-Feb-25 07:13:15.269270
#37 163.9 Preparing to unpack .../051-libgc1_1%3a8.0.6-1.1build1_amd64.deb ...
2026-Feb-25 07:13:15.269270
#37 163.9 Unpacking libgc1:amd64 (1:8.0.6-1.1build1) ...
2026-Feb-25 07:13:15.269270
#37 163.9 Selecting previously unselected package libobjc4:amd64.
2026-Feb-25 07:13:15.269270
#37 163.9 Preparing to unpack .../052-libobjc4_12.3.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:15.269270
#37 163.9 Unpacking libobjc4:amd64 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:15.269270
#37 163.9 Selecting previously unselected package libobjc-11-dev:amd64.
2026-Feb-25 07:13:15.395085
#37 163.9 Preparing to unpack .../053-libobjc-11-dev_11.4.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:15.395085
#37 163.9 Unpacking libobjc-11-dev:amd64 (11.4.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:15.395085
#37 164.0 Selecting previously unselected package libc6-i386.
2026-Feb-25 07:13:15.395085
#37 164.0 Preparing to unpack .../054-libc6-i386_2.35-0ubuntu3.13_amd64.deb ...
2026-Feb-25 07:13:15.395085
#37 164.0 Unpacking libc6-i386 (2.35-0ubuntu3.13) ...
2026-Feb-25 07:13:15.395085
#37 164.1 Selecting previously unselected package lib32gcc-s1.
2026-Feb-25 07:13:15.596485
#37 164.1 Preparing to unpack .../055-lib32gcc-s1_12.3.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:15.596485
#37 164.1 Unpacking lib32gcc-s1 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:15.596485
#37 164.1 Selecting previously unselected package lib32stdc++6.
2026-Feb-25 07:13:15.596485
#37 164.1 Preparing to unpack .../056-lib32stdc++6_12.3.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:15.596485
#37 164.1 Unpacking lib32stdc++6 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:15.596485
#37 164.1 Selecting previously unselected package libclang-common-14-dev.
2026-Feb-25 07:13:15.596485
#37 164.1 Preparing to unpack .../057-libclang-common-14-dev_1%3a14.0.0-1ubuntu1.1_amd64.deb ...
2026-Feb-25 07:13:15.596485
#37 164.1 Unpacking libclang-common-14-dev (1:14.0.0-1ubuntu1.1) ...
2026-Feb-25 07:13:15.710738
#37 164.4 Selecting previously unselected package llvm-14-linker-tools.
2026-Feb-25 07:13:15.856844
#37 164.4 Preparing to unpack .../058-llvm-14-linker-tools_1%3a14.0.0-1ubuntu1.1_amd64.deb ...
2026-Feb-25 07:13:15.856844
#37 164.4 Unpacking llvm-14-linker-tools (1:14.0.0-1ubuntu1.1) ...
2026-Feb-25 07:13:15.856844
#37 164.4 Selecting previously unselected package libclang1-14.
2026-Feb-25 07:13:15.856844
#37 164.4 Preparing to unpack .../059-libclang1-14_1%3a14.0.0-1ubuntu1.1_amd64.deb ...
2026-Feb-25 07:13:15.856844
#37 164.4 Unpacking libclang1-14 (1:14.0.0-1ubuntu1.1) ...
2026-Feb-25 07:13:15.856844
#37 164.5 Selecting previously unselected package clang-14.
2026-Feb-25 07:13:15.968306
#37 164.5 Preparing to unpack .../060-clang-14_1%3a14.0.0-1ubuntu1.1_amd64.deb ...
2026-Feb-25 07:13:15.968306
#37 164.5 Unpacking clang-14 (1:14.0.0-1ubuntu1.1) ...
2026-Feb-25 07:13:15.968306
#37 164.6 Selecting previously unselected package clang.
2026-Feb-25 07:13:15.968306
#37 164.6 Preparing to unpack .../061-clang_1%3a14.0-55~exp2_amd64.deb ...
2026-Feb-25 07:13:15.968306
#37 164.6 Unpacking clang (1:14.0-55~exp2) ...
2026-Feb-25 07:13:15.968306
#37 164.6 Selecting previously unselected package libbrotli1:amd64.
2026-Feb-25 07:13:15.968306
#37 164.6 Preparing to unpack .../062-libbrotli1_1.0.9-2build6_amd64.deb ...
2026-Feb-25 07:13:15.968306
#37 164.6 Unpacking libbrotli1:amd64 (1.0.9-2build6) ...
2026-Feb-25 07:13:15.968306
#37 164.6 Selecting previously unselected package librtmp1:amd64.
2026-Feb-25 07:13:15.968306
#37 164.6 Preparing to unpack .../063-librtmp1_2.4+20151223.gitfa8646d.1-2build4_amd64.deb ...
2026-Feb-25 07:13:15.968306
#37 164.6 Unpacking librtmp1:amd64 (2.4+20151223.gitfa8646d.1-2build4) ...
2026-Feb-25 07:13:15.968306
#37 164.6 Selecting previously unselected package libssh-4:amd64.
2026-Feb-25 07:13:15.968306
#37 164.6 Preparing to unpack .../064-libssh-4_0.9.6-2ubuntu0.22.04.6_amd64.deb ...
2026-Feb-25 07:13:15.968306
#37 164.6 Unpacking libssh-4:amd64 (0.9.6-2ubuntu0.22.04.6) ...
2026-Feb-25 07:13:15.968306
#37 164.6 Selecting previously unselected package libcurl4:amd64.
2026-Feb-25 07:13:16.071256
#37 164.6 Preparing to unpack .../065-libcurl4_7.81.0-1ubuntu1.22_amd64.deb ...
2026-Feb-25 07:13:16.071256
#37 164.6 Unpacking libcurl4:amd64 (7.81.0-1ubuntu1.22) ...
2026-Feb-25 07:13:16.071256
#37 164.7 Selecting previously unselected package curl.
2026-Feb-25 07:13:16.071256
#37 164.7 Preparing to unpack .../066-curl_7.81.0-1ubuntu1.22_amd64.deb ...
2026-Feb-25 07:13:16.071256
#37 164.7 Unpacking curl (7.81.0-1ubuntu1.22) ...
2026-Feb-25 07:13:16.071256
#37 164.7 Selecting previously unselected package libaom3:amd64.
2026-Feb-25 07:13:16.071256
#37 164.7 Preparing to unpack .../067-libaom3_3.3.0-1ubuntu0.1_amd64.deb ...
2026-Feb-25 07:13:16.071256
#37 164.7 Unpacking libaom3:amd64 (3.3.0-1ubuntu0.1) ...
2026-Feb-25 07:13:16.071256
#37 164.7 Selecting previously unselected package libva2:amd64.
2026-Feb-25 07:13:16.071256
#37 164.7 Preparing to unpack .../068-libva2_2.14.0-1_amd64.deb ...
2026-Feb-25 07:13:16.071256
#37 164.7 Unpacking libva2:amd64 (2.14.0-1) ...
2026-Feb-25 07:13:16.071256
#37 164.7 Selecting previously unselected package libmfx1:amd64.
2026-Feb-25 07:13:16.172509
#37 164.7 Preparing to unpack .../069-libmfx1_22.3.0-1_amd64.deb ...
2026-Feb-25 07:13:16.172509
#37 164.7 Unpacking libmfx1:amd64 (22.3.0-1) ...
2026-Feb-25 07:13:16.172509
#37 164.8 Selecting previously unselected package libva-drm2:amd64.
2026-Feb-25 07:13:16.172509
#37 164.8 Preparing to unpack .../070-libva-drm2_2.14.0-1_amd64.deb ...
2026-Feb-25 07:13:16.172509
#37 164.8 Unpacking libva-drm2:amd64 (2.14.0-1) ...
2026-Feb-25 07:13:16.172509
#37 164.8 Selecting previously unselected package libxfixes3:amd64.
2026-Feb-25 07:13:16.172509
#37 164.8 Preparing to unpack .../071-libxfixes3_1%3a6.0.0-1_amd64.deb ...
2026-Feb-25 07:13:16.285672
#37 164.8 Unpacking libxfixes3:amd64 (1:6.0.0-1) ...
2026-Feb-25 07:13:16.285672
#37 164.9 Selecting previously unselected package libva-x11-2:amd64.
2026-Feb-25 07:13:16.285672
#37 164.9 Preparing to unpack .../072-libva-x11-2_2.14.0-1_amd64.deb ...
2026-Feb-25 07:13:16.285672
#37 164.9 Unpacking libva-x11-2:amd64 (2.14.0-1) ...
2026-Feb-25 07:13:16.285672
#37 164.9 Selecting previously unselected package libvdpau1:amd64.
2026-Feb-25 07:13:16.285672
#37 164.9 Preparing to unpack .../073-libvdpau1_1.4-3build2_amd64.deb ...
2026-Feb-25 07:13:16.285672
#37 164.9 Unpacking libvdpau1:amd64 (1.4-3build2) ...
2026-Feb-25 07:13:16.285672
#37 164.9 Selecting previously unselected package ocl-icd-libopencl1:amd64.
2026-Feb-25 07:13:16.285672
#37 164.9 Preparing to unpack .../074-ocl-icd-libopencl1_2.2.14-3_amd64.deb ...
2026-Feb-25 07:13:16.285672
#37 164.9 Unpacking ocl-icd-libopencl1:amd64 (2.2.14-3) ...
2026-Feb-25 07:13:16.285672
#37 164.9 Selecting previously unselected package libavutil56:amd64.
2026-Feb-25 07:13:16.285672
#37 164.9 Preparing to unpack .../075-libavutil56_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:13:16.285672
#37 164.9 Unpacking libavutil56:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:16.285672
#37 165.0 Selecting previously unselected package libfreetype6:amd64.
2026-Feb-25 07:13:16.396112
#37 165.0 Preparing to unpack .../076-libfreetype6_2.11.1+dfsg-1ubuntu0.3_amd64.deb ...
2026-Feb-25 07:13:16.396112
#37 165.0 Unpacking libfreetype6:amd64 (2.11.1+dfsg-1ubuntu0.3) ...
2026-Feb-25 07:13:16.396112
#37 165.0 Selecting previously unselected package fonts-dejavu-core.
2026-Feb-25 07:13:16.396112
#37 165.0 Preparing to unpack .../077-fonts-dejavu-core_2.37-2build1_all.deb ...
2026-Feb-25 07:13:16.396112
#37 165.0 Unpacking fonts-dejavu-core (2.37-2build1) ...
2026-Feb-25 07:13:16.396112
#37 165.1 Selecting previously unselected package fontconfig-config.
2026-Feb-25 07:13:16.508970
#37 165.1 Preparing to unpack .../078-fontconfig-config_2.13.1-4.2ubuntu5_all.deb ...
2026-Feb-25 07:13:16.508970
#37 165.1 Unpacking fontconfig-config (2.13.1-4.2ubuntu5) ...
2026-Feb-25 07:13:16.508970
#37 165.1 Selecting previously unselected package libfontconfig1:amd64.
2026-Feb-25 07:13:16.508970
#37 165.1 Preparing to unpack .../079-libfontconfig1_2.13.1-4.2ubuntu5_amd64.deb ...
2026-Feb-25 07:13:16.508970
#37 165.1 Unpacking libfontconfig1:amd64 (2.13.1-4.2ubuntu5) ...
2026-Feb-25 07:13:16.508970
#37 165.1 Selecting previously unselected package libpixman-1-0:amd64.
2026-Feb-25 07:13:16.508970
#37 165.1 Preparing to unpack .../080-libpixman-1-0_0.40.0-1ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:13:16.508970
#37 165.1 Unpacking libpixman-1-0:amd64 (0.40.0-1ubuntu0.22.04.1) ...
2026-Feb-25 07:13:16.508970
#37 165.1 Selecting previously unselected package libxcb-render0:amd64.
2026-Feb-25 07:13:16.508970
#37 165.1 Preparing to unpack .../081-libxcb-render0_1.14-3ubuntu3_amd64.deb ...
2026-Feb-25 07:13:16.508970
#37 165.1 Unpacking libxcb-render0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:13:16.508970
#37 165.2 Selecting previously unselected package libxcb-shm0:amd64.
2026-Feb-25 07:13:16.508970
#37 165.2 Preparing to unpack .../082-libxcb-shm0_1.14-3ubuntu3_amd64.deb ...
2026-Feb-25 07:13:16.508970
#37 165.2 Unpacking libxcb-shm0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:13:16.508970
#37 165.2 Selecting previously unselected package libxrender1:amd64.
2026-Feb-25 07:13:16.620674
#37 165.2 Preparing to unpack .../083-libxrender1_1%3a0.9.10-1build4_amd64.deb ...
2026-Feb-25 07:13:16.620674
#37 165.2 Unpacking libxrender1:amd64 (1:0.9.10-1build4) ...
2026-Feb-25 07:13:16.620674
#37 165.2 Selecting previously unselected package libcairo2:amd64.
2026-Feb-25 07:13:16.620674
#37 165.2 Preparing to unpack .../084-libcairo2_1.16.0-5ubuntu2_amd64.deb ...
2026-Feb-25 07:13:16.620674
#37 165.2 Unpacking libcairo2:amd64 (1.16.0-5ubuntu2) ...
2026-Feb-25 07:13:16.620674
#37 165.2 Selecting previously unselected package libcodec2-1.0:amd64.
2026-Feb-25 07:13:16.620674
#37 165.2 Preparing to unpack .../085-libcodec2-1.0_1.0.1-3_amd64.deb ...
2026-Feb-25 07:13:16.620674
#37 165.2 Unpacking libcodec2-1.0:amd64 (1.0.1-3) ...
2026-Feb-25 07:13:16.620674
#37 165.3 Selecting previously unselected package libdav1d5:amd64.
2026-Feb-25 07:13:16.726435
#37 165.3 Preparing to unpack .../086-libdav1d5_0.9.2-1_amd64.deb ...
2026-Feb-25 07:13:16.726435
#37 165.3 Unpacking libdav1d5:amd64 (0.9.2-1) ...
2026-Feb-25 07:13:16.726435
#37 165.3 Selecting previously unselected package libgsm1:amd64.
2026-Feb-25 07:13:16.726435
#37 165.3 Preparing to unpack .../087-libgsm1_1.0.19-1_amd64.deb ...
2026-Feb-25 07:13:16.726435
#37 165.3 Unpacking libgsm1:amd64 (1.0.19-1) ...
2026-Feb-25 07:13:16.726435
#37 165.3 Selecting previously unselected package libmp3lame0:amd64.
2026-Feb-25 07:13:16.726435
#37 165.3 Preparing to unpack .../088-libmp3lame0_3.100-3build2_amd64.deb ...
2026-Feb-25 07:13:16.726435
#37 165.3 Unpacking libmp3lame0:amd64 (3.100-3build2) ...
2026-Feb-25 07:13:16.726435
#37 165.4 Selecting previously unselected package libopenjp2-7:amd64.
2026-Feb-25 07:13:16.726435
#37 165.4 Preparing to unpack .../089-libopenjp2-7_2.4.0-6ubuntu0.4_amd64.deb ...
2026-Feb-25 07:13:16.726435
#37 165.4 Unpacking libopenjp2-7:amd64 (2.4.0-6ubuntu0.4) ...
2026-Feb-25 07:13:16.726435
#37 165.4 Selecting previously unselected package libopus0:amd64.
2026-Feb-25 07:13:16.726435
#37 165.4 Preparing to unpack .../090-libopus0_1.3.1-0.1build2_amd64.deb ...
2026-Feb-25 07:13:16.726435
#37 165.4 Unpacking libopus0:amd64 (1.3.1-0.1build2) ...
2026-Feb-25 07:13:16.726435
#37 165.4 Selecting previously unselected package libcairo-gobject2:amd64.
2026-Feb-25 07:13:16.842619
#37 165.4 Preparing to unpack .../091-libcairo-gobject2_1.16.0-5ubuntu2_amd64.deb ...
2026-Feb-25 07:13:16.842619
#37 165.4 Unpacking libcairo-gobject2:amd64 (1.16.0-5ubuntu2) ...
2026-Feb-25 07:13:16.842619
#37 165.4 Selecting previously unselected package libgdk-pixbuf2.0-common.
2026-Feb-25 07:13:16.842619
#37 165.4 Preparing to unpack .../092-libgdk-pixbuf2.0-common_2.42.8+dfsg-1ubuntu0.4_all.deb ...
2026-Feb-25 07:13:16.842619
#37 165.4 Unpacking libgdk-pixbuf2.0-common (2.42.8+dfsg-1ubuntu0.4) ...
2026-Feb-25 07:13:16.842619
#37 165.4 Selecting previously unselected package libjpeg-turbo8:amd64.
2026-Feb-25 07:13:16.842619
#37 165.4 Preparing to unpack .../093-libjpeg-turbo8_2.1.2-0ubuntu1_amd64.deb ...
2026-Feb-25 07:13:16.842619
#37 165.4 Unpacking libjpeg-turbo8:amd64 (2.1.2-0ubuntu1) ...
2026-Feb-25 07:13:16.842619
#37 165.5 Selecting previously unselected package libjpeg8:amd64.
2026-Feb-25 07:13:16.842619
#37 165.5 Preparing to unpack .../094-libjpeg8_8c-2ubuntu10_amd64.deb ...
2026-Feb-25 07:13:16.842619
#37 165.5 Unpacking libjpeg8:amd64 (8c-2ubuntu10) ...
2026-Feb-25 07:13:16.842619
#37 165.5 Selecting previously unselected package libdeflate0:amd64.
2026-Feb-25 07:13:16.842619
#37 165.5 Preparing to unpack .../095-libdeflate0_1.10-2_amd64.deb ...
2026-Feb-25 07:13:16.842619
#37 165.5 Unpacking libdeflate0:amd64 (1.10-2) ...
2026-Feb-25 07:13:16.842619
#37 165.5 Selecting previously unselected package libjbig0:amd64.
2026-Feb-25 07:13:16.842619
#37 165.5 Preparing to unpack .../096-libjbig0_2.1-3.1ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:13:16.842619
#37 165.5 Unpacking libjbig0:amd64 (2.1-3.1ubuntu0.22.04.1) ...
2026-Feb-25 07:13:16.842619
#37 165.5 Selecting previously unselected package libwebp7:amd64.
2026-Feb-25 07:13:16.956820
#37 165.5 Preparing to unpack .../097-libwebp7_1.2.2-2ubuntu0.22.04.2_amd64.deb ...
2026-Feb-25 07:13:16.956820
#37 165.5 Unpacking libwebp7:amd64 (1.2.2-2ubuntu0.22.04.2) ...
2026-Feb-25 07:13:16.956820
#37 165.5 Selecting previously unselected package libtiff5:amd64.
2026-Feb-25 07:13:16.956820
#37 165.5 Preparing to unpack .../098-libtiff5_4.3.0-6ubuntu0.12_amd64.deb ...
2026-Feb-25 07:13:16.956820
#37 165.5 Unpacking libtiff5:amd64 (4.3.0-6ubuntu0.12) ...
2026-Feb-25 07:13:16.956820
#37 165.6 Selecting previously unselected package libgdk-pixbuf-2.0-0:amd64.
2026-Feb-25 07:13:16.956820
#37 165.6 Preparing to unpack .../099-libgdk-pixbuf-2.0-0_2.42.8+dfsg-1ubuntu0.4_amd64.deb ...
2026-Feb-25 07:13:16.956820
#37 165.6 Unpacking libgdk-pixbuf-2.0-0:amd64 (2.42.8+dfsg-1ubuntu0.4) ...
2026-Feb-25 07:13:16.956820
#37 165.6 Selecting previously unselected package fontconfig.
2026-Feb-25 07:13:16.956820
#37 165.6 Preparing to unpack .../100-fontconfig_2.13.1-4.2ubuntu5_amd64.deb ...
2026-Feb-25 07:13:16.956820
#37 165.6 Unpacking fontconfig (2.13.1-4.2ubuntu5) ...
2026-Feb-25 07:13:16.956820
#37 165.6 Selecting previously unselected package libgraphite2-3:amd64.
2026-Feb-25 07:13:16.956820
#37 165.6 Preparing to unpack .../101-libgraphite2-3_1.3.14-1build2_amd64.deb ...
2026-Feb-25 07:13:16.956820
#37 165.6 Unpacking libgraphite2-3:amd64 (1.3.14-1build2) ...
2026-Feb-25 07:13:16.956820
#37 165.6 Selecting previously unselected package libharfbuzz0b:amd64.
2026-Feb-25 07:13:17.063793
#37 165.6 Preparing to unpack .../102-libharfbuzz0b_2.7.4-1ubuntu3.2_amd64.deb ...
2026-Feb-25 07:13:17.063793
#37 165.6 Unpacking libharfbuzz0b:amd64 (2.7.4-1ubuntu3.2) ...
2026-Feb-25 07:13:17.063793
#37 165.6 Selecting previously unselected package libthai-data.
2026-Feb-25 07:13:17.063793
#37 165.7 Preparing to unpack .../103-libthai-data_0.1.29-1build1_all.deb ...
2026-Feb-25 07:13:17.063793
#37 165.7 Unpacking libthai-data (0.1.29-1build1) ...
2026-Feb-25 07:13:17.063793
#37 165.7 Selecting previously unselected package libdatrie1:amd64.
2026-Feb-25 07:13:17.063793
#37 165.7 Preparing to unpack .../104-libdatrie1_0.2.13-2_amd64.deb ...
2026-Feb-25 07:13:17.063793
#37 165.7 Unpacking libdatrie1:amd64 (0.2.13-2) ...
2026-Feb-25 07:13:17.063793
#37 165.7 Selecting previously unselected package libthai0:amd64.
2026-Feb-25 07:13:17.063793
#37 165.7 Preparing to unpack .../105-libthai0_0.1.29-1build1_amd64.deb ...
2026-Feb-25 07:13:17.063793
#37 165.7 Unpacking libthai0:amd64 (0.1.29-1build1) ...
2026-Feb-25 07:13:17.063793
#37 165.7 Selecting previously unselected package libpango-1.0-0:amd64.
2026-Feb-25 07:13:17.063793
#37 165.7 Preparing to unpack .../106-libpango-1.0-0_1.50.6+ds-2ubuntu1_amd64.deb ...
2026-Feb-25 07:13:17.063793
#37 165.7 Unpacking libpango-1.0-0:amd64 (1.50.6+ds-2ubuntu1) ...
2026-Feb-25 07:13:17.063793
#37 165.7 Selecting previously unselected package libpangoft2-1.0-0:amd64.
2026-Feb-25 07:13:17.179968
#37 165.7 Preparing to unpack .../107-libpangoft2-1.0-0_1.50.6+ds-2ubuntu1_amd64.deb ...
2026-Feb-25 07:13:17.179968
#37 165.7 Unpacking libpangoft2-1.0-0:amd64 (1.50.6+ds-2ubuntu1) ...
2026-Feb-25 07:13:17.179968
#37 165.8 Selecting previously unselected package libpangocairo-1.0-0:amd64.
2026-Feb-25 07:13:17.179968
#37 165.8 Preparing to unpack .../108-libpangocairo-1.0-0_1.50.6+ds-2ubuntu1_amd64.deb ...
2026-Feb-25 07:13:17.179968
#37 165.8 Unpacking libpangocairo-1.0-0:amd64 (1.50.6+ds-2ubuntu1) ...
2026-Feb-25 07:13:17.179968
#37 165.8 Selecting previously unselected package librsvg2-2:amd64.
2026-Feb-25 07:13:17.179968
#37 165.8 Preparing to unpack .../109-librsvg2-2_2.52.5+dfsg-3ubuntu0.2_amd64.deb ...
2026-Feb-25 07:13:17.179968
#37 165.8 Unpacking librsvg2-2:amd64 (2.52.5+dfsg-3ubuntu0.2) ...
2026-Feb-25 07:13:17.179968
#37 165.8 Selecting previously unselected package libshine3:amd64.
2026-Feb-25 07:13:17.179968
#37 165.8 Preparing to unpack .../110-libshine3_3.1.1-2_amd64.deb ...
2026-Feb-25 07:13:17.179968
#37 165.8 Unpacking libshine3:amd64 (3.1.1-2) ...
2026-Feb-25 07:13:17.179968
#37 165.8 Selecting previously unselected package libsnappy1v5:amd64.
2026-Feb-25 07:13:17.283174
#37 165.9 Preparing to unpack .../111-libsnappy1v5_1.1.8-1build3_amd64.deb ...
2026-Feb-25 07:13:17.283174
#37 165.9 Unpacking libsnappy1v5:amd64 (1.1.8-1build3) ...
2026-Feb-25 07:13:17.283174
#37 165.9 Selecting previously unselected package libspeex1:amd64.
2026-Feb-25 07:13:17.283174
#37 165.9 Preparing to unpack .../112-libspeex1_1.2~rc1.2-1.1ubuntu3_amd64.deb ...
2026-Feb-25 07:13:17.283174
#37 165.9 Unpacking libspeex1:amd64 (1.2~rc1.2-1.1ubuntu3) ...
2026-Feb-25 07:13:17.283174
#37 165.9 Selecting previously unselected package libsoxr0:amd64.
2026-Feb-25 07:13:17.283174
#37 165.9 Preparing to unpack .../113-libsoxr0_0.1.3-4build2_amd64.deb ...
2026-Feb-25 07:13:17.283174
#37 165.9 Unpacking libsoxr0:amd64 (0.1.3-4build2) ...
2026-Feb-25 07:13:17.283174
#37 165.9 Selecting previously unselected package libswresample3:amd64.
2026-Feb-25 07:13:17.283174
#37 165.9 Preparing to unpack .../114-libswresample3_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:13:17.283174
#37 165.9 Unpacking libswresample3:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:17.283174
#37 165.9 Selecting previously unselected package libogg0:amd64.
2026-Feb-25 07:13:17.283174
#37 165.9 Preparing to unpack .../115-libogg0_1.3.5-0ubuntu3_amd64.deb ...
2026-Feb-25 07:13:17.283174
#37 165.9 Unpacking libogg0:amd64 (1.3.5-0ubuntu3) ...
2026-Feb-25 07:13:17.283174
#37 166.0 Selecting previously unselected package libtheora0:amd64.
2026-Feb-25 07:13:17.397582
#37 166.0 Preparing to unpack .../116-libtheora0_1.1.1+dfsg.1-15ubuntu4_amd64.deb ...
2026-Feb-25 07:13:17.397582
#37 166.0 Unpacking libtheora0:amd64 (1.1.1+dfsg.1-15ubuntu4) ...
2026-Feb-25 07:13:17.397582
#37 166.0 Selecting previously unselected package libtwolame0:amd64.
2026-Feb-25 07:13:17.397582
#37 166.0 Preparing to unpack .../117-libtwolame0_0.4.0-2build2_amd64.deb ...
2026-Feb-25 07:13:17.397582
#37 166.0 Unpacking libtwolame0:amd64 (0.4.0-2build2) ...
2026-Feb-25 07:13:17.397582
#37 166.0 Selecting previously unselected package libvorbis0a:amd64.
2026-Feb-25 07:13:17.397582
#37 166.0 Preparing to unpack .../118-libvorbis0a_1.3.7-1build2_amd64.deb ...
2026-Feb-25 07:13:17.397582
#37 166.0 Unpacking libvorbis0a:amd64 (1.3.7-1build2) ...
2026-Feb-25 07:13:17.397582
#37 166.0 Selecting previously unselected package libvorbisenc2:amd64.
2026-Feb-25 07:13:17.397582
#37 166.0 Preparing to unpack .../119-libvorbisenc2_1.3.7-1build2_amd64.deb ...
2026-Feb-25 07:13:17.397582
#37 166.0 Unpacking libvorbisenc2:amd64 (1.3.7-1build2) ...
2026-Feb-25 07:13:17.397582
#37 166.0 Selecting previously unselected package libvpx7:amd64.
2026-Feb-25 07:13:17.397582
#37 166.0 Preparing to unpack .../120-libvpx7_1.11.0-2ubuntu2.5_amd64.deb ...
2026-Feb-25 07:13:17.397582
#37 166.0 Unpacking libvpx7:amd64 (1.11.0-2ubuntu2.5) ...
2026-Feb-25 07:13:17.397582
#37 166.1 Selecting previously unselected package libwebpmux3:amd64.
2026-Feb-25 07:13:17.501621
#37 166.1 Preparing to unpack .../121-libwebpmux3_1.2.2-2ubuntu0.22.04.2_amd64.deb ...
2026-Feb-25 07:13:17.501621
#37 166.1 Unpacking libwebpmux3:amd64 (1.2.2-2ubuntu0.22.04.2) ...
2026-Feb-25 07:13:17.501621
#37 166.1 Selecting previously unselected package libx264-163:amd64.
2026-Feb-25 07:13:17.501621
#37 166.1 Preparing to unpack .../122-libx264-163_2%3a0.163.3060+git5db6aa6-2build1_amd64.deb ...
2026-Feb-25 07:13:17.501621
#37 166.1 Unpacking libx264-163:amd64 (2:0.163.3060+git5db6aa6-2build1) ...
2026-Feb-25 07:13:17.501621
#37 166.1 Selecting previously unselected package libx265-199:amd64.
2026-Feb-25 07:13:17.501621
#37 166.1 Preparing to unpack .../123-libx265-199_3.5-2_amd64.deb ...
2026-Feb-25 07:13:17.501621
#37 166.1 Unpacking libx265-199:amd64 (3.5-2) ...
2026-Feb-25 07:13:17.501621
#37 166.2 Selecting previously unselected package libxvidcore4:amd64.
2026-Feb-25 07:13:17.650777
#37 166.2 Preparing to unpack .../124-libxvidcore4_2%3a1.3.7-1_amd64.deb ...
2026-Feb-25 07:13:17.650777
#37 166.2 Unpacking libxvidcore4:amd64 (2:1.3.7-1) ...
2026-Feb-25 07:13:17.650777
#37 166.2 Selecting previously unselected package libzvbi-common.
2026-Feb-25 07:13:17.650777
#37 166.2 Preparing to unpack .../125-libzvbi-common_0.2.35-19_all.deb ...
2026-Feb-25 07:13:17.650777
#37 166.2 Unpacking libzvbi-common (0.2.35-19) ...
2026-Feb-25 07:13:17.650777
#37 166.2 Selecting previously unselected package libzvbi0:amd64.
2026-Feb-25 07:13:17.650777
#37 166.2 Preparing to unpack .../126-libzvbi0_0.2.35-19_amd64.deb ...
2026-Feb-25 07:13:17.650777
#37 166.2 Unpacking libzvbi0:amd64 (0.2.35-19) ...
2026-Feb-25 07:13:17.650777
#37 166.2 Selecting previously unselected package libavcodec58:amd64.
2026-Feb-25 07:13:17.650777
#37 166.3 Preparing to unpack .../127-libavcodec58_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:13:17.650777
#37 166.3 Unpacking libavcodec58:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:17.650777
#37 166.3 Selecting previously unselected package libraw1394-11:amd64.
2026-Feb-25 07:13:17.756050
#37 166.3 Preparing to unpack .../128-libraw1394-11_2.1.2-2build2_amd64.deb ...
2026-Feb-25 07:13:17.756050
#37 166.3 Unpacking libraw1394-11:amd64 (2.1.2-2build2) ...
2026-Feb-25 07:13:17.756050
#37 166.3 Selecting previously unselected package libavc1394-0:amd64.
2026-Feb-25 07:13:17.756050
#37 166.3 Preparing to unpack .../129-libavc1394-0_0.5.4-5build2_amd64.deb ...
2026-Feb-25 07:13:17.756050
#37 166.3 Unpacking libavc1394-0:amd64 (0.5.4-5build2) ...
2026-Feb-25 07:13:17.756050
#37 166.4 Selecting previously unselected package libass9:amd64.
2026-Feb-25 07:13:17.756050
#37 166.4 Preparing to unpack .../130-libass9_1%3a0.15.2-1_amd64.deb ...
2026-Feb-25 07:13:17.756050
#37 166.4 Unpacking libass9:amd64 (1:0.15.2-1) ...
2026-Feb-25 07:13:17.756050
#37 166.4 Selecting previously unselected package libudfread0:amd64.
2026-Feb-25 07:13:17.756050
#37 166.4 Preparing to unpack .../131-libudfread0_1.1.2-1_amd64.deb ...
2026-Feb-25 07:13:17.756050
#37 166.4 Unpacking libudfread0:amd64 (1.1.2-1) ...
2026-Feb-25 07:13:17.756050
#37 166.4 Selecting previously unselected package libbluray2:amd64.
2026-Feb-25 07:13:17.756050
#37 166.4 Preparing to unpack .../132-libbluray2_1%3a1.3.1-1_amd64.deb ...
2026-Feb-25 07:13:17.756050
#37 166.4 Unpacking libbluray2:amd64 (1:1.3.1-1) ...
2026-Feb-25 07:13:17.756050
#37 166.4 Selecting previously unselected package libchromaprint1:amd64.
2026-Feb-25 07:13:17.871586
#37 166.4 Preparing to unpack .../133-libchromaprint1_1.5.1-2_amd64.deb ...
2026-Feb-25 07:13:17.871586
#37 166.4 Unpacking libchromaprint1:amd64 (1.5.1-2) ...
2026-Feb-25 07:13:17.871586
#37 166.4 Selecting previously unselected package libgme0:amd64.
2026-Feb-25 07:13:17.871586
#37 166.4 Preparing to unpack .../134-libgme0_0.6.3-2_amd64.deb ...
2026-Feb-25 07:13:17.871586
#37 166.5 Unpacking libgme0:amd64 (0.6.3-2) ...
2026-Feb-25 07:13:17.871586
#37 166.5 Selecting previously unselected package libmpg123-0:amd64.
2026-Feb-25 07:13:17.871586
#37 166.5 Preparing to unpack .../135-libmpg123-0_1.29.3-1ubuntu0.1_amd64.deb ...
2026-Feb-25 07:13:17.871586
#37 166.5 Unpacking libmpg123-0:amd64 (1.29.3-1ubuntu0.1) ...
2026-Feb-25 07:13:17.871586
#37 166.5 Selecting previously unselected package libvorbisfile3:amd64.
2026-Feb-25 07:13:17.871586
#37 166.5 Preparing to unpack .../136-libvorbisfile3_1.3.7-1build2_amd64.deb ...
2026-Feb-25 07:13:17.871586
#37 166.5 Unpacking libvorbisfile3:amd64 (1.3.7-1build2) ...
2026-Feb-25 07:13:17.871586
#37 166.5 Selecting previously unselected package libopenmpt0:amd64.
2026-Feb-25 07:13:17.871586
#37 166.5 Preparing to unpack .../137-libopenmpt0_0.6.1-1_amd64.deb ...
2026-Feb-25 07:13:17.871586
#37 166.5 Unpacking libopenmpt0:amd64 (0.6.1-1) ...
2026-Feb-25 07:13:17.871586
#37 166.5 Selecting previously unselected package librabbitmq4:amd64.
2026-Feb-25 07:13:17.972925
#37 166.5 Preparing to unpack .../138-librabbitmq4_0.10.0-1ubuntu2_amd64.deb ...
2026-Feb-25 07:13:17.972925
#37 166.5 Unpacking librabbitmq4:amd64 (0.10.0-1ubuntu2) ...
2026-Feb-25 07:13:17.972925
#37 166.6 Selecting previously unselected package libsrt1.4-gnutls:amd64.
2026-Feb-25 07:13:17.972925
#37 166.6 Preparing to unpack .../139-libsrt1.4-gnutls_1.4.4-4_amd64.deb ...
2026-Feb-25 07:13:17.972925
#37 166.6 Unpacking libsrt1.4-gnutls:amd64 (1.4.4-4) ...
2026-Feb-25 07:13:17.972925
#37 166.6 Selecting previously unselected package libssh-gcrypt-4:amd64.
2026-Feb-25 07:13:17.972925
#37 166.6 Preparing to unpack .../140-libssh-gcrypt-4_0.9.6-2ubuntu0.22.04.6_amd64.deb ...
2026-Feb-25 07:13:17.972925
#37 166.6 Unpacking libssh-gcrypt-4:amd64 (0.9.6-2ubuntu0.22.04.6) ...
2026-Feb-25 07:13:17.972925
#37 166.6 Selecting previously unselected package libnorm1:amd64.
2026-Feb-25 07:13:17.972925
#37 166.6 Preparing to unpack .../141-libnorm1_1.5.9+dfsg-2_amd64.deb ...
2026-Feb-25 07:13:17.972925
#37 166.6 Unpacking libnorm1:amd64 (1.5.9+dfsg-2) ...
2026-Feb-25 07:13:17.972925
#37 166.6 Selecting previously unselected package libpgm-5.3-0:amd64.
2026-Feb-25 07:13:18.076875
#37 166.6 Preparing to unpack .../142-libpgm-5.3-0_5.3.128~dfsg-2_amd64.deb ...
2026-Feb-25 07:13:18.076875
#37 166.6 Unpacking libpgm-5.3-0:amd64 (5.3.128~dfsg-2) ...
2026-Feb-25 07:13:18.076875
#37 166.7 Selecting previously unselected package libsodium23:amd64.
2026-Feb-25 07:13:18.076875
#37 166.7 Preparing to unpack .../143-libsodium23_1.0.18-1ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:13:18.076875
#37 166.7 Unpacking libsodium23:amd64 (1.0.18-1ubuntu0.22.04.1) ...
2026-Feb-25 07:13:18.076875
#37 166.7 Selecting previously unselected package libzmq5:amd64.
2026-Feb-25 07:13:18.076875
#37 166.7 Preparing to unpack .../144-libzmq5_4.3.4-2_amd64.deb ...
2026-Feb-25 07:13:18.076875
#37 166.7 Unpacking libzmq5:amd64 (4.3.4-2) ...
2026-Feb-25 07:13:18.076875
#37 166.7 Selecting previously unselected package libavformat58:amd64.
2026-Feb-25 07:13:18.076875
#37 166.7 Preparing to unpack .../145-libavformat58_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:13:18.076875
#37 166.7 Unpacking libavformat58:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:18.076875
#37 166.7 Selecting previously unselected package libbs2b0:amd64.
2026-Feb-25 07:13:18.199782
#37 166.7 Preparing to unpack .../146-libbs2b0_3.1.0+dfsg-2.2build1_amd64.deb ...
2026-Feb-25 07:13:18.199782
#37 166.8 Unpacking libbs2b0:amd64 (3.1.0+dfsg-2.2build1) ...
2026-Feb-25 07:13:18.199782
#37 166.8 Selecting previously unselected package libflite1:amd64.
2026-Feb-25 07:13:18.199782
#37 166.8 Preparing to unpack .../147-libflite1_2.2-3_amd64.deb ...
2026-Feb-25 07:13:18.199782
#37 166.8 Unpacking libflite1:amd64 (2.2-3) ...
2026-Feb-25 07:13:18.199782
#37 166.9 Selecting previously unselected package libserd-0-0:amd64.
2026-Feb-25 07:13:18.310013
#37 166.9 Preparing to unpack .../148-libserd-0-0_0.30.10-2_amd64.deb ...
2026-Feb-25 07:13:18.310013
#37 166.9 Unpacking libserd-0-0:amd64 (0.30.10-2) ...
2026-Feb-25 07:13:18.310013
#37 166.9 Selecting previously unselected package libsord-0-0:amd64.
2026-Feb-25 07:13:18.310013
#37 166.9 Preparing to unpack .../149-libsord-0-0_0.16.8-2_amd64.deb ...
2026-Feb-25 07:13:18.310013
#37 166.9 Unpacking libsord-0-0:amd64 (0.16.8-2) ...
2026-Feb-25 07:13:18.310013
#37 166.9 Selecting previously unselected package libsratom-0-0:amd64.
2026-Feb-25 07:13:18.310013
#37 166.9 Preparing to unpack .../150-libsratom-0-0_0.6.8-1_amd64.deb ...
2026-Feb-25 07:13:18.310013
#37 166.9 Unpacking libsratom-0-0:amd64 (0.6.8-1) ...
2026-Feb-25 07:13:18.310013
#37 166.9 Selecting previously unselected package liblilv-0-0:amd64.
2026-Feb-25 07:13:18.310013
#37 166.9 Preparing to unpack .../151-liblilv-0-0_0.24.12-2_amd64.deb ...
2026-Feb-25 07:13:18.310013
#37 166.9 Unpacking liblilv-0-0:amd64 (0.24.12-2) ...
2026-Feb-25 07:13:18.310013
#37 167.0 Selecting previously unselected package libmysofa1:amd64.
2026-Feb-25 07:13:18.310013
#37 167.0 Preparing to unpack .../152-libmysofa1_1.2.1~dfsg0-1_amd64.deb ...
2026-Feb-25 07:13:18.310013
#37 167.0 Unpacking libmysofa1:amd64 (1.2.1~dfsg0-1) ...
2026-Feb-25 07:13:18.310013
#37 167.0 Selecting previously unselected package libblas3:amd64.
2026-Feb-25 07:13:18.414520
#37 167.0 Preparing to unpack .../153-libblas3_3.10.0-2ubuntu1_amd64.deb ...
2026-Feb-25 07:13:18.414520
#37 167.0 Unpacking libblas3:amd64 (3.10.0-2ubuntu1) ...
2026-Feb-25 07:13:18.414520
#37 167.0 Selecting previously unselected package libgfortran5:amd64.
2026-Feb-25 07:13:18.414520
#37 167.0 Preparing to unpack .../154-libgfortran5_12.3.0-1ubuntu1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:18.414520
#37 167.0 Unpacking libgfortran5:amd64 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:18.414520
#37 167.0 Selecting previously unselected package liblapack3:amd64.
2026-Feb-25 07:13:18.414520
#37 167.0 Preparing to unpack .../155-liblapack3_3.10.0-2ubuntu1_amd64.deb ...
2026-Feb-25 07:13:18.414520
#37 167.0 Unpacking liblapack3:amd64 (3.10.0-2ubuntu1) ...
2026-Feb-25 07:13:18.414520
#37 167.1 Selecting previously unselected package libasyncns0:amd64.
2026-Feb-25 07:13:18.518697
#37 167.1 Preparing to unpack .../156-libasyncns0_0.8-6build2_amd64.deb ...
2026-Feb-25 07:13:18.518697
#37 167.1 Unpacking libasyncns0:amd64 (0.8-6build2) ...
2026-Feb-25 07:13:18.518697
#37 167.1 Selecting previously unselected package libflac8:amd64.
2026-Feb-25 07:13:18.518697
#37 167.1 Preparing to unpack .../157-libflac8_1.3.3-2ubuntu0.2_amd64.deb ...
2026-Feb-25 07:13:18.518697
#37 167.1 Unpacking libflac8:amd64 (1.3.3-2ubuntu0.2) ...
2026-Feb-25 07:13:18.518697
#37 167.1 Selecting previously unselected package libsndfile1:amd64.
2026-Feb-25 07:13:18.518697
#37 167.1 Preparing to unpack .../158-libsndfile1_1.0.31-2ubuntu0.2_amd64.deb ...
2026-Feb-25 07:13:18.518697
#37 167.1 Unpacking libsndfile1:amd64 (1.0.31-2ubuntu0.2) ...
2026-Feb-25 07:13:18.518697
#37 167.1 Selecting previously unselected package libx11-xcb1:amd64.
2026-Feb-25 07:13:18.518697
#37 167.1 Preparing to unpack .../159-libx11-xcb1_2%3a1.7.5-1ubuntu0.3_amd64.deb ...
2026-Feb-25 07:13:18.518697
#37 167.2 Unpacking libx11-xcb1:amd64 (2:1.7.5-1ubuntu0.3) ...
2026-Feb-25 07:13:18.518697
#37 167.2 Selecting previously unselected package libpulse0:amd64.
2026-Feb-25 07:13:18.518697
#37 167.2 Preparing to unpack .../160-libpulse0_1%3a15.99.1+dfsg1-1ubuntu2.2_amd64.deb ...
2026-Feb-25 07:13:18.518697
#37 167.2 Unpacking libpulse0:amd64 (1:15.99.1+dfsg1-1ubuntu2.2) ...
2026-Feb-25 07:13:18.629281
#37 167.2 Selecting previously unselected package libsphinxbase3:amd64.
2026-Feb-25 07:13:18.629281
#37 167.2 Preparing to unpack .../161-libsphinxbase3_0.8+5prealpha+1-13build1_amd64.deb ...
2026-Feb-25 07:13:18.629281
#37 167.2 Unpacking libsphinxbase3:amd64 (0.8+5prealpha+1-13build1) ...
2026-Feb-25 07:13:18.629281
#37 167.2 Selecting previously unselected package libpocketsphinx3:amd64.
2026-Feb-25 07:13:18.629281
#37 167.2 Preparing to unpack .../162-libpocketsphinx3_0.8.0+real5prealpha+1-14ubuntu1_amd64.deb ...
2026-Feb-25 07:13:18.629281
#37 167.2 Unpacking libpocketsphinx3:amd64 (0.8.0+real5prealpha+1-14ubuntu1) ...
2026-Feb-25 07:13:18.629281
#37 167.3 Selecting previously unselected package libpostproc55:amd64.
2026-Feb-25 07:13:18.629281
#37 167.3 Preparing to unpack .../163-libpostproc55_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:13:18.629281
#37 167.3 Unpacking libpostproc55:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:18.629281
#37 167.3 Selecting previously unselected package libsamplerate0:amd64.
2026-Feb-25 07:13:18.629281
#37 167.3 Preparing to unpack .../164-libsamplerate0_0.2.2-1build1_amd64.deb ...
2026-Feb-25 07:13:18.629281
#37 167.3 Unpacking libsamplerate0:amd64 (0.2.2-1build1) ...
2026-Feb-25 07:13:18.629281
#37 167.3 Selecting previously unselected package librubberband2:amd64.
2026-Feb-25 07:13:18.751711
#37 167.3 Preparing to unpack .../165-librubberband2_2.0.0-2_amd64.deb ...
2026-Feb-25 07:13:18.751711
#37 167.3 Unpacking librubberband2:amd64 (2.0.0-2) ...
2026-Feb-25 07:13:18.751711
#37 167.3 Selecting previously unselected package libswscale5:amd64.
2026-Feb-25 07:13:18.751711
#37 167.3 Preparing to unpack .../166-libswscale5_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:13:18.751711
#37 167.3 Unpacking libswscale5:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:18.751711
#37 167.3 Selecting previously unselected package libvidstab1.1:amd64.
2026-Feb-25 07:13:18.751711
#37 167.3 Preparing to unpack .../167-libvidstab1.1_1.1.0-2_amd64.deb ...
2026-Feb-25 07:13:18.751711
#37 167.3 Unpacking libvidstab1.1:amd64 (1.1.0-2) ...
2026-Feb-25 07:13:18.751711
#37 167.4 Selecting previously unselected package libzimg2:amd64.
2026-Feb-25 07:13:18.751711
#37 167.4 Preparing to unpack .../168-libzimg2_3.0.3+ds1-1_amd64.deb ...
2026-Feb-25 07:13:18.751711
#37 167.4 Unpacking libzimg2:amd64 (3.0.3+ds1-1) ...
2026-Feb-25 07:13:18.751711
#37 167.4 Selecting previously unselected package libavfilter7:amd64.
2026-Feb-25 07:13:18.751711
#37 167.4 Preparing to unpack .../169-libavfilter7_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:13:18.751711
#37 167.4 Unpacking libavfilter7:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:18.751711
#37 167.4 Selecting previously unselected package libcaca0:amd64.
2026-Feb-25 07:13:18.862707
#37 167.4 Preparing to unpack .../170-libcaca0_0.99.beta19-2.2ubuntu4.1_amd64.deb ...
2026-Feb-25 07:13:18.862707
#37 167.4 Unpacking libcaca0:amd64 (0.99.beta19-2.2ubuntu4.1) ...
2026-Feb-25 07:13:18.862707
#37 167.4 Selecting previously unselected package libcdio19:amd64.
2026-Feb-25 07:13:18.862707
#37 167.4 Preparing to unpack .../171-libcdio19_2.1.0-3ubuntu0.2_amd64.deb ...
2026-Feb-25 07:13:18.862707
#37 167.5 Unpacking libcdio19:amd64 (2.1.0-3ubuntu0.2) ...
2026-Feb-25 07:13:18.862707
#37 167.5 Selecting previously unselected package libcdio-cdda2:amd64.
2026-Feb-25 07:13:18.862707
#37 167.5 Preparing to unpack .../172-libcdio-cdda2_10.2+2.0.0-1build3_amd64.deb ...
2026-Feb-25 07:13:18.862707
#37 167.5 Unpacking libcdio-cdda2:amd64 (10.2+2.0.0-1build3) ...
2026-Feb-25 07:13:18.862707
#37 167.5 Selecting previously unselected package libcdio-paranoia2:amd64.
2026-Feb-25 07:13:18.862707
#37 167.5 Preparing to unpack .../173-libcdio-paranoia2_10.2+2.0.0-1build3_amd64.deb ...
2026-Feb-25 07:13:18.862707
#37 167.5 Unpacking libcdio-paranoia2:amd64 (10.2+2.0.0-1build3) ...
2026-Feb-25 07:13:18.862707
#37 167.5 Selecting previously unselected package libdc1394-25:amd64.
2026-Feb-25 07:13:18.862707
#37 167.5 Preparing to unpack .../174-libdc1394-25_2.2.6-4_amd64.deb ...
2026-Feb-25 07:13:18.862707
#37 167.5 Unpacking libdc1394-25:amd64 (2.2.6-4) ...
2026-Feb-25 07:13:18.862707
#37 167.5 Selecting previously unselected package libglvnd0:amd64.
2026-Feb-25 07:13:18.968451
#37 167.5 Preparing to unpack .../175-libglvnd0_1.4.0-1_amd64.deb ...
2026-Feb-25 07:13:18.968451
#37 167.5 Unpacking libglvnd0:amd64 (1.4.0-1) ...
2026-Feb-25 07:13:18.968451
#37 167.6 Selecting previously unselected package libglapi-mesa:amd64.
2026-Feb-25 07:13:18.968451
#37 167.6 Preparing to unpack .../176-libglapi-mesa_23.2.1-1ubuntu3.1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:18.968451
#37 167.6 Unpacking libglapi-mesa:amd64 (23.2.1-1ubuntu3.1~22.04.3) ...
2026-Feb-25 07:13:18.968451
#37 167.6 Selecting previously unselected package libxcb-dri2-0:amd64.
2026-Feb-25 07:13:18.968451
#37 167.6 Preparing to unpack .../177-libxcb-dri2-0_1.14-3ubuntu3_amd64.deb ...
2026-Feb-25 07:13:18.968451
#37 167.6 Unpacking libxcb-dri2-0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:13:18.968451
#37 167.6 Selecting previously unselected package libxcb-dri3-0:amd64.
2026-Feb-25 07:13:18.968451
#37 167.6 Preparing to unpack .../178-libxcb-dri3-0_1.14-3ubuntu3_amd64.deb ...
2026-Feb-25 07:13:18.968451
#37 167.6 Unpacking libxcb-dri3-0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:13:18.968451
#37 167.6 Selecting previously unselected package libxcb-glx0:amd64.
2026-Feb-25 07:13:18.968451
#37 167.6 Preparing to unpack .../179-libxcb-glx0_1.14-3ubuntu3_amd64.deb ...
2026-Feb-25 07:13:18.968451
#37 167.6 Unpacking libxcb-glx0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:13:18.968451
#37 167.6 Selecting previously unselected package libxcb-present0:amd64.
2026-Feb-25 07:13:19.072895
#37 167.6 Preparing to unpack .../180-libxcb-present0_1.14-3ubuntu3_amd64.deb ...
2026-Feb-25 07:13:19.072895
#37 167.6 Unpacking libxcb-present0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:13:19.072895
#37 167.7 Selecting previously unselected package libxcb-randr0:amd64.
2026-Feb-25 07:13:19.072895
#37 167.7 Preparing to unpack .../181-libxcb-randr0_1.14-3ubuntu3_amd64.deb ...
2026-Feb-25 07:13:19.072895
#37 167.7 Unpacking libxcb-randr0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:13:19.072895
#37 167.7 Selecting previously unselected package libxcb-sync1:amd64.
2026-Feb-25 07:13:19.072895
#37 167.7 Preparing to unpack .../182-libxcb-sync1_1.14-3ubuntu3_amd64.deb ...
2026-Feb-25 07:13:19.072895
#37 167.7 Unpacking libxcb-sync1:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:13:19.072895
#37 167.7 Selecting previously unselected package libxcb-xfixes0:amd64.
2026-Feb-25 07:13:19.072895
#37 167.7 Preparing to unpack .../183-libxcb-xfixes0_1.14-3ubuntu3_amd64.deb ...
2026-Feb-25 07:13:19.072895
#37 167.7 Unpacking libxcb-xfixes0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:13:19.072895
#37 167.7 Selecting previously unselected package libxshmfence1:amd64.
2026-Feb-25 07:13:19.072895
#37 167.7 Preparing to unpack .../184-libxshmfence1_1.3-1build4_amd64.deb ...
2026-Feb-25 07:13:19.072895
#37 167.7 Unpacking libxshmfence1:amd64 (1.3-1build4) ...
2026-Feb-25 07:13:19.072895
#37 167.7 Selecting previously unselected package libxxf86vm1:amd64.
2026-Feb-25 07:13:19.175930
#37 167.7 Preparing to unpack .../185-libxxf86vm1_1%3a1.1.4-1build3_amd64.deb ...
2026-Feb-25 07:13:19.175930
#37 167.7 Unpacking libxxf86vm1:amd64 (1:1.1.4-1build3) ...
2026-Feb-25 07:13:19.175930
#37 167.8 Selecting previously unselected package libdrm-amdgpu1:amd64.
2026-Feb-25 07:13:19.175930
#37 167.8 Preparing to unpack .../186-libdrm-amdgpu1_2.4.113-2~ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:13:19.175930
#37 167.8 Unpacking libdrm-amdgpu1:amd64 (2.4.113-2~ubuntu0.22.04.1) ...
2026-Feb-25 07:13:19.175930
#37 167.8 Selecting previously unselected package libpciaccess0:amd64.
2026-Feb-25 07:13:19.175930
#37 167.8 Preparing to unpack .../187-libpciaccess0_0.16-3_amd64.deb ...
2026-Feb-25 07:13:19.175930
#37 167.8 Unpacking libpciaccess0:amd64 (0.16-3) ...
2026-Feb-25 07:13:19.175930
#37 167.8 Selecting previously unselected package libdrm-intel1:amd64.
2026-Feb-25 07:13:19.175930
#37 167.8 Preparing to unpack .../188-libdrm-intel1_2.4.113-2~ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:13:19.175930
#37 167.8 Unpacking libdrm-intel1:amd64 (2.4.113-2~ubuntu0.22.04.1) ...
2026-Feb-25 07:13:19.175930
#37 167.8 Selecting previously unselected package libdrm-nouveau2:amd64.
2026-Feb-25 07:13:19.175930
#37 167.8 Preparing to unpack .../189-libdrm-nouveau2_2.4.113-2~ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:13:19.175930
#37 167.8 Unpacking libdrm-nouveau2:amd64 (2.4.113-2~ubuntu0.22.04.1) ...
2026-Feb-25 07:13:19.175930
#37 167.8 Selecting previously unselected package libdrm-radeon1:amd64.
2026-Feb-25 07:13:19.350182
#37 167.8 Preparing to unpack .../190-libdrm-radeon1_2.4.113-2~ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:13:19.350182
#37 167.8 Unpacking libdrm-radeon1:amd64 (2.4.113-2~ubuntu0.22.04.1) ...
2026-Feb-25 07:13:19.350182
#37 167.9 Selecting previously unselected package libllvm15:amd64.
2026-Feb-25 07:13:19.350182
#37 167.9 Preparing to unpack .../191-libllvm15_1%3a15.0.7-0ubuntu0.22.04.3_amd64.deb ...
2026-Feb-25 07:13:19.350182
#37 167.9 Unpacking libllvm15:amd64 (1:15.0.7-0ubuntu0.22.04.3) ...
2026-Feb-25 07:13:19.566654
#37 168.2 Selecting previously unselected package libsensors-config.
2026-Feb-25 07:13:19.764794
#37 168.2 Preparing to unpack .../192-libsensors-config_1%3a3.6.0-7ubuntu1_all.deb ...
2026-Feb-25 07:13:19.764794
#37 168.2 Unpacking libsensors-config (1:3.6.0-7ubuntu1) ...
2026-Feb-25 07:13:19.764794
#37 168.3 Selecting previously unselected package libsensors5:amd64.
2026-Feb-25 07:13:19.764794
#37 168.3 Preparing to unpack .../193-libsensors5_1%3a3.6.0-7ubuntu1_amd64.deb ...
2026-Feb-25 07:13:19.764794
#37 168.3 Unpacking libsensors5:amd64 (1:3.6.0-7ubuntu1) ...
2026-Feb-25 07:13:19.764794
#37 168.3 Selecting previously unselected package libgl1-mesa-dri:amd64.
2026-Feb-25 07:13:19.764794
#37 168.3 Preparing to unpack .../194-libgl1-mesa-dri_23.2.1-1ubuntu3.1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:19.764794
#37 168.3 Unpacking libgl1-mesa-dri:amd64 (23.2.1-1ubuntu3.1~22.04.3) ...
2026-Feb-25 07:13:19.764794
#37 168.4 Selecting previously unselected package libglx-mesa0:amd64.
2026-Feb-25 07:13:19.874932
#37 168.4 Preparing to unpack .../195-libglx-mesa0_23.2.1-1ubuntu3.1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:19.874932
#37 168.4 Unpacking libglx-mesa0:amd64 (23.2.1-1ubuntu3.1~22.04.3) ...
2026-Feb-25 07:13:19.874932
#37 168.5 Selecting previously unselected package libglx0:amd64.
2026-Feb-25 07:13:19.874932
#37 168.5 Preparing to unpack .../196-libglx0_1.4.0-1_amd64.deb ...
2026-Feb-25 07:13:19.874932
#37 168.5 Unpacking libglx0:amd64 (1.4.0-1) ...
2026-Feb-25 07:13:19.874932
#37 168.5 Selecting previously unselected package libgl1:amd64.
2026-Feb-25 07:13:19.874932
#37 168.5 Preparing to unpack .../197-libgl1_1.4.0-1_amd64.deb ...
2026-Feb-25 07:13:19.874932
#37 168.5 Unpacking libgl1:amd64 (1.4.0-1) ...
2026-Feb-25 07:13:19.874932
#37 168.5 Selecting previously unselected package libiec61883-0:amd64.
2026-Feb-25 07:13:19.874932
#37 168.5 Preparing to unpack .../198-libiec61883-0_1.2.0-4build3_amd64.deb ...
2026-Feb-25 07:13:19.874932
#37 168.5 Unpacking libiec61883-0:amd64 (1.2.0-4build3) ...
2026-Feb-25 07:13:19.874932
#37 168.5 Selecting previously unselected package libjack-jackd2-0:amd64.
2026-Feb-25 07:13:19.874932
#37 168.5 Preparing to unpack .../199-libjack-jackd2-0_1.9.20~dfsg-1_amd64.deb ...
2026-Feb-25 07:13:19.874932
#37 168.5 Unpacking libjack-jackd2-0:amd64 (1.9.20~dfsg-1) ...
2026-Feb-25 07:13:19.874932
#37 168.5 Selecting previously unselected package libopenal-data.
2026-Feb-25 07:13:19.983893
#37 168.5 Preparing to unpack .../200-libopenal-data_1%3a1.19.1-2build3_all.deb ...
2026-Feb-25 07:13:19.983893
#37 168.5 Unpacking libopenal-data (1:1.19.1-2build3) ...
2026-Feb-25 07:13:19.983893
#37 168.6 Selecting previously unselected package libsndio7.0:amd64.
2026-Feb-25 07:13:19.983893
#37 168.6 Preparing to unpack .../201-libsndio7.0_1.8.1-1.1_amd64.deb ...
2026-Feb-25 07:13:19.983893
#37 168.6 Unpacking libsndio7.0:amd64 (1.8.1-1.1) ...
2026-Feb-25 07:13:19.983893
#37 168.6 Selecting previously unselected package libopenal1:amd64.
2026-Feb-25 07:13:19.983893
#37 168.6 Preparing to unpack .../202-libopenal1_1%3a1.19.1-2build3_amd64.deb ...
2026-Feb-25 07:13:19.983893
#37 168.6 Unpacking libopenal1:amd64 (1:1.19.1-2build3) ...
2026-Feb-25 07:13:19.983893
#37 168.6 Selecting previously unselected package libwayland-client0:amd64.
2026-Feb-25 07:13:19.983893
#37 168.6 Preparing to unpack .../203-libwayland-client0_1.20.0-1ubuntu0.1_amd64.deb ...
2026-Feb-25 07:13:19.983893
#37 168.6 Unpacking libwayland-client0:amd64 (1.20.0-1ubuntu0.1) ...
2026-Feb-25 07:13:19.983893
#37 168.6 Selecting previously unselected package libdecor-0-0:amd64.
2026-Feb-25 07:13:19.983893
#37 168.6 Preparing to unpack .../204-libdecor-0-0_0.1.0-3build1_amd64.deb ...
2026-Feb-25 07:13:19.983893
#37 168.6 Unpacking libdecor-0-0:amd64 (0.1.0-3build1) ...
2026-Feb-25 07:13:19.983893
#37 168.7 Selecting previously unselected package libwayland-server0:amd64.
2026-Feb-25 07:13:20.090189
#37 168.7 Preparing to unpack .../205-libwayland-server0_1.20.0-1ubuntu0.1_amd64.deb ...
2026-Feb-25 07:13:20.090189
#37 168.7 Unpacking libwayland-server0:amd64 (1.20.0-1ubuntu0.1) ...
2026-Feb-25 07:13:20.090189
#37 168.7 Selecting previously unselected package libgbm1:amd64.
2026-Feb-25 07:13:20.090189
#37 168.7 Preparing to unpack .../206-libgbm1_23.2.1-1ubuntu3.1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:20.090189
#37 168.7 Unpacking libgbm1:amd64 (23.2.1-1ubuntu3.1~22.04.3) ...
2026-Feb-25 07:13:20.090189
#37 168.7 Selecting previously unselected package libwayland-cursor0:amd64.
2026-Feb-25 07:13:20.090189
#37 168.7 Preparing to unpack .../207-libwayland-cursor0_1.20.0-1ubuntu0.1_amd64.deb ...
2026-Feb-25 07:13:20.090189
#37 168.7 Unpacking libwayland-cursor0:amd64 (1.20.0-1ubuntu0.1) ...
2026-Feb-25 07:13:20.090189
#37 168.7 Selecting previously unselected package libwayland-egl1:amd64.
2026-Feb-25 07:13:20.090189
#37 168.7 Preparing to unpack .../208-libwayland-egl1_1.20.0-1ubuntu0.1_amd64.deb ...
2026-Feb-25 07:13:20.090189
#37 168.7 Unpacking libwayland-egl1:amd64 (1.20.0-1ubuntu0.1) ...
2026-Feb-25 07:13:20.090189
#37 168.7 Selecting previously unselected package libxcursor1:amd64.
2026-Feb-25 07:13:20.090189
#37 168.7 Preparing to unpack .../209-libxcursor1_1%3a1.2.0-2build4_amd64.deb ...
2026-Feb-25 07:13:20.090189
#37 168.7 Unpacking libxcursor1:amd64 (1:1.2.0-2build4) ...
2026-Feb-25 07:13:20.090189
#37 168.8 Selecting previously unselected package libxi6:amd64.
2026-Feb-25 07:13:20.201202
#37 168.8 Preparing to unpack .../210-libxi6_2%3a1.8-1build1_amd64.deb ...
2026-Feb-25 07:13:20.201202
#37 168.8 Unpacking libxi6:amd64 (2:1.8-1build1) ...
2026-Feb-25 07:13:20.201202
#37 168.8 Selecting previously unselected package libxinerama1:amd64.
2026-Feb-25 07:13:20.201202
#37 168.8 Preparing to unpack .../211-libxinerama1_2%3a1.1.4-3_amd64.deb ...
2026-Feb-25 07:13:20.201202
#37 168.8 Unpacking libxinerama1:amd64 (2:1.1.4-3) ...
2026-Feb-25 07:13:20.201202
#37 168.8 Selecting previously unselected package libxkbcommon0:amd64.
2026-Feb-25 07:13:20.201202
#37 168.8 Preparing to unpack .../212-libxkbcommon0_1.4.0-1_amd64.deb ...
2026-Feb-25 07:13:20.201202
#37 168.8 Unpacking libxkbcommon0:amd64 (1.4.0-1) ...
2026-Feb-25 07:13:20.201202
#37 168.8 Selecting previously unselected package libxrandr2:amd64.
2026-Feb-25 07:13:20.201202
#37 168.8 Preparing to unpack .../213-libxrandr2_2%3a1.5.2-1build1_amd64.deb ...
2026-Feb-25 07:13:20.201202
#37 168.8 Unpacking libxrandr2:amd64 (2:1.5.2-1build1) ...
2026-Feb-25 07:13:20.201202
#37 168.8 Selecting previously unselected package x11-common.
2026-Feb-25 07:13:20.201202
#37 168.8 Preparing to unpack .../214-x11-common_1%3a7.7+23ubuntu2_all.deb ...
2026-Feb-25 07:13:20.201202
#37 168.8 Unpacking x11-common (1:7.7+23ubuntu2) ...
2026-Feb-25 07:13:20.201202
#37 168.9 Selecting previously unselected package libxss1:amd64.
2026-Feb-25 07:13:20.312549
#37 168.9 Preparing to unpack .../215-libxss1_1%3a1.2.3-1build2_amd64.deb ...
2026-Feb-25 07:13:20.312549
#37 168.9 Unpacking libxss1:amd64 (1:1.2.3-1build2) ...
2026-Feb-25 07:13:20.312549
#37 168.9 Selecting previously unselected package libsdl2-2.0-0:amd64.
2026-Feb-25 07:13:20.312549
#37 168.9 Preparing to unpack .../216-libsdl2-2.0-0_2.0.20+dfsg-2ubuntu1.22.04.1_amd64.deb ...
2026-Feb-25 07:13:20.312549
#37 168.9 Unpacking libsdl2-2.0-0:amd64 (2.0.20+dfsg-2ubuntu1.22.04.1) ...
2026-Feb-25 07:13:20.312549
#37 168.9 Selecting previously unselected package libxcb-shape0:amd64.
2026-Feb-25 07:13:20.312549
#37 168.9 Preparing to unpack .../217-libxcb-shape0_1.14-3ubuntu3_amd64.deb ...
2026-Feb-25 07:13:20.312549
#37 168.9 Unpacking libxcb-shape0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:13:20.312549
#37 168.9 Selecting previously unselected package libxv1:amd64.
2026-Feb-25 07:13:20.312549
#37 168.9 Preparing to unpack .../218-libxv1_2%3a1.0.11-1build2_amd64.deb ...
2026-Feb-25 07:13:20.312549
#37 168.9 Unpacking libxv1:amd64 (2:1.0.11-1build2) ...
2026-Feb-25 07:13:20.312549
#37 169.0 Selecting previously unselected package libavdevice58:amd64.
2026-Feb-25 07:13:20.312549
#37 169.0 Preparing to unpack .../219-libavdevice58_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:13:20.312549
#37 169.0 Unpacking libavdevice58:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:20.312549
#37 169.0 Selecting previously unselected package ffmpeg.
2026-Feb-25 07:13:20.431979
#37 169.0 Preparing to unpack .../220-ffmpeg_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:13:20.431979
#37 169.0 Unpacking ffmpeg (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:20.431979
#37 169.0 Selecting previously unselected package icu-devtools.
2026-Feb-25 07:13:20.431979
#37 169.0 Preparing to unpack .../221-icu-devtools_70.1-2_amd64.deb ...
2026-Feb-25 07:13:20.431979
#37 169.0 Unpacking icu-devtools (70.1-2) ...
2026-Feb-25 07:13:20.431979
#37 169.0 Selecting previously unselected package libigdgmm12:amd64.
2026-Feb-25 07:13:20.431979
#37 169.0 Preparing to unpack .../222-libigdgmm12_22.1.2+ds1-1_amd64.deb ...
2026-Feb-25 07:13:20.431979
#37 169.0 Unpacking libigdgmm12:amd64 (22.1.2+ds1-1) ...
2026-Feb-25 07:13:20.431979
#37 169.1 Selecting previously unselected package intel-media-va-driver:amd64.
2026-Feb-25 07:13:20.431979
#37 169.1 Preparing to unpack .../223-intel-media-va-driver_22.3.1+dfsg1-1ubuntu2_amd64.deb ...
2026-Feb-25 07:13:20.431979
#37 169.1 Unpacking intel-media-va-driver:amd64 (22.3.1+dfsg1-1ubuntu2) ...
2026-Feb-25 07:13:20.431979
#37 169.1 Selecting previously unselected package libaacs0:amd64.
2026-Feb-25 07:13:20.605548
#37 169.1 Preparing to unpack .../224-libaacs0_0.11.1-1_amd64.deb ...
2026-Feb-25 07:13:20.605548
#37 169.1 Unpacking libaacs0:amd64 (0.11.1-1) ...
2026-Feb-25 07:13:20.605548
#37 169.1 Selecting previously unselected package libavutil-dev:amd64.
2026-Feb-25 07:13:20.605548
#37 169.1 Preparing to unpack .../225-libavutil-dev_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:13:20.605548
#37 169.1 Unpacking libavutil-dev:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:20.605548
#37 169.2 Selecting previously unselected package libswresample-dev:amd64.
2026-Feb-25 07:13:20.605548
#37 169.2 Preparing to unpack .../226-libswresample-dev_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:13:20.605548
#37 169.2 Unpacking libswresample-dev:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:20.605548
#37 169.2 Selecting previously unselected package libavcodec-dev:amd64.
2026-Feb-25 07:13:20.605548
#37 169.2 Preparing to unpack .../227-libavcodec-dev_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:13:20.605548
#37 169.2 Unpacking libavcodec-dev:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:20.605548
#37 169.3 Selecting previously unselected package libavformat-dev:amd64.
2026-Feb-25 07:13:20.731312
#37 169.3 Preparing to unpack .../228-libavformat-dev_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:13:20.731312
#37 169.3 Unpacking libavformat-dev:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:20.731312
#37 169.3 Selecting previously unselected package libpostproc-dev:amd64.
2026-Feb-25 07:13:20.731312
#37 169.3 Preparing to unpack .../229-libpostproc-dev_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:13:20.731312
#37 169.3 Unpacking libpostproc-dev:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:20.731312
#37 169.3 Selecting previously unselected package libswscale-dev:amd64.
2026-Feb-25 07:13:20.731312
#37 169.3 Preparing to unpack .../230-libswscale-dev_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:13:20.731312
#37 169.3 Unpacking libswscale-dev:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:20.731312
#37 169.4 Selecting previously unselected package libavfilter-dev:amd64.
2026-Feb-25 07:13:20.731312
#37 169.4 Preparing to unpack .../231-libavfilter-dev_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:13:20.731312
#37 169.4 Unpacking libavfilter-dev:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:20.731312
#37 169.4 Selecting previously unselected package libavdevice-dev:amd64.
2026-Feb-25 07:13:20.928381
#37 169.4 Preparing to unpack .../232-libavdevice-dev_7%3a4.4.2-0ubuntu0.22.04.1_amd64.deb ...
2026-Feb-25 07:13:20.928381
#37 169.4 Unpacking libavdevice-dev:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:20.928381
#37 169.4 Selecting previously unselected package libbdplus0:amd64.
2026-Feb-25 07:13:20.928381
#37 169.4 Preparing to unpack .../233-libbdplus0_0.2.0-1_amd64.deb ...
2026-Feb-25 07:13:20.928381
#37 169.4 Unpacking libbdplus0:amd64 (0.2.0-1) ...
2026-Feb-25 07:13:20.928381
#37 169.4 Selecting previously unselected package libclang-14-dev.
2026-Feb-25 07:13:20.928381
#37 169.4 Preparing to unpack .../234-libclang-14-dev_1%3a14.0.0-1ubuntu1.1_amd64.deb ...
2026-Feb-25 07:13:20.928381
#37 169.4 Unpacking libclang-14-dev (1:14.0.0-1ubuntu1.1) ...
2026-Feb-25 07:13:21.512559
#37 170.2 Selecting previously unselected package libclang-dev.
2026-Feb-25 07:13:21.647324
#37 170.2 Preparing to unpack .../235-libclang-dev_1%3a14.0-55~exp2_amd64.deb ...
2026-Feb-25 07:13:21.647324
#37 170.2 Unpacking libclang-dev (1:14.0-55~exp2) ...
2026-Feb-25 07:13:21.647324
#37 170.2 Selecting previously unselected package libdecor-0-plugin-1-cairo:amd64.
2026-Feb-25 07:13:21.647324
#37 170.2 Preparing to unpack .../236-libdecor-0-plugin-1-cairo_0.1.0-3build1_amd64.deb ...
2026-Feb-25 07:13:21.647324
#37 170.2 Unpacking libdecor-0-plugin-1-cairo:amd64 (0.1.0-3build1) ...
2026-Feb-25 07:13:21.647324
#37 170.2 Selecting previously unselected package libgdk-pixbuf2.0-bin.
2026-Feb-25 07:13:21.647324
#37 170.2 Preparing to unpack .../237-libgdk-pixbuf2.0-bin_2.42.8+dfsg-1ubuntu0.4_amd64.deb ...
2026-Feb-25 07:13:21.647324
#37 170.2 Unpacking libgdk-pixbuf2.0-bin (2.42.8+dfsg-1ubuntu0.4) ...
2026-Feb-25 07:13:21.647324
#37 170.2 Selecting previously unselected package libgl1-amber-dri:amd64.
2026-Feb-25 07:13:21.647324
#37 170.2 Preparing to unpack .../238-libgl1-amber-dri_21.3.9-0ubuntu1~22.04.1_amd64.deb ...
2026-Feb-25 07:13:21.647324
#37 170.2 Unpacking libgl1-amber-dri:amd64 (21.3.9-0ubuntu1~22.04.1) ...
2026-Feb-25 07:13:21.647324
#37 170.3 Selecting previously unselected package libicu-dev:amd64.
2026-Feb-25 07:13:21.801453
#37 170.3 Preparing to unpack .../239-libicu-dev_70.1-2_amd64.deb ...
2026-Feb-25 07:13:21.801453
#37 170.3 Unpacking libicu-dev:amd64 (70.1-2) ...
2026-Feb-25 07:13:21.835786
#37 170.5 Selecting previously unselected package libncurses-dev:amd64.
2026-Feb-25 07:13:21.947912
#37 170.5 Preparing to unpack .../240-libncurses-dev_6.3-2ubuntu0.1_amd64.deb ...
2026-Feb-25 07:13:21.947912
#37 170.5 Unpacking libncurses-dev:amd64 (6.3-2ubuntu0.1) ...
2026-Feb-25 07:13:21.947912
#37 170.5 Selecting previously unselected package libprotobuf-lite23:amd64.
2026-Feb-25 07:13:21.947912
#37 170.5 Preparing to unpack .../241-libprotobuf-lite23_3.12.4-1ubuntu7.22.04.4_amd64.deb ...
2026-Feb-25 07:13:21.947912
#37 170.6 Unpacking libprotobuf-lite23:amd64 (3.12.4-1ubuntu7.22.04.4) ...
2026-Feb-25 07:13:21.947912
#37 170.6 Selecting previously unselected package libprotobuf23:amd64.
2026-Feb-25 07:13:21.947912
#37 170.6 Preparing to unpack .../242-libprotobuf23_3.12.4-1ubuntu7.22.04.4_amd64.deb ...
2026-Feb-25 07:13:21.947912
#37 170.6 Unpacking libprotobuf23:amd64 (3.12.4-1ubuntu7.22.04.4) ...
2026-Feb-25 07:13:21.947912
#37 170.6 Selecting previously unselected package libprotoc23:amd64.
2026-Feb-25 07:13:22.072100
#37 170.6 Preparing to unpack .../243-libprotoc23_3.12.4-1ubuntu7.22.04.4_amd64.deb ...
2026-Feb-25 07:13:22.072100
#37 170.6 Unpacking libprotoc23:amd64 (3.12.4-1ubuntu7.22.04.4) ...
2026-Feb-25 07:13:22.072100
#37 170.6 Selecting previously unselected package librsvg2-common:amd64.
2026-Feb-25 07:13:22.072100
#37 170.6 Preparing to unpack .../244-librsvg2-common_2.52.5+dfsg-3ubuntu0.2_amd64.deb ...
2026-Feb-25 07:13:22.072100
#37 170.6 Unpacking librsvg2-common:amd64 (2.52.5+dfsg-3ubuntu0.2) ...
2026-Feb-25 07:13:22.072100
#37 170.7 Selecting previously unselected package libssl-dev:amd64.
2026-Feb-25 07:13:22.072100
#37 170.7 Preparing to unpack .../245-libssl-dev_3.0.2-0ubuntu1.21_amd64.deb ...
2026-Feb-25 07:13:22.072100
#37 170.7 Unpacking libssl-dev:amd64 (3.0.2-0ubuntu1.21) ...
2026-Feb-25 07:13:22.072100
#37 170.7 Selecting previously unselected package libtinfo-dev:amd64.
2026-Feb-25 07:13:22.181403
#37 170.7 Preparing to unpack .../246-libtinfo-dev_6.3-2ubuntu0.1_amd64.deb ...
2026-Feb-25 07:13:22.181403
#37 170.7 Unpacking libtinfo-dev:amd64 (6.3-2ubuntu0.1) ...
2026-Feb-25 07:13:22.181403
#37 170.8 Selecting previously unselected package libxml2-dev:amd64.
2026-Feb-25 07:13:22.181403
#37 170.8 Preparing to unpack .../247-libxml2-dev_2.9.13+dfsg-1ubuntu0.11_amd64.deb ...
2026-Feb-25 07:13:22.181403
#37 170.8 Unpacking libxml2-dev:amd64 (2.9.13+dfsg-1ubuntu0.11) ...
2026-Feb-25 07:13:22.181403
#37 170.8 Selecting previously unselected package llvm-14-runtime.
2026-Feb-25 07:13:22.181403
#37 170.8 Preparing to unpack .../248-llvm-14-runtime_1%3a14.0.0-1ubuntu1.1_amd64.deb ...
2026-Feb-25 07:13:22.181403
#37 170.8 Unpacking llvm-14-runtime (1:14.0.0-1ubuntu1.1) ...
2026-Feb-25 07:13:22.181403
#37 170.8 Selecting previously unselected package libpfm4:amd64.
2026-Feb-25 07:13:22.181403
#37 170.8 Preparing to unpack .../249-libpfm4_4.11.1+git32-gd0b85fb-1ubuntu0.1_amd64.deb ...
2026-Feb-25 07:13:22.181403
#37 170.8 Unpacking libpfm4:amd64 (4.11.1+git32-gd0b85fb-1ubuntu0.1) ...
2026-Feb-25 07:13:22.181403
#37 170.9 Selecting previously unselected package llvm-14.
2026-Feb-25 07:13:22.335582
#37 170.9 Preparing to unpack .../250-llvm-14_1%3a14.0.0-1ubuntu1.1_amd64.deb ...
2026-Feb-25 07:13:22.335582
#37 170.9 Unpacking llvm-14 (1:14.0.0-1ubuntu1.1) ...
2026-Feb-25 07:13:22.381187
#37 171.1 Selecting previously unselected package libffi-dev:amd64.
2026-Feb-25 07:13:22.483409
#37 171.1 Preparing to unpack .../251-libffi-dev_3.4.2-4_amd64.deb ...
2026-Feb-25 07:13:22.483409
#37 171.1 Unpacking libffi-dev:amd64 (3.4.2-4) ...
2026-Feb-25 07:13:22.483409
#37 171.1 Selecting previously unselected package python3-pygments.
2026-Feb-25 07:13:22.483409
#37 171.1 Preparing to unpack .../252-python3-pygments_2.11.2+dfsg-2ubuntu0.1_all.deb ...
2026-Feb-25 07:13:22.483409
#37 171.1 Unpacking python3-pygments (2.11.2+dfsg-2ubuntu0.1) ...
2026-Feb-25 07:13:22.483409
#37 171.2 Selecting previously unselected package llvm-14-tools.
2026-Feb-25 07:13:22.483409
#37 171.2 Preparing to unpack .../253-llvm-14-tools_1%3a14.0.0-1ubuntu1.1_amd64.deb ...
2026-Feb-25 07:13:22.654330
#37 171.2 Unpacking llvm-14-tools (1:14.0.0-1ubuntu1.1) ...
2026-Feb-25 07:13:22.654330
#37 171.2 Selecting previously unselected package libz3-4:amd64.
2026-Feb-25 07:13:22.654330
#37 171.3 Preparing to unpack .../254-libz3-4_4.8.12-1_amd64.deb ...
2026-Feb-25 07:13:22.654330
#37 171.3 Unpacking libz3-4:amd64 (4.8.12-1) ...
2026-Feb-25 07:13:22.654330
#37 171.3 Selecting previously unselected package libz3-dev:amd64.
2026-Feb-25 07:13:22.834203
#37 171.3 Preparing to unpack .../255-libz3-dev_4.8.12-1_amd64.deb ...
2026-Feb-25 07:13:22.834203
#37 171.3 Unpacking libz3-dev:amd64 (4.8.12-1) ...
2026-Feb-25 07:13:22.834203
#37 171.4 Selecting previously unselected package llvm-14-dev.
2026-Feb-25 07:13:22.834203
#37 171.4 Preparing to unpack .../256-llvm-14-dev_1%3a14.0.0-1ubuntu1.1_amd64.deb ...
2026-Feb-25 07:13:22.834203
#37 171.4 Unpacking llvm-14-dev (1:14.0.0-1ubuntu1.1) ...
2026-Feb-25 07:13:23.554349
#37 172.2 Selecting previously unselected package mesa-va-drivers:amd64.
2026-Feb-25 07:13:23.687178
#37 172.2 Preparing to unpack .../257-mesa-va-drivers_23.2.1-1ubuntu3.1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:23.687178
#37 172.2 Unpacking mesa-va-drivers:amd64 (23.2.1-1ubuntu3.1~22.04.3) ...
2026-Feb-25 07:13:23.687178
#37 172.3 Selecting previously unselected package mesa-vdpau-drivers:amd64.
2026-Feb-25 07:13:23.687178
#37 172.3 Preparing to unpack .../258-mesa-vdpau-drivers_23.2.1-1ubuntu3.1~22.04.3_amd64.deb ...
2026-Feb-25 07:13:23.687178
#37 172.3 Unpacking mesa-vdpau-drivers:amd64 (23.2.1-1ubuntu3.1~22.04.3) ...
2026-Feb-25 07:13:23.687178
#37 172.4 Selecting previously unselected package pkg-config.
2026-Feb-25 07:13:23.792424
#37 172.4 Preparing to unpack .../259-pkg-config_0.29.2-1ubuntu3_amd64.deb ...
2026-Feb-25 07:13:23.792424
#37 172.4 Unpacking pkg-config (0.29.2-1ubuntu3) ...
2026-Feb-25 07:13:23.792424
#37 172.4 Selecting previously unselected package i965-va-driver:amd64.
2026-Feb-25 07:13:23.792424
#37 172.4 Preparing to unpack .../260-i965-va-driver_2.4.1+dfsg1-1_amd64.deb ...
2026-Feb-25 07:13:23.792424
#37 172.4 Unpacking i965-va-driver:amd64 (2.4.1+dfsg1-1) ...
2026-Feb-25 07:13:23.792424
#37 172.4 Selecting previously unselected package va-driver-all:amd64.
2026-Feb-25 07:13:23.792424
#37 172.4 Preparing to unpack .../261-va-driver-all_2.14.0-1_amd64.deb ...
2026-Feb-25 07:13:23.792424
#37 172.4 Unpacking va-driver-all:amd64 (2.14.0-1) ...
2026-Feb-25 07:13:23.792424
#37 172.4 Selecting previously unselected package vdpau-driver-all:amd64.
2026-Feb-25 07:13:23.792424
#37 172.4 Preparing to unpack .../262-vdpau-driver-all_1.4-3build2_amd64.deb ...
2026-Feb-25 07:13:23.792424
#37 172.4 Unpacking vdpau-driver-all:amd64 (1.4-3build2) ...
2026-Feb-25 07:13:23.792424
#37 172.5 Selecting previously unselected package zlib1g-dev:amd64.
2026-Feb-25 07:13:24.028174
#37 172.5 Preparing to unpack .../263-zlib1g-dev_1%3a1.2.11.dfsg-2ubuntu9.2_amd64.deb ...
2026-Feb-25 07:13:24.028174
#37 172.5 Unpacking zlib1g-dev:amd64 (1:1.2.11.dfsg-2ubuntu9.2) ...
2026-Feb-25 07:13:24.028174
#37 172.5 Selecting previously unselected package libprotobuf-dev:amd64.
2026-Feb-25 07:13:24.028174
#37 172.5 Preparing to unpack .../264-libprotobuf-dev_3.12.4-1ubuntu7.22.04.4_amd64.deb ...
2026-Feb-25 07:13:24.028174
#37 172.5 Unpacking libprotobuf-dev:amd64 (3.12.4-1ubuntu7.22.04.4) ...
2026-Feb-25 07:13:24.028174
#37 172.5 Selecting previously unselected package pocketsphinx-en-us.
2026-Feb-25 07:13:24.028174
#37 172.5 Preparing to unpack .../265-pocketsphinx-en-us_0.8.0+real5prealpha+1-14ubuntu1_all.deb ...
2026-Feb-25 07:13:24.028174
#37 172.5 Unpacking pocketsphinx-en-us (0.8.0+real5prealpha+1-14ubuntu1) ...
2026-Feb-25 07:13:24.028174
#37 172.7 Selecting previously unselected package protobuf-compiler.
2026-Feb-25 07:13:24.129757
#37 172.7 Preparing to unpack .../266-protobuf-compiler_3.12.4-1ubuntu7.22.04.4_amd64.deb ...
2026-Feb-25 07:13:24.129757
#37 172.7 Unpacking protobuf-compiler (3.12.4-1ubuntu7.22.04.4) ...
2026-Feb-25 07:13:24.129757
#37 172.7 Setting up libgme0:amd64 (0.6.3-2) ...
2026-Feb-25 07:13:24.129757
#37 172.7 Setting up libssh-gcrypt-4:amd64 (0.9.6-2ubuntu0.22.04.6) ...
2026-Feb-25 07:13:24.129757
#37 172.7 Setting up media-types (7.0.0) ...
2026-Feb-25 07:13:24.129757
#37 172.7 Setting up libpipeline1:amd64 (1.5.5-1) ...
2026-Feb-25 07:13:24.129757
#37 172.8 Setting up libgraphite2-3:amd64 (1.3.14-1build2) ...
2026-Feb-25 07:13:24.129757
#37 172.8 Setting up libsrt1.4-gnutls:amd64 (1.4.4-4) ...
2026-Feb-25 07:13:24.129757
#37 172.8 Setting up libpixman-1-0:amd64 (0.40.0-1ubuntu0.22.04.1) ...
2026-Feb-25 07:13:24.129757
#37 172.8 Setting up libudfread0:amd64 (1.1.2-1) ...
2026-Feb-25 07:13:24.129757
#37 172.8 Setting up gcc-11-base:amd64 (11.4.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:24.129757
#37 172.8 Setting up libwayland-server0:amd64 (1.20.0-1ubuntu0.1) ...
2026-Feb-25 07:13:24.129757
#37 172.8 Setting up libaom3:amd64 (3.3.0-1ubuntu0.1) ...
2026-Feb-25 07:13:24.129757
#37 172.8 Setting up libpciaccess0:amd64 (0.16-3) ...
2026-Feb-25 07:13:24.129757
#37 172.8 Setting up librabbitmq4:amd64 (0.10.0-1ubuntu2) ...
2026-Feb-25 07:13:24.129757
#37 172.8 Setting up libxau6:amd64 (1:1.0.9-1build5) ...
2026-Feb-25 07:13:24.129757
#37 172.8 Setting up libraw1394-11:amd64 (2.1.2-2build2) ...
2026-Feb-25 07:13:24.129757
#37 172.8 Setting up libapparmor1:amd64 (3.0.4-2ubuntu2.5) ...
2026-Feb-25 07:13:24.129757
#37 172.8 Setting up libpsl5:amd64 (0.21.0-1.2build2) ...
2026-Feb-25 07:13:24.129757
#37 172.8 Setting up libcodec2-1.0:amd64 (1.0.1-3) ...
2026-Feb-25 07:13:24.129757
#37 172.8 Setting up libsodium23:amd64 (1.0.18-1ubuntu0.22.04.1) ...
2026-Feb-25 07:13:24.129757
#37 172.8 Setting up libmpg123-0:amd64 (1.29.3-1ubuntu0.1) ...
2026-Feb-25 07:13:24.129757
#37 172.8 Setting up libogg0:amd64 (1.3.5-0ubuntu3) ...
2026-Feb-25 07:13:24.129757
#37 172.8 Setting up libspeex1:amd64 (1.2~rc1.2-1.1ubuntu3) ...
2026-Feb-25 07:13:24.297767
#37 172.8 Setting up libshine3:amd64 (3.1.1-2) ...
2026-Feb-25 07:13:24.297767
#37 172.8 Setting up libtwolame0:amd64 (0.4.0-2build2) ...
2026-Feb-25 07:13:24.297767
#37 172.8 Setting up libdatrie1:amd64 (0.2.13-2) ...
2026-Feb-25 07:13:24.297767
#37 172.8 Setting up xdg-user-dirs (0.17-2ubuntu4) ...
2026-Feb-25 07:13:24.297767
#37 172.8 Setting up libgsm1:amd64 (1.0.19-1) ...
2026-Feb-25 07:13:24.297767
#37 172.8 Setting up libyaml-0-2:amd64 (0.2.2-1build2) ...
2026-Feb-25 07:13:24.297767
#37 172.8 Setting up libglib2.0-0:amd64 (2.72.4-0ubuntu2.9) ...
2026-Feb-25 07:13:24.297767
#37 172.8 No schema files found: doing nothing.
2026-Feb-25 07:13:24.297767
#37 172.8 Setting up libglvnd0:amd64 (1.4.0-1) ...
2026-Feb-25 07:13:24.297767
#37 172.8 Setting up libpgm-5.3-0:amd64 (5.3.128~dfsg-2) ...
2026-Feb-25 07:13:24.297767
#37 172.8 Setting up libbrotli1:amd64 (1.0.9-2build6) ...
2026-Feb-25 07:13:24.297767
#37 172.8 Setting up libgdk-pixbuf2.0-common (2.42.8+dfsg-1ubuntu0.4) ...
2026-Feb-25 07:13:24.297767
#37 172.9 Setting up libnorm1:amd64 (1.5.9+dfsg-2) ...
2026-Feb-25 07:13:24.297767
#37 172.9 Setting up libmysofa1:amd64 (1.2.1~dfsg0-1) ...
2026-Feb-25 07:13:24.297767
#37 172.9 Setting up x11-common (1:7.7+23ubuntu2) ...
2026-Feb-25 07:13:24.297767
#37 173.0 debconf: unable to initialize frontend: Dialog
2026-Feb-25 07:13:24.297767
#37 173.0 debconf: (TERM is not set, so the dialog frontend is not usable.)
2026-Feb-25 07:13:24.297767
#37 173.0 debconf: falling back to frontend: Readline
2026-Feb-25 07:13:24.398117
#37 173.0 invoke-rc.d: could not determine current runlevel
2026-Feb-25 07:13:24.398117
#37 173.0 invoke-rc.d: policy-rc.d denied execution of start.
2026-Feb-25 07:13:24.398117
#37 173.0 Setting up libsensors-config (1:3.6.0-7ubuntu1) ...
2026-Feb-25 07:13:24.398117
#37 173.0 Setting up libnghttp2-14:amd64 (1.43.0-1ubuntu0.2) ...
2026-Feb-25 07:13:24.398117
#37 173.0 Setting up libdeflate0:amd64 (1.10-2) ...
2026-Feb-25 07:13:24.398117
#37 173.0 Setting up xkb-data (2.33-1) ...
2026-Feb-25 07:13:24.398117
#37 173.0 Setting up libprotobuf23:amd64 (3.12.4-1ubuntu7.22.04.4) ...
2026-Feb-25 07:13:24.398117
#37 173.0 Setting up libigdgmm12:amd64 (22.1.2+ds1-1) ...
2026-Feb-25 07:13:24.398117
#37 173.0 Setting up libgomp1:amd64 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:24.398117
#37 173.0 Setting up libcdio19:amd64 (2.1.0-3ubuntu0.2) ...
2026-Feb-25 07:13:24.398117
#37 173.0 Setting up libxvidcore4:amd64 (2:1.3.7-1) ...
2026-Feb-25 07:13:24.398117
#37 173.0 Setting up libffi-dev:amd64 (3.4.2-4) ...
2026-Feb-25 07:13:24.398117
#37 173.0 Setting up libjbig0:amd64 (2.1-3.1ubuntu0.22.04.1) ...
2026-Feb-25 07:13:24.398117
#37 173.0 Setting up libasan6:amd64 (11.4.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:24.398117
#37 173.0 Setting up libsnappy1v5:amd64 (1.1.8-1build3) ...
2026-Feb-25 07:13:24.398117
#37 173.0 Setting up libflac8:amd64 (1.3.3-2ubuntu0.2) ...
2026-Feb-25 07:13:24.398117
#37 173.0 Setting up libasound2-data (1.2.6.1-1ubuntu1.1) ...
2026-Feb-25 07:13:24.398117
#37 173.0 Setting up libprotobuf-lite23:amd64 (3.12.4-1ubuntu7.22.04.4) ...
2026-Feb-25 07:13:24.398117
#37 173.1 Setting up libz3-4:amd64 (4.8.12-1) ...
2026-Feb-25 07:13:24.398117
#37 173.1 Setting up libblas3:amd64 (3.10.0-2ubuntu1) ...
2026-Feb-25 07:13:24.398117
#37 173.1 update-alternatives: using /usr/lib/x86_64-linux-gnu/blas/libblas.so.3 to provide /usr/lib/x86_64-linux-gnu/libblas.so.3 (libblas.so.3-x86_64-linux-gnu) in auto mode
2026-Feb-25 07:13:24.398117
#37 173.1 Setting up libglib2.0-data (2.72.4-0ubuntu2.9) ...
2026-Feb-25 07:13:24.398117
#37 173.1 Setting up libpfm4:amd64 (4.11.1+git32-gd0b85fb-1ubuntu0.1) ...
2026-Feb-25 07:13:24.398117
#37 173.1 Setting up libslang2:amd64 (2.3.2-5build4) ...
2026-Feb-25 07:13:24.498799
#37 173.1 Setting up libva2:amd64 (2.14.0-1) ...
2026-Feb-25 07:13:24.498799
#37 173.1 Setting up libx11-data (2:1.7.5-1ubuntu0.3) ...
2026-Feb-25 07:13:24.498799
#37 173.1 Setting up librtmp1:amd64 (2.4+20151223.gitfa8646d.1-2build4) ...
2026-Feb-25 07:13:24.498799
#37 173.1 Setting up libprotoc23:amd64 (3.12.4-1ubuntu7.22.04.4) ...
2026-Feb-25 07:13:24.498799
#37 173.1 Setting up libx264-163:amd64 (2:0.163.3060+git5db6aa6-2build1) ...
2026-Feb-25 07:13:24.498799
#37 173.1 Setting up libdbus-1-3:amd64 (1.12.20-2ubuntu4.1) ...
2026-Feb-25 07:13:24.498799
#37 173.1 Setting up dbus (1.12.20-2ubuntu4.1) ...
2026-Feb-25 07:13:24.498799
#37 173.2 Setting up libfribidi0:amd64 (1.0.8-2ubuntu3.1) ...
2026-Feb-25 07:13:24.498799
#37 173.2 Setting up libopus0:amd64 (1.3.1-0.1build2) ...
2026-Feb-25 07:13:24.640926
#37 173.2 Setting up libquadmath0:amd64 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:24.640926
#37 173.2 Setting up intel-media-va-driver:amd64 (22.3.1+dfsg1-1ubuntu2) ...
2026-Feb-25 07:13:24.640926
#37 173.2 Setting up libssl-dev:amd64 (3.0.2-0ubuntu1.21) ...
2026-Feb-25 07:13:24.640926
#37 173.2 Setting up libpng16-16:amd64 (1.6.37-3ubuntu0.4) ...
2026-Feb-25 07:13:24.640926
#37 173.2 Setting up libatomic1:amd64 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:24.640926
#37 173.2 Setting up libvorbis0a:amd64 (1.3.7-1build2) ...
2026-Feb-25 07:13:24.640926
#37 173.2 Setting up binfmt-support (2.2.1-2) ...
2026-Feb-25 07:13:24.640926
#37 173.2 invoke-rc.d: could not determine current runlevel
2026-Feb-25 07:13:24.640926
#37 173.2 invoke-rc.d: policy-rc.d denied execution of restart.
2026-Feb-25 07:13:24.640926
#37 173.3 Setting up pkg-config (0.29.2-1ubuntu3) ...
2026-Feb-25 07:13:24.750370
#37 173.3 Setting up fonts-dejavu-core (2.37-2build1) ...
2026-Feb-25 07:13:24.750370
#37 173.4 Setting up ucf (3.0043) ...
2026-Feb-25 07:13:24.750370
#37 173.4 debconf: unable to initialize frontend: Dialog
2026-Feb-25 07:13:24.750370
#37 173.4 debconf: (TERM is not set, so the dialog frontend is not usable.)
2026-Feb-25 07:13:24.750370
#37 173.4 debconf: falling back to frontend: Readline
2026-Feb-25 07:13:24.854767
#37 173.4 Setting up libsensors5:amd64 (1:3.6.0-7ubuntu1) ...
2026-Feb-25 07:13:24.854767
#37 173.4 Setting up libaacs0:amd64 (0.11.1-1) ...
2026-Feb-25 07:13:24.854767
#37 173.5 Setting up libjpeg-turbo8:amd64 (2.1.2-0ubuntu1) ...
2026-Feb-25 07:13:24.854767
#37 173.5 Setting up pocketsphinx-en-us (0.8.0+real5prealpha+1-14ubuntu1) ...
2026-Feb-25 07:13:24.854767
#37 173.5 Setting up libgc1:amd64 (1:8.0.6-1.1build1) ...
2026-Feb-25 07:13:24.854767
#37 173.5 Setting up libglapi-mesa:amd64 (23.2.1-1ubuntu3.1~22.04.3) ...
2026-Feb-25 07:13:24.854767
#37 173.5 Setting up libssh-4:amd64 (0.9.6-2ubuntu0.22.04.6) ...
2026-Feb-25 07:13:24.854767
#37 173.5 Setting up libgfortran5:amd64 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:24.854767
#37 173.5 Setting up libwebp7:amd64 (1.2.2-2ubuntu0.22.04.2) ...
2026-Feb-25 07:13:24.854767
#37 173.5 Setting up libubsan1:amd64 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:24.854767
#37 173.5 Setting up libbdplus0:amd64 (0.2.0-1) ...
2026-Feb-25 07:13:24.854767
#37 173.5 Setting up libnuma1:amd64 (2.0.14-3ubuntu2) ...
2026-Feb-25 07:13:24.854767
#37 173.5 Setting up libvidstab1.1:amd64 (1.1.0-2) ...
2026-Feb-25 07:13:24.854767
#37 173.5 Setting up libmd0:amd64 (1.0.4-1build1) ...
2026-Feb-25 07:13:24.854767
#37 173.5 Setting up alsa-topology-conf (1.2.5.1-2) ...
2026-Feb-25 07:13:24.854767
#37 173.5 Setting up ocl-icd-libopencl1:amd64 (2.2.14-3) ...
2026-Feb-25 07:13:24.854767
#37 173.5 Setting up libasyncns0:amd64 (0.8-6build2) ...
2026-Feb-25 07:13:24.854767
#37 173.5 Setting up libxshmfence1:amd64 (1.3-1build4) ...
2026-Feb-25 07:13:24.854767
#37 173.5 Setting up libbs2b0:amd64 (3.1.0+dfsg-2.2build1) ...
2026-Feb-25 07:13:24.854767
#37 173.5 Setting up libasound2:amd64 (1.2.6.1-1ubuntu1.1) ...
2026-Feb-25 07:13:24.854767
#37 173.5 Setting up libmpdec3:amd64 (2.5.1-2build2) ...
2026-Feb-25 07:13:24.854767
#37 173.5 Setting up libc6-i386 (2.35-0ubuntu3.13) ...
2026-Feb-25 07:13:24.854767
#37 173.5 Setting up libzimg2:amd64 (3.0.3+ds1-1) ...
2026-Feb-25 07:13:24.854767
#37 173.5 Setting up libcurl4:amd64 (7.81.0-1ubuntu1.22) ...
2026-Feb-25 07:13:24.854767
#37 173.5 Setting up libopenjp2-7:amd64 (2.4.0-6ubuntu0.4) ...
2026-Feb-25 07:13:24.854767
#37 173.5 Setting up libopenal-data (1:1.19.1-2build3) ...
2026-Feb-25 07:13:24.854767
#37 173.5 Setting up libthai-data (0.1.29-1build1) ...
2026-Feb-25 07:13:24.955411
#37 173.5 Setting up libvpx7:amd64 (1.11.0-2ubuntu2.5) ...
2026-Feb-25 07:13:24.955411
#37 173.5 Setting up curl (7.81.0-1ubuntu1.22) ...
2026-Feb-25 07:13:24.955411
#37 173.5 Setting up libwayland-egl1:amd64 (1.20.0-1ubuntu0.1) ...
2026-Feb-25 07:13:24.955411
#37 173.5 Setting up libusb-1.0-0:amd64 (2:1.0.25-1ubuntu2) ...
2026-Feb-25 07:13:24.955411
#37 173.5 Setting up libdav1d5:amd64 (0.9.2-1) ...
2026-Feb-25 07:13:24.955411
#37 173.5 Setting up libmfx1:amd64 (22.3.0-1) ...
2026-Feb-25 07:13:24.955411
#37 173.5 Setting up libc-dev-bin (2.35-0ubuntu3.13) ...
2026-Feb-25 07:13:24.955411
#37 173.5 Setting up libsamplerate0:amd64 (0.2.2-1build1) ...
2026-Feb-25 07:13:24.955411
#37 173.6 Setting up libwebpmux3:amd64 (1.2.2-2ubuntu0.22.04.2) ...
2026-Feb-25 07:13:24.955411
#37 173.6 Setting up libbsd0:amd64 (0.11.5-1) ...
2026-Feb-25 07:13:24.955411
#37 173.6 Setting up libdrm-common (2.4.113-2~ubuntu0.22.04.1) ...
2026-Feb-25 07:13:24.955411
#37 173.6 Setting up libelf1:amd64 (0.186-1ubuntu0.1) ...
2026-Feb-25 07:13:24.955411
#37 173.6 Setting up publicsuffix (20211207.1025-1) ...
2026-Feb-25 07:13:24.955411
#37 173.6 Setting up libcc1-0:amd64 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:24.955411
#37 173.6 Setting up libzvbi-common (0.2.35-19) ...
2026-Feb-25 07:13:24.955411
#37 173.6 Setting up liblsan0:amd64 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:24.955411
#37 173.6 Setting up libmp3lame0:amd64 (3.100-3build2) ...
2026-Feb-25 07:13:24.955411
#37 173.6 Setting up libitm1:amd64 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:24.955411
#37 173.6 Setting up libvorbisenc2:amd64 (1.3.7-1build2) ...
2026-Feb-25 07:13:24.955411
#37 173.6 Setting up libicu70:amd64 (70.1-2) ...
2026-Feb-25 07:13:24.955411
#37 173.6 Setting up libiec61883-0:amd64 (1.2.0-4build3) ...
2026-Feb-25 07:13:24.955411
#37 173.6 Setting up libserd-0-0:amd64 (0.30.10-2) ...
2026-Feb-25 07:13:24.955411
#37 173.6 Setting up libtsan0:amd64 (11.4.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:24.955411
#37 173.6 Setting up libxkbcommon0:amd64 (1.4.0-1) ...
2026-Feb-25 07:13:24.955411
#37 173.6 Setting up libwayland-client0:amd64 (1.20.0-1ubuntu0.1) ...
2026-Feb-25 07:13:24.955411
#37 173.6 Setting up libjpeg8:amd64 (8c-2ubuntu10) ...
2026-Feb-25 07:13:24.955411
#37 173.6 Setting up libavc1394-0:amd64 (0.5.4-5build2) ...
2026-Feb-25 07:13:24.955411
#37 173.6 Setting up cpp-11 (11.4.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:24.955411
#37 173.6 Setting up libzvbi0:amd64 (0.2.35-19) ...
2026-Feb-25 07:13:24.955411
#37 173.6 Setting up libz3-dev:amd64 (4.8.12-1) ...
2026-Feb-25 07:13:24.955411
#37 173.6 Setting up libxdmcp6:amd64 (1:1.1.3-0ubuntu5) ...
2026-Feb-25 07:13:24.955411
#37 173.6 Setting up liblapack3:amd64 (3.10.0-2ubuntu1) ...
2026-Feb-25 07:13:24.955411
#37 173.6 update-alternatives: using /usr/lib/x86_64-linux-gnu/lapack/liblapack.so.3 to provide /usr/lib/x86_64-linux-gnu/liblapack.so.3 (liblapack.so.3-x86_64-linux-gnu) in auto mode
2026-Feb-25 07:13:25.055514
#37 173.6 Setting up libxcb1:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:13:25.055514
#37 173.6 Setting up libxcb-xfixes0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:13:25.055514
#37 173.6 Setting up libzmq5:amd64 (4.3.4-2) ...
2026-Feb-25 07:13:25.055514
#37 173.6 Setting up libcaca0:amd64 (0.99.beta19-2.2ubuntu4.1) ...
2026-Feb-25 07:13:25.055514
#37 173.6 Setting up protobuf-compiler (3.12.4-1ubuntu7.22.04.4) ...
2026-Feb-25 07:13:25.055514
#37 173.6 Setting up alsa-ucm-conf (1.2.6.3-1ubuntu1.12) ...
2026-Feb-25 07:13:25.055514
#37 173.6 Setting up libxcb-render0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:13:25.055514
#37 173.6 Setting up libsoxr0:amd64 (0.1.3-4build2) ...
2026-Feb-25 07:13:25.055514
#37 173.7 Setting up libcdio-cdda2:amd64 (10.2+2.0.0-1build3) ...
2026-Feb-25 07:13:25.055514
#37 173.7 Setting up fontconfig-config (2.13.1-4.2ubuntu5) ...
2026-Feb-25 07:13:25.055514
#37 173.7 Setting up libcdio-paranoia2:amd64 (10.2+2.0.0-1build3) ...
2026-Feb-25 07:13:25.055514
#37 173.7 Setting up libxcb-glx0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:13:25.055514
#37 173.7 Setting up libedit2:amd64 (3.1-20210910-1build1) ...
2026-Feb-25 07:13:25.055514
#37 173.7 Setting up libxcb-shape0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:13:25.055514
#37 173.7 Setting up libobjc4:amd64 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:25.157637
#37 173.7 Setting up libxcb-shm0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:13:25.157637
#37 173.7 Setting up libxcb-present0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:13:25.157637
#37 173.7 Setting up libpython3.10-stdlib:amd64 (3.10.12-1~22.04.14) ...
2026-Feb-25 07:13:25.157637
#37 173.7 Setting up libthai0:amd64 (0.1.29-1build1) ...
2026-Feb-25 07:13:25.157637
#37 173.7 Setting up libvorbisfile3:amd64 (1.3.7-1build2) ...
2026-Feb-25 07:13:25.157637
#37 173.7 Setting up libfreetype6:amd64 (2.11.1+dfsg-1ubuntu0.3) ...
2026-Feb-25 07:13:25.157637
#37 173.7 Setting up lib32gcc-s1 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:25.157637
#37 173.7 Setting up lib32stdc++6 (12.3.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up libxcb-sync1:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up libdc1394-25:amd64 (2.2.6-4) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up icu-devtools (70.1-2) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up libx265-199:amd64 (3.5-2) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up librubberband2:amd64 (2.0.0-2) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up libsndio7.0:amd64 (1.8.1-1.1) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up libxcb-dri2-0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up libjack-jackd2-0:amd64 (1.9.20~dfsg-1) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up libgcc-11-dev:amd64 (11.4.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up libdrm2:amd64 (2.4.113-2~ubuntu0.22.04.1) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up libflite1:amd64 (2.2-3) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up gcc-11 (11.4.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up libva-drm2:amd64 (2.14.0-1) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up libsord-0-0:amd64 (0.16.8-2) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up libwayland-cursor0:amd64 (1.20.0-1ubuntu0.1) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up libxcb-randr0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up libsratom-0-0:amd64 (0.6.8-1) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up libdecor-0-0:amd64 (0.1.0-3build1) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up libc6-dev:amd64 (2.35-0ubuntu3.13) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up libx11-6:amd64 (2:1.7.5-1ubuntu0.3) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up libharfbuzz0b:amd64 (2.7.4-1ubuntu3.2) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up libtiff5:amd64 (4.3.0-6ubuntu0.12) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up libfontconfig1:amd64 (2.13.1-4.2ubuntu5) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up libsndfile1:amd64 (1.0.31-2ubuntu0.2) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up liblilv-0-0:amd64 (0.24.12-2) ...
2026-Feb-25 07:13:25.157637
#37 173.8 Setting up libicu-dev:amd64 (70.1-2) ...
2026-Feb-25 07:13:25.334239
#37 173.8 Setting up libxml2:amd64 (2.9.13+dfsg-1ubuntu0.11) ...
2026-Feb-25 07:13:25.334239
#37 173.8 Setting up libopenmpt0:amd64 (0.6.1-1) ...
2026-Feb-25 07:13:25.334239
#37 173.8 Setting up libpython3-stdlib:amd64 (3.10.6-1~22.04.1) ...
2026-Feb-25 07:13:25.334239
#37 173.8 Setting up libdrm-amdgpu1:amd64 (2.4.113-2~ubuntu0.22.04.1) ...
2026-Feb-25 07:13:25.334239
#37 173.8 Setting up libxcb-dri3-0:amd64 (1.14-3ubuntu3) ...
2026-Feb-25 07:13:25.334239
#37 173.8 Setting up libx11-xcb1:amd64 (2:1.7.5-1ubuntu0.3) ...
2026-Feb-25 07:13:25.334239
#37 173.8 Setting up fontconfig (2.13.1-4.2ubuntu5) ...
2026-Feb-25 07:13:25.334239
#37 173.9 Regenerating fonts cache...
2026-Feb-25 07:13:27.195688
done.
2026-Feb-25 07:13:27.353639
#37 175.9 Setting up libncurses-dev:amd64 (6.3-2ubuntu0.1) ...
2026-Feb-25 07:13:27.353639
#37 175.9 Setting up libdrm-nouveau2:amd64 (2.4.113-2~ubuntu0.22.04.1) ...
2026-Feb-25 07:13:27.353639
#37 175.9 Setting up python3.10 (3.10.12-1~22.04.14) ...
2026-Feb-25 07:13:27.737485
#37 176.4 Setting up libxrender1:amd64 (1:0.9.10-1build4) ...
2026-Feb-25 07:13:27.843185
#37 176.4 Setting up libgbm1:amd64 (23.2.1-1ubuntu3.1~22.04.3) ...
2026-Feb-25 07:13:27.843185
#37 176.4 Setting up libpulse0:amd64 (1:15.99.1+dfsg1-1ubuntu2.2) ...
2026-Feb-25 07:13:27.843185
#37 176.4 Setting up libdrm-radeon1:amd64 (2.4.113-2~ubuntu0.22.04.1) ...
2026-Feb-25 07:13:27.843185
#37 176.4 Setting up libpango-1.0-0:amd64 (1.50.6+ds-2ubuntu1) ...
2026-Feb-25 07:13:27.843185
#37 176.4 Setting up libdrm-intel1:amd64 (2.4.113-2~ubuntu0.22.04.1) ...
2026-Feb-25 07:13:27.843185
#37 176.4 Setting up libxext6:amd64 (2:1.3.4-1build1) ...
2026-Feb-25 07:13:27.843185
#37 176.4 Setting up libobjc-11-dev:amd64 (11.4.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:27.843185
#37 176.4 Setting up python3 (3.10.6-1~22.04.1) ...
2026-Feb-25 07:13:27.843185
#37 176.4 running python rtupdate hooks for python3.10...
2026-Feb-25 07:13:27.843185
#37 176.4 running python post-rtupdate hooks for python3.10...
2026-Feb-25 07:13:27.843185
#37 176.5 Setting up libopenal1:amd64 (1:1.19.1-2build3) ...
2026-Feb-25 07:13:28.011973
#37 176.5 Setting up libcairo2:amd64 (1.16.0-5ubuntu2) ...
2026-Feb-25 07:13:28.011973
#37 176.5 Setting up libxxf86vm1:amd64 (1:1.1.4-1build3) ...
2026-Feb-25 07:13:28.011973
#37 176.5 Setting up libxml2-dev:amd64 (2.9.13+dfsg-1ubuntu0.11) ...
2026-Feb-25 07:13:28.011973
#37 176.5 Setting up libass9:amd64 (1:0.15.2-1) ...
2026-Feb-25 07:13:28.011973
#37 176.5 Setting up libxfixes3:amd64 (1:6.0.0-1) ...
2026-Feb-25 07:13:28.011973
#37 176.5 Setting up shared-mime-info (2.1-2) ...
2026-Feb-25 07:13:28.490710
#37 177.2 Setting up libxinerama1:amd64 (2:1.1.4-3) ...
2026-Feb-25 07:13:28.709968
#37 177.2 Setting up libxv1:amd64 (2:1.0.11-1build2) ...
2026-Feb-25 07:13:28.709968
#37 177.2 Setting up libxrandr2:amd64 (2:1.5.2-1build1) ...
2026-Feb-25 07:13:28.709968
#37 177.2 Setting up libstdc++-11-dev:amd64 (11.4.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:28.709968
#37 177.2 Setting up zlib1g-dev:amd64 (1:1.2.11.dfsg-2ubuntu9.2) ...
2026-Feb-25 07:13:28.709968
#37 177.2 Setting up libllvm14:amd64 (1:14.0.0-1ubuntu1.1) ...
2026-Feb-25 07:13:28.709968
#37 177.2 Setting up libvdpau1:amd64 (1.4-3build2) ...
2026-Feb-25 07:13:28.709968
#37 177.2 Setting up libllvm15:amd64 (1:15.0.7-0ubuntu0.22.04.3) ...
2026-Feb-25 07:13:28.709968
#37 177.2 Setting up libtheora0:amd64 (1.1.1+dfsg.1-15ubuntu4) ...
2026-Feb-25 07:13:28.709968
#37 177.2 Setting up libgdk-pixbuf-2.0-0:amd64 (2.42.8+dfsg-1ubuntu0.4) ...
2026-Feb-25 07:13:28.709968
#37 177.2 Setting up libcairo-gobject2:amd64 (1.16.0-5ubuntu2) ...
2026-Feb-25 07:13:28.709968
#37 177.2 Setting up libxss1:amd64 (1:1.2.3-1build2) ...
2026-Feb-25 07:13:28.709968
#37 177.2 Setting up mesa-va-drivers:amd64 (23.2.1-1ubuntu3.1~22.04.3) ...
2026-Feb-25 07:13:28.709968
#37 177.2 Setting up libpangoft2-1.0-0:amd64 (1.50.6+ds-2ubuntu1) ...
2026-Feb-25 07:13:28.709968
#37 177.2 Setting up libbluray2:amd64 (1:1.3.1-1) ...
2026-Feb-25 07:13:28.709968
#37 177.2 Setting up llvm-14-linker-tools (1:14.0.0-1ubuntu1.1) ...
2026-Feb-25 07:13:28.709968
#37 177.2 Setting up libva-x11-2:amd64 (2.14.0-1) ...
2026-Feb-25 07:13:28.709968
#37 177.2 Setting up i965-va-driver:amd64 (2.4.1+dfsg1-1) ...
2026-Feb-25 07:13:28.709968
#37 177.2 Setting up libpangocairo-1.0-0:amd64 (1.50.6+ds-2ubuntu1) ...
2026-Feb-25 07:13:28.709968
#37 177.2 Setting up python3-pkg-resources (59.6.0-1.2ubuntu0.22.04.3) ...
2026-Feb-25 07:13:28.709968
#37 177.4 Setting up libgl1-amber-dri:amd64 (21.3.9-0ubuntu1~22.04.1) ...
2026-Feb-25 07:13:28.857039
#37 177.4 Setting up libtinfo-dev:amd64 (6.3-2ubuntu0.1) ...
2026-Feb-25 07:13:28.857039
#37 177.4 Setting up mesa-vdpau-drivers:amd64 (23.2.1-1ubuntu3.1~22.04.3) ...
2026-Feb-25 07:13:28.857039
#37 177.4 Setting up libxi6:amd64 (2:1.8-1build1) ...
2026-Feb-25 07:13:28.857039
#37 177.4 Setting up libsphinxbase3:amd64 (0.8+5prealpha+1-13build1) ...
2026-Feb-25 07:13:28.857039
#37 177.4 Setting up g++-11 (11.4.0-1ubuntu1~22.04.3) ...
2026-Feb-25 07:13:28.857039
#37 177.4 Setting up libxcursor1:amd64 (1:1.2.0-2build4) ...
2026-Feb-25 07:13:28.857039
#37 177.4 Setting up libgl1-mesa-dri:amd64 (23.2.1-1ubuntu3.1~22.04.3) ...
2026-Feb-25 07:13:28.857039
#37 177.4 Setting up libclang1-14 (1:14.0.0-1ubuntu1.1) ...
2026-Feb-25 07:13:28.857039
#37 177.4 Setting up python3-yaml (5.4.1-1ubuntu1) ...
2026-Feb-25 07:13:28.857039
#37 177.5 Setting up libavutil56:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:29.019269
#37 177.5 Setting up libprotobuf-dev:amd64 (3.12.4-1ubuntu7.22.04.4) ...
2026-Feb-25 07:13:29.019269
#37 177.5 Setting up librsvg2-2:amd64 (2.52.5+dfsg-3ubuntu0.2) ...
2026-Feb-25 07:13:29.019269
#37 177.5 Setting up libpocketsphinx3:amd64 (0.8.0+real5prealpha+1-14ubuntu1) ...
2026-Feb-25 07:13:29.019269
#37 177.5 Setting up python3-pygments (2.11.2+dfsg-2ubuntu0.1) ...
2026-Feb-25 07:13:29.268897
#37 177.9 Setting up llvm-14-runtime (1:14.0.0-1ubuntu1.1) ...
2026-Feb-25 07:13:29.369513
#37 177.9 Setting up va-driver-all:amd64 (2.14.0-1) ...
2026-Feb-25 07:13:29.369513
#37 177.9 Setting up libclang-common-14-dev (1:14.0.0-1ubuntu1.1) ...
2026-Feb-25 07:13:29.369513
#37 177.9 Setting up libdecor-0-plugin-1-cairo:amd64 (0.1.0-3build1) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up libpostproc55:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up libclang-cpp14 (1:14.0.0-1ubuntu1.1) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up librsvg2-common:amd64 (2.52.5+dfsg-3ubuntu0.2) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up vdpau-driver-all:amd64 (1.4-3build2) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up libgdk-pixbuf2.0-bin (2.42.8+dfsg-1ubuntu0.4) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up libclang-14-dev (1:14.0.0-1ubuntu1.1) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up libswscale5:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up libsdl2-2.0-0:amd64 (2.0.20+dfsg-2ubuntu1.22.04.1) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up llvm-14 (1:14.0.0-1ubuntu1.1) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up llvm-14-tools (1:14.0.0-1ubuntu1.1) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up libavutil-dev:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up libglx-mesa0:amd64 (23.2.1-1ubuntu3.1~22.04.3) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up clang-14 (1:14.0.0-1ubuntu1.1) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up libclang-dev (1:14.0-55~exp2) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up libglx0:amd64 (1.4.0-1) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up libpostproc-dev:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up libswresample3:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up libgl1:amd64 (1.4.0-1) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up clang (1:14.0-55~exp2) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up libswscale-dev:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up libavcodec58:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up llvm-14-dev (1:14.0.0-1ubuntu1.1) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up libchromaprint1:amd64 (1.5.1-2) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up libswresample-dev:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up libavformat58:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up libavcodec-dev:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:29.369513
#37 178.0 Setting up libavformat-dev:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:29.571393
#37 178.0 Setting up libavfilter7:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:29.571393
#37 178.0 Setting up libavfilter-dev:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:29.571393
#37 178.0 Setting up libavdevice58:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:29.571393
#37 178.1 Setting up ffmpeg (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:29.571393
#37 178.1 Setting up libavdevice-dev:amd64 (7:4.4.2-0ubuntu0.22.04.1) ...
2026-Feb-25 07:13:29.571393
#37 178.1 Processing triggers for libc-bin (2.35-0ubuntu3.6) ...
2026-Feb-25 07:13:29.571393
#37 178.1 Processing triggers for libgdk-pixbuf-2.0-0:amd64 (2.42.8+dfsg-1ubuntu0.4) ...
2026-Feb-25 07:13:29.921923
#37 DONE 178.6s
2026-Feb-25 07:13:30.073185
#55 [gpu-worker builder 4/9] RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
2026-Feb-25 07:13:30.357671
#55 0.435 info: downloading installer
2026-Feb-25 07:17:04.335019
#55 214.4 info: profile set to 'default'
2026-Feb-25 07:17:04.335019
#55 214.4 info: default host triple is x86_64-unknown-linux-gnu
2026-Feb-25 07:17:04.485198
#55 214.4 info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'
2026-Feb-25 07:17:06.477596
#55 216.6 info: latest update on 2026-02-12, rust version 1.93.1 (01f6ddf75 2026-02-11)
2026-Feb-25 07:17:06.477596
#55 216.6 info: downloading component 'cargo'
2026-Feb-25 07:17:10.811844
#55 220.9 info: downloading component 'clippy'
2026-Feb-25 07:17:13.140668
#55 223.2 info: downloading component 'rust-docs'
2026-Feb-25 07:17:17.547700
#55 227.6 info: downloading component 'rust-std'
2026-Feb-25 07:17:23.038005
#55 233.1 info: downloading component 'rustc'
2026-Feb-25 07:32:56.336315
#55 1166.4 info: downloading component 'rustfmt'
2026-Feb-25 07:32:58.489003
#55 1168.6 info: installing component 'cargo'
2026-Feb-25 07:32:59.194559
#55 1169.3 info: installing component 'clippy'
2026-Feb-25 07:32:59.629284
#55 1169.7 info: installing component 'rust-docs'
2026-Feb-25 07:33:01.497525
#55 1171.6 info: installing component 'rust-std'
2026-Feb-25 07:33:03.392333
#55 1173.5 info: installing component 'rustc'
2026-Feb-25 07:33:07.930955
#55 1178.0 info: installing component 'rustfmt'
2026-Feb-25 07:33:08.225180
#55 1178.3 info: default toolchain set to 'stable-x86_64-unknown-linux-gnu'
2026-Feb-25 07:33:08.225180
#55 1178.3
2026-Feb-25 07:33:08.384640
#55 1178.3   stable-x86_64-unknown-linux-gnu installed - rustc 1.93.1 (01f6ddf75 2026-02-11)
2026-Feb-25 07:33:08.384640
#55 1178.3
2026-Feb-25 07:33:08.384640
#55 1178.3
2026-Feb-25 07:33:08.384640
#55 1178.3 Rust is installed now. Great!
2026-Feb-25 07:33:08.384640
#55 1178.3
2026-Feb-25 07:33:08.384640
#55 1178.3 To get started you may need to restart your current shell.
2026-Feb-25 07:33:08.384640
#55 1178.3 This would reload your PATH environment variable to include
2026-Feb-25 07:33:08.384640
#55 1178.3 Cargo's bin directory ($HOME/.cargo/bin).
2026-Feb-25 07:33:08.384640
#55 1178.3
2026-Feb-25 07:33:08.384640
#55 1178.3 To configure your current shell, you need to source
2026-Feb-25 07:33:08.384640
#55 1178.3 the corresponding env file under $HOME/.cargo.
2026-Feb-25 07:33:08.384640
#55 1178.3
2026-Feb-25 07:33:08.384640
#55 1178.3 This is usually done by running one of the following (note the leading DOT):
2026-Feb-25 07:33:08.384640
#55 1178.3 . "$HOME/.cargo/env"            # For sh/bash/zsh/ash/dash/pdksh
2026-Feb-25 07:33:08.384640
#55 1178.3 source "$HOME/.cargo/env.fish"  # For fish
2026-Feb-25 07:33:08.384640
#55 1178.3 source $"($nu.home-path)/.cargo/env.nu"  # For nushell
2026-Feb-25 07:33:08.442817
#55 DONE 1178.5s
2026-Feb-25 07:33:08.594251
#56 [gpu-worker builder 5/9] COPY Cargo.toml ./
2026-Feb-25 07:33:09.081894
#56 DONE 0.6s
2026-Feb-25 07:33:09.292738
#57 [gpu-worker builder 6/9] COPY Cargo.lock ./
2026-Feb-25 07:33:09.292738
#57 DONE 0.0s
2026-Feb-25 07:33:09.292738
2026-Feb-25 07:33:09.292738
#58 [gpu-worker builder 7/9] COPY crates/ ./crates/
2026-Feb-25 07:33:09.292738
#58 DONE 0.0s
2026-Feb-25 07:33:09.292738
2026-Feb-25 07:33:09.292738
#59 [gpu-worker builder 8/9] COPY proto/ ./proto/
2026-Feb-25 07:33:09.292738
#59 DONE 0.0s
2026-Feb-25 07:33:09.292738
2026-Feb-25 07:33:09.292738
#60 [gpu-worker builder 9/9] RUN cargo build --release --bin gpu-node
2026-Feb-25 07:33:09.380965
#60 0.238     Updating crates.io index
2026-Feb-25 07:33:17.734052
#60 8.592  Downloading crates ...
2026-Feb-25 07:33:18.328651
#60 9.186   Downloaded adler v1.0.2
2026-Feb-25 07:33:18.527462
#60 9.385   Downloaded alloc-no-stdlib v2.0.4
2026-Feb-25 07:33:18.769133
#60 9.627   Downloaded base64-simd v0.7.0
2026-Feb-25 07:33:18.902972
#60 9.647   Downloaded heck v0.4.1
2026-Feb-25 07:33:18.902972
#60 9.761   Downloaded funty v2.0.0
2026-Feb-25 07:33:19.014705
#60 9.768   Downloaded compression-core v0.4.31
2026-Feb-25 07:33:19.014705
#60 9.783   Downloaded async-stream v0.3.6
2026-Feb-25 07:33:19.014705
#60 9.872   Downloaded heck v0.5.0
2026-Feb-25 07:33:19.137105
#60 9.896   Downloaded futures-macro v0.3.32
2026-Feb-25 07:33:19.137105
#60 9.918   Downloaded futures-sink v0.3.32
2026-Feb-25 07:33:19.137105
#60 9.923   Downloaded fastrand v2.3.0
2026-Feb-25 07:33:19.137105
#60 9.940   Downloaded bit-set v0.5.3
2026-Feb-25 07:33:19.137105
#60 9.943   Downloaded async-stream-impl v0.3.6
2026-Feb-25 07:33:19.137105
#60 9.995   Downloaded foreign-types-shared v0.1.1
2026-Feb-25 07:33:19.239309
#60 9.997   Downloaded block-buffer v0.10.4
2026-Feb-25 07:33:19.239309
#60 10.08   Downloaded futures-io v0.3.32
2026-Feb-25 07:33:19.239309
#60 10.09   Downloaded dirs-sys v0.4.1
2026-Feb-25 07:33:19.239309
#60 10.10   Downloaded futures-task v0.3.32
2026-Feb-25 07:33:19.360932
#60 10.10   Downloaded dirs v5.0.1
2026-Feb-25 07:33:19.360932
#60 10.13   Downloaded gzip-header v1.0.0
2026-Feb-25 07:33:19.360932
#60 10.17   Downloaded cpufeatures v0.2.17
2026-Feb-25 07:33:19.360932
#60 10.17   Downloaded hyper-tls v0.6.0
2026-Feb-25 07:33:19.360932
#60 10.22   Downloaded num-conv v0.2.0
2026-Feb-25 07:33:19.471369
#60 10.23   Downloaded lazycell v1.3.0
2026-Feb-25 07:33:19.471369
#60 10.26   Downloaded mime v0.3.17
2026-Feb-25 07:33:19.471369
#60 10.26   Downloaded if_chain v1.0.3
2026-Feb-25 07:33:19.471369
#60 10.27   Downloaded cfg-if v1.0.4
2026-Feb-25 07:33:19.471369
#60 10.28   Downloaded foreign-types v0.3.2
2026-Feb-25 07:33:19.471369
#60 10.28   Downloaded errno v0.3.14
2026-Feb-25 07:33:19.471369
#60 10.28   Downloaded debugid v0.8.0
2026-Feb-25 07:33:19.471369
#60 10.29   Downloaded generic-array v0.14.7
2026-Feb-25 07:33:19.471369
#60 10.29   Downloaded document-features v0.2.12
2026-Feb-25 07:33:19.471369
#60 10.29   Downloaded cfg_aliases v0.2.1
2026-Feb-25 07:33:19.471369
#60 10.30   Downloaded fnv v1.0.7
2026-Feb-25 07:33:19.471369
#60 10.32   Downloaded crypto-common v0.1.7
2026-Feb-25 07:33:19.471369
#60 10.33   Downloaded crunchy v0.2.4
2026-Feb-25 07:33:19.593358
#60 10.36   Downloaded fslock v0.2.1
2026-Feb-25 07:33:19.593358
#60 10.37   Downloaded atomic-waker v1.1.2
2026-Feb-25 07:33:19.593358
#60 10.39   Downloaded matchers v0.2.0
2026-Feb-25 07:33:19.593358
#60 10.40   Downloaded powerfmt v0.2.0
2026-Feb-25 07:33:19.593358
#60 10.42   Downloaded pathdiff v0.2.3
2026-Feb-25 07:33:19.593358
#60 10.45   Downloaded form_urlencoded v1.2.2
2026-Feb-25 07:33:19.707108
#60 10.45   Downloaded const-random v0.1.18
2026-Feb-25 07:33:19.707108
#60 10.46   Downloaded futures-core v0.3.32
2026-Feb-25 07:33:19.707108
#60 10.46   Downloaded const-random-macro v0.1.16
2026-Feb-25 07:33:19.707108
#60 10.46   Downloaded cooked-waker v5.0.0
2026-Feb-25 07:33:19.707108
#60 10.46   Downloaded alloc-stdlib v0.2.2
2026-Feb-25 07:33:19.707108
#60 10.46   Downloaded ahash v0.8.12
2026-Feb-25 07:33:19.707108
#60 10.47   Downloaded adler2 v2.0.1
2026-Feb-25 07:33:19.707108
#60 10.48   Downloaded psl-types v2.0.11
2026-Feb-25 07:33:19.707108
#60 10.49   Downloaded autocfg v1.5.0
2026-Feb-25 07:33:19.707108
#60 10.50   Downloaded either v1.15.0
2026-Feb-25 07:33:19.707108
#60 10.50   Downloaded digest v0.10.7
2026-Feb-25 07:33:19.707108
#60 10.51   Downloaded axum-core v0.4.5
2026-Feb-25 07:33:19.707108
#60 10.51   Downloaded dashmap v6.1.0
2026-Feb-25 07:33:19.707108
#60 10.51   Downloaded async-trait v0.1.89
2026-Feb-25 07:33:19.707108
#60 10.53   Downloaded bincode v1.3.3
2026-Feb-25 07:33:19.707108
#60 10.56   Downloaded inotify-sys v0.1.5
2026-Feb-25 07:33:19.807886
#60 10.58   Downloaded memoffset v0.9.1
2026-Feb-25 07:33:19.807886
#60 10.58   Downloaded itoa v1.0.17
2026-Feb-25 07:33:19.807886
#60 10.59   Downloaded num_cpus v1.17.0
2026-Feb-25 07:33:19.807886
#60 10.59   Downloaded potential_utf v0.1.4
2026-Feb-25 07:33:19.807886
#60 10.62   Downloaded proc-macro-rules v0.4.0
2026-Feb-25 07:33:19.807886
#60 10.63   Downloaded compression-codecs v0.4.37
2026-Feb-25 07:33:19.807886
#60 10.64   Downloaded glob v0.3.3
2026-Feb-25 07:33:19.807886
#60 10.64   Downloaded filetime v0.2.27
2026-Feb-25 07:33:19.807886
#60 10.65   Downloaded data-encoding v2.10.0
2026-Feb-25 07:33:19.807886
#60 10.65   Downloaded cexpr v0.6.0
2026-Feb-25 07:33:19.807886
#60 10.65   Downloaded futures-executor v0.3.32
2026-Feb-25 07:33:19.807886
#60 10.66   Downloaded dlv-list v0.5.2
2026-Feb-25 07:33:19.807886
#60 10.66   Downloaded displaydoc v0.2.5
2026-Feb-25 07:33:19.807886
#60 10.67   Downloaded arraydeque v0.5.1
2026-Feb-25 07:33:19.920939
#60 10.67   Downloaded convert_case v0.6.0
2026-Feb-25 07:33:19.920939
#60 10.67   Downloaded bit-vec v0.6.3
2026-Feb-25 07:33:19.920939
#60 10.67   Downloaded bitflags v1.3.2
2026-Feb-25 07:33:19.920939
#60 10.68   Downloaded prost-derive v0.13.5
2026-Feb-25 07:33:19.920939
#60 10.68   Downloaded prost v0.13.5
2026-Feb-25 07:33:19.920939
#60 10.68   Downloaded radium v0.7.0
2026-Feb-25 07:33:19.920939
#60 10.69   Downloaded fixedbitset v0.5.7
2026-Feb-25 07:33:19.920939
#60 10.69   Downloaded rand_chacha v0.3.1
2026-Feb-25 07:33:19.920939
#60 10.69   Downloaded deranged v0.5.8
2026-Feb-25 07:33:19.920939
#60 10.69   Downloaded deno_unsync v0.4.4
2026-Feb-25 07:33:19.920939
#60 10.76   Downloaded pin-utils v0.1.0
2026-Feb-25 07:33:19.920939
#60 10.78   Downloaded json5 v0.4.1
2026-Feb-25 07:33:20.021881
#60 10.78   Downloaded outref v0.1.0
2026-Feb-25 07:33:20.021881
#60 10.80   Downloaded openssl-probe v0.2.1
2026-Feb-25 07:33:20.021881
#60 10.82   Downloaded find-msvc-tools v0.1.9
2026-Feb-25 07:33:20.021881
#60 10.82   Downloaded cookie_store v0.22.1
2026-Feb-25 07:33:20.021881
#60 10.82   Downloaded futures-channel v0.3.32
2026-Feb-25 07:33:20.021881
#60 10.83   Downloaded equivalent v1.0.2
2026-Feb-25 07:33:20.021881
#60 10.83   Downloaded crossbeam-utils v0.8.21
2026-Feb-25 07:33:20.021881
#60 10.83   Downloaded dotenvy v0.15.7
2026-Feb-25 07:33:20.021881
#60 10.84   Downloaded bitflags v2.11.0
2026-Feb-25 07:33:20.021881
#60 10.85   Downloaded crc32fast v1.5.0
2026-Feb-25 07:33:20.021881
#60 10.85   Downloaded rand_core v0.9.5
2026-Feb-25 07:33:20.021881
#60 10.85   Downloaded rustc-hash v2.1.1
2026-Feb-25 07:33:20.021881
#60 10.85   Downloaded rand_chacha v0.9.0
2026-Feb-25 07:33:20.021881
#60 10.86   Downloaded getrandom v0.3.4
2026-Feb-25 07:33:20.021881
#60 10.86   Downloaded getrandom v0.2.17
2026-Feb-25 07:33:20.021881
#60 10.87   Downloaded rustc_version v0.2.3
2026-Feb-25 07:33:20.021881
#60 10.87   Downloaded futures v0.3.32
2026-Feb-25 07:33:20.021881
#60 10.88   Downloaded clang-sys v1.8.1
2026-Feb-25 07:33:20.021881
#60 10.88   Downloaded cookie v0.18.1
2026-Feb-25 07:33:20.133497
#60 10.88   Downloaded axum-core v0.5.6
2026-Feb-25 07:33:20.133497
#60 10.89   Downloaded anyhow v1.0.102
2026-Feb-25 07:33:20.133497
#60 10.89   Downloaded config v0.14.1
2026-Feb-25 07:33:20.133497
#60 10.90   Downloaded rustc-hash v1.1.0
2026-Feb-25 07:33:20.133497
#60 10.90   Downloaded allocator-api2 v0.2.21
2026-Feb-25 07:33:20.133497
#60 10.91   Downloaded rand_core v0.6.4
2026-Feb-25 07:33:20.133497
#60 10.91   Downloaded prost-types v0.13.5
2026-Feb-25 07:33:20.133497
#60 10.91   Downloaded proc-macro2 v1.0.106
2026-Feb-25 07:33:20.133497
#60 10.96   Downloaded multimap v0.10.1
2026-Feb-25 07:33:20.133497
#60 10.96   Downloaded openssl-macros v0.1.1
2026-Feb-25 07:33:20.133497
#60 10.97   Downloaded percent-encoding v2.3.2
2026-Feb-25 07:33:20.133497
#60 10.97   Downloaded lru-slab v0.1.2
2026-Feb-25 07:33:20.133497
#60 10.99   Downloaded http-body v1.0.1
2026-Feb-25 07:33:20.236333
#60 10.99   Downloaded home v0.5.12
2026-Feb-25 07:33:20.236333
#60 11.00   Downloaded quote v1.0.44
2026-Feb-25 07:33:20.236333
#60 11.00   Downloaded quinn-udp v0.5.14
2026-Feb-25 07:33:20.236333
#60 11.01   Downloaded getrandom v0.4.1
2026-Feb-25 07:33:20.236333
#60 11.01   Downloaded lazy_static v1.5.0
2026-Feb-25 07:33:20.236333
#60 11.02   Downloaded scopeguard v1.2.0
2026-Feb-25 07:33:20.236333
#60 11.02   Downloaded bytes v1.11.1
2026-Feb-25 07:33:20.236333
#60 11.03   Downloaded same-file v1.0.6
2026-Feb-25 07:33:20.236333
#60 11.03   Downloaded lock_api v0.4.14
2026-Feb-25 07:33:20.236333
#60 11.03   Downloaded flate2 v1.1.9
2026-Feb-25 07:33:20.236333
#60 11.04   Downloaded rustversion v1.0.22
2026-Feb-25 07:33:20.236333
#60 11.04   Downloaded rustls-pki-types v1.14.0
2026-Feb-25 07:33:20.236333
#60 11.05   Downloaded serde_urlencoded v0.7.1
2026-Feb-25 07:33:20.236333
#60 11.05   Downloaded quinn v0.11.9
2026-Feb-25 07:33:20.236333
#60 11.05   Downloaded deno_ops v0.176.0
2026-Feb-25 07:33:20.236333
#60 11.07   Downloaded cc v1.2.56
2026-Feb-25 07:33:20.236333
#60 11.07   Downloaded async-compression v0.4.40
2026-Feb-25 07:33:20.236333
#60 11.08   Downloaded crossbeam-channel v0.5.15
2026-Feb-25 07:33:20.236333
#60 11.09   Downloaded base64 v0.21.7
2026-Feb-25 07:33:20.236333
#60 11.09   Downloaded http-body-util v0.1.3
2026-Feb-25 07:33:20.336469
#60 11.10   Downloaded base64 v0.22.1
2026-Feb-25 07:33:20.336469
#60 11.15   Downloaded pin-project-internal v1.1.10
2026-Feb-25 07:33:20.336469
#60 11.15   Downloaded libloading v0.8.9
2026-Feb-25 07:33:20.336469
#60 11.15   Downloaded httpdate v1.0.3
2026-Feb-25 07:33:20.336469
#60 11.16   Downloaded option-ext v0.2.0
2026-Feb-25 07:33:20.336469
#60 11.16   Downloaded proc-macro-error v1.0.4
2026-Feb-25 07:33:20.336469
#60 11.17   Downloaded proc-macro-rules-macros v0.4.0
2026-Feb-25 07:33:20.336469
#60 11.17   Downloaded inotify v0.9.6
2026-Feb-25 07:33:20.336469
#60 11.17   Downloaded pin-project-lite v0.2.16
2026-Feb-25 07:33:20.336469
#60 11.18   Downloaded sha2 v0.10.9
2026-Feb-25 07:33:20.336469
#60 11.19   Downloaded ordered-multimap v0.7.3
2026-Feb-25 07:33:20.336469
#60 11.19   Downloaded ppv-lite86 v0.2.21
2026-Feb-25 07:33:20.336469
#60 11.19   Downloaded ryu v1.0.23
2026-Feb-25 07:33:20.438683
#60 11.20   Downloaded subtle v2.6.1
2026-Feb-25 07:33:20.438683
#60 11.20   Downloaded slab v0.4.12
2026-Feb-25 07:33:20.438683
#60 11.21   Downloaded sync_wrapper v1.0.2
2026-Feb-25 07:33:20.438683
#60 11.21   Downloaded tap v1.0.1
2026-Feb-25 07:33:20.438683
#60 11.21   Downloaded serde_derive v1.0.228
2026-Feb-25 07:33:20.438683
#60 11.22   Downloaded synstructure v0.13.2
2026-Feb-25 07:33:20.438683
#60 11.22   Downloaded thiserror v2.0.18
2026-Feb-25 07:33:20.438683
#60 11.23   Downloaded time-macros v0.2.27
2026-Feb-25 07:33:20.438683
#60 11.23   Downloaded thiserror-impl v2.0.18
2026-Feb-25 07:33:20.438683
#60 11.24   Downloaded brotli-decompressor v5.0.0
2026-Feb-25 07:33:20.438683
#60 11.24   Downloaded thiserror-impl v1.0.69
2026-Feb-25 07:33:20.438683
#60 11.24   Downloaded axum v0.8.8
2026-Feb-25 07:33:20.438683
#60 11.26   Downloaded tinyvec_macros v0.1.1
2026-Feb-25 07:33:20.438683
#60 11.26   Downloaded tinystr v0.8.2
2026-Feb-25 07:33:20.438683
#60 11.26   Downloaded tiny-keccak v2.0.2
2026-Feb-25 07:33:20.438683
#60 11.27   Downloaded h2 v0.4.13
2026-Feb-25 07:33:20.438683
#60 11.28   Downloaded futures-util v0.3.32
2026-Feb-25 07:33:20.438683
#60 11.30   Downloaded socket2 v0.5.10
2026-Feb-25 07:33:20.542020
#60 11.30   Downloaded aho-corasick v1.1.4
2026-Feb-25 07:33:20.542020
#60 11.31   Downloaded time-core v0.1.8
2026-Feb-25 07:33:20.542020
#60 11.31   Downloaded thread_local v1.1.9
2026-Feb-25 07:33:20.542020
#60 11.31   Downloaded tempfile v3.25.0
2026-Feb-25 07:33:20.542020
#60 11.31   Downloaded pkg-config v0.3.32
2026-Feb-25 07:33:20.542020
#60 11.32   Downloaded hashbrown v0.14.5
2026-Feb-25 07:33:20.542020
#60 11.32   Downloaded thiserror v1.0.69
2026-Feb-25 07:33:20.542020
#60 11.33   Downloaded axum v0.7.9
2026-Feb-25 07:33:20.542020
#60 11.34   Downloaded smallvec v1.15.1
2026-Feb-25 07:33:20.542020
#60 11.34   Downloaded sharded-slab v0.1.7
2026-Feb-25 07:33:20.542020
#60 11.34   Downloaded ipnet v2.11.0
2026-Feb-25 07:33:20.542020
#60 11.35   Downloaded proc-macro-error-attr v1.0.4
2026-Feb-25 07:33:20.542020
#60 11.35   Downloaded rand v0.9.2
2026-Feb-25 07:33:20.542020
#60 11.35   Downloaded semver-parser v0.7.0
2026-Feb-25 07:33:20.542020
#60 11.35   Downloaded hashlink v0.8.4
2026-Feb-25 07:33:20.542020
#60 11.36   Downloaded simd-adler32 v0.3.8
2026-Feb-25 07:33:20.542020
#60 11.36   Downloaded semver v0.9.0
2026-Feb-25 07:33:20.542020
#60 11.36   Downloaded stable_deref_trait v1.2.1
2026-Feb-25 07:33:20.542020
#60 11.36   Downloaded rust-ini v0.20.0
2026-Feb-25 07:33:20.542020
#60 11.37   Downloaded hyper-rustls v0.27.7
2026-Feb-25 07:33:20.542020
#60 11.37   Downloaded pest_derive v2.8.6
2026-Feb-25 07:33:20.542020
#60 11.38   Downloaded tinyvec v1.10.0
2026-Feb-25 07:33:20.542020
#60 11.38   Downloaded tokio-stream v0.1.18
2026-Feb-25 07:33:20.542020
#60 11.39   Downloaded toml v0.8.23
2026-Feb-25 07:33:20.542020
#60 11.39   Downloaded toml_write v0.1.2
2026-Feb-25 07:33:20.542020
#60 11.39   Downloaded toml_datetime v0.6.11
2026-Feb-25 07:33:20.542020
#60 11.40   Downloaded matchit v0.8.4
2026-Feb-25 07:33:20.542020
#60 11.40   Downloaded httparse v1.10.1
2026-Feb-25 07:33:20.659352
#60 11.40   Downloaded prost-build v0.13.5
2026-Feb-25 07:33:20.659352
#60 11.41   Downloaded parking_lot v0.12.5
2026-Feb-25 07:33:20.659352
#60 11.41   Downloaded deno_core v0.300.0
2026-Feb-25 07:33:20.659352
#60 11.43   Downloaded regex v1.12.3
2026-Feb-25 07:33:20.659352
#60 11.44   Downloaded tower-service v0.3.3
2026-Feb-25 07:33:20.659352
#60 11.44   Downloaded tower-layer v0.3.3
2026-Feb-25 07:33:20.659352
#60 11.44   Downloaded tonic-build v0.12.3
2026-Feb-25 07:33:20.659352
#60 11.44   Downloaded strum v0.25.0
2026-Feb-25 07:33:20.659352
#60 11.44   Downloaded static_assertions v1.1.0
2026-Feb-25 07:33:20.659352
#60 11.44   Downloaded rustls-webpki v0.103.9
2026-Feb-25 07:33:20.659352
#60 11.45   Downloaded quinn-proto v0.11.13
2026-Feb-25 07:33:20.659352
#60 11.46   Downloaded try-lock v0.2.5
2026-Feb-25 07:33:20.659352
#60 11.46   Downloaded pest_generator v2.8.6
2026-Feb-25 07:33:20.659352
#60 11.46   Downloaded native-tls v0.2.18
2026-Feb-25 07:33:20.659352
#60 11.46   Downloaded litrs v1.0.0
2026-Feb-25 07:33:20.659352
#60 11.47   Downloaded parking_lot_core v0.9.12
2026-Feb-25 07:33:20.659352
#60 11.47   Downloaded chrono v0.4.43
2026-Feb-25 07:33:20.659352
#60 11.48   Downloaded bindgen v0.69.5
2026-Feb-25 07:33:20.659352
#60 11.49   Downloaded once_cell v1.21.3
2026-Feb-25 07:33:20.659352
#60 11.49   Downloaded bitvec v1.0.1
2026-Feb-25 07:33:20.659352
#60 11.52   Downloaded litemap v0.8.1
2026-Feb-25 07:33:20.760652
#60 11.52   Downloaded simd-abstraction v0.7.1
2026-Feb-25 07:33:20.760652
#60 11.52   Downloaded signal-hook-registry v1.4.8
2026-Feb-25 07:33:20.760652
#60 11.52   Downloaded shlex v1.3.0
2026-Feb-25 07:33:20.760652
#60 11.53   Downloaded serde_core v1.0.228
2026-Feb-25 07:33:20.760652
#60 11.53   Downloaded matchit v0.7.3
2026-Feb-25 07:33:20.760652
#60 11.53   Downloaded rand v0.8.5
2026-Feb-25 07:33:20.760652
#60 11.54   Downloaded indexmap v1.9.3
2026-Feb-25 07:33:20.760652
#60 11.54   Downloaded strum_macros v0.25.3
2026-Feb-25 07:33:20.760652
#60 11.55   Downloaded iana-time-zone v0.1.65
2026-Feb-25 07:33:20.760652
#60 11.55   Downloaded log v0.4.29
2026-Feb-25 07:33:20.760652
#60 11.56   Downloaded tracing-log v0.2.0
2026-Feb-25 07:33:20.760652
#60 11.56   Downloaded untrusted v0.9.0
2026-Feb-25 07:33:20.760652
#60 11.56   Downloaded unicode-id-start v1.4.0
2026-Feb-25 07:33:20.760652
#60 11.57   Downloaded version_check v0.9.5
2026-Feb-25 07:33:20.760652
#60 11.57   Downloaded unicode-ident v1.0.24
2026-Feb-25 07:33:20.760652
#60 11.57   Downloaded want v0.3.1
2026-Feb-25 07:33:20.760652
#60 11.57   Downloaded rustix v0.38.44
2026-Feb-25 07:33:20.760652
#60 11.62   Downloaded utf8_iter v1.0.4
2026-Feb-25 07:33:20.863540
#60 11.62   Downloaded yoke v0.8.1
2026-Feb-25 07:33:20.863540
#60 11.63   Downloaded socket2 v0.6.2
2026-Feb-25 07:33:20.863540
#60 11.63   Downloaded zerofrom v0.1.6
2026-Feb-25 07:33:20.863540
#60 11.63   Downloaded zeroize v1.8.2
2026-Feb-25 07:33:20.863540
#60 11.63   Downloaded encoding_rs v0.8.35
2026-Feb-25 07:33:20.863540
#60 11.66   Downloaded zerovec-derive v0.11.2
2026-Feb-25 07:33:20.863540
#60 11.66   Downloaded tokio-rustls v0.26.4
2026-Feb-25 07:33:20.863540
#60 11.66   Downloaded sourcemap v8.0.1
2026-Feb-25 07:33:20.863540
#60 11.66   Downloaded zerofrom-derive v0.1.6
2026-Feb-25 07:33:20.863540
#60 11.67   Downloaded yoke-derive v0.8.1
2026-Feb-25 07:33:20.863540
#60 11.67   Downloaded rustix v1.1.3
2026-Feb-25 07:33:20.863540
#60 11.70   Downloaded regex-automata v0.4.14
2026-Feb-25 07:33:20.863540
#60 11.72   Downloaded hashbrown v0.12.3
2026-Feb-25 07:33:20.973499
#60 11.73   Downloaded wyz v0.5.1
2026-Feb-25 07:33:20.973499
#60 11.73   Downloaded which v6.0.3
2026-Feb-25 07:33:20.973499
#60 11.73   Downloaded serde_json v1.0.149
2026-Feb-25 07:33:20.973499
#60 11.74   Downloaded which v4.4.2
2026-Feb-25 07:33:20.973499
#60 11.74   Downloaded icu_collections v2.1.1
2026-Feb-25 07:33:20.973499
#60 11.75   Downloaded serde v1.0.228
2026-Feb-25 07:33:20.973499
#60 11.76   Downloaded num-bigint v0.4.6
2026-Feb-25 07:33:20.973499
#60 11.77   Downloaded zmij v1.0.21
2026-Feb-25 07:33:20.973499
#60 11.77   Downloaded writeable v0.6.2
2026-Feb-25 07:33:20.973499
#60 11.77   Downloaded http v1.4.0
2026-Feb-25 07:33:20.973499
#60 11.78   Downloaded pest v2.8.6
2026-Feb-25 07:33:20.973499
#60 11.78   Downloaded uuid v1.21.0
2026-Feb-25 07:33:20.973499
#60 11.79   Downloaded iri-string v0.7.10
2026-Feb-25 07:33:20.973499
#60 11.80   Downloaded itertools v0.14.0
2026-Feb-25 07:33:20.973499
#60 11.81   Downloaded syn v2.0.117
2026-Feb-25 07:33:20.973499
#60 11.83   Downloaded reqwest v0.12.28
2026-Feb-25 07:33:21.147805
#60 11.84   Downloaded itertools v0.12.1
2026-Feb-25 07:33:21.147805
#60 11.85   Downloaded time v0.3.47
2026-Feb-25 07:33:21.147805
#60 11.87   Downloaded tower-http v0.6.8
2026-Feb-25 07:33:21.147805
#60 11.88   Downloaded tower v0.5.3
2026-Feb-25 07:33:21.147805
#60 11.90   Downloaded tower v0.4.13
2026-Feb-25 07:33:21.147805
#60 11.91   Downloaded typenum v1.19.0
2026-Feb-25 07:33:21.147805
#60 11.91   Downloaded unicode-segmentation v1.12.0
2026-Feb-25 07:33:21.147805
#60 11.92   Downloaded vcpkg v0.2.15
2026-Feb-25 07:33:21.147805
#60 12.01   Downloaded winnow v0.7.14
2026-Feb-25 07:33:21.280089
#60 12.02   Downloaded yaml-rust2 v0.8.1
2026-Feb-25 07:33:21.280089
#60 12.07   Downloaded openssl v0.10.75
2026-Feb-25 07:33:21.280089
#60 12.08   Downloaded tracing-subscriber v0.3.22
2026-Feb-25 07:33:21.280089
#60 12.09   Downloaded regex-syntax v0.8.9
2026-Feb-25 07:33:21.280089
#60 12.10   Downloaded zerotrie v0.2.3
2026-Feb-25 07:33:21.280089
#60 12.10   Downloaded zerocopy v0.8.39
2026-Feb-25 07:33:21.280089
#60 12.14   Downloaded tracing v0.1.44
2026-Feb-25 07:33:21.388476
#60 12.15   Downloaded zerovec v0.11.5
2026-Feb-25 07:33:21.388476
#60 12.16   Downloaded tonic v0.12.3
2026-Feb-25 07:33:21.388476
#60 12.17   Downloaded toml_edit v0.22.27
2026-Feb-25 07:33:21.388476
#60 12.17   Downloaded tokio-util v0.7.18
2026-Feb-25 07:33:21.388476
#60 12.18   Downloaded deno_core_icudata v0.0.73
2026-Feb-25 07:33:21.388476
#60 12.22   Downloaded url v2.5.8
2026-Feb-25 07:33:21.388476
#60 12.22   Downloaded syn v1.0.109
2026-Feb-25 07:33:21.388476
#60 12.23   Downloaded idna v1.1.0
2026-Feb-25 07:33:21.388476
#60 12.23   Downloaded icu_properties_data v2.1.2
2026-Feb-25 07:33:21.388476
#60 12.25   Downloaded petgraph v0.7.1
2026-Feb-25 07:33:21.506597
#60 12.26   Downloaded hyper v1.8.1
2026-Feb-25 07:33:21.506597
#60 12.27   Downloaded nom v7.1.3
2026-Feb-25 07:33:21.506597
#60 12.28   Downloaded hashbrown v0.16.1
2026-Feb-25 07:33:21.506597
#60 12.28   Downloaded utoipa v4.2.3
2026-Feb-25 07:33:21.506597
#60 12.29   Downloaded publicsuffix v2.3.0
2026-Feb-25 07:33:21.506597
#60 12.29   Downloaded mio v0.8.11
2026-Feb-25 07:33:21.506597
#60 12.29   Downloaded mio v1.1.1
2026-Feb-25 07:33:21.506597
#60 12.30   Downloaded indexmap v2.13.0
2026-Feb-25 07:33:21.506597
#60 12.31   Downloaded hyper-util v0.1.20
2026-Feb-25 07:33:21.506597
#60 12.31   Downloaded prettyplease v0.2.37
2026-Feb-25 07:33:21.506597
#60 12.32   Downloaded libc v0.2.182
2026-Feb-25 07:33:21.506597
#60 12.36   Downloaded minimal-lexical v0.2.1
2026-Feb-25 07:33:21.609650
#60 12.37   Downloaded utoipa-gen v4.3.1
2026-Feb-25 07:33:21.609650
#60 12.38   Downloaded memchr v2.8.0
2026-Feb-25 07:33:21.609650
#60 12.39   Downloaded rustls v0.23.36
2026-Feb-25 07:33:21.609650
#60 12.40   Downloaded miniz_oxide v0.8.9
2026-Feb-25 07:33:21.609650
#60 12.40   Downloaded tokio v1.49.0
2026-Feb-25 07:33:21.609650
#60 12.45   Downloaded icu_normalizer_data v2.1.1
2026-Feb-25 07:33:21.609650
#60 12.45   Downloaded icu_normalizer v2.1.1
2026-Feb-25 07:33:21.609650
#60 12.46   Downloaded icu_locale_core v2.1.1
2026-Feb-25 07:33:21.609650
#60 12.47   Downloaded brotli v8.0.2
2026-Feb-25 07:33:21.730429
#60 12.48   Downloaded tokio-native-tls v0.3.1
2026-Feb-25 07:33:21.730429
#60 12.48   Downloaded tokio-macros v2.6.0
2026-Feb-25 07:33:21.730429
#60 12.49   Downloaded ron v0.8.1
2026-Feb-25 07:33:21.730429
#60 12.49   Downloaded pin-project v1.1.10
2026-Feb-25 07:33:21.730429
#60 12.51   Downloaded pest_meta v2.8.6
2026-Feb-25 07:33:21.730429
#60 12.51   Downloaded openssl-sys v0.9.111
2026-Feb-25 07:33:21.730429
#60 12.52   Downloaded icu_properties v2.1.2
2026-Feb-25 07:33:21.730429
#60 12.52   Downloaded walkdir v2.5.0
2026-Feb-25 07:33:21.730429
#60 12.52   Downloaded notify v6.1.1
2026-Feb-25 07:33:21.730429
#60 12.53   Downloaded miniz_oxide v0.7.4
2026-Feb-25 07:33:21.730429
#60 12.53   Downloaded ucd-trie v0.1.7
2026-Feb-25 07:33:21.730429
#60 12.53   Downloaded tracing-core v0.1.36
2026-Feb-25 07:33:21.730429
#60 12.53   Downloaded num-traits v0.2.19
2026-Feb-25 07:33:21.730429
#60 12.54   Downloaded tracing-attributes v0.1.31
2026-Feb-25 07:33:21.730429
#60 12.54   Downloaded icu_provider v2.1.1
2026-Feb-25 07:33:21.730429
#60 12.54   Downloaded idna_adapter v1.2.1
2026-Feb-25 07:33:21.730429
#60 12.54   Downloaded ring v0.17.14
2026-Feb-25 07:33:21.730429
#60 12.59   Downloaded hyper-timeout v0.5.2
2026-Feb-25 07:33:21.964301
#60 12.59   Downloaded serde_v8 v0.209.0
2026-Feb-25 07:33:21.964301
#60 12.59   Downloaded serde_spanned v0.6.9
2026-Feb-25 07:33:21.964301
#60 12.59   Downloaded serde_path_to_error v0.1.20
2026-Feb-25 07:33:21.964301
#60 12.60   Downloaded paste v1.0.15
2026-Feb-25 07:33:21.964301
#60 12.60   Downloaded webpki-roots v1.0.6
2026-Feb-25 07:33:21.964301
#60 12.60   Downloaded nu-ansi-term v0.50.3
2026-Feb-25 07:33:21.964301
#60 12.61   Downloaded num-integer v0.1.46
2026-Feb-25 07:33:21.964301
#60 12.61   Downloaded linux-raw-sys v0.4.15
2026-Feb-25 07:33:21.964301
#60 12.67   Downloaded linux-raw-sys v0.11.0
2026-Feb-25 07:33:24.263937
#60 15.12   Downloaded v8 v0.101.0
2026-Feb-25 07:33:25.491867
#60 16.35    Compiling proc-macro2 v1.0.106
2026-Feb-25 07:33:25.491867
#60 16.35    Compiling unicode-ident v1.0.24
2026-Feb-25 07:33:25.599452
#60 16.35    Compiling quote v1.0.44
2026-Feb-25 07:33:25.599452
#60 16.35    Compiling cfg-if v1.0.4
2026-Feb-25 07:33:25.599452
#60 16.35    Compiling libc v0.2.182
2026-Feb-25 07:33:25.599452
#60 16.35    Compiling pin-project-lite v0.2.16
2026-Feb-25 07:33:25.599452
#60 16.35    Compiling once_cell v1.21.3
2026-Feb-25 07:33:25.599452
#60 16.36    Compiling serde_core v1.0.228
2026-Feb-25 07:33:25.599452
#60 16.36    Compiling futures-core v0.3.32
2026-Feb-25 07:33:25.599452
#60 16.37    Compiling bytes v1.11.1
2026-Feb-25 07:33:25.599452
#60 16.39    Compiling memchr v2.8.0
2026-Feb-25 07:33:25.599452
#60 16.41    Compiling parking_lot_core v0.9.12
2026-Feb-25 07:33:25.599452
#60 16.41    Compiling smallvec v1.15.1
2026-Feb-25 07:33:25.599452
#60 16.43    Compiling futures-sink v0.3.32
2026-Feb-25 07:33:25.599452
#60 16.44    Compiling anyhow v1.0.102
2026-Feb-25 07:33:25.599452
#60 16.44    Compiling scopeguard v1.2.0
2026-Feb-25 07:33:25.599452
#60 16.45    Compiling zerocopy v0.8.39
2026-Feb-25 07:33:25.599452
#60 16.46    Compiling slab v0.4.12
2026-Feb-25 07:33:25.707743
#60 16.46    Compiling log v0.4.29
2026-Feb-25 07:33:25.707743
#60 16.48    Compiling itoa v1.0.17
2026-Feb-25 07:33:25.707743
#60 16.48    Compiling futures-task v0.3.32
2026-Feb-25 07:33:25.707743
#60 16.49    Compiling futures-io v0.3.32
2026-Feb-25 07:33:25.707743
#60 16.51    Compiling serde v1.0.228
2026-Feb-25 07:33:25.707743
#60 16.52    Compiling either v1.15.0
2026-Feb-25 07:33:25.707743
#60 16.53    Compiling hashbrown v0.16.1
2026-Feb-25 07:33:25.707743
#60 16.56    Compiling equivalent v1.0.2
2026-Feb-25 07:33:25.842924
#60 16.56    Compiling crunchy v0.2.4
2026-Feb-25 07:33:25.842924
#60 16.57    Compiling version_check v0.9.5
2026-Feb-25 07:33:25.842924
#60 16.61    Compiling lock_api v0.4.14
2026-Feb-25 07:33:25.842924
#60 16.63    Compiling tracing-core v0.1.36
2026-Feb-25 07:33:25.842924
#60 16.63    Compiling futures-channel v0.3.32
2026-Feb-25 07:33:25.842924
#60 16.63    Compiling tiny-keccak v2.0.2
2026-Feb-25 07:33:25.842924
#60 16.63    Compiling tower-service v0.3.3
2026-Feb-25 07:33:25.842924
#60 16.66    Compiling rustix v1.1.3
2026-Feb-25 07:33:25.842924
#60 16.66    Compiling getrandom v0.4.1
2026-Feb-25 07:33:25.842924
#60 16.70    Compiling itertools v0.14.0
2026-Feb-25 07:33:25.980630
#60 16.72    Compiling ucd-trie v0.1.7
2026-Feb-25 07:33:25.980630
#60 16.72    Compiling regex-syntax v0.8.9
2026-Feb-25 07:33:25.980630
#60 16.72    Compiling httparse v1.10.1
2026-Feb-25 07:33:25.980630
#60 16.73    Compiling prettyplease v0.2.37
2026-Feb-25 07:33:25.980630
#60 16.74    Compiling allocator-api2 v0.2.21
2026-Feb-25 07:33:25.980630
#60 16.77    Compiling bitflags v2.11.0
2026-Feb-25 07:33:25.980630
#60 16.78    Compiling linux-raw-sys v0.11.0
2026-Feb-25 07:33:25.980630
#60 16.84    Compiling aho-corasick v1.1.4
2026-Feb-25 07:33:26.087970
#60 16.85    Compiling rustversion v1.0.22
2026-Feb-25 07:33:26.087970
#60 16.89    Compiling http v1.4.0
2026-Feb-25 07:33:26.087970
#60 16.90    Compiling ahash v0.8.12
2026-Feb-25 07:33:26.087970
#60 16.91    Compiling pest v2.8.6
2026-Feb-25 07:33:26.087970
#60 16.91    Compiling fastrand v2.3.0
2026-Feb-25 07:33:26.087970
#60 16.95    Compiling fnv v1.0.7
2026-Feb-25 07:33:26.188971
#60 16.97    Compiling tower-layer v0.3.3
2026-Feb-25 07:33:26.188971
#60 16.97    Compiling atomic-waker v1.1.2
2026-Feb-25 07:33:26.188971
#60 17.02    Compiling autocfg v1.5.0
2026-Feb-25 07:33:26.188971
#60 17.02    Compiling try-lock v0.2.5
2026-Feb-25 07:33:26.188971
#60 17.03    Compiling fixedbitset v0.5.7
2026-Feb-25 07:33:26.188971
#60 17.04    Compiling sync_wrapper v1.0.2
2026-Feb-25 07:33:26.188971
#60 17.05    Compiling pin-utils v0.1.0
2026-Feb-25 07:33:26.297531
#60 17.08    Compiling indexmap v2.13.0
2026-Feb-25 07:33:26.297531
#60 17.08    Compiling percent-encoding v2.3.2
2026-Feb-25 07:33:26.297531
#60 17.12    Compiling httpdate v1.0.3
2026-Feb-25 07:33:26.297531
#60 17.13    Compiling zmij v1.0.21
2026-Feb-25 07:33:26.297531
#60 17.15    Compiling want v0.3.1
2026-Feb-25 07:33:26.297531
#60 17.15    Compiling multimap v0.10.1
2026-Feb-25 07:33:26.397937
#60 17.17    Compiling heck v0.5.0
2026-Feb-25 07:33:26.397937
#60 17.18    Compiling serde_json v1.0.149
2026-Feb-25 07:33:26.397937
#60 17.20    Compiling hashbrown v0.12.3
2026-Feb-25 07:33:26.397937
#60 17.22    Compiling base64 v0.22.1
2026-Feb-25 07:33:26.397937
#60 17.25    Compiling ipnet v2.11.0
2026-Feb-25 07:33:26.500546
#60 17.27    Compiling winnow v0.7.14
2026-Feb-25 07:33:26.500546
#60 17.29    Compiling mime v0.3.17
2026-Feb-25 07:33:26.500546
#60 17.31    Compiling thiserror v1.0.69
2026-Feb-25 07:33:26.500546
#60 17.33    Compiling toml_write v0.1.2
2026-Feb-25 07:33:26.500546
#60 17.33    Compiling indexmap v1.9.3
2026-Feb-25 07:33:26.500546
#60 17.35    Compiling encoding_rs v0.8.35
2026-Feb-25 07:33:26.500546
#60 17.36    Compiling
2026-Feb-25 07:33:26.601696
minimal-lexical v0.2.1
2026-Feb-25 07:33:26.601696
#60 17.37    Compiling lazy_static v1.5.0
2026-Feb-25 07:33:26.601696
#60 17.41    Compiling syn v2.0.117
2026-Feb-25 07:33:26.601696
#60 17.46    Compiling matchit v0.7.3
2026-Feb-25 07:33:26.601696
#60 17.46    Compiling unicode-segmentation v1.12.0
2026-Feb-25 07:33:26.727743
#60 17.46    Compiling base64 v0.21.7
2026-Feb-25 07:33:26.727743
#60 17.49    Compiling errno v0.3.14
2026-Feb-25 07:33:26.727743
#60 17.50    Compiling mio v1.1.1
2026-Feb-25 07:33:26.727743
#60 17.51    Compiling socket2 v0.6.2
2026-Feb-25 07:33:26.727743
#60 17.54    Compiling getrandom v0.2.17
2026-Feb-25 07:33:26.727743
#60 17.58    Compiling arraydeque v0.5.1
2026-Feb-25 07:33:26.877918
#60 17.62    Compiling signal-hook-registry v1.4.8
2026-Feb-25 07:33:26.877918
#60 17.63    Compiling http-body v1.0.1
2026-Feb-25 07:33:26.877918
#60 17.63    Compiling nom v7.1.3
2026-Feb-25 07:33:26.877918
#60 17.66    Compiling parking_lot v0.12.5
2026-Feb-25 07:33:26.877918
#60 17.74    Compiling petgraph v0.7.1
2026-Feb-25 07:33:26.998920
#60 17.75    Compiling rand_core v0.6.4
2026-Feb-25 07:33:26.998920
#60 17.75    Compiling http-body-util v0.1.3
2026-Feb-25 07:33:26.998920
#60 17.75    Compiling sharded-slab v0.1.7
2026-Feb-25 07:33:26.998920
#60 17.75    Compiling socket2 v0.5.10
2026-Feb-25 07:33:26.998920
#60 17.79    Compiling tracing-log v0.2.0
2026-Feb-25 07:33:26.998920
#60 17.83    Compiling convert_case v0.6.0
2026-Feb-25 07:33:26.998920
#60 17.86    Compiling
2026-Feb-25 07:33:27.201466
const-random-macro v0.1.16
2026-Feb-25 07:33:27.201466
#60 17.87    Compiling thread_local v1.1.9
2026-Feb-25 07:33:27.201466
#60 17.89    Compiling pathdiff v0.2.3
2026-Feb-25 07:33:27.201466
#60 17.91    Compiling nu-ansi-term v0.50.3
2026-Feb-25 07:33:27.201466
#60 17.94    Compiling dotenvy v0.15.7
2026-Feb-25 07:33:27.201466
#60 18.06    Compiling pest_meta v2.8.6
2026-Feb-25 07:33:27.317648
#60 18.10    Compiling const-random v0.1.18
2026-Feb-25 07:33:27.317648
#60 18.14    Compiling dlv-list v0.5.2
2026-Feb-25 07:33:27.317648
#60 18.17    Compiling regex-automata v0.4.14
2026-Feb-25 07:33:27.559955
#60 18.42    Compiling tempfile v3.25.0
2026-Feb-25 07:33:28.574102
#60 19.43    Compiling regex v1.12.3
2026-Feb-25 07:33:28.956663
#60 19.81    Compiling matchers v0.2.0
2026-Feb-25 07:33:29.076933
#60 19.85    Compiling ppv-lite86 v0.2.21
2026-Feb-25 07:33:29.076933
#60 19.89    Compiling pest_generator v2.8.6
2026-Feb-25 07:33:29.076933
#60 19.93    Compiling hashbrown v0.14.5
2026-Feb-25 07:33:29.211294
#60 20.07    Compiling rand_chacha v0.3.1
2026-Feb-25 07:33:29.343370
#60 20.14    Compiling tokio-macros v2.6.0
2026-Feb-25 07:33:29.343370
#60 20.14    Compiling futures-macro v0.3.32
2026-Feb-25 07:33:29.343370
#60 20.14    Compiling tracing-attributes v0.1.31
2026-Feb-25 07:33:29.343370
#60 20.14    Compiling serde_derive v1.0.228
2026-Feb-25 07:33:29.343370
#60 20.14    Compiling prost-derive v0.13.5
2026-Feb-25 07:33:29.343370
#60 20.14    Compiling async-trait v0.1.89
2026-Feb-25 07:33:29.343370
#60 20.14    Compiling pin-project-internal v1.1.10
2026-Feb-25 07:33:29.343370
#60 20.14    Compiling async-stream-impl v0.3.6
2026-Feb-25 07:33:29.343370
#60 20.14    Compiling thiserror-impl v1.0.69
2026-Feb-25 07:33:29.343370
#60 20.20    Compiling rand v0.8.5
2026-Feb-25 07:33:29.552769
#60 20.27    Compiling pest_derive v2.8.6
2026-Feb-25 07:33:29.552769
#60 20.41    Compiling ordered-multimap v0.7.3
2026-Feb-25 07:33:29.552769
#60 20.41    Compiling hashlink v0.8.4
2026-Feb-25 07:33:29.738421
#60 20.43    Compiling async-stream v0.3.6
2026-Feb-25 07:33:29.738421
#60 20.47    Compiling tokio v1.49.0
2026-Feb-25 07:33:29.738421
#60 20.49    Compiling futures-util v0.3.32
2026-Feb-25 07:33:29.738421
#60 20.60    Compiling rust-ini v0.20.0
2026-Feb-25 07:33:29.953485
#60 20.64    Compiling pin-project v1.1.10
2026-Feb-25 07:33:29.953485
#60 20.64    Compiling yaml-rust2 v0.8.1
2026-Feb-25 07:33:29.953485
#60 20.68    Compiling tracing v0.1.44
2026-Feb-25 07:33:29.953485
#60 20.81    Compiling tracing-subscriber v0.3.22
2026-Feb-25 07:33:30.159049
#60 20.87    Compiling prost v0.13.5
2026-Feb-25 07:33:30.193819
#60 21.05    Compiling prost-types v0.13.5
2026-Feb-25 07:33:30.750258
#60 21.61    Compiling prost-build v0.13.5
2026-Feb-25 07:33:31.013225
#60 21.87    Compiling toml_datetime v0.6.11
2026-Feb-25 07:33:31.132096
#60 21.87    Compiling serde_spanned v0.6.9
2026-Feb-25 07:33:31.132096
#60 21.87    Compiling json5 v0.4.1
2026-Feb-25 07:33:31.132096
#60 21.87    Compiling ron v0.8.1
2026-Feb-25 07:33:31.132096
#60 21.94    Compiling tonic-build v0.12.3
2026-Feb-25 07:33:31.132096
#60 21.99    Compiling toml_edit v0.22.27
2026-Feb-25 07:33:31.362569
#60 22.22    Compiling axum-core v0.4.5
2026-Feb-25 07:33:31.584000
#60 22.22    Compiling futures-executor v0.3.32
2026-Feb-25 07:33:31.584000
#60 22.30    Compiling futures v0.3.32
2026-Feb-25 07:33:31.584000
#60 22.44    Compiling gpu-worker v0.1.0 (/app/crates/gpu-worker)
2026-Feb-25 07:33:31.774541
#60 22.48 warning: use of deprecated method `tonic_build::Builder::compile`: renamed to `compile_protos()`
2026-Feb-25 07:33:31.774541
#60 22.48   --> crates/gpu-worker/build.rs:16:10
2026-Feb-25 07:33:31.774541
#60 22.48    |
2026-Feb-25 07:33:31.774541
#60 22.48 16 |         .compile(&[proto_file], &["../../proto"])?;
2026-Feb-25 07:33:31.774541
#60 22.48    |          ^^^^^^^
2026-Feb-25 07:33:31.774541
#60 22.48    |
2026-Feb-25 07:33:31.774541
#60 22.48    = note: `#[warn(deprecated)]` on by default
2026-Feb-25 07:33:31.774541
#60 22.48
2026-Feb-25 07:33:31.826828
#60 22.68 warning: `gpu-worker` (build script) generated 1 warning
2026-Feb-25 07:33:32.445092
#60 23.30    Compiling toml v0.8.23
2026-Feb-25 07:33:32.657569
#60 23.36    Compiling tokio-util v0.7.18
2026-Feb-25 07:33:32.657569
#60 23.36    Compiling tower v0.5.3
2026-Feb-25 07:33:32.657569
#60 23.36    Compiling tokio-stream v0.1.18
2026-Feb-25 07:33:32.657569
#60 23.36    Compiling gpu-pipeline v0.1.0 (/app/crates/gpu-pipeline)
2026-Feb-25 07:33:32.736811
#60 23.59    Compiling config v0.14.1
2026-Feb-25 07:33:32.843939
#60 23.69    Compiling h2 v0.4.13
2026-Feb-25 07:33:32.843939
#60 23.69    Compiling tower v0.4.13
2026-Feb-25 07:33:32.843939
#60 23.70    Compiling axum v0.7.9
2026-Feb-25 07:33:35.351410
#60 26.21    Compiling hyper v1.8.1
2026-Feb-25 07:33:36.242096
#60 27.10    Compiling hyper-util v0.1.20
2026-Feb-25 07:33:37.228456
#60 28.09    Compiling hyper-timeout v0.5.2
2026-Feb-25 07:33:37.478655
#60 28.19    Compiling tonic v0.12.3
2026-Feb-25 07:33:38.607318
#60 29.46 warning: field `config` is never read
2026-Feb-25 07:33:38.607318
#60 29.46   --> crates/gpu-worker/src/server.rs:57:5
2026-Feb-25 07:33:38.607318
#60 29.46    |
2026-Feb-25 07:33:38.607318
#60 29.46 55 | pub struct GpuWorkerServer {
2026-Feb-25 07:33:38.607318
#60 29.46    |            --------------- field in this struct
2026-Feb-25 07:33:38.607318
#60 29.46 56 |     bind_addr: SocketAddr,
2026-Feb-25 07:33:38.607318
#60 29.46 57 |     config: ServerConfig,
2026-Feb-25 07:33:38.607318
#60 29.46    |     ^^^^^^
2026-Feb-25 07:33:38.607318
#60 29.46    |
2026-Feb-25 07:33:38.607318
#60 29.46    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default
2026-Feb-25 07:33:38.607318
#60 29.46
2026-Feb-25 07:33:38.723280
#60 29.58 warning: `gpu-worker` (lib) generated 1 warning
2026-Feb-25 07:33:55.732686
#60 46.59     Finished `release` profile [optimized] target(s) in 46.38s
2026-Feb-25 07:33:55.922300
#60 DONE 46.8s
2026-Feb-25 07:33:56.113888
#61 [gpu-worker runtime 5/6] COPY --from=builder /app/target/release/gpu-node /usr/local/bin/
2026-Feb-25 07:33:56.113888
#61 DONE 0.0s
2026-Feb-25 07:33:56.113888
2026-Feb-25 07:33:56.113888
#62 [gpu-worker runtime 6/6] RUN mkdir -p /app/data && chown -R appuser:appuser /app
2026-Feb-25 07:33:56.113888
#62 DONE 0.1s
2026-Feb-25 07:33:56.266792
#63 [gpu-worker] exporting to image
2026-Feb-25 07:33:56.266792
#63 exporting layers
2026-Feb-25 07:34:11.355492
#63 exporting layers 15.2s done
2026-Feb-25 07:34:11.533578
#63 exporting manifest sha256:a204de5eee386b669cf4329535afe2c999a1ec201f28d7a4b46a672ee0a956ee done
2026-Feb-25 07:34:11.533578
#63 exporting config sha256:f1807f5bec10c85eef67afff233db8efe85a4b9cc0f27a4d5994217dc598a4ad done
2026-Feb-25 07:34:11.533578
#63 exporting attestation manifest sha256:3916d687d5e303bc03ddbb9ad59e41d969c93aa6284a97a992edcba40b6de756 done
2026-Feb-25 07:34:11.533578
#63 exporting manifest list sha256:5f1dddcd2ef4ab5b4b99e9c2fe1afb2aff77f276be710242a866d7532c13e83f done
2026-Feb-25 07:34:11.533578
#63 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_gpu-worker:6284ccef8da2b1bbb3aa00193368912f946f4bb3 done
2026-Feb-25 07:34:11.533578
#63 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_gpu-worker:6284ccef8da2b1bbb3aa00193368912f946f4bb3
2026-Feb-25 07:34:15.246566
#63 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_gpu-worker:6284ccef8da2b1bbb3aa00193368912f946f4bb3 3.9s done
2026-Feb-25 07:34:15.326528
#63 DONE 19.2s
2026-Feb-25 07:34:15.326528
2026-Feb-25 07:34:15.326528
#64 [gpu-worker] resolving provenance for metadata file
2026-Feb-25 07:34:15.326528
#64 DONE 0.0s
2026-Feb-25 07:34:15.343805
frontend  Built
2026-Feb-25 07:34:15.343805
gpu-worker  Built
2026-Feb-25 07:34:15.343805
api  Built
2026-Feb-25 07:34:15.365042
Creating .env file with runtime variables for container.
2026-Feb-25 07:34:15.781038
Removing old containers.
2026-Feb-25 07:34:16.409381
[CMD]: docker stop -t 30 frontend-o8kccgkgwsockoocow8sg88s-070328059548
2026-Feb-25 07:34:16.409381
frontend-o8kccgkgwsockoocow8sg88s-070328059548
2026-Feb-25 07:34:16.553887
[CMD]: docker rm -f frontend-o8kccgkgwsockoocow8sg88s-070328059548
2026-Feb-25 07:34:16.553887
frontend-o8kccgkgwsockoocow8sg88s-070328059548
2026-Feb-25 07:34:46.930601
[CMD]: docker stop -t 30 api-o8kccgkgwsockoocow8sg88s-070328057769
2026-Feb-25 07:34:46.930601
api-o8kccgkgwsockoocow8sg88s-070328057769
2026-Feb-25 07:34:47.061077
[CMD]: docker rm -f api-o8kccgkgwsockoocow8sg88s-070328057769
2026-Feb-25 07:34:47.061077
api-o8kccgkgwsockoocow8sg88s-070328057769
2026-Feb-25 07:35:17.429902
[CMD]: docker stop -t 30 gpu-worker-o8kccgkgwsockoocow8sg88s-070328055535
2026-Feb-25 07:35:17.429902
gpu-worker-o8kccgkgwsockoocow8sg88s-070328055535
2026-Feb-25 07:35:17.571214
[CMD]: docker rm -f gpu-worker-o8kccgkgwsockoocow8sg88s-070328055535
2026-Feb-25 07:35:17.571214
gpu-worker-o8kccgkgwsockoocow8sg88s-070328055535
2026-Feb-25 07:35:17.588575
Starting new application.
2026-Feb-25 07:35:18.064332
[CMD]: docker exec zogw0c04g4okg8w4wokkgs4k bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/zogw0c04g4okg8w4wokkgs4k/.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/zogw0c04g4okg8w4wokkgs4k -f /artifacts/zogw0c04g4okg8w4wokkgs4k/docker/docker-compose.homeserver.yml up -d'
2026-Feb-25 07:35:18.064332
Container gpu-worker-o8kccgkgwsockoocow8sg88s-071028193752  Creating
2026-Feb-25 07:35:18.100464
Container gpu-worker-o8kccgkgwsockoocow8sg88s-071028193752  Created
2026-Feb-25 07:35:18.100464
Container api-o8kccgkgwsockoocow8sg88s-071028199257  Creating
2026-Feb-25 07:35:18.126845
Container api-o8kccgkgwsockoocow8sg88s-071028199257  Created
2026-Feb-25 07:35:18.126845
Container frontend-o8kccgkgwsockoocow8sg88s-071028203514  Creating
2026-Feb-25 07:35:18.152267
Container frontend-o8kccgkgwsockoocow8sg88s-071028203514  Created
2026-Feb-25 07:35:18.169261
Container gpu-worker-o8kccgkgwsockoocow8sg88s-071028193752  Starting
2026-Feb-25 07:35:18.401931
Container gpu-worker-o8kccgkgwsockoocow8sg88s-071028193752  Started
2026-Feb-25 07:35:18.401931
Container api-o8kccgkgwsockoocow8sg88s-071028199257  Starting
2026-Feb-25 07:35:18.581637
Container api-o8kccgkgwsockoocow8sg88s-071028199257  Started
2026-Feb-25 07:35:18.581637
Container frontend-o8kccgkgwsockoocow8sg88s-071028203514  Starting
2026-Feb-25 07:35:18.731022
Container frontend-o8kccgkgwsockoocow8sg88s-071028203514  Started
2026-Feb-25 07:35:19.036567
New container started.
2026-Feb-25 07:35:19.356766
Gracefully shutting down build container: zogw0c04g4okg8w4wokkgs4k
2026-Feb-25 07:35:19.642617
[CMD]: docker stop -t 30 zogw0c04g4okg8w4wokkgs4k
2026-Feb-25 07:35:19.642617
zogw0c04g4okg8w4wokkgs4k