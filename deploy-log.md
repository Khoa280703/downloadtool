2026-Feb-25 06:38:57.168887
Starting deployment of khoa280703/downloadtool:main-zoscg4oc04gkwkssg0kw8w8w to localhost.
2026-Feb-25 06:38:57.765312
Preparing container with helper image: ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Feb-25 06:38:58.072358
[CMD]: docker stop -t 30 hswgk0sgs0cc8gswk48gwgow
2026-Feb-25 06:38:58.072358
Error response from daemon: No such container: hswgk0sgs0cc8gswk48gwgow
2026-Feb-25 06:38:58.412344
[CMD]: docker run -d --network coolify --name hswgk0sgs0cc8gswk48gwgow  --rm -v /var/run/docker.sock:/var/run/docker.sock ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Feb-25 06:38:58.412344
4c1a2ab9df0b0bb9656e5d2a8022523ea1b9b3d09e9f40d5d021913bbb920d64
2026-Feb-25 06:38:59.980154
[CMD]: docker exec hswgk0sgs0cc8gswk48gwgow bash -c 'GIT_SSH_COMMAND="ssh -o ConnectTimeout=30 -p 22 -o Port=22 -o LogLevel=ERROR -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git ls-remote https://github.com/Khoa280703/downloadtool refs/heads/main'
2026-Feb-25 06:38:59.980154
d0f6612405c79d539ced04eb7e60601aa1f793a1	refs/heads/main
2026-Feb-25 06:38:59.990199
----------------------------------------
2026-Feb-25 06:38:59.994093
Importing Khoa280703/downloadtool:main (commit sha d0f6612405c79d539ced04eb7e60601aa1f793a1) to /artifacts/hswgk0sgs0cc8gswk48gwgow.
2026-Feb-25 06:39:00.344331
[CMD]: docker exec hswgk0sgs0cc8gswk48gwgow bash -c 'git clone --depth=1 --recurse-submodules --shallow-submodules -b 'main' 'https://github.com/Khoa280703/downloadtool' '/artifacts/hswgk0sgs0cc8gswk48gwgow' && cd '/artifacts/hswgk0sgs0cc8gswk48gwgow' && if [ -f .gitmodules ]; then sed -i "s#git@\(.*\):#https://\1/#g" '/artifacts/hswgk0sgs0cc8gswk48gwgow'/.gitmodules || true && git submodule sync && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git submodule update --init --recursive --depth=1; fi && cd '/artifacts/hswgk0sgs0cc8gswk48gwgow' && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git lfs pull'
2026-Feb-25 06:39:00.344331
Cloning into '/artifacts/hswgk0sgs0cc8gswk48gwgow'...
2026-Feb-25 06:39:02.830186
[CMD]: docker exec hswgk0sgs0cc8gswk48gwgow bash -c 'cd /artifacts/hswgk0sgs0cc8gswk48gwgow && git log -1 d0f6612405c79d539ced04eb7e60601aa1f793a1 --pretty=%B'
2026-Feb-25 06:39:02.830186
fix deploy
2026-Feb-25 06:39:07.408425
[CMD]: docker exec hswgk0sgs0cc8gswk48gwgow bash -c 'test -f /artifacts/hswgk0sgs0cc8gswk48gwgow/docker/Dockerfile.homeserver && echo 'exists' || echo 'not found''
2026-Feb-25 06:39:07.408425
exists
2026-Feb-25 06:39:07.774476
[CMD]: docker exec hswgk0sgs0cc8gswk48gwgow bash -c 'cat /artifacts/hswgk0sgs0cc8gswk48gwgow/docker/Dockerfile.homeserver'
2026-Feb-25 06:39:07.774476
# Dockerfile for Home Server deployment
2026-Feb-25 06:39:07.774476
# Builds the GPU worker with CUDA support for hardware transcoding
2026-Feb-25 06:39:07.774476
2026-Feb-25 06:39:07.774476
FROM nvidia/cuda:12.3.2-devel-ubuntu22.04 AS builder
2026-Feb-25 06:39:07.774476
2026-Feb-25 06:39:07.774476
WORKDIR /app
2026-Feb-25 06:39:07.774476
2026-Feb-25 06:39:07.774476
# Install dependencies
2026-Feb-25 06:39:07.774476
RUN apt-get update && apt-get install -y \
2026-Feb-25 06:39:07.774476
curl \
2026-Feb-25 06:39:07.774476
build-essential \
2026-Feb-25 06:39:07.774476
pkg-config \
2026-Feb-25 06:39:07.774476
libssl-dev \
2026-Feb-25 06:39:07.774476
protobuf-compiler \
2026-Feb-25 06:39:07.774476
ffmpeg \
2026-Feb-25 06:39:07.774476
libavcodec-dev \
2026-Feb-25 06:39:07.774476
libavformat-dev \
2026-Feb-25 06:39:07.774476
libavutil-dev \
2026-Feb-25 06:39:07.774476
libswscale-dev \
2026-Feb-25 06:39:07.774476
libavfilter-dev \
2026-Feb-25 06:39:07.774476
libavdevice-dev \
2026-Feb-25 06:39:07.774476
clang \
2026-Feb-25 06:39:07.774476
libclang-dev \
2026-Feb-25 06:39:07.774476
&& rm -rf /var/lib/apt/lists/*
2026-Feb-25 06:39:07.774476
2026-Feb-25 06:39:07.774476
# Install Rust
2026-Feb-25 06:39:07.774476
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
2026-Feb-25 06:39:07.774476
ENV PATH="/root/.cargo/bin:${PATH}"
2026-Feb-25 06:39:07.774476
2026-Feb-25 06:39:07.774476
# Copy workspace configuration
2026-Feb-25 06:39:07.774476
COPY Cargo.toml ./
2026-Feb-25 06:39:07.774476
COPY Cargo.lock ./
2026-Feb-25 06:39:07.774476
2026-Feb-25 06:39:07.774476
# Copy all crates
2026-Feb-25 06:39:07.774476
COPY crates/ ./crates/
2026-Feb-25 06:39:07.774476
2026-Feb-25 06:39:07.774476
# Copy proto files
2026-Feb-25 06:39:07.774476
COPY proto/ ./proto/
2026-Feb-25 06:39:07.774476
2026-Feb-25 06:39:07.774476
# Build the GPU worker with CUDA support
2026-Feb-25 06:39:07.774476
# NOTE: --features gpu disabled until FFmpeg 7.x is available in the build environment
2026-Feb-25 06:39:07.774476
# Ubuntu 22.04 apt provides FFmpeg 4.4.x but ffmpeg-next = "7" requires FFmpeg 7.x
2026-Feb-25 06:39:07.774476
RUN cargo build --release --bin gpu-node
2026-Feb-25 06:39:07.774476
2026-Feb-25 06:39:07.774476
# Stage 2: Runtime
2026-Feb-25 06:39:07.774476
FROM nvidia/cuda:12.3.2-runtime-ubuntu22.04 AS runtime
2026-Feb-25 06:39:07.774476
2026-Feb-25 06:39:07.774476
WORKDIR /app
2026-Feb-25 06:39:07.774476
2026-Feb-25 06:39:07.774476
# Install runtime dependencies
2026-Feb-25 06:39:07.774476
RUN apt-get update && apt-get install -y \
2026-Feb-25 06:39:07.774476
ca-certificates \
2026-Feb-25 06:39:07.774476
libssl3 \
2026-Feb-25 06:39:07.774476
ffmpeg \
2026-Feb-25 06:39:07.774476
libavcodec58 \
2026-Feb-25 06:39:07.774476
libavformat58 \
2026-Feb-25 06:39:07.774476
libavutil56 \
2026-Feb-25 06:39:07.774476
libswscale5 \
2026-Feb-25 06:39:07.774476
libavfilter7 \
2026-Feb-25 06:39:07.774476
libavdevice58 \
2026-Feb-25 06:39:07.774476
&& rm -rf /var/lib/apt/lists/*
2026-Feb-25 06:39:07.774476
2026-Feb-25 06:39:07.774476
# Create non-root user
2026-Feb-25 06:39:07.774476
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-25 06:39:07.774476
2026-Feb-25 06:39:07.774476
# Copy binary from builder
2026-Feb-25 06:39:07.774476
COPY --from=builder /app/target/release/gpu-node /usr/local/bin/
2026-Feb-25 06:39:07.774476
2026-Feb-25 06:39:07.774476
# Create directories
2026-Feb-25 06:39:07.774476
RUN mkdir -p /app/data && chown -R appuser:appuser /app
2026-Feb-25 06:39:07.774476
2026-Feb-25 06:39:07.774476
# Switch to non-root user
2026-Feb-25 06:39:07.774476
USER appuser
2026-Feb-25 06:39:07.774476
2026-Feb-25 06:39:07.774476
# Environment variables
2026-Feb-25 06:39:07.774476
ENV GPU_WORKER_BIND=0.0.0.0:50051
2026-Feb-25 06:39:07.774476
ENV GPU_WORKER_MAX_JOBS=4
2026-Feb-25 06:39:07.774476
ENV CUDA_DEVICE_ID=0
2026-Feb-25 06:39:07.774476
ENV RUST_LOG=info
2026-Feb-25 06:39:07.774476
2026-Feb-25 06:39:07.774476
# NVIDIA runtime configuration
2026-Feb-25 06:39:07.774476
ENV NVIDIA_VISIBLE_DEVICES=all
2026-Feb-25 06:39:07.774476
ENV NVIDIA_DRIVER_CAPABILITIES=compute,video,utility
2026-Feb-25 06:39:07.774476
2026-Feb-25 06:39:07.774476
# Expose gRPC port
2026-Feb-25 06:39:07.774476
EXPOSE 50051
2026-Feb-25 06:39:07.774476
2026-Feb-25 06:39:07.774476
# Health check
2026-Feb-25 06:39:07.774476
HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
2026-Feb-25 06:39:07.774476
CMD echo "health check placeholder" || exit 0
2026-Feb-25 06:39:07.774476
2026-Feb-25 06:39:07.774476
# Run the GPU worker
2026-Feb-25 06:39:07.774476
CMD ["gpu-node"]
2026-Feb-25 06:39:08.139195
Added 4 ARG declarations to Dockerfile for service gpu-worker (multi-stage build, added to 2 stages).
2026-Feb-25 06:39:08.510222
[CMD]: docker exec hswgk0sgs0cc8gswk48gwgow bash -c 'test -f /artifacts/hswgk0sgs0cc8gswk48gwgow/docker/Dockerfile.vps && echo 'exists' || echo 'not found''
2026-Feb-25 06:39:08.510222
exists
2026-Feb-25 06:39:08.870206
[CMD]: docker exec hswgk0sgs0cc8gswk48gwgow bash -c 'cat /artifacts/hswgk0sgs0cc8gswk48gwgow/docker/Dockerfile.vps'
2026-Feb-25 06:39:08.870206
# Dockerfile for VPS deployment
2026-Feb-25 06:39:08.870206
# Builds the API server and related components without GPU support
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
# Stage 0: Build injector JS (embedded into api crate via include_str! at compile time)
2026-Feb-25 06:39:08.870206
FROM node:22-alpine AS js-builder
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
WORKDIR /app
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
RUN npm install -g pnpm
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
# Copy workspace manifests for pnpm resolution
2026-Feb-25 06:39:08.870206
COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Feb-25 06:39:08.870206
COPY packages/api-client/package.json ./packages/api-client/
2026-Feb-25 06:39:08.870206
COPY apps/injector/package.json ./apps/injector/
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
# Copy injector source and shared packages
2026-Feb-25 06:39:08.870206
COPY apps/injector/ ./apps/injector/
2026-Feb-25 06:39:08.870206
COPY packages/ ./packages/
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
# Install deps and build injector (produces dist/bm.js and dist/youtube-downloader.user.js)
2026-Feb-25 06:39:08.870206
RUN pnpm install --frozen-lockfile
2026-Feb-25 06:39:08.870206
RUN pnpm --filter @downloadtool/injector build
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
# Stage 1: Rust builder
2026-Feb-25 06:39:08.870206
FROM rust:1.88-bookworm AS builder
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
WORKDIR /app
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
# Install dependencies
2026-Feb-25 06:39:08.870206
RUN apt-get update && apt-get install -y \
2026-Feb-25 06:39:08.870206
pkg-config \
2026-Feb-25 06:39:08.870206
libssl-dev \
2026-Feb-25 06:39:08.870206
protobuf-compiler \
2026-Feb-25 06:39:08.870206
&& rm -rf /var/lib/apt/lists/*
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
# Copy workspace configuration
2026-Feb-25 06:39:08.870206
COPY Cargo.toml ./
2026-Feb-25 06:39:08.870206
COPY Cargo.lock ./
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
# Copy all crates
2026-Feb-25 06:39:08.870206
COPY crates/ ./crates/
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
# Copy proto files
2026-Feb-25 06:39:08.870206
COPY proto/ ./proto/
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
# Copy injector dist (required by include_str! in crates/api/src/routes/static_files.rs)
2026-Feb-25 06:39:08.870206
COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
# Build the release binary
2026-Feb-25 06:39:08.870206
RUN cargo build --release --bin vps-gateway
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
# Stage 2: Runtime
2026-Feb-25 06:39:08.870206
FROM debian:bookworm-slim AS runtime
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
WORKDIR /app
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
# Install runtime dependencies
2026-Feb-25 06:39:08.870206
RUN apt-get update && apt-get install -y \
2026-Feb-25 06:39:08.870206
ca-certificates \
2026-Feb-25 06:39:08.870206
libssl3 \
2026-Feb-25 06:39:08.870206
&& rm -rf /var/lib/apt/lists/*
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
# Create non-root user
2026-Feb-25 06:39:08.870206
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
# Copy binary from builder
2026-Feb-25 06:39:08.870206
COPY --from=builder /app/target/release/vps-gateway /usr/local/bin/
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
# Create directories
2026-Feb-25 06:39:08.870206
RUN mkdir -p /app/extractors /app/data && chown -R appuser:appuser /app
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
# Switch to non-root user
2026-Feb-25 06:39:08.870206
USER appuser
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
# Environment variables
2026-Feb-25 06:39:08.870206
ENV PORT=3068
2026-Feb-25 06:39:08.870206
ENV EXTRACTOR_DIR=/app/extractors
2026-Feb-25 06:39:08.870206
ENV GPU_ENABLED=false
2026-Feb-25 06:39:08.870206
ENV RUST_LOG=info
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
# Expose port
2026-Feb-25 06:39:08.870206
EXPOSE 3068
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
# Health check
2026-Feb-25 06:39:08.870206
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Feb-25 06:39:08.870206
CMD curl -f http://localhost:3068/health || exit 1
2026-Feb-25 06:39:08.870206
2026-Feb-25 06:39:08.870206
# Run the server
2026-Feb-25 06:39:08.870206
CMD ["vps-gateway"]
2026-Feb-25 06:39:09.249771
Added 6 ARG declarations to Dockerfile for service api (multi-stage build, added to 3 stages).
2026-Feb-25 06:39:09.615692
[CMD]: docker exec hswgk0sgs0cc8gswk48gwgow bash -c 'test -f /artifacts/hswgk0sgs0cc8gswk48gwgow/docker/Dockerfile.frontend && echo 'exists' || echo 'not found''
2026-Feb-25 06:39:09.615692
exists
2026-Feb-25 06:39:09.971441
[CMD]: docker exec hswgk0sgs0cc8gswk48gwgow bash -c 'cat /artifacts/hswgk0sgs0cc8gswk48gwgow/docker/Dockerfile.frontend'
2026-Feb-25 06:39:09.971441
# Dockerfile for frontend (SvelteKit Node server)
2026-Feb-25 06:39:09.971441
# Copy ALL source files BEFORE npm install so svelte-kit sync (prepare script)
2026-Feb-25 06:39:09.971441
# can find svelte.config.js and generate .svelte-kit/ correctly.
2026-Feb-25 06:39:09.971441
2026-Feb-25 06:39:09.971441
FROM node:22-alpine AS builder
2026-Feb-25 06:39:09.971441
2026-Feb-25 06:39:09.971441
WORKDIR /app
2026-Feb-25 06:39:09.971441
2026-Feb-25 06:39:09.971441
# Copy all frontend source files first (node_modules excluded via .dockerignore)
2026-Feb-25 06:39:09.971441
COPY frontend/ ./
2026-Feb-25 06:39:09.971441
2026-Feb-25 06:39:09.971441
# Install — prepare script runs svelte-kit sync with svelte.config.js available
2026-Feb-25 06:39:09.971441
RUN npm install
2026-Feb-25 06:39:09.971441
2026-Feb-25 06:39:09.971441
# Build via programmatic API — bypasses vite.config.ts loading (esbuild issues on Alpine)
2026-Feb-25 06:39:09.971441
RUN node build-docker.mjs
2026-Feb-25 06:39:09.971441
2026-Feb-25 06:39:09.971441
# Runtime
2026-Feb-25 06:39:09.971441
FROM node:22-alpine AS runtime
2026-Feb-25 06:39:09.971441
2026-Feb-25 06:39:09.971441
WORKDIR /app
2026-Feb-25 06:39:09.971441
2026-Feb-25 06:39:09.971441
COPY --from=builder /app/build ./build
2026-Feb-25 06:39:09.971441
COPY --from=builder /app/package.json ./
2026-Feb-25 06:39:09.971441
2026-Feb-25 06:39:09.971441
ENV PORT=3000
2026-Feb-25 06:39:09.971441
ENV HOST=0.0.0.0
2026-Feb-25 06:39:09.971441
2026-Feb-25 06:39:09.971441
EXPOSE 3000
2026-Feb-25 06:39:09.971441
2026-Feb-25 06:39:09.971441
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Feb-25 06:39:09.971441
CMD wget -qO- http://localhost:3000 || exit 1
2026-Feb-25 06:39:09.971441
2026-Feb-25 06:39:09.971441
CMD ["node", "build"]
2026-Feb-25 06:39:10.353920
Added 4 ARG declarations to Dockerfile for service frontend (multi-stage build, added to 2 stages).
2026-Feb-25 06:39:10.358914
Pulling & building required images.
2026-Feb-25 06:39:10.364572
Creating build-time .env file in /artifacts (outside Docker context).
2026-Feb-25 06:39:10.721483
Adding build arguments to Docker Compose build command.
2026-Feb-25 06:39:11.192836
[CMD]: docker exec hswgk0sgs0cc8gswk48gwgow bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/build-time.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/hswgk0sgs0cc8gswk48gwgow -f /artifacts/hswgk0sgs0cc8gswk48gwgow/docker/docker-compose.homeserver.yml build --pull --build-arg COOLIFY_URL --build-arg COOLIFY_FQDN --build-arg COOLIFY_BUILD_SECRETS_HASH=1bf8dfcfdeb73162b126768f9e085efbcfdffa61f0529746b67826c254109afc'
2026-Feb-25 06:39:11.192836
#1 [internal] load local bake definitions
2026-Feb-25 06:39:11.400641
#1 reading from stdin 1.79kB done
2026-Feb-25 06:39:11.400641
#1 DONE 0.0s
2026-Feb-25 06:39:11.400641
2026-Feb-25 06:39:11.400641
#2 [api internal] load build definition from Dockerfile.vps
2026-Feb-25 06:39:11.400641
#2 transferring dockerfile: 2.35kB done
2026-Feb-25 06:39:11.400641
#2 DONE 0.0s
2026-Feb-25 06:39:11.400641
2026-Feb-25 06:39:11.400641
#3 [frontend internal] load build definition from Dockerfile.frontend
2026-Feb-25 06:39:11.400641
#3 transferring dockerfile: 999B done
2026-Feb-25 06:39:11.400641
#3 DONE 0.0s
2026-Feb-25 06:39:11.400641
2026-Feb-25 06:39:11.400641
#4 [gpu-worker internal] load build definition from Dockerfile.homeserver
2026-Feb-25 06:39:11.400641
#4 transferring dockerfile: 2.28kB done
2026-Feb-25 06:39:11.400641
#4 DONE 0.0s
2026-Feb-25 06:39:11.400641
2026-Feb-25 06:39:11.400641
#5 [frontend internal] load metadata for docker.io/library/node:22-alpine
2026-Feb-25 06:39:12.571881
#5 ...
2026-Feb-25 06:39:12.571881
2026-Feb-25 06:39:12.571881
#6 [api internal] load metadata for docker.io/library/debian:bookworm-slim
2026-Feb-25 06:39:12.571881
#6 DONE 1.3s
2026-Feb-25 06:39:12.672109
#5 [api internal] load metadata for docker.io/library/node:22-alpine
2026-Feb-25 06:39:12.672109
#5 DONE 1.3s
2026-Feb-25 06:39:12.672109
2026-Feb-25 06:39:12.672109
#7 [api internal] load metadata for docker.io/library/rust:1.88-bookworm
2026-Feb-25 06:39:12.672109
#7 DONE 1.3s
2026-Feb-25 06:39:12.672109
2026-Feb-25 06:39:12.672109
#8 [gpu-worker internal] load metadata for docker.io/nvidia/cuda:12.3.2-devel-ubuntu22.04
2026-Feb-25 06:39:12.672109
#8 DONE 1.3s
2026-Feb-25 06:39:12.672109
2026-Feb-25 06:39:12.672109
#9 [gpu-worker internal] load metadata for docker.io/nvidia/cuda:12.3.2-runtime-ubuntu22.04
2026-Feb-25 06:39:12.672109
#9 DONE 1.3s
2026-Feb-25 06:39:12.672109
2026-Feb-25 06:39:12.672109
#10 [frontend internal] load .dockerignore
2026-Feb-25 06:39:12.672109
#10 transferring context: 341B done
2026-Feb-25 06:39:12.672109
#10 DONE 0.0s
2026-Feb-25 06:39:12.672109
2026-Feb-25 06:39:12.672109
#11 [api builder 1/5] FROM docker.io/library/node:22-alpine@sha256:e4bf2a82ad0a4037d28035ae71529873c069b13eb0455466ae0bc13363826e34
2026-Feb-25 06:39:12.672109
#11 resolve docker.io/library/node:22-alpine@sha256:e4bf2a82ad0a4037d28035ae71529873c069b13eb0455466ae0bc13363826e34 0.0s done
2026-Feb-25 06:39:12.672109
#11 DONE 0.0s
2026-Feb-25 06:39:12.672109
2026-Feb-25 06:39:12.672109
#12 [api runtime 1/6] FROM docker.io/library/debian:bookworm-slim@sha256:74d56e3931e0d5a1dd51f8c8a2466d21de84a271cd3b5a733b803aa91abf4421
2026-Feb-25 06:39:12.672109
#12 resolve docker.io/library/debian:bookworm-slim@sha256:74d56e3931e0d5a1dd51f8c8a2466d21de84a271cd3b5a733b803aa91abf4421 0.0s done
2026-Feb-25 06:39:12.672109
#12 DONE 0.0s
2026-Feb-25 06:39:12.672109
2026-Feb-25 06:39:12.672109
#13 [api builder 1/9] FROM docker.io/library/rust:1.88-bookworm@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0
2026-Feb-25 06:39:12.672109
#13 resolve docker.io/library/rust:1.88-bookworm@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0 0.0s done
2026-Feb-25 06:39:12.672109
#13 DONE 0.0s
2026-Feb-25 06:39:12.672109
2026-Feb-25 06:39:12.672109
#14 [gpu-worker builder 1/9] FROM docker.io/nvidia/cuda:12.3.2-devel-ubuntu22.04@sha256:6655d5fc2fb48580255a5021a81c379c325a457b74b77ac823ed67e4faa32aeb
2026-Feb-25 06:39:12.672109
#14 resolve docker.io/nvidia/cuda:12.3.2-devel-ubuntu22.04@sha256:6655d5fc2fb48580255a5021a81c379c325a457b74b77ac823ed67e4faa32aeb 0.0s done
2026-Feb-25 06:39:12.672109
#14 DONE 0.0s
2026-Feb-25 06:39:12.672109
2026-Feb-25 06:39:12.672109
#15 [gpu-worker runtime 1/6] FROM docker.io/nvidia/cuda:12.3.2-runtime-ubuntu22.04@sha256:882b43fadd789693f08de95e6014ed5f0ea118c7b342150876e153b4340e1103
2026-Feb-25 06:39:12.672109
#15 resolve docker.io/nvidia/cuda:12.3.2-runtime-ubuntu22.04@sha256:882b43fadd789693f08de95e6014ed5f0ea118c7b342150876e153b4340e1103 0.0s done
2026-Feb-25 06:39:12.672109
#15 DONE 0.0s
2026-Feb-25 06:39:12.672109
2026-Feb-25 06:39:12.672109
#16 [frontend internal] load build context
2026-Feb-25 06:39:12.674279
#16 transferring context: 197.18kB 0.0s done
2026-Feb-25 06:39:12.674279
#16 DONE 0.0s
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#17 [frontend builder 3/5] COPY frontend/ ./
2026-Feb-25 06:39:12.674279
#17 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#18 [frontend runtime 3/4] COPY --from=builder /app/build ./build
2026-Feb-25 06:39:12.674279
#18 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#19 [frontend builder 5/5] RUN node build-docker.mjs
2026-Feb-25 06:39:12.674279
#19 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#20 [frontend builder 4/5] RUN npm install
2026-Feb-25 06:39:12.674279
#20 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#21 [gpu-worker internal] load build context
2026-Feb-25 06:39:12.674279
#21 transferring context: 475.95kB 0.0s done
2026-Feb-25 06:39:12.674279
#21 DONE 0.0s
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#22 [frontend runtime 4/4] COPY --from=builder /app/package.json ./
2026-Feb-25 06:39:12.674279
#22 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#23 [api internal] load build context
2026-Feb-25 06:39:12.674279
#23 transferring context: 600.49kB 0.0s done
2026-Feb-25 06:39:12.674279
#23 DONE 0.0s
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#24 [gpu-worker builder 2/9] WORKDIR /app
2026-Feb-25 06:39:12.674279
#24 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#25 [gpu-worker builder 4/9] RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
2026-Feb-25 06:39:12.674279
#25 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#26 [gpu-worker builder 6/9] COPY Cargo.lock ./
2026-Feb-25 06:39:12.674279
#26 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#27 [gpu-worker builder 8/9] COPY proto/ ./proto/
2026-Feb-25 06:39:12.674279
#27 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#28 [gpu-worker runtime 5/6] COPY --from=builder /app/target/release/gpu-node /usr/local/bin/
2026-Feb-25 06:39:12.674279
#28 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#29 [gpu-worker runtime 3/6] RUN apt-get update && apt-get install -y     ca-certificates     libssl3     ffmpeg     libavcodec58     libavformat58     libavutil56     libswscale5     libavfilter7     libavdevice58     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 06:39:12.674279
#29 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#30 [gpu-worker builder 7/9] COPY crates/ ./crates/
2026-Feb-25 06:39:12.674279
#30 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#31 [gpu-worker builder 5/9] COPY Cargo.toml ./
2026-Feb-25 06:39:12.674279
#31 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#32 [gpu-worker builder 3/9] RUN apt-get update && apt-get install -y     curl     build-essential     pkg-config     libssl-dev     protobuf-compiler     ffmpeg     libavcodec-dev     libavformat-dev     libavutil-dev     libswscale-dev     libavfilter-dev     libavdevice-dev     clang     libclang-dev     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 06:39:12.674279
#32 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#33 [gpu-worker runtime 2/6] WORKDIR /app
2026-Feb-25 06:39:12.674279
#33 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#34 [gpu-worker runtime 4/6] RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-25 06:39:12.674279
#34 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#35 [gpu-worker builder 9/9] RUN cargo build --release --bin gpu-node
2026-Feb-25 06:39:12.674279
#35 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#36 [gpu-worker runtime 6/6] RUN mkdir -p /app/data && chown -R appuser:appuser /app
2026-Feb-25 06:39:12.674279
#36 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#37 [api builder 4/9] COPY Cargo.toml ./
2026-Feb-25 06:39:12.674279
#37 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#38 [api js-builder  7/10] COPY apps/injector/ ./apps/injector/
2026-Feb-25 06:39:12.674279
#38 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#39 [api js-builder  8/10] COPY packages/ ./packages/
2026-Feb-25 06:39:12.674279
#39 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#40 [api js-builder 10/10] RUN pnpm --filter @downloadtool/injector build
2026-Feb-25 06:39:12.674279
#40 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#41 [api builder 6/9] COPY crates/ ./crates/
2026-Feb-25 06:39:12.674279
#41 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#42 [api js-builder  5/10] COPY packages/api-client/package.json ./packages/api-client/
2026-Feb-25 06:39:12.674279
#42 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#43 [api js-builder  9/10] RUN pnpm install --frozen-lockfile
2026-Feb-25 06:39:12.674279
#43 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#44 [api builder 2/5] WORKDIR /app
2026-Feb-25 06:39:12.674279
#44 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#45 [api builder 9/9] RUN cargo build --release --bin vps-gateway
2026-Feb-25 06:39:12.674279
#45 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#46 [api js-builder  4/10] COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Feb-25 06:39:12.674279
#46 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#47 [api runtime 2/6] WORKDIR /app
2026-Feb-25 06:39:12.674279
#47 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#48 [api js-builder  3/10] RUN npm install -g pnpm
2026-Feb-25 06:39:12.674279
#48 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#49 [api builder 7/9] COPY proto/ ./proto/
2026-Feb-25 06:39:12.674279
#49 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#50 [api builder 2/9] WORKDIR /app
2026-Feb-25 06:39:12.674279
#50 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#51 [api runtime 4/6] RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-25 06:39:12.674279
#51 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#52 [api runtime 3/6] RUN apt-get update && apt-get install -y     ca-certificates     libssl3     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 06:39:12.674279
#52 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#53 [api runtime 5/6] COPY --from=builder /app/target/release/vps-gateway /usr/local/bin/
2026-Feb-25 06:39:12.674279
#53 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#54 [api builder 8/9] COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Feb-25 06:39:12.674279
#54 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#55 [api builder 5/9] COPY Cargo.lock ./
2026-Feb-25 06:39:12.674279
#55 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#56 [api js-builder  6/10] COPY apps/injector/package.json ./apps/injector/
2026-Feb-25 06:39:12.674279
#56 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#57 [api builder 3/9] RUN apt-get update && apt-get install -y     pkg-config     libssl-dev     protobuf-compiler     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 06:39:12.674279
#57 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#58 [api runtime 6/6] RUN mkdir -p /app/extractors /app/data && chown -R appuser:appuser /app
2026-Feb-25 06:39:12.674279
#58 CACHED
2026-Feb-25 06:39:12.674279
2026-Feb-25 06:39:12.674279
#59 [api] exporting to image
2026-Feb-25 06:39:12.674279
#59 exporting layers done
2026-Feb-25 06:39:12.674279
#59 exporting manifest sha256:0a0ae5d2098e4b00abb0e812bc4748e961169f7abb55a66cd0b535fc1ce46427 done
2026-Feb-25 06:39:12.674279
#59 exporting config sha256:2c114f9941c9130d877fd31bfcadc5bfb6ec636a2eae79ab950e049ee4b868ee done
2026-Feb-25 06:39:12.774353
#59 exporting attestation manifest sha256:5147eff3f9b702cbd7e1be345e691f0c6895b811bdc3cd99e1b98cf4724e99f7 0.0s done
2026-Feb-25 06:39:12.774353
#59 exporting manifest list sha256:a224c2472ba64f916bca038bc6e2b1fbe7a597373682458975eba282aa9a313e done
2026-Feb-25 06:39:12.774353
#59 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_api:d0f6612405c79d539ced04eb7e60601aa1f793a1 done
2026-Feb-25 06:39:12.774353
#59 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_api:d0f6612405c79d539ced04eb7e60601aa1f793a1 done
2026-Feb-25 06:39:12.774353
#59 DONE 0.1s
2026-Feb-25 06:39:12.774353
2026-Feb-25 06:39:12.774353
#60 [frontend] exporting to image
2026-Feb-25 06:39:12.774353
#60 exporting layers done
2026-Feb-25 06:39:12.774353
#60 exporting manifest sha256:2b7ac74b4236473bdbc9346f2b506de61f4d1f296befca629b0cf08b5862eebb done
2026-Feb-25 06:39:12.774353
#60 exporting config sha256:a96810395cbeedf365cde1af568c06aefbb063c22df12b82173f74e2eb9d64a7 done
2026-Feb-25 06:39:12.774353
#60 exporting attestation manifest sha256:384c485139ae56bbc2885e40919115862aa5e0fa03b149757f3aca268339bf3c 0.0s done
2026-Feb-25 06:39:12.774353
#60 exporting manifest list sha256:cae462a273cad090810d24321d8353ef56c647d2a06551bb8d0dafc4d7fa5080 0.0s done
2026-Feb-25 06:39:12.774353
#60 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:d0f6612405c79d539ced04eb7e60601aa1f793a1 done
2026-Feb-25 06:39:12.774353
#60 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:d0f6612405c79d539ced04eb7e60601aa1f793a1 done
2026-Feb-25 06:39:12.774353
#60 DONE 0.1s
2026-Feb-25 06:39:12.774353
2026-Feb-25 06:39:12.774353
#61 [gpu-worker] exporting to image
2026-Feb-25 06:39:12.774353
#61 exporting layers done
2026-Feb-25 06:39:12.774353
#61 exporting manifest sha256:073e01b76815a2f926d63cecc99892bb1d10b4325e87d13f4e1961e83c540d2c done
2026-Feb-25 06:39:12.774353
#61 exporting config sha256:512ecff06b1058051b92eb7bb364df2ef9135bd921d9c27c0f14c2ea35c1b896 done
2026-Feb-25 06:39:12.774353
#61 exporting attestation manifest sha256:eb2866cde74c34df3c8411a34f70a2f9a10d7e1a41b1b4cf8f3ba9b97ade3884 0.0s done
2026-Feb-25 06:39:12.774353
#61 exporting manifest list sha256:afae194f3f0d0913def8e849ab4c28cd3164c3410ee4a17f23d51127cb369823 done
2026-Feb-25 06:39:12.774353
#61 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_gpu-worker:d0f6612405c79d539ced04eb7e60601aa1f793a1 done
2026-Feb-25 06:39:12.774353
#61 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_gpu-worker:d0f6612405c79d539ced04eb7e60601aa1f793a1 0.0s done
2026-Feb-25 06:39:12.774353
#61 DONE 0.1s
2026-Feb-25 06:39:12.774353
2026-Feb-25 06:39:12.774353
#62 [api] resolving provenance for metadata file
2026-Feb-25 06:39:12.804588
#62 DONE 0.0s
2026-Feb-25 06:39:12.804588
2026-Feb-25 06:39:12.804588
#63 [frontend] resolving provenance for metadata file
2026-Feb-25 06:39:12.804588
#63 DONE 0.0s
2026-Feb-25 06:39:12.804588
2026-Feb-25 06:39:12.804588
#64 [gpu-worker] resolving provenance for metadata file
2026-Feb-25 06:39:12.804588
#64 DONE 0.0s
2026-Feb-25 06:39:12.806842
api  Built
2026-Feb-25 06:39:12.806842
frontend  Built
2026-Feb-25 06:39:12.806842
gpu-worker  Built
2026-Feb-25 06:39:12.818328
Creating .env file with runtime variables for container.
2026-Feb-25 06:39:13.476928
Removing old containers.
2026-Feb-25 06:39:14.378486
[CMD]: docker stop -t 30 frontend-o8kccgkgwsockoocow8sg88s-050839857720
2026-Feb-25 06:39:14.378486
frontend-o8kccgkgwsockoocow8sg88s-050839857720
2026-Feb-25 06:39:14.703444
[CMD]: docker rm -f frontend-o8kccgkgwsockoocow8sg88s-050839857720
2026-Feb-25 06:39:14.703444
frontend-o8kccgkgwsockoocow8sg88s-050839857720
2026-Feb-25 06:39:45.267530
[CMD]: docker stop -t 30 api-o8kccgkgwsockoocow8sg88s-050839855763
2026-Feb-25 06:39:45.267530
api-o8kccgkgwsockoocow8sg88s-050839855763
2026-Feb-25 06:39:45.595827
[CMD]: docker rm -f api-o8kccgkgwsockoocow8sg88s-050839855763
2026-Feb-25 06:39:45.595827
api-o8kccgkgwsockoocow8sg88s-050839855763
2026-Feb-25 06:40:16.395366
[CMD]: docker stop -t 30 gpu-worker-o8kccgkgwsockoocow8sg88s-050839852837
2026-Feb-25 06:40:16.395366
gpu-worker-o8kccgkgwsockoocow8sg88s-050839852837
2026-Feb-25 06:40:16.727571
[CMD]: docker rm -f gpu-worker-o8kccgkgwsockoocow8sg88s-050839852837
2026-Feb-25 06:40:16.727571
gpu-worker-o8kccgkgwsockoocow8sg88s-050839852837
2026-Feb-25 06:40:16.730554
Starting new application.
2026-Feb-25 06:40:17.812168
[CMD]: docker exec hswgk0sgs0cc8gswk48gwgow bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/hswgk0sgs0cc8gswk48gwgow/.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/hswgk0sgs0cc8gswk48gwgow -f /artifacts/hswgk0sgs0cc8gswk48gwgow/docker/docker-compose.homeserver.yml up -d'
2026-Feb-25 06:40:17.812168
Container gpu-worker-o8kccgkgwsockoocow8sg88s-063906670322  Creating
2026-Feb-25 06:40:17.850828
Container gpu-worker-o8kccgkgwsockoocow8sg88s-063906670322  Created
2026-Feb-25 06:40:17.850828
Container api-o8kccgkgwsockoocow8sg88s-063906672830  Creating
2026-Feb-25 06:40:18.099159
Container api-o8kccgkgwsockoocow8sg88s-063906672830  Created
2026-Feb-25 06:40:18.099159
Container frontend-o8kccgkgwsockoocow8sg88s-063906674703  Creating
2026-Feb-25 06:40:18.127364
Container frontend-o8kccgkgwsockoocow8sg88s-063906674703  Created
2026-Feb-25 06:40:18.135461
Container gpu-worker-o8kccgkgwsockoocow8sg88s-063906670322  Starting
2026-Feb-25 06:40:18.361081
Container gpu-worker-o8kccgkgwsockoocow8sg88s-063906670322  Started
2026-Feb-25 06:40:18.361081
Container api-o8kccgkgwsockoocow8sg88s-063906672830  Starting
2026-Feb-25 06:40:18.519214
Container api-o8kccgkgwsockoocow8sg88s-063906672830  Started
2026-Feb-25 06:40:18.519214
Container frontend-o8kccgkgwsockoocow8sg88s-063906674703  Starting
2026-Feb-25 06:40:18.683616
Container frontend-o8kccgkgwsockoocow8sg88s-063906674703  Started
2026-Feb-25 06:40:19.596978
New container started.