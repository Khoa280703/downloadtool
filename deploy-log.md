Deployment is
Failed
Find in logs






2026-Mar-21 03:29:12.674885
Starting deployment of khoa280703/downloadtool:main-zoscg4oc04gkwkssg0kw8w8w to localhost.
2026-Mar-21 03:29:13.283281
Preparing container with helper image: ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Mar-21 03:29:13.608171
[CMD]: docker stop -t 30 so800kkkogg8g0wo804c4wwc
2026-Mar-21 03:29:13.608171
Error response from daemon: No such container: so800kkkogg8g0wo804c4wwc
2026-Mar-21 03:29:13.948529
[CMD]: docker run -d --network coolify --name so800kkkogg8g0wo804c4wwc  --rm -v /var/run/docker.sock:/var/run/docker.sock ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Mar-21 03:29:13.948529
c04c51688dcd55b71e0433f34364fae3fba45b6736c28c702f80a4be8ff37870
2026-Mar-21 03:29:15.379220
[CMD]: docker exec so800kkkogg8g0wo804c4wwc bash -c 'GIT_SSH_COMMAND="ssh -o ConnectTimeout=30 -p 22 -o Port=22 -o LogLevel=ERROR -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git ls-remote https://github.com/Khoa280703/downloadtool refs/heads/main'
2026-Mar-21 03:29:15.379220
b6153d9e4682c1df48f2e096da582cb16d55164d	refs/heads/main
2026-Mar-21 03:29:15.389343
----------------------------------------
2026-Mar-21 03:29:15.393308
Importing Khoa280703/downloadtool:main (commit sha b6153d9e4682c1df48f2e096da582cb16d55164d) to /artifacts/so800kkkogg8g0wo804c4wwc.
2026-Mar-21 03:29:15.745847
[CMD]: docker exec so800kkkogg8g0wo804c4wwc bash -c 'git clone --depth=1 --recurse-submodules --shallow-submodules -b 'main' 'https://github.com/Khoa280703/downloadtool' '/artifacts/so800kkkogg8g0wo804c4wwc' && cd '/artifacts/so800kkkogg8g0wo804c4wwc' && if [ -f .gitmodules ]; then sed -i "s#git@\(.*\):#https://\1/#g" '/artifacts/so800kkkogg8g0wo804c4wwc'/.gitmodules || true && git submodule sync && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git submodule update --init --recursive --depth=1; fi && cd '/artifacts/so800kkkogg8g0wo804c4wwc' && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git lfs pull'
2026-Mar-21 03:29:15.745847
Cloning into '/artifacts/so800kkkogg8g0wo804c4wwc'...
2026-Mar-21 03:29:18.440307
[CMD]: docker exec so800kkkogg8g0wo804c4wwc bash -c 'cd /artifacts/so800kkkogg8g0wo804c4wwc && git log -1 b6153d9e4682c1df48f2e096da582cb16d55164d --pretty=%B'
2026-Mar-21 03:29:18.440307
chore(config): clean compose and coolify env boundaries
2026-Mar-21 03:29:23.472174
[CMD]: docker exec so800kkkogg8g0wo804c4wwc bash -c 'test -f /artifacts/so800kkkogg8g0wo804c4wwc/docker/Dockerfile.api && echo 'exists' || echo 'not found''
2026-Mar-21 03:29:23.472174
exists
2026-Mar-21 03:29:23.828124
[CMD]: docker exec so800kkkogg8g0wo804c4wwc bash -c 'cat /artifacts/so800kkkogg8g0wo804c4wwc/docker/Dockerfile.api'
2026-Mar-21 03:29:23.828124
# Dockerfile for API service deployment
2026-Mar-21 03:29:23.828124
# Builds the API server and related components without GPU support
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
# Stage 0: Build injector JS (embedded into api crate via include_str! at compile time)
2026-Mar-21 03:29:23.828124
FROM node:22-alpine AS js-builder
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
WORKDIR /app
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
RUN npm install -g pnpm
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
# Copy workspace manifests for pnpm resolution
2026-Mar-21 03:29:23.828124
COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Mar-21 03:29:23.828124
COPY packages/api-client/package.json ./packages/api-client/
2026-Mar-21 03:29:23.828124
COPY apps/injector/package.json ./apps/injector/
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
# Copy injector source and shared packages
2026-Mar-21 03:29:23.828124
COPY apps/injector/ ./apps/injector/
2026-Mar-21 03:29:23.828124
COPY packages/ ./packages/
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
# Install deps and build injector (produces dist/bm.js and dist/youtube-downloader.user.js)
2026-Mar-21 03:29:23.828124
RUN pnpm install --frozen-lockfile
2026-Mar-21 03:29:23.828124
RUN pnpm --filter @downloadtool/injector build
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
# Build extractor TypeScript to IIFE format (required by crates/extractor/build.rs)
2026-Mar-21 03:29:23.828124
COPY extractors/ ./extractors/
2026-Mar-21 03:29:23.828124
RUN mkdir -p extractors/dist && \
2026-Mar-21 03:29:23.828124
npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js && \
2026-Mar-21 03:29:23.828124
npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
# Stage 1: Rust builder
2026-Mar-21 03:29:23.828124
FROM rust:1.91-bookworm AS builder
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
WORKDIR /app
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
# Install dependencies
2026-Mar-21 03:29:23.828124
RUN apt-get update && apt-get install -y \
2026-Mar-21 03:29:23.828124
pkg-config \
2026-Mar-21 03:29:23.828124
libssl-dev \
2026-Mar-21 03:29:23.828124
&& rm -rf /var/lib/apt/lists/*
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
# Copy workspace configuration
2026-Mar-21 03:29:23.828124
COPY Cargo.toml ./
2026-Mar-21 03:29:23.828124
COPY Cargo.lock ./
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
# Copy all crates
2026-Mar-21 03:29:23.828124
COPY crates/ ./crates/
2026-Mar-21 03:29:23.828124
COPY config/ ./config/
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
# Copy injector dist (required by include_str! in crates/api/src/routes/static_files.rs)
2026-Mar-21 03:29:23.828124
COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
# Copy extractor source + pre-built IIFE dist (built by js-builder stage)
2026-Mar-21 03:29:23.828124
COPY extractors/ ./extractors/
2026-Mar-21 03:29:23.828124
COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
# Build the release binary
2026-Mar-21 03:29:23.828124
RUN cargo build --release --bin api-server
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
# Stage 2: Runtime
2026-Mar-21 03:29:23.828124
FROM debian:bookworm-slim AS runtime
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
WORKDIR /app
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
# Install runtime dependencies
2026-Mar-21 03:29:23.828124
RUN apt-get update && apt-get install -y \
2026-Mar-21 03:29:23.828124
ca-certificates \
2026-Mar-21 03:29:23.828124
curl \
2026-Mar-21 03:29:23.828124
libssl3 \
2026-Mar-21 03:29:23.828124
&& rm -rf /var/lib/apt/lists/*
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
# Install latest yt-dlp binary (newer than Debian package).
2026-Mar-21 03:29:23.828124
RUN set -eux; \
2026-Mar-21 03:29:23.828124
arch="$(dpkg --print-architecture)"; \
2026-Mar-21 03:29:23.828124
case "$arch" in \
2026-Mar-21 03:29:23.828124
amd64) ytdlp_asset="yt-dlp_linux" ;; \
2026-Mar-21 03:29:23.828124
arm64) ytdlp_asset="yt-dlp_linux_aarch64" ;; \
2026-Mar-21 03:29:23.828124
*) echo "Unsupported architecture: $arch" >&2; exit 1 ;; \
2026-Mar-21 03:29:23.828124
esac; \
2026-Mar-21 03:29:23.828124
curl -fL "https://github.com/yt-dlp/yt-dlp/releases/latest/download/${ytdlp_asset}" -o /usr/local/bin/yt-dlp; \
2026-Mar-21 03:29:23.828124
chmod +x /usr/local/bin/yt-dlp; \
2026-Mar-21 03:29:23.828124
/usr/local/bin/yt-dlp --version
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
# Create non-root user
2026-Mar-21 03:29:23.828124
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
# Copy binary from builder
2026-Mar-21 03:29:23.828124
COPY --from=builder /app/target/release/api-server /usr/local/bin/
2026-Mar-21 03:29:23.828124
COPY --from=builder /app/crates/api/app-migrations /app/app-migrations
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
# Create directories
2026-Mar-21 03:29:23.828124
RUN mkdir -p /app/extractors /app/data /app/proxy-state && chown -R appuser:appuser /app
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
# Switch to non-root user
2026-Mar-21 03:29:23.828124
USER appuser
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
# Environment variables
2026-Mar-21 03:29:23.828124
ENV PORT=3068
2026-Mar-21 03:29:23.828124
ENV EXTRACTOR_DIR=/app/extractors
2026-Mar-21 03:29:23.828124
ENV YTDLP_PATH=/usr/local/bin/yt-dlp
2026-Mar-21 03:29:23.828124
ENV RUST_LOG=info
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
# Expose port
2026-Mar-21 03:29:23.828124
EXPOSE 3068
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
# Health check
2026-Mar-21 03:29:23.828124
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Mar-21 03:29:23.828124
CMD curl -f http://localhost:3068/health || exit 1
2026-Mar-21 03:29:23.828124
2026-Mar-21 03:29:23.828124
# Run the server
2026-Mar-21 03:29:23.828124
CMD ["api-server"]
2026-Mar-21 03:29:24.206328
Added 102 ARG declarations to Dockerfile for service api (multi-stage build, added to 3 stages).
2026-Mar-21 03:29:24.566363
[CMD]: docker exec so800kkkogg8g0wo804c4wwc bash -c 'test -f /artifacts/so800kkkogg8g0wo804c4wwc/docker/Dockerfile.worker && echo 'exists' || echo 'not found''
2026-Mar-21 03:29:24.566363
exists
2026-Mar-21 03:29:24.921834
[CMD]: docker exec so800kkkogg8g0wo804c4wwc bash -c 'cat /artifacts/so800kkkogg8g0wo804c4wwc/docker/Dockerfile.worker'
2026-Mar-21 03:29:24.921834
# Dockerfile for mux worker deployment
2026-Mar-21 03:29:24.921834
2026-Mar-21 03:29:24.921834
# Stage 0: Build extractor TypeScript to IIFE format (required by crates/extractor/build.rs)
2026-Mar-21 03:29:24.921834
FROM node:22-alpine AS js-builder
2026-Mar-21 03:29:24.921834
2026-Mar-21 03:29:24.921834
WORKDIR /app
2026-Mar-21 03:29:24.921834
2026-Mar-21 03:29:24.921834
RUN npm install -g pnpm
2026-Mar-21 03:29:24.921834
2026-Mar-21 03:29:24.921834
COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Mar-21 03:29:24.921834
COPY packages/api-client/package.json ./packages/api-client/
2026-Mar-21 03:29:24.921834
COPY apps/injector/package.json ./apps/injector/
2026-Mar-21 03:29:24.921834
COPY packages/ ./packages/
2026-Mar-21 03:29:24.921834
COPY apps/injector/ ./apps/injector/
2026-Mar-21 03:29:24.921834
COPY extractors/ ./extractors/
2026-Mar-21 03:29:24.921834
2026-Mar-21 03:29:24.921834
RUN pnpm install --frozen-lockfile
2026-Mar-21 03:29:24.921834
RUN mkdir -p extractors/dist && \
2026-Mar-21 03:29:24.921834
npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js && \
2026-Mar-21 03:29:24.921834
npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-21 03:29:24.921834
2026-Mar-21 03:29:24.921834
# Stage 1: Rust builder
2026-Mar-21 03:29:24.921834
FROM rust:1.91-bookworm AS builder
2026-Mar-21 03:29:24.921834
2026-Mar-21 03:29:24.921834
WORKDIR /app
2026-Mar-21 03:29:24.921834
2026-Mar-21 03:29:24.921834
RUN apt-get update && apt-get install -y \
2026-Mar-21 03:29:24.921834
pkg-config \
2026-Mar-21 03:29:24.921834
libssl-dev \
2026-Mar-21 03:29:24.921834
&& rm -rf /var/lib/apt/lists/*
2026-Mar-21 03:29:24.921834
2026-Mar-21 03:29:24.921834
COPY Cargo.toml ./
2026-Mar-21 03:29:24.921834
COPY Cargo.lock ./
2026-Mar-21 03:29:24.921834
COPY crates/ ./crates/
2026-Mar-21 03:29:24.921834
COPY config/ ./config/
2026-Mar-21 03:29:24.921834
COPY extractors/ ./extractors/
2026-Mar-21 03:29:24.921834
COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-21 03:29:24.921834
2026-Mar-21 03:29:24.921834
RUN cargo build --release --bin mux-worker
2026-Mar-21 03:29:24.921834
2026-Mar-21 03:29:24.921834
# Stage 2: Runtime
2026-Mar-21 03:29:24.921834
FROM debian:bookworm-slim AS runtime
2026-Mar-21 03:29:24.921834
2026-Mar-21 03:29:24.921834
WORKDIR /app
2026-Mar-21 03:29:24.921834
2026-Mar-21 03:29:24.921834
RUN apt-get update && apt-get install -y \
2026-Mar-21 03:29:24.921834
ca-certificates \
2026-Mar-21 03:29:24.921834
curl \
2026-Mar-21 03:29:24.921834
libssl3 \
2026-Mar-21 03:29:24.921834
&& rm -rf /var/lib/apt/lists/*
2026-Mar-21 03:29:24.921834
2026-Mar-21 03:29:24.921834
RUN set -eux; \
2026-Mar-21 03:29:24.921834
arch="$(dpkg --print-architecture)"; \
2026-Mar-21 03:29:24.921834
case "$arch" in \
2026-Mar-21 03:29:24.921834
amd64) ytdlp_asset="yt-dlp_linux" ;; \
2026-Mar-21 03:29:24.921834
arm64) ytdlp_asset="yt-dlp_linux_aarch64" ;; \
2026-Mar-21 03:29:24.921834
*) echo "Unsupported architecture: $arch" >&2; exit 1 ;; \
2026-Mar-21 03:29:24.921834
esac; \
2026-Mar-21 03:29:24.921834
curl -fL "https://github.com/yt-dlp/yt-dlp/releases/latest/download/${ytdlp_asset}" -o /usr/local/bin/yt-dlp; \
2026-Mar-21 03:29:24.921834
chmod +x /usr/local/bin/yt-dlp; \
2026-Mar-21 03:29:24.921834
/usr/local/bin/yt-dlp --version
2026-Mar-21 03:29:24.921834
2026-Mar-21 03:29:24.921834
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Mar-21 03:29:24.921834
2026-Mar-21 03:29:24.921834
COPY --from=builder /app/target/release/mux-worker /usr/local/bin/
2026-Mar-21 03:29:24.921834
COPY --from=builder /app/crates/api/app-migrations /app/app-migrations
2026-Mar-21 03:29:24.921834
2026-Mar-21 03:29:24.921834
RUN mkdir -p /app/extractors /app/proxy-state && chown -R appuser:appuser /app
2026-Mar-21 03:29:24.921834
2026-Mar-21 03:29:24.921834
USER appuser
2026-Mar-21 03:29:24.921834
2026-Mar-21 03:29:24.921834
ENV EXTRACTOR_DIR=/app/extractors
2026-Mar-21 03:29:24.921834
ENV YTDLP_PATH=/usr/local/bin/yt-dlp
2026-Mar-21 03:29:24.921834
ENV RUST_LOG=info
2026-Mar-21 03:29:24.921834
2026-Mar-21 03:29:24.921834
CMD ["mux-worker"]
2026-Mar-21 03:29:25.278476
Added 102 ARG declarations to Dockerfile for service worker (multi-stage build, added to 3 stages).
2026-Mar-21 03:29:25.638779
[CMD]: docker exec so800kkkogg8g0wo804c4wwc bash -c 'test -f /artifacts/so800kkkogg8g0wo804c4wwc/docker/Dockerfile.frontend && echo 'exists' || echo 'not found''
2026-Mar-21 03:29:25.638779
exists
2026-Mar-21 03:29:25.985665
[CMD]: docker exec so800kkkogg8g0wo804c4wwc bash -c 'cat /artifacts/so800kkkogg8g0wo804c4wwc/docker/Dockerfile.frontend'
2026-Mar-21 03:29:25.985665
# Dockerfile for frontend (SvelteKit Node server)
2026-Mar-21 03:29:25.985665
# Copy ALL source files BEFORE npm install so svelte-kit sync (prepare script)
2026-Mar-21 03:29:25.985665
# can find svelte.config.js and generate .svelte-kit/ correctly.
2026-Mar-21 03:29:25.985665
2026-Mar-21 03:29:25.985665
FROM node:22-alpine AS builder
2026-Mar-21 03:29:25.985665
2026-Mar-21 03:29:25.985665
WORKDIR /app
2026-Mar-21 03:29:25.985665
2026-Mar-21 03:29:25.985665
# Copy all frontend source files first (node_modules excluded via .dockerignore)
2026-Mar-21 03:29:25.985665
COPY frontend/ ./
2026-Mar-21 03:29:25.985665
COPY config/ /config/
2026-Mar-21 03:29:25.985665
2026-Mar-21 03:29:25.985665
# Install — prepare script runs svelte-kit sync with svelte.config.js available
2026-Mar-21 03:29:25.985665
RUN npm install
2026-Mar-21 03:29:25.985665
2026-Mar-21 03:29:25.985665
# Build-time public API URL (embedded into client bundle by Vite)
2026-Mar-21 03:29:25.985665
# Runtime env is too late for import.meta.env in browser bundle.
2026-Mar-21 03:29:25.985665
ARG VITE_API_URL
2026-Mar-21 03:29:25.985665
ENV VITE_API_URL=${VITE_API_URL}
2026-Mar-21 03:29:25.985665
RUN test -n "$VITE_API_URL" || (echo "VITE_API_URL build arg is required" && exit 1)
2026-Mar-21 03:29:25.985665
2026-Mar-21 03:29:25.985665
# Generate Paraglide runtime/messages from frontend/messages/* before Vite build
2026-Mar-21 03:29:25.985665
RUN npm run paraglide:compile
2026-Mar-21 03:29:25.985665
2026-Mar-21 03:29:25.985665
# Build via programmatic API — bypasses vite.config.ts loading (esbuild issues on Alpine)
2026-Mar-21 03:29:25.985665
RUN node build-docker.mjs
2026-Mar-21 03:29:25.985665
2026-Mar-21 03:29:25.985665
# Runtime
2026-Mar-21 03:29:25.985665
FROM node:22-alpine AS runtime
2026-Mar-21 03:29:25.985665
2026-Mar-21 03:29:25.985665
WORKDIR /app
2026-Mar-21 03:29:25.985665
2026-Mar-21 03:29:25.985665
COPY --from=builder /app/build ./build
2026-Mar-21 03:29:25.985665
COPY --from=builder /app/package.json ./
2026-Mar-21 03:29:25.985665
COPY --from=builder /app/package-lock.json ./
2026-Mar-21 03:29:25.985665
2026-Mar-21 03:29:25.985665
# Runtime needs server-side deps (better-auth, pg) used by hooks/routes
2026-Mar-21 03:29:25.985665
RUN npm ci --omit=dev
2026-Mar-21 03:29:25.985665
2026-Mar-21 03:29:25.985665
ENV PORT=5168
2026-Mar-21 03:29:25.985665
ENV HOST=0.0.0.0
2026-Mar-21 03:29:25.985665
2026-Mar-21 03:29:25.985665
EXPOSE 5168
2026-Mar-21 03:29:25.985665
2026-Mar-21 03:29:25.985665
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Mar-21 03:29:25.985665
CMD wget -qO- http://127.0.0.1:5168 || exit 1
2026-Mar-21 03:29:25.985665
2026-Mar-21 03:29:25.985665
CMD ["node", "build"]
2026-Mar-21 03:29:26.356607
Added 68 ARG declarations to Dockerfile for service frontend (multi-stage build, added to 2 stages).
2026-Mar-21 03:29:26.362319
Pulling & building required images.
2026-Mar-21 03:29:26.424065
Creating build-time .env file in /artifacts (outside Docker context).
2026-Mar-21 03:29:26.792276
Adding build arguments to Docker Compose build command.
2026-Mar-21 03:29:27.300052
[CMD]: docker exec so800kkkogg8g0wo804c4wwc bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/build-time.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/so800kkkogg8g0wo804c4wwc -f /artifacts/so800kkkogg8g0wo804c4wwc/docker/docker-compose.server.yml build --pull --build-arg COOLIFY_URL --build-arg COOLIFY_FQDN --build-arg SERVICE_URL_API --build-arg BETTER_AUTH_TRUSTED_ORIGINS --build-arg WHOP_WEBHOOK_SECRET --build-arg SERVICE_FQDN_FRONTEND --build-arg POSTGRES_PASSWORD --build-arg ORIGIN --build-arg SERVICE_FQDN_API --build-arg BETTER_AUTH_SECRET --build-arg GOOGLE_CLIENT_ID --build-arg VITE_API_URL --build-arg WHOP_PLAN_ID --build-arg MUX_QUEUE_STREAM --build-arg MUX_DIRECT_DOWNLOAD --build-arg MUX_ARTIFACT_BACKEND --build-arg GOOGLE_CLIENT_SECRET --build-arg S3_REGION --build-arg S3_ENDPOINT --build-arg S3_BUCKET_NAME --build-arg S3_ACCESS_KEY_ID --build-arg S3_SECRET_ACCESS_KEY --build-arg MUX_ARTIFACT_TTL_SECS --build-arg MUX_CLEANUP_INTERVAL_SECS --build-arg REDIS_URL --build-arg MUX_FILE_TICKET_TTL_SECS --build-arg PROXY_REDIS_URL --build-arg PROXY_DATABASE_URL --build-arg PROXY_QUARANTINE_TTL_SECS --build-arg ADMIN_EMAILS --build-arg POSTGRES_HOST_PORT --build-arg SHARED_PROXY_POSTGRES_PASSWORD --build-arg SERVICE_URL_FRONTEND --build-arg MUX_WORKER_CONCURRENCY --build-arg COOLIFY_BUILD_SECRETS_HASH=cec27e36d78d424f237fd86a577de9616ef05596c2da430a32bceb33cc790887'
2026-Mar-21 03:29:27.300052
#1 [internal] load local bake definitions
2026-Mar-21 03:29:27.516748
#1 reading from stdin 7.81kB done
2026-Mar-21 03:29:27.516748
#1 DONE 0.0s
2026-Mar-21 03:29:27.516748
2026-Mar-21 03:29:27.516748
#2 [worker internal] load build definition from Dockerfile.worker
2026-Mar-21 03:29:27.516748
#2 transferring dockerfile: 4.61kB done
2026-Mar-21 03:29:27.516748
#2 DONE 0.0s
2026-Mar-21 03:29:27.516748
2026-Mar-21 03:29:27.516748
#3 [api internal] load build definition from Dockerfile.api
2026-Mar-21 03:29:27.516748
#3 transferring dockerfile: 5.75kB done
2026-Mar-21 03:29:27.516748
#3 DONE 0.0s
2026-Mar-21 03:29:27.516748
2026-Mar-21 03:29:27.516748
#4 [frontend internal] load build definition from Dockerfile.frontend
2026-Mar-21 03:29:27.516748
#4 transferring dockerfile: 2.99kB done
2026-Mar-21 03:29:27.516748
#4 DONE 0.0s
2026-Mar-21 03:29:27.516748
2026-Mar-21 03:29:27.516748
#5 [frontend internal] load metadata for docker.io/library/node:22-alpine
2026-Mar-21 03:29:28.844204
#5 DONE 1.4s
2026-Mar-21 03:29:28.844204
2026-Mar-21 03:29:28.844204
#6 [worker internal] load metadata for docker.io/library/debian:bookworm-slim
2026-Mar-21 03:29:28.844204
#6 DONE 1.4s
2026-Mar-21 03:29:28.844204
2026-Mar-21 03:29:28.844204
#7 [api internal] load metadata for docker.io/library/rust:1.91-bookworm
2026-Mar-21 03:29:28.844204
#7 DONE 1.4s
2026-Mar-21 03:29:28.844204
2026-Mar-21 03:29:28.844204
#8 [frontend internal] load .dockerignore
2026-Mar-21 03:29:28.844204
#8 transferring context: 341B done
2026-Mar-21 03:29:28.844204
#8 DONE 0.0s
2026-Mar-21 03:29:28.844204
2026-Mar-21 03:29:28.844204
#9 [frontend builder 1/8] FROM docker.io/library/node:22-alpine@sha256:8094c002d08262dba12645a3b4a15cd6cd627d30bc782f53229a2ec13ee22a00
2026-Mar-21 03:29:28.844204
#9 resolve docker.io/library/node:22-alpine@sha256:8094c002d08262dba12645a3b4a15cd6cd627d30bc782f53229a2ec13ee22a00 0.0s done
2026-Mar-21 03:29:28.846598
#9 DONE 0.0s
2026-Mar-21 03:29:28.846598
2026-Mar-21 03:29:28.846598
#10 [worker runtime 1/8] FROM docker.io/library/debian:bookworm-slim@sha256:f06537653ac770703bc45b4b113475bd402f451e85223f0f2837acbf89ab020a
2026-Mar-21 03:29:28.846598
#10 resolve docker.io/library/debian:bookworm-slim@sha256:f06537653ac770703bc45b4b113475bd402f451e85223f0f2837acbf89ab020a 0.0s done
2026-Mar-21 03:29:28.846598
#10 DONE 0.0s
2026-Mar-21 03:29:28.846598
2026-Mar-21 03:29:28.846598
#11 [api builder  1/10] FROM docker.io/library/rust:1.91-bookworm@sha256:c1e5f19e773b7878c3f7a805dd00a495e747acbdc76fb2337a4ebf0418896b33
2026-Mar-21 03:29:28.846598
#11 resolve docker.io/library/rust:1.91-bookworm@sha256:c1e5f19e773b7878c3f7a805dd00a495e747acbdc76fb2337a4ebf0418896b33 0.0s done
2026-Mar-21 03:29:28.846598
#11 DONE 0.0s
2026-Mar-21 03:29:28.846598
2026-Mar-21 03:29:28.846598
#12 [api internal] load build context
2026-Mar-21 03:29:28.846598
#12 transferring context: 1.03MB 0.0s done
2026-Mar-21 03:29:28.846598
#12 DONE 0.0s
2026-Mar-21 03:29:28.846598
2026-Mar-21 03:29:28.846598
#13 [worker builder  4/10] COPY Cargo.toml ./
2026-Mar-21 03:29:28.846598
#13 CACHED
2026-Mar-21 03:29:28.846598
2026-Mar-21 03:29:28.846598
#14 [worker builder  3/10] RUN apt-get update && apt-get install -y     pkg-config     libssl-dev     && rm -rf /var/lib/apt/lists/*
2026-Mar-21 03:29:28.846598
#14 CACHED
2026-Mar-21 03:29:28.846598
2026-Mar-21 03:29:28.846598
#15 [worker builder  2/10] WORKDIR /app
2026-Mar-21 03:29:28.846598
#15 CACHED
2026-Mar-21 03:29:28.846598
2026-Mar-21 03:29:28.846598
#16 [worker builder  5/10] COPY Cargo.lock ./
2026-Mar-21 03:29:28.846598
#16 CACHED
2026-Mar-21 03:29:28.846598
2026-Mar-21 03:29:28.846598
#17 [frontend internal] load build context
2026-Mar-21 03:29:28.846598
#17 transferring context: 2.07MB 0.0s done
2026-Mar-21 03:29:28.846598
#17 DONE 0.1s
2026-Mar-21 03:29:28.846598
2026-Mar-21 03:29:28.846598
#18 [frontend builder 2/8] WORKDIR /app
2026-Mar-21 03:29:28.948527
#18 CACHED
2026-Mar-21 03:29:28.948527
2026-Mar-21 03:29:28.948527
#19 [worker js-builder 10/11] RUN pnpm install --frozen-lockfile
2026-Mar-21 03:29:28.948527
#19 CACHED
2026-Mar-21 03:29:28.948527
2026-Mar-21 03:29:28.948527
#20 [worker js-builder  7/11] COPY packages/ ./packages/
2026-Mar-21 03:29:28.948527
#20 CACHED
2026-Mar-21 03:29:28.948527
2026-Mar-21 03:29:28.948527
#21 [worker js-builder  8/11] COPY apps/injector/ ./apps/injector/
2026-Mar-21 03:29:28.948527
#21 CACHED
2026-Mar-21 03:29:28.948527
2026-Mar-21 03:29:28.948527
#22 [worker js-builder  9/11] COPY extractors/ ./extractors/
2026-Mar-21 03:29:28.948527
#22 CACHED
2026-Mar-21 03:29:28.948527
2026-Mar-21 03:29:28.948527
#23 [api js-builder  8/12] COPY packages/ ./packages/
2026-Mar-21 03:29:28.948527
#23 CACHED
2026-Mar-21 03:29:28.948527
2026-Mar-21 03:29:28.948527
#24 [api js-builder  7/12] COPY apps/injector/ ./apps/injector/
2026-Mar-21 03:29:28.948527
#24 CACHED
2026-Mar-21 03:29:28.948527
2026-Mar-21 03:29:28.948527
#25 [api js-builder  3/11] RUN npm install -g pnpm
2026-Mar-21 03:29:28.948527
#25 CACHED
2026-Mar-21 03:29:28.948527
2026-Mar-21 03:29:28.948527
#26 [api js-builder  5/11] COPY packages/api-client/package.json ./packages/api-client/
2026-Mar-21 03:29:28.948527
#26 CACHED
2026-Mar-21 03:29:28.948527
2026-Mar-21 03:29:28.948527
#27 [api js-builder  9/12] RUN pnpm install --frozen-lockfile
2026-Mar-21 03:29:28.948527
#27 CACHED
2026-Mar-21 03:29:28.948527
2026-Mar-21 03:29:28.948527
#28 [api js-builder 10/12] RUN pnpm --filter @downloadtool/injector build
2026-Mar-21 03:29:28.948527
#28 CACHED
2026-Mar-21 03:29:28.948527
2026-Mar-21 03:29:28.948527
#29 [api js-builder 11/12] COPY extractors/ ./extractors/
2026-Mar-21 03:29:28.948527
#29 CACHED
2026-Mar-21 03:29:28.948527
2026-Mar-21 03:29:28.948527
#30 [api js-builder  6/11] COPY apps/injector/package.json ./apps/injector/
2026-Mar-21 03:29:28.948527
#30 CACHED
2026-Mar-21 03:29:28.948527
2026-Mar-21 03:29:28.948527
#31 [api js-builder  4/11] COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Mar-21 03:29:28.948527
#31 CACHED
2026-Mar-21 03:29:28.948527
2026-Mar-21 03:29:28.948527
#32 [worker js-builder 11/11] RUN mkdir -p extractors/dist &&     npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js &&     npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-21 03:29:28.948527
#32 CACHED
2026-Mar-21 03:29:28.948527
2026-Mar-21 03:29:28.948527
#33 [api js-builder 12/12] RUN mkdir -p extractors/dist &&     npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js &&     npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-21 03:29:28.948527
#33 CACHED
2026-Mar-21 03:29:28.948527
2026-Mar-21 03:29:28.948527
#34 [worker builder  6/10] COPY crates/ ./crates/
2026-Mar-21 03:29:28.948527
#34 DONE 0.1s
2026-Mar-21 03:29:28.948527
2026-Mar-21 03:29:28.948527
#35 [frontend builder 3/8] COPY frontend/ ./
2026-Mar-21 03:29:28.948527
#35 DONE 0.0s
2026-Mar-21 03:29:28.948527
2026-Mar-21 03:29:28.948527
#36 [api builder  7/10] COPY config/ ./config/
2026-Mar-21 03:29:28.948527
#36 DONE 0.0s
2026-Mar-21 03:29:28.948527
2026-Mar-21 03:29:28.948527
#37 [frontend builder 4/8] COPY config/ /config/
2026-Mar-21 03:29:28.948527
#37 DONE 0.0s
2026-Mar-21 03:29:28.948527
2026-Mar-21 03:29:28.948527
#38 [worker builder  8/10] COPY extractors/ ./extractors/
2026-Mar-21 03:29:28.948527
#38 DONE 0.0s
2026-Mar-21 03:29:28.948527
2026-Mar-21 03:29:28.948527
#39 [api builder  8/11] COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Mar-21 03:29:28.948527
#39 DONE 0.0s
2026-Mar-21 03:29:28.948527
2026-Mar-21 03:29:28.948527
#40 [worker builder  9/10] COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-21 03:29:28.948527
#40 DONE 0.0s
2026-Mar-21 03:29:28.948527
2026-Mar-21 03:29:28.948527
#41 [api builder  9/11] COPY extractors/ ./extractors/
2026-Mar-21 03:29:29.141230
#41 DONE 0.0s
2026-Mar-21 03:29:29.141230
2026-Mar-21 03:29:29.141230
#42 [api builder 10/11] COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-21 03:29:29.141230
#42 DONE 0.0s
2026-Mar-21 03:29:29.141230
2026-Mar-21 03:29:29.141230
#43 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-21 03:29:29.414824
#43 0.274     Updating crates.io index
2026-Mar-21 03:29:35.226612
#43 ...
2026-Mar-21 03:29:35.226612
2026-Mar-21 03:29:35.226612
#44 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-21 03:29:35.226612
#44 0.309     Updating crates.io index
2026-Mar-21 03:29:35.226612
#44 5.697  Downloading crates ...
2026-Mar-21 03:29:35.226612
#44 6.064   Downloaded adler v1.0.2
2026-Mar-21 03:29:35.226612
#44 6.081   Downloaded adler2 v2.0.1
2026-Mar-21 03:29:35.226612
#44 6.166   Downloaded alloc-no-stdlib v2.0.4
2026-Mar-21 03:29:35.226612
#44 6.172   Downloaded dlv-list v0.5.2
2026-Mar-21 03:29:35.226612
#44 6.182   Downloaded document-features v0.2.12
2026-Mar-21 03:29:35.226612
#44 6.241   Downloaded dunce v1.0.5
2026-Mar-21 03:29:35.226612
#44 6.249   Downloaded async-stream-impl v0.3.6
2026-Mar-21 03:29:35.226612
#44 6.276   Downloaded atomic-waker v1.1.2
2026-Mar-21 03:29:35.352317
#44 6.285   Downloaded aws-smithy-observability v0.2.6
2026-Mar-21 03:29:35.352317
#44 6.288   Downloaded futures-task v0.3.32
2026-Mar-21 03:29:35.352317
#44 6.293   Downloaded fastrand v2.3.0
2026-Mar-21 03:29:35.352317
#44 6.301   Downloaded lazycell v1.3.0
2026-Mar-21 03:29:35.352317
#44 6.307   Downloaded home v0.5.12
2026-Mar-21 03:29:35.352317
#44 6.311   Downloaded digest v0.10.7
2026-Mar-21 03:29:35.352317
#44 6.331   Downloaded base64-simd v0.8.0
2026-Mar-21 03:29:35.352317
#44 6.344   Downloaded futures-io v0.3.32
2026-Mar-21 03:29:35.352317
#44 6.357   Downloaded futures-sink v0.3.32
2026-Mar-21 03:29:35.352317
#44 6.361   Downloaded futures-core v0.3.32
2026-Mar-21 03:29:35.352317
#44 6.371   Downloaded atoi v2.0.0
2026-Mar-21 03:29:35.352317
#44 6.402   Downloaded futures-macro v0.3.32
2026-Mar-21 03:29:35.465060
#44 6.413   Downloaded funty v2.0.0
2026-Mar-21 03:29:35.465060
#44 6.416   Downloaded generic-array v0.14.7
2026-Mar-21 03:29:35.465060
#44 6.420   Downloaded cfg_aliases v0.2.1
2026-Mar-21 03:29:35.465060
#44 6.423   Downloaded gzip-header v1.0.0
2026-Mar-21 03:29:35.465060
#44 6.426   Downloaded async-stream v0.3.6
2026-Mar-21 03:29:35.465060
#44 6.437   Downloaded base16ct v0.1.1
2026-Mar-21 03:29:35.465060
#44 6.464   Downloaded http-body v0.4.6
2026-Mar-21 03:29:35.465060
#44 6.468   Downloaded httpdate v1.0.3
2026-Mar-21 03:29:35.465060
#44 6.472   Downloaded form_urlencoded v1.2.2
2026-Mar-21 03:29:35.465060
#44 6.474   Downloaded idna_adapter v1.2.1
2026-Mar-21 03:29:35.465060
#44 6.477   Downloaded heck v0.4.1
2026-Mar-21 03:29:35.465060
#44 6.480   Downloaded heck v0.5.0
2026-Mar-21 03:29:35.465060
#44 6.484   Downloaded equivalent v1.0.2
2026-Mar-21 03:29:35.465060
#44 6.487   Downloaded itoa v1.0.17
2026-Mar-21 03:29:35.465060
#44 6.492   Downloaded errno v0.3.14
2026-Mar-21 03:29:35.465060
#44 6.496   Downloaded foreign-types-shared v0.1.1
2026-Mar-21 03:29:35.465060
#44 6.499   Downloaded foreign-types v0.3.2
2026-Mar-21 03:29:35.465060
#44 6.515   Downloaded bit-set v0.5.3
2026-Mar-21 03:29:35.619009
#44 6.518   Downloaded compression-core v0.4.31
2026-Mar-21 03:29:35.619009
#44 6.525   Downloaded ff v0.12.1
2026-Mar-21 03:29:35.619009
#44 6.529   Downloaded lazy_static v1.5.0
2026-Mar-21 03:29:35.619009
#44 6.533   Downloaded hyper-tls v0.6.0
2026-Mar-21 03:29:35.619009
#44 6.537   Downloaded http-body v1.0.1
2026-Mar-21 03:29:35.619009
#44 6.540   Downloaded alloc-stdlib v0.2.2
2026-Mar-21 03:29:35.619009
#44 6.543   Downloaded const-random-macro v0.1.16
2026-Mar-21 03:29:35.619009
#44 6.545   Downloaded json5 v0.4.1
2026-Mar-21 03:29:35.619009
#44 6.549   Downloaded event-listener-strategy v0.5.4
2026-Mar-21 03:29:35.619009
#44 6.552   Downloaded if_chain v1.0.3
2026-Mar-21 03:29:35.619009
#44 6.555   Downloaded hex v0.4.3
2026-Mar-21 03:29:35.619009
#44 6.668   Downloaded ecdsa v0.14.8
2026-Mar-21 03:29:35.721197
#44 6.673   Downloaded glob v0.3.3
2026-Mar-21 03:29:35.721197
#44 6.676   Downloaded futures-executor v0.3.32
2026-Mar-21 03:29:35.721197
#44 6.679   Downloaded fslock v0.2.1
2026-Mar-21 03:29:35.721197
#44 6.683   Downloaded memoffset v0.9.1
2026-Mar-21 03:29:35.721197
#44 6.687   Downloaded mime v0.3.17
2026-Mar-21 03:29:35.721197
#44 6.690   Downloaded find-msvc-tools v0.1.9
2026-Mar-21 03:29:35.721197
#44 6.694   Downloaded displaydoc v0.2.5
2026-Mar-21 03:29:35.721197
#44 6.699   Downloaded deranged v0.5.8
2026-Mar-21 03:29:35.721197
#44 6.702   Downloaded hyper-rustls v0.24.2
2026-Mar-21 03:29:35.721197
#44 6.708   Downloaded http-body-util v0.1.3
2026-Mar-21 03:29:35.721197
#44 6.712   Downloaded crunchy v0.2.4
2026-Mar-21 03:29:35.721197
#44 6.715   Downloaded debugid v0.8.0
2026-Mar-21 03:29:35.721197
#44 6.718   Downloaded crypto-common v0.1.7
2026-Mar-21 03:29:35.721197
#44 6.720   Downloaded crc v3.4.0
2026-Mar-21 03:29:35.721197
#44 6.724   Downloaded cpufeatures v0.2.17
2026-Mar-21 03:29:35.721197
#44 6.727   Downloaded lock_api v0.4.14
2026-Mar-21 03:29:35.721197
#44 6.731   Downloaded libloading v0.8.9
2026-Mar-21 03:29:35.721197
#44 6.735   Downloaded crossbeam-queue v0.3.12
2026-Mar-21 03:29:35.721197
#44 6.739   Downloaded jobserver v0.1.34
2026-Mar-21 03:29:35.721197
#44 6.743   Downloaded fs_extra v1.3.0
2026-Mar-21 03:29:35.721197
#44 6.747   Downloaded either v1.15.0
2026-Mar-21 03:29:35.721197
#44 6.750   Downloaded const-random v0.1.18
2026-Mar-21 03:29:35.721197
#44 6.753   Downloaded cfg-if v1.0.4
2026-Mar-21 03:29:35.721197
#44 6.756   Downloaded block-buffer v0.10.4
2026-Mar-21 03:29:35.721197
#44 6.758   Downloaded cooked-waker v5.0.0
2026-Mar-21 03:29:35.721197
#44 6.761   Downloaded base64-simd v0.7.0
2026-Mar-21 03:29:35.721197
#44 6.764   Downloaded hashlink v0.8.4
2026-Mar-21 03:29:35.721197
#44 6.768   Downloaded group v0.12.1
2026-Mar-21 03:29:35.721197
#44 6.771   Downloaded futures-timer v3.0.3
2026-Mar-21 03:29:35.824158
#44 6.774   Downloaded foldhash v0.1.5
2026-Mar-21 03:29:35.824158
#44 6.777   Downloaded fnv v1.0.7
2026-Mar-21 03:29:35.824158
#44 6.780   Downloaded deno_unsync v0.4.4
2026-Mar-21 03:29:35.824158
#44 6.784   Downloaded dotenvy v0.15.7
2026-Mar-21 03:29:35.824158
#44 6.819   Downloaded no-std-compat v0.4.1
2026-Mar-21 03:29:35.824158
#44 6.821   Downloaded futures-channel v0.3.32
2026-Mar-21 03:29:35.824158
#44 6.825   Downloaded getrandom v0.2.17
2026-Mar-21 03:29:35.824158
#44 6.831   Downloaded md-5 v0.10.6
2026-Mar-21 03:29:35.824158
#44 6.836   Downloaded openssl-probe v0.2.1
2026-Mar-21 03:29:35.824158
#44 6.839   Downloaded hyper-rustls v0.27.7
2026-Mar-21 03:29:35.824158
#44 6.844   Downloaded httparse v1.10.1
2026-Mar-21 03:29:35.824158
#44 6.849   Downloaded getrandom v0.3.4
2026-Mar-21 03:29:35.824158
#44 6.856   Downloaded matchit v0.8.4
2026-Mar-21 03:29:35.824158
#44 6.861   Downloaded matchers v0.2.0
2026-Mar-21 03:29:35.824158
#44 6.863   Downloaded aws-smithy-http v0.62.6
2026-Mar-21 03:29:35.824158
#44 6.867   Downloaded autocfg v1.5.0
2026-Mar-21 03:29:35.824158
#44 6.870   Downloaded arraydeque v0.5.1
2026-Mar-21 03:29:35.824158
#44 6.874   Downloaded parking v2.2.1
2026-Mar-21 03:29:35.925660
#44 6.877   Downloaded lru v0.12.5
2026-Mar-21 03:29:35.925660
#44 6.879   Downloaded concurrent-queue v2.5.0
2026-Mar-21 03:29:35.925660
#44 6.884   Downloaded num-integer v0.1.46
2026-Mar-21 03:29:35.925660
#44 6.888   Downloaded lru-slab v0.1.2
2026-Mar-21 03:29:35.925660
#44 6.891   Downloaded elliptic-curve v0.12.3
2026-Mar-21 03:29:35.925660
#44 6.899   Downloaded nonzero_ext v0.3.0
2026-Mar-21 03:29:35.925660
#44 6.907   Downloaded jsonwebtoken v9.3.1
2026-Mar-21 03:29:35.925660
#44 6.914   Downloaded icu_properties v2.1.2
2026-Mar-21 03:29:35.925660
#44 6.918   Downloaded dashmap v6.1.0
2026-Mar-21 03:29:35.925660
#44 6.923   Downloaded icu_provider v2.1.1
2026-Mar-21 03:29:35.925660
#44 6.927   Downloaded bincode v1.3.3
2026-Mar-21 03:29:35.925660
#44 6.932   Downloaded aws-smithy-http v0.63.6
2026-Mar-21 03:29:35.925660
#44 6.936   Downloaded ordered-multimap v0.7.3
2026-Mar-21 03:29:35.925660
#44 6.940   Downloaded getrandom v0.4.1
2026-Mar-21 03:29:35.925660
#44 6.946   Downloaded log v0.4.29
2026-Mar-21 03:29:35.925660
#44 6.950   Downloaded miniz_oxide v0.7.4
2026-Mar-21 03:29:35.925660
#44 6.954   Downloaded compression-codecs v0.4.37
2026-Mar-21 03:29:35.925660
#44 6.962   Downloaded paste v1.0.15
2026-Mar-21 03:29:35.925660
#44 6.969   Downloaded pem-rfc7468 v0.7.0
2026-Mar-21 03:29:35.925660
#44 6.972   Downloaded cmake v0.1.57
2026-Mar-21 03:29:35.925660
#44 6.975   Downloaded icu_normalizer_data v2.1.1
2026-Mar-21 03:29:36.028835
#44 6.979   Downloaded byteorder v1.5.0
2026-Mar-21 03:29:36.028835
#44 6.983   Downloaded icu_normalizer v2.1.1
2026-Mar-21 03:29:36.028835
#44 6.990   Downloaded data-encoding v2.10.0
2026-Mar-21 03:29:36.028835
#44 6.994   Downloaded aws-smithy-eventstream v0.60.20
2026-Mar-21 03:29:36.028835
#44 6.999   Downloaded icu_locale_core v2.1.1
2026-Mar-21 03:29:36.028835
#44 7.012   Downloaded convert_case v0.6.0
2026-Mar-21 03:29:36.028835
#44 7.017   Downloaded flume v0.11.1
2026-Mar-21 03:29:36.028835
#44 7.023   Downloaded futures-intrusive v0.5.0
2026-Mar-21 03:29:36.028835
#44 7.029   Downloaded aws-smithy-json v0.62.5
2026-Mar-21 03:29:36.028835
#44 7.033   Downloaded async-lock v3.4.2
2026-Mar-21 03:29:36.028835
#44 7.037   Downloaded num-traits v0.2.19
2026-Mar-21 03:29:36.028835
#44 7.043   Downloaded pin-utils v0.1.0
2026-Mar-21 03:29:36.028835
#44 7.046   Downloaded parking_lot v0.12.5
2026-Mar-21 03:29:36.028835
#44 7.052   Downloaded proc-macro-error-attr v1.0.4
2026-Mar-21 03:29:36.028835
#44 7.054   Downloaded powerfmt v0.2.0
2026-Mar-21 03:29:36.028835
#44 7.057   Downloaded pem v3.0.6
2026-Mar-21 03:29:36.028835
#44 7.060   Downloaded bitflags v2.11.0
2026-Mar-21 03:29:36.028835
#44 7.069   Downloaded once_cell v1.21.3
2026-Mar-21 03:29:36.028835
#44 7.074   Downloaded cexpr v0.6.0
2026-Mar-21 03:29:36.028835
#44 7.078   Downloaded rand_chacha v0.3.1
2026-Mar-21 03:29:36.233712
#44 7.081   Downloaded parking_lot_core v0.9.12
2026-Mar-21 03:29:36.233712
#44 7.086   Downloaded miniz_oxide v0.8.9
2026-Mar-21 03:29:36.233712
#44 7.091   Downloaded hmac v0.12.1
2026-Mar-21 03:29:36.233712
#44 7.096   Downloaded bytes-utils v0.1.4
2026-Mar-21 03:29:36.233712
#44 7.099   Downloaded flate2 v1.1.9
2026-Mar-21 03:29:36.233712
#44 7.107   Downloaded bit-vec v0.6.3
2026-Mar-21 03:29:36.233712
#44 7.109   Downloaded axum-core v0.5.6
2026-Mar-21 03:29:36.233712
#44 7.114   Downloaded aws-smithy-xml v0.60.15
2026-Mar-21 03:29:36.233712
#44 7.116   Downloaded aws-smithy-query v0.60.15
2026-Mar-21 03:29:36.233712
#44 7.118   Downloaded aws-smithy-checksums v0.63.12
2026-Mar-21 03:29:36.233712
#44 7.121   Downloaded aws-smithy-async v1.2.14
2026-Mar-21 03:29:36.233712
#44 7.125   Downloaded aws-credential-types v1.2.14
2026-Mar-21 03:29:36.233712
#44 7.128   Downloaded async-trait v0.1.89
2026-Mar-21 03:29:36.233712
#44 7.135   Downloaded futures v0.3.32
2026-Mar-21 03:29:36.233712
#44 7.143   Downloaded crc-catalog v2.4.0
2026-Mar-21 03:29:36.233712
#44 7.146   Downloaded event-listener v5.4.1
2026-Mar-21 03:29:36.233712
#44 7.150   Downloaded openssl-macros v0.1.1
2026-Mar-21 03:29:36.233712
#44 7.152   Downloaded num_cpus v1.17.0
2026-Mar-21 03:29:36.233712
#44 7.157   Downloaded num-iter v0.1.45
2026-Mar-21 03:29:36.233712
#44 7.159   Downloaded num-conv v0.2.0
2026-Mar-21 03:29:36.233712
#44 7.161   Downloaded native-tls v0.2.18
2026-Mar-21 03:29:36.233712
#44 7.164   Downloaded litrs v1.0.0
2026-Mar-21 03:29:36.233712
#44 7.169   Downloaded ipnet v2.11.0
2026-Mar-21 03:29:36.233712
#44 7.172   Downloaded hashlink v0.10.0
2026-Mar-21 03:29:36.233712
#44 7.283   Downloaded rand_chacha v0.9.0
2026-Mar-21 03:29:36.337207
#44 7.287   Downloaded icu_collections v2.1.1
2026-Mar-21 03:29:36.337207
#44 7.297   Downloaded rustc-hash v2.1.1
2026-Mar-21 03:29:36.337207
#44 7.301   Downloaded rustc-hash v1.1.0
2026-Mar-21 03:29:36.337207
#44 7.303   Downloaded serde_urlencoded v0.7.1
2026-Mar-21 03:29:36.337207
#44 7.306   Downloaded rfc6979 v0.3.1
2026-Mar-21 03:29:36.337207
#44 7.308   Downloaded rand_core v0.9.5
2026-Mar-21 03:29:36.337207
#44 7.310   Downloaded pkcs1 v0.7.5
2026-Mar-21 03:29:36.337207
#44 7.315   Downloaded pin-project-lite v0.2.16
2026-Mar-21 03:29:36.337207
#44 7.324   Downloaded pin-project-internal v1.1.10
2026-Mar-21 03:29:36.337207
#44 7.326   Downloaded p256 v0.11.1
2026-Mar-21 03:29:36.337207
#44 7.331   Downloaded openssl-sys v0.9.111
2026-Mar-21 03:29:36.337207
#44 7.339   Downloaded indexmap v2.13.0
2026-Mar-21 03:29:36.337207
#44 7.346   Downloaded http v1.4.0
2026-Mar-21 03:29:36.337207
#44 7.352   Downloaded http v0.2.12
2026-Mar-21 03:29:36.337207
#44 7.358   Downloaded quote v1.0.44
2026-Mar-21 03:29:36.337207
#44 7.362   Downloaded proc-macro-error v1.0.4
2026-Mar-21 03:29:36.337207
#44 7.368   Downloaded ppv-lite86 v0.2.21
2026-Mar-21 03:29:36.337207
#44 7.371   Downloaded potential_utf v0.1.4
2026-Mar-21 03:29:36.337207
#44 7.374   Downloaded pkg-config v0.3.32
2026-Mar-21 03:29:36.337207
#44 7.377   Downloaded pkcs8 v0.10.2
2026-Mar-21 03:29:36.337207
#44 7.382   Downloaded pkcs8 v0.9.0
2026-Mar-21 03:29:36.337207
#44 7.386   Downloaded pest_generator v2.8.6
2026-Mar-21 03:29:36.438707
#44 7.389   Downloaded pest_derive v2.8.6
2026-Mar-21 03:29:36.438707
#44 7.394   Downloaded memchr v2.8.0
2026-Mar-21 03:29:36.438707
#44 7.403   Downloaded simd-abstraction v0.7.1
2026-Mar-21 03:29:36.438707
#44 7.406   Downloaded rust-ini v0.20.0
2026-Mar-21 03:29:36.438707
#44 7.409   Downloaded sha1_smol v1.0.1
2026-Mar-21 03:29:36.438707
#44 7.411   Downloaded sha1 v0.10.6
2026-Mar-21 03:29:36.441758
#44 7.414   Downloaded pin-project v1.1.10
2026-Mar-21 03:29:36.441758
#44 7.436   Downloaded rustc_version v0.2.3
2026-Mar-21 03:29:36.441758
#44 7.438   Downloaded hyper-util v0.1.20
2026-Mar-21 03:29:36.441758
#44 7.449   Downloaded radium v0.7.0
2026-Mar-21 03:29:36.441758
#44 7.451   Downloaded base64ct v1.8.3
2026-Mar-21 03:29:36.441758
#44 7.455   Downloaded aws-types v1.3.14
2026-Mar-21 03:29:36.441758
#44 7.458   Downloaded arc-swap v1.8.2
2026-Mar-21 03:29:36.441758
#44 7.464   Downloaded proc-macro-rules-macros v0.4.0
2026-Mar-21 03:29:36.441758
#44 7.466   Downloaded mio v1.1.1
2026-Mar-21 03:29:36.441758
#44 7.477   Downloaded crossbeam-utils v0.8.21
2026-Mar-21 03:29:36.441758
#44 7.481   Downloaded proc-macro-rules v0.4.0
2026-Mar-21 03:29:36.441758
#44 7.484   Downloaded percent-encoding v2.3.2
2026-Mar-21 03:29:36.441758
#44 7.486   Downloaded pathdiff v0.2.3
2026-Mar-21 03:29:36.441758
#44 7.488   Downloaded outref v0.5.2
2026-Mar-21 03:29:36.543967
#44 7.491   Downloaded outref v0.1.0
2026-Mar-21 03:29:36.543967
#44 7.493   Downloaded nu-ansi-term v0.50.3
2026-Mar-21 03:29:36.543967
#44 7.498   Downloaded litemap v0.8.1
2026-Mar-21 03:29:36.543967
#44 7.502   Downloaded crypto-bigint v0.4.9
2026-Mar-21 03:29:36.543967
#44 7.509   Downloaded serde_path_to_error v0.1.20
2026-Mar-21 03:29:36.543967
#44 7.513   Downloaded cookie v0.18.1
2026-Mar-21 03:29:36.543967
#44 7.518   Downloaded clang-sys v1.8.1
2026-Mar-21 03:29:36.543967
#44 7.521   Downloaded crossbeam-epoch v0.9.18
2026-Mar-21 03:29:36.543967
#44 7.526   Downloaded semver-parser v0.7.0
2026-Mar-21 03:29:36.543967
#44 7.529   Downloaded crc32fast v1.5.0
2026-Mar-21 03:29:36.543967
#44 7.532   Downloaded cookie_store v0.22.1
2026-Mar-21 03:29:36.543967
#44 7.535   Downloaded const-oid v0.9.6
2026-Mar-21 03:29:36.543967
#44 7.538   Downloaded aws-smithy-json v0.61.9
2026-Mar-21 03:29:36.543967
#44 7.541   Downloaded anyhow v1.0.102
2026-Mar-21 03:29:36.543967
#44 7.548   Downloaded allocator-api2 v0.2.21
2026-Mar-21 03:29:36.543967
#44 7.552   Downloaded ahash v0.8.12
2026-Mar-21 03:29:36.543967
#44 7.585   Downloaded sha2 v0.10.9
2026-Mar-21 03:29:36.543967
#44 7.593   Downloaded pest_meta v2.8.6
2026-Mar-21 03:29:36.659782
#44 7.597   Downloaded aws-smithy-runtime-api v1.11.6
2026-Mar-21 03:29:36.659782
#44 7.634   Downloaded rustc_version v0.4.1
2026-Mar-21 03:29:36.659782
#44 7.641   Downloaded strum v0.25.0
2026-Mar-21 03:29:36.659782
#44 7.645   Downloaded slab v0.4.12
2026-Mar-21 03:29:36.659782
#44 7.647   Downloaded simd-adler32 v0.3.8
2026-Mar-21 03:29:36.659782
#44 7.652   Downloaded prettyplease v0.2.37
2026-Mar-21 03:29:36.659782
#44 7.670   Downloaded simple_asn1 v0.6.4
2026-Mar-21 03:29:36.659782
#44 7.675   Downloaded subtle v2.6.1
2026-Mar-21 03:29:36.659782
#44 7.688   Downloaded iri-string v0.7.10
2026-Mar-21 03:29:36.659782
#44 7.709   Downloaded hashbrown v0.15.5
2026-Mar-21 03:29:36.766432
#44 7.718   Downloaded hashbrown v0.14.5
2026-Mar-21 03:29:36.766432
#44 7.727   Downloaded sync_wrapper v1.0.2
2026-Mar-21 03:29:36.766432
#44 7.732   Downloaded tap v1.0.1
2026-Mar-21 03:29:36.766432
#44 7.765   Downloaded itertools v0.13.0
2026-Mar-21 03:29:36.766432
#44 7.774   Downloaded psl-types v2.0.11
2026-Mar-21 03:29:36.766432
#44 7.776   Downloaded idna v1.1.0
2026-Mar-21 03:29:36.766432
#44 7.781   Downloaded itertools v0.12.1
2026-Mar-21 03:29:36.766432
#44 7.791   Downloaded tagptr v0.2.0
2026-Mar-21 03:29:36.766432
#44 7.794   Downloaded sqlx-macros-core v0.8.6
2026-Mar-21 03:29:36.766432
#44 7.816   Downloaded rand_core v0.6.4
2026-Mar-21 03:29:36.873897
#44 7.820   Downloaded ron v0.8.1
2026-Mar-21 03:29:36.873897
#44 7.829   Downloaded serde_spanned v0.6.9
2026-Mar-21 03:29:36.873897
#44 7.834   Downloaded static_assertions v1.1.0
2026-Mar-21 03:29:36.873897
#44 7.842   Downloaded stringprep v0.1.5
2026-Mar-21 03:29:36.873897
#44 7.845   Downloaded signature v1.6.4
2026-Mar-21 03:29:36.873897
#44 7.847   Downloaded nom v7.1.3
2026-Mar-21 03:29:36.873897
#44 7.855   Downloaded strum_macros v0.25.3
2026-Mar-21 03:29:36.873897
#44 7.859   Downloaded rand v0.8.5
2026-Mar-21 03:29:36.873897
#44 7.865   Downloaded smallvec v1.15.1
2026-Mar-21 03:29:36.873897
#44 7.868   Downloaded governor v0.8.1
2026-Mar-21 03:29:36.873897
#44 7.874   Downloaded signature v2.2.0
2026-Mar-21 03:29:36.873897
#44 7.882   Downloaded spki v0.6.0
2026-Mar-21 03:29:36.873897
#44 7.891   Downloaded synstructure v0.13.2
2026-Mar-21 03:29:36.873897
#44 7.897   Downloaded tinyvec_macros v0.1.1
2026-Mar-21 03:29:36.873897
#44 7.901   Downloaded time-core v0.1.8
2026-Mar-21 03:29:36.873897
#44 7.908   Downloaded config v0.14.1
2026-Mar-21 03:29:36.873897
#44 7.923   Downloaded spin v0.9.8
2026-Mar-21 03:29:36.981584
#44 7.929   Downloaded libm v0.2.16
2026-Mar-21 03:29:36.981584
#44 7.947   Downloaded icu_properties_data v2.1.2
2026-Mar-21 03:29:36.981584
#44 7.963   Downloaded base64 v0.22.1
2026-Mar-21 03:29:36.981584
#44 8.008   Downloaded der v0.6.1
2026-Mar-21 03:29:36.981584
#44 8.016   Downloaded rand v0.9.2
2026-Mar-21 03:29:36.981584
#44 8.021   Downloaded num-bigint v0.4.6
2026-Mar-21 03:29:36.981584
#44 8.031   Downloaded toml_datetime v0.6.11
2026-Mar-21 03:29:37.082846
#44 8.045   Downloaded hashbrown v0.16.1
2026-Mar-21 03:29:37.082846
#44 8.053   Downloaded stable_deref_trait v1.2.1
2026-Mar-21 03:29:37.082846
#44 8.055   Downloaded sourcemap v8.0.1
2026-Mar-21 03:29:37.082846
#44 8.059   Downloaded socket2 v0.6.2
2026-Mar-21 03:29:37.082846
#44 8.094   Downloaded futures-util v0.3.32
2026-Mar-21 03:29:37.082846
#44 8.114   Downloaded tinystr v0.8.2
2026-Mar-21 03:29:37.082846
#44 8.117   Downloaded time-macros v0.2.27
2026-Mar-21 03:29:37.082846
#44 8.120   Downloaded hkdf v0.12.4
2026-Mar-21 03:29:37.082846
#44 8.123   Downloaded aws-sdk-sso v1.96.0
2026-Mar-21 03:29:37.082846
#44 8.132   Downloaded tower-layer v0.3.3
2026-Mar-21 03:29:37.195355
#44 8.134   Downloaded thread_local v1.1.9
2026-Mar-21 03:29:37.195355
#44 8.137   Downloaded thiserror-impl v2.0.18
2026-Mar-21 03:29:37.195355
#44 8.139   Downloaded thiserror-impl v1.0.69
2026-Mar-21 03:29:37.195355
#44 8.141   Downloaded h2 v0.4.13
2026-Mar-21 03:29:37.195355
#44 8.149   Downloaded pest v2.8.6
2026-Mar-21 03:29:37.195355
#44 8.155   Downloaded semver v0.9.0
2026-Mar-21 03:29:37.195355
#44 8.157   Downloaded rsa v0.9.10
2026-Mar-21 03:29:37.195355
#44 8.164   Downloaded rustversion v1.0.22
2026-Mar-21 03:29:37.195355
#44 8.169   Downloaded thiserror v1.0.69
2026-Mar-21 03:29:37.195355
#44 8.179   Downloaded sct v0.7.1
2026-Mar-21 03:29:37.195355
#44 8.186   Downloaded sec1 v0.3.0
2026-Mar-21 03:29:37.195355
#44 8.189   Downloaded scopeguard v1.2.0
2026-Mar-21 03:29:37.195355
#44 8.191   Downloaded rustls-native-certs v0.8.3
2026-Mar-21 03:29:37.195355
#44 8.195   Downloaded base64 v0.21.7
2026-Mar-21 03:29:37.195355
#44 8.202   Downloaded toml_write v0.1.2
2026-Mar-21 03:29:37.195355
#44 8.204   Downloaded tokio-macros v2.6.0
2026-Mar-21 03:29:37.195355
#44 8.205   Downloaded semver v1.0.27
2026-Mar-21 03:29:37.195355
#44 8.208   Downloaded aws-smithy-types v1.4.6
2026-Mar-21 03:29:37.195355
#44 8.215   Downloaded want v0.3.1
2026-Mar-21 03:29:37.195355
#44 8.222   Downloaded hyper v1.8.1
2026-Mar-21 03:29:37.195355
#44 8.245   Downloaded tokio-native-tls v0.3.1
2026-Mar-21 03:29:37.296459
#44 8.249   Downloaded quanta v0.12.6
2026-Mar-21 03:29:37.296459
#44 8.264   Downloaded which v4.4.2
2026-Mar-21 03:29:37.296459
#44 8.267   Downloaded thiserror v2.0.18
2026-Mar-21 03:29:37.296459
#44 8.279   Downloaded spki v0.7.3
2026-Mar-21 03:29:37.296459
#44 8.283   Downloaded sharded-slab v0.1.7
2026-Mar-21 03:29:37.296459
#44 8.288   Downloaded tower-service v0.3.3
2026-Mar-21 03:29:37.296459
#44 8.289   Downloaded signal-hook-registry v1.4.8
2026-Mar-21 03:29:37.296459
#44 8.292   Downloaded sqlx-macros v0.8.6
2026-Mar-21 03:29:37.296459
#44 8.294   Downloaded shlex v1.3.0
2026-Mar-21 03:29:37.296459
#44 8.296   Downloaded quinn-udp v0.5.14
2026-Mar-21 03:29:37.296459
#44 8.298   Downloaded tiny-keccak v2.0.2
2026-Mar-21 03:29:37.296459
#44 8.302   Downloaded regex-lite v0.1.9
2026-Mar-21 03:29:37.296459
#44 8.308   Downloaded raw-cpuid v11.6.0
2026-Mar-21 03:29:37.296459
#44 8.341   Downloaded cc v1.2.56
2026-Mar-21 03:29:37.296459
#44 8.346   Downloaded tracing-log v0.2.0
2026-Mar-21 03:29:37.461184
#44 8.348   Downloaded bytes v1.11.1
2026-Mar-21 03:29:37.461184
#44 8.355   Downloaded yoke-derive v0.8.1
2026-Mar-21 03:29:37.461184
#44 8.357   Downloaded crypto-bigint v0.5.5
2026-Mar-21 03:29:37.461184
#44 8.371   Downloaded hyper v0.14.32
2026-Mar-21 03:29:37.461184
#44 8.382   Downloaded h2 v0.3.27
2026-Mar-21 03:29:37.461184
#44 8.391   Downloaded zerofrom-derive v0.1.6
2026-Mar-21 03:29:37.461184
#44 8.392   Downloaded zerofrom v0.1.6
2026-Mar-21 03:29:37.461184
#44 8.394   Downloaded tokio-rustls v0.24.1
2026-Mar-21 03:29:37.461184
#44 8.397   Downloaded deno_ops v0.176.0
2026-Mar-21 03:29:37.461184
#44 8.410   Downloaded der v0.7.10
2026-Mar-21 03:29:37.461184
#44 8.417   Downloaded tinyvec v1.10.0
2026-Mar-21 03:29:37.461184
#44 8.421   Downloaded tracing-attributes v0.1.31
2026-Mar-21 03:29:37.461184
#44 8.425   Downloaded aws-sigv4 v1.4.2
2026-Mar-21 03:29:37.461184
#44 8.511   Downloaded yoke v0.8.1
2026-Mar-21 03:29:37.562483
#44 8.514   Downloaded async-compression v0.4.41
2026-Mar-21 03:29:37.562483
#44 8.525   Downloaded xmlparser v0.13.6
2026-Mar-21 03:29:37.562483
#44 8.529   Downloaded whoami v1.6.1
2026-Mar-21 03:29:37.562483
#44 8.533   Downloaded rustls-pki-types v1.14.0
2026-Mar-21 03:29:37.562483
#44 8.537   Downloaded aws-runtime v1.7.2
2026-Mar-21 03:29:37.562483
#44 8.545   Downloaded tokio-stream v0.1.18
2026-Mar-21 03:29:37.562483
#44 8.551   Downloaded toml v0.8.23
2026-Mar-21 03:29:37.562483
#44 8.555   Downloaded zeroize v1.8.2
2026-Mar-21 03:29:37.562483
#44 8.556   Downloaded untrusted v0.9.0
2026-Mar-21 03:29:37.562483
#44 8.559   Downloaded tokio-rustls v0.26.4
2026-Mar-21 03:29:37.562483
#44 8.568   Downloaded tracing-core v0.1.36
2026-Mar-21 03:29:37.562483
#44 8.597   Downloaded zerovec-derive v0.11.2
2026-Mar-21 03:29:37.562483
#44 8.612   Downloaded proc-macro2 v1.0.106
2026-Mar-21 03:29:37.684538
#44 8.617   Downloaded zmij v1.0.21
2026-Mar-21 03:29:37.684538
#44 8.634   Downloaded sqlx-core v0.8.6
2026-Mar-21 03:29:37.684538
#44 8.645   Downloaded serde_v8 v0.209.0
2026-Mar-21 03:29:37.684538
#44 8.649   Downloaded try-lock v0.2.5
2026-Mar-21 03:29:37.684538
#44 8.696   Downloaded utf8_iter v1.0.4
2026-Mar-21 03:29:37.684538
#44 8.734   Downloaded toml_edit v0.22.27
2026-Mar-21 03:29:37.801411
#44 8.741   Downloaded version_check v0.9.5
2026-Mar-21 03:29:37.801411
#44 8.746   Downloaded webpki-roots v0.26.11
2026-Mar-21 03:29:37.801411
#44 8.774   Downloaded reqwest v0.12.28
2026-Mar-21 03:29:37.801411
#44 8.803   Downloaded portable-atomic v1.13.1
2026-Mar-21 03:29:37.801411
#44 8.851   Downloaded regex v1.12.3
2026-Mar-21 03:29:37.919627
#44 8.861   Downloaded sqlx-postgres v0.8.6
2026-Mar-21 03:29:37.919627
#44 8.873   Downloaded aws-sdk-ssooidc v1.98.0
2026-Mar-21 03:29:37.919627
#44 8.884   Downloaded aws-smithy-http-client v1.1.12
2026-Mar-21 03:29:37.919627
#44 8.893   Downloaded serde_json v1.0.149
2026-Mar-21 03:29:37.919627
#44 8.903   Downloaded ryu v1.0.23
2026-Mar-21 03:29:37.919627
#44 8.913   Downloaded zerotrie v0.2.3
2026-Mar-21 03:29:37.919627
#44 8.918   Downloaded tower v0.4.13
2026-Mar-21 03:29:37.919627
#44 8.933   Downloaded urlencoding v2.1.3
2026-Mar-21 03:29:37.919627
#44 8.934   Downloaded tower v0.5.3
2026-Mar-21 03:29:37.919627
#44 8.948   Downloaded serde_core v1.0.228
2026-Mar-21 03:29:37.919627
#44 8.969   Downloaded minimal-lexical v0.2.1
2026-Mar-21 03:29:38.031707
#44 9.002   Downloaded spinning_top v0.3.0
2026-Mar-21 03:29:38.031707
#44 9.004   Downloaded socket2 v0.5.10
2026-Mar-21 03:29:38.031707
#44 9.051   Downloaded tower-http v0.6.8
2026-Mar-21 03:29:38.031707
#44 9.081   Downloaded which v6.0.3
2026-Mar-21 03:29:38.146149
#44 9.084   Downloaded sqlx v0.8.6
2026-Mar-21 03:29:38.146149
#44 9.102   Downloaded tokio-util v0.7.18
2026-Mar-21 03:29:38.146149
#44 9.112   Downloaded crossbeam-channel v0.5.15
2026-Mar-21 03:29:38.146149
#44 9.117   Downloaded web-time v1.1.0
2026-Mar-21 03:29:38.146149
#44 9.120   Downloaded vsimd v0.8.0
2026-Mar-21 03:29:38.146149
#44 9.138   Downloaded writeable v0.6.2
2026-Mar-21 03:29:38.146149
#44 9.196   Downloaded wyz v0.5.1
2026-Mar-21 03:29:38.257271
#44 9.202   Downloaded yaml-rust2 v0.8.1
2026-Mar-21 03:29:38.257271
#44 9.252   Downloaded zerovec v0.11.5
2026-Mar-21 03:29:38.257271
#44 9.260   Downloaded redis v0.27.6
2026-Mar-21 03:29:38.257271
#44 9.272   Downloaded publicsuffix v2.3.0
2026-Mar-21 03:29:38.257271
#44 9.275   Downloaded unicode-id-start v1.4.0
2026-Mar-21 03:29:38.257271
#44 9.280   Downloaded ucd-trie v0.1.7
2026-Mar-21 03:29:38.257271
#44 9.282   Downloaded serde_derive v1.0.228
2026-Mar-21 03:29:38.257271
#44 9.286   Downloaded quinn v0.11.9
2026-Mar-21 03:29:38.257271
#44 9.291   Downloaded openssl v0.10.75
2026-Mar-21 03:29:38.257271
#44 9.307   Downloaded combine v4.6.7
2026-Mar-21 03:29:38.357376
#44 9.315   Downloaded rustls v0.21.12
2026-Mar-21 03:29:38.357376
#44 9.331   Downloaded winnow v0.7.14
2026-Mar-21 03:29:38.357376
#44 9.344   Downloaded unicode-properties v0.1.4
2026-Mar-21 03:29:38.357376
#44 9.393   Downloaded syn v1.0.109
2026-Mar-21 03:29:38.357376
#44 9.407   Downloaded sqlx-mysql v0.8.6
2026-Mar-21 03:29:38.482005
#44 9.416   Downloaded time v0.3.47
2026-Mar-21 03:29:38.482005
#44 9.433   Downloaded rustls-webpki v0.103.9
2026-Mar-21 03:29:38.482005
#44 9.475   Downloaded serde v1.0.228
2026-Mar-21 03:29:38.482005
#44 9.496   Downloaded unicode-ident v1.0.24
2026-Mar-21 03:29:38.482005
#44 9.499   Downloaded unicode-bidi v0.3.18
2026-Mar-21 03:29:38.482005
#44 9.532   Downloaded aho-corasick v1.1.4
2026-Mar-21 03:29:38.613106
#44 9.538   Downloaded uuid v1.21.0
2026-Mar-21 03:29:38.613106
#44 9.565   Downloaded regex-syntax v0.8.10
2026-Mar-21 03:29:38.613106
#44 9.574   Downloaded zerocopy v0.8.39
2026-Mar-21 03:29:38.613106
#44 9.607   Downloaded rustls v0.23.37
2026-Mar-21 03:29:38.613106
#44 9.621   Downloaded sqlx-sqlite v0.8.6
2026-Mar-21 03:29:38.613106
#44 9.627   Downloaded rustix v0.38.44
2026-Mar-21 03:29:38.613106
#44 9.663   Downloaded num-bigint-dig v0.8.6
2026-Mar-21 03:29:38.746140
#44 9.668   Downloaded aws-config v1.8.15
2026-Mar-21 03:29:38.746140
#44 9.677   Downloaded aws-smithy-runtime v1.10.3
2026-Mar-21 03:29:38.746140
#44 9.726   Downloaded utoipa v4.2.3
2026-Mar-21 03:29:38.846420
#44 9.810   Downloaded tracing v0.1.44
2026-Mar-21 03:29:38.846420
#44 9.843   Downloaded typenum v1.19.0
2026-Mar-21 03:29:38.846420
#44 9.884   Downloaded url v2.5.8
2026-Mar-21 03:29:39.012698
#44 9.939   Downloaded crc-fast v1.6.0
2026-Mar-21 03:29:39.012698
#44 9.949   Downloaded axum v0.8.8
2026-Mar-21 03:29:39.012698
#44 9.971   Downloaded regex-automata v0.4.14
2026-Mar-21 03:29:39.012698
#44 ...
2026-Mar-21 03:29:39.012698
2026-Mar-21 03:29:39.012698
#45 [frontend builder 5/8] RUN npm install
2026-Mar-21 03:29:39.123202
#45 ...
2026-Mar-21 03:29:39.123202
2026-Mar-21 03:29:39.123202
#43 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-21 03:29:39.123202
#43 9.454  Downloading crates ...
2026-Mar-21 03:29:39.123202
#43 9.756   Downloaded adler v1.0.2
2026-Mar-21 03:29:39.123202
#43 9.770   Downloaded adler2 v2.0.1
2026-Mar-21 03:29:39.123202
#43 9.782   Downloaded alloc-no-stdlib v2.0.4
2026-Mar-21 03:29:39.123202
#43 9.856   Downloaded alloc-stdlib v0.2.2
2026-Mar-21 03:29:39.123202
#43 9.922   Downloaded async-stream-impl v0.3.6
2026-Mar-21 03:29:39.123202
#43 10.02   Downloaded atoi v2.0.0
2026-Mar-21 03:29:39.123202
#43 10.03   Downloaded dunce v1.0.5
2026-Mar-21 03:29:39.123202
#43 10.03   Downloaded dotenvy v0.15.7
2026-Mar-21 03:29:39.123202
#43 10.04   Downloaded async-stream v0.3.6
2026-Mar-21 03:29:39.123202
#43 10.05   Downloaded atomic-waker v1.1.2
2026-Mar-21 03:29:39.123202
#43 10.10   Downloaded crunchy v0.2.4
2026-Mar-21 03:29:39.123202
#43 10.10   Downloaded aws-smithy-query v0.60.15
2026-Mar-21 03:29:39.123202
#43 10.11   Downloaded foreign-types-shared v0.1.1
2026-Mar-21 03:29:39.123202
#43 10.12   Downloaded cfg_aliases v0.2.1
2026-Mar-21 03:29:39.123202
#43 10.12   Downloaded bit-set v0.5.3
2026-Mar-21 03:29:39.123202
#43 10.12   Downloaded foreign-types v0.3.2
2026-Mar-21 03:29:39.123202
#43 10.13   Downloaded deno_unsync v0.4.4
2026-Mar-21 03:29:39.237326
#43 10.14   Downloaded const-random v0.1.18
2026-Mar-21 03:29:39.237326
#43 10.14   Downloaded document-features v0.2.12
2026-Mar-21 03:29:39.237326
#43 10.15   Downloaded futures-core v0.3.32
2026-Mar-21 03:29:39.237326
#43 10.19   Downloaded crypto-common v0.1.7
2026-Mar-21 03:29:39.237326
#43 10.19   Downloaded fnv v1.0.7
2026-Mar-21 03:29:39.237326
#43 10.20   Downloaded base64-simd v0.7.0
2026-Mar-21 03:29:39.237326
#43 10.20   Downloaded funty v2.0.0
2026-Mar-21 03:29:39.237326
#43 10.21   Downloaded ahash v0.8.12
2026-Mar-21 03:29:39.237326
#43 10.21   Downloaded fslock v0.2.1
2026-Mar-21 03:29:39.237326
#43 10.22   Downloaded form_urlencoded v1.2.2
2026-Mar-21 03:29:39.237326
#43 10.22   Downloaded errno v0.3.14
2026-Mar-21 03:29:39.237326
#43 10.23   Downloaded if_chain v1.0.3
2026-Mar-21 03:29:39.339658
#43 10.27   Downloaded compression-core v0.4.31
2026-Mar-21 03:29:39.339658
#43 10.28   Downloaded cooked-waker v5.0.0
2026-Mar-21 03:29:39.339658
#43 10.29   Downloaded block-buffer v0.10.4
2026-Mar-21 03:29:39.339658
#43 10.29   Downloaded heck v0.5.0
2026-Mar-21 03:29:39.339658
#43 10.29   Downloaded const-random-macro v0.1.16
2026-Mar-21 03:29:39.339658
#43 10.30   Downloaded debugid v0.8.0
2026-Mar-21 03:29:39.339658
#43 10.30   Downloaded base16ct v0.1.1
2026-Mar-21 03:29:39.339658
#43 10.30   Downloaded futures-task v0.3.32
2026-Mar-21 03:29:39.339658
#43 10.30   Downloaded crc v3.4.0
2026-Mar-21 03:29:39.440906
#43 10.35   Downloaded futures-sink v0.3.32
2026-Mar-21 03:29:39.440906
#43 10.35   Downloaded http-body v0.4.6
2026-Mar-21 03:29:39.440906
#43 10.36   Downloaded crossbeam-queue v0.3.12
2026-Mar-21 03:29:39.440906
#43 10.36   Downloaded cfg-if v1.0.4
2026-Mar-21 03:29:39.440906
#43 10.36   Downloaded crc-catalog v2.4.0
2026-Mar-21 03:29:39.440906
#43 10.37   Downloaded aws-smithy-observability v0.2.6
2026-Mar-21 03:29:39.440906
#43 10.37   Downloaded cpufeatures v0.2.17
2026-Mar-21 03:29:39.440906
#43 10.37   Downloaded arraydeque v0.5.1
2026-Mar-21 03:29:39.440906
#43 10.38   Downloaded home v0.5.12
2026-Mar-21 03:29:39.440906
#43 10.38   Downloaded itoa v1.0.17
2026-Mar-21 03:29:39.440906
#43 10.38   Downloaded heck v0.4.1
2026-Mar-21 03:29:39.440906
#43 10.39   Downloaded bytes-utils v0.1.4
2026-Mar-21 03:29:39.440906
#43 10.39   Downloaded byteorder v1.5.0
2026-Mar-21 03:29:39.440906
#43 10.39   Downloaded aws-smithy-http v0.62.6
2026-Mar-21 03:29:39.440906
#43 10.40   Downloaded convert_case v0.6.0
2026-Mar-21 03:29:39.440906
#43 10.40   Downloaded ff v0.12.1
2026-Mar-21 03:29:39.440906
#43 10.40   Downloaded base64ct v1.8.3
2026-Mar-21 03:29:39.440906
#43 10.41   Downloaded event-listener-strategy v0.5.4
2026-Mar-21 03:29:39.440906
#43 10.41   Downloaded bincode v1.3.3
2026-Mar-21 03:29:39.440906
#43 10.42   Downloaded aws-smithy-async v1.2.14
2026-Mar-21 03:29:39.440906
#43 10.42   Downloaded aws-smithy-checksums v0.63.12
2026-Mar-21 03:29:39.440906
#43 10.42   Downloaded autocfg v1.5.0
2026-Mar-21 03:29:39.440906
#43 10.44   Downloaded json5 v0.4.1
2026-Mar-21 03:29:39.440906
#43 10.44   Downloaded hex v0.4.3
2026-Mar-21 03:29:39.440906
#43 10.45   Downloaded matchers v0.2.0
2026-Mar-21 03:29:39.440906
#43 10.45   Downloaded md-5 v0.10.6
2026-Mar-21 03:29:39.543306
#43 10.46   Downloaded hyper-tls v0.6.0
2026-Mar-21 03:29:39.543306
#43 10.46   Downloaded dashmap v6.1.0
2026-Mar-21 03:29:39.543306
#43 10.47   Downloaded aws-smithy-json v0.61.9
2026-Mar-21 03:29:39.543306
#43 10.47   Downloaded bit-vec v0.6.3
2026-Mar-21 03:29:39.543306
#43 10.47   Downloaded compression-codecs v0.4.37
2026-Mar-21 03:29:39.543306
#43 10.48   Downloaded generic-array v0.14.7
2026-Mar-21 03:29:39.543306
#43 10.48   Downloaded aws-smithy-eventstream v0.60.20
2026-Mar-21 03:29:39.543306
#43 10.49   Downloaded base64-simd v0.8.0
2026-Mar-21 03:29:39.543306
#43 10.49   Downloaded aws-smithy-xml v0.60.15
2026-Mar-21 03:29:39.543306
#43 10.50   Downloaded concurrent-queue v2.5.0
2026-Mar-21 03:29:39.543306
#43 10.50   Downloaded cmake v0.1.57
2026-Mar-21 03:29:39.543306
#43 10.52   Downloaded deranged v0.5.8
2026-Mar-21 03:29:39.543306
#43 10.53   Downloaded mime v0.3.17
2026-Mar-21 03:29:39.543306
#43 10.53   Downloaded idna_adapter v1.2.1
2026-Mar-21 03:29:39.543306
#43 10.53   Downloaded async-lock v3.4.2
2026-Mar-21 03:29:39.543306
#43 10.54   Downloaded lock_api v0.4.14
2026-Mar-21 03:29:39.543306
#43 10.54   Downloaded no-std-compat v0.4.1
2026-Mar-21 03:29:39.543306
#43 10.54   Downloaded gzip-header v1.0.0
2026-Mar-21 03:29:39.543306
#43 10.55   Downloaded lazy_static v1.5.0
2026-Mar-21 03:29:39.543306
#43 10.55   Downloaded http-body v1.0.1
2026-Mar-21 03:29:39.543306
#43 10.55   Downloaded memoffset v0.9.1
2026-Mar-21 03:29:39.646252
#43 10.56   Downloaded cookie v0.18.1
2026-Mar-21 03:29:39.646252
#43 10.56   Downloaded async-trait v0.1.89
2026-Mar-21 03:29:39.646252
#43 10.57   Downloaded lru-slab v0.1.2
2026-Mar-21 03:29:39.646252
#43 10.57   Downloaded num-conv v0.2.0
2026-Mar-21 03:29:39.646252
#43 10.57   Downloaded lru v0.12.5
2026-Mar-21 03:29:39.646252
#43 10.57   Downloaded num-iter v0.1.45
2026-Mar-21 03:29:39.646252
#43 10.57   Downloaded litrs v1.0.0
2026-Mar-21 03:29:39.646252
#43 10.58   Downloaded ecdsa v0.14.8
2026-Mar-21 03:29:39.646252
#43 10.58   Downloaded dlv-list v0.5.2
2026-Mar-21 03:29:39.646252
#43 10.59   Downloaded data-encoding v2.10.0
2026-Mar-21 03:29:39.646252
#43 10.59   Downloaded crossbeam-utils v0.8.21
2026-Mar-21 03:29:39.646252
#43 10.59   Downloaded aws-smithy-http v0.63.6
2026-Mar-21 03:29:39.646252
#43 10.60   Downloaded futures-io v0.3.32
2026-Mar-21 03:29:39.646252
#43 10.60   Downloaded const-oid v0.9.6
2026-Mar-21 03:29:39.646252
#43 10.60   Downloaded cexpr v0.6.0
2026-Mar-21 03:29:39.646252
#43 10.61   Downloaded anyhow v1.0.102
2026-Mar-21 03:29:39.646252
#43 10.61   Downloaded openssl-probe v0.2.1
2026-Mar-21 03:29:39.646252
#43 10.62   Downloaded find-msvc-tools v0.1.9
2026-Mar-21 03:29:39.646252
#43 10.62   Downloaded lazycell v1.3.0
2026-Mar-21 03:29:39.646252
#43 10.62   Downloaded crossbeam-epoch v0.9.18
2026-Mar-21 03:29:39.646252
#43 10.63   Downloaded clang-sys v1.8.1
2026-Mar-21 03:29:39.646252
#43 10.63   Downloaded httpdate v1.0.3
2026-Mar-21 03:29:39.646252
#43 10.63   Downloaded bitflags v2.11.0
2026-Mar-21 03:29:39.646252
#43 10.64   Downloaded log v0.4.29
2026-Mar-21 03:29:39.646252
#43 10.65   Downloaded hashlink v0.10.0
2026-Mar-21 03:29:39.646252
#43 10.65   Downloaded futures-executor v0.3.32
2026-Mar-21 03:29:39.646252
#43 10.65   Downloaded hyper-rustls v0.24.2
2026-Mar-21 03:29:39.646252
#43 10.66   Downloaded ordered-multimap v0.7.3
2026-Mar-21 03:29:39.755546
#43 10.66   Downloaded aws-smithy-runtime-api v1.11.6
2026-Mar-21 03:29:39.755546
#43 10.67   Downloaded arc-swap v1.8.2
2026-Mar-21 03:29:39.755546
#43 10.67   Downloaded openssl-macros v0.1.1
2026-Mar-21 03:29:39.755546
#43 10.67   Downloaded num_cpus v1.17.0
2026-Mar-21 03:29:39.755546
#43 10.68   Downloaded nonzero_ext v0.3.0
2026-Mar-21 03:29:39.755546
#43 10.69   Downloaded matchit v0.8.4
2026-Mar-21 03:29:39.755546
#43 10.69   Downloaded libloading v0.8.9
2026-Mar-21 03:29:39.755546
#43 10.69   Downloaded hashlink v0.8.4
2026-Mar-21 03:29:39.755546
#43 10.70   Downloaded glob v0.3.3
2026-Mar-21 03:29:39.755546
#43 10.70   Downloaded once_cell v1.21.3
2026-Mar-21 03:29:39.755546
#43 10.70   Downloaded pathdiff v0.2.3
2026-Mar-21 03:29:39.755546
#43 10.71   Downloaded percent-encoding v2.3.2
2026-Mar-21 03:29:39.755546
#43 10.71   Downloaded fs_extra v1.3.0
2026-Mar-21 03:29:39.755546
#43 10.71   Downloaded pem v3.0.6
2026-Mar-21 03:29:39.755546
#43 10.72   Downloaded powerfmt v0.2.0
2026-Mar-21 03:29:39.755546
#43 10.72   Downloaded proc-macro-rules v0.4.0
2026-Mar-21 03:29:39.755546
#43 10.72   Downloaded pest_derive v2.8.6
2026-Mar-21 03:29:39.755546
#43 10.73   Downloaded proc-macro-error v1.0.4
2026-Mar-21 03:29:39.755546
#43 10.73   Downloaded psl-types v2.0.11
2026-Mar-21 03:29:39.755546
#43 10.74   Downloaded aws-config v1.8.15
2026-Mar-21 03:29:39.755546
#43 10.75   Downloaded event-listener v5.4.1
2026-Mar-21 03:29:39.755546
#43 10.75   Downloaded proc-macro-rules-macros v0.4.0
2026-Mar-21 03:29:39.755546
#43 10.75   Downloaded hyper-rustls v0.27.7
2026-Mar-21 03:29:39.755546
#43 10.76   Downloaded aws-smithy-runtime v1.10.3
2026-Mar-21 03:29:39.755546
#43 10.76   Downloaded mio v1.1.1
2026-Mar-21 03:29:39.857695
#43 10.78   Downloaded getrandom v0.2.17
2026-Mar-21 03:29:39.857695
#43 10.78   Downloaded combine v4.6.7
2026-Mar-21 03:29:39.857695
#43 10.79   Downloaded quanta v0.12.6
2026-Mar-21 03:29:39.857695
#43 10.80   Downloaded quote v1.0.44
2026-Mar-21 03:29:39.857695
#43 10.81   Downloaded rand_chacha v0.3.1
2026-Mar-21 03:29:39.857695
#43 10.81   Downloaded deno_core v0.300.0
2026-Mar-21 03:29:39.857695
#43 10.82   Downloaded rand_core v0.9.5
2026-Mar-21 03:29:39.857695
#43 10.83   Downloaded openssl-sys v0.9.111
2026-Mar-21 03:29:39.857695
#43 10.84   Downloaded radium v0.7.0
2026-Mar-21 03:29:39.857695
#43 10.84   Downloaded pest_generator v2.8.6
2026-Mar-21 03:29:39.857695
#43 10.84   Downloaded flume v0.11.1
2026-Mar-21 03:29:39.857695
#43 10.85   Downloaded rand_core v0.6.4
2026-Mar-21 03:29:39.857695
#43 10.85   Downloaded rand_chacha v0.9.0
2026-Mar-21 03:29:39.857695
#43 10.85   Downloaded miniz_oxide v0.7.4
2026-Mar-21 03:29:39.857695
#43 10.85   Downloaded bitvec v1.0.1
2026-Mar-21 03:29:39.960230
#43 10.88   Downloaded sec1 v0.3.0
2026-Mar-21 03:29:39.960230
#43 10.88   Downloaded semver-parser v0.7.0
2026-Mar-21 03:29:39.960230
#43 10.89   Downloaded semver v1.0.27
2026-Mar-21 03:29:39.960230
#43 10.89   Downloaded serde_derive v1.0.228
2026-Mar-21 03:29:39.960230
#43 10.90   Downloaded serde_core v1.0.228
2026-Mar-21 03:29:39.960230
#43 10.90   Downloaded rustc_version v0.4.1
2026-Mar-21 03:29:39.960230
#43 10.91   Downloaded flate2 v1.1.9
2026-Mar-21 03:29:39.960230
#43 10.91   Downloaded rfc6979 v0.3.1
2026-Mar-21 03:29:39.960230
#43 10.92   Downloaded miniz_oxide v0.8.9
2026-Mar-21 03:29:39.960230
#43 10.92   Downloaded semver v0.9.0
2026-Mar-21 03:29:39.960230
#43 10.92   Downloaded scopeguard v1.2.0
2026-Mar-21 03:29:39.960230
#43 10.92   Downloaded sct v0.7.1
2026-Mar-21 03:29:39.960230
#43 10.93   Downloaded ron v0.8.1
2026-Mar-21 03:29:39.960230
#43 10.94   Downloaded p256 v0.11.1
2026-Mar-21 03:29:39.960230
#43 10.94   Downloaded rustversion v1.0.22
2026-Mar-21 03:29:39.960230
#43 10.95   Downloaded rustc_version v0.2.3
2026-Mar-21 03:29:39.960230
#43 10.95   Downloaded rustc-hash v2.1.1
2026-Mar-21 03:29:39.960230
#43 10.95   Downloaded rustc-hash v1.1.0
2026-Mar-21 03:29:39.960230
#43 10.95   Downloaded icu_normalizer_data v2.1.1
2026-Mar-21 03:29:39.960230
#43 10.95   Downloaded icu_normalizer v2.1.1
2026-Mar-21 03:29:39.960230
#43 10.96   Downloaded icu_locale_core v2.1.1
2026-Mar-21 03:29:39.960230
#43 10.97   Downloaded sha1 v0.10.6
2026-Mar-21 03:29:40.183002
#43 10.97   Downloaded http v0.2.12
2026-Mar-21 03:29:40.183002
#43 10.98   Downloaded indexmap v2.13.0
2026-Mar-21 03:29:40.183002
#43 10.98   Downloaded signature v1.6.4
2026-Mar-21 03:29:40.183002
#43 10.99   Downloaded http v1.4.0
2026-Mar-21 03:29:40.183002
#43 10.99   Downloaded slab v0.4.12
2026-Mar-21 03:29:40.183002
#43 11.00   Downloaded simple_asn1 v0.6.4
2026-Mar-21 03:29:40.183002
#43 11.00   Downloaded ryu v1.0.23
2026-Mar-21 03:29:40.183002
#43 11.01   Downloaded shlex v1.3.0
2026-Mar-21 03:29:40.183002
#43 11.01   Downloaded nom v7.1.3
2026-Mar-21 03:29:40.183002
#43 11.02   Downloaded socket2 v0.5.10
2026-Mar-21 03:29:40.183002
#43 11.02   Downloaded spin v0.9.8
2026-Mar-21 03:29:40.183002
#43 11.03   Downloaded sourcemap v8.0.1
2026-Mar-21 03:29:40.183002
#43 11.03   Downloaded socket2 v0.6.2
2026-Mar-21 03:29:40.183002
#43 11.03   Downloaded simd-adler32 v0.3.8
2026-Mar-21 03:29:40.183002
#43 11.03   Downloaded simd-abstraction v0.7.1
2026-Mar-21 03:29:40.183002
#43 11.04   Downloaded signature v2.2.0
2026-Mar-21 03:29:40.183002
#43 11.04   Downloaded pest_meta v2.8.6
2026-Mar-21 03:29:40.183002
#43 11.04   Downloaded aws-sdk-s3 v1.119.0
2026-Mar-21 03:29:40.183002
#43 11.19   Downloaded toml_datetime v0.6.11
2026-Mar-21 03:29:40.284666
#43 11.20   Downloaded tracing-log v0.2.0
2026-Mar-21 03:29:40.284666
#43 11.20   Downloaded tracing-core v0.1.36
2026-Mar-21 03:29:40.284666
#43 11.21   Downloaded try-lock v0.2.5
2026-Mar-21 03:29:40.284666
#43 11.21   Downloaded utf8_iter v1.0.4
2026-Mar-21 03:29:40.284666
#43 11.21   Downloaded yoke v0.8.1
2026-Mar-21 03:29:40.284666
#43 11.21   Downloaded rustls-webpki v0.103.9
2026-Mar-21 03:29:40.284666
#43 11.22   Downloaded zmij v1.0.21
2026-Mar-21 03:29:40.284666
#43 11.22   Downloaded redis v0.27.6
2026-Mar-21 03:29:40.284666
#43 11.23   Downloaded regex-syntax v0.8.10
2026-Mar-21 03:29:40.284666
#43 11.24   Downloaded rustls v0.23.37
2026-Mar-21 03:29:40.284666
#43 11.26   Downloaded zerovec-derive v0.11.2
2026-Mar-21 03:29:40.284666
#43 11.26   Downloaded regex-automata v0.4.14
2026-Mar-21 03:29:40.284666
#43 11.28   Downloaded which v4.4.2
2026-Mar-21 03:29:40.284666
#43 11.28   Downloaded ucd-trie v0.1.7
2026-Mar-21 03:29:40.284666
#43 11.28   Downloaded zerofrom-derive v0.1.6
2026-Mar-21 03:29:40.284666
#43 11.28   Downloaded zerofrom v0.1.6
2026-Mar-21 03:29:40.284666
#43 11.28   Downloaded which v6.0.3
2026-Mar-21 03:29:40.284666
#43 11.29   Downloaded yoke-derive v0.8.1
2026-Mar-21 03:29:40.284666
#43 11.29   Downloaded whoami v1.6.1
2026-Mar-21 03:29:40.284666
#43 11.29   Downloaded zeroize v1.8.2
2026-Mar-21 03:29:40.284666
#43 11.29   Downloaded rsa v0.9.10
2026-Mar-21 03:29:40.386386
#43 11.30   Downloaded ring v0.17.14
2026-Mar-21 03:29:40.386386
#43 11.36   Downloaded reqwest v0.12.28
2026-Mar-21 03:29:40.386386
#43 11.37   Downloaded rustls-webpki v0.101.7
2026-Mar-21 03:29:40.386386
#43 11.39   Downloaded regex v1.12.3
2026-Mar-21 03:29:40.386386
#43 11.40   Downloaded quinn-proto v0.11.13
2026-Mar-21 03:29:40.523257
#43 11.40   Downloaded deno_core_icudata v0.0.73
2026-Mar-21 03:29:40.523257
#43 11.44   Downloaded uuid v1.21.0
2026-Mar-21 03:29:40.523257
#43 11.44   Downloaded utoipa v4.2.3
2026-Mar-21 03:29:40.523257
#43 11.45   Downloaded unicode-ident v1.0.24
2026-Mar-21 03:29:40.523257
#43 11.45   Downloaded unicode-id-start v1.4.0
2026-Mar-21 03:29:40.523257
#43 11.45   Downloaded unicode-bidi v0.3.18
2026-Mar-21 03:29:40.523257
#43 11.46   Downloaded rustls v0.21.12
2026-Mar-21 03:29:40.523257
#43 11.47   Downloaded serde_json v1.0.149
2026-Mar-21 03:29:40.523257
#43 11.48   Downloaded libc v0.2.182
2026-Mar-21 03:29:40.523257
#43 11.53   Downloaded rustix v0.38.44
2026-Mar-21 03:29:40.625372
#43 11.57   Downloaded serde v1.0.228
2026-Mar-21 03:29:40.625372
#43 11.57   Downloaded xmlparser v0.13.6
2026-Mar-21 03:29:40.625372
#43 11.58   Downloaded wyz v0.5.1
2026-Mar-21 03:29:40.625372
#43 11.58   Downloaded writeable v0.6.2
2026-Mar-21 03:29:40.625372
#43 11.58   Downloaded webpki-roots v0.26.11
2026-Mar-21 03:29:40.625372
#43 11.59   Downloaded web-time v1.1.0
2026-Mar-21 03:29:40.625372
#43 11.59   Downloaded want v0.3.1
2026-Mar-21 03:29:40.625372
#43 11.59   Downloaded encoding_rs v0.8.35
2026-Mar-21 03:29:40.625372
#43 11.61   Downloaded vsimd v0.8.0
2026-Mar-21 03:29:40.625372
#43 11.62   Downloaded version_check v0.9.5
2026-Mar-21 03:29:40.625372
#43 11.62   Downloaded regex-lite v0.1.9
2026-Mar-21 03:29:40.625372
#43 11.62   Downloaded sqlx-postgres v0.8.6
2026-Mar-21 03:29:40.625372
#43 11.63   Downloaded syn v2.0.117
2026-Mar-21 03:29:40.726346
#43 11.65   Downloaded linux-raw-sys v0.4.15
2026-Mar-21 03:29:40.726346
#43 11.70   Downloaded tower v0.5.3
2026-Mar-21 03:29:40.726346
#43 11.72   Downloaded tower v0.4.13
2026-Mar-21 03:29:40.726346
#43 11.73   Downloaded toml_edit v0.22.27
2026-Mar-21 03:29:40.726346
#43 11.74   Downloaded tokio-util v0.7.18
2026-Mar-21 03:29:40.842160
#43 11.75   Downloaded time v0.3.47
2026-Mar-21 03:29:40.842160
#43 11.76   Downloaded tokio v1.49.0
2026-Mar-21 03:29:40.842160
#43 11.83   Downloaded sqlx-sqlite v0.8.6
2026-Mar-21 03:29:40.842160
#43 11.83   Downloaded tracing v0.1.44
2026-Mar-21 03:29:40.842160
#43 11.85   Downloaded url v2.5.8
2026-Mar-21 03:29:40.947934
#43 11.85   Downloaded unicode-segmentation v1.12.0
2026-Mar-21 03:29:40.947934
#43 11.86   Downloaded yaml-rust2 v0.8.1
2026-Mar-21 03:29:40.947934
#43 11.90   Downloaded tracing-subscriber v0.3.22
2026-Mar-21 03:29:40.947934
#43 11.91   Downloaded zerotrie v0.2.3
2026-Mar-21 03:29:40.947934
#43 11.92   Downloaded zerocopy v0.8.39
2026-Mar-21 03:29:40.947934
#43 11.95   Downloaded sqlx-mysql v0.8.6
2026-Mar-21 03:29:40.947934
#43 11.96   Downloaded zerovec v0.11.5
2026-Mar-21 03:29:41.067550
#43 11.97   Downloaded typenum v1.19.0
2026-Mar-21 03:29:41.067550
#43 11.97   Downloaded sqlx-core v0.8.6
2026-Mar-21 03:29:41.067550
#43 11.98   Downloaded unicode-normalization v0.1.25
2026-Mar-21 03:29:41.067550
#43 11.98   Downloaded utoipa-gen v4.3.1
2026-Mar-21 03:29:41.067550
#43 11.99   Downloaded tower-http v0.6.8
2026-Mar-21 03:29:41.067550
#43 12.00   Downloaded winnow v0.7.14
2026-Mar-21 03:29:41.067550
#43 12.02   Downloaded libsqlite3-sys v0.30.1
2026-Mar-21 03:29:41.067550
#43 12.08   Downloaded sqlx v0.8.6
2026-Mar-21 03:29:41.182447
#43 12.10   Downloaded webpki-roots v1.0.6
2026-Mar-21 03:29:41.182447
#43 12.10   Downloaded syn v1.0.109
2026-Mar-21 03:29:41.182447
#43 12.11   Downloaded vcpkg v0.2.15
2026-Mar-21 03:29:41.182447
#43 12.19   Downloaded aws-lc-sys v0.38.0
2026-Mar-21 03:29:41.485947
#43 12.50   Downloaded raw-cpuid v11.6.0
2026-Mar-21 03:29:41.590342
#43 12.50   Downloaded urlencoding v2.1.3
2026-Mar-21 03:29:41.590342
#43 12.50   Downloaded untrusted v0.9.0
2026-Mar-21 03:29:41.590342
#43 12.50   Downloaded unicode-properties v0.1.4
2026-Mar-21 03:29:41.590342
#43 12.51   Downloaded rand v0.8.5
2026-Mar-21 03:29:41.590342
#43 12.51   Downloaded quinn v0.11.9
2026-Mar-21 03:29:41.590342
#43 12.51   Downloaded tower-service v0.3.3
2026-Mar-21 03:29:41.590342
#43 12.51   Downloaded tower-layer v0.3.3
2026-Mar-21 03:29:41.590342
#43 12.52   Downloaded toml_write v0.1.2
2026-Mar-21 03:29:41.590342
#43 12.52   Downloaded toml v0.8.23
2026-Mar-21 03:29:41.590342
#43 12.52   Downloaded tokio-stream v0.1.18
2026-Mar-21 03:29:41.590342
#43 12.53   Downloaded tokio-rustls v0.26.4
2026-Mar-21 03:29:41.590342
#43 12.53   Downloaded tokio-rustls v0.24.1
2026-Mar-21 03:29:41.590342
#43 12.53   Downloaded tokio-native-tls v0.3.1
2026-Mar-21 03:29:41.590342
#43 12.54   Downloaded tokio-macros v2.6.0
2026-Mar-21 03:29:41.590342
#43 12.54   Downloaded tinyvec_macros v0.1.1
2026-Mar-21 03:29:41.590342
#43 12.54   Downloaded tinyvec v1.10.0
2026-Mar-21 03:29:41.590342
#43 12.54   Downloaded tinystr v0.8.2
2026-Mar-21 03:29:41.590342
#43 12.54   Downloaded tiny-keccak v2.0.2
2026-Mar-21 03:29:41.590342
#43 12.55   Downloaded time-macros v0.2.27
2026-Mar-21 03:29:41.590342
#43 12.55   Downloaded time-core v0.1.8
2026-Mar-21 03:29:41.590342
#43 12.55   Downloaded thiserror v1.0.69
2026-Mar-21 03:29:41.590342
#43 12.56   Downloaded synstructure v0.13.2
2026-Mar-21 03:29:41.590342
#43 12.56   Downloaded rand v0.9.2
2026-Mar-21 03:29:41.590342
#43 12.57   Downloaded publicsuffix v2.3.0
2026-Mar-21 03:29:41.590342
#43 12.57   Downloaded portable-atomic v1.13.1
2026-Mar-21 03:29:41.590342
#43 12.58   Downloaded openssl v0.10.75
2026-Mar-21 03:29:41.590342
#43 12.59   Downloaded moka v0.12.13
2026-Mar-21 03:29:41.590342
#43 12.60   Downloaded tracing-attributes v0.1.31
2026-Mar-21 03:29:41.695903
#43 12.60   Downloaded thread_local v1.1.9
2026-Mar-21 03:29:41.695903
#43 12.61   Downloaded thiserror-impl v2.0.18
2026-Mar-21 03:29:41.695903
#43 12.61   Downloaded thiserror-impl v1.0.69
2026-Mar-21 03:29:41.695903
#43 12.61   Downloaded thiserror v2.0.18
2026-Mar-21 03:29:41.695903
#43 12.62   Downloaded tap v1.0.1
2026-Mar-21 03:29:41.695903
#43 12.62   Downloaded tagptr v0.2.0
2026-Mar-21 03:29:41.695903
#43 12.62   Downloaded strum_macros v0.25.3
2026-Mar-21 03:29:41.695903
#43 12.62   Downloaded strum v0.25.0
2026-Mar-21 03:29:41.695903
#43 12.63   Downloaded stringprep v0.1.5
2026-Mar-21 03:29:41.695903
#43 12.63   Downloaded static_assertions v1.1.0
2026-Mar-21 03:29:41.695903
#43 12.63   Downloaded stable_deref_trait v1.2.1
2026-Mar-21 03:29:41.695903
#43 12.63   Downloaded sqlx-macros-core v0.8.6
2026-Mar-21 03:29:41.695903
#43 12.63   Downloaded sqlx-macros v0.8.6
2026-Mar-21 03:29:41.695903
#43 12.64   Downloaded spki v0.7.3
2026-Mar-21 03:29:41.695903
#43 12.64   Downloaded prettyplease v0.2.37
2026-Mar-21 03:29:41.695903
#43 12.64   Downloaded pest v2.8.6
2026-Mar-21 03:29:41.695903
#43 12.65   Downloaded libm v0.2.16
2026-Mar-21 03:29:41.695903
#43 12.67   Downloaded idna v1.1.0
2026-Mar-21 03:29:41.695903
#43 12.67   Downloaded icu_properties_data v2.1.2
2026-Mar-21 03:29:41.695903
#43 12.69   Downloaded hyper v1.8.1
2026-Mar-21 03:29:41.695903
#43 12.69   Downloaded hyper v0.14.32
2026-Mar-21 03:29:41.695903
#43 12.71   Downloaded hkdf v0.12.4
2026-Mar-21 03:29:41.799437
#43 12.71   Downloaded hashbrown v0.15.5
2026-Mar-21 03:29:41.799437
#43 12.72   Downloaded h2 v0.4.13
2026-Mar-21 03:29:41.799437
#43 12.73   Downloaded h2 v0.3.27
2026-Mar-21 03:29:41.799437
#43 12.74   Downloaded futures-util v0.3.32
2026-Mar-21 03:29:41.799437
#43 12.76   Downloaded sync_wrapper v1.0.2
2026-Mar-21 03:29:41.799437
#43 12.76   Downloaded subtle v2.6.1
2026-Mar-21 03:29:41.799437
#43 12.76   Downloaded spki v0.6.0
2026-Mar-21 03:29:41.799437
#43 12.76   Downloaded spinning_top v0.3.0
2026-Mar-21 03:29:41.799437
#43 12.76   Downloaded smallvec v1.15.1
2026-Mar-21 03:29:41.799437
#43 12.77   Downloaded itertools v0.13.0
2026-Mar-21 03:29:41.799437
#43 12.78   Downloaded iri-string v0.7.10
2026-Mar-21 03:29:41.799437
#43 12.79   Downloaded hashbrown v0.16.1
2026-Mar-21 03:29:41.799437
#43 12.79   Downloaded itertools v0.12.1
2026-Mar-21 03:29:41.799437
#43 12.80   Downloaded hashbrown v0.14.5
2026-Mar-21 03:29:41.799437
#43 12.81   Downloaded governor v0.8.1
2026-Mar-21 03:29:41.902360
#43 12.81   Downloaded brotli v8.0.2
2026-Mar-21 03:29:41.902360
#43 12.83   Downloaded sharded-slab v0.1.7
2026-Mar-21 03:29:41.902360
#43 12.84   Downloaded sha2 v0.10.9
2026-Mar-21 03:29:41.902360
#43 12.84   Downloaded num-bigint-dig v0.8.6
2026-Mar-21 03:29:41.902360
#43 12.84   Downloaded signal-hook-registry v1.4.8
2026-Mar-21 03:29:41.902360
#43 12.85   Downloaded sha1_smol v1.0.1
2026-Mar-21 03:29:41.902360
#43 12.85   Downloaded serde_v8 v0.209.0
2026-Mar-21 03:29:41.902360
#43 12.85   Downloaded serde_path_to_error v0.1.20
2026-Mar-21 03:29:41.902360
#43 12.85   Downloaded rustls-pki-types v1.14.0
2026-Mar-21 03:29:41.902360
#43 12.86   Downloaded icu_collections v2.1.1
2026-Mar-21 03:29:41.902360
#43 12.87   Downloaded hyper-util v0.1.20
2026-Mar-21 03:29:41.902360
#43 12.87   Downloaded futures-intrusive v0.5.0
2026-Mar-21 03:29:41.902360
#43 12.88   Downloaded serde_urlencoded v0.7.1
2026-Mar-21 03:29:41.902360
#43 12.88   Downloaded serde_spanned v0.6.9
2026-Mar-21 03:29:41.902360
#43 12.88   Downloaded rustls-native-certs v0.8.3
2026-Mar-21 03:29:41.902360
#43 12.89   Downloaded rust-ini v0.20.0
2026-Mar-21 03:29:41.902360
#43 12.89   Downloaded pin-project v1.1.10
2026-Mar-21 03:29:41.902360
#43 12.91   Downloaded elliptic-curve v0.12.3
2026-Mar-21 03:29:41.902360
#43 12.91   Downloaded num-bigint v0.4.6
2026-Mar-21 03:29:42.005663
#43 12.92   Downloaded jsonwebtoken v9.3.1
2026-Mar-21 03:29:42.005663
#43 12.92   Downloaded icu_provider v2.1.1
2026-Mar-21 03:29:42.005663
#43 12.93   Downloaded icu_properties v2.1.2
2026-Mar-21 03:29:42.005663
#43 12.93   Downloaded getrandom v0.4.1
2026-Mar-21 03:29:42.005663
#43 12.93   Downloaded futures v0.3.32
2026-Mar-21 03:29:42.005663
#43 12.94   Downloaded brotli-decompressor v5.0.0
2026-Mar-21 03:29:42.005663
#43 12.95   Downloaded bindgen v0.69.5
2026-Mar-21 03:29:42.005663
#43 12.95   Downloaded aws-lc-rs v1.16.1
2026-Mar-21 03:29:42.005663
#43 12.97   Downloaded quinn-udp v0.5.14
2026-Mar-21 03:29:42.005663
#43 12.97   Downloaded proc-macro2 v1.0.106
2026-Mar-21 03:29:42.005663
#43 12.98   Downloaded proc-macro-error-attr v1.0.4
2026-Mar-21 03:29:42.005663
#43 12.98   Downloaded pkg-config v0.3.32
2026-Mar-21 03:29:42.005663
#43 12.98   Downloaded pkcs8 v0.10.2
2026-Mar-21 03:29:42.005663
#43 12.98   Downloaded parking v2.2.1
2026-Mar-21 03:29:42.005663
#43 12.98   Downloaded getrandom v0.3.4
2026-Mar-21 03:29:42.005663
#43 12.99   Downloaded axum v0.8.8
2026-Mar-21 03:29:42.005663
#43 13.00   Downloaded aws-sdk-sts v1.100.0
2026-Mar-21 03:29:42.005663
#43 13.02   Downloaded aho-corasick v1.1.4
2026-Mar-21 03:29:42.108737
#43 13.02   Downloaded potential_utf v0.1.4
2026-Mar-21 03:29:42.108737
#43 13.02   Downloaded litemap v0.8.1
2026-Mar-21 03:29:42.108737
#43 13.02   Downloaded jobserver v0.1.34
2026-Mar-21 03:29:42.108737
#43 13.03   Downloaded http-body-util v0.1.3
2026-Mar-21 03:29:42.108737
#43 13.03   Downloaded group v0.12.1
2026-Mar-21 03:29:42.108737
#43 13.03   Downloaded futures-channel v0.3.32
2026-Mar-21 03:29:42.108737
#43 13.03   Downloaded crc-fast v1.6.0
2026-Mar-21 03:29:42.108737
#43 13.04   Downloaded ppv-lite86 v0.2.21
2026-Mar-21 03:29:42.108737
#43 13.05   Downloaded pkcs1 v0.7.5
2026-Mar-21 03:29:42.108737
#43 13.05   Downloaded minimal-lexical v0.2.1
2026-Mar-21 03:29:42.108737
#43 13.05   Downloaded hmac v0.12.1
2026-Mar-21 03:29:42.108737
#43 13.06   Downloaded aws-sdk-ssooidc v1.98.0
2026-Mar-21 03:29:42.108737
#43 13.07   Downloaded pkcs8 v0.9.0
2026-Mar-21 03:29:42.108737
#43 13.07   Downloaded pin-utils v0.1.0
2026-Mar-21 03:29:42.108737
#43 13.07   Downloaded pin-project-lite v0.2.16
2026-Mar-21 03:29:42.108737
#43 13.08   Downloaded pin-project-internal v1.1.10
2026-Mar-21 03:29:42.108737
#43 13.08   Downloaded pem-rfc7468 v0.7.0
2026-Mar-21 03:29:42.108737
#43 13.09   Downloaded parking_lot_core v0.9.12
2026-Mar-21 03:29:42.108737
#43 13.09   Downloaded memchr v2.8.0
2026-Mar-21 03:29:42.108737
#43 13.10   Downloaded der v0.7.10
2026-Mar-21 03:29:42.108737
#43 13.10   Downloaded async-compression v0.4.41
2026-Mar-21 03:29:42.108737
#43 13.11   Downloaded paste v1.0.15
2026-Mar-21 03:29:42.108737
#43 13.12   Downloaded parking_lot v0.12.5
2026-Mar-21 03:29:42.213505
#43 13.12   Downloaded nu-ansi-term v0.50.3
2026-Mar-21 03:29:42.213505
#43 13.12   Downloaded ipnet v2.11.0
2026-Mar-21 03:29:42.213505
#43 13.13   Downloaded httparse v1.10.1
2026-Mar-21 03:29:42.213505
#43 13.13   Downloaded crossbeam-channel v0.5.15
2026-Mar-21 03:29:42.213505
#43 13.14   Downloaded aws-sigv4 v1.4.2
2026-Mar-21 03:29:42.213505
#43 13.22   Downloaded der v0.6.1
2026-Mar-21 03:29:42.213505
#43 13.22   Downloaded outref v0.5.2
2026-Mar-21 03:29:42.315779
#43 13.22   Downloaded num-traits v0.2.19
2026-Mar-21 03:29:42.315779
#43 13.23   Downloaded displaydoc v0.2.5
2026-Mar-21 03:29:42.315779
#43 13.23   Downloaded aws-smithy-types v1.4.6
2026-Mar-21 03:29:42.315779
#43 13.24   Downloaded outref v0.1.0
2026-Mar-21 03:29:42.315779
#43 13.24   Downloaded num-integer v0.1.46
2026-Mar-21 03:29:42.315779
#43 13.24   Downloaded native-tls v0.2.18
2026-Mar-21 03:29:42.315779
#43 13.24   Downloaded deno_ops v0.176.0
2026-Mar-21 03:29:42.315779
#43 13.25   Downloaded crypto-bigint v0.5.5
2026-Mar-21 03:29:42.315779
#43 13.27   Downloaded cc v1.2.56
2026-Mar-21 03:29:42.315779
#43 13.27   Downloaded bytes v1.11.1
2026-Mar-21 03:29:42.315779
#43 13.28   Downloaded aws-smithy-http-client v1.1.12
2026-Mar-21 03:29:42.315779
#43 13.28   Downloaded aws-runtime v1.7.2
2026-Mar-21 03:29:42.315779
#43 13.29   Downloaded futures-timer v3.0.3
2026-Mar-21 03:29:42.315779
#43 13.29   Downloaded base64 v0.22.1
2026-Mar-21 03:29:42.315779
#43 13.29   Downloaded base64 v0.21.7
2026-Mar-21 03:29:42.315779
#43 13.30   Downloaded aws-sdk-sso v1.96.0
2026-Mar-21 03:29:42.315779
#43 13.31   Downloaded foldhash v0.1.5
2026-Mar-21 03:29:42.315779
#43 13.31   Downloaded either v1.15.0
2026-Mar-21 03:29:42.315779
#43 13.32   Downloaded digest v0.10.7
2026-Mar-21 03:29:42.315779
#43 13.32   Downloaded config v0.14.1
2026-Mar-21 03:29:42.315779
#43 13.33   Downloaded aws-types v1.3.14
2026-Mar-21 03:29:42.488187
#43 ...
2026-Mar-21 03:29:42.488187
2026-Mar-21 03:29:42.488187
#45 [frontend builder 5/8] RUN npm install
2026-Mar-21 03:29:42.488187
#45 11.61
2026-Mar-21 03:29:42.488187
#45 11.61 > frontend@0.0.1 prepare
2026-Mar-21 03:29:42.488187
#45 11.61 > svelte-kit sync || echo ''
2026-Mar-21 03:29:42.488187
#45 11.61
2026-Mar-21 03:29:42.488187
#45 12.57
2026-Mar-21 03:29:42.488187
#45 12.57 added 235 packages, and audited 236 packages in 12s
2026-Mar-21 03:29:42.488187
#45 12.57
2026-Mar-21 03:29:42.488187
#45 12.57 43 packages are looking for funding
2026-Mar-21 03:29:42.488187
#45 12.57   run `npm fund` for details
2026-Mar-21 03:29:42.488187
#45 12.65
2026-Mar-21 03:29:42.488187
#45 12.65 7 vulnerabilities (2 low, 2 moderate, 3 high)
2026-Mar-21 03:29:42.488187
#45 12.65
2026-Mar-21 03:29:42.488187
#45 12.65 To address all issues, run:
2026-Mar-21 03:29:42.488187
#45 12.65   npm audit fix
2026-Mar-21 03:29:42.488187
#45 12.65
2026-Mar-21 03:29:42.488187
#45 12.65 Run `npm audit` for details.
2026-Mar-21 03:29:42.488187
#45 12.65 npm notice
2026-Mar-21 03:29:42.494381
#45 12.65 npm notice New major version of npm available! 10.9.4 -> 11.12.0
2026-Mar-21 03:29:42.494381
#45 12.65 npm notice Changelog: https://github.com/npm/cli/releases/tag/v11.12.0
2026-Mar-21 03:29:42.494381
#45 12.65 npm notice To update run: npm install -g npm@11.12.0
2026-Mar-21 03:29:42.494381
#45 12.65 npm notice
2026-Mar-21 03:29:42.494381
#45 DONE 13.4s
2026-Mar-21 03:29:42.494381
2026-Mar-21 03:29:42.494381
#43 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-21 03:29:42.494381
#43 13.33   Downloaded fastrand v2.3.0
2026-Mar-21 03:29:42.494381
#43 13.33   Downloaded crypto-bigint v0.4.9
2026-Mar-21 03:29:42.494381
#43 13.33   Downloaded allocator-api2 v0.2.21
2026-Mar-21 03:29:42.494381
#43 13.34   Downloaded equivalent v1.0.2
2026-Mar-21 03:29:42.494381
#43 13.34   Downloaded crc32fast v1.5.0
2026-Mar-21 03:29:42.494381
#43 13.34   Downloaded cookie_store v0.22.1
2026-Mar-21 03:29:42.494381
#43 13.34   Downloaded aws-smithy-json v0.62.5
2026-Mar-21 03:29:42.494381
#43 13.35   Downloaded aws-credential-types v1.2.14
2026-Mar-21 03:29:42.494381
#43 13.35   Downloaded axum-core v0.5.6
2026-Mar-21 03:29:42.494381
#43 13.35   Downloaded futures-macro v0.3.32
2026-Mar-21 03:29:42.648862
#43 13.66   Downloaded v8 v0.101.0
2026-Mar-21 03:29:42.803947
#43 ...
2026-Mar-21 03:29:42.803947
2026-Mar-21 03:29:42.803947
#46 [frontend builder 6/8] RUN test -n "https://api.snapvie.com" || (echo "VITE_API_URL build arg is required" && exit 1)
2026-Mar-21 03:29:42.803947
#46 DONE 0.3s
2026-Mar-21 03:29:42.954210
#44 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-21 03:29:42.954210
#44 10.09   Downloaded libc v0.2.182
2026-Mar-21 03:29:42.954210
#44 10.15   Downloaded unicode-segmentation v1.12.0
2026-Mar-21 03:29:42.954210
#44 10.22   Downloaded brotli-decompressor v5.0.0
2026-Mar-21 03:29:42.954210
#44 10.23   Downloaded aws-sdk-sts v1.100.0
2026-Mar-21 03:29:42.954210
#44 10.29   Downloaded tokio v1.49.0
2026-Mar-21 03:29:42.954210
#44 10.34   Downloaded rustls-webpki v0.101.7
2026-Mar-21 03:29:42.954210
#44 10.37   Downloaded utoipa-gen v4.3.1
2026-Mar-21 03:29:42.954210
#44 10.38   Downloaded aws-lc-rs v1.16.1
2026-Mar-21 03:29:42.954210
#44 10.39   Downloaded unicode-normalization v0.1.25
2026-Mar-21 03:29:42.954210
#44 10.45   Downloaded bitvec v1.0.1
2026-Mar-21 03:29:42.954210
#44 10.47   Downloaded bindgen v0.69.5
2026-Mar-21 03:29:42.954210
#44 10.52   Downloaded moka v0.12.13
2026-Mar-21 03:29:42.954210
#44 10.61   Downloaded encoding_rs v0.8.35
2026-Mar-21 03:29:42.954210
#44 10.67   Downloaded quinn-proto v0.11.13
2026-Mar-21 03:29:42.954210
#44 10.76   Downloaded ring v0.17.14
2026-Mar-21 03:29:42.954210
#44 10.81   Downloaded vcpkg v0.2.15
2026-Mar-21 03:29:42.954210
#44 10.89   Downloaded deno_core v0.300.0
2026-Mar-21 03:29:42.954210
#44 10.90   Downloaded webpki-roots v1.0.6
2026-Mar-21 03:29:42.954210
#44 10.91   Downloaded tracing-subscriber v0.3.22
2026-Mar-21 03:29:42.954210
#44 10.98   Downloaded syn v2.0.117
2026-Mar-21 03:29:42.954210
#44 11.03   Downloaded linux-raw-sys v0.4.15
2026-Mar-21 03:29:42.954210
#44 11.77   Downloaded libsqlite3-sys v0.30.1
2026-Mar-21 03:29:42.954210
#44 12.01   Downloaded brotli v8.0.2
2026-Mar-21 03:29:42.954210
#44 13.54   Downloaded aws-sdk-s3 v1.119.0
2026-Mar-21 03:29:44.300422
#44 ...
2026-Mar-21 03:29:44.300422
2026-Mar-21 03:29:44.300422
#43 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-21 03:29:44.300422
#43 14.83    Compiling proc-macro2 v1.0.106
2026-Mar-21 03:29:44.300422
#43 14.83    Compiling unicode-ident v1.0.24
2026-Mar-21 03:29:44.300422
#43 14.83    Compiling quote v1.0.44
2026-Mar-21 03:29:44.300422
#43 14.83    Compiling libc v0.2.182
2026-Mar-21 03:29:44.300422
#43 14.83    Compiling cfg-if v1.0.4
2026-Mar-21 03:29:44.300422
#43 14.83    Compiling serde v1.0.228
2026-Mar-21 03:29:44.300422
#43 14.85    Compiling serde_core v1.0.228
2026-Mar-21 03:29:44.300422
#43 14.86    Compiling version_check v0.9.5
2026-Mar-21 03:29:44.300422
#43 14.87    Compiling pin-project-lite v0.2.16
2026-Mar-21 03:29:44.300422
#43 14.89    Compiling parking_lot_core v0.9.12
2026-Mar-21 03:29:44.300422
#43 14.89    Compiling shlex v1.3.0
2026-Mar-21 03:29:44.300422
#43 14.89    Compiling memchr v2.8.0
2026-Mar-21 03:29:44.300422
#43 14.89    Compiling bytes v1.11.1
2026-Mar-21 03:29:44.300422
#43 14.91    Compiling once_cell v1.21.3
2026-Mar-21 03:29:44.300422
#43 14.92    Compiling scopeguard v1.2.0
2026-Mar-21 03:29:44.300422
#43 14.94    Compiling itoa v1.0.17
2026-Mar-21 03:29:44.300422
#43 14.94    Compiling futures-core v0.3.32
2026-Mar-21 03:29:44.300422
#43 14.94    Compiling find-msvc-tools v0.1.9
2026-Mar-21 03:29:44.300422
#43 14.94    Compiling futures-sink v0.3.32
2026-Mar-21 03:29:44.300422
#43 14.96    Compiling autocfg v1.5.0
2026-Mar-21 03:29:44.300422
#43 14.96    Compiling typenum v1.19.0
2026-Mar-21 03:29:44.300422
#43 14.98    Compiling log v0.4.29
2026-Mar-21 03:29:44.300422
#43 14.98    Compiling slab v0.4.12
2026-Mar-21 03:29:44.300422
#43 14.99    Compiling futures-task v0.3.32
2026-Mar-21 03:29:44.300422
#43 14.99    Compiling futures-io v0.3.32
2026-Mar-21 03:29:44.300422
#43 14.99    Compiling zeroize v1.8.2
2026-Mar-21 03:29:44.300422
#43 14.99    Compiling equivalent v1.0.2
2026-Mar-21 03:29:44.300422
#43 15.00    Compiling zerocopy v0.8.39
2026-Mar-21 03:29:44.300422
#43 15.00    Compiling subtle v2.6.1
2026-Mar-21 03:29:44.300422
#43 15.00    Compiling fnv v1.0.7
2026-Mar-21 03:29:44.300422
#43 15.00    Compiling hashbrown v0.16.1
2026-Mar-21 03:29:44.300422
#43 15.00    Compiling lock_api v0.4.14
2026-Mar-21 03:29:44.300422
#43 15.00    Compiling percent-encoding v2.3.2
2026-Mar-21 03:29:44.300422
#43 15.00    Compiling powerfmt v0.2.0
2026-Mar-21 03:29:44.300422
#43 15.01    Compiling time-core v0.1.8
2026-Mar-21 03:29:44.300422
#43 15.06    Compiling futures-channel v0.3.32
2026-Mar-21 03:29:44.300422
#43 15.12    Compiling tracing-core v0.1.36
2026-Mar-21 03:29:44.300422
#43 15.19    Compiling generic-array v0.14.7
2026-Mar-21 03:29:44.300422
#43 15.22    Compiling icu_properties_data v2.1.2
2026-Mar-21 03:29:44.300422
#43 15.22    Compiling num-conv v0.2.0
2026-Mar-21 03:29:44.300422
#43 15.25    Compiling icu_normalizer_data v2.1.1
2026-Mar-21 03:29:44.300422
#43 15.28    Compiling num-traits v0.2.19
2026-Mar-21 03:29:44.300422
#43 15.28    Compiling deranged v0.5.8
2026-Mar-21 03:29:44.300422
#43 15.28    Compiling pin-utils v0.1.0
2026-Mar-21 03:29:44.300422
#43 15.29    Compiling ryu v1.0.23
2026-Mar-21 03:29:44.300422
#43 15.31    Compiling crc32fast v1.5.0
2026-Mar-21 03:29:44.425593
#43 15.33    Compiling stable_deref_trait v1.2.1
2026-Mar-21 03:29:44.425593
#43 15.34    Compiling untrusted v0.9.0
2026-Mar-21 03:29:44.425593
#43 15.40    Compiling http v1.4.0
2026-Mar-21 03:29:44.425593
#43 15.43    Compiling fs_extra v1.3.0
2026-Mar-21 03:29:44.540257
#43 15.43    Compiling dunce v1.0.5
2026-Mar-21 03:29:44.540257
#43 15.44    Compiling http v0.2.12
2026-Mar-21 03:29:44.540257
#43 15.46    Compiling zmij v1.0.21
2026-Mar-21 03:29:44.540257
#43 15.48    Compiling time-macros v0.2.27
2026-Mar-21 03:29:44.540257
#43 15.49    Compiling form_urlencoded v1.2.2
2026-Mar-21 03:29:44.540257
#43 15.49    Compiling rustls-pki-types v1.14.0
2026-Mar-21 03:29:44.540257
#43 15.51    Compiling writeable v0.6.2
2026-Mar-21 03:29:44.540257
#43 15.52    Compiling tower-service v0.3.3
2026-Mar-21 03:29:44.540257
#43 15.53    Compiling litemap v0.8.1
2026-Mar-21 03:29:44.540257
#43 15.55    Compiling aws-lc-rs v1.16.1
2026-Mar-21 03:29:44.540257
#43 15.55    Compiling outref v0.5.2
2026-Mar-21 03:29:44.540257
#43 15.55    Compiling base64 v0.22.1
2026-Mar-21 03:29:44.684864
#43 15.61    Compiling vsimd v0.8.0
2026-Mar-21 03:29:44.684864
#43 15.62    Compiling httparse v1.10.1
2026-Mar-21 03:29:44.684864
#43 15.63    Compiling try-lock v0.2.5
2026-Mar-21 03:29:44.684864
#43 15.69    Compiling rustls v0.23.37
2026-Mar-21 03:29:44.786978
#43 15.71    Compiling httpdate v1.0.3
2026-Mar-21 03:29:44.786978
#43 15.72    Compiling atomic-waker v1.1.2
2026-Mar-21 03:29:44.786978
#43 15.77    Compiling want v0.3.1
2026-Mar-21 03:29:44.786978
#43 15.80    Compiling crossbeam-utils v0.8.21
2026-Mar-21 03:29:44.923784
#43 15.82    Compiling allocator-api2 v0.2.21
2026-Mar-21 03:29:44.923784
#43 15.85    Compiling tower-layer v0.3.3
2026-Mar-21 03:29:44.923784
#43 15.85    Compiling cpufeatures v0.2.17
2026-Mar-21 03:29:44.923784
#43 15.86    Compiling syn v2.0.117
2026-Mar-21 03:29:44.923784
#43 15.86    Compiling utf8_iter v1.0.4
2026-Mar-21 03:29:44.923784
#43 15.86    Compiling openssl-probe v0.2.1
2026-Mar-21 03:29:44.923784
#43 15.93    Compiling webpki-roots v1.0.6
2026-Mar-21 03:29:45.037546
#43 15.96    Compiling sync_wrapper v1.0.2
2026-Mar-21 03:29:45.037546
#43 15.99    Compiling serde_json v1.0.149
2026-Mar-21 03:29:45.037546
#43 15.99    Compiling ipnet v2.11.0
2026-Mar-21 03:29:45.037546
#43 16.02    Compiling rustversion v1.0.22
2026-Mar-21 03:29:45.037546
#43 16.05    Compiling bitflags v2.11.0
2026-Mar-21 03:29:45.140024
#43 16.09    Compiling base64-simd v0.8.0
2026-Mar-21 03:29:45.140024
#43 16.10    Compiling thiserror v2.0.18
2026-Mar-21 03:29:45.140024
#43 16.13    Compiling home v0.5.12
2026-Mar-21 03:29:45.140024
#43 16.14    Compiling getrandom v0.4.1
2026-Mar-21 03:29:45.140024
#43 16.15    Compiling
2026-Mar-21 03:29:45.299120
rustls-native-certs v0.8.3
2026-Mar-21 03:29:45.299120
#43 16.18    Compiling http-body v1.0.1
2026-Mar-21 03:29:45.299120
#43 16.20    Compiling http-body v0.4.6
2026-Mar-21 03:29:45.299120
#43 16.21    Compiling const-oid v0.9.6
2026-Mar-21 03:29:45.299120
#43 16.25    Compiling jobserver v0.1.34
2026-Mar-21 03:29:45.299120
#43 16.25    Compiling getrandom v0.2.17
2026-Mar-21 03:29:45.299120
#43 16.31    Compiling num-integer v0.1.46
2026-Mar-21 03:29:45.404683
#43 16.31    Compiling http-body-util v0.1.3
2026-Mar-21 03:29:45.404683
#43 16.31    Compiling hex v0.4.3
2026-Mar-21 03:29:45.404683
#43 16.32    Compiling base64ct v1.8.3
2026-Mar-21 03:29:45.404683
#43 16.36    Compiling pkg-config v0.3.32
2026-Mar-21 03:29:45.404683
#43 16.36    Compiling der v0.6.1
2026-Mar-21 03:29:45.404683
#43 16.41    Compiling errno v0.3.14
2026-Mar-21 03:29:45.404683
#43 16.41    Compiling
2026-Mar-21 03:29:45.516802
socket2 v0.6.2
2026-Mar-21 03:29:45.516802
#43 16.41    Compiling mio v1.1.1
2026-Mar-21 03:29:45.516802
#43 16.46    Compiling socket2 v0.5.10
2026-Mar-21 03:29:45.516802
#43 16.47    Compiling time v0.3.47
2026-Mar-21 03:29:45.516802
#43 16.50    Compiling cc v1.2.56
2026-Mar-21 03:29:45.516802
#43 16.53    Compiling vcpkg v0.2.15
2026-Mar-21 03:29:45.655143
#43 16.55    Compiling signal-hook-registry v1.4.8
2026-Mar-21 03:29:45.655143
#43 16.59    Compiling rand_core v0.6.4
2026-Mar-21 03:29:45.655143
#43 16.59    Compiling ahash v0.8.12
2026-Mar-21 03:29:45.655143
#43 16.62    Compiling uuid v1.21.0
2026-Mar-21 03:29:45.655143
#43 16.62    Compiling rustls v0.21.12
2026-Mar-21 03:29:45.655143
#43 16.66    Compiling crypto-common v0.1.7
2026-Mar-21 03:29:45.655143
#43 16.66    Compiling block-buffer v0.10.4
2026-Mar-21 03:29:45.781367
#43 16.68    Compiling aho-corasick v1.1.4
2026-Mar-21 03:29:45.781367
#43 16.70    Compiling rustix v0.38.44
2026-Mar-21 03:29:45.781367
#43 16.72    Compiling foldhash v0.1.5
2026-Mar-21 03:29:45.781367
#43 16.79    Compiling digest v0.10.7
2026-Mar-21 03:29:45.781367
#43 16.79    Compiling ff v0.12.1
2026-Mar-21 03:29:45.890838
#43 16.80    Compiling crypto-bigint v0.4.9
2026-Mar-21 03:29:45.890838
#43 16.83    Compiling regex-syntax v0.8.10
2026-Mar-21 03:29:45.890838
#43 16.83    Compiling crc-catalog v2.4.0
2026-Mar-21 03:29:45.890838
#43 16.85    Compiling glob v0.3.3
2026-Mar-21 03:29:45.890838
#43 16.89    Compiling base16ct v0.1.1
2026-Mar-21 03:29:45.890838
#43 16.90    Compiling hashbrown v0.15.5
2026-Mar-21 03:29:46.046447
#43 16.90    Compiling concurrent-queue v2.5.0
2026-Mar-21 03:29:46.046447
#43 16.94    Compiling crc v3.4.0
2026-Mar-21 03:29:46.046447
#43 16.94    Compiling group v0.12.1
2026-Mar-21 03:29:46.046447
#43 16.97    Compiling hmac v0.12.1
2026-Mar-21 03:29:46.046447
#43 16.97    Compiling sha2 v0.10.9
2026-Mar-21 03:29:46.046447
#43 16.97    Compiling linux-raw-sys v0.4.15
2026-Mar-21 03:29:46.046447
#43 17.05    Compiling prettyplease v0.2.37
2026-Mar-21 03:29:46.151967
#43 17.10    Compiling parking v2.2.1
2026-Mar-21 03:29:46.163567
#43 17.10    Compiling semver v1.0.27
2026-Mar-21 03:29:46.163567
#43 17.15    Compiling alloc-no-stdlib v2.0.4
2026-Mar-21 03:29:46.163567
#43 17.16    Compiling tinyvec_macros v0.1.1
2026-Mar-21 03:29:46.257428
#43 17.20    Compiling spki v0.6.0
2026-Mar-21 03:29:46.257428
#43 17.24    Compiling tinyvec v1.10.0
2026-Mar-21 03:29:46.257428
#43 17.27    Compiling event-listener v5.4.1
2026-Mar-21 03:29:46.371250
#43 17.29    Compiling clang-sys v1.8.1
2026-Mar-21 03:29:46.371250
#43 17.32    Compiling alloc-stdlib v0.2.2
2026-Mar-21 03:29:46.371250
#43 17.34    Compiling cmake v0.1.57
2026-Mar-21 03:29:46.371250
#43 17.36    Compiling indexmap v2.13.0
2026-Mar-21 03:29:46.371250
#43 17.38    Compiling pkcs8 v0.9.0
2026-Mar-21 03:29:46.483468
#43 17.43    Compiling rustc_version v0.4.1
2026-Mar-21 03:29:46.483468
#43 17.43    Compiling rfc6979 v0.3.1
2026-Mar-21 03:29:46.483468
#43 17.49    Compiling signature v1.6.4
2026-Mar-21 03:29:46.584450
#43 17.55    Compiling sec1 v0.3.0
2026-Mar-21 03:29:46.584450
#43 17.59    Compiling
2026-Mar-21 03:29:46.691669
foreign-types-shared v0.1.1
2026-Mar-21 03:29:46.691669
#43 17.62    Compiling tokio v1.49.0
2026-Mar-21 03:29:46.691669
#43 17.66    Compiling getrandom v0.3.4
2026-Mar-21 03:29:46.691669
#43 17.69    Compiling simd-adler32 v0.3.8
2026-Mar-21 03:29:46.691669
#43 17.69    Compiling adler2 v2.0.1
2026-Mar-21 03:29:46.691669
#43 17.70    Compiling openssl v0.10.75
2026-Mar-21 03:29:46.691669
#43 17.70    Compiling thiserror v1.0.69
2026-Mar-21 03:29:46.798658
#43 17.74    Compiling minimal-lexical v0.2.1
2026-Mar-21 03:29:46.798658
#43 17.80    Compiling elliptic-curve v0.12.3
2026-Mar-21 03:29:46.929063
#43 17.81    Compiling aws-types v1.3.14
2026-Mar-21 03:29:46.929063
#43 17.84    Compiling foreign-types v0.3.2
2026-Mar-21 03:29:46.929063
#43 17.85    Compiling webpki-roots v0.26.11
2026-Mar-21 03:29:46.929063
#43 17.87    Compiling futures-util v0.3.32
2026-Mar-21 03:29:46.929063
#43 17.87    Compiling miniz_oxide v0.8.9
2026-Mar-21 03:29:46.929063
#43 17.89    Compiling unicode-normalization v0.1.25
2026-Mar-21 03:29:46.929063
#43 17.94    Compiling ring v0.17.14
2026-Mar-21 03:29:46.929063
#43 17.94    Compiling
2026-Mar-21 03:29:47.029711
aws-lc-sys v0.38.0
2026-Mar-21 03:29:47.029711
#43 17.94    Compiling openssl-sys v0.9.111
2026-Mar-21 03:29:47.029711
#43 17.99    Compiling hashlink v0.10.0
2026-Mar-21 03:29:47.029711
#43 18.01    Compiling nom v7.1.3
2026-Mar-21 03:29:47.029711
#43 18.03    Compiling brotli-decompressor v5.0.0
2026-Mar-21 03:29:47.029711
#43 18.03    Compiling crossbeam-queue v0.3.12
2026-Mar-21 03:29:47.029711
#43 18.04    Compiling md-5 v0.10.6
2026-Mar-21 03:29:47.129940
#43 18.10    Compiling libloading v0.8.9
2026-Mar-21 03:29:47.129940
#43 18.11    Compiling unicode-properties v0.1.4
2026-Mar-21 03:29:47.129940
#43 18.14    Compiling crunchy v0.2.4
2026-Mar-21 03:29:47.254211
#43 18.20    Compiling ecdsa v0.14.8
2026-Mar-21 03:29:47.254211
#43 18.22    Compiling native-tls v0.2.18
2026-Mar-21 03:29:47.254211
#43 18.23    Compiling bindgen v0.69.5
2026-Mar-21 03:29:47.254211
#43 18.26    Compiling unicode-bidi v0.3.18
2026-Mar-21 03:29:47.393673
#43 18.30    Compiling anyhow v1.0.102
2026-Mar-21 03:29:47.393673
#43 18.35    Compiling hkdf v0.12.4
2026-Mar-21 03:29:47.393673
#43 18.40    Compiling
2026-Mar-21 03:29:47.556999
flate2 v1.1.9
2026-Mar-21 03:29:47.556999
#43 18.46    Compiling regex-automata v0.4.14
2026-Mar-21 03:29:47.556999
#43 18.49    Compiling crypto-bigint v0.5.5
2026-Mar-21 03:29:47.556999
#43 18.57    Compiling
2026-Mar-21 03:29:47.672565
p256 v0.11.1
2026-Mar-21 03:29:47.672565
#43 18.59    Compiling cookie v0.18.1
2026-Mar-21 03:29:47.672565
#43 18.59    Compiling whoami v1.6.1
2026-Mar-21 03:29:47.672565
#43 18.63    Compiling stringprep v0.1.5
2026-Mar-21 03:29:47.672565
#43 18.65    Compiling dotenvy v0.15.7
2026-Mar-21 03:29:47.672565
#43 18.68    Compiling
2026-Mar-21 03:29:47.773082
lazycell v1.3.0
2026-Mar-21 03:29:47.773082
#43 18.76    Compiling atoi v2.0.0
2026-Mar-21 03:29:47.773082
#43 18.76    Compiling byteorder v1.5.0
2026-Mar-21 03:29:47.773082
#43 18.78    Compiling lazy_static v1.5.0
2026-Mar-21 03:29:47.881026
#43 18.81    Compiling compression-core v0.4.31
2026-Mar-21 03:29:47.888898
#43 18.83    Compiling rustc-hash v1.1.0
2026-Mar-21 03:29:47.888898
#43 18.87    Compiling tiny-keccak v2.0.2
2026-Mar-21 03:29:47.888898
#43 18.89    Compiling adler v1.0.2
2026-Mar-21 03:29:48.037053
#43 18.92    Compiling fastrand v2.3.0
2026-Mar-21 03:29:48.037053
#43 18.94    Compiling ucd-trie v0.1.7
2026-Mar-21 03:29:48.037053
#43 18.97    Compiling brotli v8.0.2
2026-Mar-21 03:29:48.037053
#43 18.98    Compiling miniz_oxide v0.7.4
2026-Mar-21 03:29:48.044969
#43 19.05    Compiling rand_core v0.9.5
2026-Mar-21 03:29:48.162660
#43 19.06    Compiling gzip-header v1.0.0
2026-Mar-21 03:29:48.162660
#43 19.10    Compiling pest v2.8.6
2026-Mar-21 03:29:48.162660
#43 19.17    Compiling fslock v0.2.1
2026-Mar-21 03:29:48.276650
#43 19.21    Compiling encoding_rs v0.8.35
2026-Mar-21 03:29:48.276650
#43 19.23    Compiling radium v0.7.0
2026-Mar-21 03:29:48.276650
#43 19.29    Compiling ppv-lite86 v0.2.21
2026-Mar-21 03:29:48.446271
#43 19.30    Compiling heck v0.5.0
2026-Mar-21 03:29:48.455618
#43 19.30    Compiling litrs v1.0.0
2026-Mar-21 03:29:48.455618
#43 19.37    Compiling synstructure v0.13.2
2026-Mar-21 03:29:48.455618
#43 19.45    Compiling paste v1.0.15
2026-Mar-21 03:29:48.641275
#43 19.47    Compiling hashbrown v0.14.5
2026-Mar-21 03:29:48.641275
#43 19.49    Compiling mime v0.3.17
2026-Mar-21 03:29:48.641275
#43 19.50    Compiling psl-types v2.0.11
2026-Mar-21 03:29:48.641275
#43 19.52    Compiling cexpr v0.6.0
2026-Mar-21 03:29:48.641275
#43 19.65    Compiling rand_chacha v0.3.1
2026-Mar-21 03:29:48.767958
#43 19.68    Compiling rand_chacha v0.9.0
2026-Mar-21 03:29:48.767958
#43 19.71    Compiling arc-swap v1.8.2
2026-Mar-21 03:29:48.767958
#43 19.78    Compiling outref v0.1.0
2026-Mar-21 03:29:48.900570
#43 19.80    Compiling regex-lite v0.1.9
2026-Mar-21 03:29:48.900570
#43 19.83    Compiling heck v0.4.1
2026-Mar-21 03:29:48.900570
#43 19.87    Compiling sha1_smol v1.0.1
2026-Mar-21 03:29:48.900570
#43 19.91    Compiling rand v0.8.5
2026-Mar-21 03:29:48.900570
#43 19.91    Compiling document-features v0.2.12
2026-Mar-21 03:29:49.018654
#43 19.94    Compiling tap v1.0.1
2026-Mar-21 03:29:49.018654
#43 20.02    Compiling iri-string v0.7.10
2026-Mar-21 03:29:49.133777
#43 20.14    Compiling portable-atomic v1.13.1
2026-Mar-21 03:29:49.244312
#43 20.22    Compiling wyz v0.5.1
2026-Mar-21 03:29:49.244312
#43 20.22    Compiling rand v0.9.2
2026-Mar-21 03:29:49.244312
#43 20.23    Compiling simd-abstraction v0.7.1
2026-Mar-21 03:29:49.244312
#43 20.25    Compiling const-random-macro v0.1.16
2026-Mar-21 03:29:49.422579
#43 20.43    Compiling
2026-Mar-21 03:29:49.540188
pest_meta v2.8.6
2026-Mar-21 03:29:49.540188
#43 20.48    Compiling memoffset v0.9.1
2026-Mar-21 03:29:49.540188
#43 20.51    Compiling proc-macro-error-attr v1.0.4
2026-Mar-21 03:29:49.540188
#43 20.53    Compiling funty v2.0.0
2026-Mar-21 03:29:49.540188
#43 20.55    Compiling
2026-Mar-21 03:29:49.641570
xmlparser v0.13.6
2026-Mar-21 03:29:49.641570
#43 20.59    Compiling num-bigint v0.4.6
2026-Mar-21 03:29:49.641570
#43 20.59    Compiling regex v1.12.3
2026-Mar-21 03:29:49.641570
#43 20.62    Compiling syn v1.0.109
2026-Mar-21 03:29:49.641570
#43 20.65    Compiling const-random v0.1.18
2026-Mar-21 03:29:49.750025
#43 20.70    Compiling base64-simd v0.7.0
2026-Mar-21 03:29:49.750025
#43 20.72    Compiling event-listener-strategy v0.5.4
2026-Mar-21 03:29:49.750025
#43 20.73    Compiling serde_path_to_error v0.1.20
2026-Mar-21 03:29:49.750025
#43 20.76    Compiling sha1 v0.10.6
2026-Mar-21 03:29:49.869993
#43 20.76    Compiling proc-macro-error v1.0.4
2026-Mar-21 03:29:49.869993
#43 20.76    Compiling unicode-id-start v1.4.0
2026-Mar-21 03:29:49.869993
#43 20.85    Compiling aws-smithy-xml v0.60.15
2026-Mar-21 03:29:49.981484
#43 20.90    Compiling data-encoding v2.10.0
2026-Mar-21 03:29:49.981484
#43 20.92    Compiling matchit v0.8.4
2026-Mar-21 03:29:49.981484
#43 20.93    Compiling urlencoding v2.1.3
2026-Mar-21 03:29:49.981484
#43 20.93    Compiling if_chain v1.0.3
2026-Mar-21 03:29:49.981484
#43 20.96    Compiling serde_derive v1.0.228
2026-Mar-21 03:29:49.981484
#43 20.99    Compiling tokio-macros v2.6.0
2026-Mar-21 03:29:50.140495
#43 21.00    Compiling zerofrom-derive v0.1.6
2026-Mar-21 03:29:50.149410
#43 21.00    Compiling yoke-derive v0.8.1
2026-Mar-21 03:29:50.149410
#43 21.04    Compiling tracing-attributes v0.1.31
2026-Mar-21 03:29:50.149410
#43 21.06    Compiling futures-macro v0.3.32
2026-Mar-21 03:29:50.149410
#43 21.08    Compiling zerovec-derive v0.11.2
2026-Mar-21 03:29:50.149410
#43 21.15    Compiling
2026-Mar-21 03:29:50.255819
displaydoc v0.2.5
2026-Mar-21 03:29:50.255819
#43 21.20    Compiling thiserror-impl v2.0.18
2026-Mar-21 03:29:50.255819
#43 21.24    Compiling openssl-macros v0.1.1
2026-Mar-21 03:29:50.255819
#43 21.26    Compiling
2026-Mar-21 03:29:50.403070
async-trait v0.1.89
2026-Mar-21 03:29:50.403070
#43 21.30    Compiling tokio-stream v0.1.18
2026-Mar-21 03:29:50.403070
#43 21.41    Compiling thiserror-impl v1.0.69
2026-Mar-21 03:29:50.596075
#43 21.61    Compiling pest_generator v2.8.6
2026-Mar-21 03:29:50.732270
#43 21.63    Compiling strum_macros v0.25.3
2026-Mar-21 03:29:50.732270
#43 21.74    Compiling proc-macro-rules-macros v0.4.0
2026-Mar-21 03:29:50.857879
#43 21.75    Compiling pin-project-internal v1.1.10
2026-Mar-21 03:29:50.857879
#43 21.77    Compiling bitvec v1.0.1
2026-Mar-21 03:29:50.868242
#43 21.87    Compiling
2026-Mar-21 03:29:51.010515
bit-vec v0.6.3
2026-Mar-21 03:29:51.102612
#43 22.11    Compiling bit-set v0.5.3
2026-Mar-21 03:29:51.228526
#43 22.13    Compiling async-lock v3.4.2
2026-Mar-21 03:29:51.228526
#43 22.18    Compiling zerofrom v0.1.6
2026-Mar-21 03:29:51.228526
#43 22.24    Compiling dlv-list v0.5.2
2026-Mar-21 03:29:51.333741
#43 22.24    Compiling crossbeam-epoch v0.9.18
2026-Mar-21 03:29:51.333741
#43 22.27    Compiling tracing v0.1.44
2026-Mar-21 03:29:51.333741
#43 22.29    Compiling yoke v0.8.1
2026-Mar-21 03:29:51.333741
#43 22.29    Compiling crossbeam-channel v0.5.15
2026-Mar-21 03:29:51.333741
#43 22.34    Compiling tagptr v0.2.0
2026-Mar-21 03:29:51.450947
#43 22.37    Compiling proc-macro-rules v0.4.0
2026-Mar-21 03:29:51.450947
#43 22.39    Compiling pest_derive v2.8.6
2026-Mar-21 03:29:51.450947
#43 22.41    Compiling toml_write v0.1.2
2026-Mar-21 03:29:51.450947
#43 22.43    Compiling extractor v0.1.0 (/app/crates/extractor)
2026-Mar-21 03:29:51.450947
#43 22.44    Compiling cooked-waker v5.0.0
2026-Mar-21 03:29:51.450947
#43 22.46    Compiling zerovec v0.11.5
2026-Mar-21 03:29:51.601192
#43 22.48    Compiling zerotrie v0.2.3
2026-Mar-21 03:29:51.601192
#43 22.50    Compiling axum-core v0.5.6
2026-Mar-21 03:29:51.601192
#43 22.51    Compiling tower v0.4.13
2026-Mar-21 03:29:51.601192
#43 22.55    Compiling static_assertions v1.1.0
2026-Mar-21 03:29:51.601192
#43 22.61    Compiling deno_core_icudata v0.0.73
2026-Mar-21 03:29:51.717813
#43 22.62    Compiling winnow v0.7.14
2026-Mar-21 03:29:51.730404
#43 22.63    Compiling ordered-multimap v0.7.3
2026-Mar-21 03:29:51.730404
#43 22.66    Compiling async-stream-impl v0.3.6
2026-Mar-21 03:29:51.730404
#43 22.70    Compiling hashlink v0.8.4
2026-Mar-21 03:29:51.730404
#43 22.70    Compiling raw-cpuid v11.6.0
2026-Mar-21 03:29:51.730404
#43 22.73    Compiling
2026-Mar-21 03:29:51.841862
lru v0.12.5
2026-Mar-21 03:29:51.841862
#43 22.74    Compiling compression-codecs v0.4.37
2026-Mar-21 03:29:51.841862
#43 22.75    Compiling num_cpus v1.17.0
2026-Mar-21 03:29:51.841862
#43 22.78    Compiling base64 v0.21.7
2026-Mar-21 03:29:51.841862
#43 22.81    Compiling pin-project v1.1.10
2026-Mar-21 03:29:51.841862
#43 22.85    Compiling arraydeque v0.5.1
2026-Mar-21 03:29:51.942183
#43 22.90    Compiling unicode-segmentation v1.12.0
2026-Mar-21 03:29:51.942183
#43 22.91    Compiling sharded-slab v0.1.7
2026-Mar-21 03:29:51.942183
#43 22.91    Compiling simple_asn1 v0.6.4
2026-Mar-21 03:29:51.942183
#43 22.93    Compiling rust-ini v0.20.0
2026-Mar-21 03:29:51.942183
#43 22.95    Compiling utoipa-gen v4.3.1
2026-Mar-21 03:29:52.048179
#43 22.95    Compiling pem v3.0.6
2026-Mar-21 03:29:52.048179
#43 23.03    Compiling tinystr v0.8.2
2026-Mar-21 03:29:52.048179
#43 23.04    Compiling potential_utf v0.1.4
2026-Mar-21 03:29:52.048179
#43 23.06    Compiling yaml-rust2 v0.8.1
2026-Mar-21 03:29:52.152503
#43 23.11    Compiling strum v0.25.0
2026-Mar-21 03:29:52.152503
#43 23.13    Compiling async-stream v0.3.6
2026-Mar-21 03:29:52.152503
#43 23.16    Compiling icu_collections v2.1.1
2026-Mar-21 03:29:52.293498
#43 23.18    Compiling icu_locale_core v2.1.1
2026-Mar-21 03:29:52.293498
#43 23.22    Compiling deno_ops v0.176.0
2026-Mar-21 03:29:52.293498
#43 23.23    Compiling tracing-log v0.2.0
2026-Mar-21 03:29:52.293498
#43 23.25    Compiling spinning_top v0.3.0
2026-Mar-21 03:29:52.293498
#43 23.30    Compiling convert_case v0.6.0
2026-Mar-21 03:29:52.394733
#43 23.38    Compiling thread_local v1.1.9
2026-Mar-21 03:29:52.394733
#43 23.40    Compiling web-time v1.1.0
2026-Mar-21 03:29:52.612529
#43 23.43    Compiling nonzero_ext v0.3.0
2026-Mar-21 03:29:52.612529
#43 23.44    Compiling pathdiff v0.2.3
2026-Mar-21 03:29:52.612529
#43 23.45    Compiling futures-timer v3.0.3
2026-Mar-21 03:29:52.612529
#43 23.47    Compiling nu-ansi-term v0.50.3
2026-Mar-21 03:29:52.612529
#43 23.47    Compiling no-std-compat v0.4.1
2026-Mar-21 03:29:52.684708
#43 ...
2026-Mar-21 03:29:52.684708
2026-Mar-21 03:29:52.684708
#47 [frontend builder 7/8] RUN npm run paraglide:compile
2026-Mar-21 03:29:52.693742
#47 0.313
2026-Mar-21 03:29:52.693742
#47 0.313 > frontend@0.0.1 paraglide:compile
2026-Mar-21 03:29:52.693742
#47 0.313 > paraglide-js compile --project ./project.inlang --outdir ./src/lib/paraglide --strategy url cookie globalVariable baseLocale
2026-Mar-21 03:29:52.693742
#47 0.313
2026-Mar-21 03:29:52.693742
#47 0.662 ℹ [paraglide-js] Compiling inlang project ...
2026-Mar-21 03:29:52.693742
#47 10.03 ✔ [paraglide-js] Successfully compiled inlang project.
2026-Mar-21 03:29:52.839997
#47 ...
2026-Mar-21 03:29:52.839997
2026-Mar-21 03:29:52.839997
#43 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-21 03:29:52.839997
#43 23.85    Compiling icu_provider v2.1.1
2026-Mar-21 03:29:53.072302
#43 24.08    Compiling icu_properties v2.1.2
2026-Mar-21 03:29:53.072302
#43 24.08    Compiling rustls-webpki v0.103.9
2026-Mar-21 03:29:53.290877
#43 ...
2026-Mar-21 03:29:53.290877
2026-Mar-21 03:29:53.290877
#47 [frontend builder 7/8] RUN npm run paraglide:compile
2026-Mar-21 03:29:53.290877
#47 DONE 10.5s
2026-Mar-21 03:29:53.356574
#43 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-21 03:29:53.356574
#43 24.37    Compiling either v1.15.0
2026-Mar-21 03:29:53.468881
#43 24.41    Compiling smallvec v1.15.1
2026-Mar-21 03:29:53.468881
#43 24.42    Compiling serde_urlencoded v0.7.1
2026-Mar-21 03:29:53.468881
#43 24.42    Compiling debugid v0.8.0
2026-Mar-21 03:29:53.468881
#43 24.42    Compiling serde_spanned v0.6.9
2026-Mar-21 03:29:53.634767
#43 24.50    Compiling toml_datetime v0.6.11
2026-Mar-21 03:29:53.634767
#43 24.52    Compiling bincode v1.3.3
2026-Mar-21 03:29:53.634767
#43 24.52    Compiling json5 v0.4.1
2026-Mar-21 03:29:53.634767
#43 24.52    Compiling matchers v0.2.0
2026-Mar-21 03:29:53.634767
#43 24.56    Compiling which v4.4.2
2026-Mar-21 03:29:53.634767
#43 24.56    Compiling itertools v0.12.1
2026-Mar-21 03:29:53.634767
#43 24.57    Compiling which v6.0.3
2026-Mar-21 03:29:53.634767
#43 24.64    Compiling icu_normalizer v2.1.1
2026-Mar-21 03:29:53.744590
#43 24.74    Compiling bytes-utils v0.1.4
2026-Mar-21 03:29:53.744590
#43 24.75    Compiling itertools v0.13.0
2026-Mar-21 03:29:53.852479
#43 24.81    Compiling futures-executor v0.3.32
2026-Mar-21 03:29:53.852479
#43 24.82    Compiling toml_edit v0.22.27
2026-Mar-21 03:29:53.852479
#43 24.83    Compiling ron v0.8.1
2026-Mar-21 03:29:53.852479
#43 24.86    Compiling quanta v0.12.6
2026-Mar-21 03:29:53.955697
#43 24.88    Compiling parking_lot v0.12.5
2026-Mar-21 03:29:53.955697
#43 24.93    Compiling dashmap v6.1.0
2026-Mar-21 03:29:53.955697
#43 24.96    Compiling tracing-subscriber v0.3.22
2026-Mar-21 03:29:54.095155
#43 24.98    Compiling futures v0.3.32
2026-Mar-21 03:29:54.095155
#43 25.10    Compiling
2026-Mar-21 03:29:54.287114
idna_adapter v1.2.1
2026-Mar-21 03:29:54.287114
#43 25.12    Compiling futures-intrusive v0.5.0
2026-Mar-21 03:29:54.287114
#43 25.17    Compiling moka v0.12.13
2026-Mar-21 03:29:54.287114
#43 25.19    Compiling idna v1.1.0
2026-Mar-21 03:29:54.287114
#43 25.30    Compiling governor v0.8.1
2026-Mar-21 03:29:54.525550
#43 25.53    Compiling url v2.5.8
2026-Mar-21 03:29:54.957861
#43 25.89    Compiling utoipa v4.2.3
2026-Mar-21 03:29:54.957861
#43 25.94    Compiling publicsuffix v2.3.0
2026-Mar-21 03:29:54.970071
2026-Mar-21 03:29:56.042812
#43 26.99    Compiling cookie_store v0.22.1
2026-Mar-21 03:29:56.042812
#43 26.99    Compiling sourcemap v8.0.1
2026-Mar-21 03:29:56.153724
#43 ...
2026-Mar-21 03:29:56.163659
#44 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-21 03:29:56.163659
#44 17.12   Downloaded deno_core_icudata v0.0.73
2026-Mar-21 03:29:56.163659
#44 20.92   Downloaded aws-lc-sys v0.38.0
2026-Mar-21 03:29:56.163659
#44 24.52   Downloaded v8 v0.101.0
2026-Mar-21 03:29:56.163659
#44 26.15    Compiling proc-macro2 v1.0.106
2026-Mar-21 03:29:56.163659
#44 26.15    Compiling quote v1.0.44
2026-Mar-21 03:29:56.163659
#44 26.15    Compiling unicode-ident v1.0.24
2026-Mar-21 03:29:56.163659
#44 26.15    Compiling libc v0.2.182
2026-Mar-21 03:29:56.163659
#44 26.18    Compiling cfg-if v1.0.4
2026-Mar-21 03:29:56.163659
#44 26.18    Compiling serde v1.0.228
2026-Mar-21 03:29:56.163659
#44 26.18    Compiling serde_core v1.0.228
2026-Mar-21 03:29:56.163659
#44 26.18    Compiling pin-project-lite v0.2.16
2026-Mar-21 03:29:56.163659
#44 26.18    Compiling parking_lot_core v0.9.12
2026-Mar-21 03:29:56.163659
#44 26.19    Compiling shlex v1.3.0
2026-Mar-21 03:29:56.163659
#44 26.19    Compiling bytes v1.11.1
2026-Mar-21 03:29:56.163659
#44 26.19    Compiling scopeguard v1.2.0
2026-Mar-21 03:29:56.163659
#44 26.21    Compiling futures-core v0.3.32
2026-Mar-21 03:29:56.163659
#44 26.21    Compiling find-msvc-tools v0.1.9
2026-Mar-21 03:29:56.163659
#44 26.21    Compiling version_check v0.9.5
2026-Mar-21 03:29:56.163659
#44 26.21    Compiling memchr v2.8.0
2026-Mar-21 03:29:56.163659
#44 26.21    Compiling itoa v1.0.17
2026-Mar-21 03:29:56.163659
#44 26.21    Compiling once_cell v1.21.3
2026-Mar-21 03:29:56.163659
#44 26.21    Compiling futures-sink v0.3.32
2026-Mar-21 03:29:56.163659
#44 26.25    Compiling typenum v1.19.0
2026-Mar-21 03:29:56.163659
#44 26.27    Compiling autocfg v1.5.0
2026-Mar-21 03:29:56.163659
#44 26.27    Compiling log v0.4.29
2026-Mar-21 03:29:56.163659
#44 26.27    Compiling slab v0.4.12
2026-Mar-21 03:29:56.163659
#44 26.27    Compiling futures-io v0.3.32
2026-Mar-21 03:29:56.163659
#44 26.27    Compiling zeroize v1.8.2
2026-Mar-21 03:29:56.163659
#44 26.27    Compiling futures-task v0.3.32
2026-Mar-21 03:29:56.163659
#44 26.27    Compiling subtle v2.6.1
2026-Mar-21 03:29:56.163659
#44 26.27    Compiling fnv v1.0.7
2026-Mar-21 03:29:56.163659
#44 26.27    Compiling equivalent v1.0.2
2026-Mar-21 03:29:56.163659
#44 26.27    Compiling percent-encoding v2.3.2
2026-Mar-21 03:29:56.163659
#44 26.27    Compiling icu_properties_data v2.1.2
2026-Mar-21 03:29:56.163659
#44 26.27    Compiling icu_normalizer_data v2.1.1
2026-Mar-21 03:29:56.163659
#44 26.29    Compiling zerocopy v0.8.39
2026-Mar-21 03:29:56.163659
#44 26.32    Compiling pin-utils v0.1.0
2026-Mar-21 03:29:56.163659
#44 26.44    Compiling lock_api v0.4.14
2026-Mar-21 03:29:56.163659
#44 26.52    Compiling futures-channel v0.3.32
2026-Mar-21 03:29:56.163659
#44 26.52    Compiling hashbrown v0.16.1
2026-Mar-21 03:29:56.163659
#44 26.52    Compiling num-conv v0.2.0
2026-Mar-21 03:29:56.163659
#44 26.60    Compiling powerfmt v0.2.0
2026-Mar-21 03:29:56.163659
#44 26.69    Compiling ryu v1.0.23
2026-Mar-21 03:29:56.163659
#44 26.69    Compiling time-core v0.1.8
2026-Mar-21 03:29:56.163659
#44 26.73    Compiling crc32fast v1.5.0
2026-Mar-21 03:29:56.163659
#44 26.78    Compiling stable_deref_trait v1.2.1
2026-Mar-21 03:29:56.163659
#44 26.84    Compiling tracing-core v0.1.36
2026-Mar-21 03:29:56.163659
#44 26.86    Compiling untrusted v0.9.0
2026-Mar-21 03:29:56.163659
#44 26.89    Compiling fs_extra v1.3.0
2026-Mar-21 03:29:56.163659
#44 26.97    Compiling dunce v1.0.5
2026-Mar-21 03:29:56.163659
#44 26.97    Compiling num-traits v0.2.19
2026-Mar-21 03:29:56.163659
#44 26.97    Compiling rustls-pki-types v1.14.0
2026-Mar-21 03:29:56.163659
#44 27.04    Compiling form_urlencoded v1.2.2
2026-Mar-21 03:29:56.163659
#44 27.06    Compiling http v1.4.0
2026-Mar-21 03:29:56.163659
#44 27.12    Compiling http v0.2.12
2026-Mar-21 03:29:56.163659
#44 27.20    Compiling writeable v0.6.2
2026-Mar-21 03:29:56.324167
#44 27.20    Compiling tower-service v0.3.3
2026-Mar-21 03:29:56.333663
#44 27.20    Compiling litemap v0.8.1
2026-Mar-21 03:29:56.333663
#44 27.22    Compiling aws-lc-rs v1.16.1
2026-Mar-21 03:29:56.333663
#44 27.24    Compiling generic-array v0.14.7
2026-Mar-21 03:29:56.333663
#44 27.25    Compiling httparse v1.10.1
2026-Mar-21 03:29:56.333663
#44 27.25    Compiling outref v0.5.2
2026-Mar-21 03:29:56.333663
#44 27.26    Compiling deranged v0.5.8
2026-Mar-21 03:29:56.333663
#44 27.26    Compiling zmij v1.0.21
2026-Mar-21 03:29:56.333663
#44 27.26    Compiling vsimd v0.8.0
2026-Mar-21 03:29:56.333663
#44 27.26    Compiling time-macros v0.2.27
2026-Mar-21 03:29:56.333663
#44 27.37    Compiling try-lock v0.2.5
2026-Mar-21 03:29:56.333663
#44 ...
2026-Mar-21 03:29:56.333663
2026-Mar-21 03:29:56.333663
#43 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-21 03:29:56.333663
#43 27.19    Compiling toml v0.8.23
2026-Mar-21 03:29:57.162023
#43 28.17    Compiling config v0.14.1
2026-Mar-21 03:29:58.332056
#43 29.33    Compiling sct v0.7.1
2026-Mar-21 03:29:58.491010
#43 29.39    Compiling rustls-webpki v0.101.7
2026-Mar-21 03:29:58.491010
#43 29.50    Compiling jsonwebtoken v9.3.1
2026-Mar-21 03:29:58.599650
#43 29.59    Compiling sqlx-core v0.8.6
2026-Mar-21 03:29:59.994969
#43 30.95    Compiling tokio-util v0.7.18
2026-Mar-21 03:30:00.115060
#43 31.01    Compiling aws-smithy-async v1.2.14
2026-Mar-21 03:30:00.115060
#43 31.01    Compiling tower v0.5.3
2026-Mar-21 03:30:00.115060
#43 31.12    Compiling async-compression v0.4.41
2026-Mar-21 03:30:00.227780
#43 31.12    Compiling tokio-native-tls v0.3.1
2026-Mar-21 03:30:00.227780
#43 31.12    Compiling deno_unsync v0.4.4
2026-Mar-21 03:30:00.778533
#43 31.70    Compiling aws-smithy-types v1.4.6
2026-Mar-21 03:30:00.875960
#43 31.86    Compiling h2 v0.4.13
2026-Mar-21 03:30:00.989001
#43 31.99    Compiling h2 v0.3.27
2026-Mar-21 03:30:01.140685
#43 32.08    Compiling combine v4.6.7
2026-Mar-21 03:30:01.238603
#43 32.19    Compiling tower-http v0.6.8
2026-Mar-21 03:30:01.735586
#43 32.74    Compiling aws-smithy-runtime-api v1.11.6
2026-Mar-21 03:30:01.845595
#43 32.74    Compiling aws-smithy-eventstream v0.60.20
2026-Mar-21 03:30:01.845595
#43 32.74    Compiling aws-smithy-json v0.62.5
2026-Mar-21 03:30:01.845595
#43 32.76    Compiling aws-smithy-query v0.60.15
2026-Mar-21 03:30:01.845595
#43 32.76    Compiling aws-smithy-json v0.61.9
2026-Mar-21 03:30:01.845595
#43 32.85    Compiling v8 v0.101.0
2026-Mar-21 03:30:02.231283
#43 ...
2026-Mar-21 03:30:02.231283
2026-Mar-21 03:30:02.231283
#44 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-21 03:30:02.231283
#44 27.53    Compiling httpdate v1.0.3
2026-Mar-21 03:30:02.231283
#44 27.53    Compiling rustls v0.23.37
2026-Mar-21 03:30:02.231283
#44 27.55    Compiling base64 v0.22.1
2026-Mar-21 03:30:02.231283
#44 27.56    Compiling atomic-waker v1.1.2
2026-Mar-21 03:30:02.231283
#44 27.59    Compiling tower-layer v0.3.3
2026-Mar-21 03:30:02.231283
#44 27.60    Compiling openssl-probe v0.2.1
2026-Mar-21 03:30:02.231283
#44 27.62    Compiling cpufeatures v0.2.17
2026-Mar-21 03:30:02.231283
#44 27.64    Compiling utf8_iter v1.0.4
2026-Mar-21 03:30:02.231283
#44 27.69    Compiling webpki-roots v1.0.6
2026-Mar-21 03:30:02.231283
#44 27.73    Compiling want v0.3.1
2026-Mar-21 03:30:02.231283
#44 27.73    Compiling sync_wrapper v1.0.2
2026-Mar-21 03:30:02.231283
#44 27.77    Compiling ipnet v2.11.0
2026-Mar-21 03:30:02.231283
#44 27.77    Compiling crossbeam-utils v0.8.21
2026-Mar-21 03:30:02.231283
#44 27.93    Compiling rustversion v1.0.22
2026-Mar-21 03:30:02.231283
#44 27.99    Compiling bitflags v2.11.0
2026-Mar-21 03:30:02.231283
#44 28.01    Compiling home v0.5.12
2026-Mar-21 03:30:02.231283
#44 28.01    Compiling getrandom v0.4.1
2026-Mar-21 03:30:02.231283
#44 28.22    Compiling base64-simd v0.8.0
2026-Mar-21 03:30:02.231283
#44 28.26    Compiling rustls-native-certs v0.8.3
2026-Mar-21 03:30:02.231283
#44 28.30    Compiling http-body v0.4.6
2026-Mar-21 03:30:02.231283
#44 28.35    Compiling const-oid v0.9.6
2026-Mar-21 03:30:02.231283
#44 28.36    Compiling hex v0.4.3
2026-Mar-21 03:30:02.231283
#44 28.43    Compiling syn v2.0.117
2026-Mar-21 03:30:02.231283
#44 28.48    Compiling thiserror v2.0.18
2026-Mar-21 03:30:02.231283
#44 28.48    Compiling serde_json v1.0.149
2026-Mar-21 03:30:02.231283
#44 28.57    Compiling errno v0.3.14
2026-Mar-21 03:30:02.231283
#44 28.64    Compiling mio v1.1.1
2026-Mar-21 03:30:02.231283
#44 28.71    Compiling socket2 v0.6.2
2026-Mar-21 03:30:02.231283
#44 28.76    Compiling getrandom v0.2.17
2026-Mar-21 03:30:02.231283
#44 28.76    Compiling http-body v1.0.1
2026-Mar-21 03:30:02.231283
#44 28.80    Compiling socket2 v0.5.10
2026-Mar-21 03:30:02.231283
#44 28.95    Compiling signal-hook-registry v1.4.8
2026-Mar-21 03:30:02.231283
#44 29.00    Compiling http-body-util v0.1.3
2026-Mar-21 03:30:02.231283
#44 29.00    Compiling rand_core v0.6.4
2026-Mar-21 03:30:02.231283
#44 29.00    Compiling der v0.6.1
2026-Mar-21 03:30:02.231283
#44 29.07    Compiling crypto-common v0.1.7
2026-Mar-21 03:30:02.231283
#44 29.07    Compiling block-buffer v0.10.4
2026-Mar-21 03:30:02.231283
#44 29.17    Compiling allocator-api2 v0.2.21
2026-Mar-21 03:30:02.231283
#44 29.17    Compiling base64ct v1.8.3
2026-Mar-21 03:30:02.231283
#44 29.21    Compiling pkg-config v0.3.32
2026-Mar-21 03:30:02.231283
#44 29.65    Compiling vcpkg v0.2.15
2026-Mar-21 03:30:02.231283
#44 29.65    Compiling uuid v1.21.0
2026-Mar-21 03:30:02.231283
#44 29.70    Compiling jobserver v0.1.34
2026-Mar-21 03:30:02.231283
#44 29.70    Compiling num-integer v0.1.46
2026-Mar-21 03:30:02.231283
#44 29.70    Compiling time v0.3.47
2026-Mar-21 03:30:02.231283
#44 29.70    Compiling digest v0.10.7
2026-Mar-21 03:30:02.231283
#44 29.73    Compiling rustls v0.21.12
2026-Mar-21 03:30:02.231283
#44 29.78    Compiling ff v0.12.1
2026-Mar-21 03:30:02.231283
#44 29.79    Compiling crypto-bigint v0.4.9
2026-Mar-21 03:30:02.231283
#44 29.83    Compiling base16ct v0.1.1
2026-Mar-21 03:30:02.231283
#44 29.83    Compiling foldhash v0.1.5
2026-Mar-21 03:30:02.231283
#44 29.83    Compiling rustix v0.38.44
2026-Mar-21 03:30:02.231283
#44 29.83    Compiling crc-catalog v2.4.0
2026-Mar-21 03:30:02.231283
#44 29.86    Compiling glob v0.3.3
2026-Mar-21 03:30:02.231283
#44 30.15    Compiling group v0.12.1
2026-Mar-21 03:30:02.231283
#44 30.17    Compiling hashbrown v0.15.5
2026-Mar-21 03:30:02.231283
#44 30.18    Compiling concurrent-queue v2.5.0
2026-Mar-21 03:30:02.231283
#44 30.23    Compiling crc v3.4.0
2026-Mar-21 03:30:02.231283
#44 30.36    Compiling cc v1.2.56
2026-Mar-21 03:30:02.231283
#44 30.36    Compiling hmac v0.12.1
2026-Mar-21 03:30:02.231283
#44 30.36    Compiling sha2 v0.10.9
2026-Mar-21 03:30:02.231283
#44 30.40    Compiling prettyplease v0.2.37
2026-Mar-21 03:30:02.231283
#44 30.40    Compiling alloc-no-stdlib v2.0.4
2026-Mar-21 03:30:02.231283
#44 30.56    Compiling spki v0.6.0
2026-Mar-21 03:30:02.231283
#44 30.56    Compiling semver v1.0.27
2026-Mar-21 03:30:02.231283
#44 30.64    Compiling tinyvec_macros v0.1.1
2026-Mar-21 03:30:02.231283
#44 30.64    Compiling clang-sys v1.8.1
2026-Mar-21 03:30:02.231283
#44 30.64    Compiling linux-raw-sys v0.4.15
2026-Mar-21 03:30:02.231283
#44 30.69    Compiling parking v2.2.1
2026-Mar-21 03:30:02.231283
#44 30.91    Compiling alloc-stdlib v0.2.2
2026-Mar-21 03:30:02.231283
#44 30.95    Compiling tinyvec v1.10.0
2026-Mar-21 03:30:02.231283
#44 30.96    Compiling pkcs8 v0.9.0
2026-Mar-21 03:30:02.231283
#44 30.96    Compiling signature v1.6.4
2026-Mar-21 03:30:02.231283
#44 31.04    Compiling event-listener v5.4.1
2026-Mar-21 03:30:02.231283
#44 31.11    Compiling rustc_version v0.4.1
2026-Mar-21 03:30:02.231283
#44 31.20    Compiling aho-corasick v1.1.4
2026-Mar-21 03:30:02.231283
#44 31.28    Compiling indexmap v2.13.0
2026-Mar-21 03:30:02.231283
#44 31.29    Compiling regex-syntax v0.8.10
2026-Mar-21 03:30:02.231283
#44 31.32    Compiling openssl v0.10.75
2026-Mar-21 03:30:02.231283
#44 31.38    Compiling thiserror v1.0.69
2026-Mar-21 03:30:02.231283
#44 31.52    Compiling foreign-types-shared v0.1.1
2026-Mar-21 03:30:02.231283
#44 31.52    Compiling sec1 v0.3.0
2026-Mar-21 03:30:02.231283
#44 31.52    Compiling tokio v1.49.0
2026-Mar-21 03:30:02.231283
#44 31.68    Compiling rfc6979 v0.3.1
2026-Mar-21 03:30:02.231283
#44 31.68    Compiling minimal-lexical v0.2.1
2026-Mar-21 03:30:02.231283
#44 31.82    Compiling simd-adler32 v0.3.8
2026-Mar-21 03:30:02.231283
#44 31.84    Compiling adler2 v2.0.1
2026-Mar-21 03:30:02.231283
#44 31.92    Compiling foreign-types v0.3.2
2026-Mar-21 03:30:02.231283
#44 32.04    Compiling unicode-normalization v0.1.25
2026-Mar-21 03:30:02.231283
#44 32.04    Compiling cmake v0.1.57
2026-Mar-21 03:30:02.231283
#44 32.04    Compiling elliptic-curve v0.12.3
2026-Mar-21 03:30:02.231283
#44 32.04    Compiling aws-types v1.3.14
2026-Mar-21 03:30:02.231283
#44 32.07    Compiling nom v7.1.3
2026-Mar-21 03:30:02.231283
#44 32.07    Compiling brotli-decompressor v5.0.0
2026-Mar-21 03:30:02.231283
#44 32.18    Compiling webpki-roots v0.26.11
2026-Mar-21 03:30:02.231283
#44 32.22    Compiling crossbeam-queue v0.3.12
2026-Mar-21 03:30:02.231283
#44 32.28    Compiling md-5 v0.10.6
2026-Mar-21 03:30:02.231283
#44 32.32    Compiling miniz_oxide v0.8.9
2026-Mar-21 03:30:02.231283
#44 32.38    Compiling hashlink v0.10.0
2026-Mar-21 03:30:02.231283
#44 32.46    Compiling libloading v0.8.9
2026-Mar-21 03:30:02.231283
#44 32.58    Compiling futures-util v0.3.32
2026-Mar-21 03:30:02.231283
#44 32.58    Compiling native-tls v0.2.18
2026-Mar-21 03:30:02.231283
#44 32.59    Compiling bindgen v0.69.5
2026-Mar-21 03:30:02.231283
#44 32.63    Compiling getrandom v0.3.4
2026-Mar-21 03:30:02.231283
#44 32.63    Compiling unicode-bidi v0.3.18
2026-Mar-21 03:30:02.231283
#44 32.63    Compiling anyhow v1.0.102
2026-Mar-21 03:30:02.231283
#44 32.66    Compiling unicode-properties v0.1.4
2026-Mar-21 03:30:02.231283
#44 32.92    Compiling hkdf v0.12.4
2026-Mar-21 03:30:02.231283
#44 32.94    Compiling ecdsa v0.14.8
2026-Mar-21 03:30:02.231283
#44 33.17    Compiling crypto-bigint v0.5.5
2026-Mar-21 03:30:02.231283
#44 33.28    Compiling cookie v0.18.1
2026-Mar-21 03:30:02.368904
#44 33.34    Compiling regex-automata v0.4.14
2026-Mar-21 03:30:02.368904
#44 33.34    Compiling p256 v0.11.1
2026-Mar-21 03:30:02.368904
#44 33.34    Compiling flate2 v1.1.9
2026-Mar-21 03:30:02.368904
#44 33.41    Compiling lazy_static v1.5.0
2026-Mar-21 03:30:02.482637
#44 33.43    Compiling whoami v1.6.1
2026-Mar-21 03:30:02.482637
#44 33.51    Compiling ppv-lite86 v0.2.21
2026-Mar-21 03:30:02.482637
#44 33.53    Compiling stringprep v0.1.5
2026-Mar-21 03:30:02.587324
#44 33.56    Compiling rustc-hash v1.1.0
2026-Mar-21 03:30:02.587324
#44 33.56    Compiling lazycell v1.3.0
2026-Mar-21 03:30:02.587324
#44 33.64    Compiling atoi v2.0.0
2026-Mar-21 03:30:02.718582
#44 33.64    Compiling byteorder v1.5.0
2026-Mar-21 03:30:02.718582
#44 33.71    Compiling compression-core v0.4.31
2026-Mar-21 03:30:02.718582
#44 33.76    Compiling ring v0.17.14
2026-Mar-21 03:30:02.834066
#44 33.80    Compiling aws-lc-sys v0.38.0
2026-Mar-21 03:30:02.834066
#44 33.83    Compiling synstructure v0.13.2
2026-Mar-21 03:30:02.834066
#44 33.83    Compiling openssl-sys v0.9.111
2026-Mar-21 03:30:02.834066
#44 33.88    Compiling dotenvy v0.15.7
2026-Mar-21 03:30:02.971585
#44 33.94    Compiling adler v1.0.2
2026-Mar-21 03:30:02.971585
#44 33.98    Compiling
2026-Mar-21 03:30:03.084100
fastrand v2.3.0
2026-Mar-21 03:30:03.102895
#44 34.15    Compiling miniz_oxide v0.7.4
2026-Mar-21 03:30:03.245128
#44 34.29    Compiling rand_chacha v0.3.1
2026-Mar-21 03:30:03.245128
#44 ...
2026-Mar-21 03:30:03.245128
2026-Mar-21 03:30:03.245128
#48 [frontend builder 8/8] RUN node build-docker.mjs
2026-Mar-21 03:30:03.245128
#48 1.716 The following Vite config options will be overridden by SvelteKit:
2026-Mar-21 03:30:03.245128
#48 1.716   - build.outDir
2026-Mar-21 03:30:03.245128
#48 1.757 vite v6.4.1 building SSR bundle for production...
2026-Mar-21 03:30:03.245128
#48 1.820 transforming...
2026-Mar-21 03:30:03.245128
#48 2.905 3:29:56 AM [vite-plugin-svelte] src/routes/terms/+page.svelte:176:1 Unused CSS selector ".legal-page li strong"
2026-Mar-21 03:30:03.245128
#48 2.905 https://svelte.dev/e/css_unused_selector
2026-Mar-21 03:30:03.245128
#48 2.905 174:   }
2026-Mar-21 03:30:03.245128
#48 2.905 175:
2026-Mar-21 03:30:03.245128
#48 2.905 176:   .legal-page li strong {
2026-Mar-21 03:30:03.245128
#48 2.905                             ^
2026-Mar-21 03:30:03.245128
#48 2.905 177:     color: #2d1b36;
2026-Mar-21 03:30:03.245128
#48 2.905 178:   }
2026-Mar-21 03:30:03.245128
#48 2.905 3:29:56 AM [vite-plugin-svelte] src/routes/terms/+page.svelte:236:1 Unused CSS selector ":global(.app.theme-dark) .legal-page li strong"
2026-Mar-21 03:30:03.245128
#48 2.905 https://svelte.dev/e/css_unused_selector
2026-Mar-21 03:30:03.245128
#48 2.905 234:   }
2026-Mar-21 03:30:03.245128
#48 2.905 235:
2026-Mar-21 03:30:03.245128
#48 2.905 236:   :global(.app.theme-dark) .legal-page li strong {
2026-Mar-21 03:30:03.245128
#48 2.905                                                      ^
2026-Mar-21 03:30:03.245128
#48 2.905 237:     color: #ffffff;
2026-Mar-21 03:30:03.245128
#48 2.905 238:   }
2026-Mar-21 03:30:03.349651
#48 ...
2026-Mar-21 03:30:03.349651
2026-Mar-21 03:30:03.349651
#44 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-21 03:30:03.349651
#44 34.31    Compiling gzip-header v1.0.0
2026-Mar-21 03:30:03.349651
#44 34.39    Compiling
2026-Mar-21 03:30:03.494003
fslock v0.2.1
2026-Mar-21 03:30:03.560640
#44 34.60    Compiling ahash v0.8.12
2026-Mar-21 03:30:03.798604
#44 34.68    Compiling psl-types v2.0.11
2026-Mar-21 03:30:03.798604
#44 34.83    Compiling cexpr v0.6.0
2026-Mar-21 03:30:03.903601
#44 34.89    Compiling mime v0.3.17
2026-Mar-21 03:30:03.903601
#44 34.95    Compiling ucd-trie v0.1.7
2026-Mar-21 03:30:03.903601
#44 34.95    Compiling paste v1.0.15
2026-Mar-21 03:30:04.136052
#44 34.99    Compiling brotli v8.0.2
2026-Mar-21 03:30:04.136052
#44 35.03    Compiling litrs v1.0.0
2026-Mar-21 03:30:04.198957
#44 35.25    Compiling rand v0.8.5
2026-Mar-21 03:30:04.415211
#44 35.25    Compiling radium v0.7.0
2026-Mar-21 03:30:04.415211
#44 35.29    Compiling heck v0.5.0
2026-Mar-21 03:30:04.415211
#44 35.31    Compiling rand_core v0.9.5
2026-Mar-21 03:30:04.448234
#44 35.50    Compiling pest v2.8.6
2026-Mar-21 03:30:04.560198
#44 35.61    Compiling arc-swap v1.8.2
2026-Mar-21 03:30:04.762647
#44 35.64    Compiling iri-string v0.7.10
2026-Mar-21 03:30:04.762647
#44 35.66    Compiling heck v0.4.1
2026-Mar-21 03:30:04.856102
#44 35.90    Compiling outref v0.1.0
2026-Mar-21 03:30:04.970161
#44 35.90    Compiling sha1_smol v1.0.1
2026-Mar-21 03:30:04.981952
#44 35.95    Compiling tap v1.0.1
2026-Mar-21 03:30:04.981952
#44 35.95    Compiling regex-lite v0.1.9
2026-Mar-21 03:30:04.981952
#44 36.01    Compiling simd-abstraction v0.7.1
2026-Mar-21 03:30:05.085120
#44 36.11    Compiling wyz v0.5.1
2026-Mar-21 03:30:05.085120
#44 36.13    Compiling document-features v0.2.12
2026-Mar-21 03:30:05.281365
#44 36.18    Compiling rand_chacha v0.9.0
2026-Mar-21 03:30:05.281365
#44 36.33    Compiling memoffset v0.9.1
2026-Mar-21 03:30:05.434815
#44 36.36    Compiling encoding_rs v0.8.35
2026-Mar-21 03:30:05.434815
#44 36.39    Compiling regex v1.12.3
2026-Mar-21 03:30:05.434815
#44 36.42    Compiling serde_derive v1.0.228
2026-Mar-21 03:30:05.434815
#44 36.48    Compiling tokio-macros v2.6.0
2026-Mar-21 03:30:05.672243
#44 36.52    Compiling zerofrom-derive v0.1.6
2026-Mar-21 03:30:05.672243
#44 36.55    Compiling yoke-derive v0.8.1
2026-Mar-21 03:30:05.672243
#44 36.55    Compiling tracing-attributes v0.1.31
2026-Mar-21 03:30:05.672243
#44 36.58    Compiling zerovec-derive v0.11.2
2026-Mar-21 03:30:05.672243
#44 36.72    Compiling futures-macro v0.3.32
2026-Mar-21 03:30:05.817011
#44 36.81    Compiling displaydoc v0.2.5
2026-Mar-21 03:30:05.817011
#44 36.87    Compiling thiserror-impl v2.0.18
2026-Mar-21 03:30:05.946455
#44 36.90    Compiling tokio-stream v0.1.18
2026-Mar-21 03:30:05.946455
#44 36.92    Compiling openssl-macros v0.1.1
2026-Mar-21 03:30:05.946455
#44 37.00    Compiling num-bigint v0.4.6
2026-Mar-21 03:30:06.196673
#44 37.10    Compiling thiserror-impl v1.0.69
2026-Mar-21 03:30:06.211327
#44 37.26    Compiling async-trait v0.1.89
2026-Mar-21 03:30:06.348251
#44 37.29    Compiling proc-macro-rules-macros v0.4.0
2026-Mar-21 03:30:06.348251
#44 37.35    Compiling strum_macros v0.25.3
2026-Mar-21 03:30:06.495655
#44 37.48    Compiling xmlparser v0.13.6
2026-Mar-21 03:30:06.806903
#44 37.69    Compiling pest_meta v2.8.6
2026-Mar-21 03:30:06.806903
#44 37.72    Compiling portable-atomic v1.13.1
2026-Mar-21 03:30:07.023035
#44 37.92    Compiling funty v2.0.0
2026-Mar-21 03:30:07.056355
#44 38.10    Compiling aws-smithy-xml v0.60.15
2026-Mar-21 03:30:07.435570
#44 38.48    Compiling rand v0.9.2
2026-Mar-21 03:30:07.617588
#44 38.57    Compiling bitvec v1.0.1
2026-Mar-21 03:30:07.617588
#44 38.67    Compiling zerofrom v0.1.6
2026-Mar-21 03:30:07.723479
#44 38.71    Compiling pest_generator v2.8.6
2026-Mar-21 03:30:07.723479
#44 38.72    Compiling hashbrown v0.14.5
2026-Mar-21 03:30:07.723479
#44 38.77    Compiling pin-project-internal v1.1.10
2026-Mar-21 03:30:07.936893
#44 38.83    Compiling base64-simd v0.7.0
2026-Mar-21 03:30:07.936893
#44 38.85    Compiling event-listener-strategy v0.5.4
2026-Mar-21 03:30:07.936893
#44 38.86    Compiling yoke v0.8.1
2026-Mar-21 03:30:07.936893
#44 38.99    Compiling serde_path_to_error v0.1.20
2026-Mar-21 03:30:08.174479
#44 39.22    Compiling proc-macro-rules v0.4.0
2026-Mar-21 03:30:08.307158
#44 39.26    Compiling zerovec v0.11.5
2026-Mar-21 03:30:08.307158
#44 39.30    Compiling tracing v0.1.44
2026-Mar-21 03:30:08.307158
#44 39.36    Compiling zerotrie v0.2.3
2026-Mar-21 03:30:08.425217
#44 39.42    Compiling sha1 v0.10.6
2026-Mar-21 03:30:08.425217
#44 39.47    Compiling matchit v0.8.4
2026-Mar-21 03:30:08.589724
#44 39.49    Compiling unicode-id-start v1.4.0
2026-Mar-21 03:30:08.708480
#44 39.76    Compiling axum-core v0.5.6
2026-Mar-21 03:30:08.882036
#44 39.82    Compiling bit-vec v0.6.3
2026-Mar-21 03:30:08.882036
#44 39.85    Compiling if_chain v1.0.3
2026-Mar-21 03:30:08.882036
#44 39.85    Compiling data-encoding v2.10.0
2026-Mar-21 03:30:08.882036
#44 39.93    Compiling tinystr v0.8.2
2026-Mar-21 03:30:08.995977
#44 39.97    Compiling potential_utf v0.1.4
2026-Mar-21 03:30:08.995977
#44 40.00    Compiling urlencoding v2.1.3
2026-Mar-21 03:30:08.995977
#44 40.03    Compiling tower v0.4.13
2026-Mar-21 03:30:09.103332
#44 40.07    Compiling async-lock v3.4.2
2026-Mar-21 03:30:09.103332
#44 40.08    Compiling strum v0.25.0
2026-Mar-21 03:30:09.103332
#44 40.15    Compiling icu_locale_core v2.1.1
2026-Mar-21 03:30:09.218424
#44 40.15    Compiling icu_collections v2.1.1
2026-Mar-21 03:30:09.218424
#44 40.20    Compiling bit-set v0.5.3
2026-Mar-21 03:30:09.218424
#44 40.23    Compiling crossbeam-epoch v0.9.18
2026-Mar-21 03:30:09.218424
#44 40.23    Compiling crossbeam-channel v0.5.15
2026-Mar-21 03:30:09.218424
#44 40.27    Compiling cooked-waker v5.0.0
2026-Mar-21 03:30:09.446257
#44 40.27    Compiling deno_ops v0.176.0
2026-Mar-21 03:30:09.446257
#44 40.32    Compiling extractor v0.1.0 (/app/crates/extractor)
2026-Mar-21 03:30:09.446257
#44 40.32    Compiling tagptr v0.2.0
2026-Mar-21 03:30:09.446257
#44 40.33    Compiling pest_derive v2.8.6
2026-Mar-21 03:30:09.446257
#44 40.36    Compiling deno_core_icudata v0.0.73
2026-Mar-21 03:30:09.446257
#44 40.50    Compiling pin-project v1.1.10
2026-Mar-21 03:30:09.550778
#44 40.50    Compiling static_assertions v1.1.0
2026-Mar-21 03:30:09.550778
#44 40.53    Compiling async-stream-impl v0.3.6
2026-Mar-21 03:30:09.550778
#44 40.53    Compiling lru v0.12.5
2026-Mar-21 03:30:09.550778
#44 40.60    Compiling num_cpus v1.17.0
2026-Mar-21 03:30:09.682407
#44 40.64    Compiling compression-codecs v0.4.37
2026-Mar-21 03:30:09.682407
#44 40.73    Compiling tracing-log v0.2.0
2026-Mar-21 03:30:09.835902
#44 40.89    Compiling sharded-slab v0.1.7
2026-Mar-21 03:30:09.968345
#44 40.91    Compiling thread_local v1.1.9
2026-Mar-21 03:30:09.968345
#44 40.94    Compiling nu-ansi-term v0.50.3
2026-Mar-21 03:30:10.156619
#44 41.21    Compiling icu_provider v2.1.1
2026-Mar-21 03:30:10.359769
#44 41.26    Compiling async-stream v0.3.6
2026-Mar-21 03:30:10.437759
#44 41.49    Compiling icu_properties v2.1.2
2026-Mar-21 03:30:11.080741
#44 42.05    Compiling futures-executor v0.3.32
2026-Mar-21 03:30:11.080741
#44 42.13    Compiling smallvec v1.15.1
2026-Mar-21 03:30:11.080741
#44 42.13    Compiling either v1.15.0
2026-Mar-21 03:30:11.213154
#44 42.15    Compiling serde_urlencoded v0.7.1
2026-Mar-21 03:30:11.213154
#44 42.15    Compiling debugid v0.8.0
2026-Mar-21 03:30:11.213154
#44 42.17    Compiling bincode v1.3.3
2026-Mar-21 03:30:11.213154
#44 42.18    Compiling json5 v0.4.1
2026-Mar-21 03:30:11.213154
#44 42.26    Compiling futures v0.3.32
2026-Mar-21 03:30:11.328258
#44 42.38    Compiling itertools v0.12.1
2026-Mar-21 03:30:11.450400
#44 42.38    Compiling which v4.4.2
2026-Mar-21 03:30:11.450400
#44 42.38    Compiling which v6.0.3
2026-Mar-21 03:30:11.450400
#44 42.40    Compiling bytes-utils v0.1.4
2026-Mar-21 03:30:11.450400
#44 42.42    Compiling itertools v0.13.0
2026-Mar-21 03:30:11.450400
#44 42.50    Compiling icu_normalizer v2.1.1
2026-Mar-21 03:30:11.643184
#44 42.54    Compiling rustls-webpki v0.103.9
2026-Mar-21 03:30:11.658275
2026-Mar-21 03:30:11.682632
#44 42.73    Compiling parking_lot v0.12.5
2026-Mar-21 03:30:11.856007
#44 42.73    Compiling dashmap v6.1.0
2026-Mar-21 03:30:11.868676
#44 42.83    Compiling matchers v0.2.0
2026-Mar-21 03:30:11.868676
#44 42.90    Compiling idna_adapter v1.2.1
2026-Mar-21 03:30:11.972135
#44 42.99    Compiling futures-intrusive v0.5.0
2026-Mar-21 03:30:11.972135
#44 43.02    Compiling
2026-Mar-21 03:30:12.174787
moka v0.12.13
2026-Mar-21 03:30:12.174787
#44 43.05    Compiling idna v1.1.0
2026-Mar-21 03:30:12.174787
#44 43.07    Compiling tracing-subscriber v0.3.22
2026-Mar-21 03:30:12.428127
#44 43.48    Compiling url v2.5.8
2026-Mar-21 03:30:12.659882
#44 43.56    Compiling publicsuffix v2.3.0
2026-Mar-21 03:30:12.810231
#44 ...
2026-Mar-21 03:30:12.810231
2026-Mar-21 03:30:12.810231
#43 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-21 03:30:12.810231
#43 33.78    Compiling sqlx-postgres v0.8.6
2026-Mar-21 03:30:12.810231
#43 34.16    Compiling aws-smithy-http v0.63.6
2026-Mar-21 03:30:12.810231
#43 34.16    Compiling aws-credential-types v1.2.14
2026-Mar-21 03:30:12.810231
#43 34.16    Compiling aws-smithy-observability v0.2.6
2026-Mar-21 03:30:12.810231
#43 34.16    Compiling aws-smithy-http v0.62.6
2026-Mar-21 03:30:12.810231
#43 34.80    Compiling aws-sigv4 v1.4.2
2026-Mar-21 03:30:12.810231
#43 35.53    Compiling tokio-rustls v0.24.1
2026-Mar-21 03:30:12.810231
#43 36.47    Compiling hyper v0.14.32
2026-Mar-21 03:30:12.810231
#43 37.36    Compiling redis v0.27.6
2026-Mar-21 03:30:12.810231
#43 37.50    Compiling crc-fast v1.6.0
2026-Mar-21 03:30:12.810231
#43 37.62    Compiling hyper v1.8.1
2026-Mar-21 03:30:12.810231
#43 38.77    Compiling sqlx-macros-core v0.8.6
2026-Mar-21 03:30:12.810231
#43 40.00    Compiling hyper-util v0.1.20
2026-Mar-21 03:30:12.810231
#43 40.47    Compiling sqlx-macros v0.8.6
2026-Mar-21 03:30:12.810231
#43 40.92    Compiling aws-smithy-checksums v0.63.12
2026-Mar-21 03:30:12.810231
#43 40.98    Compiling serde_v8 v0.209.0
2026-Mar-21 03:30:12.810231
#43 41.45    Compiling deno_core v0.300.0
2026-Mar-21 03:30:12.810231
#43 41.96    Compiling hyper-tls v0.6.0
2026-Mar-21 03:30:12.810231
#43 41.96    Compiling axum v0.8.8
2026-Mar-21 03:30:12.810231
#43 42.16    Compiling hyper-rustls v0.24.2
2026-Mar-21 03:30:13.129979
#43 ...
2026-Mar-21 03:30:13.129979
2026-Mar-21 03:30:13.129979
#44 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-21 03:30:13.129979
#44 44.18    Compiling cookie_store v0.22.1
2026-Mar-21 03:30:13.280464
#44 44.18    Compiling sourcemap v8.0.1
2026-Mar-21 03:30:13.948139
#44 45.00    Compiling sct v0.7.1
2026-Mar-21 03:30:14.161827
#44 45.06    Compiling rustls-webpki v0.101.7
2026-Mar-21 03:30:14.612647
#44 ...
2026-Mar-21 03:30:14.612647
2026-Mar-21 03:30:14.612647
#48 [frontend builder 8/8] RUN node build-docker.mjs
2026-Mar-21 03:30:14.612647
#48 11.38 "optionsMiddleware" is imported from external module "@better-auth/core/api" but never used in "node_modules/better-auth/dist/api/index.mjs" and "node_modules/better-auth/dist/plugins/index.mjs".
2026-Mar-21 03:30:14.612647
#48 11.38 "getTelemetryAuthConfig" is imported from external module "@better-auth/telemetry" but never used in "node_modules/better-auth/dist/index.mjs".
2026-Mar-21 03:30:14.612647
#48 11.38 ✓ 1087 modules transformed.
2026-Mar-21 03:30:14.612647
#48 11.87 rendering chunks...
2026-Mar-21 03:30:14.612647
#48 13.67 vite v6.4.1 building for production...
2026-Mar-21 03:30:14.765530
#48 ...
2026-Mar-21 03:30:14.765530
2026-Mar-21 03:30:14.765530
#43 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-21 03:30:14.765530
#43 45.77    Compiling queue v0.1.0 (/app/crates/queue)
2026-Mar-21 03:30:20.008306
#43 ...
2026-Mar-21 03:30:20.008306
2026-Mar-21 03:30:20.008306
#44 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-21 03:30:20.008306
#44 46.40    Compiling sqlx-core v0.8.6
2026-Mar-21 03:30:20.008306
#44 47.11    Compiling v8 v0.101.0
2026-Mar-21 03:30:20.008306
#44 47.28    Compiling tokio-util v0.7.18
2026-Mar-21 03:30:20.008306
#44 47.28    Compiling aws-smithy-async v1.2.14
2026-Mar-21 03:30:20.008306
#44 47.28    Compiling tower v0.5.3
2026-Mar-21 03:30:20.008306
#44 47.28    Compiling async-compression v0.4.41
2026-Mar-21 03:30:20.008306
#44 47.28    Compiling tokio-native-tls v0.3.1
2026-Mar-21 03:30:20.008306
#44 47.29    Compiling deno_unsync v0.4.4
2026-Mar-21 03:30:20.008306
#44 47.34    Compiling tokio-rustls v0.24.1
2026-Mar-21 03:30:20.008306
#44 47.65    Compiling aws-smithy-types v1.4.6
2026-Mar-21 03:30:20.008306
#44 47.65    Compiling h2 v0.4.13
2026-Mar-21 03:30:20.008306
#44 47.65    Compiling h2 v0.3.27
2026-Mar-21 03:30:20.008306
#44 47.65    Compiling combine v4.6.7
2026-Mar-21 03:30:20.008306
#44 47.65    Compiling tower-http v0.6.8
2026-Mar-21 03:30:20.008306
#44 47.98    Compiling sqlx-postgres v0.8.6
2026-Mar-21 03:30:20.008306
#44 48.24    Compiling aws-smithy-runtime-api v1.11.6
2026-Mar-21 03:30:20.008306
#44 48.24    Compiling aws-smithy-eventstream v0.60.20
2026-Mar-21 03:30:20.008306
#44 48.24    Compiling aws-smithy-json v0.62.5
2026-Mar-21 03:30:20.008306
#44 48.24    Compiling aws-smithy-query v0.60.15
2026-Mar-21 03:30:20.008306
#44 48.24    Compiling aws-smithy-json v0.61.9
2026-Mar-21 03:30:20.008306
#44 49.12    Compiling aws-smithy-http v0.63.6
2026-Mar-21 03:30:20.008306
#44 49.12    Compiling aws-credential-types v1.2.14
2026-Mar-21 03:30:20.008306
#44 49.12    Compiling aws-smithy-observability v0.2.6
2026-Mar-21 03:30:20.008306
#44 49.12    Compiling aws-smithy-http v0.62.6
2026-Mar-21 03:30:20.008306
#44 49.36    Compiling aws-sigv4 v1.4.2
2026-Mar-21 03:30:20.008306
#44 50.48    Compiling sqlx-macros-core v0.8.6
2026-Mar-21 03:30:20.008306
#44 50.52    Compiling hyper v0.14.32
2026-Mar-21 03:30:20.008306
#44 50.61    Compiling hyper v1.8.1
2026-Mar-21 03:30:20.008306
#44 50.95    Compiling redis v0.27.6
2026-Mar-21 03:30:20.008306
#44 51.06    Compiling serde_v8 v0.209.0
2026-Mar-21 03:30:20.248981
#44 51.30    Compiling sqlx-macros v0.8.6
2026-Mar-21 03:30:20.460025
#44 51.36    Compiling deno_core v0.300.0
2026-Mar-21 03:30:20.614074
#44 51.66    Compiling crc-fast v1.6.0
2026-Mar-21 03:30:20.787082
#44 51.69    Compiling hyper-util v0.1.20
2026-Mar-21 03:30:21.744131
#44 52.79    Compiling hyper-tls v0.6.0
2026-Mar-21 03:30:21.744131
#44 52.79    Compiling axum v0.8.8
2026-Mar-21 03:30:22.299046
#44 53.35    Compiling hyper-rustls v0.24.2
2026-Mar-21 03:30:22.414160
#44 53.46    Compiling aws-smithy-checksums v0.63.12
2026-Mar-21 03:30:23.645809
#44 ...
2026-Mar-21 03:30:23.645809
2026-Mar-21 03:30:23.645809
#48 [frontend builder 8/8] RUN node build-docker.mjs
2026-Mar-21 03:30:23.645809
#48 24.14 ✔ [paraglide-js] Compilation complete (message-modules)
2026-Mar-21 03:30:23.645809
#48 24.16 transforming...
2026-Mar-21 03:30:23.645809
#48 24.92 3:30:18 AM [vite-plugin-svelte] src/routes/terms/+page.svelte:176:1 Unused CSS selector ".legal-page li strong"
2026-Mar-21 03:30:23.645809
#48 24.92 https://svelte.dev/e/css_unused_selector
2026-Mar-21 03:30:23.645809
#48 24.92 174:   }
2026-Mar-21 03:30:23.645809
#48 24.92 175:
2026-Mar-21 03:30:23.645809
#48 24.92 176:   .legal-page li strong {
2026-Mar-21 03:30:23.645809
#48 24.92                             ^
2026-Mar-21 03:30:23.645809
#48 24.92 177:     color: #2d1b36;
2026-Mar-21 03:30:23.645809
#48 24.92 178:   }
2026-Mar-21 03:30:23.645809
#48 24.92 3:30:18 AM [vite-plugin-svelte] src/routes/terms/+page.svelte:236:1 Unused CSS selector ":global(.app.theme-dark) .legal-page li strong"
2026-Mar-21 03:30:23.645809
#48 24.92 https://svelte.dev/e/css_unused_selector
2026-Mar-21 03:30:23.645809
#48 24.92 234:   }
2026-Mar-21 03:30:23.645809
#48 24.92 235:
2026-Mar-21 03:30:23.645809
#48 24.92 236:   :global(.app.theme-dark) .legal-page li strong {
2026-Mar-21 03:30:23.645809
#48 24.92                                                      ^
2026-Mar-21 03:30:23.645809
#48 24.92 237:     color: #ffffff;
2026-Mar-21 03:30:23.645809
#48 24.92 238:   }
2026-Mar-21 03:30:23.645809
#48 29.25 ✓ 958 modules transformed.
2026-Mar-21 03:30:23.645809
#48 29.51 rendering chunks...
2026-Mar-21 03:30:23.645809
#48 29.69 computing gzip size...
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/version.json                                                0.03 kB │ gzip:   0.05 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/assets/fredoka-latin-ext.CYrqKuxd.woff2           4.58 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/assets/nunito-normal-vietnamese.U01xdrZh.woff2   13.10 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/.vite/manifest.json                                             24.10 kB │ gzip:   2.54 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/assets/fredoka-latin.DM6njrJ3.woff2              29.73 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/assets/nunito-normal-latin.BzFMHfZw.woff2        39.13 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/assets/2.BCHMhk4V.css                             0.22 kB │ gzip:   0.17 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/assets/AppIcon.BlEdAg33.css                       0.47 kB │ gzip:   0.25 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/assets/4.uLVlcr3C.css                             0.76 kB │ gzip:   0.28 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/assets/16.B6JFksZy.css                            0.79 kB │ gzip:   0.43 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/assets/landing-page-config.BsO8r1pi.css           1.46 kB │ gzip:   0.51 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/assets/14.yQ4fzJo8.css                            2.45 kB │ gzip:   0.70 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/assets/26.D8eq2kjt.css                            2.45 kB │ gzip:   0.70 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/assets/12.DzC93YmD.css                            2.61 kB │ gzip:   0.71 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/assets/15.Cbw5ficc.css                            2.66 kB │ gzip:   0.71 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/assets/25.CiKCOCFN.css                            2.91 kB │ gzip:   0.75 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/assets/27.Bt4XCPKf.css                            3.63 kB │ gzip:   1.01 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/assets/22.2HioDSYe.css                            3.69 kB │ gzip:   1.05 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/assets/28.BlPYOMgk.css                            3.82 kB │ gzip:   1.03 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/assets/23.Ck83DATa.css                            4.50 kB │ gzip:   1.20 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/assets/13.BizOxsbv.css                            4.72 kB │ gzip:   1.24 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/assets/24.D-0Ycg0O.css                            5.58 kB │ gzip:   1.40 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/assets/3.B2BMSf4l.css                             8.09 kB │ gzip:   1.94 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/assets/FormatPicker.LWtiaWnF.css                 11.22 kB │ gzip:   2.70 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/assets/0.CZFhEyCk.css                            57.50 kB │ gzip:  10.69 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/entry/start.Cdr0wUEO.js                           0.08 kB │ gzip:   0.09 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/BE3tUz8f.js                                0.19 kB │ gzip:   0.17 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/D-Me5IY9.js                                0.23 kB │ gzip:   0.16 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/C2B4KuK7.js                                0.38 kB │ gzip:   0.28 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/DYGa1rTG.js                                0.38 kB │ gzip:   0.27 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/DMBFGhcc.js                                0.54 kB │ gzip:   0.34 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/1.hPP73cTE.js                               0.59 kB │ gzip:   0.36 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/Bl4ip-Pz.js                                0.67 kB │ gzip:   0.33 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/Bbu0LoaP.js                                0.82 kB │ gzip:   0.46 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/B-rWOBWI.js                                1.13 kB │ gzip:   0.65 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/D2gFui-Z.js                                1.37 kB │ gzip:   0.79 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/B3_3ZEui.js                                1.40 kB │ gzip:   0.65 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/DFejnfps.js                                1.56 kB │ gzip:   0.82 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/17.fy_ihNQ_.js                              1.72 kB │ gzip:   0.84 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/19.Blpmq7Vg.js                              1.73 kB │ gzip:   0.84 kB
2026-Mar-21 03:30:23.645809
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/18.Bd8GOdmF.js                              1.73 kB │ gzip:   0.84 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/21.BZ8KQmU1.js                              1.73 kB │ gzip:   0.84 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/20.2sf_5wKq.js                              1.73 kB │ gzip:   0.84 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/jC79MO7m.js                                2.05 kB │ gzip:   1.06 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/Y1-W1rEu.js                                2.14 kB │ gzip:   1.02 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/D5hpYBKX.js                                2.59 kB │ gzip:   1.20 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/CrozGAN7.js                                2.81 kB │ gzip:   0.64 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/14.CVOEmWD3.js                              3.03 kB │ gzip:   1.24 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/C6Vz1w93.js                                3.15 kB │ gzip:   1.48 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/12.DlQ34JUd.js                              3.40 kB │ gzip:   1.43 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/BDWVmpqy.js                                3.47 kB │ gzip:   1.57 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/r6NPxIR8.js                                4.15 kB │ gzip:   1.94 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/26.lAjSR6Xq.js                              4.49 kB │ gzip:   1.72 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/15.B_464GVz.js                              4.77 kB │ gzip:   1.69 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/DRxOZeHk.js                                5.23 kB │ gzip:   2.45 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/7.CDy5I4BW.js                               6.02 kB │ gzip:   1.81 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/10.bgSN6Ndr.js                              6.72 kB │ gzip:   2.55 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/2.DZEpSPKB.js                               7.50 kB │ gzip:   2.93 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/9.ByLN7kT0.js                               7.55 kB │ gzip:   1.80 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/22.Dw2GwmiX.js                              8.12 kB │ gzip:   2.98 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/8.VI-vInmf.js                               8.65 kB │ gzip:   2.99 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/23.BVVl23Y1.js                              8.76 kB │ gzip:   3.20 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/28.BQ71_SRL.js                              8.91 kB │ gzip:   3.18 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/6.DfCB4PYy.js                               9.12 kB │ gzip:   3.08 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/LWVHwvlG.js                                9.33 kB │ gzip:   3.97 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/27.IGGoxMRl.js                              9.77 kB │ gzip:   3.64 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/13.DKHn5Mcs.js                              9.92 kB │ gzip:   3.57 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/4.E_IcbDTM.js                              10.72 kB │ gzip:   4.08 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/24.BVWmkD7Z.js                             10.74 kB │ gzip:   3.78 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/entry/app.Dfl-X7jC.js                            11.26 kB │ gzip:   3.84 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/11.Dl6b5vJr.js                             21.06 kB │ gzip:   5.99 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/DmOCDnli.js                               25.18 kB │ gzip:   8.21 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/x-n8IVhB.js                               25.20 kB │ gzip:   9.89 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/Dy-M8iVC.js                               25.30 kB │ gzip:   9.25 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/CDEzdxkZ.js                               32.88 kB │ gzip:  12.68 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/BG07Hhix.js                               33.05 kB │ gzip:  12.06 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/0.ExrbPe_3.js                              40.92 kB │ gzip:   9.48 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/DnfRFVYx.js                               66.87 kB │ gzip:  18.32 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/16.BoXgSp58.js                             87.57 kB │ gzip:  11.72 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/D8Tq8K4-.js                               91.21 kB │ gzip:  18.95 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/BywqCEIN.js                              115.86 kB │ gzip:  26.21 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/chunks/C5mRpyjW.js                              179.17 kB │ gzip:   8.95 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/25.DX4Y-ZPd.js                            267.11 kB │ gzip: 104.85 kB
2026-Mar-21 03:30:23.661507
#48 29.71 .svelte-kit/output/client/_app/immutable/nodes/3.DBiKTx5p.js                             273.76 kB │ gzip:  85.85 kB
2026-Mar-21 03:30:23.661507
#48 29.71 ✓ built in 16.04s
2026-Mar-21 03:30:23.661507
#48 30.34 vite v6.4.1 building for production...
2026-Mar-21 03:30:23.661507
#48 30.34 transforming...
2026-Mar-21 03:30:23.661507
#48 30.35 ✓ 2 modules transformed.
2026-Mar-21 03:30:23.661507
#48 30.35 rendering chunks...
2026-Mar-21 03:30:23.661507
#48 30.35 computing gzip size...
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/client/service-worker.mjs  5.05 kB │ gzip: 1.64 kB
2026-Mar-21 03:30:23.661507
#48 30.35 ✓ built in 11ms
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/_app/immutable/assets/fredoka-latin-ext.CYrqKuxd.woff2                         4.58 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/_app/immutable/assets/nunito-normal-vietnamese.U01xdrZh.woff2                 13.10 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/_app/immutable/assets/fredoka-latin.DM6njrJ3.woff2                            29.73 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/.vite/manifest.json                                                           33.41 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/_app/immutable/assets/nunito-normal-latin.BzFMHfZw.woff2                      39.13 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/_app/immutable/assets/_layout.BCHMhk4V.css                                     0.22 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/_app/immutable/assets/AppIcon.BlEdAg33.css                                     0.47 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/_app/immutable/assets/_page.uLVlcr3C.css                                       0.76 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/_app/immutable/assets/_page.B6JFksZy.css                                       0.79 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/_app/immutable/assets/landing-page-config.CDKY1U6M.css                         1.42 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/_app/immutable/assets/_page.yQ4fzJo8.css                                       2.45 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/_app/immutable/assets/_page.D8eq2kjt.css                                       2.45 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/_app/immutable/assets/_page.DzC93YmD.css                                       2.61 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/_app/immutable/assets/_page.Cbw5ficc.css                                       2.66 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/_app/immutable/assets/_page.CiKCOCFN.css                                       2.91 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/_app/immutable/assets/_page.Bt4XCPKf.css                                       3.63 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/_app/immutable/assets/_page.2HioDSYe.css                                       3.69 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/_app/immutable/assets/_page.BlPYOMgk.css                                       3.82 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/_app/immutable/assets/FormatPicker.PGaOKC5t.css                                3.91 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/_app/immutable/assets/_page.Ck83DATa.css                                       4.50 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/_app/immutable/assets/_page.BizOxsbv.css                                       4.72 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/_app/immutable/assets/_page.D-0Ycg0O.css                                       5.58 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/_app/immutable/assets/_page.B2BMSf4l.css                                       8.09 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/_app/immutable/assets/_layout.Bf4jlJWi.css                                    56.71 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/_page.ts.js                                                      0.05 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/about/_page.ts.js                                                0.05 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/best-format-for-youtube-downloads-mp4-vs-webm/_page.ts.js        0.05 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/contact/_page.ts.js                                              0.05 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/dmca/_page.ts.js                                                 0.05 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/download-youtube-4k/_page.ts.js                                  0.05 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/download-youtube-8k-hdr/_page.ts.js                              0.05 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/download-youtube-mp3/_page.ts.js                                 0.05 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/download-youtube-playlist/_page.ts.js                            0.05 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/download-youtube-shorts/_page.ts.js                              0.05 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/download-youtube-shorts-with-audio/_page.ts.js                   0.05 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/how-to-download-youtube-playlists/_page.ts.js                    0.05 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/how-to-use-snapvie/_page.ts.js                                   0.05 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/terms/_page.ts.js                                                0.05 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/why-youtube-downloads-need-muxing/_page.ts.js                    0.05 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/why-youtube-downloads-show-360p-only/_page.ts.js                 0.05 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/false.js                                                                0.05 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/endpoints/api/checkout/_server.ts.js                                   0.12 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/endpoints/api/checkout/callback/_server.ts.js                          0.12 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/(auth)/admin/_page.server.ts.js                                  0.13 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/hooks.universal.js                                                     0.15 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/(auth)/admin/jobs/_page.server.ts.js                             0.18 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/(auth)/admin/activity/_page.server.ts.js                         0.19 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/(auth)/admin/playlists/_page.server.ts.js                        0.20 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/endpoints/api/auth/_...all_/_server.ts.js                              0.23 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/environment.js                                                          0.25 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/endpoints/share-target/_server.ts.js                                   0.25 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/shared-server.js                                                        0.28 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/(auth)/admin/overview/_page.server.ts.js                         0.36 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/endpoints/robots.txt/_server.ts.js                                     0.38 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/(auth)/admin/_layout.server.ts.js                                0.39 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/(auth)/account/_page.server.ts.js                                0.39 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/state.svelte.js                                                         0.42 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/server.js                                                               0.45 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/internal.js                                                                    0.46 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/fallbacks/error.svelte.js                                              0.66 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/client.js                                                               0.68 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/stores.js                                                               0.73 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/endpoints/api/proxy/playlist-jobs/_jobId_/_server.ts.js                0.81 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/endpoints/api/proxy/playlist-jobs/_jobId_/events/_server.ts.js         0.86 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/admin-access.js                                                         1.00 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/utils.js                                                                1.15 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/endpoints/api/proxy/playlist-jobs/_jobId_/cancel/_server.ts.js         1.18 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/endpoints/api/proxy/playlist-jobs/_jobId_/start/_server.ts.js          1.43 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/endpoints/api/proxy/jobs/_jobId_/release/_server.ts.js                 1.44 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/endpoints/api/proxy/jobs/_jobId_/events/_server.ts.js                  1.45 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/public-pages.js                                                         1.45 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/endpoints/api/proxy/jobs/_jobId_/_server.ts.js                         1.62 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/endpoints/api/proxy/playlist-jobs/_server.ts.js                        1.76 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/endpoints/sitemap.xml/_server.ts.js                                    1.82 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/download-youtube-4k/_page.svelte.js                              1.98 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/download-youtube-mp3/_page.svelte.js                             1.98 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/AdminStatusBadge.js                                                     1.98 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/download-youtube-8k-hdr/_page.svelte.js                          1.98 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/download-youtube-shorts/_page.svelte.js                          1.98 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/download-youtube-playlist/_page.svelte.js                        1.98 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/endpoints/api/proxy/jobs/_server.ts.js                                 2.00 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/exports.js                                                              2.39 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/index.js                                                                2.65 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/endpoints/api/proxy/jobs/_jobId_/file-ticket/_server.ts.js             2.66 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/FormatPicker.svelte_svelte_type_style_lang.js                           2.68 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/contact/_page.svelte.js                                          2.80 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/structured-data.js                                                      2.85 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/auth-utils.js                                                           2.97 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/about/_page.svelte.js                                            3.14 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/url.js                                                                  3.16 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/url2.js                                                                 3.32 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/AdminRecordDetailsModal.js                                              3.33 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/rust-api-proxy.js                                                       3.68 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/internal.js                                                             4.15 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/audit-log.js                                                            4.21 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/terms/_page.svelte.js                                            4.25 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/bun-sqlite-dialect.js                                                   4.29 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/node-sqlite-dialect.js                                                  4.30 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/dmca/_page.svelte.js                                             4.54 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/endpoints/api/proxy/extract/_server.ts.js                              4.88 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/(auth)/admin/proxies/_page.server.ts.js                          6.31 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/(auth)/admin/capacity/_page.svelte.js                            6.50 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/(auth)/admin/playlists/_page.svelte.js                           6.71 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/(auth)/admin/overview/_page.svelte.js                            7.11 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/hooks.server.js                                                        7.23 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/download-youtube-shorts-with-audio/_page.svelte.js               8.06 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/_layout.svelte.js                                                8.69 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/why-youtube-downloads-show-360p-only/_page.svelte.js             8.84 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/how-to-download-youtube-playlists/_page.svelte.js                9.06 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/query.js                                                                9.11 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/(auth)/admin/jobs/_page.svelte.js                                9.50 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/index3.js                                                               9.56 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/(auth)/admin/_layout.svelte.js                                   9.70 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/why-youtube-downloads-need-muxing/_page.svelte.js                9.74 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/best-format-for-youtube-downloads-mp4-vs-webm/_page.svelte.js    9.88 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/(auth)/admin/activity/_page.svelte.js                           10.22 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/remote-entry.js                                                               10.47 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/how-to-use-snapvie/_page.svelte.js                              11.11 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/runtime.js                                                             13.79 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/index4.js                                                              14.75 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/admin-dashboard.js                                                     19.43 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/(auth)/admin/proxies/_page.svelte.js                            20.00 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/landing-page-config.js                                                 24.48 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/shared.js                                                              26.42 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/root.js                                                                29.24 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/(auth)/account/_page.svelte.js                                  38.96 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/download/mux-job/_page.svelte.js                                41.23 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/playlist-download-stream-selection.js                                  49.82 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/index2.js                                                              91.55 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/SiteHeader.js                                                         109.42 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/index.js                                                                     123.18 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/AuthModal.js                                                          143.21 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/AppIcon.js                                                            272.68 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/chunks/auth.js                                                               311.75 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/_page.svelte.js                                                771.24 kB
2026-Mar-21 03:30:23.661507
#48 30.35 .svelte-kit/output/server/entries/pages/privacy/_page.svelte.js                                        957.10 kB
2026-Mar-21 03:30:23.661507
#48 30.36 ✓ built in 28.60s
2026-Mar-21 03:30:23.661507
#48 30.36
2026-Mar-21 03:30:23.661507
#48 30.36 Run npm run preview to preview your production build locally.
2026-Mar-21 03:30:23.661507
#48 30.36
2026-Mar-21 03:30:23.661507
#48 30.36 > Using @sveltejs/adapter-node
2026-Mar-21 03:30:24.043083
#48 ...
2026-Mar-21 03:30:24.043083
2026-Mar-21 03:30:24.043083
#43 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-21 03:30:24.043083
#43 55.05    Compiling tokio-rustls v0.26.4
2026-Mar-21 03:30:24.206180
#43 55.22    Compiling hyper-rustls v0.27.7
2026-Mar-21 03:30:24.349620
#43 55.36    Compiling aws-smithy-http-client v1.1.12
2026-Mar-21 03:30:24.349620
#43 55.36    Compiling reqwest v0.12.28
2026-Mar-21 03:30:25.679790
#43 56.69    Compiling aws-smithy-runtime v1.10.3
2026-Mar-21 03:30:27.724443
#43 58.73    Compiling aws-runtime v1.7.2
2026-Mar-21 03:30:27.990160
#43 ...
2026-Mar-21 03:30:27.990160
2026-Mar-21 03:30:27.990160
#48 [frontend builder 8/8] RUN node build-docker.mjs
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/zod/v4/classic/schemas.js -> node_modules/zod/v4/classic/iso.js -> node_modules/zod/v4/classic/schemas.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/binary-operation-parser.js -> node_modules/kysely/dist/esm/parser/reference-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/parser/reference-parser.js -> node_modules/kysely/dist/esm/parser/order-by-parser.js -> node_modules/kysely/dist/esm/dynamic/dynamic-reference-builder.js -> node_modules/kysely/dist/esm/parser/reference-parser.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/binary-operation-parser.js -> node_modules/kysely/dist/esm/parser/reference-parser.js -> node_modules/kysely/dist/esm/parser/order-by-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/parser/reference-parser.js -> node_modules/kysely/dist/esm/parser/order-by-parser.js -> node_modules/kysely/dist/esm/parser/reference-parser.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/binary-operation-parser.js -> node_modules/kysely/dist/esm/parser/value-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/operation-node/select-query-node.js -> node_modules/kysely/dist/esm/operation-node/query-node.js -> node_modules/kysely/dist/esm/operation-node/select-query-node.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/operation-node/query-node.js -> node_modules/kysely/dist/esm/operation-node/delete-query-node.js -> node_modules/kysely/dist/esm/operation-node/query-node.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/insert-query-builder.js -> node_modules/kysely/dist/esm/parser/select-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/parser/table-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/insert-query-builder.js -> node_modules/kysely/dist/esm/parser/select-parser.js -> node_modules/kysely/dist/esm/parser/table-parser.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/insert-query-builder.js -> node_modules/kysely/dist/esm/parser/select-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/insert-query-builder.js -> node_modules/kysely/dist/esm/parser/insert-values-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/insert-query-builder.js -> node_modules/kysely/dist/esm/parser/insert-values-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/insert-query-builder.js -> node_modules/kysely/dist/esm/parser/update-set-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/insert-query-builder.js -> node_modules/kysely/dist/esm/parser/expression-parser.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/delete-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/parser/table-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/delete-query-builder.js -> node_modules/kysely/dist/esm/parser/table-parser.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/update-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/parser/table-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/update-query-builder.js -> node_modules/kysely/dist/esm/parser/table-parser.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/parser/table-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/parser/table-parser.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/parser/with-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/merge-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/parser/table-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/parser/table-parser.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/parser/table-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/table-parser.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/parser/table-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/table-parser.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/group-by-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/set-operation-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/set-operation-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/parser/table-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/parser/table-parser.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/parser/table-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/function-module.js -> node_modules/kysely/dist/esm/parser/table-parser.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/kysely/dist/esm/parser/table-parser.js -> node_modules/kysely/dist/esm/dynamic/dynamic-table-builder.js -> node_modules/kysely/dist/esm/parser/table-parser.js
2026-Mar-21 03:30:27.990160
#48 32.70 Circular dependency: node_modules/@better-auth/core/dist/oauth2/validate-authorization-code.mjs -> node_modules/@better-auth/core/dist/oauth2/index.mjs -> node_modules/@better-auth/core/dist/oauth2/validate-authorization-code.mjs
2026-Mar-21 03:30:27.990160
#48 34.43   ✔ done
2026-Mar-21 03:30:27.990160
#48 DONE 34.9s
2026-Mar-21 03:30:28.140028
#44 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-21 03:30:28.140028
#44 55.70    Compiling queue v0.1.0 (/app/crates/queue)
2026-Mar-21 03:30:28.671706
#44 ...
2026-Mar-21 03:30:28.671706
2026-Mar-21 03:30:28.671706
#49 [frontend runtime 3/6] COPY --from=builder /app/build ./build
2026-Mar-21 03:30:28.671706
#49 DONE 0.4s
2026-Mar-21 03:30:28.866678
#50 [frontend runtime 4/6] COPY --from=builder /app/package.json ./
2026-Mar-21 03:30:28.866678
#50 DONE 0.0s
2026-Mar-21 03:30:28.866678
2026-Mar-21 03:30:28.866678
#51 [frontend runtime 5/6] COPY --from=builder /app/package-lock.json ./
2026-Mar-21 03:30:28.866678
#51 DONE 0.0s
2026-Mar-21 03:30:28.866678
2026-Mar-21 03:30:28.866678
#43 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-21 03:30:28.866678
#43 59.45    Compiling sqlx v0.8.6
2026-Mar-21 03:30:28.866678
#43 59.54    Compiling proxy v0.1.0 (/app/crates/proxy)
2026-Mar-21 03:30:28.866678
#43 59.54    Compiling job-system v0.1.0 (/app/crates/job-system)
2026-Mar-21 03:30:28.866678
#43 59.69    Compiling aws-sdk-ssooidc v1.98.0
2026-Mar-21 03:30:28.866678
#43 59.69    Compiling aws-sdk-sso v1.96.0
2026-Mar-21 03:30:28.866678
#43 59.69    Compiling aws-sdk-sts v1.100.0
2026-Mar-21 03:30:28.866678
#43 59.69    Compiling aws-sdk-s3 v1.119.0
2026-Mar-21 03:30:30.480498
#43 61.49    Compiling aws-config v1.8.15
2026-Mar-21 03:30:30.935885
#43 61.95    Compiling muxer v0.1.0 (/app/crates/muxer)
2026-Mar-21 03:30:33.148127
#43 ...
2026-Mar-21 03:30:33.148127
2026-Mar-21 03:30:33.148127
#52 [frontend runtime 6/6] RUN npm ci --omit=dev
2026-Mar-21 03:30:33.148127
#52 4.074
2026-Mar-21 03:30:33.148127
#52 4.074 > frontend@0.0.1 prepare
2026-Mar-21 03:30:33.148127
#52 4.074 > svelte-kit sync || echo ''
2026-Mar-21 03:30:33.148127
#52 4.074
2026-Mar-21 03:30:33.148127
#52 4.124 Missing Svelte config file in /app — skipping
2026-Mar-21 03:30:33.148127
#52 4.131
2026-Mar-21 03:30:33.148127
#52 4.131 added 77 packages, and audited 78 packages in 4s
2026-Mar-21 03:30:33.148127
#52 4.131
2026-Mar-21 03:30:33.148127
#52 4.131 10 packages are looking for funding
2026-Mar-21 03:30:33.148127
#52 4.131   run `npm fund` for details
2026-Mar-21 03:30:33.148127
#52 4.146
2026-Mar-21 03:30:33.148127
#52 4.146 4 vulnerabilities (2 low, 2 moderate)
2026-Mar-21 03:30:33.148127
#52 4.146
2026-Mar-21 03:30:33.148127
#52 4.146 To address all issues, run:
2026-Mar-21 03:30:33.148127
#52 4.146   npm audit fix
2026-Mar-21 03:30:33.148127
#52 4.146
2026-Mar-21 03:30:33.148127
#52 4.146 Run `npm audit` for details.
2026-Mar-21 03:30:33.148127
#52 4.147 npm notice
2026-Mar-21 03:30:33.148127
#52 4.147 npm notice New major version of npm available! 10.9.4 -> 11.12.0
2026-Mar-21 03:30:33.148127
#52 4.147 npm notice Changelog: https://github.com/npm/cli/releases/tag/v11.12.0
2026-Mar-21 03:30:33.148127
#52 4.147 npm notice To update run: npm install -g npm@11.12.0
2026-Mar-21 03:30:33.148127
#52 4.147 npm notice
2026-Mar-21 03:30:33.148127
#52 DONE 4.4s
2026-Mar-21 03:30:33.333849
#53 [frontend] exporting to image
2026-Mar-21 03:30:33.333849
#53 exporting layers
2026-Mar-21 03:30:35.597177
#53 exporting layers 2.4s done
2026-Mar-21 03:30:35.779627
#53 exporting manifest sha256:d7b8857ca52d2fa3900dcb811cca97b1473ef1ad17b2a50e997f78720f7847be done
2026-Mar-21 03:30:35.779627
#53 exporting config sha256:e6b2c6d9e1ee09f550b90b3e9943ae796043adf6a106c430e8448db002560e50 done
2026-Mar-21 03:30:35.779627
#53 exporting attestation manifest sha256:016e0133806d4b44e72dab807ec956945c9e35ecd1dfa30b6a85da0aa3390285 0.0s done
2026-Mar-21 03:30:35.779627
#53 exporting manifest list sha256:2d0d089c77ba95dbea15da9e6aaadd04fcab1dda1d5971352703c6b8512f22fb done
2026-Mar-21 03:30:35.779627
#53 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:b6153d9e4682c1df48f2e096da582cb16d55164d done
2026-Mar-21 03:30:35.779627
#53 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:b6153d9e4682c1df48f2e096da582cb16d55164d
2026-Mar-21 03:30:37.736543
#53 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:b6153d9e4682c1df48f2e096da582cb16d55164d 2.1s done
2026-Mar-21 03:30:37.852285
#53 DONE 4.6s
2026-Mar-21 03:30:37.852285
2026-Mar-21 03:30:37.852285
#54 [frontend] resolving provenance for metadata file
2026-Mar-21 03:30:38.010111
#54 DONE 0.0s
2026-Mar-21 03:30:38.010111
2026-Mar-21 03:30:38.010111
#44 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-21 03:30:38.010111
#44 63.89    Compiling tokio-rustls v0.26.4
2026-Mar-21 03:30:38.010111
#44 64.07    Compiling hyper-rustls v0.27.7
2026-Mar-21 03:30:38.010111
#44 64.23    Compiling aws-smithy-http-client v1.1.12
2026-Mar-21 03:30:38.010111
#44 64.23    Compiling reqwest v0.12.28
2026-Mar-21 03:30:38.010111
#44 65.56    Compiling aws-smithy-runtime v1.10.3
2026-Mar-21 03:30:38.010111
#44 67.57    Compiling aws-runtime v1.7.2
2026-Mar-21 03:30:38.010111
#44 68.28    Compiling sqlx v0.8.6
2026-Mar-21 03:30:38.010111
#44 68.39    Compiling proxy v0.1.0 (/app/crates/proxy)
2026-Mar-21 03:30:38.010111
#44 68.39    Compiling job-system v0.1.0 (/app/crates/job-system)
2026-Mar-21 03:30:38.010111
#44 68.54    Compiling aws-sdk-sso v1.96.0
2026-Mar-21 03:30:38.010111
#44 68.54    Compiling aws-sdk-ssooidc v1.98.0
2026-Mar-21 03:30:38.010111
#44 68.54    Compiling aws-sdk-sts v1.100.0
2026-Mar-21 03:30:38.010111
#44 68.54    Compiling aws-sdk-s3 v1.119.0
2026-Mar-21 03:30:39.243672
#44 70.29    Compiling aws-config v1.8.15
2026-Mar-21 03:30:39.621475
#44 70.67    Compiling muxer v0.1.0 (/app/crates/muxer)
2026-Mar-21 03:30:46.152360
#44 ...
2026-Mar-21 03:30:46.152360
2026-Mar-21 03:30:46.152360
#43 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-21 03:30:46.152360
#43 77.16    Compiling object-store v0.1.0 (/app/crates/object-store)
2026-Mar-21 03:30:54.593799
#43 ...
2026-Mar-21 03:30:54.593799
2026-Mar-21 03:30:54.593799
#44 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-21 03:30:54.593799
#44 85.64    Compiling object-store v0.1.0 (/app/crates/object-store)
2026-Mar-21 03:31:22.149357
#44 ...
2026-Mar-21 03:31:22.149357
2026-Mar-21 03:31:22.149357
#43 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-21 03:31:22.149357
#43 113.2    Compiling api v0.1.0 (/app/crates/api)
2026-Mar-21 03:31:31.382632
#43 ...
2026-Mar-21 03:31:31.382632
2026-Mar-21 03:31:31.382632
#44 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-21 03:31:31.382632
#44 122.4    Compiling worker v0.1.0 (/app/crates/worker)
2026-Mar-21 03:33:14.459505
#44 225.5     Finished `release` profile [optimized] target(s) in 3m 45s
2026-Mar-21 03:33:14.675624
#44 DONE 225.7s
2026-Mar-21 03:33:14.675624
2026-Mar-21 03:33:14.675624
#43 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-21 03:33:14.792969
#43 ...
2026-Mar-21 03:33:14.792969
2026-Mar-21 03:33:14.792969
#55 [api runtime 2/8] WORKDIR /app
2026-Mar-21 03:33:14.792969
#55 CACHED
2026-Mar-21 03:33:14.792969
2026-Mar-21 03:33:14.792969
#56 [api runtime 3/8] RUN apt-get update && apt-get install -y     ca-certificates     curl     libssl3     && rm -rf /var/lib/apt/lists/*
2026-Mar-21 03:33:14.792969
#56 CACHED
2026-Mar-21 03:33:14.792969
2026-Mar-21 03:33:14.792969
#57 [api runtime 4/8] RUN set -eux;     arch="$(dpkg --print-architecture)";     case "$arch" in       amd64) ytdlp_asset="yt-dlp_linux" ;;       arm64) ytdlp_asset="yt-dlp_linux_aarch64" ;;       *) echo "Unsupported architecture: $arch" >&2; exit 1 ;;     esac;     curl -fL "https://github.com/yt-dlp/yt-dlp/releases/latest/download/${ytdlp_asset}" -o /usr/local/bin/yt-dlp;     chmod +x /usr/local/bin/yt-dlp;     /usr/local/bin/yt-dlp --version
2026-Mar-21 03:33:14.792969
#57 CACHED
2026-Mar-21 03:33:14.792969
2026-Mar-21 03:33:14.792969
#58 [api runtime 5/8] RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Mar-21 03:33:14.924423
#58 CACHED
2026-Mar-21 03:33:14.924423
2026-Mar-21 03:33:14.924423
#57 [worker runtime 4/8] RUN set -eux;     arch="$(dpkg --print-architecture)";     case "$arch" in       amd64) ytdlp_asset="yt-dlp_linux" ;;       arm64) ytdlp_asset="yt-dlp_linux_aarch64" ;;       *) echo "Unsupported architecture: $arch" >&2; exit 1 ;;     esac;     curl -fL "https://github.com/yt-dlp/yt-dlp/releases/latest/download/${ytdlp_asset}" -o /usr/local/bin/yt-dlp;     chmod +x /usr/local/bin/yt-dlp;     /usr/local/bin/yt-dlp --version
2026-Mar-21 03:33:14.924423
#57 CACHED
2026-Mar-21 03:33:14.924423
2026-Mar-21 03:33:14.924423
#55 [worker runtime 2/8] WORKDIR /app
2026-Mar-21 03:33:14.924423
#55 CACHED
2026-Mar-21 03:33:14.924423
2026-Mar-21 03:33:14.924423
#56 [worker runtime 3/8] RUN apt-get update && apt-get install -y     ca-certificates     curl     libssl3     && rm -rf /var/lib/apt/lists/*
2026-Mar-21 03:33:14.924423
#56 CACHED
2026-Mar-21 03:33:14.924423
2026-Mar-21 03:33:14.924423
#59 [worker runtime 6/8] COPY --from=builder /app/target/release/mux-worker /usr/local/bin/
2026-Mar-21 03:33:14.924423
#59 DONE 0.1s
2026-Mar-21 03:33:14.924423
2026-Mar-21 03:33:14.924423
#43 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-21 03:33:15.094170
#43 ...
2026-Mar-21 03:33:15.094170
2026-Mar-21 03:33:15.094170
#60 [worker runtime 7/8] COPY --from=builder /app/crates/api/app-migrations /app/app-migrations
2026-Mar-21 03:33:15.094170
#60 DONE 0.0s
2026-Mar-21 03:33:15.094170
2026-Mar-21 03:33:15.094170
#61 [worker runtime 8/8] RUN mkdir -p /app/extractors /app/proxy-state && chown -R appuser:appuser /app
2026-Mar-21 03:33:15.232085
#61 DONE 0.3s
2026-Mar-21 03:33:15.232085
2026-Mar-21 03:33:15.232085
#43 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-21 03:33:15.384375
#43 ...
2026-Mar-21 03:33:15.384375
2026-Mar-21 03:33:15.384375
#62 [worker] exporting to image
2026-Mar-21 03:33:15.384375
#62 exporting layers
2026-Mar-21 03:33:16.960451
#62 exporting layers 1.7s done
2026-Mar-21 03:33:17.137660
#62 exporting manifest sha256:72dbf28497b76f9b53284365b3d968f22e583f31e1865c38bfc2d9792d590160 done
2026-Mar-21 03:33:17.137660
#62 exporting config sha256:ed1907c62fea31910d070716c5d64a25fb316c8e8974ce9bcadc8b70d88a6640 done
2026-Mar-21 03:33:17.137660
#62 exporting attestation manifest sha256:96ff61f2e1e49e3e97b6097e84ad6fac091ffdd3ed665a6ede4d9553e3b06313 done
2026-Mar-21 03:33:17.137660
#62 exporting manifest list sha256:92f84cca400ddf4ba02fb5ea0a0d6dae23c616eb245417dea05db48dcee378af done
2026-Mar-21 03:33:17.137660
#62 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_worker:b6153d9e4682c1df48f2e096da582cb16d55164d done
2026-Mar-21 03:33:17.137660
#62 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_worker:b6153d9e4682c1df48f2e096da582cb16d55164d
2026-Mar-21 03:33:17.469311
#62 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_worker:b6153d9e4682c1df48f2e096da582cb16d55164d 0.5s done
2026-Mar-21 03:33:17.708643
#62 DONE 2.2s
2026-Mar-21 03:33:17.708643
2026-Mar-21 03:33:17.708643
#63 [worker] resolving provenance for metadata file
2026-Mar-21 03:33:17.708643
#63 DONE 0.0s
2026-Mar-21 03:33:17.708643
2026-Mar-21 03:33:17.708643
#43 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-21 03:33:38.769212
#43 249.8     Finished `release` profile [optimized] target(s) in 4m 09s
2026-Mar-21 03:33:38.991177
#43 DONE 250.0s
2026-Mar-21 03:33:39.122600
#64 [api runtime 6/8] COPY --from=builder /app/target/release/api-server /usr/local/bin/
2026-Mar-21 03:33:39.562445
#64 DONE 0.4s
2026-Mar-21 03:33:39.730342
#65 [api runtime 7/8] COPY --from=builder /app/crates/api/app-migrations /app/app-migrations
2026-Mar-21 03:33:39.730342
#65 DONE 0.0s
2026-Mar-21 03:33:39.730342
2026-Mar-21 03:33:39.730342
#66 [api runtime 8/8] RUN mkdir -p /app/extractors /app/data /app/proxy-state && chown -R appuser:appuser /app
2026-Mar-21 03:33:39.742751
#66 DONE 0.2s
2026-Mar-21 03:33:39.883651
#67 [api] exporting to image
2026-Mar-21 03:33:39.883651
#67 exporting layers
2026-Mar-21 03:33:41.893701
#67 exporting layers 2.2s done
2026-Mar-21 03:33:42.071423
#67 exporting manifest sha256:b5ca2995ddbe5fef634c8035e224cc464354d6473399809b12a44c83ef882091 done
2026-Mar-21 03:33:42.071423
#67 exporting config sha256:a7a9756196d58724439cb74954cf879afe4fb0c89890492216d2e5617f24f7a0 done
2026-Mar-21 03:33:42.071423
#67 exporting attestation manifest sha256:2db46fd25bbf26cc44d9c8f2b5c54f12f695e7468e6e743642a62b6fadec8594 done
2026-Mar-21 03:33:42.071423
#67 exporting manifest list sha256:c54e116b2bb14bae1185e304a2ef60ebf7d3992653abe5470571e94dc2dfb611 done
2026-Mar-21 03:33:42.071423
#67 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_api:b6153d9e4682c1df48f2e096da582cb16d55164d done
2026-Mar-21 03:33:42.071423
#67 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_api:b6153d9e4682c1df48f2e096da582cb16d55164d
2026-Mar-21 03:33:42.489643
#67 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_api:b6153d9e4682c1df48f2e096da582cb16d55164d 0.6s done
2026-Mar-21 03:33:42.690440
#67 DONE 2.8s
2026-Mar-21 03:33:42.690440
2026-Mar-21 03:33:42.690440
#68 [api] resolving provenance for metadata file
2026-Mar-21 03:33:42.690440
#68 DONE 0.0s
2026-Mar-21 03:33:42.702853
worker  Built
2026-Mar-21 03:33:42.702853
frontend  Built
2026-Mar-21 03:33:42.702853
api  Built
2026-Mar-21 03:33:42.766588
Creating .env file with runtime variables for container.
2026-Mar-21 03:33:43.439095
Removing old containers.
2026-Mar-21 03:33:44.407563
[CMD]: docker stop -t 30 frontend-o8kccgkgwsockoocow8sg88s-025857329021
2026-Mar-21 03:33:44.407563
frontend-o8kccgkgwsockoocow8sg88s-025857329021
2026-Mar-21 03:33:44.762868
[CMD]: docker rm -f frontend-o8kccgkgwsockoocow8sg88s-025857329021
2026-Mar-21 03:33:44.762868
frontend-o8kccgkgwsockoocow8sg88s-025857329021
2026-Mar-21 03:33:45.089297
[CMD]: docker stop -t 30 worker-o8kccgkgwsockoocow8sg88s-025857318604
2026-Mar-21 03:33:45.089297
worker-o8kccgkgwsockoocow8sg88s-025857318604
2026-Mar-21 03:33:45.452943
[CMD]: docker rm -f worker-o8kccgkgwsockoocow8sg88s-025857318604
2026-Mar-21 03:33:45.452943
worker-o8kccgkgwsockoocow8sg88s-025857318604
2026-Mar-21 03:33:45.775326
[CMD]: docker stop -t 30 api-o8kccgkgwsockoocow8sg88s-025857306176
2026-Mar-21 03:33:45.775326
api-o8kccgkgwsockoocow8sg88s-025857306176
2026-Mar-21 03:33:46.116585
[CMD]: docker rm -f api-o8kccgkgwsockoocow8sg88s-025857306176
2026-Mar-21 03:33:46.116585
api-o8kccgkgwsockoocow8sg88s-025857306176
2026-Mar-21 03:33:46.700461
[CMD]: docker stop -t 30 postgres-o8kccgkgwsockoocow8sg88s-025857295208
2026-Mar-21 03:33:46.700461
postgres-o8kccgkgwsockoocow8sg88s-025857295208
2026-Mar-21 03:33:47.053772
[CMD]: docker rm -f postgres-o8kccgkgwsockoocow8sg88s-025857295208
2026-Mar-21 03:33:47.053772
postgres-o8kccgkgwsockoocow8sg88s-025857295208
2026-Mar-21 03:33:47.600044
[CMD]: docker stop -t 30 shared-proxy-postgres-o8kccgkgwsockoocow8sg88s-025857282983
2026-Mar-21 03:33:47.600044
shared-proxy-postgres-o8kccgkgwsockoocow8sg88s-025857282983
2026-Mar-21 03:33:47.937217
[CMD]: docker rm -f shared-proxy-postgres-o8kccgkgwsockoocow8sg88s-025857282983
2026-Mar-21 03:33:47.937217
shared-proxy-postgres-o8kccgkgwsockoocow8sg88s-025857282983
2026-Mar-21 03:33:48.466639
[CMD]: docker stop -t 30 redis-o8kccgkgwsockoocow8sg88s-025857300896
2026-Mar-21 03:33:48.466639
redis-o8kccgkgwsockoocow8sg88s-025857300896
2026-Mar-21 03:33:48.810555
[CMD]: docker rm -f redis-o8kccgkgwsockoocow8sg88s-025857300896
2026-Mar-21 03:33:48.810555
redis-o8kccgkgwsockoocow8sg88s-025857300896
2026-Mar-21 03:33:49.361047
[CMD]: docker stop -t 30 shared-proxy-redis-o8kccgkgwsockoocow8sg88s-025857289841
2026-Mar-21 03:33:49.361047
shared-proxy-redis-o8kccgkgwsockoocow8sg88s-025857289841
2026-Mar-21 03:33:49.687638
[CMD]: docker rm -f shared-proxy-redis-o8kccgkgwsockoocow8sg88s-025857289841
2026-Mar-21 03:33:49.687638
shared-proxy-redis-o8kccgkgwsockoocow8sg88s-025857289841
2026-Mar-21 03:33:49.701759
Starting new application.
2026-Mar-21 03:33:50.797300
[CMD]: docker exec so800kkkogg8g0wo804c4wwc bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/so800kkkogg8g0wo804c4wwc/.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/so800kkkogg8g0wo804c4wwc -f /artifacts/so800kkkogg8g0wo804c4wwc/docker/docker-compose.server.yml up -d'
2026-Mar-21 03:33:50.797300
time="2026-03-21T03:33:50Z" level=warning msg="volume \"o8kccgkgwsockoocow8sg88s_postgres-data\" already exists but was not created by Docker Compose. Use `external: true` to use an existing volume"
2026-Mar-21 03:33:50.812043
Container postgres-o8kccgkgwsockoocow8sg88s-032922682138  Creating
2026-Mar-21 03:33:50.812043
Container redis-o8kccgkgwsockoocow8sg88s-032922690368  Creating
2026-Mar-21 03:33:50.812043
Container shared-proxy-postgres-o8kccgkgwsockoocow8sg88s-032922667500  Creating
2026-Mar-21 03:33:50.812043
Container shared-proxy-redis-o8kccgkgwsockoocow8sg88s-032922675805  Creating
2026-Mar-21 03:33:51.074880
Container shared-proxy-redis-o8kccgkgwsockoocow8sg88s-032922675805  Created
2026-Mar-21 03:33:51.074880
Container postgres-o8kccgkgwsockoocow8sg88s-032922682138  Created
2026-Mar-21 03:33:51.087538
Container redis-o8kccgkgwsockoocow8sg88s-032922690368  Created
2026-Mar-21 03:33:51.087538
Container shared-proxy-postgres-o8kccgkgwsockoocow8sg88s-032922667500  Created
2026-Mar-21 03:33:51.087538
Container worker-o8kccgkgwsockoocow8sg88s-032922707515  Creating
2026-Mar-21 03:33:51.087538
Container api-o8kccgkgwsockoocow8sg88s-032922695744  Creating
2026-Mar-21 03:33:51.111058
Container worker-o8kccgkgwsockoocow8sg88s-032922707515  Created
2026-Mar-21 03:33:51.125416
Container api-o8kccgkgwsockoocow8sg88s-032922695744  Created
2026-Mar-21 03:33:51.125416
Container frontend-o8kccgkgwsockoocow8sg88s-032922717660  Creating
2026-Mar-21 03:33:51.145383
Container frontend-o8kccgkgwsockoocow8sg88s-032922717660  Created
2026-Mar-21 03:33:51.163100
Container redis-o8kccgkgwsockoocow8sg88s-032922690368  Starting
2026-Mar-21 03:33:51.163100
Container shared-proxy-postgres-o8kccgkgwsockoocow8sg88s-032922667500  Starting
2026-Mar-21 03:33:51.163100
Container postgres-o8kccgkgwsockoocow8sg88s-032922682138  Starting
2026-Mar-21 03:33:51.176441
Container shared-proxy-redis-o8kccgkgwsockoocow8sg88s-032922675805  Starting
2026-Mar-21 03:33:51.320584
Container shared-proxy-redis-o8kccgkgwsockoocow8sg88s-032922675805  Started
2026-Mar-21 03:33:51.334566
Container redis-o8kccgkgwsockoocow8sg88s-032922690368  Started
2026-Mar-21 03:33:51.358117
Container shared-proxy-postgres-o8kccgkgwsockoocow8sg88s-032922667500  Started
2026-Mar-21 03:33:51.556696
Error response from daemon: failed to set up container networking: driver failed programming external connectivity on endpoint postgres-o8kccgkgwsockoocow8sg88s-032922682138 (99db4f0ab8c7215daeecc01b076673f4ba2f146ea9b725114e57f66dfa2cbf8f): Bind for 127.0.0.1:15431 failed: port is already allocated
2026-Mar-21 03:33:51.570064
exit status 1
2026-Mar-21 03:33:51.623285
========================================
2026-Mar-21 03:33:51.644259
Deployment failed: Command execution failed (exit code 1): docker exec so800kkkogg8g0wo804c4wwc bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/so800kkkogg8g0wo804c4wwc/.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/so800kkkogg8g0wo804c4wwc -f /artifacts/so800kkkogg8g0wo804c4wwc/docker/docker-compose.server.yml up -d'
2026-Mar-21 03:33:51.644259
Error: time="2026-03-21T03:33:50Z" level=warning msg="volume \"o8kccgkgwsockoocow8sg88s_postgres-data\" already exists but was not created by Docker Compose. Use `external: true` to use an existing volume"
2026-Mar-21 03:33:51.644259
Container postgres-o8kccgkgwsockoocow8sg88s-032922682138  Creating
2026-Mar-21 03:33:51.644259
Container redis-o8kccgkgwsockoocow8sg88s-032922690368  Creating
2026-Mar-21 03:33:51.644259
Container shared-proxy-postgres-o8kccgkgwsockoocow8sg88s-032922667500  Creating
2026-Mar-21 03:33:51.644259
Container shared-proxy-redis-o8kccgkgwsockoocow8sg88s-032922675805  Creating
2026-Mar-21 03:33:51.644259
Container shared-proxy-redis-o8kccgkgwsockoocow8sg88s-032922675805  Created
2026-Mar-21 03:33:51.644259
Container postgres-o8kccgkgwsockoocow8sg88s-032922682138  Created
2026-Mar-21 03:33:51.644259
Container redis-o8kccgkgwsockoocow8sg88s-032922690368  Created
2026-Mar-21 03:33:51.644259
Container shared-proxy-postgres-o8kccgkgwsockoocow8sg88s-032922667500  Created
2026-Mar-21 03:33:51.644259
Container worker-o8kccgkgwsockoocow8sg88s-032922707515  Creating
2026-Mar-21 03:33:51.644259
Container api-o8kccgkgwsockoocow8sg88s-032922695744  Creating
2026-Mar-21 03:33:51.644259
Container worker-o8kccgkgwsockoocow8sg88s-032922707515  Created
2026-Mar-21 03:33:51.644259
Container api-o8kccgkgwsockoocow8sg88s-032922695744  Created
2026-Mar-21 03:33:51.644259
Container frontend-o8kccgkgwsockoocow8sg88s-032922717660  Creating
2026-Mar-21 03:33:51.644259
Container frontend-o8kccgkgwsockoocow8sg88s-032922717660  Created
2026-Mar-21 03:33:51.644259
Container redis-o8kccgkgwsockoocow8sg88s-032922690368  Starting
2026-Mar-21 03:33:51.644259
Container shared-proxy-postgres-o8kccgkgwsockoocow8sg88s-032922667500  Starting
2026-Mar-21 03:33:51.644259
Container postgres-o8kccgkgwsockoocow8sg88s-032922682138  Starting
2026-Mar-21 03:33:51.644259
Container shared-proxy-redis-o8kccgkgwsockoocow8sg88s-032922675805  Starting
2026-Mar-21 03:33:51.644259
Container shared-proxy-redis-o8kccgkgwsockoocow8sg88s-032922675805  Started
2026-Mar-21 03:33:51.644259
Container redis-o8kccgkgwsockoocow8sg88s-032922690368  Started
2026-Mar-21 03:33:51.644259
Container shared-proxy-postgres-o8kccgkgwsockoocow8sg88s-032922667500  Started
2026-Mar-21 03:33:51.644259
Error response from daemon: failed to set up container networking: driver failed programming external connectivity on endpoint postgres-o8kccgkgwsockoocow8sg88s-032922682138 (99db4f0ab8c7215daeecc01b076673f4ba2f146ea9b725114e57f66dfa2cbf8f): Bind for 127.0.0.1:15431 failed: port is already allocated
2026-Mar-21 03:33:51.644259
exit status 1
2026-Mar-21 03:33:51.667126
Error type: RuntimeException
2026-Mar-21 03:33:51.691022
Error code: 0
2026-Mar-21 03:33:51.713451
Location: /var/www/html/app/Traits/ExecuteRemoteCommand.php:243
2026-Mar-21 03:33:51.733465
Stack trace (first 5 lines):
2026-Mar-21 03:33:51.750706
#0 /var/www/html/app/Traits/ExecuteRemoteCommand.php(104): App\Jobs\ApplicationDeploymentJob->executeCommandWithProcess()
2026-Mar-21 03:33:51.773005
#1 /var/www/html/vendor/laravel/framework/src/Illuminate/Collections/Traits/EnumeratesValues.php(272): App\Jobs\ApplicationDeploymentJob->{closure:App\Traits\ExecuteRemoteCommand::execute_remote_command():71}()
2026-Mar-21 03:33:51.793196
#2 /var/www/html/app/Traits/ExecuteRemoteCommand.php(71): Illuminate\Support\Collection->each()
2026-Mar-21 03:33:51.812716
#3 /var/www/html/app/Jobs/ApplicationDeploymentJob.php(816): App\Jobs\ApplicationDeploymentJob->execute_remote_command()
2026-Mar-21 03:33:51.833950
#4 /var/www/html/app/Jobs/ApplicationDeploymentJob.php(467): App\Jobs\ApplicationDeploymentJob->deploy_docker_compose_buildpack()
2026-Mar-21 03:33:51.854540
========================================