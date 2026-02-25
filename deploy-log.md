2026-Feb-25 08:34:14.937361
Starting deployment of khoa280703/downloadtool:main-zoscg4oc04gkwkssg0kw8w8w to localhost.
2026-Feb-25 08:34:15.516381
Preparing container with helper image: ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Feb-25 08:34:15.816826
[CMD]: docker stop -t 30 mgoc8wk8s0cw40ow4ws0k4wk
2026-Feb-25 08:34:15.816826
Error response from daemon: No such container: mgoc8wk8s0cw40ow4ws0k4wk
2026-Feb-25 08:34:16.134786
[CMD]: docker run -d --network coolify --name mgoc8wk8s0cw40ow4ws0k4wk  --rm -v /var/run/docker.sock:/var/run/docker.sock ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Feb-25 08:34:16.134786
4f493c42d88b2816a251cb27825518f99508d4cf9cd941369767809fa37d27c6
2026-Feb-25 08:34:17.711540
[CMD]: docker exec mgoc8wk8s0cw40ow4ws0k4wk bash -c 'GIT_SSH_COMMAND="ssh -o ConnectTimeout=30 -p 22 -o Port=22 -o LogLevel=ERROR -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git ls-remote https://github.com/Khoa280703/downloadtool refs/heads/main'
2026-Feb-25 08:34:17.711540
7c55f8fc9d61492ef7ea10a7225d6d35c34a5c38	refs/heads/main
2026-Feb-25 08:34:17.721384
----------------------------------------
2026-Feb-25 08:34:17.726243
Importing Khoa280703/downloadtool:main (commit sha 7c55f8fc9d61492ef7ea10a7225d6d35c34a5c38) to /artifacts/mgoc8wk8s0cw40ow4ws0k4wk.
2026-Feb-25 08:34:18.091431
[CMD]: docker exec mgoc8wk8s0cw40ow4ws0k4wk bash -c 'git clone --depth=1 --recurse-submodules --shallow-submodules -b 'main' 'https://github.com/Khoa280703/downloadtool' '/artifacts/mgoc8wk8s0cw40ow4ws0k4wk' && cd '/artifacts/mgoc8wk8s0cw40ow4ws0k4wk' && if [ -f .gitmodules ]; then sed -i "s#git@\(.*\):#https://\1/#g" '/artifacts/mgoc8wk8s0cw40ow4ws0k4wk'/.gitmodules || true && git submodule sync && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git submodule update --init --recursive --depth=1; fi && cd '/artifacts/mgoc8wk8s0cw40ow4ws0k4wk' && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git lfs pull'
2026-Feb-25 08:34:18.091431
Cloning into '/artifacts/mgoc8wk8s0cw40ow4ws0k4wk'...
2026-Feb-25 08:34:20.705336
[CMD]: docker exec mgoc8wk8s0cw40ow4ws0k4wk bash -c 'cd /artifacts/mgoc8wk8s0cw40ow4ws0k4wk && git log -1 7c55f8fc9d61492ef7ea10a7225d6d35c34a5c38 --pretty=%B'
2026-Feb-25 08:34:20.705336
fix
2026-Feb-25 08:34:25.286716
[CMD]: docker exec mgoc8wk8s0cw40ow4ws0k4wk bash -c 'test -f /artifacts/mgoc8wk8s0cw40ow4ws0k4wk/docker/Dockerfile.homeserver && echo 'exists' || echo 'not found''
2026-Feb-25 08:34:25.286716
exists
2026-Feb-25 08:34:25.648583
[CMD]: docker exec mgoc8wk8s0cw40ow4ws0k4wk bash -c 'cat /artifacts/mgoc8wk8s0cw40ow4ws0k4wk/docker/Dockerfile.homeserver'
2026-Feb-25 08:34:25.648583
# Dockerfile for Home Server deployment
2026-Feb-25 08:34:25.648583
# Builds the GPU worker with CUDA support for hardware transcoding
2026-Feb-25 08:34:25.648583
2026-Feb-25 08:34:25.648583
FROM nvidia/cuda:12.3.2-devel-ubuntu22.04 AS builder
2026-Feb-25 08:34:25.648583
2026-Feb-25 08:34:25.648583
WORKDIR /app
2026-Feb-25 08:34:25.648583
2026-Feb-25 08:34:25.648583
# Install dependencies
2026-Feb-25 08:34:25.648583
RUN apt-get update && apt-get install -y \
2026-Feb-25 08:34:25.648583
curl \
2026-Feb-25 08:34:25.648583
build-essential \
2026-Feb-25 08:34:25.648583
pkg-config \
2026-Feb-25 08:34:25.648583
libssl-dev \
2026-Feb-25 08:34:25.648583
protobuf-compiler \
2026-Feb-25 08:34:25.648583
ffmpeg \
2026-Feb-25 08:34:25.648583
libavcodec-dev \
2026-Feb-25 08:34:25.648583
libavformat-dev \
2026-Feb-25 08:34:25.648583
libavutil-dev \
2026-Feb-25 08:34:25.648583
libswscale-dev \
2026-Feb-25 08:34:25.648583
libavfilter-dev \
2026-Feb-25 08:34:25.648583
libavdevice-dev \
2026-Feb-25 08:34:25.648583
clang \
2026-Feb-25 08:34:25.648583
libclang-dev \
2026-Feb-25 08:34:25.648583
&& rm -rf /var/lib/apt/lists/*
2026-Feb-25 08:34:25.648583
2026-Feb-25 08:34:25.648583
# Install Rust
2026-Feb-25 08:34:25.648583
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
2026-Feb-25 08:34:25.648583
ENV PATH="/root/.cargo/bin:${PATH}"
2026-Feb-25 08:34:25.648583
2026-Feb-25 08:34:25.648583
# Copy workspace configuration
2026-Feb-25 08:34:25.648583
COPY Cargo.toml ./
2026-Feb-25 08:34:25.648583
COPY Cargo.lock ./
2026-Feb-25 08:34:25.648583
2026-Feb-25 08:34:25.648583
# Copy all crates
2026-Feb-25 08:34:25.648583
COPY crates/ ./crates/
2026-Feb-25 08:34:25.648583
2026-Feb-25 08:34:25.648583
# Copy proto files
2026-Feb-25 08:34:25.648583
COPY proto/ ./proto/
2026-Feb-25 08:34:25.648583
2026-Feb-25 08:34:25.648583
# Build the GPU worker with CUDA support
2026-Feb-25 08:34:25.648583
# NOTE: --features gpu disabled until FFmpeg 7.x is available in the build environment
2026-Feb-25 08:34:25.648583
# Ubuntu 22.04 apt provides FFmpeg 4.4.x but ffmpeg-next = "7" requires FFmpeg 7.x
2026-Feb-25 08:34:25.648583
RUN cargo build --release --bin gpu-node
2026-Feb-25 08:34:25.648583
2026-Feb-25 08:34:25.648583
# Stage 2: Runtime
2026-Feb-25 08:34:25.648583
FROM nvidia/cuda:12.3.2-runtime-ubuntu22.04 AS runtime
2026-Feb-25 08:34:25.648583
2026-Feb-25 08:34:25.648583
WORKDIR /app
2026-Feb-25 08:34:25.648583
2026-Feb-25 08:34:25.648583
# Install runtime dependencies
2026-Feb-25 08:34:25.648583
RUN apt-get update && apt-get install -y \
2026-Feb-25 08:34:25.648583
ca-certificates \
2026-Feb-25 08:34:25.648583
libssl3 \
2026-Feb-25 08:34:25.648583
ffmpeg \
2026-Feb-25 08:34:25.648583
libavcodec58 \
2026-Feb-25 08:34:25.648583
libavformat58 \
2026-Feb-25 08:34:25.648583
libavutil56 \
2026-Feb-25 08:34:25.648583
libswscale5 \
2026-Feb-25 08:34:25.648583
libavfilter7 \
2026-Feb-25 08:34:25.648583
libavdevice58 \
2026-Feb-25 08:34:25.648583
&& rm -rf /var/lib/apt/lists/*
2026-Feb-25 08:34:25.648583
2026-Feb-25 08:34:25.648583
# Create non-root user
2026-Feb-25 08:34:25.648583
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-25 08:34:25.648583
2026-Feb-25 08:34:25.648583
# Copy binary from builder
2026-Feb-25 08:34:25.648583
COPY --from=builder /app/target/release/gpu-node /usr/local/bin/
2026-Feb-25 08:34:25.648583
2026-Feb-25 08:34:25.648583
# Create directories
2026-Feb-25 08:34:25.648583
RUN mkdir -p /app/data && chown -R appuser:appuser /app
2026-Feb-25 08:34:25.648583
2026-Feb-25 08:34:25.648583
# Switch to non-root user
2026-Feb-25 08:34:25.648583
USER appuser
2026-Feb-25 08:34:25.648583
2026-Feb-25 08:34:25.648583
# Environment variables
2026-Feb-25 08:34:25.648583
ENV GPU_WORKER_BIND=0.0.0.0:50051
2026-Feb-25 08:34:25.648583
ENV GPU_WORKER_MAX_JOBS=4
2026-Feb-25 08:34:25.648583
ENV CUDA_DEVICE_ID=0
2026-Feb-25 08:34:25.648583
ENV RUST_LOG=info
2026-Feb-25 08:34:25.648583
2026-Feb-25 08:34:25.648583
# NVIDIA runtime configuration
2026-Feb-25 08:34:25.648583
ENV NVIDIA_VISIBLE_DEVICES=all
2026-Feb-25 08:34:25.648583
ENV NVIDIA_DRIVER_CAPABILITIES=compute,video,utility
2026-Feb-25 08:34:25.648583
2026-Feb-25 08:34:25.648583
# Expose gRPC port
2026-Feb-25 08:34:25.648583
EXPOSE 50051
2026-Feb-25 08:34:25.648583
2026-Feb-25 08:34:25.648583
# Health check
2026-Feb-25 08:34:25.648583
HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
2026-Feb-25 08:34:25.648583
CMD echo "health check placeholder" || exit 0
2026-Feb-25 08:34:25.648583
2026-Feb-25 08:34:25.648583
# Run the GPU worker
2026-Feb-25 08:34:25.648583
CMD ["gpu-node"]
2026-Feb-25 08:34:26.037760
Added 12 ARG declarations to Dockerfile for service gpu-worker (multi-stage build, added to 2 stages).
2026-Feb-25 08:34:26.400937
[CMD]: docker exec mgoc8wk8s0cw40ow4ws0k4wk bash -c 'test -f /artifacts/mgoc8wk8s0cw40ow4ws0k4wk/docker/Dockerfile.vps && echo 'exists' || echo 'not found''
2026-Feb-25 08:34:26.400937
exists
2026-Feb-25 08:34:26.757314
[CMD]: docker exec mgoc8wk8s0cw40ow4ws0k4wk bash -c 'cat /artifacts/mgoc8wk8s0cw40ow4ws0k4wk/docker/Dockerfile.vps'
2026-Feb-25 08:34:26.757314
# Dockerfile for VPS deployment
2026-Feb-25 08:34:26.757314
# Builds the API server and related components without GPU support
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
# Stage 0: Build injector JS (embedded into api crate via include_str! at compile time)
2026-Feb-25 08:34:26.757314
FROM node:22-alpine AS js-builder
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
WORKDIR /app
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
RUN npm install -g pnpm
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
# Copy workspace manifests for pnpm resolution
2026-Feb-25 08:34:26.757314
COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Feb-25 08:34:26.757314
COPY packages/api-client/package.json ./packages/api-client/
2026-Feb-25 08:34:26.757314
COPY apps/injector/package.json ./apps/injector/
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
# Copy injector source and shared packages
2026-Feb-25 08:34:26.757314
COPY apps/injector/ ./apps/injector/
2026-Feb-25 08:34:26.757314
COPY packages/ ./packages/
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
# Install deps and build injector (produces dist/bm.js and dist/youtube-downloader.user.js)
2026-Feb-25 08:34:26.757314
RUN pnpm install --frozen-lockfile
2026-Feb-25 08:34:26.757314
RUN pnpm --filter @downloadtool/injector build
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
# Stage 1: Rust builder
2026-Feb-25 08:34:26.757314
FROM rust:1.88-bookworm AS builder
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
WORKDIR /app
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
# Install dependencies
2026-Feb-25 08:34:26.757314
RUN apt-get update && apt-get install -y \
2026-Feb-25 08:34:26.757314
pkg-config \
2026-Feb-25 08:34:26.757314
libssl-dev \
2026-Feb-25 08:34:26.757314
protobuf-compiler \
2026-Feb-25 08:34:26.757314
&& rm -rf /var/lib/apt/lists/*
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
# Copy workspace configuration
2026-Feb-25 08:34:26.757314
COPY Cargo.toml ./
2026-Feb-25 08:34:26.757314
COPY Cargo.lock ./
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
# Copy all crates
2026-Feb-25 08:34:26.757314
COPY crates/ ./crates/
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
# Copy proto files
2026-Feb-25 08:34:26.757314
COPY proto/ ./proto/
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
# Copy injector dist (required by include_str! in crates/api/src/routes/static_files.rs)
2026-Feb-25 08:34:26.757314
COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
# Build the release binary
2026-Feb-25 08:34:26.757314
RUN cargo build --release --bin vps-gateway
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
# Stage 2: Runtime
2026-Feb-25 08:34:26.757314
FROM debian:bookworm-slim AS runtime
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
WORKDIR /app
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
# Install runtime dependencies
2026-Feb-25 08:34:26.757314
RUN apt-get update && apt-get install -y \
2026-Feb-25 08:34:26.757314
ca-certificates \
2026-Feb-25 08:34:26.757314
libssl3 \
2026-Feb-25 08:34:26.757314
&& rm -rf /var/lib/apt/lists/*
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
# Create non-root user
2026-Feb-25 08:34:26.757314
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
# Copy binary from builder
2026-Feb-25 08:34:26.757314
COPY --from=builder /app/target/release/vps-gateway /usr/local/bin/
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
# Create directories
2026-Feb-25 08:34:26.757314
RUN mkdir -p /app/extractors /app/data && chown -R appuser:appuser /app
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
# Switch to non-root user
2026-Feb-25 08:34:26.757314
USER appuser
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
# Environment variables
2026-Feb-25 08:34:26.757314
ENV PORT=3068
2026-Feb-25 08:34:26.757314
ENV EXTRACTOR_DIR=/app/extractors
2026-Feb-25 08:34:26.757314
ENV GPU_ENABLED=false
2026-Feb-25 08:34:26.757314
ENV RUST_LOG=info
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
# Expose port
2026-Feb-25 08:34:26.757314
EXPOSE 3068
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
# Health check
2026-Feb-25 08:34:26.757314
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Feb-25 08:34:26.757314
CMD curl -f http://localhost:3068/health || exit 1
2026-Feb-25 08:34:26.757314
2026-Feb-25 08:34:26.757314
# Run the server
2026-Feb-25 08:34:26.757314
CMD ["vps-gateway"]
2026-Feb-25 08:34:27.106844
Added 18 ARG declarations to Dockerfile for service api (multi-stage build, added to 3 stages).
2026-Feb-25 08:34:27.478973
[CMD]: docker exec mgoc8wk8s0cw40ow4ws0k4wk bash -c 'test -f /artifacts/mgoc8wk8s0cw40ow4ws0k4wk/docker/Dockerfile.frontend && echo 'exists' || echo 'not found''
2026-Feb-25 08:34:27.478973
exists
2026-Feb-25 08:34:27.850601
[CMD]: docker exec mgoc8wk8s0cw40ow4ws0k4wk bash -c 'cat /artifacts/mgoc8wk8s0cw40ow4ws0k4wk/docker/Dockerfile.frontend'
2026-Feb-25 08:34:27.850601
# Dockerfile for frontend (SvelteKit Node server)
2026-Feb-25 08:34:27.850601
# Copy ALL source files BEFORE npm install so svelte-kit sync (prepare script)
2026-Feb-25 08:34:27.850601
# can find svelte.config.js and generate .svelte-kit/ correctly.
2026-Feb-25 08:34:27.850601
2026-Feb-25 08:34:27.850601
FROM node:22-alpine AS builder
2026-Feb-25 08:34:27.850601
2026-Feb-25 08:34:27.850601
WORKDIR /app
2026-Feb-25 08:34:27.850601
2026-Feb-25 08:34:27.850601
# Copy all frontend source files first (node_modules excluded via .dockerignore)
2026-Feb-25 08:34:27.850601
COPY frontend/ ./
2026-Feb-25 08:34:27.850601
2026-Feb-25 08:34:27.850601
# Install — prepare script runs svelte-kit sync with svelte.config.js available
2026-Feb-25 08:34:27.850601
RUN npm install
2026-Feb-25 08:34:27.850601
2026-Feb-25 08:34:27.850601
# Build via programmatic API — bypasses vite.config.ts loading (esbuild issues on Alpine)
2026-Feb-25 08:34:27.850601
RUN node build-docker.mjs
2026-Feb-25 08:34:27.850601
2026-Feb-25 08:34:27.850601
# Runtime
2026-Feb-25 08:34:27.850601
FROM node:22-alpine AS runtime
2026-Feb-25 08:34:27.850601
2026-Feb-25 08:34:27.850601
WORKDIR /app
2026-Feb-25 08:34:27.850601
2026-Feb-25 08:34:27.850601
COPY --from=builder /app/build ./build
2026-Feb-25 08:34:27.850601
COPY --from=builder /app/package.json ./
2026-Feb-25 08:34:27.850601
2026-Feb-25 08:34:27.850601
ENV PORT=3000
2026-Feb-25 08:34:27.850601
ENV HOST=0.0.0.0
2026-Feb-25 08:34:27.850601
2026-Feb-25 08:34:27.850601
EXPOSE 3000
2026-Feb-25 08:34:27.850601
2026-Feb-25 08:34:27.850601
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Feb-25 08:34:27.850601
CMD wget -qO- http://localhost:3000 || exit 1
2026-Feb-25 08:34:27.850601
2026-Feb-25 08:34:27.850601
CMD ["node", "build"]
2026-Feb-25 08:34:28.194360
Added 12 ARG declarations to Dockerfile for service frontend (multi-stage build, added to 2 stages).
2026-Feb-25 08:34:28.199278
Pulling & building required images.
2026-Feb-25 08:34:28.205532
Creating build-time .env file in /artifacts (outside Docker context).
2026-Feb-25 08:34:28.557444
Adding build arguments to Docker Compose build command.
2026-Feb-25 08:34:29.031505
[CMD]: docker exec mgoc8wk8s0cw40ow4ws0k4wk bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/build-time.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/mgoc8wk8s0cw40ow4ws0k4wk -f /artifacts/mgoc8wk8s0cw40ow4ws0k4wk/docker/docker-compose.homeserver.yml build --pull --build-arg COOLIFY_URL --build-arg COOLIFY_FQDN --build-arg SERVICE_URL_FRONTEND --build-arg SERVICE_FQDN_FRONTEND --build-arg SERVICE_URL_API --build-arg SERVICE_FQDN_API --build-arg COOLIFY_BUILD_SECRETS_HASH=016dc4226f6c53eea50a0f46c58a295ad358d7f449966f8f46fd81293803bce6'
2026-Feb-25 08:34:29.031505
#1 [internal] load local bake definitions
2026-Feb-25 08:34:29.241463
#1 reading from stdin 2.57kB done
2026-Feb-25 08:34:29.241463
#1 DONE 0.0s
2026-Feb-25 08:34:29.241463
2026-Feb-25 08:34:29.241463
#2 [frontend internal] load build definition from Dockerfile.frontend
2026-Feb-25 08:34:29.241463
#2 transferring dockerfile: 1.18kB done
2026-Feb-25 08:34:29.241463
#2 DONE 0.0s
2026-Feb-25 08:34:29.241463
2026-Feb-25 08:34:29.241463
#3 [api internal] load build definition from Dockerfile.vps
2026-Feb-25 08:34:29.241463
#3 transferring dockerfile: 2.63kB done
2026-Feb-25 08:34:29.241463
#3 DONE 0.0s
2026-Feb-25 08:34:29.241463
2026-Feb-25 08:34:29.241463
#4 [gpu-worker internal] load build definition from Dockerfile.homeserver
2026-Feb-25 08:34:29.241463
#4 transferring dockerfile: 2.46kB done
2026-Feb-25 08:34:29.241463
#4 DONE 0.0s
2026-Feb-25 08:34:29.241463
2026-Feb-25 08:34:29.241463
#5 [api internal] load metadata for docker.io/library/node:22-alpine
2026-Feb-25 08:34:30.221044
#5 ...
2026-Feb-25 08:34:30.221044
2026-Feb-25 08:34:30.221044
#6 [gpu-worker internal] load metadata for docker.io/nvidia/cuda:12.3.2-devel-ubuntu22.04
2026-Feb-25 08:34:30.221044
#6 DONE 1.1s
2026-Feb-25 08:34:30.321397
#7 [api internal] load metadata for docker.io/library/rust:1.88-bookworm
2026-Feb-25 08:34:30.321397
#7 DONE 1.1s
2026-Feb-25 08:34:30.321397
2026-Feb-25 08:34:30.321397
#5 [api internal] load metadata for docker.io/library/node:22-alpine
2026-Feb-25 08:34:30.321397
#5 DONE 1.2s
2026-Feb-25 08:34:30.321397
2026-Feb-25 08:34:30.321397
#8 [gpu-worker internal] load metadata for docker.io/nvidia/cuda:12.3.2-runtime-ubuntu22.04
2026-Feb-25 08:34:30.321397
#8 DONE 1.2s
2026-Feb-25 08:34:30.321397
2026-Feb-25 08:34:30.321397
#9 [api internal] load .dockerignore
2026-Feb-25 08:34:30.321397
#9 transferring context: 341B done
2026-Feb-25 08:34:30.321397
#9 DONE 0.0s
2026-Feb-25 08:34:30.321397
2026-Feb-25 08:34:30.321397
#10 [api internal] load metadata for docker.io/library/debian:bookworm-slim
2026-Feb-25 08:34:30.321397
#10 DONE 1.2s
2026-Feb-25 08:34:30.321397
2026-Feb-25 08:34:30.321397
#11 [api builder 1/5] FROM docker.io/library/node:22-alpine@sha256:e4bf2a82ad0a4037d28035ae71529873c069b13eb0455466ae0bc13363826e34
2026-Feb-25 08:34:30.321397
#11 resolve docker.io/library/node:22-alpine@sha256:e4bf2a82ad0a4037d28035ae71529873c069b13eb0455466ae0bc13363826e34 0.0s done
2026-Feb-25 08:34:30.321397
#11 DONE 0.0s
2026-Feb-25 08:34:30.321397
2026-Feb-25 08:34:30.321397
#12 [gpu-worker builder 1/9] FROM docker.io/nvidia/cuda:12.3.2-devel-ubuntu22.04@sha256:6655d5fc2fb48580255a5021a81c379c325a457b74b77ac823ed67e4faa32aeb
2026-Feb-25 08:34:30.321397
#12 resolve docker.io/nvidia/cuda:12.3.2-devel-ubuntu22.04@sha256:6655d5fc2fb48580255a5021a81c379c325a457b74b77ac823ed67e4faa32aeb 0.0s done
2026-Feb-25 08:34:30.321397
#12 DONE 0.0s
2026-Feb-25 08:34:30.321397
2026-Feb-25 08:34:30.321397
#13 [gpu-worker runtime 1/6] FROM docker.io/nvidia/cuda:12.3.2-runtime-ubuntu22.04@sha256:882b43fadd789693f08de95e6014ed5f0ea118c7b342150876e153b4340e1103
2026-Feb-25 08:34:30.321397
#13 resolve docker.io/nvidia/cuda:12.3.2-runtime-ubuntu22.04@sha256:882b43fadd789693f08de95e6014ed5f0ea118c7b342150876e153b4340e1103 0.0s done
2026-Feb-25 08:34:30.321397
#13 DONE 0.0s
2026-Feb-25 08:34:30.321397
2026-Feb-25 08:34:30.321397
#14 [api runtime 1/6] FROM docker.io/library/debian:bookworm-slim@sha256:74d56e3931e0d5a1dd51f8c8a2466d21de84a271cd3b5a733b803aa91abf4421
2026-Feb-25 08:34:30.321397
#14 resolve docker.io/library/debian:bookworm-slim@sha256:74d56e3931e0d5a1dd51f8c8a2466d21de84a271cd3b5a733b803aa91abf4421 0.0s done
2026-Feb-25 08:34:30.321397
#14 DONE 0.0s
2026-Feb-25 08:34:30.321397
2026-Feb-25 08:34:30.321397
#15 [api builder 1/9] FROM docker.io/library/rust:1.88-bookworm@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0
2026-Feb-25 08:34:30.321397
#15 resolve docker.io/library/rust:1.88-bookworm@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0 0.0s done
2026-Feb-25 08:34:30.321397
#15 DONE 0.0s
2026-Feb-25 08:34:30.321397
2026-Feb-25 08:34:30.321397
#16 [frontend internal] load build context
2026-Feb-25 08:34:30.321397
#16 transferring context: 197.18kB done
2026-Feb-25 08:34:30.321397
#16 DONE 0.0s
2026-Feb-25 08:34:30.321397
2026-Feb-25 08:34:30.321397
#17 [gpu-worker internal] load build context
2026-Feb-25 08:34:30.321397
#17 transferring context: 475.95kB 0.0s done
2026-Feb-25 08:34:30.321397
#17 DONE 0.0s
2026-Feb-25 08:34:30.321397
2026-Feb-25 08:34:30.321397
#18 [frontend builder 2/5] WORKDIR /app
2026-Feb-25 08:34:30.321397
#18 CACHED
2026-Feb-25 08:34:30.321397
2026-Feb-25 08:34:30.321397
#19 [frontend builder 4/5] RUN npm install
2026-Feb-25 08:34:30.321397
#19 CACHED
2026-Feb-25 08:34:30.323207
#20 [frontend builder 3/5] COPY frontend/ ./
2026-Feb-25 08:34:30.323207
#20 CACHED
2026-Feb-25 08:34:30.323207
2026-Feb-25 08:34:30.323207
#21 [frontend builder 5/5] RUN node build-docker.mjs
2026-Feb-25 08:34:30.323207
#21 CACHED
2026-Feb-25 08:34:30.323207
2026-Feb-25 08:34:30.323207
#22 [frontend runtime 3/4] COPY --from=builder /app/build ./build
2026-Feb-25 08:34:30.323207
#22 CACHED
2026-Feb-25 08:34:30.323207
2026-Feb-25 08:34:30.323207
#23 [api internal] load build context
2026-Feb-25 08:34:30.323207
#23 transferring context: 600.49kB 0.0s done
2026-Feb-25 08:34:30.323207
#23 DONE 0.0s
2026-Feb-25 08:34:30.323207
2026-Feb-25 08:34:30.323207
#24 [frontend runtime 4/4] COPY --from=builder /app/package.json ./
2026-Feb-25 08:34:30.323207
#24 CACHED
2026-Feb-25 08:34:30.323207
2026-Feb-25 08:34:30.323207
#25 [frontend] exporting to image
2026-Feb-25 08:34:30.323207
#25 exporting layers done
2026-Feb-25 08:34:30.323207
#25 exporting manifest sha256:af59e93e2d76249fe8e48f3b78a4c2996a0c8ec14950095b41355eacd04169f9 done
2026-Feb-25 08:34:30.323207
#25 exporting config sha256:9b744fd2ea03750445160c48f7ef79e7360b74c71b03aa041d864d27bc695e5b done
2026-Feb-25 08:34:30.437055
#25 exporting attestation manifest sha256:edf80fd0da00e2d5148c231dc3fec56585335ceb5dc1db9180a676ca08e09695 done
2026-Feb-25 08:34:30.437055
#25 exporting manifest list sha256:4b1a8613edf210061f43dde8d56dc70498d2f37d4ffad7730cc998826ee8c418 done
2026-Feb-25 08:34:30.437055
#25 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:7c55f8fc9d61492ef7ea10a7225d6d35c34a5c38 done
2026-Feb-25 08:34:30.437055
#25 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:7c55f8fc9d61492ef7ea10a7225d6d35c34a5c38 done
2026-Feb-25 08:34:30.437055
#25 DONE 0.1s
2026-Feb-25 08:34:30.437055
2026-Feb-25 08:34:30.437055
#26 [gpu-worker builder 7/9] COPY crates/ ./crates/
2026-Feb-25 08:34:30.437055
#26 CACHED
2026-Feb-25 08:34:30.437055
2026-Feb-25 08:34:30.437055
#27 [gpu-worker builder 2/9] WORKDIR /app
2026-Feb-25 08:34:30.437055
#27 CACHED
2026-Feb-25 08:34:30.437055
2026-Feb-25 08:34:30.437055
#28 [gpu-worker runtime 4/6] RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-25 08:34:30.437055
#28 CACHED
2026-Feb-25 08:34:30.437055
2026-Feb-25 08:34:30.437055
#29 [gpu-worker builder 9/9] RUN cargo build --release --bin gpu-node
2026-Feb-25 08:34:30.437055
#29 CACHED
2026-Feb-25 08:34:30.437055
2026-Feb-25 08:34:30.437055
#30 [gpu-worker runtime 2/6] WORKDIR /app
2026-Feb-25 08:34:30.437055
#30 CACHED
2026-Feb-25 08:34:30.437055
2026-Feb-25 08:34:30.437055
#31 [gpu-worker builder 6/9] COPY Cargo.lock ./
2026-Feb-25 08:34:30.437055
#31 CACHED
2026-Feb-25 08:34:30.437055
2026-Feb-25 08:34:30.437055
#32 [gpu-worker runtime 5/6] COPY --from=builder /app/target/release/gpu-node /usr/local/bin/
2026-Feb-25 08:34:30.437055
#32 CACHED
2026-Feb-25 08:34:30.437055
2026-Feb-25 08:34:30.437055
#33 [gpu-worker builder 8/9] COPY proto/ ./proto/
2026-Feb-25 08:34:30.437055
#33 CACHED
2026-Feb-25 08:34:30.437055
2026-Feb-25 08:34:30.437055
#34 [gpu-worker builder 4/9] RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
2026-Feb-25 08:34:30.437055
#34 CACHED
2026-Feb-25 08:34:30.437055
2026-Feb-25 08:34:30.437055
#35 [gpu-worker builder 5/9] COPY Cargo.toml ./
2026-Feb-25 08:34:30.437055
#35 CACHED
2026-Feb-25 08:34:30.437055
2026-Feb-25 08:34:30.437055
#36 [gpu-worker runtime 3/6] RUN apt-get update && apt-get install -y     ca-certificates     libssl3     ffmpeg     libavcodec58     libavformat58     libavutil56     libswscale5     libavfilter7     libavdevice58     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 08:34:30.437055
#36 CACHED
2026-Feb-25 08:34:30.437055
2026-Feb-25 08:34:30.437055
#37 [gpu-worker builder 3/9] RUN apt-get update && apt-get install -y     curl     build-essential     pkg-config     libssl-dev     protobuf-compiler     ffmpeg     libavcodec-dev     libavformat-dev     libavutil-dev     libswscale-dev     libavfilter-dev     libavdevice-dev     clang     libclang-dev     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 08:34:30.437055
#37 CACHED
2026-Feb-25 08:34:30.437055
2026-Feb-25 08:34:30.437055
#38 [gpu-worker runtime 6/6] RUN mkdir -p /app/data && chown -R appuser:appuser /app
2026-Feb-25 08:34:30.437055
#38 CACHED
2026-Feb-25 08:34:30.437055
2026-Feb-25 08:34:30.437055
#39 [api builder 9/9] RUN cargo build --release --bin vps-gateway
2026-Feb-25 08:34:30.437055
#39 CACHED
2026-Feb-25 08:34:30.437055
2026-Feb-25 08:34:30.437055
#40 [api runtime 3/6] RUN apt-get update && apt-get install -y     ca-certificates     libssl3     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 08:34:30.437055
#40 CACHED
2026-Feb-25 08:34:30.437055
2026-Feb-25 08:34:30.437055
#41 [api builder 8/9] COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Feb-25 08:34:30.437055
#41 CACHED
2026-Feb-25 08:34:30.437055
2026-Feb-25 08:34:30.437055
#42 [api js-builder  7/10] COPY apps/injector/ ./apps/injector/
2026-Feb-25 08:34:30.437055
#42 CACHED
2026-Feb-25 08:34:30.437055
2026-Feb-25 08:34:30.437055
#43 [api builder 5/9] COPY Cargo.lock ./
2026-Feb-25 08:34:30.437055
#43 CACHED
2026-Feb-25 08:34:30.437055
2026-Feb-25 08:34:30.437055
#18 [api builder 2/5] WORKDIR /app
2026-Feb-25 08:34:30.437055
#18 CACHED
2026-Feb-25 08:34:30.437055
2026-Feb-25 08:34:30.437055
#44 [api builder 2/9] WORKDIR /app
2026-Feb-25 08:34:30.437055
#44 CACHED
2026-Feb-25 08:34:30.439378
#45 [api builder 4/9] COPY Cargo.toml ./
2026-Feb-25 08:34:30.439378
#45 CACHED
2026-Feb-25 08:34:30.439378
2026-Feb-25 08:34:30.439378
#46 [api runtime 5/6] COPY --from=builder /app/target/release/vps-gateway /usr/local/bin/
2026-Feb-25 08:34:30.439378
#46 CACHED
2026-Feb-25 08:34:30.439378
2026-Feb-25 08:34:30.439378
#47 [api js-builder  6/10] COPY apps/injector/package.json ./apps/injector/
2026-Feb-25 08:34:30.439378
#47 CACHED
2026-Feb-25 08:34:30.439378
2026-Feb-25 08:34:30.439378
#48 [api js-builder  9/10] RUN pnpm install --frozen-lockfile
2026-Feb-25 08:34:30.439378
#48 CACHED
2026-Feb-25 08:34:30.439378
2026-Feb-25 08:34:30.439378
#49 [api builder 7/9] COPY proto/ ./proto/
2026-Feb-25 08:34:30.439378
#49 CACHED
2026-Feb-25 08:34:30.439378
2026-Feb-25 08:34:30.439378
#50 [api js-builder 10/10] RUN pnpm --filter @downloadtool/injector build
2026-Feb-25 08:34:30.439378
#50 CACHED
2026-Feb-25 08:34:30.439378
2026-Feb-25 08:34:30.439378
#51 [api runtime 4/6] RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-25 08:34:30.439378
#51 CACHED
2026-Feb-25 08:34:30.439378
2026-Feb-25 08:34:30.439378
#52 [api runtime 2/6] WORKDIR /app
2026-Feb-25 08:34:30.439378
#52 CACHED
2026-Feb-25 08:34:30.439378
2026-Feb-25 08:34:30.439378
#53 [api js-builder  8/10] COPY packages/ ./packages/
2026-Feb-25 08:34:30.439378
#53 CACHED
2026-Feb-25 08:34:30.439378
2026-Feb-25 08:34:30.439378
#54 [api builder 3/9] RUN apt-get update && apt-get install -y     pkg-config     libssl-dev     protobuf-compiler     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 08:34:30.439378
#54 CACHED
2026-Feb-25 08:34:30.439378
2026-Feb-25 08:34:30.439378
#55 [api js-builder  4/10] COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Feb-25 08:34:30.439378
#55 CACHED
2026-Feb-25 08:34:30.439378
2026-Feb-25 08:34:30.439378
#56 [api builder 6/9] COPY crates/ ./crates/
2026-Feb-25 08:34:30.439378
#56 CACHED
2026-Feb-25 08:34:30.439378
2026-Feb-25 08:34:30.439378
#57 [api js-builder  3/10] RUN npm install -g pnpm
2026-Feb-25 08:34:30.439378
#57 CACHED
2026-Feb-25 08:34:30.439378
2026-Feb-25 08:34:30.439378
#58 [api js-builder  5/10] COPY packages/api-client/package.json ./packages/api-client/
2026-Feb-25 08:34:30.439378
#58 CACHED
2026-Feb-25 08:34:30.439378
2026-Feb-25 08:34:30.439378
#59 [api runtime 6/6] RUN mkdir -p /app/extractors /app/data && chown -R appuser:appuser /app
2026-Feb-25 08:34:30.439378
#59 CACHED
2026-Feb-25 08:34:30.439378
2026-Feb-25 08:34:30.439378
#60 [api] exporting to image
2026-Feb-25 08:34:30.439378
#60 exporting layers done
2026-Feb-25 08:34:30.439378
#60 exporting manifest sha256:807e72a1d2039a43b058f99d5ce2aa8377504fa73231a165d3e9f06e309bf18a done
2026-Feb-25 08:34:30.439378
#60 exporting config sha256:df444ec535dcffc05b00ad425a1b6ba5949a8b7e69addb8957f8e6925ec59f3b done
2026-Feb-25 08:34:30.439378
#60 exporting attestation manifest sha256:99ab063381f830ac0c7b0e3d73b3683fd0ef9fb533cd927ecbe98a57054cbc8f 0.0s done
2026-Feb-25 08:34:30.439378
#60 exporting manifest list sha256:64855d6bc482e3608eb0417ac74eb4ae1d5cf35724ec949edd22caa7798e2d05 done
2026-Feb-25 08:34:30.439378
#60 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_api:7c55f8fc9d61492ef7ea10a7225d6d35c34a5c38 done
2026-Feb-25 08:34:30.439378
#60 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_api:7c55f8fc9d61492ef7ea10a7225d6d35c34a5c38 done
2026-Feb-25 08:34:30.439378
#60 DONE 0.1s
2026-Feb-25 08:34:30.439378
2026-Feb-25 08:34:30.439378
#61 [gpu-worker] exporting to image
2026-Feb-25 08:34:30.439378
#61 exporting layers done
2026-Feb-25 08:34:30.439378
#61 exporting manifest sha256:a204de5eee386b669cf4329535afe2c999a1ec201f28d7a4b46a672ee0a956ee done
2026-Feb-25 08:34:30.439378
#61 exporting config sha256:f1807f5bec10c85eef67afff233db8efe85a4b9cc0f27a4d5994217dc598a4ad done
2026-Feb-25 08:34:30.439378
#61 exporting attestation manifest sha256:0388160b70911640f829043c4058c8f555005fad361593104f90fcd69eef9109 0.0s done
2026-Feb-25 08:34:30.439378
#61 exporting manifest list sha256:6ffd70d6b588e0473126d20279c40c0eb1e5282014fa83d9bc5f75fa0d665e07 done
2026-Feb-25 08:34:30.439378
#61 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_gpu-worker:7c55f8fc9d61492ef7ea10a7225d6d35c34a5c38 done
2026-Feb-25 08:34:30.439378
#61 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_gpu-worker:7c55f8fc9d61492ef7ea10a7225d6d35c34a5c38 done
2026-Feb-25 08:34:30.439378
#61 DONE 0.1s
2026-Feb-25 08:34:30.439378
2026-Feb-25 08:34:30.439378
#62 [frontend] resolving provenance for metadata file
2026-Feb-25 08:34:30.468686
#62 DONE 0.0s
2026-Feb-25 08:34:30.468686
2026-Feb-25 08:34:30.468686
#63 [api] resolving provenance for metadata file
2026-Feb-25 08:34:30.468686
#63 DONE 0.0s
2026-Feb-25 08:34:30.468686
2026-Feb-25 08:34:30.468686
#64 [gpu-worker] resolving provenance for metadata file
2026-Feb-25 08:34:30.468686
#64 DONE 0.0s
2026-Feb-25 08:34:30.470651
api  Built
2026-Feb-25 08:34:30.470651
frontend  Built
2026-Feb-25 08:34:30.470651
gpu-worker  Built
2026-Feb-25 08:34:30.482082
Creating .env file with runtime variables for container.
2026-Feb-25 08:34:31.146487
Removing old containers.
2026-Feb-25 08:34:32.005641
[CMD]: docker stop -t 30 frontend-o8kccgkgwsockoocow8sg88s-082518752729
2026-Feb-25 08:34:32.005641
frontend-o8kccgkgwsockoocow8sg88s-082518752729
2026-Feb-25 08:34:32.331684
[CMD]: docker rm -f frontend-o8kccgkgwsockoocow8sg88s-082518752729
2026-Feb-25 08:34:32.331684
frontend-o8kccgkgwsockoocow8sg88s-082518752729
2026-Feb-25 08:35:02.869215
[CMD]: docker stop -t 30 api-o8kccgkgwsockoocow8sg88s-082518747949
2026-Feb-25 08:35:02.869215
api-o8kccgkgwsockoocow8sg88s-082518747949
2026-Feb-25 08:35:03.213897
[CMD]: docker rm -f api-o8kccgkgwsockoocow8sg88s-082518747949
2026-Feb-25 08:35:03.213897
api-o8kccgkgwsockoocow8sg88s-082518747949
2026-Feb-25 08:35:33.874736
[CMD]: docker stop -t 30 gpu-worker-o8kccgkgwsockoocow8sg88s-082518742675
2026-Feb-25 08:35:33.874736
gpu-worker-o8kccgkgwsockoocow8sg88s-082518742675
2026-Feb-25 08:35:34.206396
[CMD]: docker rm -f gpu-worker-o8kccgkgwsockoocow8sg88s-082518742675
2026-Feb-25 08:35:34.206396
gpu-worker-o8kccgkgwsockoocow8sg88s-082518742675
2026-Feb-25 08:35:34.209657
Starting new application.
2026-Feb-25 08:35:35.276708
[CMD]: docker exec mgoc8wk8s0cw40ow4ws0k4wk bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/mgoc8wk8s0cw40ow4ws0k4wk/.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/mgoc8wk8s0cw40ow4ws0k4wk -f /artifacts/mgoc8wk8s0cw40ow4ws0k4wk/docker/docker-compose.homeserver.yml up -d'
2026-Feb-25 08:35:35.276708
Container gpu-worker-o8kccgkgwsockoocow8sg88s-083424563310  Creating
2026-Feb-25 08:35:35.313728
Container gpu-worker-o8kccgkgwsockoocow8sg88s-083424563310  Created
2026-Feb-25 08:35:35.313728
Container api-o8kccgkgwsockoocow8sg88s-083424568884  Creating
2026-Feb-25 08:35:35.338846
Container api-o8kccgkgwsockoocow8sg88s-083424568884  Created
2026-Feb-25 08:35:35.338846
Container frontend-o8kccgkgwsockoocow8sg88s-083424573658  Creating
2026-Feb-25 08:35:35.587457
Container frontend-o8kccgkgwsockoocow8sg88s-083424573658  Created
2026-Feb-25 08:35:35.599956
Container gpu-worker-o8kccgkgwsockoocow8sg88s-083424563310  Starting
2026-Feb-25 08:35:35.797170
Container gpu-worker-o8kccgkgwsockoocow8sg88s-083424563310  Started
2026-Feb-25 08:35:35.797170
Container api-o8kccgkgwsockoocow8sg88s-083424568884  Starting
2026-Feb-25 08:35:35.914490
Container api-o8kccgkgwsockoocow8sg88s-083424568884  Started
2026-Feb-25 08:35:35.914490
Container frontend-o8kccgkgwsockoocow8sg88s-083424573658  Starting
2026-Feb-25 08:35:36.043978
Container frontend-o8kccgkgwsockoocow8sg88s-083424573658  Started
2026-Feb-25 08:35:36.966742
New container started.
2026-Feb-25 08:35:37.879053
Gracefully shutting down build container: mgoc8wk8s0cw40ow4ws0k4wk
2026-Feb-25 08:35:38.343811
[CMD]: docker stop -t 30 mgoc8wk8s0cw40ow4ws0k4wk
2026-Feb-25 08:35:38.343811
mgoc8wk8s0cw40ow4ws0k4wk