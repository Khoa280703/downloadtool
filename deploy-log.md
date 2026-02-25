2026-Feb-25 05:08:30.026039
Starting deployment of khoa280703/downloadtool:main-zoscg4oc04gkwkssg0kw8w8w to localhost.
2026-Feb-25 05:08:30.594644
Preparing container with helper image: ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Feb-25 05:08:30.910033
[CMD]: docker stop -t 30 bwo0440og4kooocc8kccc4sg
2026-Feb-25 05:08:30.910033
Error response from daemon: No such container: bwo0440og4kooocc8kccc4sg
2026-Feb-25 05:08:31.385728
[CMD]: docker run -d --network coolify --name bwo0440og4kooocc8kccc4sg  --rm -v /var/run/docker.sock:/var/run/docker.sock ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Feb-25 05:08:31.385728
ad11bc28c5fd711d2f8f286ee89102ae268d805e6f5d28e2cfc34844059c7aad
2026-Feb-25 05:08:33.036895
[CMD]: docker exec bwo0440og4kooocc8kccc4sg bash -c 'GIT_SSH_COMMAND="ssh -o ConnectTimeout=30 -p 22 -o Port=22 -o LogLevel=ERROR -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git ls-remote https://github.com/Khoa280703/downloadtool refs/heads/main'
2026-Feb-25 05:08:33.036895
4964f9a4405b338e6420a0e7d46a1e71ddfa1598	refs/heads/main
2026-Feb-25 05:08:33.047043
----------------------------------------
2026-Feb-25 05:08:33.051624
Importing Khoa280703/downloadtool:main (commit sha 4964f9a4405b338e6420a0e7d46a1e71ddfa1598) to /artifacts/bwo0440og4kooocc8kccc4sg.
2026-Feb-25 05:08:33.403803
[CMD]: docker exec bwo0440og4kooocc8kccc4sg bash -c 'git clone --depth=1 --recurse-submodules --shallow-submodules -b 'main' 'https://github.com/Khoa280703/downloadtool' '/artifacts/bwo0440og4kooocc8kccc4sg' && cd '/artifacts/bwo0440og4kooocc8kccc4sg' && if [ -f .gitmodules ]; then sed -i "s#git@\(.*\):#https://\1/#g" '/artifacts/bwo0440og4kooocc8kccc4sg'/.gitmodules || true && git submodule sync && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git submodule update --init --recursive --depth=1; fi && cd '/artifacts/bwo0440og4kooocc8kccc4sg' && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git lfs pull'
2026-Feb-25 05:08:33.403803
Cloning into '/artifacts/bwo0440og4kooocc8kccc4sg'...
2026-Feb-25 05:08:35.945484
[CMD]: docker exec bwo0440og4kooocc8kccc4sg bash -c 'cd /artifacts/bwo0440og4kooocc8kccc4sg && git log -1 4964f9a4405b338e6420a0e7d46a1e71ddfa1598 --pretty=%B'
2026-Feb-25 05:08:35.945484
fix: build gpu-node without --features gpu (FFmpeg 7.x not available on Ubuntu 22.04)
2026-Feb-25 05:08:40.595880
[CMD]: docker exec bwo0440og4kooocc8kccc4sg bash -c 'test -f /artifacts/bwo0440og4kooocc8kccc4sg/docker/Dockerfile.homeserver && echo 'exists' || echo 'not found''
2026-Feb-25 05:08:40.595880
exists
2026-Feb-25 05:08:40.964932
[CMD]: docker exec bwo0440og4kooocc8kccc4sg bash -c 'cat /artifacts/bwo0440og4kooocc8kccc4sg/docker/Dockerfile.homeserver'
2026-Feb-25 05:08:40.964932
# Dockerfile for Home Server deployment
2026-Feb-25 05:08:40.964932
# Builds the GPU worker with CUDA support for hardware transcoding
2026-Feb-25 05:08:40.964932
2026-Feb-25 05:08:40.964932
FROM nvidia/cuda:12.3.2-devel-ubuntu22.04 AS builder
2026-Feb-25 05:08:40.964932
2026-Feb-25 05:08:40.964932
WORKDIR /app
2026-Feb-25 05:08:40.964932
2026-Feb-25 05:08:40.964932
# Install dependencies
2026-Feb-25 05:08:40.964932
RUN apt-get update && apt-get install -y \
2026-Feb-25 05:08:40.964932
curl \
2026-Feb-25 05:08:40.964932
build-essential \
2026-Feb-25 05:08:40.964932
pkg-config \
2026-Feb-25 05:08:40.964932
libssl-dev \
2026-Feb-25 05:08:40.964932
protobuf-compiler \
2026-Feb-25 05:08:40.964932
ffmpeg \
2026-Feb-25 05:08:40.964932
libavcodec-dev \
2026-Feb-25 05:08:40.964932
libavformat-dev \
2026-Feb-25 05:08:40.964932
libavutil-dev \
2026-Feb-25 05:08:40.964932
libswscale-dev \
2026-Feb-25 05:08:40.964932
libavfilter-dev \
2026-Feb-25 05:08:40.964932
libavdevice-dev \
2026-Feb-25 05:08:40.964932
clang \
2026-Feb-25 05:08:40.964932
libclang-dev \
2026-Feb-25 05:08:40.964932
&& rm -rf /var/lib/apt/lists/*
2026-Feb-25 05:08:40.964932
2026-Feb-25 05:08:40.964932
# Install Rust
2026-Feb-25 05:08:40.964932
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
2026-Feb-25 05:08:40.964932
ENV PATH="/root/.cargo/bin:${PATH}"
2026-Feb-25 05:08:40.964932
2026-Feb-25 05:08:40.964932
# Copy workspace configuration
2026-Feb-25 05:08:40.964932
COPY Cargo.toml ./
2026-Feb-25 05:08:40.964932
COPY Cargo.lock ./
2026-Feb-25 05:08:40.964932
2026-Feb-25 05:08:40.964932
# Copy all crates
2026-Feb-25 05:08:40.964932
COPY crates/ ./crates/
2026-Feb-25 05:08:40.964932
2026-Feb-25 05:08:40.964932
# Copy proto files
2026-Feb-25 05:08:40.964932
COPY proto/ ./proto/
2026-Feb-25 05:08:40.964932
2026-Feb-25 05:08:40.964932
# Build the GPU worker with CUDA support
2026-Feb-25 05:08:40.964932
# NOTE: --features gpu disabled until FFmpeg 7.x is available in the build environment
2026-Feb-25 05:08:40.964932
# Ubuntu 22.04 apt provides FFmpeg 4.4.x but ffmpeg-next = "7" requires FFmpeg 7.x
2026-Feb-25 05:08:40.964932
RUN cargo build --release --bin gpu-node
2026-Feb-25 05:08:40.964932
2026-Feb-25 05:08:40.964932
# Stage 2: Runtime
2026-Feb-25 05:08:40.964932
FROM nvidia/cuda:12.3.2-runtime-ubuntu22.04 AS runtime
2026-Feb-25 05:08:40.964932
2026-Feb-25 05:08:40.964932
WORKDIR /app
2026-Feb-25 05:08:40.964932
2026-Feb-25 05:08:40.964932
# Install runtime dependencies
2026-Feb-25 05:08:40.964932
RUN apt-get update && apt-get install -y \
2026-Feb-25 05:08:40.964932
ca-certificates \
2026-Feb-25 05:08:40.964932
libssl3 \
2026-Feb-25 05:08:40.964932
ffmpeg \
2026-Feb-25 05:08:40.964932
libavcodec58 \
2026-Feb-25 05:08:40.964932
libavformat58 \
2026-Feb-25 05:08:40.964932
libavutil56 \
2026-Feb-25 05:08:40.964932
libswscale5 \
2026-Feb-25 05:08:40.964932
libavfilter7 \
2026-Feb-25 05:08:40.964932
libavdevice58 \
2026-Feb-25 05:08:40.964932
&& rm -rf /var/lib/apt/lists/*
2026-Feb-25 05:08:40.964932
2026-Feb-25 05:08:40.964932
# Create non-root user
2026-Feb-25 05:08:40.964932
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-25 05:08:40.964932
2026-Feb-25 05:08:40.964932
# Copy binary from builder
2026-Feb-25 05:08:40.964932
COPY --from=builder /app/target/release/gpu-node /usr/local/bin/
2026-Feb-25 05:08:40.964932
2026-Feb-25 05:08:40.964932
# Create directories
2026-Feb-25 05:08:40.964932
RUN mkdir -p /app/data && chown -R appuser:appuser /app
2026-Feb-25 05:08:40.964932
2026-Feb-25 05:08:40.964932
# Switch to non-root user
2026-Feb-25 05:08:40.964932
USER appuser
2026-Feb-25 05:08:40.964932
2026-Feb-25 05:08:40.964932
# Environment variables
2026-Feb-25 05:08:40.964932
ENV GPU_WORKER_BIND=0.0.0.0:50051
2026-Feb-25 05:08:40.964932
ENV GPU_WORKER_MAX_JOBS=4
2026-Feb-25 05:08:40.964932
ENV CUDA_DEVICE_ID=0
2026-Feb-25 05:08:40.964932
ENV RUST_LOG=info
2026-Feb-25 05:08:40.964932
2026-Feb-25 05:08:40.964932
# NVIDIA runtime configuration
2026-Feb-25 05:08:40.964932
ENV NVIDIA_VISIBLE_DEVICES=all
2026-Feb-25 05:08:40.964932
ENV NVIDIA_DRIVER_CAPABILITIES=compute,video,utility
2026-Feb-25 05:08:40.964932
2026-Feb-25 05:08:40.964932
# Expose gRPC port
2026-Feb-25 05:08:40.964932
EXPOSE 50051
2026-Feb-25 05:08:40.964932
2026-Feb-25 05:08:40.964932
# Health check
2026-Feb-25 05:08:40.964932
HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
2026-Feb-25 05:08:40.964932
CMD echo "health check placeholder" || exit 0
2026-Feb-25 05:08:40.964932
2026-Feb-25 05:08:40.964932
# Run the GPU worker
2026-Feb-25 05:08:40.964932
CMD ["gpu-node"]
2026-Feb-25 05:08:41.331256
Added 4 ARG declarations to Dockerfile for service gpu-worker (multi-stage build, added to 2 stages).
2026-Feb-25 05:08:41.678312
[CMD]: docker exec bwo0440og4kooocc8kccc4sg bash -c 'test -f /artifacts/bwo0440og4kooocc8kccc4sg/docker/Dockerfile.vps && echo 'exists' || echo 'not found''
2026-Feb-25 05:08:41.678312
exists
2026-Feb-25 05:08:42.051161
[CMD]: docker exec bwo0440og4kooocc8kccc4sg bash -c 'cat /artifacts/bwo0440og4kooocc8kccc4sg/docker/Dockerfile.vps'
2026-Feb-25 05:08:42.051161
# Dockerfile for VPS deployment
2026-Feb-25 05:08:42.051161
# Builds the API server and related components without GPU support
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
# Stage 0: Build injector JS (embedded into api crate via include_str! at compile time)
2026-Feb-25 05:08:42.051161
FROM node:22-alpine AS js-builder
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
WORKDIR /app
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
RUN npm install -g pnpm
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
# Copy workspace manifests for pnpm resolution
2026-Feb-25 05:08:42.051161
COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Feb-25 05:08:42.051161
COPY packages/api-client/package.json ./packages/api-client/
2026-Feb-25 05:08:42.051161
COPY apps/injector/package.json ./apps/injector/
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
# Copy injector source and shared packages
2026-Feb-25 05:08:42.051161
COPY apps/injector/ ./apps/injector/
2026-Feb-25 05:08:42.051161
COPY packages/ ./packages/
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
# Install deps and build injector (produces dist/bm.js and dist/youtube-downloader.user.js)
2026-Feb-25 05:08:42.051161
RUN pnpm install --frozen-lockfile
2026-Feb-25 05:08:42.051161
RUN pnpm --filter @downloadtool/injector build
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
# Stage 1: Rust builder
2026-Feb-25 05:08:42.051161
FROM rust:1.88-bookworm AS builder
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
WORKDIR /app
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
# Install dependencies
2026-Feb-25 05:08:42.051161
RUN apt-get update && apt-get install -y \
2026-Feb-25 05:08:42.051161
pkg-config \
2026-Feb-25 05:08:42.051161
libssl-dev \
2026-Feb-25 05:08:42.051161
protobuf-compiler \
2026-Feb-25 05:08:42.051161
&& rm -rf /var/lib/apt/lists/*
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
# Copy workspace configuration
2026-Feb-25 05:08:42.051161
COPY Cargo.toml ./
2026-Feb-25 05:08:42.051161
COPY Cargo.lock ./
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
# Copy all crates
2026-Feb-25 05:08:42.051161
COPY crates/ ./crates/
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
# Copy proto files
2026-Feb-25 05:08:42.051161
COPY proto/ ./proto/
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
# Copy injector dist (required by include_str! in crates/api/src/routes/static_files.rs)
2026-Feb-25 05:08:42.051161
COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
# Build the release binary
2026-Feb-25 05:08:42.051161
RUN cargo build --release --bin vps-gateway
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
# Stage 2: Runtime
2026-Feb-25 05:08:42.051161
FROM debian:bookworm-slim AS runtime
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
WORKDIR /app
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
# Install runtime dependencies
2026-Feb-25 05:08:42.051161
RUN apt-get update && apt-get install -y \
2026-Feb-25 05:08:42.051161
ca-certificates \
2026-Feb-25 05:08:42.051161
libssl3 \
2026-Feb-25 05:08:42.051161
&& rm -rf /var/lib/apt/lists/*
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
# Create non-root user
2026-Feb-25 05:08:42.051161
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
# Copy binary from builder
2026-Feb-25 05:08:42.051161
COPY --from=builder /app/target/release/vps-gateway /usr/local/bin/
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
# Create directories
2026-Feb-25 05:08:42.051161
RUN mkdir -p /app/extractors /app/data && chown -R appuser:appuser /app
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
# Switch to non-root user
2026-Feb-25 05:08:42.051161
USER appuser
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
# Environment variables
2026-Feb-25 05:08:42.051161
ENV PORT=3068
2026-Feb-25 05:08:42.051161
ENV EXTRACTOR_DIR=/app/extractors
2026-Feb-25 05:08:42.051161
ENV GPU_ENABLED=false
2026-Feb-25 05:08:42.051161
ENV RUST_LOG=info
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
# Expose port
2026-Feb-25 05:08:42.051161
EXPOSE 3068
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
# Health check
2026-Feb-25 05:08:42.051161
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Feb-25 05:08:42.051161
CMD curl -f http://localhost:3068/health || exit 1
2026-Feb-25 05:08:42.051161
2026-Feb-25 05:08:42.051161
# Run the server
2026-Feb-25 05:08:42.051161
CMD ["vps-gateway"]
2026-Feb-25 05:08:42.410144
Added 6 ARG declarations to Dockerfile for service api (multi-stage build, added to 3 stages).
2026-Feb-25 05:08:42.773364
[CMD]: docker exec bwo0440og4kooocc8kccc4sg bash -c 'test -f /artifacts/bwo0440og4kooocc8kccc4sg/docker/Dockerfile.frontend && echo 'exists' || echo 'not found''
2026-Feb-25 05:08:42.773364
exists
2026-Feb-25 05:08:43.124664
[CMD]: docker exec bwo0440og4kooocc8kccc4sg bash -c 'cat /artifacts/bwo0440og4kooocc8kccc4sg/docker/Dockerfile.frontend'
2026-Feb-25 05:08:43.124664
# Dockerfile for frontend (SvelteKit Node server)
2026-Feb-25 05:08:43.124664
# Copy ALL source files BEFORE npm install so svelte-kit sync (prepare script)
2026-Feb-25 05:08:43.124664
# can find svelte.config.js and generate .svelte-kit/ correctly.
2026-Feb-25 05:08:43.124664
2026-Feb-25 05:08:43.124664
FROM node:22-alpine AS builder
2026-Feb-25 05:08:43.124664
2026-Feb-25 05:08:43.124664
WORKDIR /app
2026-Feb-25 05:08:43.124664
2026-Feb-25 05:08:43.124664
# Copy all frontend source files first (node_modules excluded via .dockerignore)
2026-Feb-25 05:08:43.124664
COPY frontend/ ./
2026-Feb-25 05:08:43.124664
2026-Feb-25 05:08:43.124664
# Install — prepare script runs svelte-kit sync with svelte.config.js available
2026-Feb-25 05:08:43.124664
RUN npm install
2026-Feb-25 05:08:43.124664
2026-Feb-25 05:08:43.124664
# Build via programmatic API — bypasses vite.config.ts loading (esbuild issues on Alpine)
2026-Feb-25 05:08:43.124664
RUN node build-docker.mjs
2026-Feb-25 05:08:43.124664
2026-Feb-25 05:08:43.124664
# Runtime
2026-Feb-25 05:08:43.124664
FROM node:22-alpine AS runtime
2026-Feb-25 05:08:43.124664
2026-Feb-25 05:08:43.124664
WORKDIR /app
2026-Feb-25 05:08:43.124664
2026-Feb-25 05:08:43.124664
COPY --from=builder /app/build ./build
2026-Feb-25 05:08:43.124664
COPY --from=builder /app/package.json ./
2026-Feb-25 05:08:43.124664
2026-Feb-25 05:08:43.124664
ENV PORT=3000
2026-Feb-25 05:08:43.124664
ENV HOST=0.0.0.0
2026-Feb-25 05:08:43.124664
2026-Feb-25 05:08:43.124664
EXPOSE 3000
2026-Feb-25 05:08:43.124664
2026-Feb-25 05:08:43.124664
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Feb-25 05:08:43.124664
CMD wget -qO- http://localhost:3000 || exit 1
2026-Feb-25 05:08:43.124664
2026-Feb-25 05:08:43.124664
CMD ["node", "build"]
2026-Feb-25 05:08:43.496703
Added 4 ARG declarations to Dockerfile for service frontend (multi-stage build, added to 2 stages).
2026-Feb-25 05:08:43.501574
Pulling & building required images.
2026-Feb-25 05:08:43.506856
Creating build-time .env file in /artifacts (outside Docker context).
2026-Feb-25 05:08:43.873615
Adding build arguments to Docker Compose build command.
2026-Feb-25 05:08:44.359324
[CMD]: docker exec bwo0440og4kooocc8kccc4sg bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/build-time.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/bwo0440og4kooocc8kccc4sg -f /artifacts/bwo0440og4kooocc8kccc4sg/docker/docker-compose.homeserver.yml build --pull --build-arg COOLIFY_URL --build-arg COOLIFY_FQDN --build-arg COOLIFY_BUILD_SECRETS_HASH=1bf8dfcfdeb73162b126768f9e085efbcfdffa61f0529746b67826c254109afc'
2026-Feb-25 05:08:44.359324
#1 [internal] load local bake definitions
2026-Feb-25 05:08:44.570721
#1 reading from stdin 1.79kB done
2026-Feb-25 05:08:44.570721
#1 DONE 0.0s
2026-Feb-25 05:08:44.570721
2026-Feb-25 05:08:44.570721
#2 [frontend internal] load build definition from Dockerfile.frontend
2026-Feb-25 05:08:44.570721
#2 transferring dockerfile: 999B done
2026-Feb-25 05:08:44.570721
#2 DONE 0.0s
2026-Feb-25 05:08:44.570721
2026-Feb-25 05:08:44.570721
#3 [api internal] load build definition from Dockerfile.vps
2026-Feb-25 05:08:44.570721
#3 transferring dockerfile: 2.35kB done
2026-Feb-25 05:08:44.570721
#3 DONE 0.0s
2026-Feb-25 05:08:44.570721
2026-Feb-25 05:08:44.570721
#4 [gpu-worker internal] load build definition from Dockerfile.homeserver
2026-Feb-25 05:08:44.570721
#4 transferring dockerfile: 2.28kB done
2026-Feb-25 05:08:44.570721
#4 DONE 0.0s
2026-Feb-25 05:08:44.570721
2026-Feb-25 05:08:44.570721
#5 [api internal] load metadata for docker.io/library/node:22-alpine
2026-Feb-25 05:08:45.751841
#5 ...
2026-Feb-25 05:08:45.751841
2026-Feb-25 05:08:45.751841
#6 [api internal] load metadata for docker.io/library/debian:bookworm-slim
2026-Feb-25 05:08:45.751841
#6 DONE 1.3s
2026-Feb-25 05:08:45.852279
#7 [api internal] load metadata for docker.io/library/rust:1.88-bookworm
2026-Feb-25 05:08:45.852279
#7 DONE 1.3s
2026-Feb-25 05:08:45.852279
2026-Feb-25 05:08:45.852279
#5 [api internal] load metadata for docker.io/library/node:22-alpine
2026-Feb-25 05:08:45.852279
#5 DONE 1.3s
2026-Feb-25 05:08:45.852279
2026-Feb-25 05:08:45.852279
#8 [gpu-worker internal] load metadata for docker.io/nvidia/cuda:12.3.2-runtime-ubuntu22.04
2026-Feb-25 05:08:45.852279
#8 DONE 1.3s
2026-Feb-25 05:08:45.852279
2026-Feb-25 05:08:45.852279
#9 [gpu-worker internal] load metadata for docker.io/nvidia/cuda:12.3.2-devel-ubuntu22.04
2026-Feb-25 05:08:45.852279
#9 DONE 1.3s
2026-Feb-25 05:08:45.852279
2026-Feb-25 05:08:45.852279
#10 [frontend internal] load .dockerignore
2026-Feb-25 05:08:45.852279
#10 transferring context: 341B done
2026-Feb-25 05:08:45.852279
#10 DONE 0.0s
2026-Feb-25 05:08:45.852279
2026-Feb-25 05:08:45.852279
#11 [frontend builder 1/5] FROM docker.io/library/node:22-alpine@sha256:e4bf2a82ad0a4037d28035ae71529873c069b13eb0455466ae0bc13363826e34
2026-Feb-25 05:08:45.852279
#11 resolve docker.io/library/node:22-alpine@sha256:e4bf2a82ad0a4037d28035ae71529873c069b13eb0455466ae0bc13363826e34 0.0s done
2026-Feb-25 05:08:45.852279
#11 DONE 0.0s
2026-Feb-25 05:08:45.852279
2026-Feb-25 05:08:45.852279
#12 [gpu-worker builder 1/9] FROM docker.io/nvidia/cuda:12.3.2-devel-ubuntu22.04@sha256:6655d5fc2fb48580255a5021a81c379c325a457b74b77ac823ed67e4faa32aeb
2026-Feb-25 05:08:45.852279
#12 resolve docker.io/nvidia/cuda:12.3.2-devel-ubuntu22.04@sha256:6655d5fc2fb48580255a5021a81c379c325a457b74b77ac823ed67e4faa32aeb 0.0s done
2026-Feb-25 05:08:45.852279
#12 DONE 0.0s
2026-Feb-25 05:08:45.852279
2026-Feb-25 05:08:45.852279
#13 [gpu-worker runtime 1/6] FROM docker.io/nvidia/cuda:12.3.2-runtime-ubuntu22.04@sha256:882b43fadd789693f08de95e6014ed5f0ea118c7b342150876e153b4340e1103
2026-Feb-25 05:08:45.852279
#13 resolve docker.io/nvidia/cuda:12.3.2-runtime-ubuntu22.04@sha256:882b43fadd789693f08de95e6014ed5f0ea118c7b342150876e153b4340e1103 0.0s done
2026-Feb-25 05:08:45.852279
#13 DONE 0.0s
2026-Feb-25 05:08:45.852279
2026-Feb-25 05:08:45.852279
#14 [api runtime 1/6] FROM docker.io/library/debian:bookworm-slim@sha256:74d56e3931e0d5a1dd51f8c8a2466d21de84a271cd3b5a733b803aa91abf4421
2026-Feb-25 05:08:45.852279
#14 resolve docker.io/library/debian:bookworm-slim@sha256:74d56e3931e0d5a1dd51f8c8a2466d21de84a271cd3b5a733b803aa91abf4421 0.0s done
2026-Feb-25 05:08:45.852279
#14 DONE 0.0s
2026-Feb-25 05:08:45.852279
2026-Feb-25 05:08:45.852279
#15 [api builder 1/9] FROM docker.io/library/rust:1.88-bookworm@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0
2026-Feb-25 05:08:45.852279
#15 resolve docker.io/library/rust:1.88-bookworm@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0 0.0s done
2026-Feb-25 05:08:45.852279
#15 DONE 0.0s
2026-Feb-25 05:08:45.852279
2026-Feb-25 05:08:45.852279
#16 [frontend internal] load build context
2026-Feb-25 05:08:45.852279
#16 transferring context: 197.18kB 0.0s done
2026-Feb-25 05:08:45.852279
#16 DONE 0.0s
2026-Feb-25 05:08:45.852279
2026-Feb-25 05:08:45.852279
#17 [frontend builder 4/5] RUN npm install
2026-Feb-25 05:08:45.852279
#17 CACHED
2026-Feb-25 05:08:45.852279
2026-Feb-25 05:08:45.852279
#18 [frontend runtime 3/4] COPY --from=builder /app/build ./build
2026-Feb-25 05:08:45.852279
#18 CACHED
2026-Feb-25 05:08:45.852279
2026-Feb-25 05:08:45.852279
#19 [frontend builder 3/5] COPY frontend/ ./
2026-Feb-25 05:08:45.852279
#19 CACHED
2026-Feb-25 05:08:45.852279
2026-Feb-25 05:08:45.852279
#20 [frontend builder 5/5] RUN node build-docker.mjs
2026-Feb-25 05:08:45.852279
#20 CACHED
2026-Feb-25 05:08:45.852279
2026-Feb-25 05:08:45.852279
#21 [frontend runtime 4/4] COPY --from=builder /app/package.json ./
2026-Feb-25 05:08:45.852279
#21 CACHED
2026-Feb-25 05:08:45.852279
2026-Feb-25 05:08:45.852279
#22 [gpu-worker internal] load build context
2026-Feb-25 05:08:45.852279
#22 transferring context: 475.95kB 0.0s done
2026-Feb-25 05:08:45.852279
#22 DONE 0.0s
2026-Feb-25 05:08:45.852279
2026-Feb-25 05:08:45.852279
#23 [api internal] load build context
2026-Feb-25 05:08:45.852279
#23 transferring context: 600.49kB 0.0s done
2026-Feb-25 05:08:45.852279
#23 DONE 0.0s
2026-Feb-25 05:08:45.852279
2026-Feb-25 05:08:45.852279
#24 [gpu-worker builder 9/9] RUN cargo build --release --bin gpu-node
2026-Feb-25 05:08:45.852279
#24 CACHED
2026-Feb-25 05:08:45.852279
2026-Feb-25 05:08:45.852279
#25 [gpu-worker builder 2/9] WORKDIR /app
2026-Feb-25 05:08:45.852279
#25 CACHED
2026-Feb-25 05:08:45.852279
2026-Feb-25 05:08:45.852279
#26 [gpu-worker runtime 2/6] WORKDIR /app
2026-Feb-25 05:08:45.852279
#26 CACHED
2026-Feb-25 05:08:45.854421
#27 [gpu-worker runtime 4/6] RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-25 05:08:45.854421
#27 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#28 [gpu-worker builder 8/9] COPY proto/ ./proto/
2026-Feb-25 05:08:45.854421
#28 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#29 [gpu-worker builder 5/9] COPY Cargo.toml ./
2026-Feb-25 05:08:45.854421
#29 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#30 [gpu-worker runtime 5/6] COPY --from=builder /app/target/release/gpu-node /usr/local/bin/
2026-Feb-25 05:08:45.854421
#30 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#31 [gpu-worker runtime 3/6] RUN apt-get update && apt-get install -y     ca-certificates     libssl3     ffmpeg     libavcodec58     libavformat58     libavutil56     libswscale5     libavfilter7     libavdevice58     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 05:08:45.854421
#31 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#32 [gpu-worker builder 3/9] RUN apt-get update && apt-get install -y     curl     build-essential     pkg-config     libssl-dev     protobuf-compiler     ffmpeg     libavcodec-dev     libavformat-dev     libavutil-dev     libswscale-dev     libavfilter-dev     libavdevice-dev     clang     libclang-dev     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 05:08:45.854421
#32 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#33 [gpu-worker builder 6/9] COPY Cargo.lock ./
2026-Feb-25 05:08:45.854421
#33 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#34 [gpu-worker builder 4/9] RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
2026-Feb-25 05:08:45.854421
#34 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#35 [gpu-worker builder 7/9] COPY crates/ ./crates/
2026-Feb-25 05:08:45.854421
#35 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#36 [gpu-worker runtime 6/6] RUN mkdir -p /app/data && chown -R appuser:appuser /app
2026-Feb-25 05:08:45.854421
#36 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#37 [api js-builder 10/10] RUN pnpm --filter @downloadtool/injector build
2026-Feb-25 05:08:45.854421
#37 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#38 [api js-builder  8/10] COPY packages/ ./packages/
2026-Feb-25 05:08:45.854421
#38 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#39 [api builder 2/9] WORKDIR /app
2026-Feb-25 05:08:45.854421
#39 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#40 [api builder 4/9] COPY Cargo.toml ./
2026-Feb-25 05:08:45.854421
#40 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#41 [api js-builder  5/10] COPY packages/api-client/package.json ./packages/api-client/
2026-Feb-25 05:08:45.854421
#41 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#42 [api js-builder  3/10] RUN npm install -g pnpm
2026-Feb-25 05:08:45.854421
#42 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#43 [api builder 3/9] RUN apt-get update && apt-get install -y     pkg-config     libssl-dev     protobuf-compiler     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 05:08:45.854421
#43 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#44 [api js-builder  7/10] COPY apps/injector/ ./apps/injector/
2026-Feb-25 05:08:45.854421
#44 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#45 [api runtime 4/6] RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-25 05:08:45.854421
#45 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#46 [api js-builder  4/10] COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Feb-25 05:08:45.854421
#46 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#47 [api builder 7/9] COPY proto/ ./proto/
2026-Feb-25 05:08:45.854421
#47 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#48 [api builder 2/5] WORKDIR /app
2026-Feb-25 05:08:45.854421
#48 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#49 [api runtime 3/6] RUN apt-get update && apt-get install -y     ca-certificates     libssl3     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 05:08:45.854421
#49 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#50 [api builder 9/9] RUN cargo build --release --bin vps-gateway
2026-Feb-25 05:08:45.854421
#50 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#51 [api builder 8/9] COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Feb-25 05:08:45.854421
#51 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#52 [api builder 5/9] COPY Cargo.lock ./
2026-Feb-25 05:08:45.854421
#52 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#53 [api builder 6/9] COPY crates/ ./crates/
2026-Feb-25 05:08:45.854421
#53 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#54 [api js-builder  9/10] RUN pnpm install --frozen-lockfile
2026-Feb-25 05:08:45.854421
#54 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#55 [api runtime 2/6] WORKDIR /app
2026-Feb-25 05:08:45.854421
#55 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#56 [api js-builder  6/10] COPY apps/injector/package.json ./apps/injector/
2026-Feb-25 05:08:45.854421
#56 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#57 [api runtime 5/6] COPY --from=builder /app/target/release/vps-gateway /usr/local/bin/
2026-Feb-25 05:08:45.854421
#57 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#58 [api runtime 6/6] RUN mkdir -p /app/extractors /app/data && chown -R appuser:appuser /app
2026-Feb-25 05:08:45.854421
#58 CACHED
2026-Feb-25 05:08:45.854421
2026-Feb-25 05:08:45.854421
#59 [api] exporting to image
2026-Feb-25 05:08:45.955381
#59 exporting layers done
2026-Feb-25 05:08:45.955381
#59 exporting manifest sha256:0a0ae5d2098e4b00abb0e812bc4748e961169f7abb55a66cd0b535fc1ce46427 done
2026-Feb-25 05:08:45.955381
#59 exporting config sha256:2c114f9941c9130d877fd31bfcadc5bfb6ec636a2eae79ab950e049ee4b868ee done
2026-Feb-25 05:08:45.955381
#59 exporting attestation manifest sha256:7a6843238dc015018f71227c26c37dc47571681a45ad62cd18f03adc769e5eb8 0.0s done
2026-Feb-25 05:08:45.955381
#59 exporting manifest list sha256:26181aeeaab563d5b46bde29bbf5e2748b1d1003d4774fdd9f6487eac655a933 done
2026-Feb-25 05:08:45.955381
#59 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_api:4964f9a4405b338e6420a0e7d46a1e71ddfa1598 done
2026-Feb-25 05:08:45.955381
#59 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_api:4964f9a4405b338e6420a0e7d46a1e71ddfa1598 done
2026-Feb-25 05:08:45.955381
#59 DONE 0.0s
2026-Feb-25 05:08:45.955381
2026-Feb-25 05:08:45.955381
#60 [frontend] exporting to image
2026-Feb-25 05:08:45.955381
#60 exporting layers done
2026-Feb-25 05:08:45.955381
#60 exporting manifest sha256:2b7ac74b4236473bdbc9346f2b506de61f4d1f296befca629b0cf08b5862eebb done
2026-Feb-25 05:08:45.955381
#60 exporting config sha256:a96810395cbeedf365cde1af568c06aefbb063c22df12b82173f74e2eb9d64a7 done
2026-Feb-25 05:08:45.955381
#60 exporting attestation manifest sha256:ccc942d7130aaac41001081bdf29de47b5bdb5500a483c64e1de9d05216d5236 done
2026-Feb-25 05:08:45.955381
#60 exporting manifest list sha256:65737ec84d44c1a6d7f47468fab2f584222a1920faffb7119eee4d45e6203331 done
2026-Feb-25 05:08:45.955381
#60 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:4964f9a4405b338e6420a0e7d46a1e71ddfa1598 done
2026-Feb-25 05:08:45.955381
#60 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:4964f9a4405b338e6420a0e7d46a1e71ddfa1598 done
2026-Feb-25 05:08:45.955381
#60 DONE 0.1s
2026-Feb-25 05:08:45.955381
2026-Feb-25 05:08:45.955381
#61 [gpu-worker] exporting to image
2026-Feb-25 05:08:45.955381
#61 exporting layers done
2026-Feb-25 05:08:45.955381
#61 exporting manifest sha256:073e01b76815a2f926d63cecc99892bb1d10b4325e87d13f4e1961e83c540d2c done
2026-Feb-25 05:08:45.955381
#61 exporting config sha256:512ecff06b1058051b92eb7bb364df2ef9135bd921d9c27c0f14c2ea35c1b896 done
2026-Feb-25 05:08:45.955381
#61 exporting attestation manifest sha256:487dc88506ba0b8492515326a09d4144c74d62b4897064eca58b330dd916fccb 0.0s done
2026-Feb-25 05:08:45.955381
#61 exporting manifest list sha256:59e8b040c468bdba3b5ec70a1418730b4981690fd3beb3543bd5ecb306604922 done
2026-Feb-25 05:08:45.955381
#61 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_gpu-worker:4964f9a4405b338e6420a0e7d46a1e71ddfa1598 done
2026-Feb-25 05:08:45.955381
#61 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_gpu-worker:4964f9a4405b338e6420a0e7d46a1e71ddfa1598 done
2026-Feb-25 05:08:45.955381
#61 DONE 0.1s
2026-Feb-25 05:08:45.955381
2026-Feb-25 05:08:45.955381
#62 [frontend] resolving provenance for metadata file
2026-Feb-25 05:08:45.986019
#62 DONE 0.0s
2026-Feb-25 05:08:45.986019
2026-Feb-25 05:08:45.986019
#63 [api] resolving provenance for metadata file
2026-Feb-25 05:08:45.986019
#63 DONE 0.0s
2026-Feb-25 05:08:45.986019
2026-Feb-25 05:08:45.986019
#64 [gpu-worker] resolving provenance for metadata file
2026-Feb-25 05:08:45.986019
#64 DONE 0.0s
2026-Feb-25 05:08:45.987937
gpu-worker  Built
2026-Feb-25 05:08:45.987937
api  Built
2026-Feb-25 05:08:45.987937
frontend  Built
2026-Feb-25 05:08:45.999155
Creating .env file with runtime variables for container.
2026-Feb-25 05:08:46.663522
Removing old containers.
2026-Feb-25 05:08:47.555356
[CMD]: docker stop -t 30 frontend-o8kccgkgwsockoocow8sg88s-041008215319
2026-Feb-25 05:08:47.555356
frontend-o8kccgkgwsockoocow8sg88s-041008215319
2026-Feb-25 05:08:47.873948
[CMD]: docker rm -f frontend-o8kccgkgwsockoocow8sg88s-041008215319
2026-Feb-25 05:08:47.873948
frontend-o8kccgkgwsockoocow8sg88s-041008215319
2026-Feb-25 05:09:18.437068
[CMD]: docker stop -t 30 api-o8kccgkgwsockoocow8sg88s-041008213737
2026-Feb-25 05:09:18.437068
api-o8kccgkgwsockoocow8sg88s-041008213737
2026-Feb-25 05:09:18.748919
[CMD]: docker rm -f api-o8kccgkgwsockoocow8sg88s-041008213737
2026-Feb-25 05:09:18.748919
api-o8kccgkgwsockoocow8sg88s-041008213737
2026-Feb-25 05:09:49.319489
[CMD]: docker stop -t 30 gpu-worker-o8kccgkgwsockoocow8sg88s-041008211408
2026-Feb-25 05:09:49.319489
gpu-worker-o8kccgkgwsockoocow8sg88s-041008211408
2026-Feb-25 05:09:49.639451
[CMD]: docker rm -f gpu-worker-o8kccgkgwsockoocow8sg88s-041008211408
2026-Feb-25 05:09:49.639451
gpu-worker-o8kccgkgwsockoocow8sg88s-041008211408
2026-Feb-25 05:09:49.641954
Starting new application.
2026-Feb-25 05:09:50.717465
[CMD]: docker exec bwo0440og4kooocc8kccc4sg bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/bwo0440og4kooocc8kccc4sg/.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/bwo0440og4kooocc8kccc4sg -f /artifacts/bwo0440og4kooocc8kccc4sg/docker/docker-compose.homeserver.yml up -d'
2026-Feb-25 05:09:50.717465
Container gpu-worker-o8kccgkgwsockoocow8sg88s-050839852837  Creating
2026-Feb-25 05:09:50.753636
Container gpu-worker-o8kccgkgwsockoocow8sg88s-050839852837  Created
2026-Feb-25 05:09:50.753636
Container api-o8kccgkgwsockoocow8sg88s-050839855763  Creating
2026-Feb-25 05:09:50.777592
Container api-o8kccgkgwsockoocow8sg88s-050839855763  Created
2026-Feb-25 05:09:50.777592
Container frontend-o8kccgkgwsockoocow8sg88s-050839857720  Creating
2026-Feb-25 05:09:50.802188
Container frontend-o8kccgkgwsockoocow8sg88s-050839857720  Created
2026-Feb-25 05:09:50.810722
Container gpu-worker-o8kccgkgwsockoocow8sg88s-050839852837  Starting
2026-Feb-25 05:09:51.046368
Container gpu-worker-o8kccgkgwsockoocow8sg88s-050839852837  Started
2026-Feb-25 05:09:51.049264
Container api-o8kccgkgwsockoocow8sg88s-050839855763  Starting
2026-Feb-25 05:09:51.195330
Container api-o8kccgkgwsockoocow8sg88s-050839855763  Started
2026-Feb-25 05:09:51.195330
Container frontend-o8kccgkgwsockoocow8sg88s-050839857720  Starting
2026-Feb-25 05:09:51.355222
Container frontend-o8kccgkgwsockoocow8sg88s-050839857720  Started
2026-Feb-25 05:09:52.233810
New container started.