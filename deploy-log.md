2026-Feb-25 02:49:55.538063
Starting deployment of khoa280703/downloadtool:main-zoscg4oc04gkwkssg0kw8w8w to localhost.
2026-Feb-25 02:49:55.713943
Preparing container with helper image: ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Feb-25 02:49:55.811321
[CMD]: docker stop -t 30 wcwwgk4g8o0owkwogkgo40kk
2026-Feb-25 02:49:55.811321
Error response from daemon: No such container: wcwwgk4g8o0owkwogkgo40kk
2026-Feb-25 02:49:55.935619
[CMD]: docker run -d --network coolify --name wcwwgk4g8o0owkwogkgo40kk  --rm -v /var/run/docker.sock:/var/run/docker.sock ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Feb-25 02:49:55.935619
5a19af4810f2032d679844e5b7fbc1be037620dab8fc86fd575180691474ff27
2026-Feb-25 02:49:57.137578
[CMD]: docker exec wcwwgk4g8o0owkwogkgo40kk bash -c 'GIT_SSH_COMMAND="ssh -o ConnectTimeout=30 -p 22 -o Port=22 -o LogLevel=ERROR -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git ls-remote https://github.com/Khoa280703/downloadtool refs/heads/main'
2026-Feb-25 02:49:57.137578
b192ae1e024a4b21b7fa6522678ec167fd180c22	refs/heads/main
2026-Feb-25 02:49:57.147953
----------------------------------------
2026-Feb-25 02:49:57.151819
Importing Khoa280703/downloadtool:main (commit sha b192ae1e024a4b21b7fa6522678ec167fd180c22) to /artifacts/wcwwgk4g8o0owkwogkgo40kk.
2026-Feb-25 02:49:57.292010
[CMD]: docker exec wcwwgk4g8o0owkwogkgo40kk bash -c 'git clone --depth=1 --recurse-submodules --shallow-submodules -b 'main' 'https://github.com/Khoa280703/downloadtool' '/artifacts/wcwwgk4g8o0owkwogkgo40kk' && cd '/artifacts/wcwwgk4g8o0owkwogkgo40kk' && if [ -f .gitmodules ]; then sed -i "s#git@\(.*\):#https://\1/#g" '/artifacts/wcwwgk4g8o0owkwogkgo40kk'/.gitmodules || true && git submodule sync && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git submodule update --init --recursive --depth=1; fi && cd '/artifacts/wcwwgk4g8o0owkwogkgo40kk' && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git lfs pull'
2026-Feb-25 02:49:57.292010
Cloning into '/artifacts/wcwwgk4g8o0owkwogkgo40kk'...
2026-Feb-25 02:49:59.131170
[CMD]: docker exec wcwwgk4g8o0owkwogkgo40kk bash -c 'cd /artifacts/wcwwgk4g8o0owkwogkgo40kk && git log -1 b192ae1e024a4b21b7fa6522678ec167fd180c22 --pretty=%B'
2026-Feb-25 02:49:59.131170
fix: bypass vite config file loading with programmatic build script
2026-Feb-25 02:49:59.131170
2026-Feb-25 02:49:59.131170
vite uses esbuild to bundle vite.config.ts before importing it. On Alpine
2026-Feb-25 02:49:59.131170
Docker (musl libc), esbuild may fail silently causing vite to use empty
2026-Feb-25 02:49:59.131170
config (no plugins) → SvelteKit plugin never runs → '0 modules transformed'.
2026-Feb-25 02:49:59.131170
2026-Feb-25 02:49:59.131170
Solution: build-docker.mjs imports vite + sveltekit() directly from
2026-Feb-25 02:49:59.131170
node_modules at runtime — no esbuild config bundling, no temp file,
2026-Feb-25 02:49:59.131170
configFile: false. SvelteKit plugin still reads svelte.config.js normally.
2026-Feb-25 02:50:02.204286
[CMD]: docker exec wcwwgk4g8o0owkwogkgo40kk bash -c 'test -f /artifacts/wcwwgk4g8o0owkwogkgo40kk/docker/Dockerfile.homeserver && echo 'exists' || echo 'not found''
2026-Feb-25 02:50:02.204286
exists
2026-Feb-25 02:50:02.355638
[CMD]: docker exec wcwwgk4g8o0owkwogkgo40kk bash -c 'cat /artifacts/wcwwgk4g8o0owkwogkgo40kk/docker/Dockerfile.homeserver'
2026-Feb-25 02:50:02.355638
# Dockerfile for Home Server deployment
2026-Feb-25 02:50:02.355638
# Builds the GPU worker with CUDA support for hardware transcoding
2026-Feb-25 02:50:02.355638
2026-Feb-25 02:50:02.355638
FROM nvidia/cuda:12.3.2-devel-ubuntu22.04 AS builder
2026-Feb-25 02:50:02.355638
2026-Feb-25 02:50:02.355638
WORKDIR /app
2026-Feb-25 02:50:02.355638
2026-Feb-25 02:50:02.355638
# Install dependencies
2026-Feb-25 02:50:02.355638
RUN apt-get update && apt-get install -y \
2026-Feb-25 02:50:02.355638
curl \
2026-Feb-25 02:50:02.355638
build-essential \
2026-Feb-25 02:50:02.355638
pkg-config \
2026-Feb-25 02:50:02.355638
libssl-dev \
2026-Feb-25 02:50:02.355638
protobuf-compiler \
2026-Feb-25 02:50:02.355638
ffmpeg \
2026-Feb-25 02:50:02.355638
libavcodec-dev \
2026-Feb-25 02:50:02.355638
libavformat-dev \
2026-Feb-25 02:50:02.355638
libavutil-dev \
2026-Feb-25 02:50:02.355638
libswscale-dev \
2026-Feb-25 02:50:02.355638
&& rm -rf /var/lib/apt/lists/*
2026-Feb-25 02:50:02.355638
2026-Feb-25 02:50:02.355638
# Install Rust
2026-Feb-25 02:50:02.355638
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
2026-Feb-25 02:50:02.355638
ENV PATH="/root/.cargo/bin:${PATH}"
2026-Feb-25 02:50:02.355638
2026-Feb-25 02:50:02.355638
# Copy workspace configuration
2026-Feb-25 02:50:02.355638
COPY Cargo.toml ./
2026-Feb-25 02:50:02.355638
COPY Cargo.lock ./
2026-Feb-25 02:50:02.355638
2026-Feb-25 02:50:02.355638
# Copy all crates
2026-Feb-25 02:50:02.355638
COPY crates/ ./crates/
2026-Feb-25 02:50:02.355638
2026-Feb-25 02:50:02.355638
# Copy proto files
2026-Feb-25 02:50:02.355638
COPY proto/ ./proto/
2026-Feb-25 02:50:02.355638
2026-Feb-25 02:50:02.355638
# Build the GPU worker with CUDA support
2026-Feb-25 02:50:02.355638
RUN cargo build --release --bin gpu-worker-server --features gpu-support
2026-Feb-25 02:50:02.355638
2026-Feb-25 02:50:02.355638
# Stage 2: Runtime
2026-Feb-25 02:50:02.355638
FROM nvidia/cuda:12.3.2-runtime-ubuntu22.04 AS runtime
2026-Feb-25 02:50:02.355638
2026-Feb-25 02:50:02.355638
WORKDIR /app
2026-Feb-25 02:50:02.355638
2026-Feb-25 02:50:02.355638
# Install runtime dependencies
2026-Feb-25 02:50:02.355638
RUN apt-get update && apt-get install -y \
2026-Feb-25 02:50:02.355638
ca-certificates \
2026-Feb-25 02:50:02.355638
libssl3 \
2026-Feb-25 02:50:02.355638
ffmpeg \
2026-Feb-25 02:50:02.355638
libavcodec58 \
2026-Feb-25 02:50:02.355638
libavformat58 \
2026-Feb-25 02:50:02.355638
libavutil56 \
2026-Feb-25 02:50:02.355638
libswscale5 \
2026-Feb-25 02:50:02.355638
&& rm -rf /var/lib/apt/lists/*
2026-Feb-25 02:50:02.355638
2026-Feb-25 02:50:02.355638
# Create non-root user
2026-Feb-25 02:50:02.355638
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-25 02:50:02.355638
2026-Feb-25 02:50:02.355638
# Copy binary from builder
2026-Feb-25 02:50:02.355638
COPY --from=builder /app/target/release/gpu-worker-server /usr/local/bin/
2026-Feb-25 02:50:02.355638
2026-Feb-25 02:50:02.355638
# Create directories
2026-Feb-25 02:50:02.355638
RUN mkdir -p /app/data && chown -R appuser:appuser /app
2026-Feb-25 02:50:02.355638
2026-Feb-25 02:50:02.355638
# Switch to non-root user
2026-Feb-25 02:50:02.355638
USER appuser
2026-Feb-25 02:50:02.355638
2026-Feb-25 02:50:02.355638
# Environment variables
2026-Feb-25 02:50:02.355638
ENV GPU_WORKER_BIND=0.0.0.0:50051
2026-Feb-25 02:50:02.355638
ENV GPU_WORKER_MAX_JOBS=4
2026-Feb-25 02:50:02.355638
ENV CUDA_DEVICE_ID=0
2026-Feb-25 02:50:02.355638
ENV RUST_LOG=info
2026-Feb-25 02:50:02.355638
2026-Feb-25 02:50:02.355638
# NVIDIA runtime configuration
2026-Feb-25 02:50:02.355638
ENV NVIDIA_VISIBLE_DEVICES=all
2026-Feb-25 02:50:02.355638
ENV NVIDIA_DRIVER_CAPABILITIES=compute,video,utility
2026-Feb-25 02:50:02.355638
2026-Feb-25 02:50:02.355638
# Expose gRPC port
2026-Feb-25 02:50:02.355638
EXPOSE 50051
2026-Feb-25 02:50:02.355638
2026-Feb-25 02:50:02.355638
# Health check
2026-Feb-25 02:50:02.355638
HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
2026-Feb-25 02:50:02.355638
CMD echo "health check placeholder" || exit 0
2026-Feb-25 02:50:02.355638
2026-Feb-25 02:50:02.355638
# Run the GPU worker
2026-Feb-25 02:50:02.355638
CMD ["gpu-worker-server"]
2026-Feb-25 02:50:02.513693
Added 4 ARG declarations to Dockerfile for service gpu-worker (multi-stage build, added to 2 stages).
2026-Feb-25 02:50:02.662413
[CMD]: docker exec wcwwgk4g8o0owkwogkgo40kk bash -c 'test -f /artifacts/wcwwgk4g8o0owkwogkgo40kk/docker/Dockerfile.vps && echo 'exists' || echo 'not found''
2026-Feb-25 02:50:02.662413
exists
2026-Feb-25 02:50:02.804304
[CMD]: docker exec wcwwgk4g8o0owkwogkgo40kk bash -c 'cat /artifacts/wcwwgk4g8o0owkwogkgo40kk/docker/Dockerfile.vps'
2026-Feb-25 02:50:02.804304
# Dockerfile for VPS deployment
2026-Feb-25 02:50:02.804304
# Builds the API server and related components without GPU support
2026-Feb-25 02:50:02.804304
2026-Feb-25 02:50:02.804304
# Stage 1: Builder
2026-Feb-25 02:50:02.804304
FROM rust:1.85-bookworm AS builder
2026-Feb-25 02:50:02.804304
2026-Feb-25 02:50:02.804304
WORKDIR /app
2026-Feb-25 02:50:02.804304
2026-Feb-25 02:50:02.804304
# Install dependencies
2026-Feb-25 02:50:02.804304
RUN apt-get update && apt-get install -y \
2026-Feb-25 02:50:02.804304
pkg-config \
2026-Feb-25 02:50:02.804304
libssl-dev \
2026-Feb-25 02:50:02.804304
protobuf-compiler \
2026-Feb-25 02:50:02.804304
&& rm -rf /var/lib/apt/lists/*
2026-Feb-25 02:50:02.804304
2026-Feb-25 02:50:02.804304
# Copy workspace configuration
2026-Feb-25 02:50:02.804304
COPY Cargo.toml ./
2026-Feb-25 02:50:02.804304
COPY Cargo.lock ./
2026-Feb-25 02:50:02.804304
2026-Feb-25 02:50:02.804304
# Copy all crates
2026-Feb-25 02:50:02.804304
COPY crates/ ./crates/
2026-Feb-25 02:50:02.804304
2026-Feb-25 02:50:02.804304
# Copy proto files
2026-Feb-25 02:50:02.804304
COPY proto/ ./proto/
2026-Feb-25 02:50:02.804304
2026-Feb-25 02:50:02.804304
# Build the release binary
2026-Feb-25 02:50:02.804304
RUN cargo build --release --bin api-server
2026-Feb-25 02:50:02.804304
2026-Feb-25 02:50:02.804304
# Stage 2: Runtime
2026-Feb-25 02:50:02.804304
FROM debian:bookworm-slim AS runtime
2026-Feb-25 02:50:02.804304
2026-Feb-25 02:50:02.804304
WORKDIR /app
2026-Feb-25 02:50:02.804304
2026-Feb-25 02:50:02.804304
# Install runtime dependencies
2026-Feb-25 02:50:02.804304
RUN apt-get update && apt-get install -y \
2026-Feb-25 02:50:02.804304
ca-certificates \
2026-Feb-25 02:50:02.804304
libssl3 \
2026-Feb-25 02:50:02.804304
&& rm -rf /var/lib/apt/lists/*
2026-Feb-25 02:50:02.804304
2026-Feb-25 02:50:02.804304
# Create non-root user
2026-Feb-25 02:50:02.804304
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-25 02:50:02.804304
2026-Feb-25 02:50:02.804304
# Copy binary from builder
2026-Feb-25 02:50:02.804304
COPY --from=builder /app/target/release/api-server /usr/local/bin/
2026-Feb-25 02:50:02.804304
2026-Feb-25 02:50:02.804304
# Copy extractor scripts
2026-Feb-25 02:50:02.804304
COPY --from=builder /app/crates/extractor/scripts/ /app/extractors/ || true
2026-Feb-25 02:50:02.804304
2026-Feb-25 02:50:02.804304
# Create directories
2026-Feb-25 02:50:02.804304
RUN mkdir -p /app/extractors /app/data && chown -R appuser:appuser /app
2026-Feb-25 02:50:02.804304
2026-Feb-25 02:50:02.804304
# Switch to non-root user
2026-Feb-25 02:50:02.804304
USER appuser
2026-Feb-25 02:50:02.804304
2026-Feb-25 02:50:02.804304
# Environment variables
2026-Feb-25 02:50:02.804304
ENV PORT=3068
2026-Feb-25 02:50:02.804304
ENV EXTRACTOR_DIR=/app/extractors
2026-Feb-25 02:50:02.804304
ENV GPU_ENABLED=false
2026-Feb-25 02:50:02.804304
ENV RUST_LOG=info
2026-Feb-25 02:50:02.804304
2026-Feb-25 02:50:02.804304
# Expose port
2026-Feb-25 02:50:02.804304
EXPOSE 3068
2026-Feb-25 02:50:02.804304
2026-Feb-25 02:50:02.804304
# Health check
2026-Feb-25 02:50:02.804304
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Feb-25 02:50:02.804304
CMD curl -f http://localhost:3068/health || exit 1
2026-Feb-25 02:50:02.804304
2026-Feb-25 02:50:02.804304
# Run the server
2026-Feb-25 02:50:02.804304
CMD ["api-server"]
2026-Feb-25 02:50:02.952075
Added 4 ARG declarations to Dockerfile for service api (multi-stage build, added to 2 stages).
2026-Feb-25 02:50:03.097800
[CMD]: docker exec wcwwgk4g8o0owkwogkgo40kk bash -c 'test -f /artifacts/wcwwgk4g8o0owkwogkgo40kk/docker/Dockerfile.frontend && echo 'exists' || echo 'not found''
2026-Feb-25 02:50:03.097800
exists
2026-Feb-25 02:50:03.248333
[CMD]: docker exec wcwwgk4g8o0owkwogkgo40kk bash -c 'cat /artifacts/wcwwgk4g8o0owkwogkgo40kk/docker/Dockerfile.frontend'
2026-Feb-25 02:50:03.248333
# Dockerfile for frontend (SvelteKit Node server)
2026-Feb-25 02:50:03.248333
# Copy ALL source files BEFORE npm install so svelte-kit sync (prepare script)
2026-Feb-25 02:50:03.248333
# can find svelte.config.js and generate .svelte-kit/ correctly.
2026-Feb-25 02:50:03.248333
2026-Feb-25 02:50:03.248333
FROM node:22-alpine AS builder
2026-Feb-25 02:50:03.248333
2026-Feb-25 02:50:03.248333
WORKDIR /app
2026-Feb-25 02:50:03.248333
2026-Feb-25 02:50:03.248333
# Copy all frontend source files first (node_modules excluded via .dockerignore)
2026-Feb-25 02:50:03.248333
COPY frontend/ ./
2026-Feb-25 02:50:03.248333
2026-Feb-25 02:50:03.248333
# Install — prepare script runs svelte-kit sync with svelte.config.js available
2026-Feb-25 02:50:03.248333
RUN npm install
2026-Feb-25 02:50:03.248333
2026-Feb-25 02:50:03.248333
# Build via programmatic API — bypasses vite.config.ts loading (esbuild issues on Alpine)
2026-Feb-25 02:50:03.248333
RUN node build-docker.mjs
2026-Feb-25 02:50:03.248333
2026-Feb-25 02:50:03.248333
# Runtime
2026-Feb-25 02:50:03.248333
FROM node:22-alpine AS runtime
2026-Feb-25 02:50:03.248333
2026-Feb-25 02:50:03.248333
WORKDIR /app
2026-Feb-25 02:50:03.248333
2026-Feb-25 02:50:03.248333
COPY --from=builder /app/build ./build
2026-Feb-25 02:50:03.248333
COPY --from=builder /app/package.json ./
2026-Feb-25 02:50:03.248333
2026-Feb-25 02:50:03.248333
ENV PORT=3000
2026-Feb-25 02:50:03.248333
ENV HOST=0.0.0.0
2026-Feb-25 02:50:03.248333
2026-Feb-25 02:50:03.248333
EXPOSE 3000
2026-Feb-25 02:50:03.248333
2026-Feb-25 02:50:03.248333
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Feb-25 02:50:03.248333
CMD wget -qO- http://localhost:3000 || exit 1
2026-Feb-25 02:50:03.248333
2026-Feb-25 02:50:03.248333
CMD ["node", "build"]
2026-Feb-25 02:50:03.404271
Added 4 ARG declarations to Dockerfile for service frontend (multi-stage build, added to 2 stages).
2026-Feb-25 02:50:03.407804
Pulling & building required images.
2026-Feb-25 02:50:03.414229
Creating build-time .env file in /artifacts (outside Docker context).
2026-Feb-25 02:50:03.561809
Adding build arguments to Docker Compose build command.
2026-Feb-25 02:50:03.833973
[CMD]: docker exec wcwwgk4g8o0owkwogkgo40kk bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/build-time.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/wcwwgk4g8o0owkwogkgo40kk -f /artifacts/wcwwgk4g8o0owkwogkgo40kk/docker/docker-compose.homeserver.yml build --pull --build-arg COOLIFY_URL --build-arg COOLIFY_FQDN --build-arg COOLIFY_BUILD_SECRETS_HASH=1bf8dfcfdeb73162b126768f9e085efbcfdffa61f0529746b67826c254109afc'
2026-Feb-25 02:50:03.833973
#1 [internal] load local bake definitions
2026-Feb-25 02:50:04.042531
#1 reading from stdin 1.79kB done
2026-Feb-25 02:50:04.042531
#1 DONE 0.0s
2026-Feb-25 02:50:04.042531
2026-Feb-25 02:50:04.042531
#2 [frontend internal] load build definition from Dockerfile.frontend
2026-Feb-25 02:50:04.042531
#2 transferring dockerfile: 999B done
2026-Feb-25 02:50:04.042531
#2 DONE 0.0s
2026-Feb-25 02:50:04.042531
2026-Feb-25 02:50:04.042531
#3 [api internal] load build definition from Dockerfile.vps
2026-Feb-25 02:50:04.042531
#3 transferring dockerfile: 1.60kB done
2026-Feb-25 02:50:04.042531
#3 DONE 0.0s
2026-Feb-25 02:50:04.042531
2026-Feb-25 02:50:04.042531
#4 [gpu-worker internal] load build definition from Dockerfile.homeserver
2026-Feb-25 02:50:04.042531
#4 transferring dockerfile: 2.05kB done
2026-Feb-25 02:50:04.042531
#4 DONE 0.0s
2026-Feb-25 02:50:04.042531
2026-Feb-25 02:50:04.042531
#5 [gpu-worker internal] load metadata for docker.io/nvidia/cuda:12.3.2-devel-ubuntu22.04
2026-Feb-25 02:50:04.994546
#5 ...
2026-Feb-25 02:50:04.994546
2026-Feb-25 02:50:04.994546
#6 [api internal] load metadata for docker.io/library/rust:1.85-bookworm
2026-Feb-25 02:50:04.994546
#6 DONE 1.1s
2026-Feb-25 02:50:05.237504
#7 [gpu-worker internal] load metadata for docker.io/nvidia/cuda:12.3.2-runtime-ubuntu22.04
2026-Feb-25 02:50:05.237504
#7 DONE 1.1s
2026-Feb-25 02:50:05.237504
2026-Feb-25 02:50:05.237504
#8 [frontend internal] load metadata for docker.io/library/node:22-alpine
2026-Feb-25 02:50:05.237504
#8 DONE 1.1s
2026-Feb-25 02:50:05.237504
2026-Feb-25 02:50:05.237504
#5 [gpu-worker internal] load metadata for docker.io/nvidia/cuda:12.3.2-devel-ubuntu22.04
2026-Feb-25 02:50:05.237504
#5 DONE 1.1s
2026-Feb-25 02:50:05.237504
2026-Feb-25 02:50:05.237504
#9 [api internal] load .dockerignore
2026-Feb-25 02:50:05.237504
#9 transferring context: 341B done
2026-Feb-25 02:50:05.237504
#9 DONE 0.0s
2026-Feb-25 02:50:05.237504
2026-Feb-25 02:50:05.237504
#10 [api internal] load metadata for docker.io/library/debian:bookworm-slim
2026-Feb-25 02:50:05.237504
#10 DONE 1.1s
2026-Feb-25 02:50:05.237504
2026-Feb-25 02:50:05.237504
#11 [frontend builder 1/5] FROM docker.io/library/node:22-alpine@sha256:e4bf2a82ad0a4037d28035ae71529873c069b13eb0455466ae0bc13363826e34
2026-Feb-25 02:50:05.237504
#11 resolve docker.io/library/node:22-alpine@sha256:e4bf2a82ad0a4037d28035ae71529873c069b13eb0455466ae0bc13363826e34 0.0s done
2026-Feb-25 02:50:05.237504
#11 DONE 0.0s
2026-Feb-25 02:50:05.237504
2026-Feb-25 02:50:05.237504
#12 [gpu-worker runtime 1/6] FROM docker.io/nvidia/cuda:12.3.2-runtime-ubuntu22.04@sha256:882b43fadd789693f08de95e6014ed5f0ea118c7b342150876e153b4340e1103
2026-Feb-25 02:50:05.237504
#12 resolve docker.io/nvidia/cuda:12.3.2-runtime-ubuntu22.04@sha256:882b43fadd789693f08de95e6014ed5f0ea118c7b342150876e153b4340e1103 0.0s done
2026-Feb-25 02:50:05.237504
#12 DONE 0.0s
2026-Feb-25 02:50:05.237504
2026-Feb-25 02:50:05.237504
#13 [gpu-worker runtime 2/6] WORKDIR /app
2026-Feb-25 02:50:05.237504
#13 CACHED
2026-Feb-25 02:50:05.237504
2026-Feb-25 02:50:05.237504
#14 [frontend internal] load build context
2026-Feb-25 02:50:05.237504
#14 transferring context: 169.68kB done
2026-Feb-25 02:50:05.237504
#14 DONE 0.0s
2026-Feb-25 02:50:05.237504
2026-Feb-25 02:50:05.237504
#15 [frontend builder 2/5] WORKDIR /app
2026-Feb-25 02:50:05.237504
#15 CACHED
2026-Feb-25 02:50:05.237504
2026-Feb-25 02:50:05.237504
#16 [api runtime 1/7] FROM docker.io/library/debian:bookworm-slim@sha256:74d56e3931e0d5a1dd51f8c8a2466d21de84a271cd3b5a733b803aa91abf4421
2026-Feb-25 02:50:05.237504
#16 resolve docker.io/library/debian:bookworm-slim@sha256:74d56e3931e0d5a1dd51f8c8a2466d21de84a271cd3b5a733b803aa91abf4421 0.0s done
2026-Feb-25 02:50:05.237504
#16 DONE 0.0s
2026-Feb-25 02:50:05.237504
2026-Feb-25 02:50:05.237504
#17 [api runtime 2/7] WORKDIR /app
2026-Feb-25 02:50:05.237504
#17 CACHED
2026-Feb-25 02:50:05.237504
2026-Feb-25 02:50:05.237504
#18 [api runtime 3/7] RUN apt-get update && apt-get install -y     ca-certificates     libssl3     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 02:50:05.237504
#18 CACHED
2026-Feb-25 02:50:05.237504
2026-Feb-25 02:50:05.237504
#19 [api runtime 4/7] RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-25 02:50:05.237504
#19 CACHED
2026-Feb-25 02:50:05.237504
2026-Feb-25 02:50:05.237504
#20 [api internal] load build context
2026-Feb-25 02:50:05.237504
#20 transferring context: 475.95kB 0.0s done
2026-Feb-25 02:50:05.237504
#20 DONE 0.0s
2026-Feb-25 02:50:05.237504
2026-Feb-25 02:50:05.237504
#21 [frontend builder 3/5] COPY frontend/ ./
2026-Feb-25 02:50:05.237504
#21 DONE 0.0s
2026-Feb-25 02:50:05.237504
2026-Feb-25 02:50:05.237504
#22 [api builder 1/8] FROM docker.io/library/rust:1.85-bookworm@sha256:e51d0265072d2d9d5d320f6a44dde6b9ef13653b035098febd68cce8fa7c0bc4
2026-Feb-25 02:50:05.237504
#22 resolve docker.io/library/rust:1.85-bookworm@sha256:e51d0265072d2d9d5d320f6a44dde6b9ef13653b035098febd68cce8fa7c0bc4 0.0s done
2026-Feb-25 02:50:05.237504
#22 extracting sha256:353e14e5cc47664fba714a7da288001d90427c705494847ac773f5cc08199451
2026-Feb-25 02:50:09.116786
#22 extracting sha256:353e14e5cc47664fba714a7da288001d90427c705494847ac773f5cc08199451 3.9s done
2026-Feb-25 02:50:09.116786
#22 extracting sha256:50edd9a93fcd3168d25021d6d0863290a0ce5692f406e0614ce3c8cef848babd
2026-Feb-25 02:50:11.039928
#22 extracting sha256:50edd9a93fcd3168d25021d6d0863290a0ce5692f406e0614ce3c8cef848babd 2.1s done
2026-Feb-25 02:50:11.039928
#22 DONE 6.0s
2026-Feb-25 02:50:11.039928
2026-Feb-25 02:50:11.039928
#23 [gpu-worker runtime 3/6] RUN apt-get update && apt-get install -y     ca-certificates     libssl3     ffmpeg     libavcodec58     libavformat58     libavutil56     libswscale5     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 02:50:11.039928
#23 0.689 Get:1 http://archive.ubuntu.com/ubuntu jammy InRelease [270 kB]
2026-Feb-25 02:50:11.039928
#23 0.757 Get:2 http://security.ubuntu.com/ubuntu jammy-security InRelease [129 kB]
2026-Feb-25 02:50:11.039928
#23 1.624 Get:3 https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2204/x86_64  InRelease [1581 B]
2026-Feb-25 02:50:11.039928
#23 2.118 Get:4 http://security.ubuntu.com/ubuntu jammy-security/restricted amd64 Packages [6626 kB]
2026-Feb-25 02:50:11.039928
#23 2.139 Get:5 http://archive.ubuntu.com/ubuntu jammy-updates InRelease [128 kB]
2026-Feb-25 02:50:11.039928
#23 2.488 Get:6 http://archive.ubuntu.com/ubuntu jammy-backports InRelease [127 kB]
2026-Feb-25 02:50:11.039928
#23 2.836 Get:7 http://archive.ubuntu.com/ubuntu jammy/main amd64 Packages [1792 kB]
2026-Feb-25 02:50:11.039928
#23 2.991 Get:8 https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2204/x86_64  Packages [2383 kB]
2026-Feb-25 02:50:11.039928
#23 3.869 Get:9 http://archive.ubuntu.com/ubuntu jammy/multiverse amd64 Packages [266 kB]
2026-Feb-25 02:50:11.039928
#23 3.913 Get:10 http://archive.ubuntu.com/ubuntu jammy/universe amd64 Packages [17.5 MB]
2026-Feb-25 02:50:11.039928
#23 5.384 Get:11 http://archive.ubuntu.com/ubuntu jammy/restricted amd64 Packages [164 kB]
2026-Feb-25 02:50:11.039928
#23 5.385 Get:12 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 Packages [4100 kB]
2026-Feb-25 02:50:11.039928
#23 5.415 Get:13 http://security.ubuntu.com/ubuntu jammy-security/universe amd64 Packages [1301 kB]
2026-Feb-25 02:50:11.039928
#23 5.469 Get:14 http://archive.ubuntu.com/ubuntu jammy-updates/multiverse amd64 Packages [70.9 kB]
2026-Feb-25 02:50:11.039928
#23 5.470 Get:15 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 Packages [1613 kB]
2026-Feb-25 02:50:11.039928
#23 5.578 Get:16 http://archive.ubuntu.com/ubuntu jammy-updates/restricted amd64 Packages [6835 kB]
2026-Feb-25 02:50:11.039928
#23 5.798 Get:17 http://archive.ubuntu.com/ubuntu jammy-backports/universe amd64 Packages [37.2 kB]
2026-Feb-25 02:50:11.039928
#23 5.798 Get:18 http://archive.ubuntu.com/ubuntu jammy-backports/main amd64 Packages [83.9 kB]
2026-Feb-25 02:50:11.578859
#23 ...
2026-Feb-25 02:50:11.578859
2026-Feb-25 02:50:11.578859
#24 [api builder 2/8] WORKDIR /app
2026-Feb-25 02:50:11.578859
#24 DONE 0.5s
2026-Feb-25 02:50:11.578859
2026-Feb-25 02:50:11.578859
#25 [gpu-worker builder 1/9] FROM docker.io/nvidia/cuda:12.3.2-devel-ubuntu22.04@sha256:6655d5fc2fb48580255a5021a81c379c325a457b74b77ac823ed67e4faa32aeb
2026-Feb-25 02:50:11.578859
#25 resolve docker.io/nvidia/cuda:12.3.2-devel-ubuntu22.04@sha256:6655d5fc2fb48580255a5021a81c379c325a457b74b77ac823ed67e4faa32aeb 0.0s done
2026-Feb-25 02:50:11.578859
#25 extracting sha256:8ebe7e080c37469e9f54a4a0506785a20a769e656bd6f745c02d2e034ae5a2f8
2026-Feb-25 02:50:11.730227
#25 ...
2026-Feb-25 02:50:11.730227
2026-Feb-25 02:50:11.730227
#26 [api builder 3/8] RUN apt-get update && apt-get install -y     pkg-config     libssl-dev     protobuf-compiler     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 02:50:12.171615
#26 0.592 Get:1 http://deb.debian.org/debian bookworm InRelease [151 kB]
2026-Feb-25 02:50:13.028864
#26 1.449 Get:2 http://deb.debian.org/debian bookworm-updates InRelease [55.4 kB]
2026-Feb-25 02:50:13.436993
#26 1.857 Get:3 http://deb.debian.org/debian-security bookworm-security InRelease [48.0 kB]
2026-Feb-25 02:50:13.923830
#26 ...
2026-Feb-25 02:50:13.923830
2026-Feb-25 02:50:13.923830
#27 [frontend builder 4/5] RUN npm install
2026-Feb-25 02:50:13.923830
#27 7.776
2026-Feb-25 02:50:13.923830
#27 7.776 > frontend@0.0.1 prepare
2026-Feb-25 02:50:13.923830
#27 7.776 > svelte-kit sync || echo ''
2026-Feb-25 02:50:13.923830
#27 7.776
2026-Feb-25 02:50:13.923830
#27 8.618
2026-Feb-25 02:50:13.923830
#27 8.618 added 110 packages, and audited 111 packages in 8s
2026-Feb-25 02:50:13.923830
#27 8.618
2026-Feb-25 02:50:13.923830
#27 8.618 20 packages are looking for funding
2026-Feb-25 02:50:13.923830
#27 8.618   run `npm fund` for details
2026-Feb-25 02:50:13.923830
#27 8.628
2026-Feb-25 02:50:13.923830
#27 8.628 5 low severity vulnerabilities
2026-Feb-25 02:50:13.923830
#27 8.628
2026-Feb-25 02:50:13.923830
#27 8.628 To address all issues (including breaking changes), run:
2026-Feb-25 02:50:13.923830
#27 8.628   npm audit fix --force
2026-Feb-25 02:50:13.923830
#27 8.628
2026-Feb-25 02:50:13.923830
#27 8.628 Run `npm audit` for details.
2026-Feb-25 02:50:13.923830
#27 8.630 npm notice
2026-Feb-25 02:50:13.923830
#27 8.630 npm notice New major version of npm available! 10.9.4 -> 11.10.1
2026-Feb-25 02:50:13.923830
#27 8.630 npm notice Changelog: https://github.com/npm/cli/releases/tag/v11.10.1
2026-Feb-25 02:50:13.923830
#27 8.630 npm notice To update run: npm install -g npm@11.10.1
2026-Feb-25 02:50:13.923830
#27 8.630 npm notice
2026-Feb-25 02:50:13.923830
#27 DONE 8.8s
2026-Feb-25 02:50:13.923830
2026-Feb-25 02:50:13.923830
#26 [api builder 3/8] RUN apt-get update && apt-get install -y     pkg-config     libssl-dev     protobuf-compiler     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 02:50:13.923830
#26 2.197 Get:4 http://deb.debian.org/debian bookworm/main amd64 Packages [8792 kB]
2026-Feb-25 02:50:15.735972
#26 ...
2026-Feb-25 02:50:15.735972
2026-Feb-25 02:50:15.735972
#28 [frontend builder 5/5] RUN node build-docker.mjs
2026-Feb-25 02:50:15.735972
#28 1.025 The following Vite config options will be overridden by SvelteKit:
2026-Feb-25 02:50:15.735972
#28 1.025   - build.outDir
2026-Feb-25 02:50:15.735972
#28 1.051 vite v6.4.1 building SSR bundle for production...
2026-Feb-25 02:50:15.735972
#28 1.071 transforming...
2026-Feb-25 02:50:15.735972
#28 1.489 2:50:15 AM [vite-plugin-svelte] src/components/InterstitialAd.svelte:28:20 This reference only captures the initial value of `countdownSeconds`. Did you mean to reference it inside a closure instead?
2026-Feb-25 02:50:15.735972
#28 1.489 https://svelte.dev/e/state_referenced_locally
2026-Feb-25 02:50:15.735972
#28 1.489 26:
2026-Feb-25 02:50:15.735972
#28 1.489 27:   /** Current countdown value */
2026-Feb-25 02:50:15.735972
#28 1.489 28:   let count = $state(countdownSeconds);
2026-Feb-25 02:50:15.735972
#28 1.489                                          ^
2026-Feb-25 02:50:15.735972
#28 1.489 29:
2026-Feb-25 02:50:15.735972
#28 1.489 30:   /** Whether countdown is complete */
2026-Feb-25 02:50:15.735972
#28 1.698 ✓ 123 modules transformed.
2026-Feb-25 02:50:15.735972
#28 1.701 ✗ Build failed in 650ms
2026-Feb-25 02:50:15.735972
#28 1.702 node:internal/modules/run_main:123
2026-Feb-25 02:50:15.735972
#28 1.702     triggerUncaughtException(
2026-Feb-25 02:50:15.735972
#28 1.702     ^
2026-Feb-25 02:50:15.735972
#28 1.702
2026-Feb-25 02:50:15.735972
#28 1.702 [vite:load-fallback] Could not load /app/src/lib/analytics (imported by src/routes/privacy/+page.svelte): ENOENT: no such file or directory, open '/app/src/lib/analytics'
2026-Feb-25 02:50:15.735972
#28 1.702     at async open (node:internal/fs/promises:636:25)
2026-Feb-25 02:50:15.735972
#28 1.702     at async Object.readFile (node:internal/fs/promises:1235:14)
2026-Feb-25 02:50:15.735972
#28 1.702     at async Object.handler (file:///app/node_modules/vite/dist/node/chunks/dep-D4NMHUTW.js:45872:27)
2026-Feb-25 02:50:15.735972
#28 1.702     at async PluginDriver.hookFirstAndGetPlugin (file:///app/node_modules/rollup/dist/es/shared/node-entry.js:22453:28)
2026-Feb-25 02:50:15.735972
#28 1.702     at async file:///app/node_modules/rollup/dist/es/shared/node-entry.js:21445:33
2026-Feb-25 02:50:15.735972
#28 1.702     at async Queue.work (file:///app/node_modules/rollup/dist/es/shared/node-entry.js:22681:32) {
2026-Feb-25 02:50:15.735972
#28 1.702   errno: -2,
2026-Feb-25 02:50:15.735972
#28 1.702   code: 'PLUGIN_ERROR',
2026-Feb-25 02:50:15.735972
#28 1.702   syscall: 'open',
2026-Feb-25 02:50:15.735972
#28 1.702   path: '/app/src/lib/analytics',
2026-Feb-25 02:50:15.735972
#28 1.702   pluginCode: 'ENOENT',
2026-Feb-25 02:50:15.735972
#28 1.702   plugin: 'vite:load-fallback',
2026-Feb-25 02:50:15.735972
#28 1.702   hook: 'load',
2026-Feb-25 02:50:15.735972
#28 1.702   watchFiles: [
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/server/index.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/app/server/remote/index.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/.svelte-kit/generated/server/internal.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/components/svelte-5/error.svelte',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/src/routes/+layout.svelte',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/src/routes/+page.svelte',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/src/routes/privacy/+page.svelte',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/package.json',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/shared-server.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/server/constants.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/server/respond.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/esm-env/index.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/utils/promise.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/utils/env.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/server/utils.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/server/app.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/app/server/remote/command.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/app/server/remote/prerender.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/app/server/remote/form.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/app/server/remote/query.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/app/paths/internal/server.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/.svelte-kit/generated/root.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/app/state/index.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/svelte/src/internal/server/index.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/svelte/src/index-server.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/app/environment/index.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/src/components/CookieConsent.svelte',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/src/components/AdBanner.svelte',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/src/components/UrlInput.svelte',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/src/components/BatchInput.svelte',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/src/components/FormatPicker.svelte',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/src/components/DownloadBtn.svelte',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/src/components/BatchProgress.svelte',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/src/components/InterstitialAd.svelte',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/esm-env/false.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/esm-env/true.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/utils/error.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/constants.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/utils/escape.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/app/server/remote/shared.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/utils/http.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/shared.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/form-utils.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/svelte/src/legacy/legacy-server.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/.svelte-kit/generated/root.svelte',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/app/state/client.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/app/state/server.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/svelte/src/constants.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/svelte/src/internal/shared/attributes.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/svelte/src/store/utils.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/svelte/src/internal/shared/utils.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/svelte/src/escaping.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/svelte/src/internal/server/hydration.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/svelte/src/internal/shared/validate.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/svelte/src/utils.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/svelte/src/internal/server/errors.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/svelte/src/internal/server/context.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/svelte/src/internal/server/renderer.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/svelte/src/internal/server/blocks/html.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/svelte/src/internal/server/dev.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/svelte/src/internal/shared/clone.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/svelte/src/internal/server/abort-signal.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/svelte/src/internal/server/hydratable.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/svelte/src/internal/server/blocks/snippet.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/server/endpoint.js',
2026-Feb-25 02:50:15.735972
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/server/page/index.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/server/page/render.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/server/page/respond_with_error.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/@sveltejs/kit/src/utils/url.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/@sveltejs/kit/src/utils/routing.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/server/data/index.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/server/cookie.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/server/fetch.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/@sveltejs/kit/src/utils/page_nodes.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/@sveltejs/kit/src/utils/exports.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/server/page/actions.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/server/env_module.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/server/page/server_routing.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/server/validate-headers.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/pathname.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/server/remote.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/telemetry/record_span.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/telemetry/otel.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/server/page/data_serializer.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/svelte/src/transition/index.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/utils.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/svelte/src/legacy/legacy-client.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/svelte/src/internal/flags/index.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/client/state.svelte.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/@sveltejs/kit/src/runtime/client/client.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/svelte/src/index-client.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/svelte/src/internal/shared/warnings.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/svelte/src/internal/shared/errors.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/svelte/src/internal/server/warnings.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/svelte/src/internal/server/crypto.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/svelte/src/internal/server/render-context.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/svelte/src/html-tree-validation.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/svelte/src/internal/shared/dev.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/svelte/src/internal/client/constants.js',
2026-Feb-25 02:50:15.738573
#28 1.702     '/app/node_modules/@sveltejs/kit/src/utils/array.js',
2026-Feb-25 02:50:15.738573
#28 1.702     ... 10 more items
2026-Feb-25 02:50:15.738573
#28 1.702   ]
2026-Feb-25 02:50:15.738573
#28 1.702 }
2026-Feb-25 02:50:15.738573
#28 1.702
2026-Feb-25 02:50:15.738573
#28 1.702 Node.js v22.22.0
2026-Feb-25 02:50:15.738573
#28 ERROR: process "/bin/sh -c node build-docker.mjs" did not complete successfully: exit code: 1
2026-Feb-25 02:50:15.805988
#25 [gpu-worker builder 1/9] FROM docker.io/nvidia/cuda:12.3.2-devel-ubuntu22.04@sha256:6655d5fc2fb48580255a5021a81c379c325a457b74b77ac823ed67e4faa32aeb
2026-Feb-25 02:50:15.805988
#25 extracting sha256:8ebe7e080c37469e9f54a4a0506785a20a769e656bd6f745c02d2e034ae5a2f8 10.7s done
2026-Feb-25 02:50:15.805988
#25 DONE 10.7s
2026-Feb-25 02:50:15.805988
2026-Feb-25 02:50:15.805988
#29 [gpu-worker builder 2/9] WORKDIR /app
2026-Feb-25 02:50:15.805988
#29 CANCELED
2026-Feb-25 02:50:15.805988
2026-Feb-25 02:50:15.805988
#26 [api builder 3/8] RUN apt-get update && apt-get install -y     pkg-config     libssl-dev     protobuf-compiler     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 02:50:15.805988
#26 CANCELED
2026-Feb-25 02:50:15.805988
2026-Feb-25 02:50:15.805988
#23 [gpu-worker runtime 3/6] RUN apt-get update && apt-get install -y     ca-certificates     libssl3     ffmpeg     libavcodec58     libavformat58     libavutil56     libswscale5     && rm -rf /var/lib/apt/lists/*
2026-Feb-25 02:50:15.805988
#23 8.927 Get:19 http://security.ubuntu.com/ubuntu jammy-security/multiverse amd64 Packages [62.6 kB]
2026-Feb-25 02:50:15.805988
#23 9.869 Get:20 http://security.ubuntu.com/ubuntu jammy-security/main amd64 Packages [3767 kB]
2026-Feb-25 02:50:15.805988
#23 CANCELED
2026-Feb-25 02:50:15.805988
------
2026-Feb-25 02:50:15.805988
> [frontend builder 5/5] RUN node build-docker.mjs:
2026-Feb-25 02:50:15.805988
1.702     '/app/node_modules/svelte/src/internal/server/render-context.js',
2026-Feb-25 02:50:15.805988
1.702     '/app/node_modules/svelte/src/html-tree-validation.js',
2026-Feb-25 02:50:15.805988
1.702     '/app/node_modules/svelte/src/internal/shared/dev.js',
2026-Feb-25 02:50:15.805988
1.702     '/app/node_modules/svelte/src/internal/client/constants.js',
2026-Feb-25 02:50:15.805988
1.702     '/app/node_modules/@sveltejs/kit/src/utils/array.js',
2026-Feb-25 02:50:15.805988
1.702     ... 10 more items
2026-Feb-25 02:50:15.805988
1.702   ]
2026-Feb-25 02:50:15.805988
1.702 }
2026-Feb-25 02:50:15.805988
1.702
2026-Feb-25 02:50:15.805988
1.702 Node.js v22.22.0
2026-Feb-25 02:50:15.805988
------
2026-Feb-25 02:50:15.808688
Dockerfile.frontend:18
2026-Feb-25 02:50:15.808688
2026-Feb-25 02:50:15.808688
--------------------
2026-Feb-25 02:50:15.808688
2026-Feb-25 02:50:15.808688
16 |
2026-Feb-25 02:50:15.808688
2026-Feb-25 02:50:15.808688
17 |     # Build via programmatic API — bypasses vite.config.ts loading (esbuild issues on Alpine)
2026-Feb-25 02:50:15.808688
2026-Feb-25 02:50:15.808688
18 | >>> RUN node build-docker.mjs
2026-Feb-25 02:50:15.808688
2026-Feb-25 02:50:15.808688
19 |
2026-Feb-25 02:50:15.808688
2026-Feb-25 02:50:15.808688
20 |     # Runtime
2026-Feb-25 02:50:15.808688
2026-Feb-25 02:50:15.808688
--------------------
2026-Feb-25 02:50:15.808688
2026-Feb-25 02:50:15.808688
target frontend: failed to solve: process "/bin/sh -c node build-docker.mjs" did not complete successfully: exit code: 1
2026-Feb-25 02:50:15.811644
exit status 1
2026-Feb-25 02:50:15.843907
========================================
2026-Feb-25 02:50:15.849813
Deployment failed: Command execution failed (exit code 1): docker exec wcwwgk4g8o0owkwogkgo40kk bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/build-time.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/wcwwgk4g8o0owkwogkgo40kk -f /artifacts/wcwwgk4g8o0owkwogkgo40kk/docker/docker-compose.homeserver.yml build --pull --build-arg COOLIFY_URL --build-arg COOLIFY_FQDN --build-arg COOLIFY_BUILD_SECRETS_HASH=1bf8dfcfdeb73162b126768f9e085efbcfdffa61f0529746b67826c254109afc'
2026-Feb-25 02:50:15.849813
Error: Dockerfile.frontend:18
2026-Feb-25 02:50:15.849813
2026-Feb-25 02:50:15.849813
--------------------
2026-Feb-25 02:50:15.849813
2026-Feb-25 02:50:15.849813
16 |
2026-Feb-25 02:50:15.849813
2026-Feb-25 02:50:15.849813
17 |     # Build via programmatic API — bypasses vite.config.ts loading (esbuild issues on Alpine)
2026-Feb-25 02:50:15.849813
2026-Feb-25 02:50:15.849813
18 | >>> RUN node build-docker.mjs
2026-Feb-25 02:50:15.849813
2026-Feb-25 02:50:15.849813
19 |
2026-Feb-25 02:50:15.849813
2026-Feb-25 02:50:15.849813
20 |     # Runtime
2026-Feb-25 02:50:15.849813
2026-Feb-25 02:50:15.849813
--------------------
2026-Feb-25 02:50:15.849813
2026-Feb-25 02:50:15.849813
target frontend: failed to solve: process "/bin/sh -c node build-docker.mjs" did not complete successfully: exit code: 1
2026-Feb-25 02:50:15.849813
2026-Feb-25 02:50:15.849813
exit status 1
2026-Feb-25 02:50:15.856742
Error type: RuntimeException
2026-Feb-25 02:50:15.861502
Error code: 0
2026-Feb-25 02:50:15.866123
Location: /var/www/html/app/Traits/ExecuteRemoteCommand.php:243
2026-Feb-25 02:50:15.870284
Stack trace (first 5 lines):
2026-Feb-25 02:50:15.874878
#0 /var/www/html/app/Traits/ExecuteRemoteCommand.php(104): App\Jobs\ApplicationDeploymentJob->executeCommandWithProcess()
2026-Feb-25 02:50:15.879379
#1 /var/www/html/vendor/laravel/framework/src/Illuminate/Collections/Traits/EnumeratesValues.php(272): App\Jobs\ApplicationDeploymentJob->{closure:App\Traits\ExecuteRemoteCommand::execute_remote_command():71}()
2026-Feb-25 02:50:15.884194
#2 /var/www/html/app/Traits/ExecuteRemoteCommand.php(71): Illuminate\Support\Collection->each()
2026-Feb-25 02:50:15.888402
#3 /var/www/html/app/Jobs/ApplicationDeploymentJob.php(730): App\Jobs\ApplicationDeploymentJob->execute_remote_command()
2026-Feb-25 02:50:15.892635
#4 /var/www/html/app/Jobs/ApplicationDeploymentJob.php(467): App\Jobs\ApplicationDeploymentJob->deploy_docker_compose_buildpack()
2026-Feb-25 02:50:15.899399
========================================
2026-Feb-25 02:50:16.151024
Gracefully shutting down build container: wcwwgk4g8o0owkwogkgo40kk
2026-Feb-25 02:50:16.425255
[CMD]: docker stop -t 30 wcwwgk4g8o0owkwogkgo40kk
2026-Feb-25 02:50:16.425255
wcwwgk4g8o0owkwogkgo40kk