2026-Mar-10 02:21:36.092585
Starting deployment of khoa280703/downloadtool:main-zoscg4oc04gkwkssg0kw8w8w to localhost.
2026-Mar-10 02:21:36.660579
Preparing container with helper image: ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Mar-10 02:21:36.953581
[CMD]: docker stop -t 30 woc844so8wg0cks08cwcgowg
2026-Mar-10 02:21:36.953581
Error response from daemon: No such container: woc844so8wg0cks08cwcgowg
2026-Mar-10 02:21:37.272024
[CMD]: docker run -d --network coolify --name woc844so8wg0cks08cwcgowg  --rm -v /var/run/docker.sock:/var/run/docker.sock ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Mar-10 02:21:37.272024
fb27c4cce931289db75103f782dd582b7a780613981b0cba9f3b97813927cb90
2026-Mar-10 02:21:38.910160
[CMD]: docker exec woc844so8wg0cks08cwcgowg bash -c 'GIT_SSH_COMMAND="ssh -o ConnectTimeout=30 -p 22 -o Port=22 -o LogLevel=ERROR -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git ls-remote https://github.com/Khoa280703/downloadtool refs/heads/main'
2026-Mar-10 02:21:38.910160
6e96c70cad023efec322d476ffeb62cf7afd80e0	refs/heads/main
2026-Mar-10 02:21:38.920378
----------------------------------------
2026-Mar-10 02:21:38.924402
Importing Khoa280703/downloadtool:main (commit sha 6e96c70cad023efec322d476ffeb62cf7afd80e0) to /artifacts/woc844so8wg0cks08cwcgowg.
2026-Mar-10 02:21:39.273138
[CMD]: docker exec woc844so8wg0cks08cwcgowg bash -c 'git clone --depth=1 --recurse-submodules --shallow-submodules -b 'main' 'https://github.com/Khoa280703/downloadtool' '/artifacts/woc844so8wg0cks08cwcgowg' && cd '/artifacts/woc844so8wg0cks08cwcgowg' && if [ -f .gitmodules ]; then sed -i "s#git@\(.*\):#https://\1/#g" '/artifacts/woc844so8wg0cks08cwcgowg'/.gitmodules || true && git submodule sync && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git submodule update --init --recursive --depth=1; fi && cd '/artifacts/woc844so8wg0cks08cwcgowg' && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git lfs pull'
2026-Mar-10 02:21:39.273138
Cloning into '/artifacts/woc844so8wg0cks08cwcgowg'...
2026-Mar-10 02:21:42.002447
[CMD]: docker exec woc844so8wg0cks08cwcgowg bash -c 'cd /artifacts/woc844so8wg0cks08cwcgowg && git log -1 6e96c70cad023efec322d476ffeb62cf7afd80e0 --pretty=%B'
2026-Mar-10 02:21:42.002447
fix: disable caching for mux job proxy routes
2026-Mar-10 02:21:46.781281
[CMD]: docker exec woc844so8wg0cks08cwcgowg bash -c 'test -f /artifacts/woc844so8wg0cks08cwcgowg/docker/Dockerfile.api && echo 'exists' || echo 'not found''
2026-Mar-10 02:21:46.781281
exists
2026-Mar-10 02:21:47.128034
[CMD]: docker exec woc844so8wg0cks08cwcgowg bash -c 'cat /artifacts/woc844so8wg0cks08cwcgowg/docker/Dockerfile.api'
2026-Mar-10 02:21:47.128034
# Dockerfile for API service deployment
2026-Mar-10 02:21:47.128034
# Builds the API server and related components without GPU support
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
# Stage 0: Build injector JS (embedded into api crate via include_str! at compile time)
2026-Mar-10 02:21:47.128034
FROM node:22-alpine AS js-builder
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
WORKDIR /app
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
RUN npm install -g pnpm
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
# Copy workspace manifests for pnpm resolution
2026-Mar-10 02:21:47.128034
COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Mar-10 02:21:47.128034
COPY packages/api-client/package.json ./packages/api-client/
2026-Mar-10 02:21:47.128034
COPY apps/injector/package.json ./apps/injector/
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
# Copy injector source and shared packages
2026-Mar-10 02:21:47.128034
COPY apps/injector/ ./apps/injector/
2026-Mar-10 02:21:47.128034
COPY packages/ ./packages/
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
# Install deps and build injector (produces dist/bm.js and dist/youtube-downloader.user.js)
2026-Mar-10 02:21:47.128034
RUN pnpm install --frozen-lockfile
2026-Mar-10 02:21:47.128034
RUN pnpm --filter @downloadtool/injector build
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
# Build extractor TypeScript to IIFE format (required by crates/extractor/build.rs)
2026-Mar-10 02:21:47.128034
COPY extractors/ ./extractors/
2026-Mar-10 02:21:47.128034
RUN mkdir -p extractors/dist && \
2026-Mar-10 02:21:47.128034
npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js && \
2026-Mar-10 02:21:47.128034
npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
# Stage 1: Rust builder
2026-Mar-10 02:21:47.128034
FROM rust:1.91-bookworm AS builder
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
WORKDIR /app
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
# Install dependencies
2026-Mar-10 02:21:47.128034
RUN apt-get update && apt-get install -y \
2026-Mar-10 02:21:47.128034
pkg-config \
2026-Mar-10 02:21:47.128034
libssl-dev \
2026-Mar-10 02:21:47.128034
&& rm -rf /var/lib/apt/lists/*
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
# Copy workspace configuration
2026-Mar-10 02:21:47.128034
COPY Cargo.toml ./
2026-Mar-10 02:21:47.128034
COPY Cargo.lock ./
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
# Copy all crates
2026-Mar-10 02:21:47.128034
COPY crates/ ./crates/
2026-Mar-10 02:21:47.128034
COPY config/ ./config/
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
# Copy injector dist (required by include_str! in crates/api/src/routes/static_files.rs)
2026-Mar-10 02:21:47.128034
COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
# Copy extractor source + pre-built IIFE dist (built by js-builder stage)
2026-Mar-10 02:21:47.128034
COPY extractors/ ./extractors/
2026-Mar-10 02:21:47.128034
COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
# Build the release binary
2026-Mar-10 02:21:47.128034
RUN cargo build --release --bin api-server
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
# Stage 2: Runtime
2026-Mar-10 02:21:47.128034
FROM debian:bookworm-slim AS runtime
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
WORKDIR /app
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
# Install runtime dependencies
2026-Mar-10 02:21:47.128034
RUN apt-get update && apt-get install -y \
2026-Mar-10 02:21:47.128034
ca-certificates \
2026-Mar-10 02:21:47.128034
curl \
2026-Mar-10 02:21:47.128034
libssl3 \
2026-Mar-10 02:21:47.128034
&& rm -rf /var/lib/apt/lists/*
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
# Install latest yt-dlp binary (newer than Debian package).
2026-Mar-10 02:21:47.128034
RUN set -eux; \
2026-Mar-10 02:21:47.128034
arch="$(dpkg --print-architecture)"; \
2026-Mar-10 02:21:47.128034
case "$arch" in \
2026-Mar-10 02:21:47.128034
amd64) ytdlp_asset="yt-dlp_linux" ;; \
2026-Mar-10 02:21:47.128034
arm64) ytdlp_asset="yt-dlp_linux_aarch64" ;; \
2026-Mar-10 02:21:47.128034
*) echo "Unsupported architecture: $arch" >&2; exit 1 ;; \
2026-Mar-10 02:21:47.128034
esac; \
2026-Mar-10 02:21:47.128034
curl -fL "https://github.com/yt-dlp/yt-dlp/releases/latest/download/${ytdlp_asset}" -o /usr/local/bin/yt-dlp; \
2026-Mar-10 02:21:47.128034
chmod +x /usr/local/bin/yt-dlp; \
2026-Mar-10 02:21:47.128034
/usr/local/bin/yt-dlp --version
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
# Create non-root user
2026-Mar-10 02:21:47.128034
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
# Copy binary from builder
2026-Mar-10 02:21:47.128034
COPY --from=builder /app/target/release/api-server /usr/local/bin/
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
# Create directories
2026-Mar-10 02:21:47.128034
RUN mkdir -p /app/extractors /app/data && chown -R appuser:appuser /app
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
# Switch to non-root user
2026-Mar-10 02:21:47.128034
USER appuser
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
# Environment variables
2026-Mar-10 02:21:47.128034
ENV PORT=3068
2026-Mar-10 02:21:47.128034
ENV EXTRACTOR_DIR=/app/extractors
2026-Mar-10 02:21:47.128034
ENV YTDLP_PATH=/usr/local/bin/yt-dlp
2026-Mar-10 02:21:47.128034
ENV RUST_LOG=info
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
# Expose port
2026-Mar-10 02:21:47.128034
EXPOSE 3068
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
# Health check
2026-Mar-10 02:21:47.128034
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Mar-10 02:21:47.128034
CMD curl -f http://localhost:3068/health || exit 1
2026-Mar-10 02:21:47.128034
2026-Mar-10 02:21:47.128034
# Run the server
2026-Mar-10 02:21:47.128034
CMD ["api-server"]
2026-Mar-10 02:21:47.488367
Added 90 ARG declarations to Dockerfile for service api (multi-stage build, added to 3 stages).
2026-Mar-10 02:21:47.837361
[CMD]: docker exec woc844so8wg0cks08cwcgowg bash -c 'test -f /artifacts/woc844so8wg0cks08cwcgowg/docker/Dockerfile.worker && echo 'exists' || echo 'not found''
2026-Mar-10 02:21:47.837361
exists
2026-Mar-10 02:21:48.181952
[CMD]: docker exec woc844so8wg0cks08cwcgowg bash -c 'cat /artifacts/woc844so8wg0cks08cwcgowg/docker/Dockerfile.worker'
2026-Mar-10 02:21:48.181952
# Dockerfile for mux worker deployment
2026-Mar-10 02:21:48.181952
2026-Mar-10 02:21:48.181952
# Stage 0: Build extractor TypeScript to IIFE format (required by crates/extractor/build.rs)
2026-Mar-10 02:21:48.181952
FROM node:22-alpine AS js-builder
2026-Mar-10 02:21:48.181952
2026-Mar-10 02:21:48.181952
WORKDIR /app
2026-Mar-10 02:21:48.181952
2026-Mar-10 02:21:48.181952
RUN npm install -g pnpm
2026-Mar-10 02:21:48.181952
2026-Mar-10 02:21:48.181952
COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Mar-10 02:21:48.181952
COPY packages/api-client/package.json ./packages/api-client/
2026-Mar-10 02:21:48.181952
COPY apps/injector/package.json ./apps/injector/
2026-Mar-10 02:21:48.181952
COPY packages/ ./packages/
2026-Mar-10 02:21:48.181952
COPY apps/injector/ ./apps/injector/
2026-Mar-10 02:21:48.181952
COPY extractors/ ./extractors/
2026-Mar-10 02:21:48.181952
2026-Mar-10 02:21:48.181952
RUN pnpm install --frozen-lockfile
2026-Mar-10 02:21:48.181952
RUN mkdir -p extractors/dist && \
2026-Mar-10 02:21:48.181952
npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js && \
2026-Mar-10 02:21:48.181952
npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-10 02:21:48.181952
2026-Mar-10 02:21:48.181952
# Stage 1: Rust builder
2026-Mar-10 02:21:48.181952
FROM rust:1.91-bookworm AS builder
2026-Mar-10 02:21:48.181952
2026-Mar-10 02:21:48.181952
WORKDIR /app
2026-Mar-10 02:21:48.181952
2026-Mar-10 02:21:48.181952
RUN apt-get update && apt-get install -y \
2026-Mar-10 02:21:48.181952
pkg-config \
2026-Mar-10 02:21:48.181952
libssl-dev \
2026-Mar-10 02:21:48.181952
&& rm -rf /var/lib/apt/lists/*
2026-Mar-10 02:21:48.181952
2026-Mar-10 02:21:48.181952
COPY Cargo.toml ./
2026-Mar-10 02:21:48.181952
COPY Cargo.lock ./
2026-Mar-10 02:21:48.181952
COPY crates/ ./crates/
2026-Mar-10 02:21:48.181952
COPY config/ ./config/
2026-Mar-10 02:21:48.181952
COPY extractors/ ./extractors/
2026-Mar-10 02:21:48.181952
COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-10 02:21:48.181952
2026-Mar-10 02:21:48.181952
RUN cargo build --release --bin mux-worker
2026-Mar-10 02:21:48.181952
2026-Mar-10 02:21:48.181952
# Stage 2: Runtime
2026-Mar-10 02:21:48.181952
FROM debian:bookworm-slim AS runtime
2026-Mar-10 02:21:48.181952
2026-Mar-10 02:21:48.181952
WORKDIR /app
2026-Mar-10 02:21:48.181952
2026-Mar-10 02:21:48.181952
RUN apt-get update && apt-get install -y \
2026-Mar-10 02:21:48.181952
ca-certificates \
2026-Mar-10 02:21:48.181952
curl \
2026-Mar-10 02:21:48.181952
libssl3 \
2026-Mar-10 02:21:48.181952
&& rm -rf /var/lib/apt/lists/*
2026-Mar-10 02:21:48.181952
2026-Mar-10 02:21:48.181952
RUN set -eux; \
2026-Mar-10 02:21:48.181952
arch="$(dpkg --print-architecture)"; \
2026-Mar-10 02:21:48.181952
case "$arch" in \
2026-Mar-10 02:21:48.181952
amd64) ytdlp_asset="yt-dlp_linux" ;; \
2026-Mar-10 02:21:48.181952
arm64) ytdlp_asset="yt-dlp_linux_aarch64" ;; \
2026-Mar-10 02:21:48.181952
*) echo "Unsupported architecture: $arch" >&2; exit 1 ;; \
2026-Mar-10 02:21:48.181952
esac; \
2026-Mar-10 02:21:48.181952
curl -fL "https://github.com/yt-dlp/yt-dlp/releases/latest/download/${ytdlp_asset}" -o /usr/local/bin/yt-dlp; \
2026-Mar-10 02:21:48.181952
chmod +x /usr/local/bin/yt-dlp; \
2026-Mar-10 02:21:48.181952
/usr/local/bin/yt-dlp --version
2026-Mar-10 02:21:48.181952
2026-Mar-10 02:21:48.181952
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Mar-10 02:21:48.181952
2026-Mar-10 02:21:48.181952
COPY --from=builder /app/target/release/mux-worker /usr/local/bin/
2026-Mar-10 02:21:48.181952
2026-Mar-10 02:21:48.181952
RUN mkdir -p /app/extractors /app/mux-artifacts && chown -R appuser:appuser /app
2026-Mar-10 02:21:48.181952
2026-Mar-10 02:21:48.181952
USER appuser
2026-Mar-10 02:21:48.181952
2026-Mar-10 02:21:48.181952
ENV EXTRACTOR_DIR=/app/extractors
2026-Mar-10 02:21:48.181952
ENV YTDLP_PATH=/usr/local/bin/yt-dlp
2026-Mar-10 02:21:48.181952
ENV RUST_LOG=info
2026-Mar-10 02:21:48.181952
ENV MUX_JOB_OUTPUT_DIR=/app/mux-artifacts
2026-Mar-10 02:21:48.181952
2026-Mar-10 02:21:48.181952
CMD ["mux-worker"]
2026-Mar-10 02:21:48.540186
Added 90 ARG declarations to Dockerfile for service worker (multi-stage build, added to 3 stages).
2026-Mar-10 02:21:48.886724
[CMD]: docker exec woc844so8wg0cks08cwcgowg bash -c 'test -f /artifacts/woc844so8wg0cks08cwcgowg/docker/Dockerfile.frontend && echo 'exists' || echo 'not found''
2026-Mar-10 02:21:48.886724
exists
2026-Mar-10 02:21:49.229035
[CMD]: docker exec woc844so8wg0cks08cwcgowg bash -c 'cat /artifacts/woc844so8wg0cks08cwcgowg/docker/Dockerfile.frontend'
2026-Mar-10 02:21:49.229035
# Dockerfile for frontend (SvelteKit Node server)
2026-Mar-10 02:21:49.229035
# Copy ALL source files BEFORE npm install so svelte-kit sync (prepare script)
2026-Mar-10 02:21:49.229035
# can find svelte.config.js and generate .svelte-kit/ correctly.
2026-Mar-10 02:21:49.229035
2026-Mar-10 02:21:49.229035
FROM node:22-alpine AS builder
2026-Mar-10 02:21:49.229035
2026-Mar-10 02:21:49.229035
WORKDIR /app
2026-Mar-10 02:21:49.229035
2026-Mar-10 02:21:49.229035
# Copy all frontend source files first (node_modules excluded via .dockerignore)
2026-Mar-10 02:21:49.229035
COPY frontend/ ./
2026-Mar-10 02:21:49.229035
COPY config/ /config/
2026-Mar-10 02:21:49.229035
2026-Mar-10 02:21:49.229035
# Install — prepare script runs svelte-kit sync with svelte.config.js available
2026-Mar-10 02:21:49.229035
RUN npm install
2026-Mar-10 02:21:49.229035
2026-Mar-10 02:21:49.229035
# Build-time public API URL (embedded into client bundle by Vite)
2026-Mar-10 02:21:49.229035
# Runtime env is too late for import.meta.env in browser bundle.
2026-Mar-10 02:21:49.229035
ARG VITE_API_URL
2026-Mar-10 02:21:49.229035
ENV VITE_API_URL=${VITE_API_URL}
2026-Mar-10 02:21:49.229035
RUN test -n "$VITE_API_URL" || (echo "VITE_API_URL build arg is required" && exit 1)
2026-Mar-10 02:21:49.229035
2026-Mar-10 02:21:49.229035
# Generate Paraglide runtime/messages from frontend/messages/* before Vite build
2026-Mar-10 02:21:49.229035
RUN npm run paraglide:compile
2026-Mar-10 02:21:49.229035
2026-Mar-10 02:21:49.229035
# Build via programmatic API — bypasses vite.config.ts loading (esbuild issues on Alpine)
2026-Mar-10 02:21:49.229035
RUN node build-docker.mjs
2026-Mar-10 02:21:49.229035
2026-Mar-10 02:21:49.229035
# Runtime
2026-Mar-10 02:21:49.229035
FROM node:22-alpine AS runtime
2026-Mar-10 02:21:49.229035
2026-Mar-10 02:21:49.229035
WORKDIR /app
2026-Mar-10 02:21:49.229035
2026-Mar-10 02:21:49.229035
COPY --from=builder /app/build ./build
2026-Mar-10 02:21:49.229035
COPY --from=builder /app/package.json ./
2026-Mar-10 02:21:49.229035
COPY --from=builder /app/package-lock.json ./
2026-Mar-10 02:21:49.229035
2026-Mar-10 02:21:49.229035
# Runtime needs server-side deps (better-auth, pg) used by hooks/routes
2026-Mar-10 02:21:49.229035
RUN npm ci --omit=dev
2026-Mar-10 02:21:49.229035
2026-Mar-10 02:21:49.229035
ENV PORT=5168
2026-Mar-10 02:21:49.229035
ENV HOST=0.0.0.0
2026-Mar-10 02:21:49.229035
2026-Mar-10 02:21:49.229035
EXPOSE 5168
2026-Mar-10 02:21:49.229035
2026-Mar-10 02:21:49.229035
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Mar-10 02:21:49.229035
CMD wget -qO- http://127.0.0.1:5168 || exit 1
2026-Mar-10 02:21:49.229035
2026-Mar-10 02:21:49.229035
CMD ["node", "build"]
2026-Mar-10 02:21:49.564395
Added 60 ARG declarations to Dockerfile for service frontend (multi-stage build, added to 2 stages).
2026-Mar-10 02:21:49.570303
Pulling & building required images.
2026-Mar-10 02:21:49.612471
Creating build-time .env file in /artifacts (outside Docker context).
2026-Mar-10 02:21:49.961887
Adding build arguments to Docker Compose build command.
2026-Mar-10 02:21:50.421491
[CMD]: docker exec woc844so8wg0cks08cwcgowg bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/build-time.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/woc844so8wg0cks08cwcgowg -f /artifacts/woc844so8wg0cks08cwcgowg/docker/docker-compose.server.yml build --pull --build-arg COOLIFY_URL --build-arg COOLIFY_FQDN --build-arg SERVICE_URL_API --build-arg SERVICE_FQDN_FRONTEND --build-arg SERVICE_FQDN_API --build-arg ORIGIN --build-arg SERVICE_URL_FRONTEND --build-arg WHOP_WEBHOOK_SECRET --build-arg BETTER_AUTH_TRUSTED_ORIGINS --build-arg POSTGRES_PASSWORD --build-arg BETTER_AUTH_SECRET --build-arg SOCKS5_PROXY_URL --build-arg GOOGLE_CLIENT_ID --build-arg GOOGLE_CLIENT_SECRET --build-arg WHOP_PLAN_ID --build-arg VITE_API_URL --build-arg DOCKER_REDIS_URL --build-arg PROXY_LIST --build-arg MUX_QUEUE_STREAM --build-arg MUX_DIRECT_DOWNLOAD --build-arg MUX_ARTIFACT_BACKEND --build-arg S3_BUCKET_NAME --build-arg S3_ACCESS_KEY_ID --build-arg S3_SECRET_ACCESS_KEY --build-arg S3_REGION --build-arg S3_ENDPOINT --build-arg MUX_ARTIFACT_TTL_SECS --build-arg MUX_CLEANUP_INTERVAL_SECS --build-arg REDIS_URL --build-arg MUX_FILE_TICKET_TTL_SECS --build-arg COOLIFY_BUILD_SECRETS_HASH=3e99cefcd03abf1e712d5cec36a4cce835391695da6958216a57b079deecf05f'
2026-Mar-10 02:21:50.421491
#1 [internal] load local bake definitions
2026-Mar-10 02:21:50.645374
#1 reading from stdin 12.77kB done
2026-Mar-10 02:21:50.645374
#1 DONE 0.0s
2026-Mar-10 02:21:50.645374
2026-Mar-10 02:21:50.645374
#2 [worker internal] load build definition from Dockerfile.worker
2026-Mar-10 02:21:50.645374
#2 transferring dockerfile: 4.23kB done
2026-Mar-10 02:21:50.645374
#2 DONE 0.0s
2026-Mar-10 02:21:50.645374
2026-Mar-10 02:21:50.645374
#3 [api internal] load build definition from Dockerfile.api
2026-Mar-10 02:21:50.645374
#3 transferring dockerfile: 5.30kB done
2026-Mar-10 02:21:50.645374
#3 DONE 0.0s
2026-Mar-10 02:21:50.645374
2026-Mar-10 02:21:50.645374
#4 [frontend internal] load build definition from Dockerfile.frontend
2026-Mar-10 02:21:50.645374
#4 transferring dockerfile: 2.75kB done
2026-Mar-10 02:21:50.645374
#4 DONE 0.0s
2026-Mar-10 02:21:50.645374
2026-Mar-10 02:21:50.645374
#5 [frontend internal] load metadata for docker.io/library/node:22-alpine
2026-Mar-10 02:21:51.692062
#5 DONE 1.1s
2026-Mar-10 02:21:51.692062
2026-Mar-10 02:21:51.692062
#6 [api internal] load metadata for docker.io/library/rust:1.91-bookworm
2026-Mar-10 02:21:51.692062
#6 DONE 1.1s
2026-Mar-10 02:21:51.692062
2026-Mar-10 02:21:51.692062
#7 [worker internal] load metadata for docker.io/library/debian:bookworm-slim
2026-Mar-10 02:21:51.692062
#7 DONE 1.1s
2026-Mar-10 02:21:51.692062
2026-Mar-10 02:21:51.692062
#8 [worker internal] load .dockerignore
2026-Mar-10 02:21:51.692062
#8 transferring context: 341B done
2026-Mar-10 02:21:51.692062
#8 DONE 0.0s
2026-Mar-10 02:21:51.692062
2026-Mar-10 02:21:51.692062
#9 [api runtime 1/7] FROM docker.io/library/debian:bookworm-slim@sha256:74d56e3931e0d5a1dd51f8c8a2466d21de84a271cd3b5a733b803aa91abf4421
2026-Mar-10 02:21:51.692062
#9 resolve docker.io/library/debian:bookworm-slim@sha256:74d56e3931e0d5a1dd51f8c8a2466d21de84a271cd3b5a733b803aa91abf4421 0.0s done
2026-Mar-10 02:21:51.692062
#9 DONE 0.0s
2026-Mar-10 02:21:51.692062
2026-Mar-10 02:21:51.692062
#10 [worker builder 1/8] FROM docker.io/library/node:22-alpine@sha256:8094c002d08262dba12645a3b4a15cd6cd627d30bc782f53229a2ec13ee22a00
2026-Mar-10 02:21:51.692062
#10 resolve docker.io/library/node:22-alpine@sha256:8094c002d08262dba12645a3b4a15cd6cd627d30bc782f53229a2ec13ee22a00 0.0s done
2026-Mar-10 02:21:51.692062
#10 DONE 0.0s
2026-Mar-10 02:21:51.692062
2026-Mar-10 02:21:51.692062
#11 [api builder  1/10] FROM docker.io/library/rust:1.91-bookworm@sha256:c1e5f19e773b7878c3f7a805dd00a495e747acbdc76fb2337a4ebf0418896b33
2026-Mar-10 02:21:51.692062
#11 resolve docker.io/library/rust:1.91-bookworm@sha256:c1e5f19e773b7878c3f7a805dd00a495e747acbdc76fb2337a4ebf0418896b33 0.0s done
2026-Mar-10 02:21:51.692062
#11 DONE 0.0s
2026-Mar-10 02:21:51.692062
2026-Mar-10 02:21:51.692062
#12 [worker internal] load build context
2026-Mar-10 02:21:51.692062
#12 transferring context: 818.75kB 0.0s done
2026-Mar-10 02:21:51.795897
#12 transferring context: 818.75kB 0.0s done
2026-Mar-10 02:21:51.795897
#12 DONE 0.0s
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#13 [frontend internal] load build context
2026-Mar-10 02:21:51.795897
#13 transferring context: 1.47MB 0.0s done
2026-Mar-10 02:21:51.795897
#13 DONE 0.0s
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#14 [frontend runtime 5/6] COPY --from=builder /app/package-lock.json ./
2026-Mar-10 02:21:51.795897
#14 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#15 [frontend runtime 3/6] COPY --from=builder /app/build ./build
2026-Mar-10 02:21:51.795897
#15 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#16 [frontend builder 7/8] RUN npm run paraglide:compile
2026-Mar-10 02:21:51.795897
#16 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#17 [frontend builder 6/8] RUN test -n "https://api-download.khoadangbui.online" || (echo "VITE_API_URL build arg is required" && exit 1)
2026-Mar-10 02:21:51.795897
#17 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#18 [frontend builder 3/8] COPY frontend/ ./
2026-Mar-10 02:21:51.795897
#18 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#19 [frontend builder 8/8] RUN node build-docker.mjs
2026-Mar-10 02:21:51.795897
#19 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#20 [frontend builder 4/8] COPY config/ /config/
2026-Mar-10 02:21:51.795897
#20 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#21 [frontend builder 5/8] RUN npm install
2026-Mar-10 02:21:51.795897
#21 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#22 [frontend runtime 4/6] COPY --from=builder /app/package.json ./
2026-Mar-10 02:21:51.795897
#22 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#23 [worker js-builder  7/11] COPY packages/ ./packages/
2026-Mar-10 02:21:51.795897
#23 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#24 [worker js-builder 11/11] RUN mkdir -p extractors/dist &&     npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js &&     npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-10 02:21:51.795897
#24 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#25 [worker js-builder  8/11] COPY apps/injector/ ./apps/injector/
2026-Mar-10 02:21:51.795897
#25 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#26 [worker js-builder  9/11] COPY extractors/ ./extractors/
2026-Mar-10 02:21:51.795897
#26 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#27 [worker js-builder 10/11] RUN pnpm install --frozen-lockfile
2026-Mar-10 02:21:51.795897
#27 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#28 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-10 02:21:51.795897
#28 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#29 [worker builder  9/10] COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-10 02:21:51.795897
#29 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#30 [worker runtime 6/7] COPY --from=builder /app/target/release/mux-worker /usr/local/bin/
2026-Mar-10 02:21:51.795897
#30 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#31 [worker builder  8/10] COPY extractors/ ./extractors/
2026-Mar-10 02:21:51.795897
#31 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#32 [frontend runtime 6/6] RUN npm ci --omit=dev
2026-Mar-10 02:21:51.795897
#32 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#33 [api js-builder  8/12] COPY packages/ ./packages/
2026-Mar-10 02:21:51.795897
#33 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#34 [api builder  3/10] RUN apt-get update && apt-get install -y     pkg-config     libssl-dev     && rm -rf /var/lib/apt/lists/*
2026-Mar-10 02:21:51.795897
#34 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#35 [api runtime 4/7] RUN set -eux;     arch="$(dpkg --print-architecture)";     case "$arch" in       amd64) ytdlp_asset="yt-dlp_linux" ;;       arm64) ytdlp_asset="yt-dlp_linux_aarch64" ;;       *) echo "Unsupported architecture: $arch" >&2; exit 1 ;;     esac;     curl -fL "https://github.com/yt-dlp/yt-dlp/releases/latest/download/${ytdlp_asset}" -o /usr/local/bin/yt-dlp;     chmod +x /usr/local/bin/yt-dlp;     /usr/local/bin/yt-dlp --version
2026-Mar-10 02:21:51.795897
#35 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#36 [api builder  9/11] COPY extractors/ ./extractors/
2026-Mar-10 02:21:51.795897
#36 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#37 [api builder  6/10] COPY crates/ ./crates/
2026-Mar-10 02:21:51.795897
#37 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#38 [api builder  8/11] COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Mar-10 02:21:51.795897
#38 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#39 [api builder 10/11] COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-10 02:21:51.795897
#39 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#40 [api builder  2/10] WORKDIR /app
2026-Mar-10 02:21:51.795897
#40 CACHED
2026-Mar-10 02:21:51.795897
2026-Mar-10 02:21:51.795897
#41 [api js-builder 10/12] RUN pnpm --filter @downloadtool/injector build
2026-Mar-10 02:21:51.795897
#41 CACHED
2026-Mar-10 02:21:51.798151
#42 [api js-builder  4/11] COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Mar-10 02:21:51.798151
#42 CACHED
2026-Mar-10 02:21:51.798151
2026-Mar-10 02:21:51.798151
#43 [api js-builder  3/11] RUN npm install -g pnpm
2026-Mar-10 02:21:51.798151
#43 CACHED
2026-Mar-10 02:21:51.798151
2026-Mar-10 02:21:51.798151
#44 [api runtime 6/7] COPY --from=builder /app/target/release/api-server /usr/local/bin/
2026-Mar-10 02:21:51.798151
#44 CACHED
2026-Mar-10 02:21:51.798151
2026-Mar-10 02:21:51.798151
#45 [api js-builder  9/12] RUN pnpm install --frozen-lockfile
2026-Mar-10 02:21:51.798151
#45 CACHED
2026-Mar-10 02:21:51.798151
2026-Mar-10 02:21:51.798151
#46 [api builder  7/10] COPY config/ ./config/
2026-Mar-10 02:21:51.798151
#46 CACHED
2026-Mar-10 02:21:51.798151
2026-Mar-10 02:21:51.798151
#47 [api builder  4/10] COPY Cargo.toml ./
2026-Mar-10 02:21:51.798151
#47 CACHED
2026-Mar-10 02:21:51.798151
2026-Mar-10 02:21:51.798151
#48 [api js-builder 12/12] RUN mkdir -p extractors/dist &&     npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js &&     npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-10 02:21:51.798151
#48 CACHED
2026-Mar-10 02:21:51.798151
2026-Mar-10 02:21:51.798151
#49 [api runtime 2/7] WORKDIR /app
2026-Mar-10 02:21:51.798151
#49 CACHED
2026-Mar-10 02:21:51.798151
2026-Mar-10 02:21:51.798151
#50 [api builder 2/8] WORKDIR /app
2026-Mar-10 02:21:51.798151
#50 CACHED
2026-Mar-10 02:21:51.798151
2026-Mar-10 02:21:51.798151
#51 [api js-builder  5/11] COPY packages/api-client/package.json ./packages/api-client/
2026-Mar-10 02:21:51.798151
#51 CACHED
2026-Mar-10 02:21:51.798151
2026-Mar-10 02:21:51.798151
#52 [api js-builder 11/12] COPY extractors/ ./extractors/
2026-Mar-10 02:21:51.798151
#52 CACHED
2026-Mar-10 02:21:51.798151
2026-Mar-10 02:21:51.798151
#53 [api js-builder  7/12] COPY apps/injector/ ./apps/injector/
2026-Mar-10 02:21:51.798151
#53 CACHED
2026-Mar-10 02:21:51.798151
2026-Mar-10 02:21:51.798151
#54 [api runtime 3/7] RUN apt-get update && apt-get install -y     ca-certificates     curl     libssl3     && rm -rf /var/lib/apt/lists/*
2026-Mar-10 02:21:51.798151
#54 CACHED
2026-Mar-10 02:21:51.798151
2026-Mar-10 02:21:51.798151
#55 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-10 02:21:51.798151
#55 CACHED
2026-Mar-10 02:21:51.798151
2026-Mar-10 02:21:51.798151
#56 [api js-builder  6/11] COPY apps/injector/package.json ./apps/injector/
2026-Mar-10 02:21:51.798151
#56 CACHED
2026-Mar-10 02:21:51.798151
2026-Mar-10 02:21:51.798151
#57 [api runtime 5/7] RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Mar-10 02:21:51.798151
#57 CACHED
2026-Mar-10 02:21:51.798151
2026-Mar-10 02:21:51.798151
#58 [api builder  5/10] COPY Cargo.lock ./
2026-Mar-10 02:21:51.798151
#58 CACHED
2026-Mar-10 02:21:51.798151
2026-Mar-10 02:21:51.798151
#59 [worker runtime 7/7] RUN mkdir -p /app/extractors /app/mux-artifacts && chown -R appuser:appuser /app
2026-Mar-10 02:21:51.798151
#59 CACHED
2026-Mar-10 02:21:51.798151
2026-Mar-10 02:21:51.798151
#60 [api runtime 7/7] RUN mkdir -p /app/extractors /app/data && chown -R appuser:appuser /app
2026-Mar-10 02:21:51.798151
#60 CACHED
2026-Mar-10 02:21:51.798151
2026-Mar-10 02:21:51.798151
#61 [api] exporting to image
2026-Mar-10 02:21:51.798151
#61 exporting layers done
2026-Mar-10 02:21:51.798151
#61 exporting manifest sha256:57df27dea0155dbd599f3916a09b0135099e477ef5becb8a3e107d67cedeba6d done
2026-Mar-10 02:21:51.798151
#61 exporting config sha256:b4343efbb7a49a4d93057f19c585c14404a73e94b85839e79a6ae56625909bfc done
2026-Mar-10 02:21:51.798151
#61 exporting attestation manifest sha256:e582d18e8c0176875fb00d341e084bb51aa92b39e22c2a6cc35331a33387f49b 0.0s done
2026-Mar-10 02:21:51.798151
#61 exporting manifest list sha256:366d93557f2cfd6919d7d33753321ad006cd44d80e7182e72f3a033435a4f0f3 done
2026-Mar-10 02:21:51.798151
#61 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_api:6e96c70cad023efec322d476ffeb62cf7afd80e0 done
2026-Mar-10 02:21:51.798151
#61 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_api:6e96c70cad023efec322d476ffeb62cf7afd80e0 done
2026-Mar-10 02:21:51.798151
#61 DONE 0.1s
2026-Mar-10 02:21:51.798151
2026-Mar-10 02:21:51.798151
#62 [worker] exporting to image
2026-Mar-10 02:21:51.798151
#62 exporting layers done
2026-Mar-10 02:21:51.798151
#62 exporting manifest sha256:1c68706a59e921993c437c05287ef2860711645d14dbf71bea222d526adfb433 done
2026-Mar-10 02:21:51.798151
#62 exporting config sha256:b9175a2a3c5c1826c794c1493f8edebab04f34ff72ae5d2df82f8741012a5fc7 done
2026-Mar-10 02:21:51.798151
#62 exporting attestation manifest sha256:2830bb3c02dab3084ee1cbca71b9fa87629a685d9d156be2a527600b47c65bfd 0.0s done
2026-Mar-10 02:21:51.798151
#62 exporting manifest list sha256:f3098a26fd5e9c56409c82fffdd260334e1f2808efc5f849ff4ff8ff13bbfbe7 done
2026-Mar-10 02:21:51.798151
#62 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_worker:6e96c70cad023efec322d476ffeb62cf7afd80e0 done
2026-Mar-10 02:21:51.798151
#62 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_worker:6e96c70cad023efec322d476ffeb62cf7afd80e0 done
2026-Mar-10 02:21:51.905002
#62 DONE 0.1s
2026-Mar-10 02:21:51.905002
2026-Mar-10 02:21:51.905002
#63 [frontend] exporting to image
2026-Mar-10 02:21:51.905002
#63 exporting layers done
2026-Mar-10 02:21:51.905002
#63 exporting manifest sha256:80116d119f721e6b95bea86669e9a41afc9f87846c0c6a9499ad0bb08722adb9 done
2026-Mar-10 02:21:51.905002
#63 exporting config sha256:3c40f57d302cff2764c22c21aae1ba298100a723cfc32e005a83e4d7e2ffee1c done
2026-Mar-10 02:21:51.905002
#63 exporting attestation manifest sha256:2c66d6f1d7551bfcfc503301022c6cc0db5fa95842a10fb929f13cb69bf36aa2 0.0s done
2026-Mar-10 02:21:51.905002
#63 exporting manifest list sha256:fe30f24f10354d8625dcc7dc83cdee0256a79810df2c45430e910a457321f854 done
2026-Mar-10 02:21:51.905002
#63 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:6e96c70cad023efec322d476ffeb62cf7afd80e0 done
2026-Mar-10 02:21:51.905002
#63 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:6e96c70cad023efec322d476ffeb62cf7afd80e0 done
2026-Mar-10 02:21:51.905002
#63 DONE 0.1s
2026-Mar-10 02:21:51.905002
2026-Mar-10 02:21:51.905002
#64 [api] resolving provenance for metadata file
2026-Mar-10 02:21:51.991447
#64 DONE 0.0s
2026-Mar-10 02:21:51.991447
2026-Mar-10 02:21:51.991447
#65 [frontend] resolving provenance for metadata file
2026-Mar-10 02:21:51.991447
#65 DONE 0.0s
2026-Mar-10 02:21:51.991447
2026-Mar-10 02:21:51.991447
#66 [worker] resolving provenance for metadata file
2026-Mar-10 02:21:51.991447
#66 DONE 0.0s
2026-Mar-10 02:21:51.994094
frontend  Built
2026-Mar-10 02:21:51.994094
api  Built
2026-Mar-10 02:21:51.994094
worker  Built
2026-Mar-10 02:21:52.042589
Creating .env file with runtime variables for container.
2026-Mar-10 02:21:52.676825
Removing old containers.
2026-Mar-10 02:21:53.459320
[CMD]: docker stop -t 30 frontend-o8kccgkgwsockoocow8sg88s-021445506257
2026-Mar-10 02:21:53.459320
frontend-o8kccgkgwsockoocow8sg88s-021445506257
2026-Mar-10 02:21:53.783869
[CMD]: docker rm -f frontend-o8kccgkgwsockoocow8sg88s-021445506257
2026-Mar-10 02:21:53.783869
frontend-o8kccgkgwsockoocow8sg88s-021445506257
2026-Mar-10 02:22:24.446508
[CMD]: docker stop -t 30 api-o8kccgkgwsockoocow8sg88s-021445490722
2026-Mar-10 02:22:24.446508
api-o8kccgkgwsockoocow8sg88s-021445490722
2026-Mar-10 02:22:24.768284
[CMD]: docker rm -f api-o8kccgkgwsockoocow8sg88s-021445490722
2026-Mar-10 02:22:24.768284
api-o8kccgkgwsockoocow8sg88s-021445490722
2026-Mar-10 02:22:55.216936
[CMD]: docker stop -t 30 worker-o8kccgkgwsockoocow8sg88s-021445499593
2026-Mar-10 02:22:55.216936
worker-o8kccgkgwsockoocow8sg88s-021445499593
2026-Mar-10 02:22:55.535394
[CMD]: docker rm -f worker-o8kccgkgwsockoocow8sg88s-021445499593
2026-Mar-10 02:22:55.535394
worker-o8kccgkgwsockoocow8sg88s-021445499593
2026-Mar-10 02:22:55.976652
[CMD]: docker stop -t 30 postgres-o8kccgkgwsockoocow8sg88s-021445481555
2026-Mar-10 02:22:55.976652
postgres-o8kccgkgwsockoocow8sg88s-021445481555
2026-Mar-10 02:22:56.285170
[CMD]: docker rm -f postgres-o8kccgkgwsockoocow8sg88s-021445481555
2026-Mar-10 02:22:56.285170
postgres-o8kccgkgwsockoocow8sg88s-021445481555
2026-Mar-10 02:22:56.789414
[CMD]: docker stop -t 30 redis-o8kccgkgwsockoocow8sg88s-021445486598
2026-Mar-10 02:22:56.789414
redis-o8kccgkgwsockoocow8sg88s-021445486598
2026-Mar-10 02:22:57.115444
[CMD]: docker rm -f redis-o8kccgkgwsockoocow8sg88s-021445486598
2026-Mar-10 02:22:57.115444
redis-o8kccgkgwsockoocow8sg88s-021445486598
2026-Mar-10 02:22:57.118749
Starting new application.
2026-Mar-10 02:22:58.185254
[CMD]: docker exec woc844so8wg0cks08cwcgowg bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/woc844so8wg0cks08cwcgowg/.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/woc844so8wg0cks08cwcgowg -f /artifacts/woc844so8wg0cks08cwcgowg/docker/docker-compose.server.yml up -d'
2026-Mar-10 02:22:58.185254
time="2026-03-10T02:22:58Z" level=warning msg="volume \"o8kccgkgwsockoocow8sg88s_postgres-data\" already exists but was not created by Docker Compose. Use `external: true` to use an existing volume"
2026-Mar-10 02:22:58.187776
Container redis-o8kccgkgwsockoocow8sg88s-022146056898  Creating
2026-Mar-10 02:22:58.187776
Container postgres-o8kccgkgwsockoocow8sg88s-022146051516  Creating
2026-Mar-10 02:22:58.549163
Container redis-o8kccgkgwsockoocow8sg88s-022146056898  Created
2026-Mar-10 02:22:58.549163
Container postgres-o8kccgkgwsockoocow8sg88s-022146051516  Created
2026-Mar-10 02:22:58.551812
Container api-o8kccgkgwsockoocow8sg88s-022146061268  Creating
2026-Mar-10 02:22:58.551812
Container worker-o8kccgkgwsockoocow8sg88s-022146070609  Creating
2026-Mar-10 02:22:58.577144
Container api-o8kccgkgwsockoocow8sg88s-022146061268  Created
2026-Mar-10 02:22:58.577144
Container frontend-o8kccgkgwsockoocow8sg88s-022146077647  Creating
2026-Mar-10 02:22:58.579746
Container worker-o8kccgkgwsockoocow8sg88s-022146070609  Created
2026-Mar-10 02:22:58.946770
Container frontend-o8kccgkgwsockoocow8sg88s-022146077647  Created
2026-Mar-10 02:22:58.962270
Container redis-o8kccgkgwsockoocow8sg88s-022146056898  Starting
2026-Mar-10 02:22:58.962270
Container postgres-o8kccgkgwsockoocow8sg88s-022146051516  Starting
2026-Mar-10 02:22:59.080882
Container postgres-o8kccgkgwsockoocow8sg88s-022146051516  Started
2026-Mar-10 02:22:59.088637
Container redis-o8kccgkgwsockoocow8sg88s-022146056898  Started
2026-Mar-10 02:22:59.091281
Container worker-o8kccgkgwsockoocow8sg88s-022146070609  Starting
2026-Mar-10 02:22:59.091281
Container api-o8kccgkgwsockoocow8sg88s-022146061268  Starting
2026-Mar-10 02:22:59.208450
Container worker-o8kccgkgwsockoocow8sg88s-022146070609  Started
2026-Mar-10 02:22:59.216421
Container api-o8kccgkgwsockoocow8sg88s-022146061268  Started
2026-Mar-10 02:22:59.216421
Container frontend-o8kccgkgwsockoocow8sg88s-022146077647  Starting
2026-Mar-10 02:22:59.333410
Container frontend-o8kccgkgwsockoocow8sg88s-022146077647  Started
2026-Mar-10 02:23:00.223303
New container started.
2026-Mar-10 02:23:01.139801
Gracefully shutting down build container: woc844so8wg0cks08cwcgowg
2026-Mar-10 02:23:01.695231
[CMD]: docker stop -t 30 woc844so8wg0cks08cwcgowg
2026-Mar-10 02:23:01.695231
woc844so8wg0cks08cwcgowg