2026-Feb-25 08:25:06.694464
Starting deployment of khoa280703/downloadtool:main-zoscg4oc04gkwkssg0kw8w8w to localhost.
2026-Feb-25 08:25:07.292789
Preparing container with helper image: ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Feb-25 08:25:07.617380
[CMD]: docker stop -t 30 ac84ccs0ow8c0ksoocs0s4og
2026-Feb-25 08:25:07.617380
Error response from daemon: No such container: ac84ccs0ow8c0ksoocs0s4og
2026-Feb-25 08:25:07.964512
[CMD]: docker run -d --network coolify --name ac84ccs0ow8c0ksoocs0s4og  --rm -v /var/run/docker.sock:/var/run/docker.sock ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Feb-25 08:25:07.964512
bc548026b9ab22836db3d8e1610e1b58008a93a6ee6d8b8b8597dc7b5bcc7a90
2026-Feb-25 08:25:11.734932
[CMD]: docker exec ac84ccs0ow8c0ksoocs0s4og bash -c 'GIT_SSH_COMMAND="ssh -o ConnectTimeout=30 -p 22 -o Port=22 -o LogLevel=ERROR -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git ls-remote https://github.com/Khoa280703/downloadtool refs/heads/main'
2026-Feb-25 08:25:11.734932
7018562fe60a5232a865dbda115b8e79c33092c6	refs/heads/main
2026-Feb-25 08:25:11.746477
----------------------------------------
2026-Feb-25 08:25:11.750957
Importing Khoa280703/downloadtool:main (commit sha 7018562fe60a5232a865dbda115b8e79c33092c6) to /artifacts/ac84ccs0ow8c0ksoocs0s4og.
2026-Feb-25 08:25:12.115344
[CMD]: docker exec ac84ccs0ow8c0ksoocs0s4og bash -c 'git clone --depth=1 --recurse-submodules --shallow-submodules -b 'main' 'https://github.com/Khoa280703/downloadtool' '/artifacts/ac84ccs0ow8c0ksoocs0s4og' && cd '/artifacts/ac84ccs0ow8c0ksoocs0s4og' && if [ -f .gitmodules ]; then sed -i "s#git@\(.*\):#https://\1/#g" '/artifacts/ac84ccs0ow8c0ksoocs0s4og'/.gitmodules || true && git submodule sync && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git submodule update --init --recursive --depth=1; fi && cd '/artifacts/ac84ccs0ow8c0ksoocs0s4og' && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git lfs pull'
2026-Feb-25 08:25:12.115344
Cloning into '/artifacts/ac84ccs0ow8c0ksoocs0s4og'...
2026-Feb-25 08:25:14.768799
[CMD]: docker exec ac84ccs0ow8c0ksoocs0s4og bash -c 'cd /artifacts/ac84ccs0ow8c0ksoocs0s4og && git log -1 7018562fe60a5232a865dbda115b8e79c33092c6 --pretty=%B'
2026-Feb-25 08:25:14.768799
clean build
2026-Feb-25 08:25:19.477181
[CMD]: docker exec ac84ccs0ow8c0ksoocs0s4og bash -c 'test -f /artifacts/ac84ccs0ow8c0ksoocs0s4og/docker/Dockerfile.homeserver && echo 'exists' || echo 'not found''
2026-Feb-25 08:25:19.477181
exists
2026-Feb-25 08:25:19.833479
[CMD]: docker exec ac84ccs0ow8c0ksoocs0s4og bash -c 'cat /artifacts/ac84ccs0ow8c0ksoocs0s4og/docker/Dockerfile.homeserver'
2026-Feb-25 08:25:19.833479
# Dockerfile for Home Server deployment
2026-Feb-25 08:25:19.833479
# Builds the GPU worker with CUDA support for hardware transcoding
2026-Feb-25 08:25:19.833479
2026-Feb-25 08:25:19.833479
FROM nvidia/cuda:12.3.2-devel-ubuntu22.04 AS builder
2026-Feb-25 08:25:19.833479
2026-Feb-25 08:25:19.833479
WORKDIR /app
2026-Feb-25 08:25:19.833479
2026-Feb-25 08:25:19.833479
# Install dependencies
2026-Feb-25 08:25:19.833479
RUN apt-get update && apt-get install -y \
2026-Feb-25 08:25:19.833479
curl \
2026-Feb-25 08:25:19.833479
build-essential \
2026-Feb-25 08:25:19.833479
pkg-config \
2026-Feb-25 08:25:19.833479
libssl-dev \
2026-Feb-25 08:25:19.833479
protobuf-compiler \
2026-Feb-25 08:25:19.833479
ffmpeg \
2026-Feb-25 08:25:19.833479
libavcodec-dev \
2026-Feb-25 08:25:19.833479
libavformat-dev \
2026-Feb-25 08:25:19.833479
libavutil-dev \
2026-Feb-25 08:25:19.833479
libswscale-dev \
2026-Feb-25 08:25:19.833479
libavfilter-dev \
2026-Feb-25 08:25:19.833479
libavdevice-dev \
2026-Feb-25 08:25:19.833479
clang \
2026-Feb-25 08:25:19.833479
libclang-dev \
2026-Feb-25 08:25:19.833479
&& rm -rf /var/lib/apt/lists/*
2026-Feb-25 08:25:19.833479
2026-Feb-25 08:25:19.833479
# Install Rust
2026-Feb-25 08:25:19.833479
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
2026-Feb-25 08:25:19.833479
ENV PATH="/root/.cargo/bin:${PATH}"
2026-Feb-25 08:25:19.833479
2026-Feb-25 08:25:19.833479
# Copy workspace configuration
2026-Feb-25 08:25:19.833479
COPY Cargo.toml ./
2026-Feb-25 08:25:19.833479
COPY Cargo.lock ./
2026-Feb-25 08:25:19.833479
2026-Feb-25 08:25:19.833479
# Copy all crates
2026-Feb-25 08:25:19.833479
COPY crates/ ./crates/
2026-Feb-25 08:25:19.833479
2026-Feb-25 08:25:19.833479
# Copy proto files
2026-Feb-25 08:25:19.833479
COPY proto/ ./proto/
2026-Feb-25 08:25:19.833479
2026-Feb-25 08:25:19.833479
# Build the GPU worker with CUDA support
2026-Feb-25 08:25:19.833479
# NOTE: --features gpu disabled until FFmpeg 7.x is available in the build environment
2026-Feb-25 08:25:19.833479
# Ubuntu 22.04 apt provides FFmpeg 4.4.x but ffmpeg-next = "7" requires FFmpeg 7.x
2026-Feb-25 08:25:19.833479
RUN cargo build --release --bin gpu-node
2026-Feb-25 08:25:19.833479
2026-Feb-25 08:25:19.833479
# Stage 2: Runtime
2026-Feb-25 08:25:19.833479
FROM nvidia/cuda:12.3.2-runtime-ubuntu22.04 AS runtime
2026-Feb-25 08:25:19.833479
2026-Feb-25 08:25:19.833479
WORKDIR /app
2026-Feb-25 08:25:19.833479
2026-Feb-25 08:25:19.833479
# Install runtime dependencies
2026-Feb-25 08:25:19.833479
RUN apt-get update && apt-get install -y \
2026-Feb-25 08:25:19.833479
ca-certificates \
2026-Feb-25 08:25:19.833479
libssl3 \
2026-Feb-25 08:25:19.833479
ffmpeg \
2026-Feb-25 08:25:19.833479
libavcodec58 \
2026-Feb-25 08:25:19.833479
libavformat58 \
2026-Feb-25 08:25:19.833479
libavutil56 \
2026-Feb-25 08:25:19.833479
libswscale5 \
2026-Feb-25 08:25:19.833479
libavfilter7 \
2026-Feb-25 08:25:19.833479
libavdevice58 \
2026-Feb-25 08:25:19.833479
&& rm -rf /var/lib/apt/lists/*
2026-Feb-25 08:25:19.833479
2026-Feb-25 08:25:19.833479
# Create non-root user
2026-Feb-25 08:25:19.833479
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-25 08:25:19.833479
2026-Feb-25 08:25:19.833479
# Copy binary from builder
2026-Feb-25 08:25:19.833479
COPY --from=builder /app/target/release/gpu-node /usr/local/bin/
2026-Feb-25 08:25:19.833479
2026-Feb-25 08:25:19.833479
# Create directories
2026-Feb-25 08:25:19.833479
RUN mkdir -p /app/data && chown -R appuser:appuser /app
2026-Feb-25 08:25:19.833479
2026-Feb-25 08:25:19.833479
# Switch to non-root user
2026-Feb-25 08:25:19.833479
USER appuser
2026-Feb-25 08:25:19.833479
2026-Feb-25 08:25:19.833479
# Environment variables
2026-Feb-25 08:25:19.833479
ENV GPU_WORKER_BIND=0.0.0.0:50051
2026-Feb-25 08:25:19.833479
ENV GPU_WORKER_MAX_JOBS=4
2026-Feb-25 08:25:19.833479
ENV CUDA_DEVICE_ID=0
2026-Feb-25 08:25:19.833479
ENV RUST_LOG=info
2026-Feb-25 08:25:19.833479
2026-Feb-25 08:25:19.833479
# NVIDIA runtime configuration
2026-Feb-25 08:25:19.833479
ENV NVIDIA_VISIBLE_DEVICES=all
2026-Feb-25 08:25:19.833479
ENV NVIDIA_DRIVER_CAPABILITIES=compute,video,utility
2026-Feb-25 08:25:19.833479
2026-Feb-25 08:25:19.833479
# Expose gRPC port
2026-Feb-25 08:25:19.833479
EXPOSE 50051
2026-Feb-25 08:25:19.833479
2026-Feb-25 08:25:19.833479
# Health check
2026-Feb-25 08:25:19.833479
HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
2026-Feb-25 08:25:19.833479
CMD echo "health check placeholder" || exit 0
2026-Feb-25 08:25:19.833479
2026-Feb-25 08:25:19.833479
# Run the GPU worker
2026-Feb-25 08:25:19.833479
CMD ["gpu-node"]
2026-Feb-25 08:25:20.195315
Added 12 ARG declarations to Dockerfile for service gpu-worker (multi-stage build, added to 2 stages).
2026-Feb-25 08:25:20.548592
[CMD]: docker exec ac84ccs0ow8c0ksoocs0s4og bash -c 'test -f /artifacts/ac84ccs0ow8c0ksoocs0s4og/docker/Dockerfile.vps && echo 'exists' || echo 'not found''
2026-Feb-25 08:25:20.548592
exists
2026-Feb-25 08:25:20.916113
[CMD]: docker exec ac84ccs0ow8c0ksoocs0s4og bash -c 'cat /artifacts/ac84ccs0ow8c0ksoocs0s4og/docker/Dockerfile.vps'
2026-Feb-25 08:25:20.916113
# Dockerfile for VPS deployment
2026-Feb-25 08:25:20.916113
# Builds the API server and related components without GPU support
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
# Stage 0: Build injector JS (embedded into api crate via include_str! at compile time)
2026-Feb-25 08:25:20.916113
FROM node:22-alpine AS js-builder
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
WORKDIR /app
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
RUN npm install -g pnpm
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
# Copy workspace manifests for pnpm resolution
2026-Feb-25 08:25:20.916113
COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Feb-25 08:25:20.916113
COPY packages/api-client/package.json ./packages/api-client/
2026-Feb-25 08:25:20.916113
COPY apps/injector/package.json ./apps/injector/
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
# Copy injector source and shared packages
2026-Feb-25 08:25:20.916113
COPY apps/injector/ ./apps/injector/
2026-Feb-25 08:25:20.916113
COPY packages/ ./packages/
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
# Install deps and build injector (produces dist/bm.js and dist/youtube-downloader.user.js)
2026-Feb-25 08:25:20.916113
RUN pnpm install --frozen-lockfile
2026-Feb-25 08:25:20.916113
RUN pnpm --filter @downloadtool/injector build
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
# Stage 1: Rust builder
2026-Feb-25 08:25:20.916113
FROM rust:1.88-bookworm AS builder
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
WORKDIR /app
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
# Install dependencies
2026-Feb-25 08:25:20.916113
RUN apt-get update && apt-get install -y \
2026-Feb-25 08:25:20.916113
pkg-config \
2026-Feb-25 08:25:20.916113
libssl-dev \
2026-Feb-25 08:25:20.916113
protobuf-compiler \
2026-Feb-25 08:25:20.916113
&& rm -rf /var/lib/apt/lists/*
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
# Copy workspace configuration
2026-Feb-25 08:25:20.916113
COPY Cargo.toml ./
2026-Feb-25 08:25:20.916113
COPY Cargo.lock ./
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
# Copy all crates
2026-Feb-25 08:25:20.916113
COPY crates/ ./crates/
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
# Copy proto files
2026-Feb-25 08:25:20.916113
COPY proto/ ./proto/
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
# Copy injector dist (required by include_str! in crates/api/src/routes/static_files.rs)
2026-Feb-25 08:25:20.916113
COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
# Build the release binary
2026-Feb-25 08:25:20.916113
RUN cargo build --release --bin vps-gateway
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
# Stage 2: Runtime
2026-Feb-25 08:25:20.916113
FROM debian:bookworm-slim AS runtime
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
WORKDIR /app
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
# Install runtime dependencies
2026-Feb-25 08:25:20.916113
RUN apt-get update && apt-get install -y \
2026-Feb-25 08:25:20.916113
ca-certificates \
2026-Feb-25 08:25:20.916113
libssl3 \
2026-Feb-25 08:25:20.916113
&& rm -rf /var/lib/apt/lists/*
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
# Create non-root user
2026-Feb-25 08:25:20.916113
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
# Copy binary from builder
2026-Feb-25 08:25:20.916113
COPY --from=builder /app/target/release/vps-gateway /usr/local/bin/
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
# Create directories
2026-Feb-25 08:25:20.916113
RUN mkdir -p /app/extractors /app/data && chown -R appuser:appuser /app
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
# Switch to non-root user
2026-Feb-25 08:25:20.916113
USER appuser
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
# Environment variables
2026-Feb-25 08:25:20.916113
ENV PORT=3068
2026-Feb-25 08:25:20.916113
ENV EXTRACTOR_DIR=/app/extractors
2026-Feb-25 08:25:20.916113
ENV GPU_ENABLED=false
2026-Feb-25 08:25:20.916113
ENV RUST_LOG=info
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
# Expose port
2026-Feb-25 08:25:20.916113
EXPOSE 3068
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
# Health check
2026-Feb-25 08:25:20.916113
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Feb-25 08:25:20.916113
CMD curl -f http://localhost:3068/health || exit 1
2026-Feb-25 08:25:20.916113
2026-Feb-25 08:25:20.916113
# Run the server
2026-Feb-25 08:25:20.916113
CMD ["vps-gateway"]
2026-Feb-25 08:25:21.281012
Added 18 ARG declarations to Dockerfile for service api (multi-stage build, added to 3 stages).
2026-Feb-25 08:25:21.637507
[CMD]: docker exec ac84ccs0ow8c0ksoocs0s4og bash -c 'test -f /artifacts/ac84ccs0ow8c0ksoocs0s4og/docker/Dockerfile.frontend && echo 'exists' || echo 'not found''
2026-Feb-25 08:25:21.637507
exists
2026-Feb-25 08:25:21.997370
[CMD]: docker exec ac84ccs0ow8c0ksoocs0s4og bash -c 'cat /artifacts/ac84ccs0ow8c0ksoocs0s4og/docker/Dockerfile.frontend'
2026-Feb-25 08:25:21.997370
# Dockerfile for frontend (SvelteKit Node server)
2026-Feb-25 08:25:21.997370
# Copy ALL source files BEFORE npm install so svelte-kit sync (prepare script)
2026-Feb-25 08:25:21.997370
# can find svelte.config.js and generate .svelte-kit/ correctly.
2026-Feb-25 08:25:21.997370
2026-Feb-25 08:25:21.997370
FROM node:22-alpine AS builder
2026-Feb-25 08:25:21.997370
2026-Feb-25 08:25:21.997370
WORKDIR /app
2026-Feb-25 08:25:21.997370
2026-Feb-25 08:25:21.997370
# Copy all frontend source files first (node_modules excluded via .dockerignore)
2026-Feb-25 08:25:21.997370
COPY frontend/ ./
2026-Feb-25 08:25:21.997370
2026-Feb-25 08:25:21.997370
# Install — prepare script runs svelte-kit sync with svelte.config.js available
2026-Feb-25 08:25:21.997370
RUN npm install
2026-Feb-25 08:25:21.997370
2026-Feb-25 08:25:21.997370
# Build via programmatic API — bypasses vite.config.ts loading (esbuild issues on Alpine)
2026-Feb-25 08:25:21.997370
RUN node build-docker.mjs
2026-Feb-25 08:25:21.997370
2026-Feb-25 08:25:21.997370
# Runtime
2026-Feb-25 08:25:21.997370
FROM node:22-alpine AS runtime
2026-Feb-25 08:25:21.997370
2026-Feb-25 08:25:21.997370
WORKDIR /app
2026-Feb-25 08:25:21.997370
2026-Feb-25 08:25:21.997370
COPY --from=builder /app/build ./build
2026-Feb-25 08:25:21.997370
COPY --from=builder /app/package.json ./
2026-Feb-25 08:25:21.997370
2026-Feb-25 08:25:21.997370
ENV PORT=3000
2026-Feb-25 08:25:21.997370
ENV HOST=0.0.0.0
2026-Feb-25 08:25:21.997370
2026-Feb-25 08:25:21.997370
EXPOSE 3000
2026-Feb-25 08:25:21.997370
2026-Feb-25 08:25:21.997370
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Feb-25 08:25:21.997370
CMD wget -qO- http://localhost:3000 || exit 1
2026-Feb-25 08:25:21.997370
2026-Feb-25 08:25:21.997370
CMD ["node", "build"]
2026-Feb-25 08:25:22.347295
Added 12 ARG declarations to Dockerfile for service frontend (multi-stage build, added to 2 stages).
2026-Feb-25 08:25:22.351903
Pulling & building required images.
2026-Feb-25 08:25:22.358894
Creating build-time .env file in /artifacts (outside Docker context).
2026-Feb-25 08:25:22.719621
Adding build arguments to Docker Compose build command.
2026-Feb-25 08:25:23.200233
[CMD]: docker exec ac84ccs0ow8c0ksoocs0s4og bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/build-time.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/ac84ccs0ow8c0ksoocs0s4og -f /artifacts/ac84ccs0ow8c0ksoocs0s4og/docker/docker-compose.homeserver.yml build --pull --build-arg COOLIFY_URL --build-arg COOLIFY_FQDN --build-arg SERVICE_URL_FRONTEND --build-arg SERVICE_FQDN_FRONTEND --build-arg SERVICE_URL_API --build-arg SERVICE_FQDN_API --build-arg COOLIFY_BUILD_SECRETS_HASH=016dc4226f6c53eea50a0f46c58a295ad358d7f449966f8f46fd81293803bce6'
2026-Feb-25 08:25:23.200233
#1 [internal] load local bake definitions
2026-Feb-25 08:25:23.412006
#1 reading from stdin 2.57kB done
2026-Feb-25 08:25:23.412006
#1 DONE 0.0s
2026-Feb-25 08:25:23.412006
2026-Feb-25 08:25:23.412006
#2 [frontend internal] load build definition from Dockerfile.frontend
2026-Feb-25 08:25:23.412006
#2 transferring dockerfile: 1.18kB done
2026-Feb-25 08:25:23.412006
#2 DONE 0.0s
2026-Feb-25 08:25:23.412006
2026-Feb-25 08:25:23.412006
#3 [api internal] load build definition from Dockerfile.vps
2026-Feb-25 08:25:23.412006
#3 transferring dockerfile: 2.63kB done
2026-Feb-25 08:25:23.412006
#3 DONE 0.0s
2026-Feb-25 08:25:23.412006
2026-Feb-25 08:25:23.412006
#4 [gpu-worker internal] load build definition from Dockerfile.homeserver
2026-Feb-25 08:25:23.412006
#4 transferring dockerfile: 2.46kB done
2026-Feb-25 08:25:23.412006
#4 DONE 0.0s
2026-Feb-25 08:25:23.412006
2026-Feb-25 08:25:23.412006
#5 [api internal] load metadata for docker.io/library/node:22-alpine
2026-Feb-25 08:25:24.635901
#5 ...
2026-Feb-25 08:25:24.635901
2026-Feb-25 08:25:24.635901
#6 [gpu-worker internal] load metadata for docker.io/nvidia/cuda:12.3.2-devel-ubuntu22.04
2026-Feb-25 08:25:24.635901
#6 DONE 1.4s
2026-Feb-25 08:25:24.740692
#7 [api internal] load metadata for docker.io/library/rust:1.88-bookworm
2026-Feb-25 08:25:24.740692
#7 DONE 1.4s
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#8 [gpu-worker internal] load metadata for docker.io/nvidia/cuda:12.3.2-runtime-ubuntu22.04
2026-Feb-25 08:25:24.740692
#8 DONE 1.4s
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#9 [api internal] load .dockerignore
2026-Feb-25 08:25:24.740692
#9 transferring context: 341B done
2026-Feb-25 08:25:24.740692
#9 DONE 0.0s
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#10 [api internal] load metadata for docker.io/library/debian:bookworm-slim
2026-Feb-25 08:25:24.740692
#10 DONE 1.4s
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#5 [api internal] load metadata for docker.io/library/node:22-alpine
2026-Feb-25 08:25:24.740692
#5 DONE 1.4s
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#11 [gpu-worker builder 1/9] FROM docker.io/nvidia/cuda:12.3.2-devel-ubuntu22.04@sha256:6655d5fc2fb48580255a5021a81c379c325a457b74b77ac823ed67e4faa32aeb
2026-Feb-25 08:25:24.740692
#11 resolve docker.io/nvidia/cuda:12.3.2-devel-ubuntu22.04@sha256:6655d5fc2fb48580255a5021a81c379c325a457b74b77ac823ed67e4faa32aeb 0.0s done
2026-Feb-25 08:25:24.740692
#11 DONE 0.0s
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#12 [gpu-worker runtime 1/6] FROM docker.io/nvidia/cuda:12.3.2-runtime-ubuntu22.04@sha256:882b43fadd789693f08de95e6014ed5f0ea118c7b342150876e153b4340e1103
2026-Feb-25 08:25:24.740692
#12 resolve docker.io/nvidia/cuda:12.3.2-runtime-ubuntu22.04@sha256:882b43fadd789693f08de95e6014ed5f0ea118c7b342150876e153b4340e1103 0.0s done
2026-Feb-25 08:25:24.740692
#12 DONE 0.0s
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#13 [api builder 1/9] FROM docker.io/library/rust:1.88-bookworm@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0
2026-Feb-25 08:25:24.740692
#13 resolve docker.io/library/rust:1.88-bookworm@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0 0.0s done
2026-Feb-25 08:25:24.740692
#13 DONE 0.0s
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#14 [api runtime 1/6] FROM docker.io/library/debian:bookworm-slim@sha256:74d56e3931e0d5a1dd51f8c8a2466d21de84a271cd3b5a733b803aa91abf4421
2026-Feb-25 08:25:24.740692
#14 resolve docker.io/library/debian:bookworm-slim@sha256:74d56e3931e0d5a1dd51f8c8a2466d21de84a271cd3b5a733b803aa91abf4421 0.0s done
2026-Feb-25 08:25:24.740692
#14 DONE 0.0s
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#15 [frontend builder 1/5] FROM docker.io/library/node:22-alpine@sha256:e4bf2a82ad0a4037d28035ae71529873c069b13eb0455466ae0bc13363826e34
2026-Feb-25 08:25:24.740692
#15 resolve docker.io/library/node:22-alpine@sha256:e4bf2a82ad0a4037d28035ae71529873c069b13eb0455466ae0bc13363826e34 0.0s done
2026-Feb-25 08:25:24.740692
#15 DONE 0.0s
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#16 [gpu-worker internal] load build context
2026-Feb-25 08:25:24.740692
#16 transferring context: 475.95kB 0.0s done
2026-Feb-25 08:25:24.740692
#16 DONE 0.0s
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#17 [gpu-worker runtime 3/6] RUN apt-get update && apt-get install -y     ca-certificates     libssl3     ffmpeg     libavcodec58     libavformat58     libavutil56     libswscale5     libavfilter7     libavdevice58     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 08:25:24.740692
#17 CACHED
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#18 [gpu-worker runtime 5/6] COPY --from=builder /app/target/release/gpu-node /usr/local/bin/
2026-Feb-25 08:25:24.740692
#18 CACHED
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#19 [gpu-worker builder 5/9] COPY Cargo.toml ./
2026-Feb-25 08:25:24.740692
#19 CACHED
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#20 [gpu-worker runtime 2/6] WORKDIR /app
2026-Feb-25 08:25:24.740692
#20 CACHED
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#21 [gpu-worker builder 9/9] RUN cargo build --release --bin gpu-node
2026-Feb-25 08:25:24.740692
#21 CACHED
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#22 [gpu-worker builder 3/9] RUN apt-get update && apt-get install -y     curl     build-essential     pkg-config     libssl-dev     protobuf-compiler     ffmpeg     libavcodec-dev     libavformat-dev     libavutil-dev     libswscale-dev     libavfilter-dev     libavdevice-dev     clang     libclang-dev     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 08:25:24.740692
#22 CACHED
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#23 [gpu-worker builder 2/9] WORKDIR /app
2026-Feb-25 08:25:24.740692
#23 CACHED
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#24 [gpu-worker builder 8/9] COPY proto/ ./proto/
2026-Feb-25 08:25:24.740692
#24 CACHED
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#25 [gpu-worker builder 7/9] COPY crates/ ./crates/
2026-Feb-25 08:25:24.740692
#25 CACHED
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#26 [gpu-worker builder 6/9] COPY Cargo.lock ./
2026-Feb-25 08:25:24.740692
#26 CACHED
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#27 [gpu-worker builder 4/9] RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
2026-Feb-25 08:25:24.740692
#27 CACHED
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#28 [gpu-worker runtime 4/6] RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-25 08:25:24.740692
#28 CACHED
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#29 [gpu-worker runtime 6/6] RUN mkdir -p /app/data && chown -R appuser:appuser /app
2026-Feb-25 08:25:24.740692
#29 CACHED
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#30 [frontend internal] load build context
2026-Feb-25 08:25:24.740692
#30 transferring context: 197.18kB done
2026-Feb-25 08:25:24.740692
#30 DONE 0.0s
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#31 [frontend builder 4/5] RUN npm install
2026-Feb-25 08:25:24.740692
#31 CACHED
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#32 [frontend builder 3/5] COPY frontend/ ./
2026-Feb-25 08:25:24.740692
#32 CACHED
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#33 [frontend runtime 3/4] COPY --from=builder /app/build ./build
2026-Feb-25 08:25:24.740692
#33 CACHED
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#34 [frontend builder 5/5] RUN node build-docker.mjs
2026-Feb-25 08:25:24.740692
#34 CACHED
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#35 [api internal] load build context
2026-Feb-25 08:25:24.740692
#35 transferring context: 600.49kB 0.0s done
2026-Feb-25 08:25:24.740692
#35 DONE 0.0s
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#36 [frontend runtime 4/4] COPY --from=builder /app/package.json ./
2026-Feb-25 08:25:24.740692
#36 CACHED
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#37 [api js-builder  5/10] COPY packages/api-client/package.json ./packages/api-client/
2026-Feb-25 08:25:24.740692
#37 CACHED
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#38 [api js-builder 10/10] RUN pnpm --filter @downloadtool/injector build
2026-Feb-25 08:25:24.740692
#38 CACHED
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#39 [api builder 8/9] COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Feb-25 08:25:24.740692
#39 CACHED
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#40 [api builder 6/9] COPY crates/ ./crates/
2026-Feb-25 08:25:24.740692
#40 CACHED
2026-Feb-25 08:25:24.740692
2026-Feb-25 08:25:24.740692
#41 [api runtime 5/6] COPY --from=builder /app/target/release/vps-gateway /usr/local/bin/
2026-Feb-25 08:25:24.740692
#41 CACHED
2026-Feb-25 08:25:24.742798
#42 [api builder 9/9] RUN cargo build --release --bin vps-gateway
2026-Feb-25 08:25:24.742798
#42 CACHED
2026-Feb-25 08:25:24.742798
2026-Feb-25 08:25:24.742798
#43 [api js-builder  4/10] COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Feb-25 08:25:24.742798
#43 CACHED
2026-Feb-25 08:25:24.742798
2026-Feb-25 08:25:24.742798
#44 [api builder 7/9] COPY proto/ ./proto/
2026-Feb-25 08:25:24.742798
#44 CACHED
2026-Feb-25 08:25:24.742798
2026-Feb-25 08:25:24.742798
#45 [api js-builder  7/10] COPY apps/injector/ ./apps/injector/
2026-Feb-25 08:25:24.742798
#45 CACHED
2026-Feb-25 08:25:24.742798
2026-Feb-25 08:25:24.742798
#46 [api builder 4/9] COPY Cargo.toml ./
2026-Feb-25 08:25:24.742798
#46 CACHED
2026-Feb-25 08:25:24.742798
2026-Feb-25 08:25:24.742798
#47 [api js-builder  3/10] RUN npm install -g pnpm
2026-Feb-25 08:25:24.742798
#47 CACHED
2026-Feb-25 08:25:24.742798
2026-Feb-25 08:25:24.742798
#48 [api runtime 4/6] RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-25 08:25:24.742798
#48 CACHED
2026-Feb-25 08:25:24.742798
2026-Feb-25 08:25:24.742798
#49 [api builder 2/5] WORKDIR /app
2026-Feb-25 08:25:24.742798
#49 CACHED
2026-Feb-25 08:25:24.742798
2026-Feb-25 08:25:24.742798
#50 [api js-builder  8/10] COPY packages/ ./packages/
2026-Feb-25 08:25:24.742798
#50 CACHED
2026-Feb-25 08:25:24.742798
2026-Feb-25 08:25:24.742798
#51 [api builder 3/9] RUN apt-get update && apt-get install -y     pkg-config     libssl-dev     protobuf-compiler     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 08:25:24.742798
#51 CACHED
2026-Feb-25 08:25:24.742798
2026-Feb-25 08:25:24.742798
#52 [api js-builder  9/10] RUN pnpm install --frozen-lockfile
2026-Feb-25 08:25:24.742798
#52 CACHED
2026-Feb-25 08:25:24.742798
2026-Feb-25 08:25:24.742798
#53 [api js-builder  6/10] COPY apps/injector/package.json ./apps/injector/
2026-Feb-25 08:25:24.742798
#53 CACHED
2026-Feb-25 08:25:24.742798
2026-Feb-25 08:25:24.742798
#54 [api runtime 3/6] RUN apt-get update && apt-get install -y     ca-certificates     libssl3     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 08:25:24.742798
#54 CACHED
2026-Feb-25 08:25:24.742798
2026-Feb-25 08:25:24.742798
#55 [api runtime 2/6] WORKDIR /app
2026-Feb-25 08:25:24.742798
#55 CACHED
2026-Feb-25 08:25:24.742798
2026-Feb-25 08:25:24.742798
#56 [api builder 5/9] COPY Cargo.lock ./
2026-Feb-25 08:25:24.742798
#56 CACHED
2026-Feb-25 08:25:24.742798
2026-Feb-25 08:25:24.742798
#57 [api builder 2/9] WORKDIR /app
2026-Feb-25 08:25:24.742798
#57 CACHED
2026-Feb-25 08:25:24.742798
2026-Feb-25 08:25:24.742798
#58 [api runtime 6/6] RUN mkdir -p /app/extractors /app/data && chown -R appuser:appuser /app
2026-Feb-25 08:25:24.742798
#58 CACHED
2026-Feb-25 08:25:24.742798
2026-Feb-25 08:25:24.742798
#59 [api] exporting to image
2026-Feb-25 08:25:24.742798
#59 exporting layers done
2026-Feb-25 08:25:24.742798
#59 exporting manifest sha256:807e72a1d2039a43b058f99d5ce2aa8377504fa73231a165d3e9f06e309bf18a done
2026-Feb-25 08:25:24.742798
#59 exporting config sha256:df444ec535dcffc05b00ad425a1b6ba5949a8b7e69addb8957f8e6925ec59f3b
2026-Feb-25 08:25:24.844844
#59 exporting config sha256:df444ec535dcffc05b00ad425a1b6ba5949a8b7e69addb8957f8e6925ec59f3b done
2026-Feb-25 08:25:24.844844
#59 exporting attestation manifest sha256:002e6d19c0030fe42483babeeb15ce3d739c9ebd6def3422ec414c9f860598e2 0.0s done
2026-Feb-25 08:25:24.844844
#59 exporting manifest list sha256:36f9bc0bda6c08ee2a0f0b1adadda475ec7e90c498349d32cbac05c85a95a253 done
2026-Feb-25 08:25:24.844844
#59 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_api:7018562fe60a5232a865dbda115b8e79c33092c6 done
2026-Feb-25 08:25:24.844844
#59 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_api:7018562fe60a5232a865dbda115b8e79c33092c6 done
2026-Feb-25 08:25:24.844844
#59 DONE 0.1s
2026-Feb-25 08:25:24.844844
2026-Feb-25 08:25:24.844844
#60 [gpu-worker] exporting to image
2026-Feb-25 08:25:24.844844
#60 exporting layers done
2026-Feb-25 08:25:24.844844
#60 exporting manifest sha256:a204de5eee386b669cf4329535afe2c999a1ec201f28d7a4b46a672ee0a956ee done
2026-Feb-25 08:25:24.844844
#60 exporting config sha256:f1807f5bec10c85eef67afff233db8efe85a4b9cc0f27a4d5994217dc598a4ad done
2026-Feb-25 08:25:24.844844
#60 exporting attestation manifest sha256:518a94f82eb8dd0a84afce8d2037ee17256e8b814a7bb415ecc07cf9a8a580e3 0.0s done
2026-Feb-25 08:25:24.844844
#60 exporting manifest list sha256:77d47f28bd0941200f05e928149293ebf1dfb3f04e645ba392c04a79c3e50a6f done
2026-Feb-25 08:25:24.844844
#60 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_gpu-worker:7018562fe60a5232a865dbda115b8e79c33092c6 done
2026-Feb-25 08:25:24.844844
#60 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_gpu-worker:7018562fe60a5232a865dbda115b8e79c33092c6 0.0s done
2026-Feb-25 08:25:24.844844
#60 DONE 0.1s
2026-Feb-25 08:25:24.844844
2026-Feb-25 08:25:24.844844
#61 [frontend] exporting to image
2026-Feb-25 08:25:24.844844
#61 exporting layers done
2026-Feb-25 08:25:24.844844
#61 exporting manifest sha256:af59e93e2d76249fe8e48f3b78a4c2996a0c8ec14950095b41355eacd04169f9 done
2026-Feb-25 08:25:24.844844
#61 exporting config sha256:9b744fd2ea03750445160c48f7ef79e7360b74c71b03aa041d864d27bc695e5b done
2026-Feb-25 08:25:24.844844
#61 exporting attestation manifest sha256:555982e0c9515420d8c340d5555dcffd1502b54f7a45f4358b2f8616912c208f 0.0s done
2026-Feb-25 08:25:24.844844
#61 exporting manifest list sha256:304baaacc2356f7e7e9576287593a1b365c69aee9a516a3076d57e5e8b2bc9b1 done
2026-Feb-25 08:25:24.844844
#61 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:7018562fe60a5232a865dbda115b8e79c33092c6 done
2026-Feb-25 08:25:24.844844
#61 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:7018562fe60a5232a865dbda115b8e79c33092c6 done
2026-Feb-25 08:25:24.844844
#61 DONE 0.1s
2026-Feb-25 08:25:24.844844
2026-Feb-25 08:25:24.844844
#62 [frontend] resolving provenance for metadata file
2026-Feb-25 08:25:24.877540
#62 DONE 0.0s
2026-Feb-25 08:25:24.877540
2026-Feb-25 08:25:24.877540
#63 [gpu-worker] resolving provenance for metadata file
2026-Feb-25 08:25:24.877540
#63 DONE 0.0s
2026-Feb-25 08:25:24.877540
2026-Feb-25 08:25:24.877540
#64 [api] resolving provenance for metadata file
2026-Feb-25 08:25:24.877540
#64 DONE 0.0s
2026-Feb-25 08:25:24.879824
frontend  Built
2026-Feb-25 08:25:24.879824
gpu-worker  Built
2026-Feb-25 08:25:24.879824
api  Built
2026-Feb-25 08:25:24.892003
Creating .env file with runtime variables for container.
2026-Feb-25 08:25:25.541181
Removing old containers.
2026-Feb-25 08:25:26.407749
[CMD]: docker stop -t 30 frontend-o8kccgkgwsockoocow8sg88s-080804114333
2026-Feb-25 08:25:26.407749
frontend-o8kccgkgwsockoocow8sg88s-080804114333
2026-Feb-25 08:25:26.736419
[CMD]: docker rm -f frontend-o8kccgkgwsockoocow8sg88s-080804114333
2026-Feb-25 08:25:26.736419
frontend-o8kccgkgwsockoocow8sg88s-080804114333
2026-Feb-25 08:25:57.672390
[CMD]: docker stop -t 30 api-o8kccgkgwsockoocow8sg88s-080804110048
2026-Feb-25 08:25:57.672390
api-o8kccgkgwsockoocow8sg88s-080804110048
2026-Feb-25 08:25:58.042529
[CMD]: docker rm -f api-o8kccgkgwsockoocow8sg88s-080804110048
2026-Feb-25 08:25:58.042529
api-o8kccgkgwsockoocow8sg88s-080804110048
2026-Feb-25 08:26:28.612056
[CMD]: docker stop -t 30 gpu-worker-o8kccgkgwsockoocow8sg88s-080804103893
2026-Feb-25 08:26:28.612056
gpu-worker-o8kccgkgwsockoocow8sg88s-080804103893
2026-Feb-25 08:26:29.125981
[CMD]: docker rm -f gpu-worker-o8kccgkgwsockoocow8sg88s-080804103893
2026-Feb-25 08:26:29.125981
gpu-worker-o8kccgkgwsockoocow8sg88s-080804103893
2026-Feb-25 08:26:29.131054
Starting new application.
2026-Feb-25 08:26:30.173781
[CMD]: docker exec ac84ccs0ow8c0ksoocs0s4og bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/ac84ccs0ow8c0ksoocs0s4og/.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/ac84ccs0ow8c0ksoocs0s4og -f /artifacts/ac84ccs0ow8c0ksoocs0s4og/docker/docker-compose.homeserver.yml up -d'
2026-Feb-25 08:26:30.173781
Container gpu-worker-o8kccgkgwsockoocow8sg88s-082518742675  Creating
2026-Feb-25 08:26:30.216306
Container gpu-worker-o8kccgkgwsockoocow8sg88s-082518742675  Created
2026-Feb-25 08:26:30.216306
Container api-o8kccgkgwsockoocow8sg88s-082518747949  Creating
2026-Feb-25 08:26:30.242745
Container api-o8kccgkgwsockoocow8sg88s-082518747949  Created
2026-Feb-25 08:26:30.242745
Container frontend-o8kccgkgwsockoocow8sg88s-082518752729  Creating
2026-Feb-25 08:26:30.495644
Container frontend-o8kccgkgwsockoocow8sg88s-082518752729  Created
2026-Feb-25 08:26:30.504037
Container gpu-worker-o8kccgkgwsockoocow8sg88s-082518742675  Starting
2026-Feb-25 08:26:30.740245
Container gpu-worker-o8kccgkgwsockoocow8sg88s-082518742675  Started
2026-Feb-25 08:26:30.740245
Container api-o8kccgkgwsockoocow8sg88s-082518747949  Starting
2026-Feb-25 08:26:30.905543
Container api-o8kccgkgwsockoocow8sg88s-082518747949  Started
2026-Feb-25 08:26:30.905543
Container frontend-o8kccgkgwsockoocow8sg88s-082518752729  Starting
2026-Feb-25 08:26:31.058311
Container frontend-o8kccgkgwsockoocow8sg88s-082518752729  Started
2026-Feb-25 08:26:31.972867
New container started.
2026-Feb-25 08:26:32.900184
Gracefully shutting down build container: ac84ccs0ow8c0ksoocs0s4og
2026-Feb-25 08:26:33.361653
[CMD]: docker stop -t 30 ac84ccs0ow8c0ksoocs0s4og
2026-Feb-25 08:26:33.361653
ac84ccs0ow8c0ksoocs0s4og