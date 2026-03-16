2026-Mar-16 01:44:42.646825
Starting deployment of khoa280703/downloadtool:main-zoscg4oc04gkwkssg0kw8w8w to localhost.
2026-Mar-16 01:44:43.212582
Preparing container with helper image: ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Mar-16 01:44:43.508010
[CMD]: docker stop -t 30 u88wkggs8ocgkwgoookcgs0s
2026-Mar-16 01:44:43.508010
Error response from daemon: No such container: u88wkggs8ocgkwgoookcgs0s
2026-Mar-16 01:44:43.834390
[CMD]: docker run -d --network coolify --name u88wkggs8ocgkwgoookcgs0s  --rm -v /var/run/docker.sock:/var/run/docker.sock ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Mar-16 01:44:43.834390
de016546497c1ea646025dc078000eb1b59871b1121553c1c9e56958962bb2d2
2026-Mar-16 01:44:45.243640
[CMD]: docker exec u88wkggs8ocgkwgoookcgs0s bash -c 'GIT_SSH_COMMAND="ssh -o ConnectTimeout=30 -p 22 -o Port=22 -o LogLevel=ERROR -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git ls-remote https://github.com/Khoa280703/downloadtool refs/heads/main'
2026-Mar-16 01:44:45.243640
0af48d45e712266903e7550ff44ff5d6aabd6976	refs/heads/main
2026-Mar-16 01:44:45.254055
----------------------------------------
2026-Mar-16 01:44:45.258256
Importing Khoa280703/downloadtool:main (commit sha 0af48d45e712266903e7550ff44ff5d6aabd6976) to /artifacts/u88wkggs8ocgkwgoookcgs0s.
2026-Mar-16 01:44:45.600237
[CMD]: docker exec u88wkggs8ocgkwgoookcgs0s bash -c 'git clone --depth=1 --recurse-submodules --shallow-submodules -b 'main' 'https://github.com/Khoa280703/downloadtool' '/artifacts/u88wkggs8ocgkwgoookcgs0s' && cd '/artifacts/u88wkggs8ocgkwgoookcgs0s' && if [ -f .gitmodules ]; then sed -i "s#git@\(.*\):#https://\1/#g" '/artifacts/u88wkggs8ocgkwgoookcgs0s'/.gitmodules || true && git submodule sync && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git submodule update --init --recursive --depth=1; fi && cd '/artifacts/u88wkggs8ocgkwgoookcgs0s' && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git lfs pull'
2026-Mar-16 01:44:45.600237
Cloning into '/artifacts/u88wkggs8ocgkwgoookcgs0s'...
2026-Mar-16 01:44:47.862383
[CMD]: docker exec u88wkggs8ocgkwgoookcgs0s bash -c 'cd /artifacts/u88wkggs8ocgkwgoookcgs0s && git log -1 0af48d45e712266903e7550ff44ff5d6aabd6976 --pretty=%B'
2026-Mar-16 01:44:47.862383
fix: simplify mobile fetch button
2026-Mar-16 01:44:52.176570
[CMD]: docker exec u88wkggs8ocgkwgoookcgs0s bash -c 'test -f /artifacts/u88wkggs8ocgkwgoookcgs0s/docker/Dockerfile.api && echo 'exists' || echo 'not found''
2026-Mar-16 01:44:52.176570
exists
2026-Mar-16 01:44:52.521066
[CMD]: docker exec u88wkggs8ocgkwgoookcgs0s bash -c 'cat /artifacts/u88wkggs8ocgkwgoookcgs0s/docker/Dockerfile.api'
2026-Mar-16 01:44:52.521066
# Dockerfile for API service deployment
2026-Mar-16 01:44:52.521066
# Builds the API server and related components without GPU support
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
# Stage 0: Build injector JS (embedded into api crate via include_str! at compile time)
2026-Mar-16 01:44:52.521066
FROM node:22-alpine AS js-builder
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
WORKDIR /app
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
RUN npm install -g pnpm
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
# Copy workspace manifests for pnpm resolution
2026-Mar-16 01:44:52.521066
COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Mar-16 01:44:52.521066
COPY packages/api-client/package.json ./packages/api-client/
2026-Mar-16 01:44:52.521066
COPY apps/injector/package.json ./apps/injector/
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
# Copy injector source and shared packages
2026-Mar-16 01:44:52.521066
COPY apps/injector/ ./apps/injector/
2026-Mar-16 01:44:52.521066
COPY packages/ ./packages/
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
# Install deps and build injector (produces dist/bm.js and dist/youtube-downloader.user.js)
2026-Mar-16 01:44:52.521066
RUN pnpm install --frozen-lockfile
2026-Mar-16 01:44:52.521066
RUN pnpm --filter @downloadtool/injector build
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
# Build extractor TypeScript to IIFE format (required by crates/extractor/build.rs)
2026-Mar-16 01:44:52.521066
COPY extractors/ ./extractors/
2026-Mar-16 01:44:52.521066
RUN mkdir -p extractors/dist && \
2026-Mar-16 01:44:52.521066
npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js && \
2026-Mar-16 01:44:52.521066
npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
# Stage 1: Rust builder
2026-Mar-16 01:44:52.521066
FROM rust:1.91-bookworm AS builder
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
WORKDIR /app
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
# Install dependencies
2026-Mar-16 01:44:52.521066
RUN apt-get update && apt-get install -y \
2026-Mar-16 01:44:52.521066
pkg-config \
2026-Mar-16 01:44:52.521066
libssl-dev \
2026-Mar-16 01:44:52.521066
&& rm -rf /var/lib/apt/lists/*
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
# Copy workspace configuration
2026-Mar-16 01:44:52.521066
COPY Cargo.toml ./
2026-Mar-16 01:44:52.521066
COPY Cargo.lock ./
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
# Copy all crates
2026-Mar-16 01:44:52.521066
COPY crates/ ./crates/
2026-Mar-16 01:44:52.521066
COPY config/ ./config/
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
# Copy injector dist (required by include_str! in crates/api/src/routes/static_files.rs)
2026-Mar-16 01:44:52.521066
COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
# Copy extractor source + pre-built IIFE dist (built by js-builder stage)
2026-Mar-16 01:44:52.521066
COPY extractors/ ./extractors/
2026-Mar-16 01:44:52.521066
COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
# Build the release binary
2026-Mar-16 01:44:52.521066
RUN cargo build --release --bin api-server
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
# Stage 2: Runtime
2026-Mar-16 01:44:52.521066
FROM debian:bookworm-slim AS runtime
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
WORKDIR /app
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
# Install runtime dependencies
2026-Mar-16 01:44:52.521066
RUN apt-get update && apt-get install -y \
2026-Mar-16 01:44:52.521066
ca-certificates \
2026-Mar-16 01:44:52.521066
curl \
2026-Mar-16 01:44:52.521066
libssl3 \
2026-Mar-16 01:44:52.521066
&& rm -rf /var/lib/apt/lists/*
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
# Install latest yt-dlp binary (newer than Debian package).
2026-Mar-16 01:44:52.521066
RUN set -eux; \
2026-Mar-16 01:44:52.521066
arch="$(dpkg --print-architecture)"; \
2026-Mar-16 01:44:52.521066
case "$arch" in \
2026-Mar-16 01:44:52.521066
amd64) ytdlp_asset="yt-dlp_linux" ;; \
2026-Mar-16 01:44:52.521066
arm64) ytdlp_asset="yt-dlp_linux_aarch64" ;; \
2026-Mar-16 01:44:52.521066
*) echo "Unsupported architecture: $arch" >&2; exit 1 ;; \
2026-Mar-16 01:44:52.521066
esac; \
2026-Mar-16 01:44:52.521066
curl -fL "https://github.com/yt-dlp/yt-dlp/releases/latest/download/${ytdlp_asset}" -o /usr/local/bin/yt-dlp; \
2026-Mar-16 01:44:52.521066
chmod +x /usr/local/bin/yt-dlp; \
2026-Mar-16 01:44:52.521066
/usr/local/bin/yt-dlp --version
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
# Create non-root user
2026-Mar-16 01:44:52.521066
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
# Copy binary from builder
2026-Mar-16 01:44:52.521066
COPY --from=builder /app/target/release/api-server /usr/local/bin/
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
# Create directories
2026-Mar-16 01:44:52.521066
RUN mkdir -p /app/extractors /app/data /app/proxy-state && chown -R appuser:appuser /app
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
# Switch to non-root user
2026-Mar-16 01:44:52.521066
USER appuser
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
# Environment variables
2026-Mar-16 01:44:52.521066
ENV PORT=3068
2026-Mar-16 01:44:52.521066
ENV EXTRACTOR_DIR=/app/extractors
2026-Mar-16 01:44:52.521066
ENV YTDLP_PATH=/usr/local/bin/yt-dlp
2026-Mar-16 01:44:52.521066
ENV RUST_LOG=info
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
# Expose port
2026-Mar-16 01:44:52.521066
EXPOSE 3068
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
# Health check
2026-Mar-16 01:44:52.521066
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Mar-16 01:44:52.521066
CMD curl -f http://localhost:3068/health || exit 1
2026-Mar-16 01:44:52.521066
2026-Mar-16 01:44:52.521066
# Run the server
2026-Mar-16 01:44:52.521066
CMD ["api-server"]
2026-Mar-16 01:44:52.887228
Added 93 ARG declarations to Dockerfile for service api (multi-stage build, added to 3 stages).
2026-Mar-16 01:44:53.237485
[CMD]: docker exec u88wkggs8ocgkwgoookcgs0s bash -c 'test -f /artifacts/u88wkggs8ocgkwgoookcgs0s/docker/Dockerfile.worker && echo 'exists' || echo 'not found''
2026-Mar-16 01:44:53.237485
exists
2026-Mar-16 01:44:53.584872
[CMD]: docker exec u88wkggs8ocgkwgoookcgs0s bash -c 'cat /artifacts/u88wkggs8ocgkwgoookcgs0s/docker/Dockerfile.worker'
2026-Mar-16 01:44:53.584872
# Dockerfile for mux worker deployment
2026-Mar-16 01:44:53.584872
2026-Mar-16 01:44:53.584872
# Stage 0: Build extractor TypeScript to IIFE format (required by crates/extractor/build.rs)
2026-Mar-16 01:44:53.584872
FROM node:22-alpine AS js-builder
2026-Mar-16 01:44:53.584872
2026-Mar-16 01:44:53.584872
WORKDIR /app
2026-Mar-16 01:44:53.584872
2026-Mar-16 01:44:53.584872
RUN npm install -g pnpm
2026-Mar-16 01:44:53.584872
2026-Mar-16 01:44:53.584872
COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Mar-16 01:44:53.584872
COPY packages/api-client/package.json ./packages/api-client/
2026-Mar-16 01:44:53.584872
COPY apps/injector/package.json ./apps/injector/
2026-Mar-16 01:44:53.584872
COPY packages/ ./packages/
2026-Mar-16 01:44:53.584872
COPY apps/injector/ ./apps/injector/
2026-Mar-16 01:44:53.584872
COPY extractors/ ./extractors/
2026-Mar-16 01:44:53.584872
2026-Mar-16 01:44:53.584872
RUN pnpm install --frozen-lockfile
2026-Mar-16 01:44:53.584872
RUN mkdir -p extractors/dist && \
2026-Mar-16 01:44:53.584872
npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js && \
2026-Mar-16 01:44:53.584872
npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-16 01:44:53.584872
2026-Mar-16 01:44:53.584872
# Stage 1: Rust builder
2026-Mar-16 01:44:53.584872
FROM rust:1.91-bookworm AS builder
2026-Mar-16 01:44:53.584872
2026-Mar-16 01:44:53.584872
WORKDIR /app
2026-Mar-16 01:44:53.584872
2026-Mar-16 01:44:53.584872
RUN apt-get update && apt-get install -y \
2026-Mar-16 01:44:53.584872
pkg-config \
2026-Mar-16 01:44:53.584872
libssl-dev \
2026-Mar-16 01:44:53.584872
&& rm -rf /var/lib/apt/lists/*
2026-Mar-16 01:44:53.584872
2026-Mar-16 01:44:53.584872
COPY Cargo.toml ./
2026-Mar-16 01:44:53.584872
COPY Cargo.lock ./
2026-Mar-16 01:44:53.584872
COPY crates/ ./crates/
2026-Mar-16 01:44:53.584872
COPY config/ ./config/
2026-Mar-16 01:44:53.584872
COPY extractors/ ./extractors/
2026-Mar-16 01:44:53.584872
COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-16 01:44:53.584872
2026-Mar-16 01:44:53.584872
RUN cargo build --release --bin mux-worker
2026-Mar-16 01:44:53.584872
2026-Mar-16 01:44:53.584872
# Stage 2: Runtime
2026-Mar-16 01:44:53.584872
FROM debian:bookworm-slim AS runtime
2026-Mar-16 01:44:53.584872
2026-Mar-16 01:44:53.584872
WORKDIR /app
2026-Mar-16 01:44:53.584872
2026-Mar-16 01:44:53.584872
RUN apt-get update && apt-get install -y \
2026-Mar-16 01:44:53.584872
ca-certificates \
2026-Mar-16 01:44:53.584872
curl \
2026-Mar-16 01:44:53.584872
libssl3 \
2026-Mar-16 01:44:53.584872
&& rm -rf /var/lib/apt/lists/*
2026-Mar-16 01:44:53.584872
2026-Mar-16 01:44:53.584872
RUN set -eux; \
2026-Mar-16 01:44:53.584872
arch="$(dpkg --print-architecture)"; \
2026-Mar-16 01:44:53.584872
case "$arch" in \
2026-Mar-16 01:44:53.584872
amd64) ytdlp_asset="yt-dlp_linux" ;; \
2026-Mar-16 01:44:53.584872
arm64) ytdlp_asset="yt-dlp_linux_aarch64" ;; \
2026-Mar-16 01:44:53.584872
*) echo "Unsupported architecture: $arch" >&2; exit 1 ;; \
2026-Mar-16 01:44:53.584872
esac; \
2026-Mar-16 01:44:53.584872
curl -fL "https://github.com/yt-dlp/yt-dlp/releases/latest/download/${ytdlp_asset}" -o /usr/local/bin/yt-dlp; \
2026-Mar-16 01:44:53.584872
chmod +x /usr/local/bin/yt-dlp; \
2026-Mar-16 01:44:53.584872
/usr/local/bin/yt-dlp --version
2026-Mar-16 01:44:53.584872
2026-Mar-16 01:44:53.584872
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Mar-16 01:44:53.584872
2026-Mar-16 01:44:53.584872
COPY --from=builder /app/target/release/mux-worker /usr/local/bin/
2026-Mar-16 01:44:53.584872
2026-Mar-16 01:44:53.584872
RUN mkdir -p /app/extractors /app/mux-artifacts /app/proxy-state && chown -R appuser:appuser /app
2026-Mar-16 01:44:53.584872
2026-Mar-16 01:44:53.584872
USER appuser
2026-Mar-16 01:44:53.584872
2026-Mar-16 01:44:53.584872
ENV EXTRACTOR_DIR=/app/extractors
2026-Mar-16 01:44:53.584872
ENV YTDLP_PATH=/usr/local/bin/yt-dlp
2026-Mar-16 01:44:53.584872
ENV RUST_LOG=info
2026-Mar-16 01:44:53.584872
ENV MUX_JOB_OUTPUT_DIR=/app/mux-artifacts
2026-Mar-16 01:44:53.584872
2026-Mar-16 01:44:53.584872
CMD ["mux-worker"]
2026-Mar-16 01:44:53.950462
Added 93 ARG declarations to Dockerfile for service worker (multi-stage build, added to 3 stages).
2026-Mar-16 01:44:54.302940
[CMD]: docker exec u88wkggs8ocgkwgoookcgs0s bash -c 'test -f /artifacts/u88wkggs8ocgkwgoookcgs0s/docker/Dockerfile.frontend && echo 'exists' || echo 'not found''
2026-Mar-16 01:44:54.302940
exists
2026-Mar-16 01:44:54.639912
[CMD]: docker exec u88wkggs8ocgkwgoookcgs0s bash -c 'cat /artifacts/u88wkggs8ocgkwgoookcgs0s/docker/Dockerfile.frontend'
2026-Mar-16 01:44:54.639912
# Dockerfile for frontend (SvelteKit Node server)
2026-Mar-16 01:44:54.639912
# Copy ALL source files BEFORE npm install so svelte-kit sync (prepare script)
2026-Mar-16 01:44:54.639912
# can find svelte.config.js and generate .svelte-kit/ correctly.
2026-Mar-16 01:44:54.639912
2026-Mar-16 01:44:54.639912
FROM node:22-alpine AS builder
2026-Mar-16 01:44:54.639912
2026-Mar-16 01:44:54.639912
WORKDIR /app
2026-Mar-16 01:44:54.639912
2026-Mar-16 01:44:54.639912
# Copy all frontend source files first (node_modules excluded via .dockerignore)
2026-Mar-16 01:44:54.639912
COPY frontend/ ./
2026-Mar-16 01:44:54.639912
COPY config/ /config/
2026-Mar-16 01:44:54.639912
2026-Mar-16 01:44:54.639912
# Install — prepare script runs svelte-kit sync with svelte.config.js available
2026-Mar-16 01:44:54.639912
RUN npm install
2026-Mar-16 01:44:54.639912
2026-Mar-16 01:44:54.639912
# Build-time public API URL (embedded into client bundle by Vite)
2026-Mar-16 01:44:54.639912
# Runtime env is too late for import.meta.env in browser bundle.
2026-Mar-16 01:44:54.639912
ARG VITE_API_URL
2026-Mar-16 01:44:54.639912
ENV VITE_API_URL=${VITE_API_URL}
2026-Mar-16 01:44:54.639912
RUN test -n "$VITE_API_URL" || (echo "VITE_API_URL build arg is required" && exit 1)
2026-Mar-16 01:44:54.639912
2026-Mar-16 01:44:54.639912
# Generate Paraglide runtime/messages from frontend/messages/* before Vite build
2026-Mar-16 01:44:54.639912
RUN npm run paraglide:compile
2026-Mar-16 01:44:54.639912
2026-Mar-16 01:44:54.639912
# Build via programmatic API — bypasses vite.config.ts loading (esbuild issues on Alpine)
2026-Mar-16 01:44:54.639912
RUN node build-docker.mjs
2026-Mar-16 01:44:54.639912
2026-Mar-16 01:44:54.639912
# Runtime
2026-Mar-16 01:44:54.639912
FROM node:22-alpine AS runtime
2026-Mar-16 01:44:54.639912
2026-Mar-16 01:44:54.639912
WORKDIR /app
2026-Mar-16 01:44:54.639912
2026-Mar-16 01:44:54.639912
COPY --from=builder /app/build ./build
2026-Mar-16 01:44:54.639912
COPY --from=builder /app/package.json ./
2026-Mar-16 01:44:54.639912
COPY --from=builder /app/package-lock.json ./
2026-Mar-16 01:44:54.639912
2026-Mar-16 01:44:54.639912
# Runtime needs server-side deps (better-auth, pg) used by hooks/routes
2026-Mar-16 01:44:54.639912
RUN npm ci --omit=dev
2026-Mar-16 01:44:54.639912
2026-Mar-16 01:44:54.639912
ENV PORT=5168
2026-Mar-16 01:44:54.639912
ENV HOST=0.0.0.0
2026-Mar-16 01:44:54.639912
2026-Mar-16 01:44:54.639912
EXPOSE 5168
2026-Mar-16 01:44:54.639912
2026-Mar-16 01:44:54.639912
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Mar-16 01:44:54.639912
CMD wget -qO- http://127.0.0.1:5168 || exit 1
2026-Mar-16 01:44:54.639912
2026-Mar-16 01:44:54.639912
CMD ["node", "build"]
2026-Mar-16 01:44:54.989012
Added 62 ARG declarations to Dockerfile for service frontend (multi-stage build, added to 2 stages).
2026-Mar-16 01:44:54.994857
Pulling & building required images.
2026-Mar-16 01:44:55.039252
Creating build-time .env file in /artifacts (outside Docker context).
2026-Mar-16 01:44:55.402228
Adding build arguments to Docker Compose build command.
2026-Mar-16 01:44:55.885341
[CMD]: docker exec u88wkggs8ocgkwgoookcgs0s bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/build-time.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/u88wkggs8ocgkwgoookcgs0s -f /artifacts/u88wkggs8ocgkwgoookcgs0s/docker/docker-compose.server.yml build --pull --build-arg COOLIFY_URL --build-arg COOLIFY_FQDN --build-arg SERVICE_URL_API --build-arg SERVICE_FQDN_FRONTEND --build-arg SERVICE_FQDN_API --build-arg ORIGIN --build-arg SERVICE_URL_FRONTEND --build-arg WHOP_WEBHOOK_SECRET --build-arg BETTER_AUTH_TRUSTED_ORIGINS --build-arg POSTGRES_PASSWORD --build-arg BETTER_AUTH_SECRET --build-arg SOCKS5_PROXY_URL --build-arg GOOGLE_CLIENT_ID --build-arg GOOGLE_CLIENT_SECRET --build-arg WHOP_PLAN_ID --build-arg VITE_API_URL --build-arg DOCKER_REDIS_URL --build-arg PROXY_LIST --build-arg MUX_QUEUE_STREAM --build-arg MUX_DIRECT_DOWNLOAD --build-arg MUX_ARTIFACT_BACKEND --build-arg S3_BUCKET_NAME --build-arg S3_ACCESS_KEY_ID --build-arg S3_SECRET_ACCESS_KEY --build-arg S3_REGION --build-arg S3_ENDPOINT --build-arg ADMIN_EMAILS --build-arg MUX_ARTIFACT_TTL_SECS --build-arg MUX_CLEANUP_INTERVAL_SECS --build-arg REDIS_URL --build-arg MUX_FILE_TICKET_TTL_SECS --build-arg COOLIFY_BUILD_SECRETS_HASH=147102dc33226fc8d787cf78687ecc4cf02b2168677d826e17042f8d4c5646fc'
2026-Mar-16 01:44:55.885341
#1 [internal] load local bake definitions
2026-Mar-16 01:44:56.100518
#1 reading from stdin 12.93kB done
2026-Mar-16 01:44:56.100518
#1 DONE 0.0s
2026-Mar-16 01:44:56.100518
2026-Mar-16 01:44:56.100518
#2 [worker internal] load build definition from Dockerfile.worker
2026-Mar-16 01:44:56.100518
#2 transferring dockerfile: 4.30kB done
2026-Mar-16 01:44:56.100518
#2 DONE 0.0s
2026-Mar-16 01:44:56.100518
2026-Mar-16 01:44:56.100518
#3 [api internal] load build definition from Dockerfile.api
2026-Mar-16 01:44:56.100518
#3 transferring dockerfile: 5.37kB done
2026-Mar-16 01:44:56.100518
#3 DONE 0.0s
2026-Mar-16 01:44:56.100518
2026-Mar-16 01:44:56.100518
#4 [frontend internal] load build definition from Dockerfile.frontend
2026-Mar-16 01:44:56.100518
#4 transferring dockerfile: 2.79kB done
2026-Mar-16 01:44:56.100518
#4 DONE 0.0s
2026-Mar-16 01:44:56.100518
2026-Mar-16 01:44:56.100518
#5 [frontend internal] load metadata for docker.io/library/node:22-alpine
2026-Mar-16 01:44:58.000139
#5 ...
2026-Mar-16 01:44:58.000139
2026-Mar-16 01:44:58.000139
#6 [worker internal] load metadata for docker.io/library/debian:bookworm-slim
2026-Mar-16 01:44:58.000139
#6 DONE 1.9s
2026-Mar-16 01:44:58.150347
#7 [api internal] load metadata for docker.io/library/rust:1.91-bookworm
2026-Mar-16 01:44:58.619653
#7 DONE 2.6s
2026-Mar-16 01:44:58.619653
2026-Mar-16 01:44:58.619653
#5 [api internal] load metadata for docker.io/library/node:22-alpine
2026-Mar-16 01:44:58.619653
#5 DONE 2.6s
2026-Mar-16 01:44:58.619653
2026-Mar-16 01:44:58.619653
#8 [worker internal] load .dockerignore
2026-Mar-16 01:44:58.619653
#8 transferring context: 341B done
2026-Mar-16 01:44:58.619653
#8 DONE 0.0s
2026-Mar-16 01:44:58.619653
2026-Mar-16 01:44:58.619653
#9 [worker runtime 1/7] FROM docker.io/library/debian:bookworm-slim@sha256:74d56e3931e0d5a1dd51f8c8a2466d21de84a271cd3b5a733b803aa91abf4421
2026-Mar-16 01:44:58.619653
#9 resolve docker.io/library/debian:bookworm-slim@sha256:74d56e3931e0d5a1dd51f8c8a2466d21de84a271cd3b5a733b803aa91abf4421 0.0s done
2026-Mar-16 01:44:58.619653
#9 DONE 0.0s
2026-Mar-16 01:44:58.619653
2026-Mar-16 01:44:58.619653
#10 [api internal] load build context
2026-Mar-16 01:44:58.619653
#10 transferring context: 897.02kB 0.0s done
2026-Mar-16 01:44:58.619653
#10 DONE 0.1s
2026-Mar-16 01:44:58.619653
2026-Mar-16 01:44:58.619653
#11 [worker builder 1/8] FROM docker.io/library/node:22-alpine@sha256:8094c002d08262dba12645a3b4a15cd6cd627d30bc782f53229a2ec13ee22a00
2026-Mar-16 01:44:58.619653
#11 resolve docker.io/library/node:22-alpine@sha256:8094c002d08262dba12645a3b4a15cd6cd627d30bc782f53229a2ec13ee22a00 0.0s done
2026-Mar-16 01:44:58.619653
#11 DONE 0.1s
2026-Mar-16 01:44:58.619653
2026-Mar-16 01:44:58.619653
#12 [frontend internal] load build context
2026-Mar-16 01:44:58.621820
#12 transferring context: 2.03MB 0.0s done
2026-Mar-16 01:44:58.621820
#12 DONE 0.1s
2026-Mar-16 01:44:58.621820
2026-Mar-16 01:44:58.621820
#13 [api runtime 2/7] WORKDIR /app
2026-Mar-16 01:44:58.621820
#13 DONE 0.0s
2026-Mar-16 01:44:58.621820
2026-Mar-16 01:44:58.621820
#14 [worker runtime 3/7] RUN apt-get update && apt-get install -y     ca-certificates     curl     libssl3     && rm -rf /var/lib/apt/lists/*
2026-Mar-16 01:44:58.780574
#14 ...
2026-Mar-16 01:44:58.780574
2026-Mar-16 01:44:58.780574
#15 [worker builder 2/8] WORKDIR /app
2026-Mar-16 01:44:58.780574
#15 DONE 0.0s
2026-Mar-16 01:44:58.780574
2026-Mar-16 01:44:58.780574
#16 [frontend builder 3/8] COPY frontend/ ./
2026-Mar-16 01:44:58.780574
#16 DONE 0.1s
2026-Mar-16 01:44:58.780574
2026-Mar-16 01:44:58.780574
#17 [frontend builder 4/8] COPY config/ /config/
2026-Mar-16 01:44:58.780574
#17 DONE 0.0s
2026-Mar-16 01:44:58.886662
#14 [worker runtime 3/7] RUN apt-get update && apt-get install -y     ca-certificates     curl     libssl3     && rm -rf /var/lib/apt/lists/*
2026-Mar-16 01:44:58.886662
#14 0.274 Get:1 http://deb.debian.org/debian bookworm InRelease [151 kB]
2026-Mar-16 01:44:59.229898
#14 0.398 Get:2 http://deb.debian.org/debian bookworm-updates InRelease [55.4 kB]
2026-Mar-16 01:44:59.229898
#14 0.435 Get:3 http://deb.debian.org/debian-security bookworm-security InRelease [48.0 kB]
2026-Mar-16 01:44:59.229898
#14 0.470 Get:4 http://deb.debian.org/debian bookworm/main amd64 Packages [8792 kB]
2026-Mar-16 01:44:59.530230
#14 0.810 Get:5 http://deb.debian.org/debian bookworm-updates/main amd64 Packages [6924 B]
2026-Mar-16 01:44:59.530230
#14 0.810 Get:6 http://deb.debian.org/debian-security bookworm-security/main amd64 Packages [294 kB]
2026-Mar-16 01:45:00.116089
#14 1.504 Fetched 9348 kB in 1s (7202 kB/s)
2026-Mar-16 01:45:00.116089
#14 1.504 Reading package lists...
2026-Mar-16 01:45:00.430111
#14 ...
2026-Mar-16 01:45:00.430111
2026-Mar-16 01:45:00.430111
#18 [worker js-builder  3/11] RUN npm install -g pnpm
2026-Mar-16 01:45:00.430111
#18 1.553
2026-Mar-16 01:45:00.430111
#18 1.553 added 1 package in 1s
2026-Mar-16 01:45:00.430111
#18 1.553
2026-Mar-16 01:45:00.430111
#18 1.553 1 package is looking for funding
2026-Mar-16 01:45:00.430111
#18 1.553   run `npm fund` for details
2026-Mar-16 01:45:00.430111
#18 1.554 npm notice
2026-Mar-16 01:45:00.430111
#18 1.554 npm notice New major version of npm available! 10.9.4 -> 11.11.1
2026-Mar-16 01:45:00.430111
#18 1.554 npm notice Changelog: https://github.com/npm/cli/releases/tag/v11.11.1
2026-Mar-16 01:45:00.430111
#18 1.554 npm notice To update run: npm install -g npm@11.11.1
2026-Mar-16 01:45:00.430111
#18 1.554 npm notice
2026-Mar-16 01:45:00.430111
#18 DONE 1.7s
2026-Mar-16 01:45:00.430111
2026-Mar-16 01:45:00.430111
#19 [api js-builder  4/11] COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Mar-16 01:45:00.430111
#19 DONE 0.0s
2026-Mar-16 01:45:00.430111
2026-Mar-16 01:45:00.430111
#20 [worker js-builder  5/11] COPY packages/api-client/package.json ./packages/api-client/
2026-Mar-16 01:45:00.430111
#20 DONE 0.0s
2026-Mar-16 01:45:00.430111
2026-Mar-16 01:45:00.430111
#21 [api js-builder  6/11] COPY apps/injector/package.json ./apps/injector/
2026-Mar-16 01:45:00.430111
#21 DONE 0.0s
2026-Mar-16 01:45:00.563996
#22 [api js-builder  7/12] COPY apps/injector/ ./apps/injector/
2026-Mar-16 01:45:00.563996
#22 DONE 0.1s
2026-Mar-16 01:45:00.563996
2026-Mar-16 01:45:00.563996
#23 [worker js-builder  7/11] COPY packages/ ./packages/
2026-Mar-16 01:45:00.563996
#23 DONE 0.1s
2026-Mar-16 01:45:00.563996
2026-Mar-16 01:45:00.563996
#24 [worker builder  1/10] FROM docker.io/library/rust:1.91-bookworm@sha256:c1e5f19e773b7878c3f7a805dd00a495e747acbdc76fb2337a4ebf0418896b33
2026-Mar-16 01:45:00.563996
#24 resolve docker.io/library/rust:1.91-bookworm@sha256:c1e5f19e773b7878c3f7a805dd00a495e747acbdc76fb2337a4ebf0418896b33 0.0s done
2026-Mar-16 01:45:00.563996
#24 sha256:627be014d1ff781d1abd9cc5d97cd162f7a7b72eedf59e7d3adf629fb29a64db 59.77MB / 213.34MB 1.8s
2026-Mar-16 01:45:00.563996
#24 sha256:e8d2a98f6bdfdbb1ba3c937c5e47cfa2cd11e74487543d277ca84f21f12ba393 9.44MB / 211.46MB 1.8s
2026-Mar-16 01:45:00.563996
#24 sha256:c237534654fe7a5c118fcee78652af952e57a4a07cc322c0ae3c367839bb0ccc 1.05MB / 64.40MB 1.8s
2026-Mar-16 01:45:00.563996
#24 sha256:6ae8659f7a8d357662281a0f87eb293725bb75ffa6c7356c38567f557d8a1f11 2.10MB / 24.03MB 1.8s
2026-Mar-16 01:45:00.730193
#24 ...
2026-Mar-16 01:45:00.730193
2026-Mar-16 01:45:00.730193
#25 [api js-builder  8/12] COPY packages/ ./packages/
2026-Mar-16 01:45:00.730193
#25 DONE 0.1s
2026-Mar-16 01:45:00.730193
2026-Mar-16 01:45:00.730193
#26 [worker js-builder  8/11] COPY apps/injector/ ./apps/injector/
2026-Mar-16 01:45:00.730193
#26 DONE 0.1s
2026-Mar-16 01:45:00.730193
2026-Mar-16 01:45:00.730193
#27 [worker js-builder  9/11] COPY extractors/ ./extractors/
2026-Mar-16 01:45:00.730193
#27 DONE 0.0s
2026-Mar-16 01:45:00.730193
2026-Mar-16 01:45:00.730193
#24 [worker builder  1/10] FROM docker.io/library/rust:1.91-bookworm@sha256:c1e5f19e773b7878c3f7a805dd00a495e747acbdc76fb2337a4ebf0418896b33
2026-Mar-16 01:45:00.730193
#24 sha256:627be014d1ff781d1abd9cc5d97cd162f7a7b72eedf59e7d3adf629fb29a64db 72.35MB / 213.34MB 2.1s
2026-Mar-16 01:45:00.880373
#24 sha256:6ae8659f7a8d357662281a0f87eb293725bb75ffa6c7356c38567f557d8a1f11 4.19MB / 24.03MB 2.1s
2026-Mar-16 01:45:01.180446
#24 sha256:627be014d1ff781d1abd9cc5d97cd162f7a7b72eedf59e7d3adf629fb29a64db 89.13MB / 213.34MB 2.6s
2026-Mar-16 01:45:01.180446
#24 sha256:e8d2a98f6bdfdbb1ba3c937c5e47cfa2cd11e74487543d277ca84f21f12ba393 20.97MB / 211.46MB 2.4s
2026-Mar-16 01:45:01.180446
#24 sha256:c237534654fe7a5c118fcee78652af952e57a4a07cc322c0ae3c367839bb0ccc 5.24MB / 64.40MB 2.4s
2026-Mar-16 01:45:01.308180
#24 sha256:6ae8659f7a8d357662281a0f87eb293725bb75ffa6c7356c38567f557d8a1f11 7.34MB / 24.03MB 2.6s
2026-Mar-16 01:45:01.424167
#24 sha256:6ae8659f7a8d357662281a0f87eb293725bb75ffa6c7356c38567f557d8a1f11 9.44MB / 24.03MB 2.7s
2026-Mar-16 01:45:01.631461
#24 sha256:c237534654fe7a5c118fcee78652af952e57a4a07cc322c0ae3c367839bb0ccc 12.58MB / 64.40MB 2.9s
2026-Mar-16 01:45:01.929603
#24 sha256:627be014d1ff781d1abd9cc5d97cd162f7a7b72eedf59e7d3adf629fb29a64db 116.39MB / 213.34MB 3.3s
2026-Mar-16 01:45:02.054958
#24 sha256:e8d2a98f6bdfdbb1ba3c937c5e47cfa2cd11e74487543d277ca84f21f12ba393 33.55MB / 211.46MB 3.3s
2026-Mar-16 01:45:02.054958
#24 sha256:c237534654fe7a5c118fcee78652af952e57a4a07cc322c0ae3c367839bb0ccc 20.97MB / 64.40MB 3.3s
2026-Mar-16 01:45:02.054958
#24 sha256:6ae8659f7a8d357662281a0f87eb293725bb75ffa6c7356c38567f557d8a1f11 14.68MB / 24.03MB 3.3s
2026-Mar-16 01:45:02.164217
#24 sha256:6ae8659f7a8d357662281a0f87eb293725bb75ffa6c7356c38567f557d8a1f11 16.78MB / 24.03MB 3.5s
2026-Mar-16 01:45:02.347529
#24 sha256:c237534654fe7a5c118fcee78652af952e57a4a07cc322c0ae3c367839bb0ccc 25.17MB / 64.40MB 3.6s
2026-Mar-16 01:45:02.529843
#24 sha256:627be014d1ff781d1abd9cc5d97cd162f7a7b72eedf59e7d3adf629fb29a64db 132.12MB / 213.34MB 3.9s
2026-Mar-16 01:45:02.529843
#24 sha256:c237534654fe7a5c118fcee78652af952e57a4a07cc322c0ae3c367839bb0ccc 29.36MB / 64.40MB 3.8s
2026-Mar-16 01:45:02.772613
#24 sha256:c237534654fe7a5c118fcee78652af952e57a4a07cc322c0ae3c367839bb0ccc 35.65MB / 64.40MB 4.1s
2026-Mar-16 01:45:02.772613
#24 sha256:6ae8659f7a8d357662281a0f87eb293725bb75ffa6c7356c38567f557d8a1f11 19.92MB / 24.03MB 4.1s
2026-Mar-16 01:45:02.876063
#24 sha256:627be014d1ff781d1abd9cc5d97cd162f7a7b72eedf59e7d3adf629fb29a64db 143.65MB / 213.34MB 4.2s
2026-Mar-16 01:45:02.876063
#24 sha256:c237534654fe7a5c118fcee78652af952e57a4a07cc322c0ae3c367839bb0ccc 39.85MB / 64.40MB 4.2s
2026-Mar-16 01:45:03.093267
#24 sha256:6ae8659f7a8d357662281a0f87eb293725bb75ffa6c7356c38567f557d8a1f11 24.03MB / 24.03MB 4.4s done
2026-Mar-16 01:45:03.199390
#24 sha256:c237534654fe7a5c118fcee78652af952e57a4a07cc322c0ae3c367839bb0ccc 48.23MB / 64.40MB 4.5s
2026-Mar-16 01:45:03.337772
#24 sha256:627be014d1ff781d1abd9cc5d97cd162f7a7b72eedf59e7d3adf629fb29a64db 158.33MB / 213.34MB 4.7s
2026-Mar-16 01:45:03.337772
#24 sha256:e8d2a98f6bdfdbb1ba3c937c5e47cfa2cd11e74487543d277ca84f21f12ba393 46.14MB / 211.46MB 4.7s
2026-Mar-16 01:45:03.337772
#24 sha256:c8443a297fa42e27cb10653777dd5a53f82a65fbc8b2d33f82b8722199f941d3 0B / 48.48MB 0.2s
2026-Mar-16 01:45:03.450253
#24 sha256:c237534654fe7a5c118fcee78652af952e57a4a07cc322c0ae3c367839bb0ccc 54.53MB / 64.40MB 4.8s
2026-Mar-16 01:45:03.579556
#24 sha256:627be014d1ff781d1abd9cc5d97cd162f7a7b72eedf59e7d3adf629fb29a64db 169.87MB / 213.34MB 5.0s
2026-Mar-16 01:45:03.731216
#24 ...
2026-Mar-16 01:45:03.731216
2026-Mar-16 01:45:03.731216
#28 [api js-builder  9/12] RUN pnpm install --frozen-lockfile
2026-Mar-16 01:45:03.731216
#28 0.649 Scope: all 3 workspace projects
2026-Mar-16 01:45:03.731216
#28 0.738 Lockfile is up to date, resolution step is skipped
2026-Mar-16 01:45:03.731216
#28 0.777 Progress: resolved 1, reused 0, downloaded 0, added 0
2026-Mar-16 01:45:03.731216
#28 0.853 Packages: +105
2026-Mar-16 01:45:03.731216
#28 0.853 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
2026-Mar-16 01:45:03.731216
#28 1.778 Progress: resolved 105, reused 0, downloaded 78, added 78
2026-Mar-16 01:45:03.731216
#28 2.583 Progress: resolved 105, reused 0, downloaded 105, added 105, done
2026-Mar-16 01:45:03.731216
#28 2.642 .../esbuild@0.25.12/node_modules/esbuild postinstall$ node install.js
2026-Mar-16 01:45:03.731216
#28 2.706 .../esbuild@0.25.12/node_modules/esbuild postinstall: Done
2026-Mar-16 01:45:03.731216
#28 2.771
2026-Mar-16 01:45:03.731216
#28 2.884 Done in 2.7s using pnpm v10.32.1
2026-Mar-16 01:45:03.731216
#28 DONE 3.0s
2026-Mar-16 01:45:03.731216
2026-Mar-16 01:45:03.731216
#29 [worker js-builder 10/11] RUN pnpm install --frozen-lockfile
2026-Mar-16 01:45:03.731216
#29 0.634 Scope: all 3 workspace projects
2026-Mar-16 01:45:03.731216
#29 0.720 Lockfile is up to date, resolution step is skipped
2026-Mar-16 01:45:03.731216
#29 0.755 Progress: resolved 1, reused 0, downloaded 0, added 0
2026-Mar-16 01:45:03.731216
#29 0.856 Packages: +105
2026-Mar-16 01:45:03.731216
#29 0.856 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
2026-Mar-16 01:45:03.731216
#29 1.756 Progress: resolved 105, reused 0, downloaded 56, added 55
2026-Mar-16 01:45:03.731216
#29 2.543 Progress: resolved 105, reused 0, downloaded 105, added 105, done
2026-Mar-16 01:45:03.731216
#29 2.607 .../esbuild@0.25.12/node_modules/esbuild postinstall$ node install.js
2026-Mar-16 01:45:03.731216
#29 2.676 .../esbuild@0.25.12/node_modules/esbuild postinstall: Done
2026-Mar-16 01:45:03.731216
#29 2.745
2026-Mar-16 01:45:03.731216
#29 2.858 Done in 2.6s using pnpm v10.32.1
2026-Mar-16 01:45:03.731216
#29 DONE 3.0s
2026-Mar-16 01:45:03.731216
2026-Mar-16 01:45:03.731216
#24 [worker builder  1/10] FROM docker.io/library/rust:1.91-bookworm@sha256:c1e5f19e773b7878c3f7a805dd00a495e747acbdc76fb2337a4ebf0418896b33
2026-Mar-16 01:45:03.880394
#24 sha256:627be014d1ff781d1abd9cc5d97cd162f7a7b72eedf59e7d3adf629fb29a64db 183.50MB / 213.34MB 5.3s
2026-Mar-16 01:45:03.880394
#24 sha256:c237534654fe7a5c118fcee78652af952e57a4a07cc322c0ae3c367839bb0ccc 58.72MB / 64.40MB 5.1s
2026-Mar-16 01:45:04.180709
#24 sha256:627be014d1ff781d1abd9cc5d97cd162f7a7b72eedf59e7d3adf629fb29a64db 198.18MB / 213.34MB 5.6s
2026-Mar-16 01:45:04.330302
#24 sha256:c237534654fe7a5c118fcee78652af952e57a4a07cc322c0ae3c367839bb0ccc 64.40MB / 64.40MB 5.6s done
2026-Mar-16 01:45:04.463559
#24 sha256:627be014d1ff781d1abd9cc5d97cd162f7a7b72eedf59e7d3adf629fb29a64db 213.34MB / 213.34MB 5.8s done
2026-Mar-16 01:45:04.463559
#24 sha256:c8443a297fa42e27cb10653777dd5a53f82a65fbc8b2d33f82b8722199f941d3 3.15MB / 48.48MB 1.4s
2026-Mar-16 01:45:04.585253
#24 sha256:627be014d1ff781d1abd9cc5d97cd162f7a7b72eedf59e7d3adf629fb29a64db 213.34MB / 213.34MB 5.8s done
2026-Mar-16 01:45:04.585253
#24 sha256:e8d2a98f6bdfdbb1ba3c937c5e47cfa2cd11e74487543d277ca84f21f12ba393 57.67MB / 211.46MB 5.9s
2026-Mar-16 01:45:04.701727
#24 sha256:c8443a297fa42e27cb10653777dd5a53f82a65fbc8b2d33f82b8722199f941d3 6.29MB / 48.48MB 1.7s
2026-Mar-16 01:45:05.081489
#24 ...
2026-Mar-16 01:45:05.081489
2026-Mar-16 01:45:05.081489
#14 [worker runtime 3/7] RUN apt-get update && apt-get install -y     ca-certificates     curl     libssl3     && rm -rf /var/lib/apt/lists/*
2026-Mar-16 01:45:05.081489
#14 1.504 Reading package lists...
2026-Mar-16 01:45:05.081489
#14 1.974 Reading package lists...
2026-Mar-16 01:45:05.081489
#14 2.511 Building dependency tree...
2026-Mar-16 01:45:05.081489
#14 2.653 Reading state information...
2026-Mar-16 01:45:05.081489
#14 2.888 The following additional packages will be installed:
2026-Mar-16 01:45:05.081489
#14 2.889   krb5-locales libbrotli1 libcurl4 libgssapi-krb5-2 libk5crypto3 libkeyutils1
2026-Mar-16 01:45:05.081489
#14 2.889   libkrb5-3 libkrb5support0 libldap-2.5-0 libldap-common libnghttp2-14 libpsl5
2026-Mar-16 01:45:05.081489
#14 2.889   librtmp1 libsasl2-2 libsasl2-modules libsasl2-modules-db libssh2-1 openssl
2026-Mar-16 01:45:05.081489
#14 2.889   publicsuffix
2026-Mar-16 01:45:05.081489
#14 2.891 Suggested packages:
2026-Mar-16 01:45:05.081489
#14 2.891   krb5-doc krb5-user libsasl2-modules-gssapi-mit
2026-Mar-16 01:45:05.081489
#14 2.891   | libsasl2-modules-gssapi-heimdal libsasl2-modules-ldap libsasl2-modules-otp
2026-Mar-16 01:45:05.081489
#14 2.891   libsasl2-modules-sql
2026-Mar-16 01:45:05.081489
#14 3.057 The following NEW packages will be installed:
2026-Mar-16 01:45:05.081489
#14 3.057   ca-certificates curl krb5-locales libbrotli1 libcurl4 libgssapi-krb5-2
2026-Mar-16 01:45:05.081489
#14 3.057   libk5crypto3 libkeyutils1 libkrb5-3 libkrb5support0 libldap-2.5-0
2026-Mar-16 01:45:05.081489
#14 3.058   libldap-common libnghttp2-14 libpsl5 librtmp1 libsasl2-2 libsasl2-modules
2026-Mar-16 01:45:05.081489
#14 3.058   libsasl2-modules-db libssh2-1 libssl3 openssl publicsuffix
2026-Mar-16 01:45:05.081489
#14 3.153 0 upgraded, 22 newly installed, 0 to remove and 0 not upgraded.
2026-Mar-16 01:45:05.081489
#14 3.153 Need to get 6111 kB of archives.
2026-Mar-16 01:45:05.081489
#14 3.153 After this operation, 15.7 MB of additional disk space will be used.
2026-Mar-16 01:45:05.081489
#14 3.153 Get:1 http://deb.debian.org/debian-security bookworm-security/main amd64 libssl3 amd64 3.0.18-1~deb12u2 [2030 kB]
2026-Mar-16 01:45:05.081489
#14 3.442 Get:2 http://deb.debian.org/debian-security bookworm-security/main amd64 openssl amd64 3.0.18-1~deb12u2 [1433 kB]
2026-Mar-16 01:45:05.081489
#14 3.516 Get:3 http://deb.debian.org/debian bookworm/main amd64 ca-certificates all 20230311+deb12u1 [155 kB]
2026-Mar-16 01:45:05.081489
#14 3.518 Get:4 http://deb.debian.org/debian bookworm/main amd64 krb5-locales all 1.20.1-2+deb12u4 [63.4 kB]
2026-Mar-16 01:45:05.081489
#14 3.551 Get:5 http://deb.debian.org/debian bookworm/main amd64 libbrotli1 amd64 1.0.9-2+b6 [275 kB]
2026-Mar-16 01:45:05.085728
#14 3.555 Get:6 http://deb.debian.org/debian bookworm/main amd64 libkrb5support0 amd64 1.20.1-2+deb12u4 [33.2 kB]
2026-Mar-16 01:45:05.085728
#14 3.555 Get:7 http://deb.debian.org/debian bookworm/main amd64 libk5crypto3 amd64 1.20.1-2+deb12u4 [79.8 kB]
2026-Mar-16 01:45:05.085728
#14 3.556 Get:8 http://deb.debian.org/debian bookworm/main amd64 libkeyutils1 amd64 1.6.3-2 [8808 B]
2026-Mar-16 01:45:05.085728
#14 3.557 Get:9 http://deb.debian.org/debian bookworm/main amd64 libkrb5-3 amd64 1.20.1-2+deb12u4 [334 kB]
2026-Mar-16 01:45:05.085728
#14 3.564 Get:10 http://deb.debian.org/debian bookworm/main amd64 libgssapi-krb5-2 amd64 1.20.1-2+deb12u4 [135 kB]
2026-Mar-16 01:45:05.085728
#14 3.566 Get:11 http://deb.debian.org/debian bookworm/main amd64 libsasl2-modules-db amd64 2.1.28+dfsg-10 [20.3 kB]
2026-Mar-16 01:45:05.085728
#14 3.567 Get:12 http://deb.debian.org/debian bookworm/main amd64 libsasl2-2 amd64 2.1.28+dfsg-10 [59.7 kB]
2026-Mar-16 01:45:05.085728
#14 3.568 Get:13 http://deb.debian.org/debian bookworm/main amd64 libldap-2.5-0 amd64 2.5.13+dfsg-5 [183 kB]
2026-Mar-16 01:45:05.085728
#14 3.585 Get:14 http://deb.debian.org/debian bookworm/main amd64 libnghttp2-14 amd64 1.52.0-1+deb12u2 [73.0 kB]
2026-Mar-16 01:45:05.085728
#14 3.618 Get:15 http://deb.debian.org/debian bookworm/main amd64 libpsl5 amd64 0.21.2-1 [58.7 kB]
2026-Mar-16 01:45:05.085728
#14 3.619 Get:16 http://deb.debian.org/debian bookworm/main amd64 librtmp1 amd64 2.4+20151223.gitfa8646d.1-2+b2 [60.8 kB]
2026-Mar-16 01:45:05.085728
#14 3.621 Get:17 http://deb.debian.org/debian bookworm/main amd64 libssh2-1 amd64 1.10.0-3+b1 [179 kB]
2026-Mar-16 01:45:05.085728
#14 3.626 Get:18 http://deb.debian.org/debian bookworm/main amd64 libcurl4 amd64 7.88.1-10+deb12u14 [392 kB]
2026-Mar-16 01:45:05.085728
#14 3.634 Get:19 http://deb.debian.org/debian bookworm/main amd64 curl amd64 7.88.1-10+deb12u14 [316 kB]
2026-Mar-16 01:45:05.085728
#14 3.641 Get:20 http://deb.debian.org/debian bookworm/main amd64 libldap-common all 2.5.13+dfsg-5 [29.3 kB]
2026-Mar-16 01:45:05.085728
#14 3.641 Get:21 http://deb.debian.org/debian bookworm/main amd64 libsasl2-modules amd64 2.1.28+dfsg-10 [66.6 kB]
2026-Mar-16 01:45:05.085728
#14 3.642 Get:22 http://deb.debian.org/debian bookworm/main amd64 publicsuffix all 20230209.2326-1 [126 kB]
2026-Mar-16 01:45:05.085728
#14 3.765 debconf: delaying package configuration, since apt-utils is not installed
2026-Mar-16 01:45:05.085728
#14 3.800 Fetched 6111 kB in 1s (10.7 MB/s)
2026-Mar-16 01:45:05.085728
#14 3.822 Selecting previously unselected package libssl3:amd64.
2026-Mar-16 01:45:05.085728
#14 3.822 (Reading database ... 
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
2026-Mar-16 01:45:05.085728
#14 3.828 Preparing to unpack .../00-libssl3_3.0.18-1~deb12u2_amd64.deb ...
2026-Mar-16 01:45:05.085728
#14 3.831 Unpacking libssl3:amd64 (3.0.18-1~deb12u2) ...
2026-Mar-16 01:45:05.085728
#14 3.959 Selecting previously unselected package openssl.
2026-Mar-16 01:45:05.085728
#14 3.960 Preparing to unpack .../01-openssl_3.0.18-1~deb12u2_amd64.deb ...
2026-Mar-16 01:45:05.085728
#14 3.961 Unpacking openssl (3.0.18-1~deb12u2) ...
2026-Mar-16 01:45:05.085728
#14 4.057 Selecting previously unselected package ca-certificates.
2026-Mar-16 01:45:05.085728
#14 4.058 Preparing to unpack .../02-ca-certificates_20230311+deb12u1_all.deb ...
2026-Mar-16 01:45:05.085728
#14 4.059 Unpacking ca-certificates (20230311+deb12u1) ...
2026-Mar-16 01:45:05.085728
#14 4.101 Selecting previously unselected package krb5-locales.
2026-Mar-16 01:45:05.085728
#14 4.103 Preparing to unpack .../03-krb5-locales_1.20.1-2+deb12u4_all.deb ...
2026-Mar-16 01:45:05.085728
#14 4.104 Unpacking krb5-locales (1.20.1-2+deb12u4) ...
2026-Mar-16 01:45:05.085728
#14 4.124 Selecting previously unselected package libbrotli1:amd64.
2026-Mar-16 01:45:05.085728
#14 4.125 Preparing to unpack .../04-libbrotli1_1.0.9-2+b6_amd64.deb ...
2026-Mar-16 01:45:05.085728
#14 4.126 Unpacking libbrotli1:amd64 (1.0.9-2+b6) ...
2026-Mar-16 01:45:05.085728
#14 4.160 Selecting previously unselected package libkrb5support0:amd64.
2026-Mar-16 01:45:05.085728
#14 4.161 Preparing to unpack .../05-libkrb5support0_1.20.1-2+deb12u4_amd64.deb ...
2026-Mar-16 01:45:05.085728
#14 4.162 Unpacking libkrb5support0:amd64 (1.20.1-2+deb12u4) ...
2026-Mar-16 01:45:05.085728
#14 4.180 Selecting previously unselected package libk5crypto3:amd64.
2026-Mar-16 01:45:05.085728
#14 4.181 Preparing to unpack .../06-libk5crypto3_1.20.1-2+deb12u4_amd64.deb ...
2026-Mar-16 01:45:05.085728
#14 4.183 Unpacking libk5crypto3:amd64 (1.20.1-2+deb12u4) ...
2026-Mar-16 01:45:05.085728
#14 4.205 Selecting previously unselected package libkeyutils1:amd64.
2026-Mar-16 01:45:05.085728
#14 4.206 Preparing to unpack .../07-libkeyutils1_1.6.3-2_amd64.deb ...
2026-Mar-16 01:45:05.085728
#14 4.207 Unpacking libkeyutils1:amd64 (1.6.3-2) ...
2026-Mar-16 01:45:05.085728
#14 4.222 Selecting previously unselected package libkrb5-3:amd64.
2026-Mar-16 01:45:05.085728
#14 4.224 Preparing to unpack .../08-libkrb5-3_1.20.1-2+deb12u4_amd64.deb ...
2026-Mar-16 01:45:05.085728
#14 4.225 Unpacking libkrb5-3:amd64 (1.20.1-2+deb12u4) ...
2026-Mar-16 01:45:05.085728
#14 4.263 Selecting previously unselected package libgssapi-krb5-2:amd64.
2026-Mar-16 01:45:05.085728
#14 4.265 Preparing to unpack .../09-libgssapi-krb5-2_1.20.1-2+deb12u4_amd64.deb ...
2026-Mar-16 01:45:05.085728
#14 4.266 Unpacking libgssapi-krb5-2:amd64 (1.20.1-2+deb12u4) ...
2026-Mar-16 01:45:05.085728
#14 4.287 Selecting previously unselected package libsasl2-modules-db:amd64.
2026-Mar-16 01:45:05.085728
#14 4.288 Preparing to unpack .../10-libsasl2-modules-db_2.1.28+dfsg-10_amd64.deb ...
2026-Mar-16 01:45:05.085728
#14 4.289 Unpacking libsasl2-modules-db:amd64 (2.1.28+dfsg-10) ...
2026-Mar-16 01:45:05.085728
#14 4.305 Selecting previously unselected package libsasl2-2:amd64.
2026-Mar-16 01:45:05.085728
#14 4.306 Preparing to unpack .../11-libsasl2-2_2.1.28+dfsg-10_amd64.deb ...
2026-Mar-16 01:45:05.085728
#14 4.308 Unpacking libsasl2-2:amd64 (2.1.28+dfsg-10) ...
2026-Mar-16 01:45:05.085728
#14 4.327 Selecting previously unselected package libldap-2.5-0:amd64.
2026-Mar-16 01:45:05.085728
#14 4.328 Preparing to unpack .../12-libldap-2.5-0_2.5.13+dfsg-5_amd64.deb ...
2026-Mar-16 01:45:05.085728
#14 4.330 Unpacking libldap-2.5-0:amd64 (2.5.13+dfsg-5) ...
2026-Mar-16 01:45:05.085728
#14 4.357 Selecting previously unselected package libnghttp2-14:amd64.
2026-Mar-16 01:45:05.085728
#14 4.358 Preparing to unpack .../13-libnghttp2-14_1.52.0-1+deb12u2_amd64.deb ...
2026-Mar-16 01:45:05.085728
#14 4.359 Unpacking libnghttp2-14:amd64 (1.52.0-1+deb12u2) ...
2026-Mar-16 01:45:05.085728
#14 4.379 Selecting previously unselected package libpsl5:amd64.
2026-Mar-16 01:45:05.085728
#14 4.380 Preparing to unpack .../14-libpsl5_0.21.2-1_amd64.deb ...
2026-Mar-16 01:45:05.085728
#14 4.381 Unpacking libpsl5:amd64 (0.21.2-1) ...
2026-Mar-16 01:45:05.085728
#14 4.398 Selecting previously unselected package librtmp1:amd64.
2026-Mar-16 01:45:05.085728
#14 4.400 Preparing to unpack .../15-librtmp1_2.4+20151223.gitfa8646d.1-2+b2_amd64.deb ...
2026-Mar-16 01:45:05.085728
#14 4.401 Unpacking librtmp1:amd64 (2.4+20151223.gitfa8646d.1-2+b2) ...
2026-Mar-16 01:45:05.085728
#14 4.419 Selecting previously unselected package libssh2-1:amd64.
2026-Mar-16 01:45:05.085728
#14 4.420 Preparing to unpack .../16-libssh2-1_1.10.0-3+b1_amd64.deb ...
2026-Mar-16 01:45:05.085728
#14 4.421 Unpacking libssh2-1:amd64 (1.10.0-3+b1) ...
2026-Mar-16 01:45:05.085728
#14 4.449 Selecting previously unselected package libcurl4:amd64.
2026-Mar-16 01:45:05.085728
#14 4.449 Preparing to unpack .../17-libcurl4_7.88.1-10+deb12u14_amd64.deb ...
2026-Mar-16 01:45:05.085728
#14 4.450 Unpacking libcurl4:amd64 (7.88.1-10+deb12u14) ...
2026-Mar-16 01:45:05.085728
#14 4.481 Selecting previously unselected package curl.
2026-Mar-16 01:45:05.085728
#14 4.482 Preparing to unpack .../18-curl_7.88.1-10+deb12u14_amd64.deb ...
2026-Mar-16 01:45:05.085728
#14 4.483 Unpacking curl (7.88.1-10+deb12u14) ...
2026-Mar-16 01:45:05.085728
#14 4.511 Selecting previously unselected package libldap-common.
2026-Mar-16 01:45:05.085728
#14 4.512 Preparing to unpack .../19-libldap-common_2.5.13+dfsg-5_all.deb ...
2026-Mar-16 01:45:05.085728
#14 4.513 Unpacking libldap-common (2.5.13+dfsg-5) ...
2026-Mar-16 01:45:05.085728
#14 4.532 Selecting previously unselected package libsasl2-modules:amd64.
2026-Mar-16 01:45:05.085728
#14 4.534 Preparing to unpack .../20-libsasl2-modules_2.1.28+dfsg-10_amd64.deb ...
2026-Mar-16 01:45:05.085728
#14 4.540 Unpacking libsasl2-modules:amd64 (2.1.28+dfsg-10) ...
2026-Mar-16 01:45:05.085728
#14 4.561 Selecting previously unselected package publicsuffix.
2026-Mar-16 01:45:05.085728
#14 4.562 Preparing to unpack .../21-publicsuffix_20230209.2326-1_all.deb ...
2026-Mar-16 01:45:05.085728
#14 4.563 Unpacking publicsuffix (20230209.2326-1) ...
2026-Mar-16 01:45:05.085728
#14 4.591 Setting up libkeyutils1:amd64 (1.6.3-2) ...
2026-Mar-16 01:45:05.085728
#14 4.597 Setting up libpsl5:amd64 (0.21.2-1) ...
2026-Mar-16 01:45:05.085728
#14 4.600 Setting up libbrotli1:amd64 (1.0.9-2+b6) ...
2026-Mar-16 01:45:05.085728
#14 4.603 Setting up libssl3:amd64 (3.0.18-1~deb12u2) ...
2026-Mar-16 01:45:05.085728
#14 4.606 Setting up libnghttp2-14:amd64 (1.52.0-1+deb12u2) ...
2026-Mar-16 01:45:05.085728
#14 4.609 Setting up krb5-locales (1.20.1-2+deb12u4) ...
2026-Mar-16 01:45:05.085728
#14 4.612 Setting up libldap-common (2.5.13+dfsg-5) ...
2026-Mar-16 01:45:05.085728
#14 4.616 Setting up libkrb5support0:amd64 (1.20.1-2+deb12u4) ...
2026-Mar-16 01:45:05.085728
#14 4.619 Setting up libsasl2-modules-db:amd64 (2.1.28+dfsg-10) ...
2026-Mar-16 01:45:05.085728
#14 4.622 Setting up librtmp1:amd64 (2.4+20151223.gitfa8646d.1-2+b2) ...
2026-Mar-16 01:45:05.085728
#14 4.625 Setting up libk5crypto3:amd64 (1.20.1-2+deb12u4) ...
2026-Mar-16 01:45:05.085728
#14 4.628 Setting up libsasl2-2:amd64 (2.1.28+dfsg-10) ...
2026-Mar-16 01:45:05.085728
#14 4.631 Setting up libssh2-1:amd64 (1.10.0-3+b1) ...
2026-Mar-16 01:45:05.085728
#14 4.634 Setting up libkrb5-3:amd64 (1.20.1-2+deb12u4) ...
2026-Mar-16 01:45:05.085728
#14 4.637 Setting up openssl (3.0.18-1~deb12u2) ...
2026-Mar-16 01:45:05.085728
#14 4.642 Setting up publicsuffix (20230209.2326-1) ...
2026-Mar-16 01:45:05.085728
#14 4.653 Setting up libsasl2-modules:amd64 (2.1.28+dfsg-10) ...
2026-Mar-16 01:45:05.085728
#14 4.660 Setting up libldap-2.5-0:amd64 (2.5.13+dfsg-5) ...
2026-Mar-16 01:45:05.085728
#14 4.663 Setting up ca-certificates (20230311+deb12u1) ...
2026-Mar-16 01:45:05.085728
#14 4.730 debconf: unable to initialize frontend: Dialog
2026-Mar-16 01:45:05.085728
#14 4.730 debconf: (TERM is not set, so the dialog frontend is not usable.)
2026-Mar-16 01:45:05.085728
#14 4.730 debconf: falling back to frontend: Readline
2026-Mar-16 01:45:05.085728
#14 4.730 debconf: unable to initialize frontend: Readline
2026-Mar-16 01:45:05.085728
#14 4.730 debconf: (Can't locate Term/ReadLine.pm in @INC (you may need to install the Term::ReadLine module) (@INC contains: /etc/perl /usr/local/lib/x86_64-linux-gnu/perl/5.36.0 /usr/local/share/perl/5.36.0 /usr/lib/x86_64-linux-gnu/perl5/5.36 /usr/share/perl5 /usr/lib/x86_64-linux-gnu/perl-base /usr/lib/x86_64-linux-gnu/perl/5.36 /usr/share/perl/5.36 /usr/local/lib/site_perl) at /usr/share/perl5/Debconf/FrontEnd/Readline.pm line 7.)
2026-Mar-16 01:45:05.085728
#14 4.730 debconf: falling back to frontend: Teletype
2026-Mar-16 01:45:05.085728
#14 5.187 Updating certificates in /etc/ssl/certs...
2026-Mar-16 01:45:05.085728
#14 5.767 142 added, 0 removed; done.
2026-Mar-16 01:45:05.085728
#14 5.786 Setting up libgssapi-krb5-2:amd64 (1.20.1-2+deb12u4) ...
2026-Mar-16 01:45:05.085728
#14 5.791 Setting up libcurl4:amd64 (7.88.1-10+deb12u14) ...
2026-Mar-16 01:45:05.085728
#14 5.794 Setting up curl (7.88.1-10+deb12u14) ...
2026-Mar-16 01:45:05.085728
#14 5.797 Processing triggers for libc-bin (2.36-9+deb12u13) ...
2026-Mar-16 01:45:05.085728
#14 5.810 Processing triggers for ca-certificates (20230311+deb12u1) ...
2026-Mar-16 01:45:05.085728
#14 5.814 Updating certificates in /etc/ssl/certs...
2026-Mar-16 01:45:05.085728
#14 6.279 0 added, 0 removed; done.
2026-Mar-16 01:45:05.085728
#14 6.279 Running hooks in /etc/ca-certificates/update.d...
2026-Mar-16 01:45:05.085728
#14 6.280 done.
2026-Mar-16 01:45:05.085728
#14 DONE 6.5s
2026-Mar-16 01:45:05.085728
2026-Mar-16 01:45:05.085728
#24 [worker builder  1/10] FROM docker.io/library/rust:1.91-bookworm@sha256:c1e5f19e773b7878c3f7a805dd00a495e747acbdc76fb2337a4ebf0418896b33
2026-Mar-16 01:45:05.231766
#24 ...
2026-Mar-16 01:45:05.231766
2026-Mar-16 01:45:05.231766
#30 [api js-builder 10/12] RUN pnpm --filter @downloadtool/injector build
2026-Mar-16 01:45:05.231766
#30 0.651
2026-Mar-16 01:45:05.231766
#30 0.651 > @downloadtool/injector@0.0.1 build /app/apps/injector
2026-Mar-16 01:45:05.231766
#30 0.651 > vite build && vite build --config vite.userscript.config.ts
2026-Mar-16 01:45:05.231766
#30 0.651
2026-Mar-16 01:45:05.231766
#30 0.900 vite v6.4.1 building for production...
2026-Mar-16 01:45:05.231766
#30 0.945 transforming...
2026-Mar-16 01:45:05.231766
#30 0.991 ✓ 4 modules transformed.
2026-Mar-16 01:45:05.231766
#30 1.012 rendering chunks...
2026-Mar-16 01:45:05.231766
#30 1.014 computing gzip size...
2026-Mar-16 01:45:05.231766
#30 1.016 dist/bm.js  6.20 kB │ gzip: 2.34 kB
2026-Mar-16 01:45:05.231766
#30 1.016 ✓ built in 90ms
2026-Mar-16 01:45:05.231766
#30 1.283 vite v6.4.1 building for production...
2026-Mar-16 01:45:05.231766
#30 1.321 transforming...
2026-Mar-16 01:45:05.231766
#30 1.380 ✓ 4 modules transformed.
2026-Mar-16 01:45:05.231766
#30 1.399 rendering chunks...
2026-Mar-16 01:45:05.231766
#30 1.458 computing gzip size...
2026-Mar-16 01:45:05.231766
#30 1.460 dist/youtube-downloader.user.js  10.42 kB │ gzip: 3.20 kB
2026-Mar-16 01:45:05.231766
#30 1.460 ✓ built in 166ms
2026-Mar-16 01:45:05.231766
#30 DONE 1.5s
2026-Mar-16 01:45:05.231766
2026-Mar-16 01:45:05.231766
#31 [api js-builder 11/12] COPY extractors/ ./extractors/
2026-Mar-16 01:45:05.231766
#31 DONE 0.0s
2026-Mar-16 01:45:05.231766
2026-Mar-16 01:45:05.231766
#24 [worker builder  1/10] FROM docker.io/library/rust:1.91-bookworm@sha256:c1e5f19e773b7878c3f7a805dd00a495e747acbdc76fb2337a4ebf0418896b33
2026-Mar-16 01:45:05.231766
#24 sha256:c8443a297fa42e27cb10653777dd5a53f82a65fbc8b2d33f82b8722199f941d3 9.44MB / 48.48MB 2.1s
2026-Mar-16 01:45:05.531545
#24 sha256:e8d2a98f6bdfdbb1ba3c937c5e47cfa2cd11e74487543d277ca84f21f12ba393 69.21MB / 211.46MB 6.9s
2026-Mar-16 01:45:05.531545
#24 sha256:c8443a297fa42e27cb10653777dd5a53f82a65fbc8b2d33f82b8722199f941d3 12.58MB / 48.48MB 2.4s
2026-Mar-16 01:45:05.831589
#24 sha256:c8443a297fa42e27cb10653777dd5a53f82a65fbc8b2d33f82b8722199f941d3 16.78MB / 48.48MB 2.7s
2026-Mar-16 01:45:06.281506
#24 sha256:e8d2a98f6bdfdbb1ba3c937c5e47cfa2cd11e74487543d277ca84f21f12ba393 80.74MB / 211.46MB 7.7s
2026-Mar-16 01:45:06.281506
#24 sha256:c8443a297fa42e27cb10653777dd5a53f82a65fbc8b2d33f82b8722199f941d3 22.02MB / 48.48MB 3.2s
2026-Mar-16 01:45:06.431374
#24 sha256:c8443a297fa42e27cb10653777dd5a53f82a65fbc8b2d33f82b8722199f941d3 26.21MB / 48.48MB 3.3s
2026-Mar-16 01:45:06.581468
#24 ...
2026-Mar-16 01:45:06.581468
2026-Mar-16 01:45:06.581468
#32 [worker js-builder 11/11] RUN mkdir -p extractors/dist &&     npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js &&     npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-16 01:45:06.581468
#32 2.171
2026-Mar-16 01:45:06.581468
#32 2.171   extractors/dist/types.js  1.2kb
2026-Mar-16 01:45:06.581468
#32 2.171
2026-Mar-16 01:45:06.581468
#32 2.171 ⚡ Done in 3ms
2026-Mar-16 01:45:06.581468
#32 2.837
2026-Mar-16 01:45:06.581468
#32 2.837   extractors/dist/youtube.js  22.4kb
2026-Mar-16 01:45:06.581468
#32 2.837
2026-Mar-16 01:45:06.581468
#32 2.837 ⚡ Done in 3ms
2026-Mar-16 01:45:06.581468
#32 DONE 2.9s
2026-Mar-16 01:45:06.581468
2026-Mar-16 01:45:06.581468
#24 [worker builder  1/10] FROM docker.io/library/rust:1.91-bookworm@sha256:c1e5f19e773b7878c3f7a805dd00a495e747acbdc76fb2337a4ebf0418896b33
2026-Mar-16 01:45:06.581468
#24 sha256:c8443a297fa42e27cb10653777dd5a53f82a65fbc8b2d33f82b8722199f941d3 32.51MB / 48.48MB 3.5s
2026-Mar-16 01:45:06.720421
#24 sha256:c8443a297fa42e27cb10653777dd5a53f82a65fbc8b2d33f82b8722199f941d3 48.48MB / 48.48MB 3.7s done
2026-Mar-16 01:45:06.881551
#24 sha256:e8d2a98f6bdfdbb1ba3c937c5e47cfa2cd11e74487543d277ca84f21f12ba393 94.37MB / 211.46MB 8.3s
2026-Mar-16 01:45:06.881551
#24 sha256:c8443a297fa42e27cb10653777dd5a53f82a65fbc8b2d33f82b8722199f941d3 48.48MB / 48.48MB 3.7s done
2026-Mar-16 01:45:06.881551
#24 extracting sha256:c8443a297fa42e27cb10653777dd5a53f82a65fbc8b2d33f82b8722199f941d3
2026-Mar-16 01:45:07.181360
#24 sha256:e8d2a98f6bdfdbb1ba3c937c5e47cfa2cd11e74487543d277ca84f21f12ba393 105.91MB / 211.46MB 8.6s
2026-Mar-16 01:45:07.482277
#24 sha256:e8d2a98f6bdfdbb1ba3c937c5e47cfa2cd11e74487543d277ca84f21f12ba393 121.63MB / 211.46MB 8.9s
2026-Mar-16 01:45:07.631332
#24 sha256:e8d2a98f6bdfdbb1ba3c937c5e47cfa2cd11e74487543d277ca84f21f12ba393 133.17MB / 211.46MB 9.0s
2026-Mar-16 01:45:07.931478
#24 sha256:e8d2a98f6bdfdbb1ba3c937c5e47cfa2cd11e74487543d277ca84f21f12ba393 153.09MB / 211.46MB 9.3s
2026-Mar-16 01:45:08.081395
#24 sha256:e8d2a98f6bdfdbb1ba3c937c5e47cfa2cd11e74487543d277ca84f21f12ba393 166.72MB / 211.46MB 9.5s
2026-Mar-16 01:45:08.227210
#24 ...
2026-Mar-16 01:45:08.227210
2026-Mar-16 01:45:08.227210
#33 [api js-builder 12/12] RUN mkdir -p extractors/dist &&     npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js &&     npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-16 01:45:08.227210
#33 2.410
2026-Mar-16 01:45:08.227210
#33 2.410   extractors/dist/types.js  1.2kb
2026-Mar-16 01:45:08.227210
#33 2.410
2026-Mar-16 01:45:08.227210
#33 2.410 ⚡ Done in 3ms
2026-Mar-16 01:45:08.227210
#33 3.029
2026-Mar-16 01:45:08.227210
#33 3.029   extractors/dist/youtube.js  22.4kb
2026-Mar-16 01:45:08.227210
#33 3.029
2026-Mar-16 01:45:08.227210
#33 3.029 ⚡ Done in 3ms
2026-Mar-16 01:45:08.227210
#33 DONE 3.0s
2026-Mar-16 01:45:08.227210
2026-Mar-16 01:45:08.227210
#24 [worker builder  1/10] FROM docker.io/library/rust:1.91-bookworm@sha256:c1e5f19e773b7878c3f7a805dd00a495e747acbdc76fb2337a4ebf0418896b33
2026-Mar-16 01:45:08.227210
#24 extracting sha256:c8443a297fa42e27cb10653777dd5a53f82a65fbc8b2d33f82b8722199f941d3 1.5s done
2026-Mar-16 01:45:08.381459
#24 sha256:e8d2a98f6bdfdbb1ba3c937c5e47cfa2cd11e74487543d277ca84f21f12ba393 196.08MB / 211.46MB 9.8s
2026-Mar-16 01:45:08.381459
#24 extracting sha256:c8443a297fa42e27cb10653777dd5a53f82a65fbc8b2d33f82b8722199f941d3 1.5s done
2026-Mar-16 01:45:08.381459
#24 extracting sha256:6ae8659f7a8d357662281a0f87eb293725bb75ffa6c7356c38567f557d8a1f11
2026-Mar-16 01:45:08.531376
#24 sha256:e8d2a98f6bdfdbb1ba3c937c5e47cfa2cd11e74487543d277ca84f21f12ba393 209.72MB / 211.46MB 9.9s
2026-Mar-16 01:45:08.663701
#24 sha256:e8d2a98f6bdfdbb1ba3c937c5e47cfa2cd11e74487543d277ca84f21f12ba393 211.46MB / 211.46MB 9.9s done
2026-Mar-16 01:45:08.663701
#24 extracting sha256:6ae8659f7a8d357662281a0f87eb293725bb75ffa6c7356c38567f557d8a1f11 0.4s done
2026-Mar-16 01:45:08.814105
#24 extracting sha256:6ae8659f7a8d357662281a0f87eb293725bb75ffa6c7356c38567f557d8a1f11 0.4s done
2026-Mar-16 01:45:08.814105
#24 extracting sha256:c237534654fe7a5c118fcee78652af952e57a4a07cc322c0ae3c367839bb0ccc
2026-Mar-16 01:45:08.814105
#24 ...
2026-Mar-16 01:45:08.814105
2026-Mar-16 01:45:08.814105
#34 [frontend builder 5/8] RUN npm install
2026-Mar-16 01:45:08.943214
#34 ...
2026-Mar-16 01:45:08.943214
2026-Mar-16 01:45:08.943214
#35 [worker runtime 4/7] RUN set -eux;     arch="$(dpkg --print-architecture)";     case "$arch" in       amd64) ytdlp_asset="yt-dlp_linux" ;;       arm64) ytdlp_asset="yt-dlp_linux_aarch64" ;;       *) echo "Unsupported architecture: $arch" >&2; exit 1 ;;     esac;     curl -fL "https://github.com/yt-dlp/yt-dlp/releases/latest/download/${ytdlp_asset}" -o /usr/local/bin/yt-dlp;     chmod +x /usr/local/bin/yt-dlp;     /usr/local/bin/yt-dlp --version
2026-Mar-16 01:45:08.943214
#35 0.171 + dpkg --print-architecture
2026-Mar-16 01:45:08.943214
#35 0.173 + arch=amd64
2026-Mar-16 01:45:08.943214
#35 0.173 + ytdlp_asset=yt-dlp_linux
2026-Mar-16 01:45:08.943214
#35 0.173 + curl -fL https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux -o /usr/local/bin/yt-dlp
2026-Mar-16 01:45:08.943214
#35 0.179   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2026-Mar-16 01:45:08.943214
#35 0.179                                  Dload  Upload   Total   Spent    Left  Speed
2026-Mar-16 01:45:08.943214
#35 0.180 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2026-Mar-16 01:45:08.943214
#35 0.881 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2026-Mar-16 01:45:08.943214
#35 1.870 
  1 34.4M    1  619k    0     0   366k      0  0:01:36  0:00:01  0:01:35  366k
  6 34.4M    6 2251k    0     0   836k      0  0:00:42  0:00:02  0:00:40 1631k
2026-Mar-16 01:45:09.093210
12 34.4M   12 4571k    0     0  1238k      0  0:00:28  0:00:03  0:00:25 1975k
2026-Mar-16 01:45:10.095689
21 34.4M   21 7675k    0     0  1635k      0  0:00:21  0:00:04  0:00:17 2350k
2026-Mar-16 01:45:10.581195
#35 ...
2026-Mar-16 01:45:10.581195
2026-Mar-16 01:45:10.581195
#24 [worker builder  1/10] FROM docker.io/library/rust:1.91-bookworm@sha256:c1e5f19e773b7878c3f7a805dd00a495e747acbdc76fb2337a4ebf0418896b33
2026-Mar-16 01:45:10.581195
#24 extracting sha256:c237534654fe7a5c118fcee78652af952e57a4a07cc322c0ae3c367839bb0ccc 1.9s done
2026-Mar-16 01:45:10.581195
#24 extracting sha256:e8d2a98f6bdfdbb1ba3c937c5e47cfa2cd11e74487543d277ca84f21f12ba393
2026-Mar-16 01:45:10.731250
#24 extracting sha256:c237534654fe7a5c118fcee78652af952e57a4a07cc322c0ae3c367839bb0ccc 1.9s done
2026-Mar-16 01:45:11.234603
#24 ...
2026-Mar-16 01:45:11.234603
2026-Mar-16 01:45:11.234603
#34 [frontend builder 5/8] RUN npm install
2026-Mar-16 01:45:11.234603
#34 11.24
2026-Mar-16 01:45:11.234603
#34 11.24 > frontend@0.0.1 prepare
2026-Mar-16 01:45:11.234603
#34 11.24 > svelte-kit sync || echo ''
2026-Mar-16 01:45:11.234603
#34 11.24
2026-Mar-16 01:45:11.234603
#34 12.13
2026-Mar-16 01:45:11.234603
#34 12.13 added 237 packages, and audited 238 packages in 12s
2026-Mar-16 01:45:11.234603
#34 12.13
2026-Mar-16 01:45:11.234603
#34 12.13 43 packages are looking for funding
2026-Mar-16 01:45:11.234603
#34 12.13   run `npm fund` for details
2026-Mar-16 01:45:11.234603
#34 12.19
2026-Mar-16 01:45:11.234603
#34 12.19 7 vulnerabilities (2 low, 2 moderate, 3 high)
2026-Mar-16 01:45:11.234603
#34 12.19
2026-Mar-16 01:45:11.234603
#34 12.19 To address all issues, run:
2026-Mar-16 01:45:11.234603
#34 12.19   npm audit fix
2026-Mar-16 01:45:11.234603
#34 12.19
2026-Mar-16 01:45:11.234603
#34 12.19 Run `npm audit` for details.
2026-Mar-16 01:45:11.234603
#34 12.20 npm notice
2026-Mar-16 01:45:11.234603
#34 12.20 npm notice New major version of npm available! 10.9.4 -> 11.11.1
2026-Mar-16 01:45:11.234603
#34 12.20 npm notice Changelog: https://github.com/npm/cli/releases/tag/v11.11.1
2026-Mar-16 01:45:11.234603
#34 12.20 npm notice To update run: npm install -g npm@11.11.1
2026-Mar-16 01:45:11.234603
#34 12.20 npm notice
2026-Mar-16 01:45:11.234603
#34 DONE 12.5s
2026-Mar-16 01:45:11.386602
#36 [frontend builder 6/8] RUN test -n "https://api-download.khoadangbui.online" || (echo "VITE_API_URL build arg is required" && exit 1)
2026-Mar-16 01:45:11.552994
#36 DONE 0.3s
2026-Mar-16 01:45:11.552994
2026-Mar-16 01:45:11.552994
#35 [worker runtime 4/7] RUN set -eux;     arch="$(dpkg --print-architecture)";     case "$arch" in       amd64) ytdlp_asset="yt-dlp_linux" ;;       arm64) ytdlp_asset="yt-dlp_linux_aarch64" ;;       *) echo "Unsupported architecture: $arch" >&2; exit 1 ;;     esac;     curl -fL "https://github.com/yt-dlp/yt-dlp/releases/latest/download/${ytdlp_asset}" -o /usr/local/bin/yt-dlp;     chmod +x /usr/local/bin/yt-dlp;     /usr/local/bin/yt-dlp --version
2026-Mar-16 01:45:11.552994
#35 1.870 
  1 34.4M    1  619k    0     0   366k      0  0:01:36  0:00:01  0:01:35  366k
  6 34.4M    6 2251k    0     0   836k      0  0:00:42  0:00:02  0:00:40 1631k
 12 34.4M   12 4571k    0     0  1238k      0  0:00:28  0:00:03  0:00:25 1975k
 21 34.4M   21 7675k    0     0  1635k      0  0:00:21  0:00:04  0:00:17 2350k
 31 34.4M   31 10.7M    0     0  1925k      0  0:00:18  0:00:05  0:00:13 2585k
2026-Mar-16 01:45:12.094389
43 34.4M   43 15.0M    0     0  2299k      0  0:00:15  0:00:06  0:00:09 2954k
2026-Mar-16 01:45:13.064310
58 34.4M   58 20.0M    0     0  2621k      0  0:00:13  0:00:07  0:00:06 3560k
2026-Mar-16 01:45:14.093643
72 34.4M   72 25.0M    0     0  2945k      0  0:00:11  0:00:08  0:00:03 4205k
2026-Mar-16 01:45:15.093928
88 34.4M   88 30.4M    0     0  3213k      0  0:00:10  0:00:09  0:00:01 4694k
2026-Mar-16 01:45:15.531689
100 34.4M  100 34.4M    0     0  3429k      0  0:00:10  0:00:10 --:--:-- 5294k
2026-Mar-16 01:45:15.765816
#35 10.46 + chmod +x /usr/local/bin/yt-dlp
2026-Mar-16 01:45:15.765816
#35 10.46 + /usr/local/bin/yt-dlp --version
2026-Mar-16 01:45:16.208102
#35 11.13 2026.03.13
2026-Mar-16 01:45:16.756477
#35 DONE 11.5s
2026-Mar-16 01:45:16.756477
2026-Mar-16 01:45:16.756477
#37 [api runtime 5/7] RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Mar-16 01:45:16.933671
#37 DONE 0.2s
2026-Mar-16 01:45:16.933671
2026-Mar-16 01:45:16.933671
#24 [api builder  1/10] FROM docker.io/library/rust:1.91-bookworm@sha256:c1e5f19e773b7878c3f7a805dd00a495e747acbdc76fb2337a4ebf0418896b33
2026-Mar-16 01:45:16.933671
#24 extracting sha256:e8d2a98f6bdfdbb1ba3c937c5e47cfa2cd11e74487543d277ca84f21f12ba393 5.0s done
2026-Mar-16 01:45:16.933671
#24 extracting sha256:627be014d1ff781d1abd9cc5d97cd162f7a7b72eedf59e7d3adf629fb29a64db
2026-Mar-16 01:45:18.886089
#24 ...
2026-Mar-16 01:45:18.886089
2026-Mar-16 01:45:18.886089
#38 [frontend builder 7/8] RUN npm run paraglide:compile
2026-Mar-16 01:45:18.886089
#38 0.310
2026-Mar-16 01:45:18.886089
#38 0.310 > frontend@0.0.1 paraglide:compile
2026-Mar-16 01:45:18.886089
#38 0.310 > paraglide-js compile --project ./project.inlang --outdir ./src/lib/paraglide --strategy url cookie globalVariable baseLocale
2026-Mar-16 01:45:18.886089
#38 0.310
2026-Mar-16 01:45:18.886089
#38 0.817 ℹ [paraglide-js] Compiling inlang project ...
2026-Mar-16 01:45:18.886089
#38 7.214 ✔ [paraglide-js] Successfully compiled inlang project.
2026-Mar-16 01:45:18.886089
#38 DONE 7.3s
2026-Mar-16 01:45:19.037458
#39 [frontend builder 8/8] RUN node build-docker.mjs
2026-Mar-16 01:45:19.520503
#39 ...
2026-Mar-16 01:45:19.520503
2026-Mar-16 01:45:19.520503
#24 [api builder  1/10] FROM docker.io/library/rust:1.91-bookworm@sha256:c1e5f19e773b7878c3f7a805dd00a495e747acbdc76fb2337a4ebf0418896b33
2026-Mar-16 01:45:19.520503
#24 extracting sha256:627be014d1ff781d1abd9cc5d97cd162f7a7b72eedf59e7d3adf629fb29a64db 3.8s done
2026-Mar-16 01:45:19.520503
#24 DONE 20.8s
2026-Mar-16 01:45:19.528094
#40 [api builder  2/10] WORKDIR /app
2026-Mar-16 01:45:19.679576
#40 DONE 0.2s
2026-Mar-16 01:45:19.679576
2026-Mar-16 01:45:19.679576
#41 [api builder  3/10] RUN apt-get update && apt-get install -y     pkg-config     libssl-dev     && rm -rf /var/lib/apt/lists/*
2026-Mar-16 01:45:19.801376
#41 0.272 Get:1 http://deb.debian.org/debian bookworm InRelease [151 kB]
2026-Mar-16 01:45:20.007951
#41 0.375 Get:2 http://deb.debian.org/debian bookworm-updates InRelease [55.4 kB]
2026-Mar-16 01:45:20.007951
#41 0.413 Get:3 http://deb.debian.org/debian-security bookworm-security InRelease [48.0 kB]
2026-Mar-16 01:45:20.007951
#41 0.451 Get:4 http://deb.debian.org/debian bookworm/main amd64 Packages [8792 kB]
2026-Mar-16 01:45:20.328393
#41 0.799 Get:5 http://deb.debian.org/debian bookworm-updates/main amd64 Packages [6924 B]
2026-Mar-16 01:45:20.479123
#41 0.800 Get:6 http://deb.debian.org/debian-security bookworm-security/main amd64 Packages [294 kB]
2026-Mar-16 01:45:21.127178
#41 1.448 Fetched 9348 kB in 1s (7276 kB/s)
2026-Mar-16 01:45:21.127178
#41 1.448 Reading package lists...
2026-Mar-16 01:45:21.393053
2026-Mar-16 01:45:21.564986
#41 1.885 Reading package lists...
2026-Mar-16 01:45:21.951454
#41 2.302 Building dependency tree...
2026-Mar-16 01:45:21.951454
#41 2.422 Reading state information...
2026-Mar-16 01:45:22.094837
#41 2.565 pkg-config is already the newest version (1.8.1-1).
2026-Mar-16 01:45:22.094837
#41 2.565 pkg-config set to manually installed.
2026-Mar-16 01:45:22.094837
#41 2.565 The following additional packages will be installed:
2026-Mar-16 01:45:22.214797
#41 2.566   libssl3 openssl
2026-Mar-16 01:45:22.214797
#41 2.566 Suggested packages:
2026-Mar-16 01:45:22.214797
#41 2.566   libssl-doc
2026-Mar-16 01:45:22.214797
#41 2.596 The following packages will be upgraded:
2026-Mar-16 01:45:22.214797
#41 2.596   libssl-dev libssl3 openssl
2026-Mar-16 01:45:22.365238
#41 2.686 3 upgraded, 0 newly installed, 0 to remove and 53 not upgraded.
2026-Mar-16 01:45:22.365238
#41 2.686 Need to get 5906 kB of archives.
2026-Mar-16 01:45:22.365238
#41 2.686 After this operation, 15.4 kB of additional disk space will be used.
2026-Mar-16 01:45:22.365238
#41 2.686 Get:1 http://deb.debian.org/debian-security bookworm-security/main amd64 libssl-dev amd64 3.0.18-1~deb12u2 [2444 kB]
2026-Mar-16 01:45:22.540221
#41 3.011 Get:2 http://deb.debian.org/debian-security bookworm-security/main amd64 libssl3 amd64 3.0.18-1~deb12u2 [2030 kB]
2026-Mar-16 01:45:22.731834
#41 3.052 Get:3 http://deb.debian.org/debian-security bookworm-security/main amd64 openssl amd64 3.0.18-1~deb12u2 [1433 kB]
2026-Mar-16 01:45:22.953970
#41 3.207 debconf: delaying package configuration, since apt-utils is not installed
2026-Mar-16 01:45:22.953970
#41 3.235 Fetched 5906 kB in 0s (12.2 MB/s)
2026-Mar-16 01:45:22.953970
#41 3.252 (Reading database ... 
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
(Reading database ... 23257 files and directories currently installed.)
2026-Mar-16 01:45:22.953970
#41 3.271 Preparing to unpack .../libssl-dev_3.0.18-1~deb12u2_amd64.deb ...
2026-Mar-16 01:45:22.953970
#41 3.274 Unpacking libssl-dev:amd64 (3.0.18-1~deb12u2) over (3.0.17-1~deb12u3) ...
2026-Mar-16 01:45:23.115428
#41 3.464 Preparing to unpack .../libssl3_3.0.18-1~deb12u2_amd64.deb ...
2026-Mar-16 01:45:23.115428
#41 3.468 Unpacking libssl3:amd64 (3.0.18-1~deb12u2) over (3.0.17-1~deb12u3) ...
2026-Mar-16 01:45:23.115428
#41 3.586 Preparing to unpack .../openssl_3.0.18-1~deb12u2_amd64.deb ...
2026-Mar-16 01:45:23.268383
#41 3.589 Unpacking openssl (3.0.18-1~deb12u2) over (3.0.17-1~deb12u3) ...
2026-Mar-16 01:45:23.474549
#41 3.831 Setting up libssl3:amd64 (3.0.18-1~deb12u2) ...
2026-Mar-16 01:45:23.474549
#41 3.834 Setting up libssl-dev:amd64 (3.0.18-1~deb12u2) ...
2026-Mar-16 01:45:23.474549
#41 3.837 Setting up openssl (3.0.18-1~deb12u2) ...
2026-Mar-16 01:45:23.474549
#41 3.843 Processing triggers for libc-bin (2.36-9+deb12u13) ...
2026-Mar-16 01:45:23.474549
#41 DONE 3.9s
2026-Mar-16 01:45:23.474549
2026-Mar-16 01:45:23.474549
#42 [worker builder  4/10] COPY Cargo.toml ./
2026-Mar-16 01:45:23.679944
#42 DONE 0.0s
2026-Mar-16 01:45:23.679944
2026-Mar-16 01:45:23.679944
#43 [api builder  5/10] COPY Cargo.lock ./
2026-Mar-16 01:45:23.679944
#43 DONE 0.0s
2026-Mar-16 01:45:23.679944
2026-Mar-16 01:45:23.679944
#44 [worker builder  6/10] COPY crates/ ./crates/
2026-Mar-16 01:45:23.679944
#44 DONE 0.0s
2026-Mar-16 01:45:23.679944
2026-Mar-16 01:45:23.679944
#45 [api builder  7/10] COPY config/ ./config/
2026-Mar-16 01:45:23.679944
#45 DONE 0.0s
2026-Mar-16 01:45:23.679944
2026-Mar-16 01:45:23.679944
#46 [api builder  8/11] COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Mar-16 01:45:23.679944
#46 DONE 0.1s
2026-Mar-16 01:45:23.679944
2026-Mar-16 01:45:23.679944
#47 [worker builder  8/10] COPY extractors/ ./extractors/
2026-Mar-16 01:45:23.873452
#47 DONE 0.1s
2026-Mar-16 01:45:23.873452
2026-Mar-16 01:45:23.873452
#48 [api builder  9/11] COPY extractors/ ./extractors/
2026-Mar-16 01:45:23.873452
#48 DONE 0.0s
2026-Mar-16 01:45:23.873452
2026-Mar-16 01:45:23.873452
#49 [worker builder  9/10] COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-16 01:45:23.873452
#49 DONE 0.0s
2026-Mar-16 01:45:23.873452
2026-Mar-16 01:45:23.873452
#50 [api builder 10/11] COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-16 01:45:23.873452
#50 DONE 0.0s
2026-Mar-16 01:45:23.873452
2026-Mar-16 01:45:23.873452
#51 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-16 01:45:24.126978
#51 0.254     Updating crates.io index
2026-Mar-16 01:45:29.203361
#51 5.481  Downloading crates ...
2026-Mar-16 01:45:29.588002
#51 5.865   Downloaded adler2 v2.0.1
2026-Mar-16 01:45:29.741307
#51 5.937   Downloaded adler v1.0.2
2026-Mar-16 01:45:29.741307
#51 6.019   Downloaded atoi v2.0.0
2026-Mar-16 01:45:29.854625
#51 6.057   Downloaded atomic-waker v1.1.2
2026-Mar-16 01:45:29.854625
#51 6.099   Downloaded alloc-no-stdlib v2.0.4
2026-Mar-16 01:45:29.854625
#51 6.108   Downloaded alloc-stdlib v0.2.2
2026-Mar-16 01:45:29.854625
#51 6.132   Downloaded aws-smithy-observability v0.2.6
2026-Mar-16 01:45:29.978325
#51 6.184   Downloaded compression-core v0.4.31
2026-Mar-16 01:45:29.978325
#51 6.187   Downloaded async-stream v0.3.6
2026-Mar-16 01:45:29.978325
#51 6.193   Downloaded const-random v0.1.18
2026-Mar-16 01:45:29.978325
#51 6.196   Downloaded base64-simd v0.8.0
2026-Mar-16 01:45:29.978325
#51 6.200   Downloaded crossbeam-queue v0.3.12
2026-Mar-16 01:45:29.978325
#51 6.204   Downloaded bit-set v0.5.3
2026-Mar-16 01:45:29.978325
#51 6.208   Downloaded base64-simd v0.7.0
2026-Mar-16 01:45:29.978325
#51 6.211   Downloaded arraydeque v0.5.1
2026-Mar-16 01:45:29.978325
#51 6.218   Downloaded digest v0.10.7
2026-Mar-16 01:45:29.978325
#51 ...
2026-Mar-16 01:45:29.978325
2026-Mar-16 01:45:29.978325
#39 [frontend builder 8/8] RUN node build-docker.mjs
2026-Mar-16 01:45:29.978325
#39 1.065 The following Vite config options will be overridden by SvelteKit:
2026-Mar-16 01:45:29.978325
#39 1.065   - build.outDir
2026-Mar-16 01:45:29.978325
#39 1.096 vite v6.4.1 building SSR bundle for production...
2026-Mar-16 01:45:29.978325
#39 1.121 transforming...
2026-Mar-16 01:45:29.978325
#39 5.942 "optionsMiddleware" is imported from external module "@better-auth/core/api" but never used in "node_modules/better-auth/dist/api/index.mjs" and "node_modules/better-auth/dist/plugins/index.mjs".
2026-Mar-16 01:45:29.978325
#39 5.942 "getTelemetryAuthConfig" is imported from external module "@better-auth/telemetry" but never used in "node_modules/better-auth/dist/index.mjs".
2026-Mar-16 01:45:29.978325
#39 5.943 ✓ 962 modules transformed.
2026-Mar-16 01:45:29.978325
#39 6.157 rendering chunks...
2026-Mar-16 01:45:29.978325
#39 7.280 vite v6.4.1 building for production...
2026-Mar-16 01:45:30.079888
#39 ...
2026-Mar-16 01:45:30.079888
2026-Mar-16 01:45:30.079888
#51 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-16 01:45:30.079888
#51 6.272   Downloaded crc v3.4.0
2026-Mar-16 01:45:30.079888
#51 6.283   Downloaded async-stream-impl v0.3.6
2026-Mar-16 01:45:30.079888
#51 6.287   Downloaded base16ct v0.1.1
2026-Mar-16 01:45:30.079888
#51 6.291   Downloaded cfg_aliases v0.2.1
2026-Mar-16 01:45:30.079888
#51 6.295   Downloaded foreign-types-shared v0.1.1
2026-Mar-16 01:45:30.079888
#51 6.357   Downloaded const-random-macro v0.1.16
2026-Mar-16 01:45:30.181787
#51 6.361   Downloaded crc-catalog v2.4.0
2026-Mar-16 01:45:30.181787
#51 6.377   Downloaded crypto-common v0.1.7
2026-Mar-16 01:45:30.181787
#51 6.381   Downloaded block-buffer v0.10.4
2026-Mar-16 01:45:30.181787
#51 6.389   Downloaded cpufeatures v0.2.17
2026-Mar-16 01:45:30.181787
#51 6.402   Downloaded dlv-list v0.5.2
2026-Mar-16 01:45:30.181787
#51 6.414   Downloaded dunce v1.0.5
2026-Mar-16 01:45:30.181787
#51 6.416   Downloaded document-features v0.2.12
2026-Mar-16 01:45:30.181787
#51 6.419   Downloaded form_urlencoded v1.2.2
2026-Mar-16 01:45:30.181787
#51 6.423   Downloaded displaydoc v0.2.5
2026-Mar-16 01:45:30.181787
#51 6.431   Downloaded fslock v0.2.1
2026-Mar-16 01:45:30.181787
#51 6.438   Downloaded http-body v0.4.6
2026-Mar-16 01:45:30.181787
#51 6.444   Downloaded crunchy v0.2.4
2026-Mar-16 01:45:30.181787
#51 6.447   Downloaded cooked-waker v5.0.0
2026-Mar-16 01:45:30.181787
#51 6.450   Downloaded aws-smithy-query v0.60.15
2026-Mar-16 01:45:30.181787
#51 6.456   Downloaded heck v0.4.1
2026-Mar-16 01:45:30.282702
#51 6.461   Downloaded aws-smithy-checksums v0.63.12
2026-Mar-16 01:45:30.282702
#51 6.465   Downloaded aws-smithy-async v1.2.14
2026-Mar-16 01:45:30.282702
#51 6.471   Downloaded fnv v1.0.7
2026-Mar-16 01:45:30.282702
#51 6.474   Downloaded lru v0.12.5
2026-Mar-16 01:45:30.282702
#51 6.477   Downloaded dashmap v6.1.0
2026-Mar-16 01:45:30.282702
#51 6.487   Downloaded async-trait v0.1.89
2026-Mar-16 01:45:30.282702
#51 6.517   Downloaded debugid v0.8.0
2026-Mar-16 01:45:30.282702
#51 6.521   Downloaded cmake v0.1.57
2026-Mar-16 01:45:30.282702
#51 6.529   Downloaded data-encoding v2.10.0
2026-Mar-16 01:45:30.282702
#51 6.532   Downloaded concurrent-queue v2.5.0
2026-Mar-16 01:45:30.282702
#51 6.537   Downloaded convert_case v0.6.0
2026-Mar-16 01:45:30.282702
#51 6.541   Downloaded bytes-utils v0.1.4
2026-Mar-16 01:45:30.282702
#51 6.546   Downloaded funty v2.0.0
2026-Mar-16 01:45:30.282702
#51 6.548   Downloaded aws-smithy-xml v0.60.15
2026-Mar-16 01:45:30.282702
#51 6.552   Downloaded bincode v1.3.3
2026-Mar-16 01:45:30.282702
#51 6.557   Downloaded autocfg v1.5.0
2026-Mar-16 01:45:30.383973
#51 6.561   Downloaded byteorder v1.5.0
2026-Mar-16 01:45:30.383973
#51 6.565   Downloaded matchers v0.2.0
2026-Mar-16 01:45:30.383973
#51 6.568   Downloaded lru-slab v0.1.2
2026-Mar-16 01:45:30.383973
#51 6.571   Downloaded aws-smithy-http v0.62.6
2026-Mar-16 01:45:30.383973
#51 6.576   Downloaded compression-codecs v0.4.37
2026-Mar-16 01:45:30.383973
#51 6.583   Downloaded cfg-if v1.0.4
2026-Mar-16 01:45:30.383973
#51 6.587   Downloaded axum-core v0.5.6
2026-Mar-16 01:45:30.383973
#51 6.591   Downloaded aws-smithy-json v0.61.9
2026-Mar-16 01:45:30.383973
#51 6.595   Downloaded aws-smithy-http v0.63.6
2026-Mar-16 01:45:30.383973
#51 6.600   Downloaded cexpr v0.6.0
2026-Mar-16 01:45:30.383973
#51 6.604   Downloaded md-5 v0.10.6
2026-Mar-16 01:45:30.383973
#51 6.608   Downloaded bit-vec v0.6.3
2026-Mar-16 01:45:30.383973
#51 6.611   Downloaded base64ct v1.8.3
2026-Mar-16 01:45:30.383973
#51 6.616   Downloaded anyhow v1.0.102
2026-Mar-16 01:45:30.383973
#51 6.625   Downloaded mime v0.3.17
2026-Mar-16 01:45:30.383973
#51 6.629   Downloaded futures-macro v0.3.32
2026-Mar-16 01:45:30.383973
#51 6.633   Downloaded hex v0.4.3
2026-Mar-16 01:45:30.383973
#51 6.637   Downloaded cookie v0.18.1
2026-Mar-16 01:45:30.383973
#51 6.642   Downloaded dotenvy v0.15.7
2026-Mar-16 01:45:30.383973
#51 6.647   Downloaded memoffset v0.9.1
2026-Mar-16 01:45:30.383973
#51 6.650   Downloaded litrs v1.0.0
2026-Mar-16 01:45:30.383973
#51 6.655   Downloaded futures-task v0.3.32
2026-Mar-16 01:45:30.383973
#51 6.659   Downloaded equivalent v1.0.2
2026-Mar-16 01:45:30.383973
#51 6.661   Downloaded heck v0.5.0
2026-Mar-16 01:45:30.485479
#51 6.665   Downloaded nonzero_ext v0.3.0
2026-Mar-16 01:45:30.485479
#51 6.673   Downloaded litemap v0.8.1
2026-Mar-16 01:45:30.485479
#51 6.678   Downloaded http-body v1.0.1
2026-Mar-16 01:45:30.485479
#51 6.680   Downloaded futures-sink v0.3.32
2026-Mar-16 01:45:30.485479
#51 6.683   Downloaded config v0.14.1
2026-Mar-16 01:45:30.485479
#51 6.691   Downloaded deranged v0.5.8
2026-Mar-16 01:45:30.485479
#51 6.693   Downloaded crossbeam-utils v0.8.21
2026-Mar-16 01:45:30.485479
#51 6.697   Downloaded cookie_store v0.22.1
2026-Mar-16 01:45:30.485479
#51 6.701   Downloaded aws-types v1.3.14
2026-Mar-16 01:45:30.485479
#51 6.705   Downloaded clang-sys v1.8.1
2026-Mar-16 01:45:30.485479
#51 6.709   Downloaded async-lock v3.4.2
2026-Mar-16 01:45:30.485479
#51 6.713   Downloaded matchit v0.8.4
2026-Mar-16 01:45:30.485479
#51 6.717   Downloaded log v0.4.29
2026-Mar-16 01:45:30.485479
#51 6.722   Downloaded bytes v1.11.1
2026-Mar-16 01:45:30.485479
#51 6.728   Downloaded aws-smithy-runtime-api v1.11.6
2026-Mar-16 01:45:30.485479
#51 6.735   Downloaded openssl-probe v0.2.1
2026-Mar-16 01:45:30.485479
#51 6.737   Downloaded openssl-macros v0.1.1
2026-Mar-16 01:45:30.485479
#51 6.739   Downloaded nu-ansi-term v0.50.3
2026-Mar-16 01:45:30.485479
#51 6.743   Downloaded lazycell v1.3.0
2026-Mar-16 01:45:30.485479
#51 6.745   Downloaded fastrand v2.3.0
2026-Mar-16 01:45:30.485479
#51 6.748   Downloaded native-tls v0.2.18
2026-Mar-16 01:45:30.485479
#51 6.750   Downloaded aws-sdk-sso v1.96.0
2026-Mar-16 01:45:30.485479
#51 6.760   Downloaded json5 v0.4.1
2026-Mar-16 01:45:30.485479
#51 6.763   Downloaded httpdate v1.0.3
2026-Mar-16 01:45:30.586455
#51 6.765   Downloaded gzip-header v1.0.0
2026-Mar-16 01:45:30.586455
#51 6.767   Downloaded futures-core v0.3.32
2026-Mar-16 01:45:30.586455
#51 6.770   Downloaded event-listener-strategy v0.5.4
2026-Mar-16 01:45:30.586455
#51 6.773   Downloaded idna_adapter v1.2.1
2026-Mar-16 01:45:30.586455
#51 6.775   Downloaded base64 v0.22.1
2026-Mar-16 01:45:30.586455
#51 6.782   Downloaded arc-swap v1.8.2
2026-Mar-16 01:45:30.586455
#51 6.789   Downloaded crypto-bigint v0.5.5
2026-Mar-16 01:45:30.586455
#51 6.805   Downloaded pathdiff v0.2.3
2026-Mar-16 01:45:30.586455
#51 6.809   Downloaded pin-utils v0.1.0
2026-Mar-16 01:45:30.586455
#51 6.812   Downloaded pem v3.0.6
2026-Mar-16 01:45:30.586455
#51 6.815   Downloaded potential_utf v0.1.4
2026-Mar-16 01:45:30.586455
#51 6.817   Downloaded percent-encoding v2.3.2
2026-Mar-16 01:45:30.586455
#51 6.819   Downloaded pem-rfc7468 v0.7.0
2026-Mar-16 01:45:30.586455
#51 6.824   Downloaded pin-project-lite v0.2.16
2026-Mar-16 01:45:30.586455
#51 6.834   Downloaded pkcs8 v0.10.2
2026-Mar-16 01:45:30.586455
#51 6.839   Downloaded aws-config v1.8.15
2026-Mar-16 01:45:30.586455
#51 6.848   Downloaded pin-project-internal v1.1.10
2026-Mar-16 01:45:30.586455
#51 6.851   Downloaded errno v0.3.14
2026-Mar-16 01:45:30.586455
#51 6.854   Downloaded lazy_static v1.5.0
2026-Mar-16 01:45:30.586455
#51 6.857   Downloaded if_chain v1.0.3
2026-Mar-16 01:45:30.586455
#51 6.859   Downloaded hyper-tls v0.6.0
2026-Mar-16 01:45:30.586455
#51 6.861   Downloaded generic-array v0.14.7
2026-Mar-16 01:45:30.586455
#51 6.864   Downloaded itoa v1.0.17
2026-Mar-16 01:45:30.686551
#51 6.866   Downloaded ff v0.12.1
2026-Mar-16 01:45:30.686551
#51 6.870   Downloaded futures-io v0.3.32
2026-Mar-16 01:45:30.686551
#51 6.872   Downloaded foreign-types v0.3.2
2026-Mar-16 01:45:30.686551
#51 6.873   Downloaded pkg-config v0.3.32
2026-Mar-16 01:45:30.686551
#51 6.877   Downloaded proc-macro-error-attr v1.0.4
2026-Mar-16 01:45:30.686551
#51 6.879   Downloaded brotli-decompressor v5.0.0
2026-Mar-16 01:45:30.686551
#51 6.886   Downloaded ipnet v2.11.0
2026-Mar-16 01:45:30.686551
#51 6.889   Downloaded proc-macro-error v1.0.4
2026-Mar-16 01:45:30.686551
#51 6.897   Downloaded bindgen v0.69.5
2026-Mar-16 01:45:30.686551
#51 6.908   Downloaded proc-macro-rules v0.4.0
2026-Mar-16 01:45:30.686551
#51 6.911   Downloaded radium v0.7.0
2026-Mar-16 01:45:30.686551
#51 6.913   Downloaded proc-macro2 v1.0.106
2026-Mar-16 01:45:30.686551
#51 6.918   Downloaded psl-types v2.0.11
2026-Mar-16 01:45:30.686551
#51 6.920   Downloaded proc-macro-rules-macros v0.4.0
2026-Mar-16 01:45:30.686551
#51 6.922   Downloaded moka v0.12.13
2026-Mar-16 01:45:30.686551
#51 6.933   Downloaded rand_core v0.6.4
2026-Mar-16 01:45:30.686551
#51 6.936   Downloaded nom v7.1.3
2026-Mar-16 01:45:30.686551
#51 6.943   Downloaded no-std-compat v0.4.1
2026-Mar-16 01:45:30.686551
#51 6.945   Downloaded ecdsa v0.14.8
2026-Mar-16 01:45:30.686551
#51 6.947   Downloaded deno_core v0.300.0
2026-Mar-16 01:45:30.686551
#51 6.961   Downloaded rand_chacha v0.3.1
2026-Mar-16 01:45:30.686551
#51 6.964   Downloaded quinn-udp v0.5.14
2026-Mar-16 01:45:30.787514
#51 6.968   Downloaded num-bigint v0.4.6
2026-Mar-16 01:45:30.787514
#51 6.977   Downloaded num-bigint-dig v0.8.6
2026-Mar-16 01:45:30.787514
#51 6.985   Downloaded rustc-hash v1.1.0
2026-Mar-16 01:45:30.787514
#51 6.988   Downloaded rustc_version v0.2.3
2026-Mar-16 01:45:30.787514
#51 6.990   Downloaded rustc-hash v2.1.1
2026-Mar-16 01:45:30.787514
#51 6.992   Downloaded rustc_version v0.4.1
2026-Mar-16 01:45:30.787514
#51 6.995   Downloaded rustls-native-certs v0.8.3
2026-Mar-16 01:45:30.787514
#51 7.000   Downloaded openssl-sys v0.9.111
2026-Mar-16 01:45:30.787514
#51 7.011   Downloaded scopeguard v1.2.0
2026-Mar-16 01:45:30.787514
#51 7.013   Downloaded ryu v1.0.23
2026-Mar-16 01:45:30.787514
#51 7.019   Downloaded rustversion v1.0.22
2026-Mar-16 01:45:30.787514
#51 7.023   Downloaded rustls-pki-types v1.14.0
2026-Mar-16 01:45:30.787514
#51 7.026   Downloaded p256 v0.11.1
2026-Mar-16 01:45:30.787514
#51 7.030   Downloaded hyper-rustls v0.24.2
2026-Mar-16 01:45:30.787514
#51 7.033   Downloaded rust-ini v0.20.0
2026-Mar-16 01:45:30.787514
#51 7.036   Downloaded brotli v8.0.2
2026-Mar-16 01:45:30.787514
#51 7.052   Downloaded home v0.5.12
2026-Mar-16 01:45:30.787514
#51 7.054   Downloaded hashlink v0.10.0
2026-Mar-16 01:45:30.787514
#51 7.057   Downloaded jobserver v0.1.34
2026-Mar-16 01:45:30.787514
#51 7.060   Downloaded quanta v0.12.6
2026-Mar-16 01:45:30.787514
#51 7.063   Downloaded outref v0.1.0
2026-Mar-16 01:45:30.787514
#51 7.065   Downloaded num_cpus v1.17.0
2026-Mar-16 01:45:30.888466
#51 7.069   Downloaded num-iter v0.1.45
2026-Mar-16 01:45:30.888466
#51 7.072   Downloaded libloading v0.8.9
2026-Mar-16 01:45:30.888466
#51 7.076   Downloaded http-body-util v0.1.3
2026-Mar-16 01:45:30.888466
#51 7.079   Downloaded futures-channel v0.3.32
2026-Mar-16 01:45:30.888466
#51 7.082   Downloaded num-conv v0.2.0
2026-Mar-16 01:45:30.888466
#51 7.084   Downloaded bitvec v1.0.1
2026-Mar-16 01:45:30.888466
#51 7.110   Downloaded ordered-multimap v0.7.3
2026-Mar-16 01:45:30.888466
#51 7.113   Downloaded hashlink v0.8.4
2026-Mar-16 01:45:30.888466
#51 7.115   Downloaded glob v0.3.3
2026-Mar-16 01:45:30.888466
#51 7.117   Downloaded futures-timer v3.0.3
2026-Mar-16 01:45:30.888466
#51 7.119   Downloaded hyper-rustls v0.27.7
2026-Mar-16 01:45:30.888466
#51 7.123   Downloaded event-listener v5.4.1
2026-Mar-16 01:45:30.888466
#51 7.125   Downloaded num-integer v0.1.46
2026-Mar-16 01:45:30.888466
#51 7.127   Downloaded group v0.12.1
2026-Mar-16 01:45:30.888466
#51 7.130   Downloaded aws-lc-rs v1.16.1
2026-Mar-16 01:45:30.888466
#51 7.147   Downloaded mio v1.1.1
2026-Mar-16 01:45:30.888466
#51 7.157   Downloaded serde_urlencoded v0.7.1
2026-Mar-16 01:45:30.888466
#51 7.162   Downloaded stable_deref_trait v1.2.1
2026-Mar-16 01:45:30.888466
#51 7.164   Downloaded time-core v0.1.8
2026-Mar-16 01:45:30.888466
#51 7.166   Downloaded stringprep v0.1.5
2026-Mar-16 01:45:30.989562
#51 7.168   Downloaded static_assertions v1.1.0
2026-Mar-16 01:45:30.989562
#51 7.171   Downloaded tinystr v0.8.2
2026-Mar-16 01:45:30.989562
#51 7.174   Downloaded tiny-keccak v2.0.2
2026-Mar-16 01:45:30.989562
#51 7.178   Downloaded thread_local v1.1.9
2026-Mar-16 01:45:30.989562
#51 7.180   Downloaded thiserror v2.0.18
2026-Mar-16 01:45:30.989562
#51 7.190   Downloaded rfc6979 v0.3.1
2026-Mar-16 01:45:30.989562
#51 7.192   Downloaded semver-parser v0.7.0
2026-Mar-16 01:45:30.989562
#51 7.194   Downloaded rand_core v0.9.5
2026-Mar-16 01:45:30.989562
#51 7.196   Downloaded rand_chacha v0.9.0
2026-Mar-16 01:45:30.989562
#51 7.198   Downloaded quote v1.0.44
2026-Mar-16 01:45:30.989562
#51 7.202   Downloaded once_cell v1.21.3
2026-Mar-16 01:45:30.989562
#51 7.207   Downloaded getrandom v0.2.17
2026-Mar-16 01:45:30.989562
#51 7.211   Downloaded thiserror v1.0.69
2026-Mar-16 01:45:30.989562
#51 7.222   Downloaded tokio-rustls v0.26.4
2026-Mar-16 01:45:30.989562
#51 7.227   Downloaded toml_datetime v0.6.11
2026-Mar-16 01:45:30.989562
#51 7.229   Downloaded miniz_oxide v0.7.4
2026-Mar-16 01:45:30.989562
#51 7.233   Downloaded sec1 v0.3.0
2026-Mar-16 01:45:30.989562
#51 7.236   Downloaded sct v0.7.1
2026-Mar-16 01:45:30.989562
#51 7.244   Downloaded try-lock v0.2.5
2026-Mar-16 01:45:30.989562
#51 7.246   Downloaded rsa v0.9.10
2026-Mar-16 01:45:30.989562
#51 7.255   Downloaded untrusted v0.9.0
2026-Mar-16 01:45:30.989562
#51 7.258   Downloaded unicode-id-start v1.4.0
2026-Mar-16 01:45:30.989562
#51 7.263   Downloaded unicode-bidi v0.3.18
2026-Mar-16 01:45:30.989562
#51 7.267   Downloaded redis v0.27.6
2026-Mar-16 01:45:31.092326
#51 7.278   Downloaded ucd-trie v0.1.7
2026-Mar-16 01:45:31.092326
#51 7.279   Downloaded sync_wrapper v1.0.2
2026-Mar-16 01:45:31.092326
#51 7.280   Downloaded regex v1.12.3
2026-Mar-16 01:45:31.092326
#51 7.287   Downloaded quinn-proto v0.11.13
2026-Mar-16 01:45:31.092326
#51 7.295   Downloaded icu_properties v2.1.2
2026-Mar-16 01:45:31.092326
#51 7.299   Downloaded elliptic-curve v0.12.3
2026-Mar-16 01:45:31.092326
#51 7.305   Downloaded tower-service v0.3.3
2026-Mar-16 01:45:31.092326
#51 7.307   Downloaded strum_macros v0.25.3
2026-Mar-16 01:45:31.092326
#51 7.310   Downloaded semver v1.0.27
2026-Mar-16 01:45:31.092326
#51 7.314   Downloaded tinyvec_macros v0.1.1
2026-Mar-16 01:45:31.092326
#51 7.315   Downloaded tap v1.0.1
2026-Mar-16 01:45:31.092326
#51 7.317   Downloaded tagptr v0.2.0
2026-Mar-16 01:45:31.092326
#51 7.319   Downloaded subtle v2.6.1
2026-Mar-16 01:45:31.092326
#51 7.321   Downloaded semver v0.9.0
2026-Mar-16 01:45:31.092326
#51 7.323   Downloaded ron v0.8.1
2026-Mar-16 01:45:31.092326
#51 7.334   Downloaded utf8_iter v1.0.4
2026-Mar-16 01:45:31.092326
#51 7.336   Downloaded tokio-macros v2.6.0
2026-Mar-16 01:45:31.092326
#51 7.337   Downloaded serde_derive v1.0.228
2026-Mar-16 01:45:31.092326
#51 7.341   Downloaded http v1.4.0
2026-Mar-16 01:45:31.092326
#51 7.346   Downloaded icu_collections v2.1.1
2026-Mar-16 01:45:31.092326
#51 7.354   Downloaded tokio-stream v0.1.18
2026-Mar-16 01:45:31.092326
#51 7.361   Downloaded tokio-native-tls v0.3.1
2026-Mar-16 01:45:31.092326
#51 7.363   Downloaded serde_core v1.0.228
2026-Mar-16 01:45:31.092326
#51 7.366   Downloaded hyper-util v0.1.20
2026-Mar-16 01:45:31.195658
#51 7.375   Downloaded http v0.2.12
2026-Mar-16 01:45:31.195658
#51 7.381   Downloaded flate2 v1.1.9
2026-Mar-16 01:45:31.195658
#51 7.389   Downloaded thiserror-impl v1.0.69
2026-Mar-16 01:45:31.195658
#51 7.390   Downloaded synstructure v0.13.2
2026-Mar-16 01:45:31.195658
#51 7.392   Downloaded indexmap v2.13.0
2026-Mar-16 01:45:31.195658
#51 7.397   Downloaded getrandom v0.3.4
2026-Mar-16 01:45:31.195658
#51 7.401   Downloaded futures-intrusive v0.5.0
2026-Mar-16 01:45:31.195658
#51 7.407   Downloaded icu_normalizer_data v2.1.1
2026-Mar-16 01:45:31.195658
#51 7.410   Downloaded icu_normalizer v2.1.1
2026-Mar-16 01:45:31.195658
#51 7.414   Downloaded icu_locale_core v2.1.1
2026-Mar-16 01:45:31.195658
#51 7.423   Downloaded tracing-core v0.1.36
2026-Mar-16 01:45:31.195658
#51 7.426   Downloaded tracing-attributes v0.1.31
2026-Mar-16 01:45:31.195658
#51 7.430   Downloaded regex-lite v0.1.9
2026-Mar-16 01:45:31.195658
#51 7.433   Downloaded tracing-log v0.2.0
2026-Mar-16 01:45:31.195658
#51 7.434   Downloaded tower-layer v0.3.3
2026-Mar-16 01:45:31.195658
#51 7.436   Downloaded toml_write v0.1.2
2026-Mar-16 01:45:31.195658
#51 7.437   Downloaded strum v0.25.0
2026-Mar-16 01:45:31.195658
#51 7.438   Downloaded rand v0.9.2
2026-Mar-16 01:45:31.195658
#51 7.444   Downloaded rand v0.8.5
2026-Mar-16 01:45:31.195658
#51 7.449   Downloaded quinn v0.11.9
2026-Mar-16 01:45:31.195658
#51 7.454   Downloaded publicsuffix v2.3.0
2026-Mar-16 01:45:31.195658
#51 7.457   Downloaded num-traits v0.2.19
2026-Mar-16 01:45:31.195658
#51 7.460   Downloaded icu_provider v2.1.1
2026-Mar-16 01:45:31.195658
#51 7.464   Downloaded httparse v1.10.1
2026-Mar-16 01:45:31.195658
#51 7.468   Downloaded futures v0.3.32
2026-Mar-16 01:45:31.295968
#51 7.477   Downloaded flume v0.11.1
2026-Mar-16 01:45:31.295968
#51 7.483   Downloaded toml v0.8.23
2026-Mar-16 01:45:31.295968
#51 7.488   Downloaded urlencoding v2.1.3
2026-Mar-16 01:45:31.295968
#51 7.490   Downloaded want v0.3.1
2026-Mar-16 01:45:31.295968
#51 7.492   Downloaded web-time v1.1.0
2026-Mar-16 01:45:31.295968
#51 7.495   Downloaded whoami v1.6.1
2026-Mar-16 01:45:31.295968
#51 7.498   Downloaded rustls-webpki v0.101.7
2026-Mar-16 01:45:31.295968
#51 7.522   Downloaded unicode-properties v0.1.4
2026-Mar-16 01:45:31.295968
#51 7.526   Downloaded xmlparser v0.13.6
2026-Mar-16 01:45:31.295968
#51 7.530   Downloaded serde v1.0.228
2026-Mar-16 01:45:31.295968
#51 7.535   Downloaded reqwest v0.12.28
2026-Mar-16 01:45:31.295968
#51 7.541   Downloaded portable-atomic v1.13.1
2026-Mar-16 01:45:31.295968
#51 7.550   Downloaded hyper v0.14.32
2026-Mar-16 01:45:31.295968
#51 7.559   Downloaded hkdf v0.12.4
2026-Mar-16 01:45:31.295968
#51 7.563   Downloaded libm v0.2.16
2026-Mar-16 01:45:31.398836
#51 7.579   Downloaded itertools v0.12.1
2026-Mar-16 01:45:31.398836
#51 7.588   Downloaded idna v1.1.0
2026-Mar-16 01:45:31.398836
#51 7.592   Downloaded icu_properties_data v2.1.2
2026-Mar-16 01:45:31.398836
#51 7.606   Downloaded zmij v1.0.21
2026-Mar-16 01:45:31.398836
#51 7.608   Downloaded sqlx-postgres v0.8.6
2026-Mar-16 01:45:31.398836
#51 7.620   Downloaded toml_edit v0.22.27
2026-Mar-16 01:45:31.398836
#51 7.626   Downloaded zerofrom-derive v0.1.6
2026-Mar-16 01:45:31.398836
#51 7.628   Downloaded zeroize v1.8.2
2026-Mar-16 01:45:31.398836
#51 7.630   Downloaded utoipa-gen v4.3.1
2026-Mar-16 01:45:31.398836
#51 7.637   Downloaded yoke v0.8.1
2026-Mar-16 01:45:31.398836
#51 7.640   Downloaded typenum v1.19.0
2026-Mar-16 01:45:31.398836
#51 7.644   Downloaded vcpkg v0.2.15
2026-Mar-16 01:45:31.500807
#51 7.726   Downloaded regex-syntax v0.8.10
2026-Mar-16 01:45:31.500807
#51 7.735   Downloaded tracing-subscriber v0.3.22
2026-Mar-16 01:45:31.500807
#51 7.746   Downloaded zerofrom v0.1.6
2026-Mar-16 01:45:31.500807
#51 7.749   Downloaded yoke-derive v0.8.1
2026-Mar-16 01:45:31.500807
#51 7.751   Downloaded tower-http v0.6.8
2026-Mar-16 01:45:31.500807
#51 7.762   Downloaded tower v0.5.3
2026-Mar-16 01:45:31.500807
#51 7.775   Downloaded tower v0.4.13
2026-Mar-16 01:45:31.601791
#51 7.789   Downloaded openssl v0.10.75
2026-Mar-16 01:45:31.601791
#51 7.800   Downloaded time v0.3.47
2026-Mar-16 01:45:31.601791
#51 7.816   Downloaded syn v1.0.109
2026-Mar-16 01:45:31.601791
#51 7.829   Downloaded libc v0.2.182
2026-Mar-16 01:45:31.703857
#51 7.886   Downloaded tokio-util v0.7.18
2026-Mar-16 01:45:31.703857
#51 7.898   Downloaded syn v2.0.117
2026-Mar-16 01:45:31.703857
#51 7.911   Downloaded sqlx v0.8.6
2026-Mar-16 01:45:31.703857
#51 7.929   Downloaded aws-lc-sys v0.38.0
2026-Mar-16 01:45:32.056767
#51 8.244   Downloaded sqlx-core v0.8.6
2026-Mar-16 01:45:32.056767
#51 8.255   Downloaded winnow v0.7.14
2026-Mar-16 01:45:32.056767
#51 8.266   Downloaded unicode-segmentation v1.12.0
2026-Mar-16 01:45:32.056767
#51 8.270   Downloaded url v2.5.8
2026-Mar-16 01:45:32.056767
#51 8.273   Downloaded webpki-roots v1.0.6
2026-Mar-16 01:45:32.056767
#51 8.276   Downloaded unicode-normalization v0.1.25
2026-Mar-16 01:45:32.056767
#51 8.280   Downloaded tracing v0.1.44
2026-Mar-16 01:45:32.056767
#51 8.297   Downloaded serde_v8 v0.209.0
2026-Mar-16 01:45:32.056767
#51 8.301   Downloaded zerovec-derive v0.11.2
2026-Mar-16 01:45:32.056767
#51 8.303   Downloaded sqlx-sqlite v0.8.6
2026-Mar-16 01:45:32.056767
#51 8.308   Downloaded encoding_rs v0.8.35
2026-Mar-16 01:45:32.056767
#51 8.328   Downloaded sqlx-mysql v0.8.6
2026-Mar-16 01:45:32.195334
#51 8.336   Downloaded serde_json v1.0.149
2026-Mar-16 01:45:32.195334
#51 8.345   Downloaded hyper v1.8.1
2026-Mar-16 01:45:32.195334
#51 8.353   Downloaded hashbrown v0.16.1
2026-Mar-16 01:45:32.195334
#51 8.358   Downloaded hashbrown v0.14.5
2026-Mar-16 01:45:32.195334
#51 8.364   Downloaded ring v0.17.14
2026-Mar-16 01:45:32.195334
#51 8.412   Downloaded h2 v0.4.13
2026-Mar-16 01:45:32.195334
#51 8.419   Downloaded tokio v1.49.0
2026-Mar-16 01:45:32.195334
#51 8.473   Downloaded h2 v0.3.27
2026-Mar-16 01:45:32.305595
#51 8.480   Downloaded governor v0.8.1
2026-Mar-16 01:45:32.305595
#51 8.485   Downloaded futures-util v0.3.32
2026-Mar-16 01:45:32.305595
#51 8.502   Downloaded wyz v0.5.1
2026-Mar-16 01:45:32.305595
#51 8.504   Downloaded writeable v0.6.2
2026-Mar-16 01:45:32.305595
#51 8.506   Downloaded which v6.0.3
2026-Mar-16 01:45:32.305595
#51 8.508   Downloaded utoipa v4.2.3
2026-Mar-16 01:45:32.305595
#51 8.511   Downloaded unicode-ident v1.0.24
2026-Mar-16 01:45:32.305595
#51 8.515   Downloaded raw-cpuid v11.6.0
2026-Mar-16 01:45:32.305595
#51 8.518   Downloaded itertools v0.13.0
2026-Mar-16 01:45:32.305595
#51 8.528   Downloaded iri-string v0.7.10
2026-Mar-16 01:45:32.305595
#51 8.538   Downloaded linux-raw-sys v0.4.15
2026-Mar-16 01:45:32.410997
#51 8.586   Downloaded hashbrown v0.15.5
2026-Mar-16 01:45:32.410997
#51 8.592   Downloaded rustls v0.23.37
2026-Mar-16 01:45:32.410997
#51 8.606   Downloaded rustls v0.21.12
2026-Mar-16 01:45:32.410997
#51 8.622   Downloaded which v4.4.2
2026-Mar-16 01:45:32.410997
#51 8.625   Downloaded webpki-roots v0.26.11
2026-Mar-16 01:45:32.410997
#51 8.627   Downloaded rustls-webpki v0.103.9
2026-Mar-16 01:45:32.410997
#51 8.631   Downloaded rustix v0.38.44
2026-Mar-16 01:45:32.410997
#51 8.671   Downloaded vsimd v0.8.0
2026-Mar-16 01:45:32.410997
#51 8.675   Downloaded uuid v1.21.0
2026-Mar-16 01:45:32.410997
#51 8.678   Downloaded version_check v0.9.5
2026-Mar-16 01:45:32.410997
#51 8.680   Downloaded deno_core_icudata v0.0.73
2026-Mar-16 01:45:32.513651
#51 8.715   Downloaded regex-automata v0.4.14
2026-Mar-16 01:45:32.513651
#51 8.730   Downloaded hmac v0.12.1
2026-Mar-16 01:45:32.513651
#51 8.733   Downloaded getrandom v0.4.1
2026-Mar-16 01:45:32.513651
#51 8.737   Downloaded tokio-rustls v0.24.1
2026-Mar-16 01:45:32.513651
#51 8.740   Downloaded zerotrie v0.2.3
2026-Mar-16 01:45:32.513651
#51 8.744   Downloaded tinyvec v1.10.0
2026-Mar-16 01:45:32.513651
#51 8.747   Downloaded jsonwebtoken v9.3.1
2026-Mar-16 01:45:32.513651
#51 8.753   Downloaded yaml-rust2 v0.8.1
2026-Mar-16 01:45:32.513651
#51 8.791   Downloaded time-macros v0.2.27
2026-Mar-16 01:45:32.625589
#51 8.794   Downloaded thiserror-impl v2.0.18
2026-Mar-16 01:45:32.625589
#51 8.796   Downloaded sqlx-macros-core v0.8.6
2026-Mar-16 01:45:32.625589
#51 8.798   Downloaded spki v0.7.3
2026-Mar-16 01:45:32.625589
#51 8.801   Downloaded spinning_top v0.3.0
2026-Mar-16 01:45:32.625589
#51 8.803   Downloaded spin v0.9.8
2026-Mar-16 01:45:32.625589
#51 8.806   Downloaded sourcemap v8.0.1
2026-Mar-16 01:45:32.625589
#51 8.808   Downloaded socket2 v0.6.2
2026-Mar-16 01:45:32.625589
#51 8.810   Downloaded zerocopy v0.8.39
2026-Mar-16 01:45:32.625589
#51 8.840   Downloaded socket2 v0.5.10
2026-Mar-16 01:45:32.625589
#51 8.842   Downloaded smallvec v1.15.1
2026-Mar-16 01:45:32.625589
#51 8.844   Downloaded slab v0.4.12
2026-Mar-16 01:45:32.625589
#51 8.846   Downloaded simple_asn1 v0.6.4
2026-Mar-16 01:45:32.625589
#51 8.847   Downloaded simd-adler32 v0.3.8
2026-Mar-16 01:45:32.625589
#51 8.850   Downloaded signature v2.2.0
2026-Mar-16 01:45:32.625589
#51 8.852   Downloaded signal-hook-registry v1.4.8
2026-Mar-16 01:45:32.625589
#51 8.853   Downloaded sharded-slab v0.1.7
2026-Mar-16 01:45:32.625589
#51 8.857   Downloaded prettyplease v0.2.37
2026-Mar-16 01:45:32.625589
#51 8.862   Downloaded sqlx-macros v0.8.6
2026-Mar-16 01:45:32.625589
#51 8.863   Downloaded spki v0.6.0
2026-Mar-16 01:45:32.625589
#51 8.865   Downloaded simd-abstraction v0.7.1
2026-Mar-16 01:45:32.625589
#51 8.867   Downloaded signature v1.6.4
2026-Mar-16 01:45:32.625589
#51 8.869   Downloaded shlex v1.3.0
2026-Mar-16 01:45:32.625589
#51 8.870   Downloaded sha2 v0.10.9
2026-Mar-16 01:45:32.625589
#51 8.874   Downloaded sha1_smol v1.0.1
2026-Mar-16 01:45:32.625589
#51 8.876   Downloaded sha1 v0.10.6
2026-Mar-16 01:45:32.625589
#51 8.878   Downloaded pest_meta v2.8.6
2026-Mar-16 01:45:32.625589
#51 8.881   Downloaded pest v2.8.6
2026-Mar-16 01:45:32.625589
#51 8.887   Downloaded serde_spanned v0.6.9
2026-Mar-16 01:45:32.625589
#51 8.888   Downloaded serde_path_to_error v0.1.20
2026-Mar-16 01:45:32.625589
#51 8.890   Downloaded aws-sdk-s3 v1.119.0
2026-Mar-16 01:45:32.832179
#51 9.034   Downloaded find-msvc-tools v0.1.9
2026-Mar-16 01:45:32.832179
#51 9.036   Downloaded aws-sdk-sts v1.100.0
2026-Mar-16 01:45:32.832179
#51 9.054   Downloaded ppv-lite86 v0.2.21
2026-Mar-16 01:45:32.832179
#51 9.056   Downloaded powerfmt v0.2.0
2026-Mar-16 01:45:32.832179
#51 9.057   Downloaded pkcs8 v0.9.0
2026-Mar-16 01:45:32.832179
#51 9.062   Downloaded pin-project v1.1.10
2026-Mar-16 01:45:32.832179
#51 9.080   Downloaded pest_derive v2.8.6
2026-Mar-16 01:45:32.832179
#51 9.083   Downloaded futures-executor v0.3.32
2026-Mar-16 01:45:32.832179
#51 9.085   Downloaded fs_extra v1.3.0
2026-Mar-16 01:45:32.832179
#51 9.087   Downloaded either v1.15.0
2026-Mar-16 01:45:32.832179
#51 9.088   Downloaded deno_unsync v0.4.4
2026-Mar-16 01:45:32.832179
#51 9.091   Downloaded axum v0.8.8
2026-Mar-16 01:45:32.832179
#51 9.100   Downloaded aho-corasick v1.1.4
2026-Mar-16 01:45:32.832179
#51 9.106   Downloaded pkcs1 v0.7.5
2026-Mar-16 01:45:32.832179
#51 9.109   Downloaded foldhash v0.1.5
2026-Mar-16 01:45:32.936086
#51 9.111   Downloaded pest_generator v2.8.6
2026-Mar-16 01:45:32.936086
#51 9.113   Downloaded crc-fast v1.6.0
2026-Mar-16 01:45:32.936086
#51 9.122   Downloaded paste v1.0.15
2026-Mar-16 01:45:32.936086
#51 9.126   Downloaded parking_lot_core v0.9.12
2026-Mar-16 01:45:32.936086
#51 9.129   Downloaded parking_lot v0.12.5
2026-Mar-16 01:45:32.936086
#51 9.132   Downloaded miniz_oxide v0.8.9
2026-Mar-16 01:45:32.936086
#51 9.135   Downloaded minimal-lexical v0.2.1
2026-Mar-16 01:45:32.936086
#51 9.142   Downloaded memchr v2.8.0
2026-Mar-16 01:45:32.936086
#51 9.150   Downloaded der v0.7.10
2026-Mar-16 01:45:32.936086
#51 9.158   Downloaded der v0.6.1
2026-Mar-16 01:45:32.936086
#51 9.165   Downloaded combine v4.6.7
2026-Mar-16 01:45:32.936086
#51 9.175   Downloaded aws-smithy-types v1.4.6
2026-Mar-16 01:45:32.936086
#51 9.181   Downloaded aws-smithy-runtime v1.10.3
2026-Mar-16 01:45:32.936086
#51 9.190   Downloaded aws-sdk-ssooidc v1.98.0
2026-Mar-16 01:45:32.936086
#51 9.202   Downloaded async-compression v0.4.41
2026-Mar-16 01:45:32.936086
#51 9.213   Downloaded parking v2.2.1
2026-Mar-16 01:45:33.036161
#51 9.215   Downloaded outref v0.5.2
2026-Mar-16 01:45:33.036161
#51 9.217   Downloaded deno_ops v0.176.0
2026-Mar-16 01:45:33.036161
#51 9.229   Downloaded cc v1.2.56
2026-Mar-16 01:45:33.036161
#51 9.233   Downloaded aws-smithy-http-client v1.1.12
2026-Mar-16 01:45:33.036161
#51 9.240   Downloaded aws-sigv4 v1.4.2
2026-Mar-16 01:45:33.149889
#51 9.323   Downloaded crossbeam-channel v0.5.15
2026-Mar-16 01:45:33.149889
#51 9.328   Downloaded base64 v0.21.7
2026-Mar-16 01:45:33.149889
#51 9.333   Downloaded aws-runtime v1.7.2
2026-Mar-16 01:45:33.149889
#51 9.337   Downloaded ahash v0.8.12
2026-Mar-16 01:45:33.149889
#51 9.340   Downloaded bitflags v2.11.0
2026-Mar-16 01:45:33.149889
#51 9.346   Downloaded aws-credential-types v1.2.14
2026-Mar-16 01:45:33.149889
#51 9.348   Downloaded crypto-bigint v0.4.9
2026-Mar-16 01:45:33.149889
#51 9.353   Downloaded crossbeam-epoch v0.9.18
2026-Mar-16 01:45:33.149889
#51 9.356   Downloaded crc32fast v1.5.0
2026-Mar-16 01:45:33.149889
#51 9.359   Downloaded const-oid v0.9.6
2026-Mar-16 01:45:33.149889
#51 9.361   Downloaded aws-smithy-json v0.62.5
2026-Mar-16 01:45:33.149889
#51 9.363   Downloaded lock_api v0.4.14
2026-Mar-16 01:45:33.149889
#51 9.364   Downloaded allocator-api2 v0.2.21
2026-Mar-16 01:45:33.149889
#51 9.367   Downloaded aws-smithy-eventstream v0.60.20
2026-Mar-16 01:45:33.149889
#51 9.371   Downloaded libsqlite3-sys v0.30.1
2026-Mar-16 01:45:33.149889
#51 9.427   Downloaded zerovec v0.11.5
2026-Mar-16 01:45:33.313147
#51 9.590   Downloaded v8 v0.101.0
2026-Mar-16 01:45:33.764077
#51 ...
2026-Mar-16 01:45:33.764077
2026-Mar-16 01:45:33.764077
#52 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-16 01:45:33.764077
#52 0.252     Updating crates.io index
2026-Mar-16 01:45:33.764077
#52 5.695  Downloading crates ...
2026-Mar-16 01:45:33.764077
#52 6.242   Downloaded document-features v0.2.12
2026-Mar-16 01:45:33.764077
#52 6.275   Downloaded alloc-no-stdlib v2.0.4
2026-Mar-16 01:45:33.764077
#52 6.317   Downloaded digest v0.10.7
2026-Mar-16 01:45:33.764077
#52 6.373   Downloaded alloc-stdlib v0.2.2
2026-Mar-16 01:45:33.764077
#52 6.383   Downloaded adler2 v2.0.1
2026-Mar-16 01:45:33.764077
#52 6.386   Downloaded adler v1.0.2
2026-Mar-16 01:45:33.764077
#52 6.389   Downloaded allocator-api2 v0.2.21
2026-Mar-16 01:45:33.764077
#52 6.413   Downloaded deranged v0.5.8
2026-Mar-16 01:45:33.764077
#52 6.478   Downloaded bit-set v0.5.3
2026-Mar-16 01:45:33.764077
#52 6.484   Downloaded http-body v0.4.6
2026-Mar-16 01:45:33.764077
#52 6.489   Downloaded cpufeatures v0.2.17
2026-Mar-16 01:45:33.764077
#52 6.493   Downloaded aws-smithy-query v0.60.15
2026-Mar-16 01:45:33.764077
#52 6.496   Downloaded foreign-types-shared v0.1.1
2026-Mar-16 01:45:33.764077
#52 6.499   Downloaded crunchy v0.2.4
2026-Mar-16 01:45:33.764077
#52 6.502   Downloaded const-random v0.1.18
2026-Mar-16 01:45:33.764077
#52 6.520   Downloaded bit-vec v0.6.3
2026-Mar-16 01:45:33.764077
#52 6.534   Downloaded cooked-waker v5.0.0
2026-Mar-16 01:45:33.764077
#52 6.559   Downloaded hex v0.4.3
2026-Mar-16 01:45:33.764077
#52 6.563   Downloaded lazycell v1.3.0
2026-Mar-16 01:45:33.764077
#52 6.570   Downloaded foreign-types v0.3.2
2026-Mar-16 01:45:33.764077
#52 6.573   Downloaded itoa v1.0.17
2026-Mar-16 01:45:33.764077
#52 6.576   Downloaded form_urlencoded v1.2.2
2026-Mar-16 01:45:33.764077
#52 6.579   Downloaded futures-sink v0.3.32
2026-Mar-16 01:45:33.764077
#52 6.589   Downloaded generic-array v0.14.7
2026-Mar-16 01:45:33.764077
#52 6.593   Downloaded crc v3.4.0
2026-Mar-16 01:45:33.764077
#52 6.597   Downloaded compression-core v0.4.31
2026-Mar-16 01:45:33.764077
#52 6.610   Downloaded aws-smithy-observability v0.2.6
2026-Mar-16 01:45:33.764077
#52 6.615   Downloaded debugid v0.8.0
2026-Mar-16 01:45:33.764077
#52 6.629   Downloaded async-stream-impl v0.3.6
2026-Mar-16 01:45:33.764077
#52 6.647   Downloaded futures-io v0.3.32
2026-Mar-16 01:45:33.764077
#52 6.651   Downloaded httpdate v1.0.3
2026-Mar-16 01:45:33.764077
#52 6.655   Downloaded deno_unsync v0.4.4
2026-Mar-16 01:45:33.764077
#52 6.660   Downloaded fnv v1.0.7
2026-Mar-16 01:45:33.764077
#52 6.663   Downloaded hyper-tls v0.6.0
2026-Mar-16 01:45:33.764077
#52 6.671   Downloaded errno v0.3.14
2026-Mar-16 01:45:33.764077
#52 6.676   Downloaded crc-catalog v2.4.0
2026-Mar-16 01:45:33.764077
#52 6.687   Downloaded futures-core v0.3.32
2026-Mar-16 01:45:33.764077
#52 6.691   Downloaded crypto-common v0.1.7
2026-Mar-16 01:45:33.764077
#52 6.694   Downloaded block-buffer v0.10.4
2026-Mar-16 01:45:33.764077
#52 6.703   Downloaded cfg-if v1.0.4
2026-Mar-16 01:45:33.771665
#52 6.707   Downloaded atoi v2.0.0
2026-Mar-16 01:45:33.771665
#52 6.722   Downloaded async-stream v0.3.6
2026-Mar-16 01:45:33.771665
#52 6.730   Downloaded gzip-header v1.0.0
2026-Mar-16 01:45:33.771665
#52 6.732   Downloaded dunce v1.0.5
2026-Mar-16 01:45:33.771665
#52 6.736   Downloaded event-listener-strategy v0.5.4
2026-Mar-16 01:45:33.771665
#52 6.739   Downloaded if_chain v1.0.3
2026-Mar-16 01:45:33.771665
#52 6.743   Downloaded crossbeam-queue v0.3.12
2026-Mar-16 01:45:33.771665
#52 6.746   Downloaded futures-macro v0.3.32
2026-Mar-16 01:45:33.771665
#52 6.749   Downloaded idna_adapter v1.2.1
2026-Mar-16 01:45:33.771665
#52 6.752   Downloaded funty v2.0.0
2026-Mar-16 01:45:33.771665
#52 6.754   Downloaded cfg_aliases v0.2.1
2026-Mar-16 01:45:33.771665
#52 6.757   Downloaded equivalent v1.0.2
2026-Mar-16 01:45:33.771665
#52 6.761   Downloaded heck v0.4.1
2026-Mar-16 01:45:33.771665
#52 6.764   Downloaded json5 v0.4.1
2026-Mar-16 01:45:33.771665
#52 6.768   Downloaded ff v0.12.1
2026-Mar-16 01:45:33.771665
#52 6.797   Downloaded atomic-waker v1.1.2
2026-Mar-16 01:45:33.771665
#52 6.802   Downloaded const-random-macro v0.1.16
2026-Mar-16 01:45:33.771665
#52 6.804   Downloaded base64-simd v0.8.0
2026-Mar-16 01:45:33.771665
#52 6.807   Downloaded base16ct v0.1.1
2026-Mar-16 01:45:33.771665
#52 6.816   Downloaded home v0.5.12
2026-Mar-16 01:45:33.771665
#52 6.818   Downloaded base64-simd v0.7.0
2026-Mar-16 01:45:33.771665
#52 6.821   Downloaded futures-task v0.3.32
2026-Mar-16 01:45:33.771665
#52 6.827   Downloaded http-body v1.0.1
2026-Mar-16 01:45:33.771665
#52 6.831   Downloaded lazy_static v1.5.0
2026-Mar-16 01:45:33.771665
#52 6.834   Downloaded lru v0.12.5
2026-Mar-16 01:45:33.771665
#52 6.838   Downloaded ahash v0.8.12
2026-Mar-16 01:45:33.771665
#52 6.842   Downloaded dotenvy v0.15.7
2026-Mar-16 01:45:33.771665
#52 6.847   Downloaded glob v0.3.3
2026-Mar-16 01:45:33.771665
#52 6.850   Downloaded hyper-rustls v0.24.2
2026-Mar-16 01:45:33.771665
#52 6.854   Downloaded futures-timer v3.0.3
2026-Mar-16 01:45:33.771665
#52 6.858   Downloaded hashlink v0.10.0
2026-Mar-16 01:45:33.771665
#52 6.862   Downloaded aws-smithy-async v1.2.14
2026-Mar-16 01:45:33.771665
#52 6.904   Downloaded async-trait v0.1.89
2026-Mar-16 01:45:33.771665
#52 6.911   Downloaded memoffset v0.9.1
2026-Mar-16 01:45:33.771665
#52 6.915   Downloaded no-std-compat v0.4.1
2026-Mar-16 01:45:33.771665
#52 6.918   Downloaded aws-smithy-xml v0.60.15
2026-Mar-16 01:45:33.771665
#52 6.920   Downloaded libloading v0.8.9
2026-Mar-16 01:45:33.771665
#52 6.926   Downloaded heck v0.5.0
2026-Mar-16 01:45:33.771665
#52 6.930   Downloaded matchers v0.2.0
2026-Mar-16 01:45:33.771665
#52 6.933   Downloaded ecdsa v0.14.8
2026-Mar-16 01:45:33.771665
#52 6.936   Downloaded ipnet v2.11.0
2026-Mar-16 01:45:33.771665
#52 6.939   Downloaded md-5 v0.10.6
2026-Mar-16 01:45:33.771665
#52 6.942   Downloaded bincode v1.3.3
2026-Mar-16 01:45:33.771665
#52 6.945   Downloaded compression-codecs v0.4.37
2026-Mar-16 01:45:33.771665
#52 6.952   Downloaded foldhash v0.1.5
2026-Mar-16 01:45:33.771665
#52 6.954   Downloaded aws-smithy-eventstream v0.60.20
2026-Mar-16 01:45:33.771665
#52 6.958   Downloaded autocfg v1.5.0
2026-Mar-16 01:45:33.771665
#52 6.962   Downloaded http-body-util v0.1.3
2026-Mar-16 01:45:33.771665
#52 6.965   Downloaded group v0.12.1
2026-Mar-16 01:45:33.771665
#52 6.967   Downloaded fs_extra v1.3.0
2026-Mar-16 01:45:33.771665
#52 6.970   Downloaded convert_case v0.6.0
2026-Mar-16 01:45:33.771665
#52 6.972   Downloaded cmake v0.1.57
2026-Mar-16 01:45:33.771665
#52 6.975   Downloaded base64ct v1.8.3
2026-Mar-16 01:45:33.771665
#52 6.979   Downloaded arraydeque v0.5.1
2026-Mar-16 01:45:33.771665
#52 6.981   Downloaded find-msvc-tools v0.1.9
2026-Mar-16 01:45:33.771665
#52 6.984   Downloaded fastrand v2.3.0
2026-Mar-16 01:45:33.771665
#52 6.987   Downloaded dashmap v6.1.0
2026-Mar-16 01:45:33.771665
#52 6.991   Downloaded cexpr v0.6.0
2026-Mar-16 01:45:33.771665
#52 6.995   Downloaded fslock v0.2.1
2026-Mar-16 01:45:33.771665
#52 6.998   Downloaded nonzero_ext v0.3.0
2026-Mar-16 01:45:33.771665
#52 7.007   Downloaded displaydoc v0.2.5
2026-Mar-16 01:45:33.771665
#52 7.014   Downloaded num_cpus v1.17.0
2026-Mar-16 01:45:33.771665
#52 7.020   Downloaded openssl-probe v0.2.1
2026-Mar-16 01:45:33.771665
#52 7.023   Downloaded lock_api v0.4.14
2026-Mar-16 01:45:33.771665
#52 7.025   Downloaded jobserver v0.1.34
2026-Mar-16 01:45:33.771665
#52 7.029   Downloaded hyper-rustls v0.27.7
2026-Mar-16 01:45:33.771665
#52 7.034   Downloaded litemap v0.8.1
2026-Mar-16 01:45:33.771665
#52 7.037   Downloaded hmac v0.12.1
2026-Mar-16 01:45:33.771665
#52 7.041   Downloaded either v1.15.0
2026-Mar-16 01:45:33.771665
#52 7.044   Downloaded crypto-bigint v0.4.9
2026-Mar-16 01:45:33.771665
#52 7.053   Downloaded crc32fast v1.5.0
2026-Mar-16 01:45:33.771665
#52 7.056   Downloaded cookie_store v0.22.1
2026-Mar-16 01:45:33.771665
#52 7.061   Downloaded const-oid v0.9.6
2026-Mar-16 01:45:33.771665
#52 7.064   Downloaded aws-smithy-json v0.62.5
2026-Mar-16 01:45:33.771665
#52 7.068   Downloaded getrandom v0.2.17
2026-Mar-16 01:45:33.771665
#52 7.073   Downloaded async-lock v3.4.2
2026-Mar-16 01:45:33.771665
#52 7.077   Downloaded openssl-macros v0.1.1
2026-Mar-16 01:45:33.771665
#52 7.079   Downloaded httparse v1.10.1
2026-Mar-16 01:45:33.771665
#52 7.084   Downloaded futures-executor v0.3.32
2026-Mar-16 01:45:33.771665
#52 7.087   Downloaded bytes-utils v0.1.4
2026-Mar-16 01:45:33.771665
#52 7.091   Downloaded aws-smithy-http v0.63.6
2026-Mar-16 01:45:33.771665
#52 7.094   Downloaded aws-credential-types v1.2.14
2026-Mar-16 01:45:33.771665
#52 7.097   Downloaded hashlink v0.8.4
2026-Mar-16 01:45:33.771665
#52 7.100   Downloaded axum-core v0.5.6
2026-Mar-16 01:45:33.771665
#52 7.103   Downloaded aws-smithy-http v0.62.6
2026-Mar-16 01:45:33.771665
#52 7.107   Downloaded lru-slab v0.1.2
2026-Mar-16 01:45:33.771665
#52 7.109   Downloaded dlv-list v0.5.2
2026-Mar-16 01:45:33.771665
#52 7.112   Downloaded data-encoding v2.10.0
2026-Mar-16 01:45:33.771665
#52 7.145   Downloaded crossbeam-epoch v0.9.18
2026-Mar-16 01:45:33.771665
#52 7.152   Downloaded outref v0.5.2
2026-Mar-16 01:45:33.771665
#52 7.155   Downloaded parking v2.2.1
2026-Mar-16 01:45:33.771665
#52 7.158   Downloaded matchit v0.8.4
2026-Mar-16 01:45:33.771665
#52 7.162   Downloaded log v0.4.29
2026-Mar-16 01:45:33.771665
#52 7.167   Downloaded nu-ansi-term v0.50.3
2026-Mar-16 01:45:33.771665
#52 7.175   Downloaded percent-encoding v2.3.2
2026-Mar-16 01:45:33.771665
#52 7.177   Downloaded potential_utf v0.1.4
2026-Mar-16 01:45:33.771665
#52 7.180   Downloaded pin-project-internal v1.1.10
2026-Mar-16 01:45:33.771665
#52 7.183   Downloaded parking_lot_core v0.9.12
2026-Mar-16 01:45:33.771665
#52 7.187   Downloaded pin-project-lite v0.2.16
2026-Mar-16 01:45:33.771665
#52 7.197   Downloaded once_cell v1.21.3
2026-Mar-16 01:45:33.771665
#52 7.202   Downloaded icu_provider v2.1.1
2026-Mar-16 01:45:33.771665
#52 7.206   Downloaded icu_normalizer_data v2.1.1
2026-Mar-16 01:45:33.771665
#52 7.210   Downloaded flume v0.11.1
2026-Mar-16 01:45:33.771665
#52 7.216   Downloaded icu_properties v2.1.2
2026-Mar-16 01:45:33.771665
#52 7.219   Downloaded icu_locale_core v2.1.1
2026-Mar-16 01:45:33.771665
#52 7.229   Downloaded getrandom v0.4.1
2026-Mar-16 01:45:33.771665
#52 7.235   Downloaded getrandom v0.3.4
2026-Mar-16 01:45:33.771665
#52 7.240   Downloaded futures-channel v0.3.32
2026-Mar-16 01:45:33.771665
#52 7.243   Downloaded futures v0.3.32
2026-Mar-16 01:45:33.771665
#52 7.251   Downloaded event-listener v5.4.1
2026-Mar-16 01:45:33.771665
#52 7.255   Downloaded icu_normalizer v2.1.1
2026-Mar-16 01:45:33.771665
#52 7.261   Downloaded num-integer v0.1.46
2026-Mar-16 01:45:33.771665
#52 7.265   Downloaded bytes v1.11.1
2026-Mar-16 01:45:33.771665
#52 7.273   Downloaded psl-types v2.0.11
2026-Mar-16 01:45:33.771665
#52 7.275   Downloaded pkg-config v0.3.32
2026-Mar-16 01:45:33.771665
#52 7.279   Downloaded proc-macro-error-attr v1.0.4
2026-Mar-16 01:45:33.771665
#52 7.282   Downloaded proc-macro-rules-macros v0.4.0
2026-Mar-16 01:45:33.771665
#52 7.283   Downloaded pin-utils v0.1.0
2026-Mar-16 01:45:33.771665
#52 7.286   Downloaded pest_generator v2.8.6
2026-Mar-16 01:45:33.771665
#52 7.289   Downloaded pest_derive v2.8.6
2026-Mar-16 01:45:33.771665
#52 7.294   Downloaded pem v3.0.6
2026-Mar-16 01:45:33.771665
#52 7.296   Downloaded num-traits v0.2.19
2026-Mar-16 01:45:33.771665
#52 7.301   Downloaded miniz_oxide v0.7.4
2026-Mar-16 01:45:33.771665
#52 7.304   Downloaded futures-intrusive v0.5.0
2026-Mar-16 01:45:33.771665
#52 7.312   Downloaded powerfmt v0.2.0
2026-Mar-16 01:45:33.771665
#52 7.314   Downloaded aws-sdk-sso v1.96.0
2026-Mar-16 01:45:33.771665
#52 7.324   Downloaded elliptic-curve v0.12.3
2026-Mar-16 01:45:33.771665
#52 7.331   Downloaded der v0.7.10
2026-Mar-16 01:45:33.771665
#52 7.339   Downloaded der v0.6.1
2026-Mar-16 01:45:33.771665
#52 7.347   Downloaded pathdiff v0.2.3
2026-Mar-16 01:45:33.771665
#52 7.349   Downloaded paste v1.0.15
2026-Mar-16 01:45:33.771665
#52 7.355   Downloaded ordered-multimap v0.7.3
2026-Mar-16 01:45:33.771665
#52 7.359   Downloaded jsonwebtoken v9.3.1
2026-Mar-16 01:45:33.771665
#52 7.366   Downloaded native-tls v0.2.18
2026-Mar-16 01:45:33.771665
#52 7.369   Downloaded crossbeam-utils v0.8.21
2026-Mar-16 01:45:33.771665
#52 7.373   Downloaded config v0.14.1
2026-Mar-16 01:45:33.771665
#52 7.381   Downloaded bitflags v2.11.0
2026-Mar-16 01:45:33.771665
#52 7.389   Downloaded aws-smithy-runtime-api v1.11.6
2026-Mar-16 01:45:33.771665
#52 7.396   Downloaded aws-smithy-checksums v0.63.12
2026-Mar-16 01:45:33.771665
#52 7.399   Downloaded arc-swap v1.8.2
2026-Mar-16 01:45:33.771665
#52 7.405   Downloaded anyhow v1.0.102
2026-Mar-16 01:45:33.771665
#52 7.412   Downloaded outref v0.1.0
2026-Mar-16 01:45:33.771665
#52 7.415   Downloaded deno_ops v0.176.0
2026-Mar-16 01:45:33.771665
#52 7.430   Downloaded proc-macro-rules v0.4.0
2026-Mar-16 01:45:33.771665
#52 7.432   Downloaded pem-rfc7468 v0.7.0
2026-Mar-16 01:45:33.771665
#52 7.436   Downloaded pkcs8 v0.9.0
2026-Mar-16 01:45:33.771665
#52 7.441   Downloaded crossbeam-channel v0.5.15
2026-Mar-16 01:45:33.771665
#52 7.450   Downloaded proc-macro-error v1.0.4
2026-Mar-16 01:45:33.771665
#52 7.456   Downloaded rand_chacha v0.9.0
2026-Mar-16 01:45:33.771665
#52 7.458   Downloaded radium v0.7.0
2026-Mar-16 01:45:33.771665
#52 7.460   Downloaded quanta v0.12.6
2026-Mar-16 01:45:33.771665
#52 7.465   Downloaded combine v4.6.7
2026-Mar-16 01:45:33.771665
#52 7.473   Downloaded aws-smithy-runtime v1.10.3
2026-Mar-16 01:45:33.771665
#52 7.482   Downloaded pkcs8 v0.10.2
2026-Mar-16 01:45:33.771665
#52 7.488   Downloaded pkcs1 v0.7.5
2026-Mar-16 01:45:33.771665
#52 7.492   Downloaded minimal-lexical v0.2.1
2026-Mar-16 01:45:33.771665
#52 7.503   Downloaded rand_chacha v0.3.1
2026-Mar-16 01:45:33.771665
#52 7.508   Downloaded rustls-native-certs v0.8.3
2026-Mar-16 01:45:33.771665
#52 7.512   Downloaded scopeguard v1.2.0
2026-Mar-16 01:45:33.771665
#52 7.514   Downloaded ryu v1.0.23
2026-Mar-16 01:45:33.771665
#52 7.520   Downloaded rustversion v1.0.22
2026-Mar-16 01:45:33.771665
#52 7.527   Downloaded rustc_version v0.4.1
2026-Mar-16 01:45:33.771665
#52 7.530   Downloaded sha1_smol v1.0.1
2026-Mar-16 01:45:33.771665
#52 7.532   Downloaded sha1 v0.10.6
2026-Mar-16 01:45:33.771665
#52 7.535   Downloaded serde_urlencoded v0.7.1
2026-Mar-16 01:45:33.771665
#52 7.538   Downloaded rustls-pki-types v1.14.0
2026-Mar-16 01:45:33.771665
#52 7.542   Downloaded serde_spanned v0.6.9
2026-Mar-16 01:45:33.771665
#52 7.544   Downloaded num-bigint-dig v0.8.6
2026-Mar-16 01:45:33.771665
#52 7.551   Downloaded bitvec v1.0.1
2026-Mar-16 01:45:33.771665
#52 7.578   Downloaded serde_v8 v0.209.0
2026-Mar-16 01:45:33.771665
#52 7.581   Downloaded rust-ini v0.20.0
2026-Mar-16 01:45:33.771665
#52 7.584   Downloaded ron v0.8.1
2026-Mar-16 01:45:33.771665
#52 7.592   Downloaded serde_core v1.0.228
2026-Mar-16 01:45:33.771665
#52 7.596   Downloaded openssl-sys v0.9.111
2026-Mar-16 01:45:33.771665
#52 7.603   Downloaded deno_core v0.300.0
2026-Mar-16 01:45:33.771665
#52 7.618   Downloaded num-bigint v0.4.6
2026-Mar-16 01:45:33.771665
#52 7.625   Downloaded p256 v0.11.1
2026-Mar-16 01:45:33.771665
#52 7.631   Downloaded brotli-decompressor v5.0.0
2026-Mar-16 01:45:33.771665
#52 7.637   Downloaded signature v2.2.0
2026-Mar-16 01:45:33.771665
#52 7.639   Downloaded signature v1.6.4
2026-Mar-16 01:45:33.771665
#52 7.642   Downloaded stable_deref_trait v1.2.1
2026-Mar-16 01:45:33.771665
#52 7.645   Downloaded spki v0.6.0
2026-Mar-16 01:45:33.771665
#52 7.648   Downloaded bindgen v0.69.5
2026-Mar-16 01:45:33.771665
#52 7.655   Downloaded simd-abstraction v0.7.1
2026-Mar-16 01:45:33.771665
#52 7.657   Downloaded axum v0.8.8
2026-Mar-16 01:45:33.771665
#52 7.668   Downloaded sqlx-macros v0.8.6
2026-Mar-16 01:45:33.771665
#52 7.670   Downloaded slab v0.4.12
2026-Mar-16 01:45:33.771665
#52 7.672   Downloaded aws-sdk-sts v1.100.0
2026-Mar-16 01:45:33.771665
#52 7.692   Downloaded sqlx-macros-core v0.8.6
2026-Mar-16 01:45:33.771665
#52 7.695   Downloaded signal-hook-registry v1.4.8
2026-Mar-16 01:45:33.771665
#52 7.697   Downloaded pest v2.8.6
2026-Mar-16 01:45:33.771665
#52 7.705   Downloaded simple_asn1 v0.6.4
2026-Mar-16 01:45:33.771665
#52 7.707   Downloaded simd-adler32 v0.3.8
2026-Mar-16 01:45:33.771665
#52 7.710   Downloaded openssl v0.10.75
2026-Mar-16 01:45:33.771665
#52 7.725   Downloaded toml_datetime v0.6.11
2026-Mar-16 01:45:33.771665
#52 7.726   Downloaded semver-parser v0.7.0
2026-Mar-16 01:45:33.771665
#52 7.728   Downloaded semver v1.0.27
2026-Mar-16 01:45:33.771665
#52 7.732   Downloaded spki v0.7.3
2026-Mar-16 01:45:33.771665
#52 7.735   Downloaded pest_meta v2.8.6
2026-Mar-16 01:45:33.771665
#52 7.739   Downloaded smallvec v1.15.1
2026-Mar-16 01:45:33.771665
#52 7.741   Downloaded spin v0.9.8
2026-Mar-16 01:45:33.771665
#52 7.745   Downloaded sourcemap v8.0.1
2026-Mar-16 01:45:33.771665
#52 7.748   Downloaded spinning_top v0.3.0
2026-Mar-16 01:45:33.771665
#52 7.751   Downloaded semver v0.9.0
2026-Mar-16 01:45:33.771665
#52 7.753   Downloaded try-lock v0.2.5
2026-Mar-16 01:45:33.771665
#52 7.754   Downloaded tracing-log v0.2.0
2026-Mar-16 01:45:33.771665
#52 7.756   Downloaded prettyplease v0.2.37
2026-Mar-16 01:45:33.771665
#52 7.761   Downloaded socket2 v0.6.2
2026-Mar-16 01:45:33.771665
#52 7.763   Downloaded socket2 v0.5.10
2026-Mar-16 01:45:33.771665
#52 7.765   Downloaded sec1 v0.3.0
2026-Mar-16 01:45:33.771665
#52 7.767   Downloaded tokio-stream v0.1.18
2026-Mar-16 01:45:33.771665
#52 7.775   Downloaded sha2 v0.10.9
2026-Mar-16 01:45:33.771665
#52 7.778   Downloaded shlex v1.3.0
2026-Mar-16 01:45:33.771665
#52 7.780   Downloaded serde_path_to_error v0.1.20
2026-Mar-16 01:45:33.771665
#52 7.782   Downloaded tokio-rustls v0.26.4
2026-Mar-16 01:45:33.771665
#52 7.785   Downloaded tracing-core v0.1.36
2026-Mar-16 01:45:33.771665
#52 7.788   Downloaded serde_derive v1.0.228
2026-Mar-16 01:45:33.771665
#52 7.793   Downloaded sharded-slab v0.1.7
2026-Mar-16 01:45:33.771665
#52 7.797   Downloaded tokio-rustls v0.24.1
2026-Mar-16 01:45:33.771665
#52 7.800   Downloaded sct v0.7.1
2026-Mar-16 01:45:33.771665
#52 7.806   Downloaded moka v0.12.13
2026-Mar-16 01:45:33.771665
#52 7.815   Downloaded aws-lc-rs v1.16.1
2026-Mar-16 01:45:33.771665
#52 7.829   Downloaded aws-config v1.8.15
2026-Mar-16 01:45:33.771665
#52 7.837   Downloaded webpki-roots v0.26.11
2026-Mar-16 01:45:33.771665
#52 7.839   Downloaded want v0.3.1
2026-Mar-16 01:45:33.771665
#52 7.840   Downloaded version_check v0.9.5
2026-Mar-16 01:45:33.771665
#52 7.842   Downloaded utf8_iter v1.0.4
2026-Mar-16 01:45:33.771665
#52 7.843   Downloaded urlencoding v2.1.3
2026-Mar-16 01:45:33.771665
#52 7.845   Downloaded untrusted v0.9.0
2026-Mar-16 01:45:33.771665
#52 7.847   Downloaded aho-corasick v1.1.4
2026-Mar-16 01:45:33.771665
#52 7.853   Downloaded rfc6979 v0.3.1
2026-Mar-16 01:45:33.771665
#52 7.855   Downloaded rand_core v0.6.4
2026-Mar-16 01:45:33.771665
#52 7.856   Downloaded libm v0.2.16
2026-Mar-16 01:45:33.771665
#52 7.871   Downloaded sync_wrapper v1.0.2
2026-Mar-16 01:45:33.771665
#52 7.872   Downloaded rand v0.8.5
2026-Mar-16 01:45:33.771665
#52 7.877   Downloaded time-core v0.1.8
2026-Mar-16 01:45:33.771665
#52 7.878   Downloaded tinyvec_macros v0.1.1
2026-Mar-16 01:45:33.771665
#52 7.879   Downloaded tap v1.0.1
2026-Mar-16 01:45:33.771665
#52 7.881   Downloaded quinn v0.11.9
2026-Mar-16 01:45:33.771665
#52 7.885   Downloaded web-time v1.1.0
2026-Mar-16 01:45:33.771665
#52 7.888   Downloaded vsimd v0.8.0
2026-Mar-16 01:45:33.771665
#52 7.891   Downloaded unicode-id-start v1.4.0
2026-Mar-16 01:45:33.771665
#52 7.895   Downloaded thread_local v1.1.9
2026-Mar-16 01:45:33.771665
#52 7.898   Downloaded publicsuffix v2.3.0
2026-Mar-16 01:45:33.771665
#52 7.902   Downloaded thiserror-impl v2.0.18
2026-Mar-16 01:45:33.771665
#52 7.905   Downloaded stringprep v0.1.5
2026-Mar-16 01:45:33.771665
#52 7.907   Downloaded ucd-trie v0.1.7
2026-Mar-16 01:45:33.771665
#52 7.908   Downloaded static_assertions v1.1.0
2026-Mar-16 01:45:33.771665
#52 7.910   Downloaded synstructure v0.13.2
2026-Mar-16 01:45:33.771665
#52 7.912   Downloaded thiserror-impl v1.0.69
2026-Mar-16 01:45:33.771665
#52 7.913   Downloaded thiserror v2.0.18
2026-Mar-16 01:45:33.771665
#52 7.923   Downloaded utoipa v4.2.3
2026-Mar-16 01:45:33.771665
#52 7.926   Downloaded tokio-native-tls v0.3.1
2026-Mar-16 01:45:33.771665
#52 7.929   Downloaded tinystr v0.8.2
2026-Mar-16 01:45:33.771665
#52 7.931   Downloaded unicode-ident v1.0.24
2026-Mar-16 01:45:33.771665
#52 7.935   Downloaded tiny-keccak v2.0.2
2026-Mar-16 01:45:33.771665
#52 7.938   Downloaded time-macros v0.2.27
2026-Mar-16 01:45:33.771665
#52 7.942   Downloaded toml_write v0.1.2
2026-Mar-16 01:45:33.771665
#52 7.943   Downloaded strum_macros v0.25.3
2026-Mar-16 01:45:33.771665
#52 7.946   Downloaded unicode-bidi v0.3.18
2026-Mar-16 01:45:33.771665
#52 7.949   Downloaded uuid v1.21.0
2026-Mar-16 01:45:33.771665
#52 7.953   Downloaded tower-layer v0.3.3
2026-Mar-16 01:45:33.771665
#52 7.955   Downloaded thiserror v1.0.69
2026-Mar-16 01:45:33.771665
#52 7.963   Downloaded zerofrom v0.1.6
2026-Mar-16 01:45:33.771665
#52 7.964   Downloaded tagptr v0.2.0
2026-Mar-16 01:45:33.771665
#52 7.966   Downloaded unicode-properties v0.1.4
2026-Mar-16 01:45:33.771665
#52 7.968   Downloaded zerofrom-derive v0.1.6
2026-Mar-16 01:45:33.771665
#52 7.970   Downloaded yoke-derive v0.8.1
2026-Mar-16 01:45:33.771665
#52 7.971   Downloaded subtle v2.6.1
2026-Mar-16 01:45:33.771665
#52 7.973   Downloaded which v4.4.2
2026-Mar-16 01:45:33.771665
#52 7.975   Downloaded strum v0.25.0
2026-Mar-16 01:45:33.771665
#52 7.976   Downloaded portable-atomic v1.13.1
2026-Mar-16 01:45:33.771665
#52 7.987   Downloaded tower-service v0.3.3
2026-Mar-16 01:45:33.771665
#52 7.988   Downloaded tokio-macros v2.6.0
2026-Mar-16 01:45:33.771665
#52 7.990   Downloaded itertools v0.13.0
2026-Mar-16 01:45:33.771665
#52 8.000   Downloaded icu_properties_data v2.1.2
2026-Mar-16 01:45:33.771665
#52 8.016   Downloaded zerovec-derive v0.11.2
2026-Mar-16 01:45:33.771665
#52 8.019   Downloaded zmij v1.0.21
2026-Mar-16 01:45:33.771665
#52 8.022   Downloaded xmlparser v0.13.6
2026-Mar-16 01:45:33.771665
#52 8.025   Downloaded raw-cpuid v11.6.0
2026-Mar-16 01:45:33.771665
#52 8.030   Downloaded regex-lite v0.1.9
2026-Mar-16 01:45:33.771665
#52 8.033   Downloaded serde v1.0.228
2026-Mar-16 01:45:33.771665
#52 8.038   Downloaded rustls-webpki v0.103.9
2026-Mar-16 01:45:33.771665
#52 8.042   Downloaded rsa v0.9.10
2026-Mar-16 01:45:33.771665
#52 8.048   Downloaded tinyvec v1.10.0
2026-Mar-16 01:45:33.771665
#52 8.052   Downloaded zeroize v1.8.2
2026-Mar-16 01:45:33.771665
#52 8.055   Downloaded brotli v8.0.2
2026-Mar-16 01:45:33.771665
#52 8.073   Downloaded wyz v0.5.1
2026-Mar-16 01:45:33.771665
#52 8.075   Downloaded rand v0.9.2
2026-Mar-16 01:45:33.771665
#52 8.080   Downloaded toml v0.8.23
2026-Mar-16 01:45:33.771665
#52 8.083   Downloaded writeable v0.6.2
2026-Mar-16 01:45:33.771665
#52 8.085   Downloaded tracing-attributes v0.1.31
2026-Mar-16 01:45:33.771665
#52 8.088   Downloaded regex v1.12.3
2026-Mar-16 01:45:33.771665
#52 8.095   Downloaded yoke v0.8.1
2026-Mar-16 01:45:33.771665
#52 8.097   Downloaded libc v0.2.182
2026-Mar-16 01:45:33.771665
#52 8.146   Downloaded rustls-webpki v0.101.7
2026-Mar-16 01:45:33.771665
#52 8.169   Downloaded sqlx-sqlite v0.8.6
2026-Mar-16 01:45:33.771665
#52 8.175   Downloaded sqlx-core v0.8.6
2026-Mar-16 01:45:33.771665
#52 8.185   Downloaded redis v0.27.6
2026-Mar-16 01:45:33.771665
#52 8.194   Downloaded sqlx-postgres v0.8.6
2026-Mar-16 01:45:33.771665
#52 8.205   Downloaded tower v0.5.3
2026-Mar-16 01:45:33.771665
#52 8.218   Downloaded sqlx-mysql v0.8.6
2026-Mar-16 01:45:33.771665
#52 8.226   Downloaded serde_json v1.0.149
2026-Mar-16 01:45:33.771665
#52 8.235   Downloaded reqwest v0.12.28
2026-Mar-16 01:45:33.771665
#52 8.241   Downloaded tower-http v0.6.8
2026-Mar-16 01:45:33.771665
#52 8.253   Downloaded rustls v0.21.12
2026-Mar-16 01:45:33.771665
#52 8.267   Downloaded typenum v1.19.0
2026-Mar-16 01:45:33.771665
#52 8.270   Downloaded rustls v0.23.37
2026-Mar-16 01:45:33.771665
#52 8.285   Downloaded regex-syntax v0.8.10
2026-Mar-16 01:45:33.771665
#52 8.292   Downloaded rustix v0.38.44
2026-Mar-16 01:45:33.771665
#52 8.331   Downloaded url v2.5.8
2026-Mar-16 01:45:33.771665
#52 8.334   Downloaded sqlx v0.8.6
2026-Mar-16 01:45:33.771665
#52 8.353   Downloaded unicode-segmentation v1.12.0
2026-Mar-16 01:45:33.771665
#52 8.356   Downloaded unicode-normalization v0.1.25
2026-Mar-16 01:45:33.771665
#52 8.360   Downloaded encoding_rs v0.8.35
2026-Mar-16 01:45:33.771665
#52 8.380   Downloaded utoipa-gen v4.3.1
2026-Mar-16 01:45:33.771665
#52 8.387   Downloaded quinn-proto v0.11.13
2026-Mar-16 01:45:33.771665
#52 8.394   Downloaded whoami v1.6.1
2026-Mar-16 01:45:33.771665
#52 8.397   Downloaded which v6.0.3
2026-Mar-16 01:45:33.771665
#52 8.399   Downloaded hyper v1.8.1
2026-Mar-16 01:45:33.771665
#52 8.407   Downloaded hyper v0.14.32
2026-Mar-16 01:45:33.771665
#52 8.417   Downloaded hkdf v0.12.4
2026-Mar-16 01:45:33.771665
#52 8.419   Downloaded hashbrown v0.16.1
2026-Mar-16 01:45:33.771665
#52 8.425   Downloaded hashbrown v0.15.5
2026-Mar-16 01:45:33.771665
#52 8.431   Downloaded h2 v0.4.13
2026-Mar-16 01:45:33.771665
#52 8.439   Downloaded tracing-subscriber v0.3.22
2026-Mar-16 01:45:33.771665
#52 8.449   Downloaded vcpkg v0.2.15
2026-Mar-16 01:45:33.771665
#52 8.527   Downloaded regex-automata v0.4.14
2026-Mar-16 01:45:33.771665
#52 8.546   Downloaded h2 v0.3.27
2026-Mar-16 01:45:33.771665
#52 8.554   Downloaded governor v0.8.1
2026-Mar-16 01:45:33.771665
#52 8.561   Downloaded zerotrie v0.2.3
2026-Mar-16 01:45:33.771665
#52 8.566   Downloaded toml_edit v0.22.27
2026-Mar-16 01:45:33.771665
#52 8.572   Downloaded futures-util v0.3.32
2026-Mar-16 01:45:33.771665
#52 8.590   Downloaded crc-fast v1.6.0
2026-Mar-16 01:45:33.771665
#52 8.602   Downloaded tower v0.4.13
2026-Mar-16 01:45:33.771665
#52 8.619   Downloaded rustc_version v0.2.3
2026-Mar-16 01:45:33.771665
#52 8.620   Downloaded time v0.3.47
2026-Mar-16 01:45:33.771665
#52 8.638   Downloaded zerovec v0.11.5
2026-Mar-16 01:45:33.771665
#52 8.645   Downloaded syn v1.0.109
2026-Mar-16 01:45:33.771665
#52 8.656   Downloaded yaml-rust2 v0.8.1
2026-Mar-16 01:45:33.771665
#52 8.694   Downloaded winnow v0.7.14
2026-Mar-16 01:45:33.771665
#52 8.707   Downloaded tokio-util v0.7.18
2026-Mar-16 01:45:33.771665
#52 8.717   Downloaded syn v2.0.117
2026-Mar-16 01:45:33.771665
#52 8.729   Downloaded rustc-hash v2.1.1
2026-Mar-16 01:45:33.771665
#52 8.731   Downloaded webpki-roots v1.0.6
2026-Mar-16 01:45:33.771665
#52 8.734   Downloaded zerocopy v0.8.39
2026-Mar-16 01:45:33.771665
#52 8.765   Downloaded rustc-hash v1.1.0
2026-Mar-16 01:45:33.771665
#52 8.766   Downloaded rand_core v0.9.5
2026-Mar-16 01:45:33.771665
#52 8.768   Downloaded quote v1.0.44
2026-Mar-16 01:45:33.771665
#52 8.771   Downloaded aws-sdk-s3 v1.119.0
2026-Mar-16 01:45:33.771665
#52 8.897   Downloaded quinn-udp v0.5.14
2026-Mar-16 01:45:33.771665
#52 8.900   Downloaded proc-macro2 v1.0.106
2026-Mar-16 01:45:33.771665
#52 8.903   Downloaded pin-project v1.1.10
2026-Mar-16 01:45:33.771665
#52 8.922   Downloaded nom v7.1.3
2026-Mar-16 01:45:33.771665
#52 8.928   Downloaded mio v1.1.1
2026-Mar-16 01:45:33.771665
#52 8.936   Downloaded tracing v0.1.44
2026-Mar-16 01:45:33.771665
#52 8.953   Downloaded itertools v0.12.1
2026-Mar-16 01:45:33.771665
#52 8.962   Downloaded iri-string v0.7.10
2026-Mar-16 01:45:33.771665
#52 8.971   Downloaded indexmap v2.13.0
2026-Mar-16 01:45:33.771665
#52 8.976   Downloaded idna v1.1.0
2026-Mar-16 01:45:33.771665
#52 8.981   Downloaded hyper-util v0.1.20
2026-Mar-16 01:45:33.771665
#52 8.988   Downloaded http v1.4.0
2026-Mar-16 01:45:33.771665
#52 8.993   Downloaded http v0.2.12
2026-Mar-16 01:45:33.771665
#52 8.997   Downloaded hashbrown v0.14.5
2026-Mar-16 01:45:33.771665
#52 9.003   Downloaded flate2 v1.1.9
2026-Mar-16 01:45:33.771665
#52 9.009   Downloaded ppv-lite86 v0.2.21
2026-Mar-16 01:45:33.771665
#52 9.011   Downloaded memchr v2.8.0
2026-Mar-16 01:45:33.771665
#52 9.017   Downloaded icu_collections v2.1.1
2026-Mar-16 01:45:33.771665
#52 9.027   Downloaded cc v1.2.56
2026-Mar-16 01:45:33.771665
#52 9.031   Downloaded aws-sdk-ssooidc v1.98.0
2026-Mar-16 01:45:33.771665
#52 9.042   Downloaded async-compression v0.4.41
2026-Mar-16 01:45:33.771665
#52 9.053   Downloaded aws-smithy-types v1.4.6
2026-Mar-16 01:45:33.771665
#52 9.060   Downloaded tokio v1.49.0
2026-Mar-16 01:45:33.771665
#52 9.116   Downloaded miniz_oxide v0.8.9
2026-Mar-16 01:45:33.771665
#52 9.119   Downloaded aws-sigv4 v1.4.2
2026-Mar-16 01:45:33.771665
#52 9.190   Downloaded ring v0.17.14
2026-Mar-16 01:45:33.771665
#52 9.245   Downloaded parking_lot v0.12.5
2026-Mar-16 01:45:33.771665
#52 9.249   Downloaded crypto-bigint v0.5.5
2026-Mar-16 01:45:33.771665
#52 9.258   Downloaded base64 v0.21.7
2026-Mar-16 01:45:33.771665
#52 9.263   Downloaded aws-smithy-http-client v1.1.12
2026-Mar-16 01:45:33.771665
#52 9.268   Downloaded aws-runtime v1.7.2
2026-Mar-16 01:45:33.771665
#52 9.273   Downloaded linux-raw-sys v0.4.15
2026-Mar-16 01:45:33.771665
#52 9.326   Downloaded base64 v0.22.1
2026-Mar-16 01:45:33.771665
#52 9.331   Downloaded num-iter v0.1.45
2026-Mar-16 01:45:33.771665
#52 9.333   Downloaded num-conv v0.2.0
2026-Mar-16 01:45:33.771665
#52 9.334   Downloaded mime v0.3.17
2026-Mar-16 01:45:33.771665
#52 9.337   Downloaded litrs v1.0.0
2026-Mar-16 01:45:33.771665
#52 9.342   Downloaded clang-sys v1.8.1
2026-Mar-16 01:45:33.771665
#52 9.345   Downloaded byteorder v1.5.0
2026-Mar-16 01:45:33.771665
#52 9.348   Downloaded aws-smithy-json v0.61.9
2026-Mar-16 01:45:33.771665
#52 9.350   Downloaded cookie v0.18.1
2026-Mar-16 01:45:33.771665
#52 9.353   Downloaded concurrent-queue v2.5.0
2026-Mar-16 01:45:33.771665
#52 9.356   Downloaded aws-types v1.3.14
2026-Mar-16 01:45:33.771665
#52 9.365   Downloaded libsqlite3-sys v0.30.1
2026-Mar-16 01:45:33.771665
#52 9.544   Downloaded deno_core_icudata v0.0.73
2026-Mar-16 01:45:33.937725
#52 10.23   Downloaded aws-lc-sys v0.38.0
2026-Mar-16 01:45:34.486044
#52 ...
2026-Mar-16 01:45:34.486044
2026-Mar-16 01:45:34.486044
#51 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-16 01:45:34.486044
#51 10.76    Compiling proc-macro2 v1.0.106
2026-Mar-16 01:45:34.486044
#51 10.76    Compiling unicode-ident v1.0.24
2026-Mar-16 01:45:34.588685
#51 10.76    Compiling quote v1.0.44
2026-Mar-16 01:45:34.588685
#51 10.76    Compiling libc v0.2.182
2026-Mar-16 01:45:34.588685
#51 10.77    Compiling cfg-if v1.0.4
2026-Mar-16 01:45:34.588685
#51 10.78    Compiling serde v1.0.228
2026-Mar-16 01:45:34.588685
#51 10.79    Compiling serde_core v1.0.228
2026-Mar-16 01:45:34.588685
#51 10.79    Compiling version_check v0.9.5
2026-Mar-16 01:45:34.588685
#51 10.80    Compiling pin-project-lite v0.2.16
2026-Mar-16 01:45:34.588685
#51 10.81    Compiling parking_lot_core v0.9.12
2026-Mar-16 01:45:34.588685
#51 10.82    Compiling shlex v1.3.0
2026-Mar-16 01:45:34.588685
#51 10.83    Compiling memchr v2.8.0
2026-Mar-16 01:45:34.588685
#51 10.83    Compiling bytes v1.11.1
2026-Mar-16 01:45:34.588685
#51 10.83    Compiling once_cell v1.21.3
2026-Mar-16 01:45:34.588685
#51 10.85    Compiling scopeguard v1.2.0
2026-Mar-16 01:45:34.588685
#51 10.86    Compiling itoa v1.0.17
2026-Mar-16 01:45:34.588685
#51 10.86    Compiling futures-core v0.3.32
2026-Mar-16 01:45:34.588685
#51 10.87    Compiling find-msvc-tools v0.1.9
2026-Mar-16 01:45:34.701913
#51 10.87    Compiling futures-sink v0.3.32
2026-Mar-16 01:45:34.701913
#51 10.88    Compiling autocfg v1.5.0
2026-Mar-16 01:45:34.701913
#51 10.88    Compiling typenum v1.19.0
2026-Mar-16 01:45:34.701913
#51 10.88    Compiling log v0.4.29
2026-Mar-16 01:45:34.701913
#51 10.89    Compiling slab v0.4.12
2026-Mar-16 01:45:34.701913
#51 10.89    Compiling futures-io v0.3.32
2026-Mar-16 01:45:34.701913
#51 10.89    Compiling futures-task v0.3.32
2026-Mar-16 01:45:34.701913
#51 10.89    Compiling zeroize v1.8.2
2026-Mar-16 01:45:34.701913
#51 10.89    Compiling zerocopy v0.8.39
2026-Mar-16 01:45:34.701913
#51 10.89    Compiling equivalent v1.0.2
2026-Mar-16 01:45:34.701913
#51 10.89    Compiling subtle v2.6.1
2026-Mar-16 01:45:34.701913
#51 10.89    Compiling fnv v1.0.7
2026-Mar-16 01:45:34.701913
#51 10.89    Compiling hashbrown v0.16.1
2026-Mar-16 01:45:34.701913
#51 10.89    Compiling percent-encoding v2.3.2
2026-Mar-16 01:45:34.701913
#51 10.89    Compiling time-core v0.1.8
2026-Mar-16 01:45:34.701913
#51 10.89    Compiling icu_normalizer_data v2.1.1
2026-Mar-16 01:45:34.701913
#51 10.96    Compiling lock_api v0.4.14
2026-Mar-16 01:45:34.701913
#51 10.98    Compiling icu_properties_data v2.1.2
2026-Mar-16 01:45:34.815818
#51 11.09    Compiling generic-array v0.14.7
2026-Mar-16 01:45:34.956954
#51 11.13    Compiling tracing-core v0.1.36
2026-Mar-16 01:45:34.956954
#51 11.15    Compiling futures-channel v0.3.32
2026-Mar-16 01:45:34.956954
#51 11.15    Compiling num-conv v0.2.0
2026-Mar-16 01:45:34.956954
#51 11.15    Compiling powerfmt v0.2.0
2026-Mar-16 01:45:34.956954
#51 11.15    Compiling pin-utils v0.1.0
2026-Mar-16 01:45:34.956954
#51 11.15    Compiling ryu v1.0.23
2026-Mar-16 01:45:34.956954
#51 11.15    Compiling stable_deref_trait v1.2.1
2026-Mar-16 01:45:34.956954
#51 11.23    Compiling num-traits v0.2.19
2026-Mar-16 01:45:35.059376
#51 11.23    Compiling crc32fast v1.5.0
2026-Mar-16 01:45:35.059376
#51 11.24    Compiling untrusted v0.9.0
2026-Mar-16 01:45:35.059376
#51 11.24    Compiling fs_extra v1.3.0
2026-Mar-16 01:45:35.059376
#51 11.24    Compiling dunce v1.0.5
2026-Mar-16 01:45:35.059376
#51 11.24    Compiling zmij v1.0.21
2026-Mar-16 01:45:35.059376
#51 11.24    Compiling form_urlencoded v1.2.2
2026-Mar-16 01:45:35.059376
#51 11.25    Compiling rustls-pki-types v1.14.0
2026-Mar-16 01:45:35.059376
#51 11.34    Compiling http v1.4.0
2026-Mar-16 01:45:35.159491
#51 11.34    Compiling time-macros v0.2.27
2026-Mar-16 01:45:35.159491
#51 11.39    Compiling http v0.2.12
2026-Mar-16 01:45:35.159491
#51 11.40    Compiling aws-lc-rs v1.16.1
2026-Mar-16 01:45:35.159491
#51 11.43    Compiling deranged v0.5.8
2026-Mar-16 01:45:35.159491
#51 11.44    Compiling tower-service v0.3.3
2026-Mar-16 01:45:35.278205
#51 11.45    Compiling litemap v0.8.1
2026-Mar-16 01:45:35.278205
#51 11.46    Compiling writeable v0.6.2
2026-Mar-16 01:45:35.278205
#51 11.47    Compiling base64 v0.22.1
2026-Mar-16 01:45:35.278205
#51 11.48    Compiling vsimd v0.8.0
2026-Mar-16 01:45:35.278205
#51 11.49    Compiling outref v0.5.2
2026-Mar-16 01:45:35.278205
#51 11.51    Compiling httparse v1.10.1
2026-Mar-16 01:45:35.278205
#51 11.55    Compiling try-lock v0.2.5
2026-Mar-16 01:45:35.384072
#51 11.60    Compiling rustls v0.23.37
2026-Mar-16 01:45:35.384072
#51 11.61    Compiling httpdate v1.0.3
2026-Mar-16 01:45:35.384072
#51 11.64    Compiling allocator-api2 v0.2.21
2026-Mar-16 01:45:35.384072
#51 11.65    Compiling crossbeam-utils v0.8.21
2026-Mar-16 01:45:35.384072
#51 11.66    Compiling atomic-waker v1.1.2
2026-Mar-16 01:45:35.520589
#51 11.67    Compiling want v0.3.1
2026-Mar-16 01:45:35.520589
#51 11.69    Compiling webpki-roots v1.0.6
2026-Mar-16 01:45:35.520589
#51 11.70    Compiling openssl-probe v0.2.1
2026-Mar-16 01:45:35.520589
#51 11.72    Compiling tower-layer v0.3.3
2026-Mar-16 01:45:35.520589
#51 11.74    Compiling cpufeatures v0.2.17
2026-Mar-16 01:45:35.520589
#51 11.74    Compiling utf8_iter v1.0.4
2026-Mar-16 01:45:35.520589
#51 11.80    Compiling sync_wrapper v1.0.2
2026-Mar-16 01:45:35.655266
#51 11.84    Compiling ipnet v2.11.0
2026-Mar-16 01:45:35.655266
#51 11.84    Compiling serde_json v1.0.149
2026-Mar-16 01:45:35.655266
#51 11.87    Compiling thiserror v2.0.18
2026-Mar-16 01:45:35.655266
#51 11.89    Compiling bitflags v2.11.0
2026-Mar-16 01:45:35.655266
#51 11.93    Compiling base64-simd v0.8.0
2026-Mar-16 01:45:35.820357
#51 11.94    Compiling rustversion v1.0.22
2026-Mar-16 01:45:35.820357
#51 11.96    Compiling getrandom v0.4.1
2026-Mar-16 01:45:35.820357
#51 11.97    Compiling home v0.5.12
2026-Mar-16 01:45:35.820357
#51 11.98    Compiling rustls-native-certs v0.8.3
2026-Mar-16 01:45:35.820357
#51 11.99    Compiling hex v0.4.3
2026-Mar-16 01:45:35.820357
#51 12.03    Compiling const-oid v0.9.6
2026-Mar-16 01:45:35.820357
#51 12.10    Compiling syn v2.0.117
2026-Mar-16 01:45:35.975902
#51 12.10    Compiling errno v0.3.14
2026-Mar-16 01:45:35.975902
#51 12.13    Compiling socket2 v0.6.2
2026-Mar-16 01:45:35.975902
#51 12.13    Compiling mio v1.1.1
2026-Mar-16 01:45:35.975902
#51 12.13    Compiling getrandom v0.2.17
2026-Mar-16 01:45:35.975902
#51 12.13    Compiling http-body v1.0.1
2026-Mar-16 01:45:35.975902
#51 12.16    Compiling http-body v0.4.6
2026-Mar-16 01:45:35.975902
#51 12.17    Compiling socket2 v0.5.10
2026-Mar-16 01:45:35.975902
#51 12.19    Compiling der v0.6.1
2026-Mar-16 01:45:35.975902
#51 12.25    Compiling jobserver v0.1.34
2026-Mar-16 01:45:36.077510
#51 12.25    Compiling signal-hook-registry v1.4.8
2026-Mar-16 01:45:36.077510
#51 12.29    Compiling num-integer v0.1.46
2026-Mar-16 01:45:36.077510
#51 12.29    Compiling http-body-util v0.1.3
2026-Mar-16 01:45:36.077510
#51 12.33    Compiling pkg-config v0.3.32
2026-Mar-16 01:45:36.077510
#51 12.35    Compiling rand_core v0.6.4
2026-Mar-16 01:45:36.077510
#51 12.35    Compiling vcpkg v0.2.15
2026-Mar-16 01:45:36.193080
#51 12.37    Compiling base64ct v1.8.3
2026-Mar-16 01:45:36.193080
#51 12.47    Compiling cc v1.2.56
2026-Mar-16 01:45:36.193080
#51 12.47    Compiling crypto-common v0.1.7
2026-Mar-16 01:45:36.311201
#51 12.48    Compiling block-buffer v0.10.4
2026-Mar-16 01:45:36.311201
#51 12.51    Compiling uuid v1.21.0
2026-Mar-16 01:45:36.311201
#51 12.54    Compiling ahash v0.8.12
2026-Mar-16 01:45:36.311201
#51 12.59    Compiling rustls v0.21.12
2026-Mar-16 01:45:36.414849
#51 12.61    Compiling digest v0.10.7
2026-Mar-16 01:45:36.414849
#51 12.61    Compiling crypto-bigint v0.4.9
2026-Mar-16 01:45:36.414849
#51 12.62    Compiling ff v0.12.1
2026-Mar-16 01:45:36.414849
#51 12.62    Compiling aho-corasick v1.1.4
2026-Mar-16 01:45:36.414849
#51 12.69    Compiling time v0.3.47
2026-Mar-16 01:45:36.523613
#51 12.69    Compiling regex-syntax v0.8.10
2026-Mar-16 01:45:36.523613
#51 12.70    Compiling crc-catalog v2.4.0
2026-Mar-16 01:45:36.523613
#51 12.71    Compiling rustix v0.38.44
2026-Mar-16 01:45:36.523613
#51 12.73    Compiling foldhash v0.1.5
2026-Mar-16 01:45:36.523613
#51 12.73    Compiling base16ct v0.1.1
2026-Mar-16 01:45:36.523613
#51 12.75    Compiling glob v0.3.3
2026-Mar-16 01:45:36.523613
#51 12.76    Compiling concurrent-queue v2.5.0
2026-Mar-16 01:45:36.523613
#51 12.77    Compiling group v0.12.1
2026-Mar-16 01:45:36.523613
#51 12.79    Compiling hmac v0.12.1
2026-Mar-16 01:45:36.630980
#51 12.81    Compiling sha2 v0.10.9
2026-Mar-16 01:45:36.630980
#51 12.87    Compiling crc v3.4.0
2026-Mar-16 01:45:36.630980
#51 12.89    Compiling prettyplease v0.2.37
2026-Mar-16 01:45:36.630980
#51 12.91    Compiling hashbrown v0.15.5
2026-Mar-16 01:45:36.734577
#51 12.91    Compiling alloc-no-stdlib v2.0.4
2026-Mar-16 01:45:36.734577
#51 12.94    Compiling spki v0.6.0
2026-Mar-16 01:45:36.734577
#51 13.00    Compiling parking v2.2.1
2026-Mar-16 01:45:36.844194
#51 13.06    Compiling tinyvec_macros v0.1.1
2026-Mar-16 01:45:36.844194
#51 13.09    Compiling semver v1.0.27
2026-Mar-16 01:45:37.027285
#51 13.16    Compiling clang-sys v1.8.1
2026-Mar-16 01:45:37.027285
#51 13.19    Compiling linux-raw-sys v0.4.15
2026-Mar-16 01:45:37.027285
#51 13.19    Compiling tinyvec v1.10.0
2026-Mar-16 01:45:37.027285
#51 13.21    Compiling indexmap v2.13.0
2026-Mar-16 01:45:37.128842
#51 13.36    Compiling pkcs8 v0.9.0
2026-Mar-16 01:45:37.128842
#51 13.36    Compiling event-listener v5.4.1
2026-Mar-16 01:45:37.128842
#51 13.36    Compiling rustc_version v0.4.1
2026-Mar-16 01:45:37.272289
#51 13.46    Compiling cmake v0.1.57
2026-Mar-16 01:45:37.272289
#51 13.46    Compiling alloc-stdlib v0.2.2
2026-Mar-16 01:45:37.272289
#51 13.47    Compiling rfc6979 v0.3.1
2026-Mar-16 01:45:37.384000
#51 13.57    Compiling sec1 v0.3.0
2026-Mar-16 01:45:37.384000
#51 13.66    Compiling signature v1.6.4
2026-Mar-16 01:45:37.506461
#51 13.73    Compiling tokio v1.49.0
2026-Mar-16 01:45:37.506461
#51 13.78    Compiling foreign-types-shared v0.1.1
2026-Mar-16 01:45:37.635197
#51 13.88    Compiling simd-adler32 v0.3.8
2026-Mar-16 01:45:37.635197
#51 13.88    Compiling openssl v0.10.75
2026-Mar-16 01:45:37.635197
#51 13.88    Compiling minimal-lexical v0.2.1
2026-Mar-16 01:45:37.635197
#51 13.88    Compiling thiserror v1.0.69
2026-Mar-16 01:45:37.635197
#51 13.88    Compiling getrandom v0.3.4
2026-Mar-16 01:45:37.635197
#51 13.91    Compiling adler2 v2.0.1
2026-Mar-16 01:45:37.748820
#51 13.97    Compiling elliptic-curve v0.12.3
2026-Mar-16 01:45:37.748820
#51 14.03    Compiling foreign-types v0.3.2
2026-Mar-16 01:45:37.868520
#51 14.03    Compiling aws-types v1.3.14
2026-Mar-16 01:45:37.868520
#51 14.10    Compiling webpki-roots v0.26.11
2026-Mar-16 01:45:38.094426
#51 14.28    Compiling brotli-decompressor v5.0.0
2026-Mar-16 01:45:38.094426
#51 14.30    Compiling unicode-normalization v0.1.25
2026-Mar-16 01:45:38.094426
#51 14.36    Compiling crossbeam-queue v0.3.12
2026-Mar-16 01:45:38.094426
#51 14.36    Compiling md-5 v0.10.6
2026-Mar-16 01:45:38.094426
#51 14.37    Compiling ring v0.17.14
2026-Mar-16 01:45:38.195208
#51 14.42    Compiling aws-lc-sys v0.38.0
2026-Mar-16 01:45:38.195208
#51 14.45    Compiling openssl-sys v0.9.111
2026-Mar-16 01:45:38.195208
#51 14.47    Compiling hashlink v0.10.0
2026-Mar-16 01:45:38.195208
#51 14.47    Compiling
2026-Mar-16 01:45:38.310234
miniz_oxide v0.8.9
2026-Mar-16 01:45:38.310234
#51 14.51    Compiling futures-util v0.3.32
2026-Mar-16 01:45:38.416721
#51 14.59    Compiling nom v7.1.3
2026-Mar-16 01:45:38.416721
#51 14.60    Compiling regex-automata v0.4.14
2026-Mar-16 01:45:38.416721
#51 14.61    Compiling libloading v0.8.9
2026-Mar-16 01:45:38.416721
#51 14.65    Compiling ecdsa v0.14.8
2026-Mar-16 01:45:38.416721
#51 14.65    Compiling native-tls v0.2.18
2026-Mar-16 01:45:38.416721
#51 14.66    Compiling unicode-properties v0.1.4
2026-Mar-16 01:45:38.416721
#51 14.66    Compiling unicode-bidi v0.3.18
2026-Mar-16 01:45:38.577071
#51 14.74    Compiling bindgen v0.69.5
2026-Mar-16 01:45:38.674160
#51 14.95    Compiling crunchy v0.2.4
2026-Mar-16 01:45:38.785422
#51 15.04    Compiling anyhow v1.0.102
2026-Mar-16 01:45:39.033001
#51 15.23    Compiling atoi v2.0.0
2026-Mar-16 01:45:39.136737
#51 15.36    Compiling p256 v0.11.1
2026-Mar-16 01:45:39.136737
#51 15.36    Compiling hkdf v0.12.4
2026-Mar-16 01:45:39.136737
#51 15.41    Compiling crypto-bigint v0.5.5
2026-Mar-16 01:45:39.261792
#51 15.50    Compiling cookie v0.18.1
2026-Mar-16 01:45:39.510560
#51 15.67    Compiling stringprep v0.1.5
2026-Mar-16 01:45:39.510560
#51 15.67    Compiling fastrand v2.3.0
2026-Mar-16 01:45:39.510560
#51 15.68    Compiling lazycell v1.3.0
2026-Mar-16 01:45:39.510560
#51 15.69    Compiling adler v1.0.2
2026-Mar-16 01:45:39.737581
#51 15.92    Compiling whoami v1.6.1
2026-Mar-16 01:45:39.737581
#51 16.01    Compiling ppv-lite86 v0.2.21
2026-Mar-16 01:45:39.855588
#51 16.01    Compiling flate2 v1.1.9
2026-Mar-16 01:45:39.855588
#51 16.01    Compiling ucd-trie v0.1.7
2026-Mar-16 01:45:40.094969
#51 16.18    Compiling synstructure v0.13.2
2026-Mar-16 01:45:40.094969
#51 16.22    Compiling compression-core v0.4.31
2026-Mar-16 01:45:40.336537
#51 16.55    Compiling brotli v8.0.2
2026-Mar-16 01:45:40.336537
#51 16.55    Compiling lazy_static v1.5.0
2026-Mar-16 01:45:40.336537
#51 16.56    Compiling rand_chacha v0.3.1
2026-Mar-16 01:45:40.336537
#51 16.61    Compiling
2026-Mar-16 01:45:40.432204
byteorder v1.5.0
2026-Mar-16 01:45:40.432204
#51 16.64    Compiling rustc-hash v1.1.0
2026-Mar-16 01:45:40.432204
#51 16.66    Compiling hashbrown v0.14.5
2026-Mar-16 01:45:40.432204
#51 16.66    Compiling tiny-keccak v2.0.2
2026-Mar-16 01:45:40.432204
#51 16.71    Compiling dotenvy v0.15.7
2026-Mar-16 01:45:40.573558
#51 16.75    Compiling pest v2.8.6
2026-Mar-16 01:45:40.573558
#51 16.80    Compiling miniz_oxide v0.7.4
2026-Mar-16 01:45:40.833186
#51 17.02    Compiling rand_core v0.9.5
2026-Mar-16 01:45:40.833186
#51 17.11    Compiling
2026-Mar-16 01:45:40.987128
gzip-header v1.0.0
2026-Mar-16 01:45:41.162075
#51 17.32    Compiling rand v0.8.5
2026-Mar-16 01:45:41.173697
#51 17.35    Compiling fslock v0.2.1
2026-Mar-16 01:45:41.173697
#51 17.43    Compiling cexpr v0.6.0
2026-Mar-16 01:45:41.329199
#51 17.50    Compiling encoding_rs v0.8.35
2026-Mar-16 01:45:41.340229
#51 17.61    Compiling
2026-Mar-16 01:45:41.464849
mime v0.3.17
2026-Mar-16 01:45:41.595126
#51 17.79    Compiling regex v1.12.3
2026-Mar-16 01:45:41.595126
#51 17.87    Compiling litrs v1.0.0
2026-Mar-16 01:45:41.699823
#51 ...
2026-Mar-16 01:45:41.699823
2026-Mar-16 01:45:41.699823
#39 [frontend builder 8/8] RUN node build-docker.mjs
2026-Mar-16 01:45:41.699823
#39 14.31 ✔ [paraglide-js] Compilation complete (message-modules)
2026-Mar-16 01:45:41.699823
#39 14.32 transforming...
2026-Mar-16 01:45:41.699823
#39 19.53 ✓ 824 modules transformed.
2026-Mar-16 01:45:41.699823
#39 19.97 rendering chunks...
2026-Mar-16 01:45:41.699823
#39 20.34 computing gzip size...
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/version.json                                                0.03 kB │ gzip:   0.05 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/assets/fredoka-latin-ext.CYrqKuxd.woff2           4.58 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/assets/nunito-normal-vietnamese.U01xdrZh.woff2   13.10 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/.vite/manifest.json                                             13.70 kB │ gzip:   1.81 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/assets/fredoka-latin.DM6njrJ3.woff2              29.73 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/assets/nunito-normal-latin.BzFMHfZw.woff2        39.13 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/assets/2.Cut_J1eR.css                             0.25 kB │ gzip:   0.18 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/assets/AppIcon.BlEdAg33.css                       0.47 kB │ gzip:   0.25 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/assets/11.B6JFksZy.css                            0.79 kB │ gzip:   0.43 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/assets/4.DvJYPIor.css                             1.18 kB │ gzip:   0.38 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/assets/12.CiKCOCFN.css                            2.91 kB │ gzip:   0.75 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/assets/3.BhQSW3U_.css                            17.70 kB │ gzip:   3.72 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/assets/0.qO957f4K.css                            54.00 kB │ gzip:  10.28 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/entry/start.Dn6Sje-S.js                           0.08 kB │ gzip:   0.09 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/BcO0PN7E.js                                0.19 kB │ gzip:   0.17 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/DIE5cPeq.js                                0.23 kB │ gzip:   0.16 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/Du3tBzWD.js                                0.37 kB │ gzip:   0.28 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/GrIWbIIL.js                                0.37 kB │ gzip:   0.26 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/DsEMc5Gj.js                                0.38 kB │ gzip:   0.27 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/BfhmWpUP.js                                0.54 kB │ gzip:   0.34 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/nodes/1.Ciw3NHeB.js                               0.55 kB │ gzip:   0.35 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/DM4YYLpb.js                                0.64 kB │ gzip:   0.33 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/BKG35LlR.js                                0.81 kB │ gzip:   0.46 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/nodes/6.BFmSmGmE.js                               0.98 kB │ gzip:   0.58 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/kvLMV7Dy.js                                1.13 kB │ gzip:   0.65 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/CmKB7RlG.js                                1.28 kB │ gzip:   0.61 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/D2gFui-Z.js                                1.37 kB │ gzip:   0.79 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/CCZ_6Ile.js                                1.91 kB │ gzip:   0.97 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/CpOETX07.js                                2.05 kB │ gzip:   1.06 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/CAoDen0a.js                                2.54 kB │ gzip:   1.22 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/CXOcYVvd.js                                2.59 kB │ gzip:   1.19 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/CrozGAN7.js                                2.81 kB │ gzip:   0.64 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/DIrn14Ou.js                                3.11 kB │ gzip:   1.40 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/De-C3LjH.js                                3.58 kB │ gzip:   1.61 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/nodes/8.DS4pXZ4O.js                               3.66 kB │ gzip:   1.39 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/CNHS8Cde.js                                4.15 kB │ gzip:   1.94 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/nodes/7.C3-0X48B.js                               4.40 kB │ gzip:   1.43 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/nodes/10.DIscW8KC.js                              5.02 kB │ gzip:   1.80 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/ByCkm1B7.js                                5.23 kB │ gzip:   2.44 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/nodes/9.BxTgHM7c.js                               6.13 kB │ gzip:   2.01 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/nodes/2.DU58KlyI.js                               6.98 kB │ gzip:   2.74 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/entry/app.BzmvnfC8.js                             8.01 kB │ gzip:   3.09 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/BywOnpAC.js                               10.82 kB │ gzip:   4.58 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/CV_rm0BC.js                               25.09 kB │ gzip:   9.83 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/DmOCDnli.js                               25.18 kB │ gzip:   8.21 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/nodes/4.BNglYVh9.js                              29.44 kB │ gzip:  11.57 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/ug6aNx1l.js                               32.89 kB │ gzip:  12.68 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/D2ajqCGR.js                               33.15 kB │ gzip:  12.03 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/nodes/0.BXZrdEUJ.js                              43.17 kB │ gzip:  10.42 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/gcBsgrmI.js                               70.17 kB │ gzip:  18.73 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/nodes/11.BJ7D65Ch.js                             79.85 kB │ gzip:  11.00 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/1kHQH8jj.js                               92.94 kB │ gzip:  19.63 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/chunks/mf4ithD4.js                              132.79 kB │ gzip:   7.39 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/nodes/12.zzlShNH7.js                            267.27 kB │ gzip: 104.84 kB
2026-Mar-16 01:45:41.699823
#39 20.39 .svelte-kit/output/client/_app/immutable/nodes/3.CFt3rqga.js                             368.32 kB │ gzip: 100.68 kB
2026-Mar-16 01:45:41.699823
#39 20.39 ✓ built in 13.10s
2026-Mar-16 01:45:41.699823
#39 21.49
2026-Mar-16 01:45:41.699823
#39 21.49 node:internal/event_target:1118
2026-Mar-16 01:45:41.699823
#39 21.49   process.nextTick(() => { throw err; });
2026-Mar-16 01:45:41.699823
#39 21.49                            ^
2026-Mar-16 01:45:41.699823
#39 21.49 Error: The following pages contain links to /#tools, but no element with id="tools" exists on / - see the `handleMissingId` option in https://svelte.dev/docs/kit/configuration#prerender for more info:
2026-Mar-16 01:45:41.699823
#39 21.49   - /
2026-Mar-16 01:45:41.699823
#39 21.49 To suppress or handle this error, implement `handleMissingId` in https://svelte.dev/docs/kit/configuration#prerender
2026-Mar-16 01:45:41.699823
#39 21.49     at file:///app/node_modules/@sveltejs/kit/src/core/config/options.js:235:13
2026-Mar-16 01:45:41.699823
#39 21.49     at file:///app/node_modules/@sveltejs/kit/src/core/postbuild/prerender.js:75:25
2026-Mar-16 01:45:41.699823
#39 21.49     at prerender (file:///app/node_modules/@sveltejs/kit/src/core/postbuild/prerender.js:575:4)
2026-Mar-16 01:45:41.699823
#39 21.49     at async MessagePort.<anonymous> (file:///app/node_modules/@sveltejs/kit/src/utils/fork.js:23:16)
2026-Mar-16 01:45:41.699823
#39 21.49
2026-Mar-16 01:45:41.699823
#39 21.49 Node.js v22.22.1
2026-Mar-16 01:45:41.699823
#39 ERROR: process "/bin/sh -c node build-docker.mjs" did not complete successfully: exit code: 1
2026-Mar-16 01:45:41.699823
2026-Mar-16 01:45:41.699823
#52 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-16 01:45:41.699823
#52 11.29   Downloaded v8 v0.101.0
2026-Mar-16 01:45:41.699823
#52 12.78    Compiling proc-macro2 v1.0.106
2026-Mar-16 01:45:41.699823
#52 12.78    Compiling unicode-ident v1.0.24
2026-Mar-16 01:45:41.699823
#52 12.78    Compiling quote v1.0.44
2026-Mar-16 01:45:41.699823
#52 12.78    Compiling libc v0.2.182
2026-Mar-16 01:45:41.699823
#52 12.80    Compiling cfg-if v1.0.4
2026-Mar-16 01:45:41.699823
#52 12.81    Compiling serde v1.0.228
2026-Mar-16 01:45:41.699823
#52 12.82    Compiling serde_core v1.0.228
2026-Mar-16 01:45:41.699823
#52 12.82    Compiling pin-project-lite v0.2.16
2026-Mar-16 01:45:41.699823
#52 12.82    Compiling parking_lot_core v0.9.12
2026-Mar-16 01:45:41.699823
#52 12.82    Compiling shlex v1.3.0
2026-Mar-16 01:45:41.699823
#52 12.82    Compiling bytes v1.11.1
2026-Mar-16 01:45:41.699823
#52 12.82    Compiling futures-core v0.3.32
2026-Mar-16 01:45:41.699823
#52 12.82    Compiling scopeguard v1.2.0
2026-Mar-16 01:45:41.699823
#52 12.84    Compiling version_check v0.9.5
2026-Mar-16 01:45:41.699823
#52 12.89    Compiling find-msvc-tools v0.1.9
2026-Mar-16 01:45:41.699823
#52 12.89    Compiling memchr v2.8.0
2026-Mar-16 01:45:41.699823
#52 12.89    Compiling itoa v1.0.17
2026-Mar-16 01:45:41.699823
#52 12.89    Compiling once_cell v1.21.3
2026-Mar-16 01:45:41.699823
#52 12.98    Compiling futures-sink v0.3.32
2026-Mar-16 01:45:41.699823
#52 12.98    Compiling typenum v1.19.0
2026-Mar-16 01:45:41.699823
#52 13.00    Compiling autocfg v1.5.0
2026-Mar-16 01:45:41.699823
#52 13.00    Compiling log v0.4.29
2026-Mar-16 01:45:41.699823
#52 13.00    Compiling slab v0.4.12
2026-Mar-16 01:45:41.699823
#52 13.00    Compiling futures-task v0.3.32
2026-Mar-16 01:45:41.699823
#52 13.02    Compiling futures-io v0.3.32
2026-Mar-16 01:45:41.699823
#52 13.02    Compiling zeroize v1.8.2
2026-Mar-16 01:45:41.699823
#52 13.03    Compiling subtle v2.6.1
2026-Mar-16 01:45:41.699823
#52 13.04    Compiling fnv v1.0.7
2026-Mar-16 01:45:41.699823
#52 13.04    Compiling equivalent v1.0.2
2026-Mar-16 01:45:41.699823
#52 13.08    Compiling percent-encoding v2.3.2
2026-Mar-16 01:45:41.699823
#52 13.08    Compiling icu_normalizer_data v2.1.1
2026-Mar-16 01:45:41.699823
#52 13.10    Compiling zerocopy v0.8.39
2026-Mar-16 01:45:41.699823
#52 13.14    Compiling lock_api v0.4.14
2026-Mar-16 01:45:41.699823
#52 13.14    Compiling icu_properties_data v2.1.2
2026-Mar-16 01:45:41.699823
#52 13.14    Compiling pin-utils v0.1.0
2026-Mar-16 01:45:41.699823
#52 13.14    Compiling hashbrown v0.16.1
2026-Mar-16 01:45:41.699823
#52 13.16    Compiling ryu v1.0.23
2026-Mar-16 01:45:41.699823
#52 13.16    Compiling powerfmt v0.2.0
2026-Mar-16 01:45:41.699823
#52 13.32    Compiling num-conv v0.2.0
2026-Mar-16 01:45:41.699823
#52 13.32    Compiling time-core v0.1.8
2026-Mar-16 01:45:41.699823
#52 13.32    Compiling crc32fast v1.5.0
2026-Mar-16 01:45:41.699823
#52 13.32    Compiling generic-array v0.14.7
2026-Mar-16 01:45:41.699823
#52 13.40    Compiling tracing-core v0.1.36
2026-Mar-16 01:45:41.699823
#52 13.42    Compiling stable_deref_trait v1.2.1
2026-Mar-16 01:45:41.699823
#52 13.45    Compiling untrusted v0.9.0
2026-Mar-16 01:45:41.699823
#52 13.49    Compiling dunce v1.0.5
2026-Mar-16 01:45:41.699823
#52 13.57    Compiling http v1.4.0
2026-Mar-16 01:45:41.699823
#52 13.57    Compiling num-traits v0.2.19
2026-Mar-16 01:45:41.699823
#52 13.60    Compiling fs_extra v1.3.0
2026-Mar-16 01:45:41.699823
#52 13.66    Compiling futures-channel v0.3.32
2026-Mar-16 01:45:41.699823
#52 13.66    Compiling http v0.2.12
2026-Mar-16 01:45:41.699823
#52 13.66    Compiling rustls-pki-types v1.14.0
2026-Mar-16 01:45:41.699823
#52 13.66    Compiling form_urlencoded v1.2.2
2026-Mar-16 01:45:41.699823
#52 13.66    Compiling litemap v0.8.1
2026-Mar-16 01:45:41.699823
#52 13.66    Compiling tower-service v0.3.3
2026-Mar-16 01:45:41.699823
#52 13.66    Compiling time-macros v0.2.27
2026-Mar-16 01:45:41.699823
#52 13.66    Compiling deranged v0.5.8
2026-Mar-16 01:45:41.699823
#52 13.66    Compiling aws-lc-rs v1.16.1
2026-Mar-16 01:45:41.699823
#52 13.66    Compiling writeable v0.6.2
2026-Mar-16 01:45:41.699823
#52 13.70    Compiling vsimd v0.8.0
2026-Mar-16 01:45:41.699823
#52 13.71    Compiling httparse v1.10.1
2026-Mar-16 01:45:41.699823
#52 13.81    Compiling zmij v1.0.21
2026-Mar-16 01:45:41.699823
#52 13.84    Compiling outref v0.5.2
2026-Mar-16 01:45:41.699823
#52 13.98    Compiling try-lock v0.2.5
2026-Mar-16 01:45:41.699823
#52 14.16    Compiling want v0.3.1
2026-Mar-16 01:45:41.699823
#52 14.28    Compiling base64-simd v0.8.0
2026-Mar-16 01:45:41.699823
#52 14.31    Compiling rustls v0.23.37
2026-Mar-16 01:45:41.699823
#52 14.31    Compiling httpdate v1.0.3
2026-Mar-16 01:45:41.699823
#52 14.31    Compiling base64 v0.22.1
2026-Mar-16 01:45:41.699823
#52 14.38    Compiling atomic-waker v1.1.2
2026-Mar-16 01:45:41.699823
#52 14.44    Compiling webpki-roots v1.0.6
2026-Mar-16 01:45:41.699823
#52 14.44    Compiling utf8_iter v1.0.4
2026-Mar-16 01:45:41.699823
#52 14.44    Compiling tower-layer v0.3.3
2026-Mar-16 01:45:41.699823
#52 14.44    Compiling cpufeatures v0.2.17
2026-Mar-16 01:45:41.699823
#52 14.51    Compiling openssl-probe v0.2.1
2026-Mar-16 01:45:41.699823
#52 14.55    Compiling sync_wrapper v1.0.2
2026-Mar-16 01:45:41.699823
#52 14.61    Compiling crossbeam-utils v0.8.21
2026-Mar-16 01:45:41.699823
#52 14.62    Compiling ipnet v2.11.0
2026-Mar-16 01:45:41.699823
#52 14.71    Compiling syn v2.0.117
2026-Mar-16 01:45:41.699823
#52 14.75    Compiling bitflags v2.11.0
2026-Mar-16 01:45:41.699823
#52 14.75    Compiling rustversion v1.0.22
2026-Mar-16 01:45:41.699823
#52 14.79    Compiling home v0.5.12
2026-Mar-16 01:45:41.699823
#52 14.86    Compiling getrandom v0.4.1
2026-Mar-16 01:45:41.699823
#52 14.86    Compiling http-body v0.4.6
2026-Mar-16 01:45:41.699823
#52 14.91    Compiling http-body v1.0.1
2026-Mar-16 01:45:41.699823
#52 14.92    Compiling rustls-native-certs v0.8.3
2026-Mar-16 01:45:41.699823
#52 14.95    Compiling hex v0.4.3
2026-Mar-16 01:45:41.699823
#52 15.00    Compiling const-oid v0.9.6
2026-Mar-16 01:45:41.699823
#52 15.08    Compiling errno v0.3.14
2026-Mar-16 01:45:41.699823
#52 15.10    Compiling socket2 v0.6.2
2026-Mar-16 01:45:41.699823
#52 15.11    Compiling mio v1.1.1
2026-Mar-16 01:45:41.699823
#52 15.13    Compiling getrandom v0.2.17
2026-Mar-16 01:45:41.699823
#52 15.16    Compiling socket2 v0.5.10
2026-Mar-16 01:45:41.699823
#52 15.16    Compiling thiserror v2.0.18
2026-Mar-16 01:45:41.699823
#52 15.22    Compiling jobserver v0.1.34
2026-Mar-16 01:45:41.699823
#52 15.22    Compiling http-body-util v0.1.3
2026-Mar-16 01:45:41.699823
#52 15.32    Compiling serde_json v1.0.149
2026-Mar-16 01:45:41.699823
#52 15.34    Compiling signal-hook-registry v1.4.8
2026-Mar-16 01:45:41.699823
#52 15.34    Compiling der v0.6.1
2026-Mar-16 01:45:41.699823
#52 15.48    Compiling allocator-api2 v0.2.21
2026-Mar-16 01:45:41.699823
#52 15.54    Compiling rand_core v0.6.4
2026-Mar-16 01:45:41.699823
#52 15.60    Compiling vcpkg v0.2.15
2026-Mar-16 01:45:41.699823
#52 15.68    Compiling cc v1.2.56
2026-Mar-16 01:45:41.699823
#52 15.71    Compiling time v0.3.47
2026-Mar-16 01:45:41.699823
#52 15.72    Compiling pkg-config v0.3.32
2026-Mar-16 01:45:41.699823
#52 15.77    Compiling base64ct v1.8.3
2026-Mar-16 01:45:41.699823
#52 15.79    Compiling uuid v1.21.0
2026-Mar-16 01:45:41.699823
#52 15.89    Compiling rustls v0.21.12
2026-Mar-16 01:45:41.699823
#52 15.91    Compiling ff v0.12.1
2026-Mar-16 01:45:41.699823
#52 16.04    Compiling base16ct v0.1.1
2026-Mar-16 01:45:41.699823
#52 16.09    Compiling rustix v0.38.44
2026-Mar-16 01:45:41.699823
#52 16.09    Compiling crc-catalog v2.4.0
2026-Mar-16 01:45:41.699823
#52 16.12    Compiling glob v0.3.3
2026-Mar-16 01:45:41.699823
#52 16.15    Compiling num-integer v0.1.46
2026-Mar-16 01:45:41.699823
#52 16.17    Compiling foldhash v0.1.5
2026-Mar-16 01:45:41.699823
#52 16.23    Compiling group v0.12.1
2026-Mar-16 01:45:41.699823
#52 16.23    Compiling concurrent-queue v2.5.0
2026-Mar-16 01:45:41.699823
#52 16.39    Compiling crc v3.4.0
2026-Mar-16 01:45:41.699823
#52 16.51    Compiling hashbrown v0.15.5
2026-Mar-16 01:45:41.699823
#52 16.52    Compiling semver v1.0.27
2026-Mar-16 01:45:41.699823
#52 16.56    Compiling tinyvec_macros v0.1.1
2026-Mar-16 01:45:41.699823
#52 16.65    Compiling prettyplease v0.2.37
2026-Mar-16 01:45:41.699823
#52 16.73    Compiling crypto-common v0.1.7
2026-Mar-16 01:45:41.699823
#52 16.73    Compiling block-buffer v0.10.4
2026-Mar-16 01:45:41.699823
#52 16.73    Compiling spki v0.6.0
2026-Mar-16 01:45:41.699823
#52 16.74    Compiling clang-sys v1.8.1
2026-Mar-16 01:45:41.699823
#52 16.75    Compiling alloc-no-stdlib v2.0.4
2026-Mar-16 01:45:41.699823
#52 16.75    Compiling linux-raw-sys v0.4.15
2026-Mar-16 01:45:41.699823
#52 16.83    Compiling parking v2.2.1
2026-Mar-16 01:45:41.699823
#52 16.87    Compiling tinyvec v1.10.0
2026-Mar-16 01:45:41.699823
#52 16.90    Compiling indexmap v2.13.0
2026-Mar-16 01:45:41.699823
#52 16.94    Compiling rustc_version v0.4.1
2026-Mar-16 01:45:41.699823
#52 17.01    Compiling pkcs8 v0.9.0
2026-Mar-16 01:45:41.699823
#52 17.28    Compiling cmake v0.1.57
2026-Mar-16 01:45:41.699823
#52 17.30    Compiling digest v0.10.7
2026-Mar-16 01:45:41.699823
#52 17.30    Compiling crypto-bigint v0.4.9
2026-Mar-16 01:45:41.699823
#52 17.30    Compiling sec1 v0.3.0
2026-Mar-16 01:45:41.699823
#52 17.30    Compiling event-listener v5.4.1
2026-Mar-16 01:45:41.699823
#52 17.30    Compiling alloc-stdlib v0.2.2
2026-Mar-16 01:45:41.699823
#52 17.48    Compiling tokio v1.49.0
2026-Mar-16 01:45:41.699823
#52 17.48    Compiling aho-corasick v1.1.4
2026-Mar-16 01:45:41.699823
#52 17.48    Compiling adler2 v2.0.1
2026-Mar-16 01:45:41.699823
#52 17.53    Compiling simd-adler32 v0.3.8
2026-Mar-16 01:45:41.699823
#52 17.65    Compiling minimal-lexical v0.2.1
2026-Mar-16 01:45:41.699823
#52 17.72    Compiling foreign-types-shared v0.1.1
2026-Mar-16 01:45:41.699823
#52 17.75    Compiling thiserror v1.0.69
2026-Mar-16 01:45:41.699823
#52 17.90    Compiling regex-syntax v0.8.10
2026-Mar-16 01:45:41.699823
#52 17.90    Compiling openssl v0.10.75
2026-Mar-16 01:45:41.699823
#52 17.94    Compiling hmac v0.12.1
2026-Mar-16 01:45:41.699823
#52 17.96    Compiling sha2 v0.10.9
2026-Mar-16 01:45:41.699823
#52 17.96    Compiling signature v1.6.4
2026-Mar-16 01:45:41.699823
#52 17.97    Compiling nom v7.1.3
2026-Mar-16 01:45:41.699823
#52 17.97    Compiling brotli-decompressor v5.0.0
2026-Mar-16 01:45:41.890071
#52 18.02    Compiling md-5 v0.10.6
2026-Mar-16 01:45:41.890071
#52 18.04    Compiling miniz_oxide v0.8.9
2026-Mar-16 01:45:41.968509
#52 CANCELED
2026-Mar-16 01:45:41.977567
#51 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-16 01:45:42.024126
#51 CANCELED
2026-Mar-16 01:45:42.024126
------
2026-Mar-16 01:45:42.024126
> [frontend builder 8/8] RUN node build-docker.mjs:
2026-Mar-16 01:45:42.024126
21.49                            ^
2026-Mar-16 01:45:42.024126
21.49 Error: The following pages contain links to /#tools, but no element with id="tools" exists on / - see the `handleMissingId` option in https://svelte.dev/docs/kit/configuration#prerender for more info:
2026-Mar-16 01:45:42.024126
21.49   - /
2026-Mar-16 01:45:42.024126
21.49 To suppress or handle this error, implement `handleMissingId` in https://svelte.dev/docs/kit/configuration#prerender
2026-Mar-16 01:45:42.024126
21.49     at file:///app/node_modules/@sveltejs/kit/src/core/config/options.js:235:13
2026-Mar-16 01:45:42.024126
21.49     at file:///app/node_modules/@sveltejs/kit/src/core/postbuild/prerender.js:75:25
2026-Mar-16 01:45:42.024126
21.49     at prerender (file:///app/node_modules/@sveltejs/kit/src/core/postbuild/prerender.js:575:4)
2026-Mar-16 01:45:42.024126
21.49     at async MessagePort.<anonymous> (file:///app/node_modules/@sveltejs/kit/src/utils/fork.js:23:16)
2026-Mar-16 01:45:42.024126
21.49
2026-Mar-16 01:45:42.024126
21.49 Node.js v22.22.1
2026-Mar-16 01:45:42.024126
------
2026-Mar-16 01:45:42.032828
Dockerfile.frontend:57
2026-Mar-16 01:45:42.032828
2026-Mar-16 01:45:42.032828
--------------------
2026-Mar-16 01:45:42.032828
2026-Mar-16 01:45:42.032828
55 |
2026-Mar-16 01:45:42.032828
2026-Mar-16 01:45:42.032828
56 |     # Build via programmatic API — bypasses vite.config.ts loading (esbuild issues on Alpine)
2026-Mar-16 01:45:42.032828
2026-Mar-16 01:45:42.032828
57 | >>> RUN node build-docker.mjs
2026-Mar-16 01:45:42.032828
2026-Mar-16 01:45:42.032828
58 |
2026-Mar-16 01:45:42.032828
2026-Mar-16 01:45:42.032828
59 |     # Runtime
2026-Mar-16 01:45:42.032828
2026-Mar-16 01:45:42.032828
--------------------
2026-Mar-16 01:45:42.032828
2026-Mar-16 01:45:42.032828
target frontend: failed to solve: process "/bin/sh -c node build-docker.mjs" did not complete successfully: exit code: 1
2026-Mar-16 01:45:42.032828
2026-Mar-16 01:45:42.032828
exit status 1
2026-Mar-16 01:45:42.072830
========================================
2026-Mar-16 01:45:42.084439
Deployment failed: Command execution failed (exit code 1): docker exec u88wkggs8ocgkwgoookcgs0s bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/build-time.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/u88wkggs8ocgkwgoookcgs0s -f /artifacts/u88wkggs8ocgkwgoookcgs0s/docker/docker-compose.server.yml build --pull --build-arg COOLIFY_URL --build-arg COOLIFY_FQDN --build-arg SERVICE_URL_API --build-arg SERVICE_FQDN_FRONTEND --build-arg SERVICE_FQDN_API --build-arg ORIGIN --build-arg SERVICE_URL_FRONTEND --build-arg WHOP_WEBHOOK_SECRET --build-arg BETTER_AUTH_TRUSTED_ORIGINS --build-arg POSTGRES_PASSWORD --build-arg BETTER_AUTH_SECRET --build-arg SOCKS5_PROXY_URL --build-arg GOOGLE_CLIENT_ID --build-arg GOOGLE_CLIENT_SECRET --build-arg WHOP_PLAN_ID --build-arg VITE_API_URL --build-arg DOCKER_REDIS_URL --build-arg PROXY_LIST --build-arg MUX_QUEUE_STREAM --build-arg MUX_DIRECT_DOWNLOAD --build-arg MUX_ARTIFACT_BACKEND --build-arg S3_BUCKET_NAME --build-arg S3_ACCESS_KEY_ID --build-arg S3_SECRET_ACCESS_KEY --build-arg S3_REGION --build-arg S3_ENDPOINT --build-arg ADMIN_EMAILS --build-arg MUX_ARTIFACT_TTL_SECS --build-arg MUX_CLEANUP_INTERVAL_SECS --build-arg REDIS_URL --build-arg MUX_FILE_TICKET_TTL_SECS --build-arg COOLIFY_BUILD_SECRETS_HASH=147102dc33226fc8d787cf78687ecc4cf02b2168677d826e17042f8d4c5646fc'
2026-Mar-16 01:45:42.084439
Error: Dockerfile.frontend:57
2026-Mar-16 01:45:42.084439
2026-Mar-16 01:45:42.084439
--------------------
2026-Mar-16 01:45:42.084439
2026-Mar-16 01:45:42.084439
55 |
2026-Mar-16 01:45:42.084439
2026-Mar-16 01:45:42.084439
56 |     # Build via programmatic API — bypasses vite.config.ts loading (esbuild issues on Alpine)
2026-Mar-16 01:45:42.084439
2026-Mar-16 01:45:42.084439
57 | >>> RUN node build-docker.mjs
2026-Mar-16 01:45:42.084439
2026-Mar-16 01:45:42.084439
58 |
2026-Mar-16 01:45:42.084439
2026-Mar-16 01:45:42.084439
59 |     # Runtime
2026-Mar-16 01:45:42.084439
2026-Mar-16 01:45:42.084439
--------------------
2026-Mar-16 01:45:42.084439
2026-Mar-16 01:45:42.084439
target frontend: failed to solve: process "/bin/sh -c node build-docker.mjs" did not complete successfully: exit code: 1
2026-Mar-16 01:45:42.084439
2026-Mar-16 01:45:42.084439
exit status 1
2026-Mar-16 01:45:42.096222
Error type: RuntimeException
2026-Mar-16 01:45:42.107578
Error code: 0
2026-Mar-16 01:45:42.119225
Location: /var/www/html/app/Traits/ExecuteRemoteCommand.php:243
2026-Mar-16 01:45:42.130652
Stack trace (first 5 lines):
2026-Mar-16 01:45:42.141710
#0 /var/www/html/app/Traits/ExecuteRemoteCommand.php(104): App\Jobs\ApplicationDeploymentJob->executeCommandWithProcess()
2026-Mar-16 01:45:42.152974
#1 /var/www/html/vendor/laravel/framework/src/Illuminate/Collections/Traits/EnumeratesValues.php(272): App\Jobs\ApplicationDeploymentJob->{closure:App\Traits\ExecuteRemoteCommand::execute_remote_command():71}()
2026-Mar-16 01:45:42.164407
#2 /var/www/html/app/Traits/ExecuteRemoteCommand.php(71): Illuminate\Support\Collection->each()
2026-Mar-16 01:45:42.177347
#3 /var/www/html/app/Jobs/ApplicationDeploymentJob.php(730): App\Jobs\ApplicationDeploymentJob->execute_remote_command()
2026-Mar-16 01:45:42.191395
#4 /var/www/html/app/Jobs/ApplicationDeploymentJob.php(467): App\Jobs\ApplicationDeploymentJob->deploy_docker_compose_buildpack()
2026-Mar-16 01:45:42.204278
========================================
2026-Mar-16 01:45:43.073087
Gracefully shutting down build container: u88wkggs8ocgkwgoookcgs0s