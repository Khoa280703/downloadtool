2026-Mar-23 01:56:29.194336
Starting deployment of khoa280703/downloadtool:main-zoscg4oc04gkwkssg0kw8w8w to localhost.
2026-Mar-23 01:56:29.347364
Preparing container with helper image: ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Mar-23 01:56:29.440484
[CMD]: docker stop -t 30 row00kkoog4wc0cc8c400888
2026-Mar-23 01:56:29.440484
Error response from daemon: No such container: row00kkoog4wc0cc8c400888
2026-Mar-23 01:56:29.555269
[CMD]: docker run -d --network coolify --name row00kkoog4wc0cc8c400888  --rm -v /var/run/docker.sock:/var/run/docker.sock ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Mar-23 01:56:29.555269
14dcce21a2775cf4ddf54a10153676bc8c0800477d068e32c9ed6daf320e066d
2026-Mar-23 01:56:30.552913
[CMD]: docker exec row00kkoog4wc0cc8c400888 bash -c 'GIT_SSH_COMMAND="ssh -o ConnectTimeout=30 -p 22 -o Port=22 -o LogLevel=ERROR -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git ls-remote https://github.com/Khoa280703/downloadtool refs/heads/main'
2026-Mar-23 01:56:30.552913
3bf1238b2350ae4f3732e6368c475a81f97af531	refs/heads/main
2026-Mar-23 01:56:30.562679
----------------------------------------
2026-Mar-23 01:56:30.567742
Importing Khoa280703/downloadtool:main (commit sha 3bf1238b2350ae4f3732e6368c475a81f97af531) to /artifacts/row00kkoog4wc0cc8c400888.
2026-Mar-23 01:56:30.718073
[CMD]: docker exec row00kkoog4wc0cc8c400888 bash -c 'git clone --depth=1 --recurse-submodules --shallow-submodules -b 'main' 'https://github.com/Khoa280703/downloadtool' '/artifacts/row00kkoog4wc0cc8c400888' && cd '/artifacts/row00kkoog4wc0cc8c400888' && if [ -f .gitmodules ]; then sed -i "s#git@\(.*\):#https://\1/#g" '/artifacts/row00kkoog4wc0cc8c400888'/.gitmodules || true && git submodule sync && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git submodule update --init --recursive --depth=1; fi && cd '/artifacts/row00kkoog4wc0cc8c400888' && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git lfs pull'
2026-Mar-23 01:56:30.718073
Cloning into '/artifacts/row00kkoog4wc0cc8c400888'...
2026-Mar-23 01:56:32.550001
[CMD]: docker exec row00kkoog4wc0cc8c400888 bash -c 'cd /artifacts/row00kkoog4wc0cc8c400888 && git log -1 3bf1238b2350ae4f3732e6368c475a81f97af531 --pretty=%B'
2026-Mar-23 01:56:32.550001
fix(migrations): restore artifact migration checksum
2026-Mar-23 01:56:35.434554
[CMD]: docker exec row00kkoog4wc0cc8c400888 bash -c 'test -f /artifacts/row00kkoog4wc0cc8c400888/docker/Dockerfile.api && echo 'exists' || echo 'not found''
2026-Mar-23 01:56:35.434554
exists
2026-Mar-23 01:56:35.572796
[CMD]: docker exec row00kkoog4wc0cc8c400888 bash -c 'cat /artifacts/row00kkoog4wc0cc8c400888/docker/Dockerfile.api'
2026-Mar-23 01:56:35.572796
# Dockerfile for API service deployment
2026-Mar-23 01:56:35.572796
# Builds the API server and related components without GPU support
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
# Stage 0: Build injector JS (embedded into api crate via include_str! at compile time)
2026-Mar-23 01:56:35.572796
FROM node:22-alpine AS js-builder
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
WORKDIR /app
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
RUN npm install -g pnpm
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
# Copy workspace manifests for pnpm resolution
2026-Mar-23 01:56:35.572796
COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Mar-23 01:56:35.572796
COPY packages/api-client/package.json ./packages/api-client/
2026-Mar-23 01:56:35.572796
COPY apps/injector/package.json ./apps/injector/
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
# Copy injector source and shared packages
2026-Mar-23 01:56:35.572796
COPY apps/injector/ ./apps/injector/
2026-Mar-23 01:56:35.572796
COPY packages/ ./packages/
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
# Install deps and build injector (produces dist/bm.js and dist/youtube-downloader.user.js)
2026-Mar-23 01:56:35.572796
RUN pnpm install --frozen-lockfile
2026-Mar-23 01:56:35.572796
RUN pnpm --filter @downloadtool/injector build
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
# Build extractor TypeScript to IIFE format (required by crates/extractor/build.rs)
2026-Mar-23 01:56:35.572796
COPY extractors/ ./extractors/
2026-Mar-23 01:56:35.572796
RUN mkdir -p extractors/dist && \
2026-Mar-23 01:56:35.572796
npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js && \
2026-Mar-23 01:56:35.572796
npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
# Stage 1: Rust builder
2026-Mar-23 01:56:35.572796
FROM rust:1.91-bookworm AS builder
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
WORKDIR /app
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
# Install dependencies
2026-Mar-23 01:56:35.572796
RUN apt-get update && apt-get install -y \
2026-Mar-23 01:56:35.572796
pkg-config \
2026-Mar-23 01:56:35.572796
libssl-dev \
2026-Mar-23 01:56:35.572796
&& rm -rf /var/lib/apt/lists/*
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
# Copy workspace configuration
2026-Mar-23 01:56:35.572796
COPY Cargo.toml ./
2026-Mar-23 01:56:35.572796
COPY Cargo.lock ./
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
# Copy all crates
2026-Mar-23 01:56:35.572796
COPY crates/ ./crates/
2026-Mar-23 01:56:35.572796
COPY config/ ./config/
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
# Copy injector dist (required by include_str! in crates/api/src/routes/static_files.rs)
2026-Mar-23 01:56:35.572796
COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
# Copy extractor source + pre-built IIFE dist (built by js-builder stage)
2026-Mar-23 01:56:35.572796
COPY extractors/ ./extractors/
2026-Mar-23 01:56:35.572796
COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
# Build the release binary
2026-Mar-23 01:56:35.572796
RUN cargo build --release --bin api-server
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
# Stage 2: Runtime
2026-Mar-23 01:56:35.572796
FROM debian:bookworm-slim AS runtime
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
WORKDIR /app
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
# Install runtime dependencies
2026-Mar-23 01:56:35.572796
RUN apt-get update && apt-get install -y \
2026-Mar-23 01:56:35.572796
ca-certificates \
2026-Mar-23 01:56:35.572796
curl \
2026-Mar-23 01:56:35.572796
libssl3 \
2026-Mar-23 01:56:35.572796
&& rm -rf /var/lib/apt/lists/*
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
# Install latest yt-dlp binary (newer than Debian package).
2026-Mar-23 01:56:35.572796
RUN set -eux; \
2026-Mar-23 01:56:35.572796
arch="$(dpkg --print-architecture)"; \
2026-Mar-23 01:56:35.572796
case "$arch" in \
2026-Mar-23 01:56:35.572796
amd64) ytdlp_asset="yt-dlp_linux" ;; \
2026-Mar-23 01:56:35.572796
arm64) ytdlp_asset="yt-dlp_linux_aarch64" ;; \
2026-Mar-23 01:56:35.572796
*) echo "Unsupported architecture: $arch" >&2; exit 1 ;; \
2026-Mar-23 01:56:35.572796
esac; \
2026-Mar-23 01:56:35.572796
curl -fL "https://github.com/yt-dlp/yt-dlp/releases/latest/download/${ytdlp_asset}" -o /usr/local/bin/yt-dlp; \
2026-Mar-23 01:56:35.572796
chmod +x /usr/local/bin/yt-dlp; \
2026-Mar-23 01:56:35.572796
/usr/local/bin/yt-dlp --version
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
# Create non-root user
2026-Mar-23 01:56:35.572796
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
# Copy binary from builder
2026-Mar-23 01:56:35.572796
COPY --from=builder /app/target/release/api-server /usr/local/bin/
2026-Mar-23 01:56:35.572796
COPY --from=builder /app/crates/api/app-migrations /app/app-migrations
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
# Create directories
2026-Mar-23 01:56:35.572796
RUN mkdir -p /app/extractors /app/data /app/proxy-state && chown -R appuser:appuser /app
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
# Switch to non-root user
2026-Mar-23 01:56:35.572796
USER appuser
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
# Environment variables
2026-Mar-23 01:56:35.572796
ENV PORT=3068
2026-Mar-23 01:56:35.572796
ENV EXTRACTOR_DIR=/app/extractors
2026-Mar-23 01:56:35.572796
ENV YTDLP_PATH=/usr/local/bin/yt-dlp
2026-Mar-23 01:56:35.572796
ENV RUST_LOG=info
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
# Expose port
2026-Mar-23 01:56:35.572796
EXPOSE 3068
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
# Health check
2026-Mar-23 01:56:35.572796
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Mar-23 01:56:35.572796
CMD curl -f http://localhost:3068/health || exit 1
2026-Mar-23 01:56:35.572796
2026-Mar-23 01:56:35.572796
# Run the server
2026-Mar-23 01:56:35.572796
CMD ["api-server"]
2026-Mar-23 01:56:35.731565
Added 81 ARG declarations to Dockerfile for service api (multi-stage build, added to 3 stages).
2026-Mar-23 01:56:35.877526
[CMD]: docker exec row00kkoog4wc0cc8c400888 bash -c 'test -f /artifacts/row00kkoog4wc0cc8c400888/docker/Dockerfile.worker && echo 'exists' || echo 'not found''
2026-Mar-23 01:56:35.877526
exists
2026-Mar-23 01:56:36.032570
[CMD]: docker exec row00kkoog4wc0cc8c400888 bash -c 'cat /artifacts/row00kkoog4wc0cc8c400888/docker/Dockerfile.worker'
2026-Mar-23 01:56:36.032570
# Dockerfile for mux worker deployment
2026-Mar-23 01:56:36.032570
2026-Mar-23 01:56:36.032570
# Stage 0: Build extractor TypeScript to IIFE format (required by crates/extractor/build.rs)
2026-Mar-23 01:56:36.032570
FROM node:22-alpine AS js-builder
2026-Mar-23 01:56:36.032570
2026-Mar-23 01:56:36.032570
WORKDIR /app
2026-Mar-23 01:56:36.032570
2026-Mar-23 01:56:36.032570
RUN npm install -g pnpm
2026-Mar-23 01:56:36.032570
2026-Mar-23 01:56:36.032570
COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Mar-23 01:56:36.032570
COPY packages/api-client/package.json ./packages/api-client/
2026-Mar-23 01:56:36.032570
COPY apps/injector/package.json ./apps/injector/
2026-Mar-23 01:56:36.032570
COPY packages/ ./packages/
2026-Mar-23 01:56:36.032570
COPY apps/injector/ ./apps/injector/
2026-Mar-23 01:56:36.032570
COPY extractors/ ./extractors/
2026-Mar-23 01:56:36.032570
2026-Mar-23 01:56:36.032570
RUN pnpm install --frozen-lockfile
2026-Mar-23 01:56:36.032570
RUN mkdir -p extractors/dist && \
2026-Mar-23 01:56:36.032570
npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js && \
2026-Mar-23 01:56:36.032570
npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-23 01:56:36.032570
2026-Mar-23 01:56:36.032570
# Stage 1: Rust builder
2026-Mar-23 01:56:36.032570
FROM rust:1.91-bookworm AS builder
2026-Mar-23 01:56:36.032570
2026-Mar-23 01:56:36.032570
WORKDIR /app
2026-Mar-23 01:56:36.032570
2026-Mar-23 01:56:36.032570
RUN apt-get update && apt-get install -y \
2026-Mar-23 01:56:36.032570
pkg-config \
2026-Mar-23 01:56:36.032570
libssl-dev \
2026-Mar-23 01:56:36.032570
&& rm -rf /var/lib/apt/lists/*
2026-Mar-23 01:56:36.032570
2026-Mar-23 01:56:36.032570
COPY Cargo.toml ./
2026-Mar-23 01:56:36.032570
COPY Cargo.lock ./
2026-Mar-23 01:56:36.032570
COPY crates/ ./crates/
2026-Mar-23 01:56:36.032570
COPY config/ ./config/
2026-Mar-23 01:56:36.032570
COPY extractors/ ./extractors/
2026-Mar-23 01:56:36.032570
COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-23 01:56:36.032570
2026-Mar-23 01:56:36.032570
RUN cargo build --release --bin mux-worker
2026-Mar-23 01:56:36.032570
2026-Mar-23 01:56:36.032570
# Stage 2: Runtime
2026-Mar-23 01:56:36.032570
FROM debian:bookworm-slim AS runtime
2026-Mar-23 01:56:36.032570
2026-Mar-23 01:56:36.032570
WORKDIR /app
2026-Mar-23 01:56:36.032570
2026-Mar-23 01:56:36.032570
RUN apt-get update && apt-get install -y \
2026-Mar-23 01:56:36.032570
ca-certificates \
2026-Mar-23 01:56:36.032570
curl \
2026-Mar-23 01:56:36.032570
libssl3 \
2026-Mar-23 01:56:36.032570
&& rm -rf /var/lib/apt/lists/*
2026-Mar-23 01:56:36.032570
2026-Mar-23 01:56:36.032570
RUN set -eux; \
2026-Mar-23 01:56:36.032570
arch="$(dpkg --print-architecture)"; \
2026-Mar-23 01:56:36.032570
case "$arch" in \
2026-Mar-23 01:56:36.032570
amd64) ytdlp_asset="yt-dlp_linux" ;; \
2026-Mar-23 01:56:36.032570
arm64) ytdlp_asset="yt-dlp_linux_aarch64" ;; \
2026-Mar-23 01:56:36.032570
*) echo "Unsupported architecture: $arch" >&2; exit 1 ;; \
2026-Mar-23 01:56:36.032570
esac; \
2026-Mar-23 01:56:36.032570
curl -fL "https://github.com/yt-dlp/yt-dlp/releases/latest/download/${ytdlp_asset}" -o /usr/local/bin/yt-dlp; \
2026-Mar-23 01:56:36.032570
chmod +x /usr/local/bin/yt-dlp; \
2026-Mar-23 01:56:36.032570
/usr/local/bin/yt-dlp --version
2026-Mar-23 01:56:36.032570
2026-Mar-23 01:56:36.032570
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Mar-23 01:56:36.032570
2026-Mar-23 01:56:36.032570
COPY --from=builder /app/target/release/mux-worker /usr/local/bin/
2026-Mar-23 01:56:36.032570
COPY --from=builder /app/crates/api/app-migrations /app/app-migrations
2026-Mar-23 01:56:36.032570
2026-Mar-23 01:56:36.032570
RUN mkdir -p /app/extractors /app/proxy-state && chown -R appuser:appuser /app
2026-Mar-23 01:56:36.032570
2026-Mar-23 01:56:36.032570
USER appuser
2026-Mar-23 01:56:36.032570
2026-Mar-23 01:56:36.032570
ENV EXTRACTOR_DIR=/app/extractors
2026-Mar-23 01:56:36.032570
ENV YTDLP_PATH=/usr/local/bin/yt-dlp
2026-Mar-23 01:56:36.032570
ENV RUST_LOG=info
2026-Mar-23 01:56:36.032570
2026-Mar-23 01:56:36.032570
CMD ["mux-worker"]
2026-Mar-23 01:56:36.186122
Added 81 ARG declarations to Dockerfile for service worker (multi-stage build, added to 3 stages).
2026-Mar-23 01:56:36.336688
[CMD]: docker exec row00kkoog4wc0cc8c400888 bash -c 'test -f /artifacts/row00kkoog4wc0cc8c400888/docker/Dockerfile.frontend && echo 'exists' || echo 'not found''
2026-Mar-23 01:56:36.336688
exists
2026-Mar-23 01:56:36.477167
[CMD]: docker exec row00kkoog4wc0cc8c400888 bash -c 'cat /artifacts/row00kkoog4wc0cc8c400888/docker/Dockerfile.frontend'
2026-Mar-23 01:56:36.477167
# Dockerfile for frontend (SvelteKit Node server)
2026-Mar-23 01:56:36.477167
# Copy ALL source files BEFORE npm install so svelte-kit sync (prepare script)
2026-Mar-23 01:56:36.477167
# can find svelte.config.js and generate .svelte-kit/ correctly.
2026-Mar-23 01:56:36.477167
2026-Mar-23 01:56:36.477167
FROM node:22-alpine AS builder
2026-Mar-23 01:56:36.477167
2026-Mar-23 01:56:36.477167
WORKDIR /app
2026-Mar-23 01:56:36.477167
2026-Mar-23 01:56:36.477167
# Copy all frontend source files first (node_modules excluded via .dockerignore)
2026-Mar-23 01:56:36.477167
COPY frontend/ ./
2026-Mar-23 01:56:36.477167
COPY config/ /config/
2026-Mar-23 01:56:36.477167
2026-Mar-23 01:56:36.477167
# Install — prepare script runs svelte-kit sync with svelte.config.js available
2026-Mar-23 01:56:36.477167
RUN npm install
2026-Mar-23 01:56:36.477167
2026-Mar-23 01:56:36.477167
# Build-time public API URL (embedded into client bundle by Vite)
2026-Mar-23 01:56:36.477167
# Runtime env is too late for import.meta.env in browser bundle.
2026-Mar-23 01:56:36.477167
ARG VITE_API_URL
2026-Mar-23 01:56:36.477167
ENV VITE_API_URL=${VITE_API_URL}
2026-Mar-23 01:56:36.477167
RUN test -n "$VITE_API_URL" || (echo "VITE_API_URL build arg is required" && exit 1)
2026-Mar-23 01:56:36.477167
2026-Mar-23 01:56:36.477167
# Generate Paraglide runtime/messages from frontend/messages/* before Vite build
2026-Mar-23 01:56:36.477167
RUN npm run paraglide:compile
2026-Mar-23 01:56:36.477167
2026-Mar-23 01:56:36.477167
# Build via programmatic API — bypasses vite.config.ts loading (esbuild issues on Alpine)
2026-Mar-23 01:56:36.477167
RUN node build-docker.mjs
2026-Mar-23 01:56:36.477167
2026-Mar-23 01:56:36.477167
# Runtime
2026-Mar-23 01:56:36.477167
FROM node:22-alpine AS runtime
2026-Mar-23 01:56:36.477167
2026-Mar-23 01:56:36.477167
WORKDIR /app
2026-Mar-23 01:56:36.477167
2026-Mar-23 01:56:36.477167
COPY --from=builder /app/build ./build
2026-Mar-23 01:56:36.477167
COPY --from=builder /app/package.json ./
2026-Mar-23 01:56:36.477167
COPY --from=builder /app/package-lock.json ./
2026-Mar-23 01:56:36.477167
2026-Mar-23 01:56:36.477167
# Runtime needs server-side deps (better-auth, pg) used by hooks/routes
2026-Mar-23 01:56:36.477167
RUN npm ci --omit=dev
2026-Mar-23 01:56:36.477167
2026-Mar-23 01:56:36.477167
ENV PORT=5168
2026-Mar-23 01:56:36.477167
ENV HOST=0.0.0.0
2026-Mar-23 01:56:36.477167
2026-Mar-23 01:56:36.477167
EXPOSE 5168
2026-Mar-23 01:56:36.477167
2026-Mar-23 01:56:36.477167
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Mar-23 01:56:36.477167
CMD wget -qO- http://127.0.0.1:5168 || exit 1
2026-Mar-23 01:56:36.477167
2026-Mar-23 01:56:36.477167
CMD ["node", "build"]
2026-Mar-23 01:56:36.627619
Added 54 ARG declarations to Dockerfile for service frontend (multi-stage build, added to 2 stages).
2026-Mar-23 01:56:36.633208
Pulling & building required images.
2026-Mar-23 01:56:36.683873
Creating build-time .env file in /artifacts (outside Docker context).
2026-Mar-23 01:56:36.835800
Adding build arguments to Docker Compose build command.
2026-Mar-23 01:56:37.119671
[CMD]: docker exec row00kkoog4wc0cc8c400888 bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/build-time.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/row00kkoog4wc0cc8c400888 -f /artifacts/row00kkoog4wc0cc8c400888/docker/docker-compose.server.yml build --pull --build-arg COOLIFY_URL --build-arg COOLIFY_FQDN --build-arg SERVICE_URL_API --build-arg BETTER_AUTH_TRUSTED_ORIGINS --build-arg WHOP_WEBHOOK_SECRET --build-arg SERVICE_FQDN_FRONTEND --build-arg POSTGRES_PASSWORD --build-arg ORIGIN --build-arg SERVICE_FQDN_API --build-arg BETTER_AUTH_SECRET --build-arg GOOGLE_CLIENT_ID --build-arg VITE_API_URL --build-arg WHOP_PLAN_ID --build-arg GOOGLE_CLIENT_SECRET --build-arg S3_REGION --build-arg S3_ENDPOINT --build-arg S3_BUCKET_NAME --build-arg S3_ACCESS_KEY_ID --build-arg S3_SECRET_ACCESS_KEY --build-arg MUX_ARTIFACT_TTL_SECS --build-arg MUX_CLEANUP_INTERVAL_SECS --build-arg MUX_FILE_TICKET_TTL_SECS --build-arg PROXY_QUARANTINE_TTL_SECS --build-arg ADMIN_EMAILS --build-arg SHARED_PROXY_POSTGRES_PASSWORD --build-arg SERVICE_URL_FRONTEND --build-arg MUX_WORKER_CONCURRENCY --build-arg COOLIFY_BUILD_SECRETS_HASH=c8a6670d0bb115c25e090e96cdbe00d1a4714ec95abb323c725abeed4171a126'
2026-Mar-23 01:56:37.119671
#1 [internal] load local bake definitions
2026-Mar-23 01:56:37.335371
#1 reading from stdin 6.33kB done
2026-Mar-23 01:56:37.335371
#1 DONE 0.0s
2026-Mar-23 01:56:37.335371
2026-Mar-23 01:56:37.335371
#2 [worker internal] load build definition from Dockerfile.worker
2026-Mar-23 01:56:37.335371
#2 transferring dockerfile: 4.16kB done
2026-Mar-23 01:56:37.335371
#2 DONE 0.0s
2026-Mar-23 01:56:37.335371
2026-Mar-23 01:56:37.335371
#3 [frontend internal] load build definition from Dockerfile.frontend
2026-Mar-23 01:56:37.335371
#3 transferring dockerfile: 2.69kB done
2026-Mar-23 01:56:37.335371
#3 DONE 0.0s
2026-Mar-23 01:56:37.335371
2026-Mar-23 01:56:37.335371
#4 [api internal] load build definition from Dockerfile.api
2026-Mar-23 01:56:37.335371
#4 transferring dockerfile: 5.30kB done
2026-Mar-23 01:56:37.335371
#4 DONE 0.0s
2026-Mar-23 01:56:37.335371
2026-Mar-23 01:56:37.335371
#5 [api internal] load metadata for docker.io/library/node:22-alpine
2026-Mar-23 01:56:38.449514
#5 ...
2026-Mar-23 01:56:38.449514
2026-Mar-23 01:56:38.449514
#6 [api internal] load metadata for docker.io/library/rust:1.91-bookworm
2026-Mar-23 01:56:38.449514
#6 DONE 1.1s
2026-Mar-23 01:56:38.449514
2026-Mar-23 01:56:38.449514
#7 [worker internal] load metadata for docker.io/library/debian:bookworm-slim
2026-Mar-23 01:56:38.449514
#7 DONE 1.1s
2026-Mar-23 01:56:38.599585
#5 [api internal] load metadata for docker.io/library/node:22-alpine
2026-Mar-23 01:56:38.851388
#5 DONE 1.6s
2026-Mar-23 01:56:38.851388
2026-Mar-23 01:56:38.851388
#8 [worker internal] load .dockerignore
2026-Mar-23 01:56:38.851388
#8 transferring context: 341B done
2026-Mar-23 01:56:38.851388
#8 DONE 0.0s
2026-Mar-23 01:56:38.851388
2026-Mar-23 01:56:38.851388
#9 [worker runtime 1/8] FROM docker.io/library/debian:bookworm-slim@sha256:f06537653ac770703bc45b4b113475bd402f451e85223f0f2837acbf89ab020a
2026-Mar-23 01:56:38.851388
#9 resolve docker.io/library/debian:bookworm-slim@sha256:f06537653ac770703bc45b4b113475bd402f451e85223f0f2837acbf89ab020a 0.0s done
2026-Mar-23 01:56:38.851388
#9 DONE 0.0s
2026-Mar-23 01:56:38.851388
2026-Mar-23 01:56:38.851388
#10 [worker builder 1/8] FROM docker.io/library/node:22-alpine@sha256:8094c002d08262dba12645a3b4a15cd6cd627d30bc782f53229a2ec13ee22a00
2026-Mar-23 01:56:38.851388
#10 resolve docker.io/library/node:22-alpine@sha256:8094c002d08262dba12645a3b4a15cd6cd627d30bc782f53229a2ec13ee22a00 0.0s done
2026-Mar-23 01:56:38.851388
#10 DONE 0.0s
2026-Mar-23 01:56:38.851388
2026-Mar-23 01:56:38.851388
#11 [worker builder  1/11] FROM docker.io/library/rust:1.91-bookworm@sha256:c1e5f19e773b7878c3f7a805dd00a495e747acbdc76fb2337a4ebf0418896b33
2026-Mar-23 01:56:38.851388
#11 resolve docker.io/library/rust:1.91-bookworm@sha256:c1e5f19e773b7878c3f7a805dd00a495e747acbdc76fb2337a4ebf0418896b33 0.0s done
2026-Mar-23 01:56:38.851388
#11 DONE 0.0s
2026-Mar-23 01:56:38.851388
2026-Mar-23 01:56:38.851388
#12 [api internal] load build context
2026-Mar-23 01:56:38.851388
#12 transferring context: 1.03MB 0.0s done
2026-Mar-23 01:56:38.851388
#12 DONE 0.0s
2026-Mar-23 01:56:38.851388
2026-Mar-23 01:56:38.851388
#13 [api builder  3/11] RUN apt-get update && apt-get install -y     pkg-config     libssl-dev     && rm -rf /var/lib/apt/lists/*
2026-Mar-23 01:56:38.851388
#13 CACHED
2026-Mar-23 01:56:38.851388
2026-Mar-23 01:56:38.851388
#14 [api builder  2/11] WORKDIR /app
2026-Mar-23 01:56:38.851388
#14 CACHED
2026-Mar-23 01:56:38.851388
2026-Mar-23 01:56:38.851388
#15 [api builder  4/11] COPY Cargo.toml ./
2026-Mar-23 01:56:38.851388
#15 CACHED
2026-Mar-23 01:56:38.851388
2026-Mar-23 01:56:38.851388
#16 [api builder  5/11] COPY Cargo.lock ./
2026-Mar-23 01:56:38.851388
#16 CACHED
2026-Mar-23 01:56:38.851388
2026-Mar-23 01:56:38.851388
#17 [frontend internal] load build context
2026-Mar-23 01:56:38.851388
#17 transferring context: 2.07MB 0.0s done
2026-Mar-23 01:56:38.851388
#17 DONE 0.0s
2026-Mar-23 01:56:38.851388
2026-Mar-23 01:56:38.851388
#18 [worker js-builder  3/12] RUN npm install -g pnpm
2026-Mar-23 01:56:38.851388
#18 CACHED
2026-Mar-23 01:56:38.851388
2026-Mar-23 01:56:38.851388
#19 [worker js-builder  4/12] COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Mar-23 01:56:38.851388
#19 CACHED
2026-Mar-23 01:56:38.851388
2026-Mar-23 01:56:38.851388
#20 [worker js-builder  5/12] COPY packages/api-client/package.json ./packages/api-client/
2026-Mar-23 01:56:38.851388
#20 CACHED
2026-Mar-23 01:56:38.851388
2026-Mar-23 01:56:38.851388
#21 [worker js-builder  8/11] COPY apps/injector/ ./apps/injector/
2026-Mar-23 01:56:38.851388
#21 CACHED
2026-Mar-23 01:56:38.851388
2026-Mar-23 01:56:38.851388
#22 [worker js-builder  9/11] COPY extractors/ ./extractors/
2026-Mar-23 01:56:38.851388
#22 CACHED
2026-Mar-23 01:56:38.851388
2026-Mar-23 01:56:38.851388
#23 [worker js-builder 10/11] RUN pnpm install --frozen-lockfile
2026-Mar-23 01:56:38.851388
#23 CACHED
2026-Mar-23 01:56:38.851388
2026-Mar-23 01:56:38.851388
#24 [worker js-builder  7/11] COPY packages/ ./packages/
2026-Mar-23 01:56:38.851388
#24 CACHED
2026-Mar-23 01:56:38.851388
2026-Mar-23 01:56:38.851388
#25 [worker js-builder  6/12] COPY apps/injector/package.json ./apps/injector/
2026-Mar-23 01:56:38.851388
#25 CACHED
2026-Mar-23 01:56:38.851388
2026-Mar-23 01:56:38.851388
#26 [worker builder 2/8] WORKDIR /app
2026-Mar-23 01:56:38.851388
#26 CACHED
2026-Mar-23 01:56:38.851388
2026-Mar-23 01:56:38.851388
#27 [worker js-builder 11/11] RUN mkdir -p extractors/dist &&     npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js &&     npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-23 01:56:38.851388
#27 CACHED
2026-Mar-23 01:56:38.851388
2026-Mar-23 01:56:38.851388
#28 [worker builder  6/11] COPY crates/ ./crates/
2026-Mar-23 01:56:38.951789
#28 DONE 0.0s
2026-Mar-23 01:56:38.951789
2026-Mar-23 01:56:38.951789
#20 [api js-builder  5/12] COPY packages/api-client/package.json ./packages/api-client/
2026-Mar-23 01:56:38.951789
#20 CACHED
2026-Mar-23 01:56:38.951789
2026-Mar-23 01:56:38.951789
#29 [api js-builder  9/12] RUN pnpm install --frozen-lockfile
2026-Mar-23 01:56:38.951789
#29 CACHED
2026-Mar-23 01:56:38.951789
2026-Mar-23 01:56:38.951789
#30 [api js-builder 10/12] RUN pnpm --filter @downloadtool/injector build
2026-Mar-23 01:56:38.951789
#30 CACHED
2026-Mar-23 01:56:38.951789
2026-Mar-23 01:56:38.951789
#31 [api js-builder  8/12] COPY packages/ ./packages/
2026-Mar-23 01:56:38.951789
#31 CACHED
2026-Mar-23 01:56:38.951789
2026-Mar-23 01:56:38.951789
#32 [api js-builder 11/12] COPY extractors/ ./extractors/
2026-Mar-23 01:56:38.951789
#32 CACHED
2026-Mar-23 01:56:38.951789
2026-Mar-23 01:56:38.951789
#25 [api js-builder  6/12] COPY apps/injector/package.json ./apps/injector/
2026-Mar-23 01:56:38.951789
#25 CACHED
2026-Mar-23 01:56:38.951789
2026-Mar-23 01:56:38.951789
#19 [api js-builder  4/12] COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Mar-23 01:56:38.951789
#19 CACHED
2026-Mar-23 01:56:38.951789
2026-Mar-23 01:56:38.951789
#33 [api js-builder  7/12] COPY apps/injector/ ./apps/injector/
2026-Mar-23 01:56:38.951789
#33 CACHED
2026-Mar-23 01:56:38.951789
2026-Mar-23 01:56:38.951789
#18 [api js-builder  3/12] RUN npm install -g pnpm
2026-Mar-23 01:56:38.951789
#18 CACHED
2026-Mar-23 01:56:38.951789
2026-Mar-23 01:56:38.951789
#34 [api js-builder 12/12] RUN mkdir -p extractors/dist &&     npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js &&     npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-23 01:56:38.951789
#34 CACHED
2026-Mar-23 01:56:38.951789
2026-Mar-23 01:56:38.951789
#35 [frontend builder 3/8] COPY frontend/ ./
2026-Mar-23 01:56:38.951789
#35 CACHED
2026-Mar-23 01:56:38.951789
2026-Mar-23 01:56:38.951789
#36 [frontend runtime 3/6] COPY --from=builder /app/build ./build
2026-Mar-23 01:56:38.951789
#36 CACHED
2026-Mar-23 01:56:38.951789
2026-Mar-23 01:56:38.951789
#37 [frontend runtime 5/6] COPY --from=builder /app/package-lock.json ./
2026-Mar-23 01:56:38.951789
#37 CACHED
2026-Mar-23 01:56:38.951789
2026-Mar-23 01:56:38.951789
#38 [frontend builder 8/8] RUN node build-docker.mjs
2026-Mar-23 01:56:38.951789
#38 CACHED
2026-Mar-23 01:56:38.951789
2026-Mar-23 01:56:38.951789
#39 [frontend builder 4/8] COPY config/ /config/
2026-Mar-23 01:56:38.951789
#39 CACHED
2026-Mar-23 01:56:38.951789
2026-Mar-23 01:56:38.951789
#40 [frontend builder 7/8] RUN npm run paraglide:compile
2026-Mar-23 01:56:38.951789
#40 CACHED
2026-Mar-23 01:56:38.951789
2026-Mar-23 01:56:38.951789
#41 [frontend runtime 4/6] COPY --from=builder /app/package.json ./
2026-Mar-23 01:56:38.951789
#41 CACHED
2026-Mar-23 01:56:38.951789
2026-Mar-23 01:56:38.951789
#42 [frontend builder 6/8] RUN test -n "https://api.snapvie.com" || (echo "VITE_API_URL build arg is required" && exit 1)
2026-Mar-23 01:56:38.951789
#42 CACHED
2026-Mar-23 01:56:38.951789
2026-Mar-23 01:56:38.951789
#26 [frontend builder 2/8] WORKDIR /app
2026-Mar-23 01:56:38.951789
#26 CACHED
2026-Mar-23 01:56:38.951789
2026-Mar-23 01:56:38.951789
#43 [frontend builder 5/8] RUN npm install
2026-Mar-23 01:56:38.951789
#43 CACHED
2026-Mar-23 01:56:38.951789
2026-Mar-23 01:56:38.951789
#44 [frontend runtime 6/6] RUN npm ci --omit=dev
2026-Mar-23 01:56:38.951789
#44 CACHED
2026-Mar-23 01:56:38.951789
2026-Mar-23 01:56:38.951789
#45 [worker builder  7/11] COPY config/ ./config/
2026-Mar-23 01:56:38.951789
#45 DONE 0.0s
2026-Mar-23 01:56:38.951789
2026-Mar-23 01:56:38.951789
#46 [api builder  8/11] COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Mar-23 01:56:38.951789
#46 DONE 0.0s
2026-Mar-23 01:56:38.951789
2026-Mar-23 01:56:38.951789
#47 [frontend] exporting to image
2026-Mar-23 01:56:38.951789
#47 exporting layers done
2026-Mar-23 01:56:38.951789
#47 exporting manifest sha256:07f87ed34eb703cc583b12e79c6e3ed86f8adc4a28124050baa4ca60a1dca2d4 done
2026-Mar-23 01:56:38.951789
#47 exporting config sha256:c7dd330d38a9cb52bf25f78c0dd365b313f4ae4d3763819de3c6eaadf9964f52 done
2026-Mar-23 01:56:38.951789
#47 exporting attestation manifest sha256:b2dc3bcb35515df001a83544ba90e59ebf53a44ca431ff92d04f9e12c5db8cc7 0.0s done
2026-Mar-23 01:56:38.951789
#47 exporting manifest list sha256:9cdedd3caae5d7750f64cde8bef0dc67cf97e152e4bcc8f5c080c1bc020955d7 done
2026-Mar-23 01:56:38.951789
#47 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:3bf1238b2350ae4f3732e6368c475a81f97af531 done
2026-Mar-23 01:56:38.951789
#47 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:3bf1238b2350ae4f3732e6368c475a81f97af531 done
2026-Mar-23 01:56:38.951789
#47 DONE 0.1s
2026-Mar-23 01:56:38.951789
2026-Mar-23 01:56:38.951789
#48 [worker builder  8/10] COPY extractors/ ./extractors/
2026-Mar-23 01:56:38.951789
#48 DONE 0.0s
2026-Mar-23 01:56:38.954066
#49 [api builder  9/11] COPY extractors/ ./extractors/
2026-Mar-23 01:56:38.954066
#49 DONE 0.0s
2026-Mar-23 01:56:38.954066
2026-Mar-23 01:56:38.954066
#50 [api builder 10/11] COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-23 01:56:39.177486
#50 DONE 0.0s
2026-Mar-23 01:56:39.177486
2026-Mar-23 01:56:39.177486
#51 [worker builder  9/10] COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-23 01:56:39.177486
#51 DONE 0.0s
2026-Mar-23 01:56:39.177486
2026-Mar-23 01:56:39.177486
#52 [frontend] resolving provenance for metadata file
2026-Mar-23 01:56:39.177486
#52 DONE 0.0s
2026-Mar-23 01:56:39.177486
2026-Mar-23 01:56:39.177486
#53 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-23 01:56:39.393127
#53 0.270     Updating crates.io index
2026-Mar-23 01:56:44.257159
#53 5.284  Downloading crates ...
2026-Mar-23 01:56:44.554326
#53 5.581   Downloaded adler v1.0.2
2026-Mar-23 01:56:44.665904
#53 5.605   Downloaded alloc-no-stdlib v2.0.4
2026-Mar-23 01:56:44.665904
#53 5.646   Downloaded atomic-waker v1.1.2
2026-Mar-23 01:56:44.665904
#53 5.693   Downloaded crunchy v0.2.4
2026-Mar-23 01:56:44.778405
#53 5.702   Downloaded cfg-if v1.0.4
2026-Mar-23 01:56:44.778405
#53 5.710   Downloaded alloc-stdlib v0.2.2
2026-Mar-23 01:56:44.778405
#53 5.713   Downloaded adler2 v2.0.1
2026-Mar-23 01:56:44.778405
#53 5.738   Downloaded ahash v0.8.12
2026-Mar-23 01:56:44.778405
#53 5.750   Downloaded block-buffer v0.10.4
2026-Mar-23 01:56:44.778405
#53 5.757   Downloaded aws-smithy-observability v0.2.6
2026-Mar-23 01:56:44.778405
#53 5.761   Downloaded cooked-waker v5.0.0
2026-Mar-23 01:56:44.778405
#53 5.763   Downloaded atoi v2.0.0
2026-Mar-23 01:56:44.778405
#53 5.766   Downloaded base64-simd v0.7.0
2026-Mar-23 01:56:44.778405
#53 5.773   Downloaded aws-smithy-query v0.60.15
2026-Mar-23 01:56:44.778405
#53 5.805   Downloaded base16ct v0.1.1
2026-Mar-23 01:56:44.891799
#53 5.809   Downloaded cfg_aliases v0.2.1
2026-Mar-23 01:56:44.891799
#53 5.813   Downloaded crc-catalog v2.4.0
2026-Mar-23 01:56:44.891799
#53 5.816   Downloaded debugid v0.8.0
2026-Mar-23 01:56:44.891799
#53 5.820   Downloaded base64-simd v0.8.0
2026-Mar-23 01:56:44.891799
#53 5.823   Downloaded bit-set v0.5.3
2026-Mar-23 01:56:44.891799
#53 5.857   Downloaded cpufeatures v0.2.17
2026-Mar-23 01:56:44.891799
#53 5.862   Downloaded const-random v0.1.18
2026-Mar-23 01:56:44.891799
#53 5.865   Downloaded compression-core v0.4.31
2026-Mar-23 01:56:44.891799
#53 5.868   Downloaded async-stream-impl v0.3.6
2026-Mar-23 01:56:44.891799
#53 5.871   Downloaded async-stream v0.3.6
2026-Mar-23 01:56:44.891799
#53 5.875   Downloaded crypto-common v0.1.7
2026-Mar-23 01:56:44.891799
#53 5.878   Downloaded const-random-macro v0.1.16
2026-Mar-23 01:56:44.891799
#53 5.885   Downloaded crossbeam-queue v0.3.12
2026-Mar-23 01:56:44.891799
#53 5.889   Downloaded crc v3.4.0
2026-Mar-23 01:56:44.891799
#53 5.894   Downloaded deno_unsync v0.4.4
2026-Mar-23 01:56:44.891799
#53 5.919   Downloaded aws-smithy-checksums v0.63.12
2026-Mar-23 01:56:44.992295
#53 5.924   Downloaded data-encoding v2.10.0
2026-Mar-23 01:56:44.992295
#53 5.927   Downloaded compression-codecs v0.4.37
2026-Mar-23 01:56:44.992295
#53 5.933   Downloaded lru v0.12.5
2026-Mar-23 01:56:44.992295
#53 5.936   Downloaded axum-core v0.5.6
2026-Mar-23 01:56:44.992295
#53 5.940   Downloaded aws-smithy-http v0.62.6
2026-Mar-23 01:56:44.992295
#53 5.944   Downloaded event-listener-strategy v0.5.4
2026-Mar-23 01:56:44.992295
#53 5.947   Downloaded cexpr v0.6.0
2026-Mar-23 01:56:44.992295
#53 5.951   Downloaded aws-credential-types v1.2.14
2026-Mar-23 01:56:44.992295
#53 5.955   Downloaded aws-smithy-json v0.61.9
2026-Mar-23 01:56:44.992295
#53 5.958   Downloaded aws-smithy-eventstream v0.60.20
2026-Mar-23 01:56:44.992295
#53 5.973   Downloaded aws-smithy-xml v0.60.15
2026-Mar-23 01:56:44.992295
#53 5.977   Downloaded cmake v0.1.57
2026-Mar-23 01:56:44.992295
#53 5.981   Downloaded lru-slab v0.1.2
2026-Mar-23 01:56:44.992295
#53 5.984   Downloaded aws-smithy-http v0.63.6
2026-Mar-23 01:56:44.992295
#53 5.988   Downloaded aws-smithy-async v1.2.14
2026-Mar-23 01:56:44.992295
#53 5.993   Downloaded mime v0.3.17
2026-Mar-23 01:56:44.992295
#53 5.996   Downloaded foreign-types v0.3.2
2026-Mar-23 01:56:44.992295
#53 5.999   Downloaded errno v0.3.14
2026-Mar-23 01:56:44.992295
#53 6.003   Downloaded equivalent v1.0.2
2026-Mar-23 01:56:44.992295
#53 6.006   Downloaded byteorder v1.5.0
2026-Mar-23 01:56:44.992295
#53 6.009   Downloaded aws-types v1.3.14
2026-Mar-23 01:56:44.992295
#53 6.013   Downloaded aws-smithy-json v0.62.5
2026-Mar-23 01:56:44.992295
#53 6.016   Downloaded md-5 v0.10.6
2026-Mar-23 01:56:44.992295
#53 6.019   Downloaded matchers v0.2.0
2026-Mar-23 01:56:45.094803
#53 6.022   Downloaded async-lock v3.4.2
2026-Mar-23 01:56:45.094803
#53 6.026   Downloaded cookie_store v0.22.1
2026-Mar-23 01:56:45.094803
#53 6.031   Downloaded crypto-bigint v0.4.9
2026-Mar-23 01:56:45.094803
#53 6.040   Downloaded lock_api v0.4.14
2026-Mar-23 01:56:45.094803
#53 6.043   Downloaded no-std-compat v0.4.1
2026-Mar-23 01:56:45.094803
#53 6.045   Downloaded memoffset v0.9.1
2026-Mar-23 01:56:45.094803
#53 6.047   Downloaded litrs v1.0.0
2026-Mar-23 01:56:45.094803
#53 6.053   Downloaded litemap v0.8.1
2026-Mar-23 01:56:45.094803
#53 6.057   Downloaded dashmap v6.1.0
2026-Mar-23 01:56:45.094803
#53 6.062   Downloaded crossbeam-utils v0.8.21
2026-Mar-23 01:56:45.094803
#53 6.067   Downloaded crc32fast v1.5.0
2026-Mar-23 01:56:45.094803
#53 6.071   Downloaded cookie v0.18.1
2026-Mar-23 01:56:45.094803
#53 6.075   Downloaded bitflags v2.11.0
2026-Mar-23 01:56:45.094803
#53 6.083   Downloaded bincode v1.3.3
2026-Mar-23 01:56:45.094803
#53 6.087   Downloaded config v0.14.1
2026-Mar-23 01:56:45.094803
#53 6.096   Downloaded nonzero_ext v0.3.0
2026-Mar-23 01:56:45.094803
#53 6.104   Downloaded num-iter v0.1.45
2026-Mar-23 01:56:45.094803
#53 6.106   Downloaded futures-sink v0.3.32
2026-Mar-23 01:56:45.094803
#53 6.108   Downloaded nu-ansi-term v0.50.3
2026-Mar-23 01:56:45.094803
#53 6.113   Downloaded httpdate v1.0.3
2026-Mar-23 01:56:45.094803
#53 6.116   Downloaded crossbeam-channel v0.5.15
2026-Mar-23 01:56:45.094803
#53 6.122   Downloaded hex v0.4.3
2026-Mar-23 01:56:45.195254
#53 6.125   Downloaded funty v2.0.0
2026-Mar-23 01:56:45.195254
#53 6.127   Downloaded aws-smithy-types v1.4.6
2026-Mar-23 01:56:45.195254
#53 6.135   Downloaded crypto-bigint v0.5.5
2026-Mar-23 01:56:45.195254
#53 6.147   Downloaded openssl-macros v0.1.1
2026-Mar-23 01:56:45.195254
#53 6.150   Downloaded outref v0.5.2
2026-Mar-23 01:56:45.195254
#53 6.155   Downloaded ordered-multimap v0.7.3
2026-Mar-23 01:56:45.195254
#53 6.159   Downloaded lazycell v1.3.0
2026-Mar-23 01:56:45.195254
#53 6.162   Downloaded crc-fast v1.6.0
2026-Mar-23 01:56:45.195254
#53 6.174   Downloaded parking_lot v0.12.5
2026-Mar-23 01:56:45.195254
#53 6.178   Downloaded brotli-decompressor v5.0.0
2026-Mar-23 01:56:45.195254
#53 6.187   Downloaded heck v0.4.1
2026-Mar-23 01:56:45.195254
#53 6.189   Downloaded fslock v0.2.1
2026-Mar-23 01:56:45.195254
#53 6.193   Downloaded aws-config v1.8.15
2026-Mar-23 01:56:45.195254
#53 6.203   Downloaded percent-encoding v2.3.2
2026-Mar-23 01:56:45.195254
#53 6.206   Downloaded proc-macro-error-attr v1.0.4
2026-Mar-23 01:56:45.195254
#53 6.209   Downloaded pin-utils v0.1.0
2026-Mar-23 01:56:45.195254
#53 6.213   Downloaded proc-macro-rules-macros v0.4.0
2026-Mar-23 01:56:45.195254
#53 6.215   Downloaded proc-macro2 v1.0.106
2026-Mar-23 01:56:45.195254
#53 6.220   Downloaded proc-macro-rules v0.4.0
2026-Mar-23 01:56:45.195254
#53 6.222   Downloaded nom v7.1.3
2026-Mar-23 01:56:45.296292
#53 6.231   Downloaded mio v1.1.1
2026-Mar-23 01:56:45.296292
#53 6.241   Downloaded moka v0.12.13
2026-Mar-23 01:56:45.296292
#53 6.252   Downloaded deno_core v0.300.0
2026-Mar-23 01:56:45.296292
#53 6.266   Downloaded radium v0.7.0
2026-Mar-23 01:56:45.296292
#53 6.269   Downloaded quote v1.0.44
2026-Mar-23 01:56:45.296292
#53 6.274   Downloaded num-bigint-dig v0.8.6
2026-Mar-23 01:56:45.296292
#53 6.283   Downloaded rand_chacha v0.3.1
2026-Mar-23 01:56:45.296292
#53 6.287   Downloaded rand_core v0.9.5
2026-Mar-23 01:56:45.296292
#53 6.290   Downloaded rand_core v0.6.4
2026-Mar-23 01:56:45.296292
#53 6.292   Downloaded heck v0.5.0
2026-Mar-23 01:56:45.296292
#53 6.294   Downloaded idna_adapter v1.2.1
2026-Mar-23 01:56:45.296292
#53 6.296   Downloaded gzip-header v1.0.0
2026-Mar-23 01:56:45.296292
#53 6.298   Downloaded rand_chacha v0.9.0
2026-Mar-23 01:56:45.296292
#53 6.300   Downloaded home v0.5.12
2026-Mar-23 01:56:45.296292
#53 6.302   Downloaded futures-task v0.3.32
2026-Mar-23 01:56:45.296292
#53 6.304   Downloaded brotli v8.0.2
2026-Mar-23 01:56:45.296292
#53 6.320   Downloaded http-body v0.4.6
2026-Mar-23 01:56:45.296292
#53 6.323   Downloaded num-bigint v0.4.6
2026-Mar-23 01:56:45.407695
#53 6.330   Downloaded if_chain v1.0.3
2026-Mar-23 01:56:45.407695
#53 6.332   Downloaded quinn-udp v0.5.14
2026-Mar-23 01:56:45.407695
#53 6.335   Downloaded quanta v0.12.6
2026-Mar-23 01:56:45.407695
#53 6.339   Downloaded psl-types v2.0.11
2026-Mar-23 01:56:45.407695
#53 6.341   Downloaded ppv-lite86 v0.2.21
2026-Mar-23 01:56:45.407695
#53 6.343   Downloaded pkg-config v0.3.32
2026-Mar-23 01:56:45.407695
#53 6.346   Downloaded pkcs8 v0.10.2
2026-Mar-23 01:56:45.407695
#53 6.351   Downloaded pkcs8 v0.9.0
2026-Mar-23 01:56:45.407695
#53 6.358   Downloaded http-body v1.0.1
2026-Mar-23 01:56:45.407695
#53 6.365   Downloaded rustversion v1.0.22
2026-Mar-23 01:56:45.407695
#53 6.371   Downloaded simd-abstraction v0.7.1
2026-Mar-23 01:56:45.407695
#53 6.374   Downloaded deranged v0.5.8
2026-Mar-23 01:56:45.407695
#53 6.376   Downloaded signature v1.6.4
2026-Mar-23 01:56:45.407695
#53 6.379   Downloaded sharded-slab v0.1.7
2026-Mar-23 01:56:45.407695
#53 6.384   Downloaded parking v2.2.1
2026-Mar-23 01:56:45.407695
#53 6.386   Downloaded simple_asn1 v0.6.4
2026-Mar-23 01:56:45.407695
#53 6.388   Downloaded slab v0.4.12
2026-Mar-23 01:56:45.407695
#53 6.390   Downloaded simd-adler32 v0.3.8
2026-Mar-23 01:56:45.407695
#53 6.393   Downloaded sourcemap v8.0.1
2026-Mar-23 01:56:45.407695
#53 6.397   Downloaded dunce v1.0.5
2026-Mar-23 01:56:45.407695
#53 6.398   Downloaded document-features v0.2.12
2026-Mar-23 01:56:45.407695
#53 6.400   Downloaded signal-hook-registry v1.4.8
2026-Mar-23 01:56:45.407695
#53 6.402   Downloaded shlex v1.3.0
2026-Mar-23 01:56:45.407695
#53 6.404   Downloaded socket2 v0.6.2
2026-Mar-23 01:56:45.407695
#53 6.407   Downloaded sha2 v0.10.9
2026-Mar-23 01:56:45.407695
#53 6.411   Downloaded sha1_smol v1.0.1
2026-Mar-23 01:56:45.407695
#53 6.413   Downloaded pest v2.8.6
2026-Mar-23 01:56:45.407695
#53 6.421   Downloaded potential_utf v0.1.4
2026-Mar-23 01:56:45.407695
#53 6.423   Downloaded portable-atomic v1.13.1
2026-Mar-23 01:56:45.407695
#53 6.434   Downloaded spki v0.7.3
2026-Mar-23 01:56:45.526949
#53 6.438   Downloaded tinystr v0.8.2
2026-Mar-23 01:56:45.526949
#53 6.441   Downloaded tiny-keccak v2.0.2
2026-Mar-23 01:56:45.526949
#53 6.446   Downloaded fnv v1.0.7
2026-Mar-23 01:56:45.526949
#53 6.448   Downloaded tinyvec v1.10.0
2026-Mar-23 01:56:45.526949
#53 6.453   Downloaded tinyvec_macros v0.1.1
2026-Mar-23 01:56:45.526949
#53 6.454   Downloaded itoa v1.0.17
2026-Mar-23 01:56:45.526949
#53 6.456   Downloaded lazy_static v1.5.0
2026-Mar-23 01:56:45.526949
#53 6.459   Downloaded tokio-native-tls v0.3.1
2026-Mar-23 01:56:45.526949
#53 6.462   Downloaded tokio-rustls v0.26.4
2026-Mar-23 01:56:45.526949
#53 6.466   Downloaded tokio-rustls v0.24.1
2026-Mar-23 01:56:45.526949
#53 6.470   Downloaded fs_extra v1.3.0
2026-Mar-23 01:56:45.526949
#53 6.472   Downloaded ipnet v2.11.0
2026-Mar-23 01:56:45.526949
#53 6.475   Downloaded regex v1.12.3
2026-Mar-23 01:56:45.526949
#53 6.484   Downloaded toml_write v0.1.2
2026-Mar-23 01:56:45.526949
#53 6.487   Downloaded toml v0.8.23
2026-Mar-23 01:56:45.526949
#53 6.490   Downloaded regex-automata v0.4.14
2026-Mar-23 01:56:45.526949
#53 6.509   Downloaded redis v0.27.6
2026-Mar-23 01:56:45.526949
#53 6.521   Downloaded rsa v0.9.10
2026-Mar-23 01:56:45.526949
#53 6.530   Downloaded rustls-webpki v0.101.7
2026-Mar-23 01:56:45.526949
#53 6.554   Downloaded zerofrom v0.1.6
2026-Mar-23 01:56:45.627309
#53 6.556   Downloaded zeroize v1.8.2
2026-Mar-23 01:56:45.627309
#53 6.560   Downloaded utoipa v4.2.3
2026-Mar-23 01:56:45.627309
#53 6.564   Downloaded zerovec-derive v0.11.2
2026-Mar-23 01:56:45.627309
#53 6.567   Downloaded sqlx-mysql v0.8.6
2026-Mar-23 01:56:45.627309
#53 6.575   Downloaded sqlx v0.8.6
2026-Mar-23 01:56:45.627309
#53 6.594   Downloaded sqlx-sqlite v0.8.6
2026-Mar-23 01:56:45.627309
#53 6.600   Downloaded zmij v1.0.21
2026-Mar-23 01:56:45.627309
#53 6.603   Downloaded time v0.3.47
2026-Mar-23 01:56:45.627309
#53 6.619   Downloaded syn v2.0.117
2026-Mar-23 01:56:45.627309
#53 6.632   Downloaded syn v1.0.109
2026-Mar-23 01:56:45.627309
#53 6.644   Downloaded hyper-rustls v0.27.7
2026-Mar-23 01:56:45.627309
#53 6.647   Downloaded httparse v1.10.1
2026-Mar-23 01:56:45.627309
#53 6.651   Downloaded hmac v0.12.1
2026-Mar-23 01:56:45.627309
#53 6.654   Downloaded getrandom v0.2.17
2026-Mar-23 01:56:45.737723
#53 6.659   Downloaded rustls-native-certs v0.8.3
2026-Mar-23 01:56:45.737723
#53 6.663   Downloaded sqlx-postgres v0.8.6
2026-Mar-23 01:56:45.737723
#53 6.675   Downloaded ring v0.17.14
2026-Mar-23 01:56:45.737723
#53 6.723   Downloaded typenum v1.19.0
2026-Mar-23 01:56:45.737723
#53 6.726   Downloaded url v2.5.8
2026-Mar-23 01:56:45.737723
#53 6.729   Downloaded unicode-segmentation v1.12.0
2026-Mar-23 01:56:45.737723
#53 6.733   Downloaded unicode-normalization v0.1.25
2026-Mar-23 01:56:45.737723
#53 6.737   Downloaded winnow v0.7.14
2026-Mar-23 01:56:45.737723
#53 6.749   Downloaded spin v0.9.8
2026-Mar-23 01:56:45.737723
#53 6.752   Downloaded tracing-subscriber v0.3.22
2026-Mar-23 01:56:45.737723
#53 6.764   Downloaded webpki-roots v1.0.6
2026-Mar-23 01:56:45.838123
#53 6.769   Downloaded tokio-stream v0.1.18
2026-Mar-23 01:56:45.838123
#53 6.777   Downloaded vcpkg v0.2.15
2026-Mar-23 01:56:45.942530
#53 6.870   Downloaded yoke v0.8.1
2026-Mar-23 01:56:45.942530
#53 6.874   Downloaded tracing-attributes v0.1.31
2026-Mar-23 01:56:45.942530
#53 6.879   Downloaded zerocopy v0.8.39
2026-Mar-23 01:56:45.942530
#53 6.916   Downloaded indexmap v2.13.0
2026-Mar-23 01:56:45.942530
#53 6.923   Downloaded hyper-util v0.1.20
2026-Mar-23 01:56:45.942530
#53 6.933   Downloaded http v1.4.0
2026-Mar-23 01:56:45.942530
#53 6.938   Downloaded http v0.2.12
2026-Mar-23 01:56:45.942530
#53 6.943   Downloaded futures-intrusive v0.5.0
2026-Mar-23 01:56:45.942530
#53 6.950   Downloaded rustls-webpki v0.103.9
2026-Mar-23 01:56:45.942530
#53 6.954   Downloaded regex-lite v0.1.9
2026-Mar-23 01:56:45.942530
#53 6.958   Downloaded itertools v0.13.0
2026-Mar-23 01:56:45.942530
#53 6.969   Downloaded sqlx-core v0.8.6
2026-Mar-23 01:56:46.043435
#53 6.982   Downloaded tower v0.5.3
2026-Mar-23 01:56:46.043435
#53 6.998   Downloaded futures-util v0.3.32
2026-Mar-23 01:56:46.043435
#53 7.021   Downloaded tower-http v0.6.8
2026-Mar-23 01:56:46.043435
#53 7.033   Downloaded hyper v0.14.32
2026-Mar-23 01:56:46.043435
#53 7.044   Downloaded h2 v0.4.13
2026-Mar-23 01:56:46.043435
#53 7.054   Downloaded tower v0.4.13
2026-Mar-23 01:56:46.043435
#53 7.070   Downloaded libm v0.2.16
2026-Mar-23 01:56:46.145684
#53 7.089   Downloaded yaml-rust2 v0.8.1
2026-Mar-23 01:56:46.145684
#53 7.137   Downloaded zerovec v0.11.5
2026-Mar-23 01:56:46.145684
#53 7.144   Downloaded itertools v0.12.1
2026-Mar-23 01:56:46.145684
#53 7.154   Downloaded idna v1.1.0
2026-Mar-23 01:56:46.145684
#53 7.158   Downloaded icu_properties_data v2.1.2
2026-Mar-23 01:56:46.251568
#53 7.174   Downloaded regex-syntax v0.8.10
2026-Mar-23 01:56:46.251568
#53 7.183   Downloaded hyper v1.8.1
2026-Mar-23 01:56:46.251568
#53 7.191   Downloaded hkdf v0.12.4
2026-Mar-23 01:56:46.251568
#53 7.194   Downloaded hashbrown v0.16.1
2026-Mar-23 01:56:46.251568
#53 7.201   Downloaded h2 v0.3.27
2026-Mar-23 01:56:46.251568
#53 7.210   Downloaded hashbrown v0.15.5
2026-Mar-23 01:56:46.251568
#53 7.218   Downloaded tracing v0.1.44
2026-Mar-23 01:56:46.251568
#53 7.238   Downloaded hashbrown v0.14.5
2026-Mar-23 01:56:46.251568
#53 7.245   Downloaded governor v0.8.1
2026-Mar-23 01:56:46.251568
#53 7.251   Downloaded iri-string v0.7.10
2026-Mar-23 01:56:46.251568
#53 7.263   Downloaded prettyplease v0.2.37
2026-Mar-23 01:56:46.251568
#53 7.268   Downloaded icu_collections v2.1.1
2026-Mar-23 01:56:46.251568
#53 7.278   Downloaded flate2 v1.1.9
2026-Mar-23 01:56:46.353646
#53 7.286   Downloaded xmlparser v0.13.6
2026-Mar-23 01:56:46.353646
#53 7.290   Downloaded writeable v0.6.2
2026-Mar-23 01:56:46.353646
#53 7.293   Downloaded unicode-id-start v1.4.0
2026-Mar-23 01:56:46.353646
#53 7.297   Downloaded spinning_top v0.3.0
2026-Mar-23 01:56:46.353646
#53 7.299   Downloaded socket2 v0.5.10
2026-Mar-23 01:56:46.353646
#53 7.302   Downloaded smallvec v1.15.1
2026-Mar-23 01:56:46.353646
#53 7.305   Downloaded pest_meta v2.8.6
2026-Mar-23 01:56:46.353646
#53 7.308   Downloaded libc v0.2.182
2026-Mar-23 01:56:46.353646
#53 7.359   Downloaded icu_normalizer_data v2.1.1
2026-Mar-23 01:56:46.353646
#53 7.361   Downloaded icu_locale_core v2.1.1
2026-Mar-23 01:56:46.353646
#53 7.370   Downloaded icu_normalizer v2.1.1
2026-Mar-23 01:56:46.353646
#53 7.375   Downloaded flume v0.11.1
2026-Mar-23 01:56:46.353646
#53 7.381   Downloaded rustls-pki-types v1.14.0
2026-Mar-23 01:56:46.505275
#53 7.386   Downloaded pin-project v1.1.10
2026-Mar-23 01:56:46.505275
#53 7.406   Downloaded num-traits v0.2.19
2026-Mar-23 01:56:46.505275
#53 7.409   Downloaded getrandom v0.3.4
2026-Mar-23 01:56:46.505275
#53 7.415   Downloaded utoipa-gen v4.3.1
2026-Mar-23 01:56:46.505275
#53 7.423   Downloaded icu_provider v2.1.1
2026-Mar-23 01:56:46.505275
#53 7.426   Downloaded getrandom v0.4.1
2026-Mar-23 01:56:46.505275
#53 7.431   Downloaded jsonwebtoken v9.3.1
2026-Mar-23 01:56:46.505275
#53 7.437   Downloaded icu_properties v2.1.2
2026-Mar-23 01:56:46.505275
#53 7.440   Downloaded generic-array v0.14.7
2026-Mar-23 01:56:46.505275
#53 7.442   Downloaded elliptic-curve v0.12.3
2026-Mar-23 01:56:46.505275
#53 7.447   Downloaded futures v0.3.32
2026-Mar-23 01:56:46.505275
#53 7.454   Downloaded tokio-util v0.7.18
2026-Mar-23 01:56:46.505275
#53 7.465   Downloaded event-listener v5.4.1
2026-Mar-23 01:56:46.505275
#53 7.467   Downloaded tower-service v0.3.3
2026-Mar-23 01:56:46.505275
#53 7.468   Downloaded tower-layer v0.3.3
2026-Mar-23 01:56:46.505275
#53 7.470   Downloaded toml_edit v0.22.27
2026-Mar-23 01:56:46.505275
#53 7.476   Downloaded tokio v1.49.0
2026-Mar-23 01:56:46.505275
#53 7.532   Downloaded aws-lc-sys v0.38.0
2026-Mar-23 01:56:46.857524
#53 7.838   Downloaded futures-channel v0.3.32
2026-Mar-23 01:56:46.857524
#53 7.840   Downloaded serde_json v1.0.149
2026-Mar-23 01:56:46.857524
#53 7.849   Downloaded unicode-ident v1.0.24
2026-Mar-23 01:56:46.857524
#53 7.852   Downloaded unicode-bidi v0.3.18
2026-Mar-23 01:56:46.857524
#53 7.856   Downloaded tracing-core v0.1.36
2026-Mar-23 01:56:46.857524
#53 7.859   Downloaded signature v2.2.0
2026-Mar-23 01:56:46.857524
#53 7.861   Downloaded zerofrom-derive v0.1.6
2026-Mar-23 01:56:46.857524
#53 7.862   Downloaded wyz v0.5.1
2026-Mar-23 01:56:46.857524
#53 7.864   Downloaded which v6.0.3
2026-Mar-23 01:56:46.857524
#53 7.866   Downloaded uuid v1.21.0
2026-Mar-23 01:56:46.857524
#53 7.871   Downloaded untrusted v0.9.0
2026-Mar-23 01:56:46.857524
#53 7.873   Downloaded unicode-properties v0.1.4
2026-Mar-23 01:56:46.857524
#53 7.876   Downloaded ucd-trie v0.1.7
2026-Mar-23 01:56:46.857524
#53 7.877   Downloaded rustc_version v0.2.3
2026-Mar-23 01:56:46.857524
#53 7.878   Downloaded rustc-hash v2.1.1
2026-Mar-23 01:56:46.857524
#53 7.880   Downloaded proc-macro-error v1.0.4
2026-Mar-23 01:56:46.857524
#53 7.884   Downloaded glob v0.3.3
2026-Mar-23 01:56:46.957686
#53 7.886   Downloaded futures-executor v0.3.32
2026-Mar-23 01:56:46.957686
#53 7.888   Downloaded dotenvy v0.15.7
2026-Mar-23 01:56:46.957686
#53 7.892   Downloaded displaydoc v0.2.5
2026-Mar-23 01:56:46.957686
#53 7.896   Downloaded digest v0.10.7
2026-Mar-23 01:56:46.957686
#53 7.898   Downloaded yoke-derive v0.8.1
2026-Mar-23 01:56:46.957686
#53 7.899   Downloaded whoami v1.6.1
2026-Mar-23 01:56:46.957686
#53 7.902   Downloaded which v4.4.2
2026-Mar-23 01:56:46.957686
#53 7.903   Downloaded webpki-roots v0.26.11
2026-Mar-23 01:56:46.957686
#53 7.905   Downloaded web-time v1.1.0
2026-Mar-23 01:56:46.957686
#53 7.907   Downloaded encoding_rs v0.8.35
2026-Mar-23 01:56:46.957686
#53 7.927   Downloaded want v0.3.1
2026-Mar-23 01:56:46.957686
#53 7.928   Downloaded vsimd v0.8.0
2026-Mar-23 01:56:46.957686
#53 7.931   Downloaded version_check v0.9.5
2026-Mar-23 01:56:46.957686
#53 7.932   Downloaded utf8_iter v1.0.4
2026-Mar-23 01:56:46.957686
#53 7.933   Downloaded urlencoding v2.1.3
2026-Mar-23 01:56:46.957686
#53 7.935   Downloaded try-lock v0.2.5
2026-Mar-23 01:56:46.957686
#53 7.936   Downloaded tracing-log v0.2.0
2026-Mar-23 01:56:46.957686
#53 7.938   Downloaded serde v1.0.228
2026-Mar-23 01:56:46.957686
#53 7.942   Downloaded rustls v0.23.37
2026-Mar-23 01:56:46.957686
#53 7.956   Downloaded rustix v0.38.44
2026-Mar-23 01:56:47.057997
#53 7.995   Downloaded hyper-rustls v0.24.2
2026-Mar-23 01:56:47.057997
#53 8.000   Downloaded http-body-util v0.1.3
2026-Mar-23 01:56:47.057997
#53 8.002   Downloaded rustls v0.21.12
2026-Mar-23 01:56:47.057997
#53 8.018   Downloaded linux-raw-sys v0.4.15
2026-Mar-23 01:56:47.057997
#53 8.075   Downloaded reqwest v0.12.28
2026-Mar-23 01:56:47.057997
#53 8.081   Downloaded libloading v0.8.9
2026-Mar-23 01:56:47.057997
#53 8.084   Downloaded toml_datetime v0.6.11
2026-Mar-23 01:56:47.057997
#53 8.085   Downloaded raw-cpuid v11.6.0
2026-Mar-23 01:56:47.162848
#53 8.089   Downloaded hashlink v0.8.4
2026-Mar-23 01:56:47.162848
#53 8.092   Downloaded deno_core_icudata v0.0.73
2026-Mar-23 01:56:47.162848
#53 8.128   Downloaded tokio-macros v2.6.0
2026-Mar-23 01:56:47.162848
#53 8.129   Downloaded jobserver v0.1.34
2026-Mar-23 01:56:47.162848
#53 8.131   Downloaded hashlink v0.10.0
2026-Mar-23 01:56:47.162848
#53 8.134   Downloaded group v0.12.1
2026-Mar-23 01:56:47.162848
#53 8.136   Downloaded futures-timer v3.0.3
2026-Mar-23 01:56:47.162848
#53 8.138   Downloaded quinn-proto v0.11.13
2026-Mar-23 01:56:47.162848
#53 8.146   Downloaded foldhash v0.1.5
2026-Mar-23 01:56:47.162848
#53 8.148   Downloaded find-msvc-tools v0.1.9
2026-Mar-23 01:56:47.162848
#53 8.150   Downloaded rand v0.9.2
2026-Mar-23 01:56:47.162848
#53 8.154   Downloaded rand v0.8.5
2026-Mar-23 01:56:47.162848
#53 8.159   Downloaded quinn v0.11.9
2026-Mar-23 01:56:47.162848
#53 8.162   Downloaded publicsuffix v2.3.0
2026-Mar-23 01:56:47.162848
#53 8.165   Downloaded either v1.15.0
2026-Mar-23 01:56:47.162848
#53 8.167   Downloaded ecdsa v0.14.8
2026-Mar-23 01:56:47.162848
#53 8.169   Downloaded dlv-list v0.5.2
2026-Mar-23 01:56:47.162848
#53 8.171   Downloaded time-macros v0.2.27
2026-Mar-23 01:56:47.162848
#53 8.174   Downloaded time-core v0.1.8
2026-Mar-23 01:56:47.162848
#53 8.175   Downloaded thread_local v1.1.9
2026-Mar-23 01:56:47.162848
#53 8.177   Downloaded thiserror-impl v2.0.18
2026-Mar-23 01:56:47.162848
#53 8.179   Downloaded thiserror-impl v1.0.69
2026-Mar-23 01:56:47.162848
#53 8.181   Downloaded thiserror v2.0.18
2026-Mar-23 01:56:47.162848
#53 8.190   Downloaded thiserror v1.0.69
2026-Mar-23 01:56:47.345236
#53 8.197   Downloaded tap v1.0.1
2026-Mar-23 01:56:47.345236
#53 8.198   Downloaded tagptr v0.2.0
2026-Mar-23 01:56:47.345236
#53 8.200   Downloaded synstructure v0.13.2
2026-Mar-23 01:56:47.345236
#53 8.201   Downloaded sync_wrapper v1.0.2
2026-Mar-23 01:56:47.345236
#53 8.202   Downloaded subtle v2.6.1
2026-Mar-23 01:56:47.345236
#53 8.204   Downloaded strum_macros v0.25.3
2026-Mar-23 01:56:47.345236
#53 8.206   Downloaded strum v0.25.0
2026-Mar-23 01:56:47.345236
#53 8.207   Downloaded stringprep v0.1.5
2026-Mar-23 01:56:47.345236
#53 8.209   Downloaded static_assertions v1.1.0
2026-Mar-23 01:56:47.345236
#53 8.211   Downloaded stable_deref_trait v1.2.1
2026-Mar-23 01:56:47.345236
#53 8.212   Downloaded sqlx-macros-core v0.8.6
2026-Mar-23 01:56:47.345236
#53 8.215   Downloaded sqlx-macros v0.8.6
2026-Mar-23 01:56:47.345236
#53 8.216   Downloaded spki v0.6.0
2026-Mar-23 01:56:47.345236
#53 8.218   Downloaded sha1 v0.10.6
2026-Mar-23 01:56:47.345236
#53 8.220   Downloaded serde_v8 v0.209.0
2026-Mar-23 01:56:47.345236
#53 8.223   Downloaded serde_urlencoded v0.7.1
2026-Mar-23 01:56:47.345236
#53 8.226   Downloaded serde_spanned v0.6.9
2026-Mar-23 01:56:47.345236
#53 8.227   Downloaded serde_path_to_error v0.1.20
2026-Mar-23 01:56:47.345236
#53 8.229   Downloaded serde_derive v1.0.228
2026-Mar-23 01:56:47.345236
#53 8.233   Downloaded serde_core v1.0.228
2026-Mar-23 01:56:47.345236
#53 8.237   Downloaded semver-parser v0.7.0
2026-Mar-23 01:56:47.345236
#53 8.238   Downloaded semver v1.0.27
2026-Mar-23 01:56:47.345236
#53 8.241   Downloaded semver v0.9.0
2026-Mar-23 01:56:47.345236
#53 8.243   Downloaded sec1 v0.3.0
2026-Mar-23 01:56:47.345236
#53 8.245   Downloaded sct v0.7.1
2026-Mar-23 01:56:47.345236
#53 8.252   Downloaded scopeguard v1.2.0
2026-Mar-23 01:56:47.345236
#53 8.254   Downloaded ryu v1.0.23
2026-Mar-23 01:56:47.345236
#53 8.259   Downloaded p256 v0.11.1
2026-Mar-23 01:56:47.345236
#53 8.264   Downloaded openssl-sys v0.9.111
2026-Mar-23 01:56:47.345236
#53 8.271   Downloaded aws-sdk-s3 v1.119.0
2026-Mar-23 01:56:47.449288
#53 8.416   Downloaded rustc_version v0.4.1
2026-Mar-23 01:56:47.449288
#53 8.418   Downloaded rustc-hash v1.1.0
2026-Mar-23 01:56:47.449288
#53 8.419   Downloaded zerotrie v0.2.3
2026-Mar-23 01:56:47.449288
#53 8.423   Downloaded rust-ini v0.20.0
2026-Mar-23 01:56:47.449288
#53 8.425   Downloaded ron v0.8.1
2026-Mar-23 01:56:47.449288
#53 8.434   Downloaded rfc6979 v0.3.1
2026-Mar-23 01:56:47.449288
#53 8.436   Downloaded openssl v0.10.75
2026-Mar-23 01:56:47.449288
#53 8.449   Downloaded json5 v0.4.1
2026-Mar-23 01:56:47.449288
#53 8.451   Downloaded pkcs1 v0.7.5
2026-Mar-23 01:56:47.449288
#53 8.455   Downloaded pin-project-lite v0.2.16
2026-Mar-23 01:56:47.449288
#53 8.462   Downloaded pin-project-internal v1.1.10
2026-Mar-23 01:56:47.449288
#53 8.464   Downloaded pest_generator v2.8.6
2026-Mar-23 01:56:47.449288
#53 8.466   Downloaded pest_derive v2.8.6
2026-Mar-23 01:56:47.449288
#53 8.469   Downloaded parking_lot_core v0.9.12
2026-Mar-23 01:56:47.449288
#53 8.471   Downloaded bitvec v1.0.1
2026-Mar-23 01:56:47.551075
#53 8.497   Downloaded bindgen v0.69.5
2026-Mar-23 01:56:47.551075
#53 8.506   Downloaded aws-lc-rs v1.16.1
2026-Mar-23 01:56:47.551075
#53 8.518   Downloaded powerfmt v0.2.0
2026-Mar-23 01:56:47.551075
#53 8.520   Downloaded pem-rfc7468 v0.7.0
2026-Mar-23 01:56:47.551075
#53 8.522   Downloaded pem v3.0.6
2026-Mar-23 01:56:47.551075
#53 8.524   Downloaded paste v1.0.15
2026-Mar-23 01:56:47.551075
#53 8.528   Downloaded aws-sdk-sts v1.100.0
2026-Mar-23 01:56:47.551075
#53 8.544   Downloaded pathdiff v0.2.3
2026-Mar-23 01:56:47.551075
#53 8.545   Downloaded miniz_oxide v0.8.9
2026-Mar-23 01:56:47.551075
#53 8.548   Downloaded futures-core v0.3.32
2026-Mar-23 01:56:47.551075
#53 8.550   Downloaded minimal-lexical v0.2.1
2026-Mar-23 01:56:47.551075
#53 8.556   Downloaded hyper-tls v0.6.0
2026-Mar-23 01:56:47.551075
#53 8.558   Downloaded foreign-types-shared v0.1.1
2026-Mar-23 01:56:47.551075
#53 8.559   Downloaded axum v0.8.8
2026-Mar-23 01:56:47.551075
#53 8.571   Downloaded aho-corasick v1.1.4
2026-Mar-23 01:56:47.551075
#53 8.578   Downloaded combine v4.6.7
2026-Mar-23 01:56:47.651740
#53 8.587   Downloaded aws-smithy-runtime v1.10.3
2026-Mar-23 01:56:47.651740
#53 8.595   Downloaded once_cell v1.21.3
2026-Mar-23 01:56:47.651740
#53 8.598   Downloaded memchr v2.8.0
2026-Mar-23 01:56:47.651740
#53 8.606   Downloaded async-compression v0.4.41
2026-Mar-23 01:56:47.651740
#53 8.617   Downloaded outref v0.1.0
2026-Mar-23 01:56:47.651740
#53 8.618   Downloaded openssl-probe v0.2.1
2026-Mar-23 01:56:47.651740
#53 8.620   Downloaded miniz_oxide v0.7.4
2026-Mar-23 01:56:47.651740
#53 8.623   Downloaded der v0.7.10
2026-Mar-23 01:56:47.651740
#53 8.631   Downloaded cc v1.2.56
2026-Mar-23 01:56:47.651740
#53 8.635   Downloaded base64 v0.21.7
2026-Mar-23 01:56:47.651740
#53 8.641   Downloaded aws-sdk-ssooidc v1.98.0
2026-Mar-23 01:56:47.651740
#53 8.654   Downloaded aws-sdk-sso v1.96.0
2026-Mar-23 01:56:47.651740
#53 8.664   Downloaded aws-runtime v1.7.2
2026-Mar-23 01:56:47.651740
#53 8.670   Downloaded num_cpus v1.17.0
2026-Mar-23 01:56:47.651740
#53 8.674   Downloaded num-integer v0.1.46
2026-Mar-23 01:56:47.651740
#53 8.676   Downloaded crossbeam-epoch v0.9.18
2026-Mar-23 01:56:47.651740
#53 8.679   Downloaded aws-sigv4 v1.4.2
2026-Mar-23 01:56:47.752581
#53 8.748   Downloaded aws-smithy-runtime-api v1.11.6
2026-Mar-23 01:56:47.752581
#53 8.753   Downloaded aws-smithy-http-client v1.1.12
2026-Mar-23 01:56:47.752581
#53 8.758   Downloaded num-conv v0.2.0
2026-Mar-23 01:56:47.752581
#53 8.759   Downloaded log v0.4.29
2026-Mar-23 01:56:47.752581
#53 8.762   Downloaded futures-macro v0.3.32
2026-Mar-23 01:56:47.752581
#53 8.763   Downloaded der v0.6.1
2026-Mar-23 01:56:47.752581
#53 8.771   Downloaded clang-sys v1.8.1
2026-Mar-23 01:56:47.752581
#53 8.774   Downloaded bytes v1.11.1
2026-Mar-23 01:56:47.752581
#53 8.779   Downloaded base64 v0.22.1
2026-Mar-23 01:56:47.887990
#53 8.785   Downloaded native-tls v0.2.18
2026-Mar-23 01:56:47.887990
#53 8.787   Downloaded matchit v0.8.4
2026-Mar-23 01:56:47.887990
#53 8.789   Downloaded futures-io v0.3.32
2026-Mar-23 01:56:47.887990
#53 8.791   Downloaded form_urlencoded v1.2.2
2026-Mar-23 01:56:47.887990
#53 8.792   Downloaded deno_ops v0.176.0
2026-Mar-23 01:56:47.887990
#53 8.803   Downloaded const-oid v0.9.6
2026-Mar-23 01:56:47.887990
#53 8.806   Downloaded arc-swap v1.8.2
2026-Mar-23 01:56:47.887990
#53 8.812   Downloaded allocator-api2 v0.2.21
2026-Mar-23 01:56:47.887990
#53 8.816   Downloaded base64ct v1.8.3
2026-Mar-23 01:56:47.887990
#53 8.820   Downloaded anyhow v1.0.102
2026-Mar-23 01:56:47.887990
#53 8.826   Downloaded ff v0.12.1
2026-Mar-23 01:56:47.887990
#53 8.828   Downloaded fastrand v2.3.0
2026-Mar-23 01:56:47.887990
#53 8.830   Downloaded convert_case v0.6.0
2026-Mar-23 01:56:47.887990
#53 8.832   Downloaded concurrent-queue v2.5.0
2026-Mar-23 01:56:47.887990
#53 8.834   Downloaded async-trait v0.1.89
2026-Mar-23 01:56:47.887990
#53 8.840   Downloaded arraydeque v0.5.1
2026-Mar-23 01:56:47.887990
#53 8.842   Downloaded bytes-utils v0.1.4
2026-Mar-23 01:56:47.887990
#53 8.844   Downloaded bit-vec v0.6.3
2026-Mar-23 01:56:47.887990
#53 8.846   Downloaded autocfg v1.5.0
2026-Mar-23 01:56:47.887990
#53 8.851   Downloaded libsqlite3-sys v0.30.1
2026-Mar-23 01:56:47.994881
#53 9.022   Downloaded v8 v0.101.0
2026-Mar-23 01:56:48.963336
#53 ...
2026-Mar-23 01:56:48.963336
2026-Mar-23 01:56:48.963336
#54 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-23 01:56:48.963336
#54 0.284     Updating crates.io index
2026-Mar-23 01:56:48.963336
#54 6.290  Downloading crates ...
2026-Mar-23 01:56:48.963336
#54 6.521   Downloaded adler v1.0.2
2026-Mar-23 01:56:48.963336
#54 6.531   Downloaded adler2 v2.0.1
2026-Mar-23 01:56:48.963336
#54 6.596   Downloaded alloc-stdlib v0.2.2
2026-Mar-23 01:56:48.963336
#54 6.715   Downloaded compression-core v0.4.31
2026-Mar-23 01:56:48.963336
#54 6.764   Downloaded crypto-common v0.1.7
2026-Mar-23 01:56:48.963336
#54 6.767   Downloaded const-random-macro v0.1.16
2026-Mar-23 01:56:48.963336
#54 6.770   Downloaded cfg_aliases v0.2.1
2026-Mar-23 01:56:48.963336
#54 6.788   Downloaded async-stream v0.3.6
2026-Mar-23 01:56:48.963336
#54 6.804   Downloaded digest v0.10.7
2026-Mar-23 01:56:48.963336
#54 6.816   Downloaded cooked-waker v5.0.0
2026-Mar-23 01:56:48.963336
#54 6.820   Downloaded crc v3.4.0
2026-Mar-23 01:56:48.963336
#54 6.824   Downloaded crossbeam-queue v0.3.12
2026-Mar-23 01:56:48.963336
#54 6.827   Downloaded async-stream-impl v0.3.6
2026-Mar-23 01:56:48.963336
#54 6.830   Downloaded block-buffer v0.10.4
2026-Mar-23 01:56:48.963336
#54 6.833   Downloaded const-random v0.1.18
2026-Mar-23 01:56:48.963336
#54 6.836   Downloaded cfg-if v1.0.4
2026-Mar-23 01:56:48.963336
#54 6.848   Downloaded crunchy v0.2.4
2026-Mar-23 01:56:48.963336
#54 6.877   Downloaded base64-simd v0.7.0
2026-Mar-23 01:56:48.963336
#54 6.881   Downloaded atomic-waker v1.1.2
2026-Mar-23 01:56:48.963336
#54 6.884   Downloaded base16ct v0.1.1
2026-Mar-23 01:56:48.963336
#54 6.887   Downloaded dunce v1.0.5
2026-Mar-23 01:56:48.963336
#54 6.891   Downloaded cpufeatures v0.2.17
2026-Mar-23 01:56:48.963336
#54 6.904   Downloaded fnv v1.0.7
2026-Mar-23 01:56:48.963336
#54 6.906   Downloaded dotenvy v0.15.7
2026-Mar-23 01:56:48.963336
#54 6.914   Downloaded funty v2.0.0
2026-Mar-23 01:56:48.963336
#54 6.930   Downloaded futures-core v0.3.32
2026-Mar-23 01:56:48.963336
#54 6.936   Downloaded aws-smithy-observability v0.2.6
2026-Mar-23 01:56:48.963336
#54 6.940   Downloaded debugid v0.8.0
2026-Mar-23 01:56:48.963336
#54 6.945   Downloaded base64-simd v0.8.0
2026-Mar-23 01:56:48.963336
#54 6.949   Downloaded cexpr v0.6.0
2026-Mar-23 01:56:48.963336
#54 6.954   Downloaded crc-catalog v2.4.0
2026-Mar-23 01:56:48.963336
#54 6.958   Downloaded aws-smithy-checksums v0.63.12
2026-Mar-23 01:56:48.963336
#54 6.962   Downloaded aws-smithy-async v1.2.14
2026-Mar-23 01:56:48.963336
#54 6.967   Downloaded bit-set v0.5.3
2026-Mar-23 01:56:48.963336
#54 6.970   Downloaded alloc-no-stdlib v2.0.4
2026-Mar-23 01:56:48.963336
#54 6.975   Downloaded aws-smithy-query v0.60.15
2026-Mar-23 01:56:48.963336
#54 6.978   Downloaded atoi v2.0.0
2026-Mar-23 01:56:48.963336
#54 6.996   Downloaded futures-task v0.3.32
2026-Mar-23 01:56:48.963336
#54 6.999   Downloaded foreign-types v0.3.2
2026-Mar-23 01:56:48.963336
#54 7.003   Downloaded byteorder v1.5.0
2026-Mar-23 01:56:48.963336
#54 7.007   Downloaded concurrent-queue v2.5.0
2026-Mar-23 01:56:48.963336
#54 7.012   Downloaded compression-codecs v0.4.37
2026-Mar-23 01:56:48.963336
#54 7.020   Downloaded ff v0.12.1
2026-Mar-23 01:56:48.963336
#54 7.023   Downloaded data-encoding v2.10.0
2026-Mar-23 01:56:48.963336
#54 7.025   Downloaded convert_case v0.6.0
2026-Mar-23 01:56:48.963336
#54 7.028   Downloaded axum-core v0.5.6
2026-Mar-23 01:56:48.963336
#54 7.033   Downloaded dashmap v6.1.0
2026-Mar-23 01:56:48.963336
#54 7.037   Downloaded aws-smithy-xml v0.60.15
2026-Mar-23 01:56:48.963336
#54 7.040   Downloaded cmake v0.1.57
2026-Mar-23 01:56:48.963336
#54 7.043   Downloaded aws-smithy-json v0.61.9
2026-Mar-23 01:56:48.963336
#54 7.046   Downloaded bytes-utils v0.1.4
2026-Mar-23 01:56:48.963336
#54 7.049   Downloaded aws-credential-types v1.2.14
2026-Mar-23 01:56:48.963336
#54 7.054   Downloaded arraydeque v0.5.1
2026-Mar-23 01:56:48.963336
#54 7.058   Downloaded crc32fast v1.5.0
2026-Mar-23 01:56:48.963336
#54 7.063   Downloaded lru v0.12.5
2026-Mar-23 01:56:48.963336
#54 7.067   Downloaded memoffset v0.9.1
2026-Mar-23 01:56:48.963336
#54 7.071   Downloaded fastrand v2.3.0
2026-Mar-23 01:56:48.963336
#54 7.075   Downloaded httpdate v1.0.3
2026-Mar-23 01:56:48.963336
#54 7.078   Downloaded errno v0.3.14
2026-Mar-23 01:56:48.963336
#54 7.082   Downloaded cookie v0.18.1
2026-Mar-23 01:56:48.963336
#54 7.087   Downloaded mime v0.3.17
2026-Mar-23 01:56:48.963336
#54 7.091   Downloaded allocator-api2 v0.2.21
2026-Mar-23 01:56:48.963336
#54 7.095   Downloaded itoa v1.0.17
2026-Mar-23 01:56:48.963336
#54 7.099   Downloaded lock_api v0.4.14
2026-Mar-23 01:56:48.963336
#54 7.102   Downloaded const-oid v0.9.6
2026-Mar-23 01:56:48.963336
#54 7.106   Downloaded aws-smithy-http v0.63.6
2026-Mar-23 01:56:48.963336
#54 7.111   Downloaded crypto-bigint v0.4.9
2026-Mar-23 01:56:48.963336
#54 7.120   Downloaded litrs v1.0.0
2026-Mar-23 01:56:48.963336
#54 7.127   Downloaded cookie_store v0.22.1
2026-Mar-23 01:56:48.963336
#54 7.132   Downloaded crossbeam-epoch v0.9.18
2026-Mar-23 01:56:48.963336
#54 7.137   Downloaded config v0.14.1
2026-Mar-23 01:56:48.963336
#54 7.147   Downloaded num-conv v0.2.0
2026-Mar-23 01:56:48.963336
#54 7.150   Downloaded deno_ops v0.176.0
2026-Mar-23 01:56:48.963336
#54 7.166   Downloaded lazy_static v1.5.0
2026-Mar-23 01:56:48.963336
#54 7.170   Downloaded event-listener-strategy v0.5.4
2026-Mar-23 01:56:48.963336
#54 7.173   Downloaded http-body v0.4.6
2026-Mar-23 01:56:48.963336
#54 7.177   Downloaded der v0.6.1
2026-Mar-23 01:56:48.963336
#54 7.187   Downloaded openssl-probe v0.2.1
2026-Mar-23 01:56:48.963336
#54 7.191   Downloaded parking v2.2.1
2026-Mar-23 01:56:48.963336
#54 7.195   Downloaded percent-encoding v2.3.2
2026-Mar-23 01:56:48.963336
#54 7.198   Downloaded async-compression v0.4.41
2026-Mar-23 01:56:48.963336
#54 7.211   Downloaded pem v3.0.6
2026-Mar-23 01:56:48.963336
#54 7.214   Downloaded parking_lot_core v0.9.12
2026-Mar-23 01:56:48.963336
#54 7.219   Downloaded miniz_oxide v0.8.9
2026-Mar-23 01:56:48.963336
#54 7.224   Downloaded heck v0.5.0
2026-Mar-23 01:56:48.963336
#54 7.227   Downloaded pem-rfc7468 v0.7.0
2026-Mar-23 01:56:48.963336
#54 7.232   Downloaded hyper-tls v0.6.0
2026-Mar-23 01:56:48.963336
#54 7.235   Downloaded json5 v0.4.1
2026-Mar-23 01:56:48.963336
#54 7.239   Downloaded combine v4.6.7
2026-Mar-23 01:56:48.963336
#54 7.248   Downloaded pin-utils v0.1.0
2026-Mar-23 01:56:48.963336
#54 7.252   Downloaded pin-project-internal v1.1.10
2026-Mar-23 01:56:48.963336
#54 7.256   Downloaded pin-project-lite v0.2.16
2026-Mar-23 01:56:48.963336
#54 7.266   Downloaded pkcs8 v0.10.2
2026-Mar-23 01:56:48.963336
#54 7.273   Downloaded pkg-config v0.3.32
2026-Mar-23 01:56:48.963336
#54 7.276   Downloaded psl-types v2.0.11
2026-Mar-23 01:56:48.963336
#54 7.278   Downloaded proc-macro-rules v0.4.0
2026-Mar-23 01:56:48.963336
#54 7.281   Downloaded proc-macro-error-attr v1.0.4
2026-Mar-23 01:56:48.963336
#54 7.283   Downloaded powerfmt v0.2.0
2026-Mar-23 01:56:48.963336
#54 7.285   Downloaded deno_core v0.300.0
2026-Mar-23 01:56:48.963336
#54 7.300   Downloaded proc-macro-error v1.0.4
2026-Mar-23 01:56:48.963336
#54 7.306   Downloaded bitvec v1.0.1
2026-Mar-23 01:56:48.963336
#54 7.338   Downloaded rand_chacha v0.9.0
2026-Mar-23 01:56:48.963336
#54 7.342   Downloaded rand_core v0.6.4
2026-Mar-23 01:56:48.963336
#54 7.345   Downloaded rustc_version v0.4.1
2026-Mar-23 01:56:48.963336
#54 7.347   Downloaded either v1.15.0
2026-Mar-23 01:56:48.967622
#54 7.349   Downloaded rustc-hash v2.1.1
2026-Mar-23 01:56:48.967622
#54 7.351   Downloaded rustc-hash v1.1.0
2026-Mar-23 01:56:48.967622
#54 7.353   Downloaded rfc6979 v0.3.1
2026-Mar-23 01:56:48.967622
#54 7.354   Downloaded http-body v1.0.1
2026-Mar-23 01:56:48.967622
#54 7.357   Downloaded openssl-sys v0.9.111
2026-Mar-23 01:56:48.967622
#54 7.365   Downloaded rustc_version v0.2.3
2026-Mar-23 01:56:48.967622
#54 7.366   Downloaded openssl v0.10.75
2026-Mar-23 01:56:48.967622
#54 7.378   Downloaded rustversion v1.0.22
2026-Mar-23 01:56:48.967622
#54 7.383   Downloaded no-std-compat v0.4.1
2026-Mar-23 01:56:48.967622
#54 7.384   Downloaded brotli v8.0.2
2026-Mar-23 01:56:48.967622
#54 7.404   Downloaded http-body-util v0.1.3
2026-Mar-23 01:56:48.967622
#54 7.408   Downloaded displaydoc v0.2.5
2026-Mar-23 01:56:48.967622
#54 7.414   Downloaded group v0.12.1
2026-Mar-23 01:56:48.967622
#54 7.416   Downloaded p256 v0.11.1
2026-Mar-23 01:56:48.967622
#54 7.421   Downloaded aws-sdk-s3 v1.119.0
2026-Mar-23 01:56:48.967622
#54 7.571   Downloaded sec1 v0.3.0
2026-Mar-23 01:56:48.967622
#54 7.576   Downloaded spki v0.7.3
2026-Mar-23 01:56:48.967622
#54 7.581   Downloaded sqlx-macros-core v0.8.6
2026-Mar-23 01:56:48.967622
#54 7.586   Downloaded tokio-macros v2.6.0
2026-Mar-23 01:56:48.967622
#54 7.588   Downloaded tinyvec v1.10.0
2026-Mar-23 01:56:48.967622
#54 7.593   Downloaded raw-cpuid v11.6.0
2026-Mar-23 01:56:48.967622
#54 7.598   Downloaded regex-lite v0.1.9
2026-Mar-23 01:56:48.967622
#54 7.602   Downloaded try-lock v0.2.5
2026-Mar-23 01:56:48.967622
#54 7.604   Downloaded unicode-properties v0.1.4
2026-Mar-23 01:56:48.967622
#54 7.607   Downloaded regex-syntax v0.8.10
2026-Mar-23 01:56:48.967622
#54 7.615   Downloaded regex-automata v0.4.14
2026-Mar-23 01:56:48.967622
#54 7.632   Downloaded uuid v1.21.0
2026-Mar-23 01:56:48.967622
#54 7.636   Downloaded unicode-bidi v0.3.18
2026-Mar-23 01:56:48.967622
#54 7.640   Downloaded ucd-trie v0.1.7
2026-Mar-23 01:56:48.967622
#54 7.642   Downloaded ring v0.17.14
2026-Mar-23 01:56:48.967622
#54 7.691   Downloaded nu-ansi-term v0.50.3
2026-Mar-23 01:56:48.967622
#54 7.695   Downloaded nonzero_ext v0.3.0
2026-Mar-23 01:56:48.967622
#54 7.704   Downloaded getrandom v0.2.17
2026-Mar-23 01:56:48.967622
#54 7.709   Downloaded tracing-core v0.1.36
2026-Mar-23 01:56:48.967622
#54 7.714   Downloaded tracing-attributes v0.1.31
2026-Mar-23 01:56:48.967622
#54 7.719   Downloaded reqwest v0.12.28
2026-Mar-23 01:56:48.967622
#54 7.725   Downloaded vsimd v0.8.0
2026-Mar-23 01:56:48.967622
#54 7.728   Downloaded version_check v0.9.5
2026-Mar-23 01:56:48.967622
#54 7.730   Downloaded sqlx-sqlite v0.8.6
2026-Mar-23 01:56:48.967622
#54 7.735   Downloaded sqlx-postgres v0.8.6
2026-Mar-23 01:56:48.967622
#54 7.750   Downloaded zeroize v1.8.2
2026-Mar-23 01:56:48.967622
#54 7.752   Downloaded zerovec-derive v0.11.2
2026-Mar-23 01:56:48.967622
#54 7.755   Downloaded zmij v1.0.21
2026-Mar-23 01:56:48.967622
#54 7.758   Downloaded yoke v0.8.1
2026-Mar-23 01:56:48.967622
#54 7.760   Downloaded xmlparser v0.13.6
2026-Mar-23 01:56:48.967622
#54 7.764   Downloaded zerofrom-derive v0.1.6
2026-Mar-23 01:56:48.967622
#54 7.766   Downloaded zerofrom v0.1.6
2026-Mar-23 01:56:48.967622
#54 7.767   Downloaded yoke-derive v0.8.1
2026-Mar-23 01:56:48.967622
#54 7.769   Downloaded time v0.3.47
2026-Mar-23 01:56:48.967622
#54 7.790   Downloaded sqlx-mysql v0.8.6
2026-Mar-23 01:56:48.967622
#54 7.799   Downloaded serde_json v1.0.149
2026-Mar-23 01:56:48.967622
#54 7.809   Downloaded serde v1.0.228
2026-Mar-23 01:56:48.967622
#54 7.814   Downloaded typenum v1.19.0
2026-Mar-23 01:56:48.967622
#54 7.818   Downloaded tower-http v0.6.8
2026-Mar-23 01:56:48.967622
#54 7.831   Downloaded tracing-subscriber v0.3.22
2026-Mar-23 01:56:48.967622
#54 7.845   Downloaded tracing v0.1.44
2026-Mar-23 01:56:48.967622
#54 7.866   Downloaded utoipa-gen v4.3.1
2026-Mar-23 01:56:48.967622
#54 7.875   Downloaded unicode-segmentation v1.12.0
2026-Mar-23 01:56:48.967622
#54 7.879   Downloaded vcpkg v0.2.15
2026-Mar-23 01:56:48.967622
#54 7.959   Downloaded subtle v2.6.1
2026-Mar-23 01:56:48.967622
#54 7.961   Downloaded icu_locale_core v2.1.1
2026-Mar-23 01:56:48.967622
#54 7.971   Downloaded quinn-udp v0.5.14
2026-Mar-23 01:56:48.967622
#54 7.974   Downloaded hmac v0.12.1
2026-Mar-23 01:56:48.967622
#54 7.977   Downloaded getrandom v0.3.4
2026-Mar-23 01:56:48.967622
#54 7.982   Downloaded flume v0.11.1
2026-Mar-23 01:56:48.967622
#54 7.987   Downloaded spki v0.6.0
2026-Mar-23 01:56:48.967622
#54 7.989   Downloaded rust-ini v0.20.0
2026-Mar-23 01:56:48.967622
#54 7.991   Downloaded jsonwebtoken v9.3.1
2026-Mar-23 01:56:48.967622
#54 7.997   Downloaded futures v0.3.32
2026-Mar-23 01:56:48.967622
#54 8.003   Downloaded elliptic-curve v0.12.3
2026-Mar-23 01:56:48.967622
#54 8.008   Downloaded httparse v1.10.1
2026-Mar-23 01:56:48.967622
#54 8.011   Downloaded hashlink v0.8.4
2026-Mar-23 01:56:48.967622
#54 8.014   Downloaded litemap v0.8.1
2026-Mar-23 01:56:48.967622
#54 8.016   Downloaded tower v0.5.3
2026-Mar-23 01:56:48.967622
#54 8.030   Downloaded tokio v1.49.0
2026-Mar-23 01:56:48.967622
#54 8.083   Downloaded aws-lc-sys v0.38.0
2026-Mar-23 01:56:48.967622
#54 8.391   Downloaded toml_write v0.1.2
2026-Mar-23 01:56:48.967622
#54 8.393   Downloaded utf8_iter v1.0.4
2026-Mar-23 01:56:48.967622
#54 8.396   Downloaded which v4.4.2
2026-Mar-23 01:56:48.967622
#54 8.398   Downloaded hashbrown v0.14.5
2026-Mar-23 01:56:48.967622
#54 8.404   Downloaded webpki-roots v0.26.11
2026-Mar-23 01:56:48.967622
#54 8.406   Downloaded want v0.3.1
2026-Mar-23 01:56:48.967622
#54 8.408   Downloaded unicode-id-start v1.4.0
2026-Mar-23 01:56:48.967622
#54 8.411   Downloaded rand v0.9.2
2026-Mar-23 01:56:48.967622
#54 8.416   Downloaded num-bigint v0.4.6
2026-Mar-23 01:56:48.967622
#54 8.422   Downloaded icu_collections v2.1.1
2026-Mar-23 01:56:48.967622
#54 8.429   Downloaded hyper-util v0.1.20
2026-Mar-23 01:56:48.967622
#54 8.437   Downloaded http v1.4.0
2026-Mar-23 01:56:48.967622
#54 8.442   Downloaded http v0.2.12
2026-Mar-23 01:56:48.967622
#54 8.446   Downloaded futures-intrusive v0.5.0
2026-Mar-23 01:56:48.967622
#54 8.452   Downloaded urlencoding v2.1.3
2026-Mar-23 01:56:48.967622
#54 8.453   Downloaded untrusted v0.9.0
2026-Mar-23 01:56:48.967622
#54 8.456   Downloaded toml v0.8.23
2026-Mar-23 01:56:48.967622
#54 8.459   Downloaded rustls-pki-types v1.14.0
2026-Mar-23 01:56:48.967622
#54 8.462   Downloaded ron v0.8.1
2026-Mar-23 01:56:48.967622
#54 8.470   Downloaded indexmap v2.13.0
2026-Mar-23 01:56:48.967622
#54 8.475   Downloaded icu_provider v2.1.1
2026-Mar-23 01:56:48.967622
#54 8.478   Downloaded icu_properties v2.1.2
2026-Mar-23 01:56:48.967622
#54 8.481   Downloaded icu_normalizer_data v2.1.1
2026-Mar-23 01:56:48.967622
#54 8.483   Downloaded getrandom v0.4.1
2026-Mar-23 01:56:48.967622
#54 8.488   Downloaded wyz v0.5.1
2026-Mar-23 01:56:48.967622
#54 8.489   Downloaded num-bigint-dig v0.8.6
2026-Mar-23 01:56:48.967622
#54 8.495   Downloaded itertools v0.12.1
2026-Mar-23 01:56:48.967622
#54 8.504   Downloaded rsa v0.9.10
2026-Mar-23 01:56:48.967622
#54 8.511   Downloaded web-time v1.1.0
2026-Mar-23 01:56:48.967622
#54 8.513   Downloaded writeable v0.6.2
2026-Mar-23 01:56:48.967622
#54 8.516   Downloaded which v6.0.3
2026-Mar-23 01:56:48.967622
#54 8.518   Downloaded governor v0.8.1
2026-Mar-23 01:56:48.967622
#54 8.523   Downloaded hashbrown v0.16.1
2026-Mar-23 01:56:48.967622
#54 8.531   Downloaded rustls-webpki v0.103.9
2026-Mar-23 01:56:48.967622
#54 8.535   Downloaded hyper v1.8.1
2026-Mar-23 01:56:48.967622
#54 8.546   Downloaded hashbrown v0.15.5
2026-Mar-23 01:56:48.967622
#54 8.553   Downloaded utoipa v4.2.3
2026-Mar-23 01:56:48.967622
#54 8.557   Downloaded unicode-ident v1.0.24
2026-Mar-23 01:56:48.967622
#54 8.560   Downloaded iri-string v0.7.10
2026-Mar-23 01:56:48.967622
#54 8.571   Downloaded whoami v1.6.1
2026-Mar-23 01:56:48.967622
#54 8.574   Downloaded idna v1.1.0
2026-Mar-23 01:56:48.967622
#54 8.578   Downloaded itertools v0.13.0
2026-Mar-23 01:56:48.967622
#54 8.587   Downloaded hkdf v0.12.4
2026-Mar-23 01:56:48.967622
#54 8.590   Downloaded futures-util v0.3.32
2026-Mar-23 01:56:48.967622
#54 8.608   Downloaded h2 v0.3.27
2026-Mar-23 01:56:48.967622
#54 8.616   Downloaded sqlx-core v0.8.6
2026-Mar-23 01:56:48.967622
#54 8.626   Downloaded toml_edit v0.22.27
2026-Mar-23 01:56:48.967622
#54 8.632   Downloaded hyper v0.14.32
2026-Mar-23 01:56:48.967622
#54 8.641   Downloaded h2 v0.4.13
2026-Mar-23 01:56:48.967622
#54 8.649   Downloaded libm v0.2.16
2026-Mar-23 01:56:48.967622
#54 8.663   Downloaded icu_properties_data v2.1.2
2026-Mar-23 01:56:48.967622
#54 8.677   Downloaded tower v0.4.13
2026-Mar-23 01:56:48.967622
#54 8.691   Downloaded rustls-webpki v0.101.7
2026-Mar-23 01:56:48.967622
#54 8.715   Downloaded flate2 v1.1.9
2026-Mar-23 01:56:48.967622
#54 8.722   Downloaded sqlx v0.8.6
2026-Mar-23 01:56:48.967622
#54 8.740   Downloaded toml_datetime v0.6.11
2026-Mar-23 01:56:48.967622
#54 8.742   Downloaded thiserror-impl v1.0.69
2026-Mar-23 01:56:48.967622
#54 8.744   Downloaded rustls-native-certs v0.8.3
2026-Mar-23 01:56:48.967622
#54 8.747   Downloaded rand_core v0.9.5
2026-Mar-23 01:56:48.967622
#54 8.749   Downloaded pin-project v1.1.10
2026-Mar-23 01:56:48.967622
#54 8.767   Downloaded icu_normalizer v2.1.1
2026-Mar-23 01:56:48.967622
#54 8.771   Downloaded tracing-log v0.2.0
2026-Mar-23 01:56:48.967622
#54 8.773   Downloaded tower-service v0.3.3
2026-Mar-23 01:56:48.967622
#54 8.774   Downloaded tower-layer v0.3.3
2026-Mar-23 01:56:48.967622
#54 8.775   Downloaded tokio-stream v0.1.18
2026-Mar-23 01:56:48.967622
#54 8.782   Downloaded tokio-rustls v0.26.4
2026-Mar-23 01:56:48.967622
#54 8.785   Downloaded syn v1.0.109
2026-Mar-23 01:56:48.967622
#54 8.796   Downloaded tokio-rustls v0.24.1
2026-Mar-23 01:56:48.967622
#54 8.799   Downloaded tokio-native-tls v0.3.1
2026-Mar-23 01:56:48.967622
#54 8.802   Downloaded regex v1.12.3
2026-Mar-23 01:56:48.967622
#54 8.808   Downloaded redis v0.27.6
2026-Mar-23 01:56:48.967622
#54 8.818   Downloaded deno_core_icudata v0.0.73
2026-Mar-23 01:56:48.967622
#54 8.853   Downloaded rand v0.8.5
2026-Mar-23 01:56:48.967622
#54 8.857   Downloaded rustls v0.21.12
2026-Mar-23 01:56:48.967622
#54 8.871   Downloaded quinn-proto v0.11.13
2026-Mar-23 01:56:48.967622
#54 8.880   Downloaded syn v2.0.117
2026-Mar-23 01:56:48.967622
#54 8.893   Downloaded quinn v0.11.9
2026-Mar-23 01:56:48.967622
#54 8.897   Downloaded rustix v0.38.44
2026-Mar-23 01:56:48.967622
#54 8.933   Downloaded publicsuffix v2.3.0
2026-Mar-23 01:56:48.967622
#54 8.936   Downloaded tinyvec_macros v0.1.1
2026-Mar-23 01:56:48.967622
#54 8.939   Downloaded tinystr v0.8.2
2026-Mar-23 01:56:48.967622
#54 8.943   Downloaded rustls v0.23.37
2026-Mar-23 01:56:48.967622
#54 8.957   Downloaded tiny-keccak v2.0.2
2026-Mar-23 01:56:48.967622
#54 8.960   Downloaded time-macros v0.2.27
2026-Mar-23 01:56:48.967622
#54 8.963   Downloaded time-core v0.1.8
2026-Mar-23 01:56:48.967622
#54 8.964   Downloaded thread_local v1.1.9
2026-Mar-23 01:56:48.967622
#54 8.966   Downloaded thiserror-impl v2.0.18
2026-Mar-23 01:56:48.967622
#54 8.968   Downloaded thiserror v2.0.18
2026-Mar-23 01:56:48.967622
#54 8.976   Downloaded thiserror v1.0.69
2026-Mar-23 01:56:48.967622
#54 8.985   Downloaded tap v1.0.1
2026-Mar-23 01:56:48.967622
#54 8.986   Downloaded tagptr v0.2.0
2026-Mar-23 01:56:48.967622
#54 8.988   Downloaded synstructure v0.13.2
2026-Mar-23 01:56:48.967622
#54 8.989   Downloaded sync_wrapper v1.0.2
2026-Mar-23 01:56:48.967622
#54 8.990   Downloaded strum_macros v0.25.3
2026-Mar-23 01:56:48.967622
#54 8.993   Downloaded strum v0.25.0
2026-Mar-23 01:56:48.967622
#54 8.994   Downloaded zerotrie v0.2.3
2026-Mar-23 01:56:48.967622
#54 8.998   Downloaded stringprep v0.1.5
2026-Mar-23 01:56:48.967622
#54 9.000   Downloaded zerovec v0.11.5
2026-Mar-23 01:56:48.967622
#54 9.007   Downloaded static_assertions v1.1.0
2026-Mar-23 01:56:48.967622
#54 9.009   Downloaded yaml-rust2 v0.8.1
2026-Mar-23 01:56:48.967622
#54 9.050   Downloaded stable_deref_trait v1.2.1
2026-Mar-23 01:56:48.967622
#54 9.052   Downloaded prettyplease v0.2.37
2026-Mar-23 01:56:48.967622
#54 9.057   Downloaded portable-atomic v1.13.1
2026-Mar-23 01:56:48.967622
#54 9.067   Downloaded url v2.5.8
2026-Mar-23 01:56:48.967622
#54 9.069   Downloaded sqlx-macros v0.8.6
2026-Mar-23 01:56:48.967622
#54 9.070   Downloaded spinning_top v0.3.0
2026-Mar-23 01:56:48.967622
#54 9.072   Downloaded zerocopy v0.8.39
2026-Mar-23 01:56:48.967622
#54 9.102   Downloaded spin v0.9.8
2026-Mar-23 01:56:48.967622
#54 9.104   Downloaded unicode-normalization v0.1.25
2026-Mar-23 01:56:48.967622
#54 9.108   Downloaded sourcemap v8.0.1
2026-Mar-23 01:56:48.967622
#54 9.111   Downloaded socket2 v0.6.2
2026-Mar-23 01:56:48.967622
#54 9.112   Downloaded socket2 v0.5.10
2026-Mar-23 01:56:48.967622
#54 9.114   Downloaded smallvec v1.15.1
2026-Mar-23 01:56:48.967622
#54 9.117   Downloaded slab v0.4.12
2026-Mar-23 01:56:48.967622
#54 9.118   Downloaded simple_asn1 v0.6.4
2026-Mar-23 01:56:48.967622
#54 9.120   Downloaded simd-adler32 v0.3.8
2026-Mar-23 01:56:48.967622
#54 9.122   Downloaded simd-abstraction v0.7.1
2026-Mar-23 01:56:48.967622
#54 9.123   Downloaded signature v2.2.0
2026-Mar-23 01:56:48.967622
#54 9.125   Downloaded signature v1.6.4
2026-Mar-23 01:56:48.967622
#54 9.127   Downloaded tokio-util v0.7.18
2026-Mar-23 01:56:48.967622
#54 9.136   Downloaded signal-hook-registry v1.4.8
2026-Mar-23 01:56:48.967622
#54 9.137   Downloaded shlex v1.3.0
2026-Mar-23 01:56:48.967622
#54 9.139   Downloaded sharded-slab v0.1.7
2026-Mar-23 01:56:48.967622
#54 9.143   Downloaded sha2 v0.10.9
2026-Mar-23 01:56:48.967622
#54 9.146   Downloaded sha1_smol v1.0.1
2026-Mar-23 01:56:48.967622
#54 9.148   Downloaded sha1 v0.10.6
2026-Mar-23 01:56:48.967622
#54 9.150   Downloaded serde_v8 v0.209.0
2026-Mar-23 01:56:48.967622
#54 9.153   Downloaded serde_urlencoded v0.7.1
2026-Mar-23 01:56:48.967622
#54 9.155   Downloaded serde_spanned v0.6.9
2026-Mar-23 01:56:48.967622
#54 9.156   Downloaded winnow v0.7.14
2026-Mar-23 01:56:48.967622
#54 9.168   Downloaded serde_path_to_error v0.1.20
2026-Mar-23 01:56:48.967622
#54 9.170   Downloaded serde_derive v1.0.228
2026-Mar-23 01:56:48.967622
#54 9.173   Downloaded serde_core v1.0.228
2026-Mar-23 01:56:48.967622
#54 9.176   Downloaded semver-parser v0.7.0
2026-Mar-23 01:56:48.967622
#54 9.178   Downloaded semver v1.0.27
2026-Mar-23 01:56:48.967622
#54 9.181   Downloaded semver v0.9.0
2026-Mar-23 01:56:48.967622
#54 9.182   Downloaded pest_meta v2.8.6
2026-Mar-23 01:56:48.967622
#54 9.185   Downloaded pest v2.8.6
2026-Mar-23 01:56:48.967622
#54 9.191   Downloaded hyper-rustls v0.27.7
2026-Mar-23 01:56:48.967622
#54 9.194   Downloaded hashlink v0.10.0
2026-Mar-23 01:56:48.967622
#54 9.196   Downloaded futures-executor v0.3.32
2026-Mar-23 01:56:48.967622
#54 9.198   Downloaded futures-channel v0.3.32
2026-Mar-23 01:56:48.967622
#54 9.200   Downloaded fs_extra v1.3.0
2026-Mar-23 01:56:48.967622
#54 9.202   Downloaded foldhash v0.1.5
2026-Mar-23 01:56:48.967622
#54 9.203   Downloaded webpki-roots v1.0.6
2026-Mar-23 01:56:48.967622
#54 9.206   Downloaded find-msvc-tools v0.1.9
2026-Mar-23 01:56:48.967622
#54 9.208   Downloaded event-listener v5.4.1
2026-Mar-23 01:56:48.967622
#54 9.211   Downloaded sct v0.7.1
2026-Mar-23 01:56:48.967622
#54 9.217   Downloaded scopeguard v1.2.0
2026-Mar-23 01:56:48.967622
#54 9.219   Downloaded libc v0.2.182
2026-Mar-23 01:56:48.967622
#54 9.274   Downloaded ryu v1.0.23
2026-Mar-23 01:56:48.967622
#54 9.278   Downloaded proc-macro-rules-macros v0.4.0
2026-Mar-23 01:56:48.967622
#54 9.280   Downloaded potential_utf v0.1.4
2026-Mar-23 01:56:48.967622
#54 9.281   Downloaded outref v0.5.2
2026-Mar-23 01:56:48.967622
#54 9.283   Downloaded outref v0.1.0
2026-Mar-23 01:56:48.967622
#54 9.284   Downloaded libloading v0.8.9
2026-Mar-23 01:56:48.967622
#54 9.287   Downloaded jobserver v0.1.34
2026-Mar-23 01:56:48.967622
#54 9.290   Downloaded ipnet v2.11.0
2026-Mar-23 01:56:48.967622
#54 9.292   Downloaded idna_adapter v1.2.1
2026-Mar-23 01:56:48.967622
#54 9.293   Downloaded hyper-rustls v0.24.2
2026-Mar-23 01:56:48.967622
#54 9.296   Downloaded futures-timer v3.0.3
2026-Mar-23 01:56:48.967622
#54 9.299   Downloaded glob v0.3.3
2026-Mar-23 01:56:48.967622
#54 9.301   Downloaded form_urlencoded v1.2.2
2026-Mar-23 01:56:48.967622
#54 9.302   Downloaded quanta v0.12.6
2026-Mar-23 01:56:48.967622
#54 9.306   Downloaded proc-macro2 v1.0.106
2026-Mar-23 01:56:48.967622
#54 9.310   Downloaded ppv-lite86 v0.2.21
2026-Mar-23 01:56:48.967622
#54 9.312   Downloaded nom v7.1.3
2026-Mar-23 01:56:48.967622
#54 9.319   Downloaded moka v0.12.13
2026-Mar-23 01:56:48.967622
#54 9.328   Downloaded rand_chacha v0.3.1
2026-Mar-23 01:56:48.967622
#54 9.330   Downloaded radium v0.7.0
2026-Mar-23 01:56:48.967622
#54 9.332   Downloaded quote v1.0.44
2026-Mar-23 01:56:48.967622
#54 9.336   Downloaded matchers v0.2.0
2026-Mar-23 01:56:48.967622
#54 9.338   Downloaded lazycell v1.3.0
2026-Mar-23 01:56:48.967622
#54 9.339   Downloaded hex v0.4.3
2026-Mar-23 01:56:48.967622
#54 9.342   Downloaded gzip-header v1.0.0
2026-Mar-23 01:56:48.967622
#54 9.344   Downloaded generic-array v0.14.7
2026-Mar-23 01:56:48.967622
#54 9.346   Downloaded fslock v0.2.1
2026-Mar-23 01:56:48.967622
#54 9.348   Downloaded mio v1.1.1
2026-Mar-23 01:56:48.967622
#54 9.356   Downloaded bindgen v0.69.5
2026-Mar-23 01:56:48.967622
#54 9.364   Downloaded aws-lc-rs v1.16.1
2026-Mar-23 01:56:48.967622
#54 9.377   Downloaded pkcs8 v0.9.0
2026-Mar-23 01:56:48.967622
#54 9.380   Downloaded pkcs1 v0.7.5
2026-Mar-23 01:56:48.967622
#54 9.384   Downloaded foreign-types-shared v0.1.1
2026-Mar-23 01:56:48.967622
#54 9.385   Downloaded brotli-decompressor v5.0.0
2026-Mar-23 01:56:48.967622
#54 9.392   Downloaded axum v0.8.8
2026-Mar-23 01:56:48.967622
#54 9.402   Downloaded aws-sdk-sts v1.100.0
2026-Mar-23 01:56:48.967622
#54 9.420   Downloaded pest_generator v2.8.6
2026-Mar-23 01:56:48.967622
#54 9.422   Downloaded heck v0.4.1
2026-Mar-23 01:56:48.967622
#54 9.424   Downloaded futures-macro v0.3.32
2026-Mar-23 01:56:48.967622
#54 9.426   Downloaded futures-io v0.3.32
2026-Mar-23 01:56:48.967622
#54 9.427   Downloaded aws-config v1.8.15
2026-Mar-23 01:56:48.967622
#54 9.434   Downloaded encoding_rs v0.8.35
2026-Mar-23 01:56:48.967622
#54 9.454   Downloaded aho-corasick v1.1.4
2026-Mar-23 01:56:48.967622
#54 9.462   Downloaded crc-fast v1.6.0
2026-Mar-23 01:56:48.967622
#54 9.473   Downloaded pest_derive v2.8.6
2026-Mar-23 01:56:48.967622
#54 9.477   Downloaded minimal-lexical v0.2.1
2026-Mar-23 01:56:48.967622
#54 9.484   Downloaded memchr v2.8.0
2026-Mar-23 01:56:48.967622
#54 9.491   Downloaded aws-smithy-runtime v1.10.3
2026-Mar-23 01:56:48.967622
#54 9.500   Downloaded pathdiff v0.2.3
2026-Mar-23 01:56:48.967622
#54 9.501   Downloaded paste v1.0.15
2026-Mar-23 01:56:48.967622
#54 9.507   Downloaded parking_lot v0.12.5
2026-Mar-23 01:56:48.967622
#54 9.511   Downloaded num-traits v0.2.19
2026-Mar-23 01:56:48.967622
#54 9.515   Downloaded ordered-multimap v0.7.3
2026-Mar-23 01:56:48.967622
#54 9.517   Downloaded once_cell v1.21.3
2026-Mar-23 01:56:48.967622
#54 9.521   Downloaded num-iter v0.1.45
2026-Mar-23 01:56:48.967622
#54 9.522   Downloaded crossbeam-channel v0.5.15
2026-Mar-23 01:56:48.967622
#54 9.527   Downloaded aws-smithy-http-client v1.1.12
2026-Mar-23 01:56:48.967622
#54 9.532   Downloaded aws-sigv4 v1.4.2
2026-Mar-23 01:56:48.967622
#54 9.611   Downloaded aws-runtime v1.7.2
2026-Mar-23 01:56:48.967622
#54 9.617   Downloaded num-integer v0.1.46
2026-Mar-23 01:56:48.967622
#54 9.619   Downloaded miniz_oxide v0.7.4
2026-Mar-23 01:56:48.967622
#54 9.622   Downloaded der v0.7.10
2026-Mar-23 01:56:48.967622
#54 9.631   Downloaded crypto-bigint v0.5.5
2026-Mar-23 01:56:48.967622
#54 9.642   Downloaded cc v1.2.56
2026-Mar-23 01:56:48.967622
#54 9.646   Downloaded base64 v0.22.1
2026-Mar-23 01:56:48.967622
#54 9.651   Downloaded base64 v0.21.7
2026-Mar-23 01:56:48.967622
#54 9.656   Downloaded aws-smithy-types v1.4.6
2026-Mar-23 01:56:48.967622
#54 9.662   Downloaded aws-sdk-ssooidc v1.98.0
2026-Mar-23 01:56:48.967622
#54 9.673   Downloaded aws-sdk-sso v1.96.0
2026-Mar-23 01:56:48.967622
#54 9.682   Downloaded openssl-macros v0.1.1
2026-Mar-23 01:56:48.967622
#54 9.683   Downloaded num_cpus v1.17.0
2026-Mar-23 01:56:48.967622
#54 9.687   Downloaded log v0.4.29
2026-Mar-23 01:56:48.967622
#54 9.691   Downloaded ecdsa v0.14.8
2026-Mar-23 01:56:48.967622
#54 9.693   Downloaded document-features v0.2.12
2026-Mar-23 01:56:48.967622
#54 9.694   Downloaded dlv-list v0.5.2
2026-Mar-23 01:56:48.967622
#54 9.697   Downloaded deno_unsync v0.4.4
2026-Mar-23 01:56:48.967622
#54 9.700   Downloaded bytes v1.11.1
2026-Mar-23 01:56:48.967622
#54 9.705   Downloaded aws-smithy-runtime-api v1.11.6
2026-Mar-23 01:56:48.967622
#54 9.711   Downloaded arc-swap v1.8.2
2026-Mar-23 01:56:48.967622
#54 9.717   Downloaded native-tls v0.2.18
2026-Mar-23 01:56:48.967622
#54 9.719   Downloaded matchit v0.8.4
2026-Mar-23 01:56:48.967622
#54 9.722   Downloaded linux-raw-sys v0.4.15
2026-Mar-23 01:56:48.967622
#54 9.776   Downloaded home v0.5.12
2026-Mar-23 01:56:48.967622
#54 9.777   Downloaded futures-sink v0.3.32
2026-Mar-23 01:56:48.967622
#54 9.778   Downloaded equivalent v1.0.2
2026-Mar-23 01:56:48.967622
#54 9.779   Downloaded anyhow v1.0.102
2026-Mar-23 01:56:48.967622
#54 9.784   Downloaded aws-smithy-json v0.62.5
2026-Mar-23 01:56:48.967622
#54 9.786   Downloaded crossbeam-utils v0.8.21
2026-Mar-23 01:56:48.967622
#54 9.790   Downloaded base64ct v1.8.3
2026-Mar-23 01:56:48.967622
#54 9.793   Downloaded aws-types v1.3.14
2026-Mar-23 01:56:48.967622
#54 9.795   Downloaded aws-smithy-http v0.62.6
2026-Mar-23 01:56:48.967622
#54 9.798   Downloaded ahash v0.8.12
2026-Mar-23 01:56:48.967622
#54 9.801   Downloaded clang-sys v1.8.1
2026-Mar-23 01:56:48.967622
#54 9.804   Downloaded async-lock v3.4.2
2026-Mar-23 01:56:48.967622
#54 9.806   Downloaded md-5 v0.10.6
2026-Mar-23 01:56:48.967622
#54 9.808   Downloaded lru-slab v0.1.2
2026-Mar-23 01:56:48.967622
#54 9.809   Downloaded deranged v0.5.8
2026-Mar-23 01:56:48.967622
#54 9.811   Downloaded if_chain v1.0.3
2026-Mar-23 01:56:48.967622
#54 9.812   Downloaded bitflags v2.11.0
2026-Mar-23 01:56:48.967622
#54 9.817   Downloaded aws-smithy-eventstream v0.60.20
2026-Mar-23 01:56:48.967622
#54 9.820   Downloaded async-trait v0.1.89
2026-Mar-23 01:56:48.967622
#54 9.825   Downloaded bit-vec v0.6.3
2026-Mar-23 01:56:48.967622
#54 9.827   Downloaded bincode v1.3.3
2026-Mar-23 01:56:48.967622
#54 9.830   Downloaded autocfg v1.5.0
2026-Mar-23 01:56:48.967622
#54 9.858   Downloaded v8 v0.101.0
2026-Mar-23 01:56:49.238264
#54 ...
2026-Mar-23 01:56:49.238264
2026-Mar-23 01:56:49.238264
#53 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-23 01:56:49.238264
#53 10.27    Compiling proc-macro2 v1.0.106
2026-Mar-23 01:56:49.238264
#53 10.27    Compiling unicode-ident v1.0.24
2026-Mar-23 01:56:49.348295
#53 10.27    Compiling quote v1.0.44
2026-Mar-23 01:56:49.348295
#53 10.27    Compiling libc v0.2.182
2026-Mar-23 01:56:49.348295
#53 10.27    Compiling cfg-if v1.0.4
2026-Mar-23 01:56:49.348295
#53 10.27    Compiling serde v1.0.228
2026-Mar-23 01:56:49.348295
#53 10.28    Compiling serde_core v1.0.228
2026-Mar-23 01:56:49.348295
#53 10.30    Compiling version_check v0.9.5
2026-Mar-23 01:56:49.348295
#53 10.32    Compiling pin-project-lite v0.2.16
2026-Mar-23 01:56:49.348295
#53 10.33    Compiling parking_lot_core v0.9.12
2026-Mar-23 01:56:49.348295
#53 10.33    Compiling shlex v1.3.0
2026-Mar-23 01:56:49.348295
#53 10.33    Compiling memchr v2.8.0
2026-Mar-23 01:56:49.348295
#53 10.34    Compiling once_cell v1.21.3
2026-Mar-23 01:56:49.348295
#53 10.34    Compiling bytes v1.11.1
2026-Mar-23 01:56:49.348295
#53 10.35    Compiling scopeguard v1.2.0
2026-Mar-23 01:56:49.348295
#53 10.37    Compiling itoa v1.0.17
2026-Mar-23 01:56:49.460163
#53 10.37    Compiling futures-core v0.3.32
2026-Mar-23 01:56:49.460163
#53 10.38    Compiling find-msvc-tools v0.1.9
2026-Mar-23 01:56:49.460163
#53 10.41    Compiling futures-sink v0.3.32
2026-Mar-23 01:56:49.460163
#53 10.42    Compiling autocfg v1.5.0
2026-Mar-23 01:56:49.460163
#53 10.42    Compiling typenum v1.19.0
2026-Mar-23 01:56:49.460163
#53 10.43    Compiling log v0.4.29
2026-Mar-23 01:56:49.460163
#53 10.44    Compiling slab v0.4.12
2026-Mar-23 01:56:49.460163
#53 10.45    Compiling futures-task v0.3.32
2026-Mar-23 01:56:49.460163
#53 10.46    Compiling futures-io v0.3.32
2026-Mar-23 01:56:49.460163
#53 10.47    Compiling zeroize v1.8.2
2026-Mar-23 01:56:49.460163
#53 10.47    Compiling zerocopy v0.8.39
2026-Mar-23 01:56:49.460163
#53 10.49    Compiling equivalent v1.0.2
2026-Mar-23 01:56:49.589976
#53 10.50    Compiling subtle v2.6.1
2026-Mar-23 01:56:49.589976
#53 10.51    Compiling fnv v1.0.7
2026-Mar-23 01:56:49.589976
#53 10.53    Compiling hashbrown v0.16.1
2026-Mar-23 01:56:49.589976
#53 10.57    Compiling lock_api v0.4.14
2026-Mar-23 01:56:49.589976
#53 10.58    Compiling generic-array v0.14.7
2026-Mar-23 01:56:49.589976
#53 10.62    Compiling futures-channel v0.3.32
2026-Mar-23 01:56:49.712195
#53 10.64    Compiling tracing-core v0.1.36
2026-Mar-23 01:56:49.712195
#53 10.65    Compiling percent-encoding v2.3.2
2026-Mar-23 01:56:49.712195
#53 10.66    Compiling time-core v0.1.8
2026-Mar-23 01:56:49.712195
#53 10.66    Compiling powerfmt v0.2.0
2026-Mar-23 01:56:49.712195
#53 10.69    Compiling icu_properties_data v2.1.2
2026-Mar-23 01:56:49.712195
#53 10.69    Compiling num-conv v0.2.0
2026-Mar-23 01:56:49.712195
#53 10.69    Compiling icu_normalizer_data v2.1.1
2026-Mar-23 01:56:49.712195
#53 10.70    Compiling pin-utils v0.1.0
2026-Mar-23 01:56:49.712195
#53 10.71    Compiling ryu v1.0.23
2026-Mar-23 01:56:49.712195
#53 10.74    Compiling stable_deref_trait v1.2.1
2026-Mar-23 01:56:49.880725
#53 10.75    Compiling crc32fast v1.5.0
2026-Mar-23 01:56:49.880725
#53 10.77    Compiling untrusted v0.9.0
2026-Mar-23 01:56:49.880725
#53 10.83    Compiling num-traits v0.2.19
2026-Mar-23 01:56:49.880725
#53 10.83    Compiling http v1.4.0
2026-Mar-23 01:56:49.880725
#53 10.83    Compiling dunce v1.0.5
2026-Mar-23 01:56:49.880725
#53 10.91    Compiling time-macros v0.2.27
2026-Mar-23 01:56:50.005815
#53 10.91    Compiling deranged v0.5.8
2026-Mar-23 01:56:50.005815
#53 10.92    Compiling fs_extra v1.3.0
2026-Mar-23 01:56:50.005815
#53 10.94    Compiling http v0.2.12
2026-Mar-23 01:56:50.005815
#53 10.95    Compiling zmij v1.0.21
2026-Mar-23 01:56:50.005815
#53 10.95    Compiling form_urlencoded v1.2.2
2026-Mar-23 01:56:50.005815
#53 10.95    Compiling rustls-pki-types v1.14.0
2026-Mar-23 01:56:50.005815
#53 10.96    Compiling writeable v0.6.2
2026-Mar-23 01:56:50.005815
#53 10.96    Compiling litemap v0.8.1
2026-Mar-23 01:56:50.005815
#53 10.97    Compiling tower-service v0.3.3
2026-Mar-23 01:56:50.005815
#53 10.98    Compiling aws-lc-rs v1.16.1
2026-Mar-23 01:56:50.005815
#53 11.03    Compiling outref v0.5.2
2026-Mar-23 01:56:50.121656
#53 11.07    Compiling base64 v0.22.1
2026-Mar-23 01:56:50.121656
#53 11.07    Compiling httparse v1.10.1
2026-Mar-23 01:56:50.121656
#53 11.07    Compiling vsimd v0.8.0
2026-Mar-23 01:56:50.121656
#53 11.07    Compiling try-lock v0.2.5
2026-Mar-23 01:56:50.121656
#53 11.12    Compiling httpdate v1.0.3
2026-Mar-23 01:56:50.221888
#53 11.16    Compiling rustls v0.23.37
2026-Mar-23 01:56:50.221888
#53 11.16    Compiling crossbeam-utils v0.8.21
2026-Mar-23 01:56:50.221888
#53 11.19    Compiling want v0.3.1
2026-Mar-23 01:56:50.221888
#53 11.23    Compiling allocator-api2 v0.2.21
2026-Mar-23 01:56:50.328609
#53 11.25    Compiling atomic-waker v1.1.2
2026-Mar-23 01:56:50.328609
#53 11.25    Compiling tower-layer v0.3.3
2026-Mar-23 01:56:50.328609
#53 11.25    Compiling utf8_iter v1.0.4
2026-Mar-23 01:56:50.328609
#53 11.30    Compiling openssl-probe v0.2.1
2026-Mar-23 01:56:50.328609
#53 11.32    Compiling cpufeatures v0.2.17
2026-Mar-23 01:56:50.459312
#53 11.40    Compiling webpki-roots v1.0.6
2026-Mar-23 01:56:50.459312
#53 11.41    Compiling sync_wrapper v1.0.2
2026-Mar-23 01:56:50.573501
#53 11.50    Compiling serde_json v1.0.149
2026-Mar-23 01:56:50.573501
#53 11.50    Compiling ipnet v2.11.0
2026-Mar-23 01:56:50.573501
#53 11.57    Compiling http-body v1.0.1
2026-Mar-23 01:56:50.573501
#53 11.58    Compiling syn v2.0.117
2026-Mar-23 01:56:50.573501
#53 11.60    Compiling base64-simd v0.8.0
2026-Mar-23 01:56:50.694099
#53 11.60    Compiling bitflags v2.11.0
2026-Mar-23 01:56:50.694099
#53 11.60    Compiling rustversion v1.0.22
2026-Mar-23 01:56:50.694099
#53 11.60    Compiling thiserror v2.0.18
2026-Mar-23 01:56:50.694099
#53 11.72    Compiling http-body v0.4.6
2026-Mar-23 01:56:50.794215
#53 11.74    Compiling home v0.5.12
2026-Mar-23 01:56:50.794215
#53 11.75    Compiling getrandom v0.4.1
2026-Mar-23 01:56:50.794215
#53 11.78    Compiling rustls-native-certs v0.8.3
2026-Mar-23 01:56:50.794215
#53 11.80    Compiling jobserver v0.1.34
2026-Mar-23 01:56:50.794215
#53 11.82    Compiling
2026-Mar-23 01:56:50.913541
http-body-util v0.1.3
2026-Mar-23 01:56:50.913541
#53 11.83    Compiling getrandom v0.2.17
2026-Mar-23 01:56:50.913541
#53 11.83    Compiling const-oid v0.9.6
2026-Mar-23 01:56:50.913541
#53 11.90    Compiling errno v0.3.14
2026-Mar-23 01:56:50.913541
#53 11.90    Compiling socket2 v0.6.2
2026-Mar-23 01:56:51.036840
#53 12.00    Compiling mio v1.1.1
2026-Mar-23 01:56:51.036840
#53 12.00    Compiling socket2 v0.5.10
2026-Mar-23 01:56:51.036840
#53 12.04    Compiling hex v0.4.3
2026-Mar-23 01:56:51.036840
#53 12.06    Compiling
2026-Mar-23 01:56:51.138787
der v0.6.1
2026-Mar-23 01:56:51.138787
#53 12.13    Compiling pkg-config v0.3.32
2026-Mar-23 01:56:51.138787
#53 12.15    Compiling base64ct v1.8.3
2026-Mar-23 01:56:51.138787
#53 12.16    Compiling num-integer v0.1.46
2026-Mar-23 01:56:51.252611
#53 12.16    Compiling vcpkg v0.2.15
2026-Mar-23 01:56:51.252611
#53 12.18    Compiling ahash v0.8.12
2026-Mar-23 01:56:51.252611
#53 12.21    Compiling cc v1.2.56
2026-Mar-23 01:56:51.252611
#53 12.22    Compiling signal-hook-registry v1.4.8
2026-Mar-23 01:56:51.252611
#53 12.25    Compiling rand_core v0.6.4
2026-Mar-23 01:56:51.252611
#53 12.28    Compiling time v0.3.47
2026-Mar-23 01:56:51.369904
#53 12.28    Compiling block-buffer v0.10.4
2026-Mar-23 01:56:51.369904
#53 12.28    Compiling crypto-common v0.1.7
2026-Mar-23 01:56:51.369904
#53 12.36    Compiling rustls v0.21.12
2026-Mar-23 01:56:51.369904
#53 12.39    Compiling aho-corasick v1.1.4
2026-Mar-23 01:56:51.497810
#53 12.48    Compiling uuid v1.21.0
2026-Mar-23 01:56:51.497810
#53 12.48    Compiling foldhash v0.1.5
2026-Mar-23 01:56:51.497810
#53 12.52    Compiling rustix v0.38.44
2026-Mar-23 01:56:51.640355
#53 12.59    Compiling digest v0.10.7
2026-Mar-23 01:56:51.640355
#53 12.61    Compiling base16ct v0.1.1
2026-Mar-23 01:56:51.640355
#53 12.61    Compiling crc-catalog v2.4.0
2026-Mar-23 01:56:51.640355
#53 12.61    Compiling glob v0.3.3
2026-Mar-23 01:56:51.753759
#53 12.69    Compiling regex-syntax v0.8.10
2026-Mar-23 01:56:51.753759
#53 12.78    Compiling hashbrown v0.15.5
2026-Mar-23 01:56:51.921089
#53 12.81    Compiling ff v0.12.1
2026-Mar-23 01:56:51.921089
#53 12.81    Compiling crypto-bigint v0.4.9
2026-Mar-23 01:56:51.921089
#53 12.83    Compiling concurrent-queue v2.5.0
2026-Mar-23 01:56:52.074372
#53 12.99    Compiling hmac v0.12.1
2026-Mar-23 01:56:52.074372
#53 12.99    Compiling sha2 v0.10.9
2026-Mar-23 01:56:52.074372
#53 12.99    Compiling crc v3.4.0
2026-Mar-23 01:56:52.175345
#53 13.20    Compiling alloc-no-stdlib v2.0.4
2026-Mar-23 01:56:52.324064
#53 13.26    Compiling cmake v0.1.57
2026-Mar-23 01:56:52.331059
#53 13.29    Compiling clang-sys v1.8.1
2026-Mar-23 01:56:52.442772
#53 13.36    Compiling group v0.12.1
2026-Mar-23 01:56:52.442772
#53 13.47    Compiling tinyvec_macros v0.1.1
2026-Mar-23 01:56:52.442772
#53 13.47    Compiling parking v2.2.1
2026-Mar-23 01:56:52.560626
#53 13.55    Compiling semver v1.0.27
2026-Mar-23 01:56:52.560626
#53 13.58    Compiling prettyplease v0.2.37
2026-Mar-23 01:56:52.679319
#53 13.59    Compiling linux-raw-sys v0.4.15
2026-Mar-23 01:56:52.835601
#53 13.72    Compiling spki v0.6.0
2026-Mar-23 01:56:52.835601
#53 13.80    Compiling indexmap v2.13.0
2026-Mar-23 01:56:52.835601
#53 13.80    Compiling event-listener v5.4.1
2026-Mar-23 01:56:52.835601
#53 13.86    Compiling alloc-stdlib v0.2.2
2026-Mar-23 01:56:52.952011
#53 13.92    Compiling signature v1.6.4
2026-Mar-23 01:56:52.952011
#53 13.93    Compiling tinyvec v1.10.0
2026-Mar-23 01:56:52.952011
#53 13.98    Compiling rfc6979 v0.3.1
2026-Mar-23 01:56:53.088018
#53 14.01    Compiling rustc_version v0.4.1
2026-Mar-23 01:56:53.088018
#53 14.01    Compiling pkcs8 v0.9.0
2026-Mar-23 01:56:53.240471
#53 14.17    Compiling getrandom v0.3.4
2026-Mar-23 01:56:53.240471
#53 14.19    Compiling minimal-lexical v0.2.1
2026-Mar-23 01:56:53.240471
#53 14.27    Compiling adler2 v2.0.1
2026-Mar-23 01:56:53.378971
#53 14.29    Compiling foreign-types-shared v0.1.1
2026-Mar-23 01:56:53.378971
#53 14.31    Compiling simd-adler32 v0.3.8
2026-Mar-23 01:56:53.378971
#53 14.31    Compiling openssl v0.10.75
2026-Mar-23 01:56:53.486558
#53 14.46    Compiling sec1 v0.3.0
2026-Mar-23 01:56:53.625125
#53 14.57    Compiling ring v0.17.14
2026-Mar-23 01:56:53.625125
#53 14.57    Compiling aws-lc-sys v0.38.0
2026-Mar-23 01:56:53.625125
#53 14.57    Compiling openssl-sys v0.9.111
2026-Mar-23 01:56:53.625125
#53 14.60    Compiling tokio v1.49.0
2026-Mar-23 01:56:53.625125
#53 14.65    Compiling thiserror v1.0.69
2026-Mar-23 01:56:53.766255
#53 14.65    Compiling foreign-types v0.3.2
2026-Mar-23 01:56:53.766255
#53 14.71    Compiling aws-types v1.3.14
2026-Mar-23 01:56:53.766255
#53 14.79    Compiling unicode-normalization v0.1.25
2026-Mar-23 01:56:53.959898
#53 14.84    Compiling miniz_oxide v0.8.9
2026-Mar-23 01:56:53.959898
#53 14.84    Compiling futures-util v0.3.32
2026-Mar-23 01:56:53.959898
#53 14.84    Compiling hashlink v0.10.0
2026-Mar-23 01:56:53.959898
#53 14.98    Compiling elliptic-curve v0.12.3
2026-Mar-23 01:56:53.959898
#53 14.98    Compiling nom v7.1.3
2026-Mar-23 01:56:54.116139
#53 15.07    Compiling brotli-decompressor v5.0.0
2026-Mar-23 01:56:54.116139
#53 15.07    Compiling webpki-roots v0.26.11
2026-Mar-23 01:56:54.277795
#53 15.19    Compiling crossbeam-queue v0.3.12
2026-Mar-23 01:56:54.277795
#53 15.23    Compiling md-5 v0.10.6
2026-Mar-23 01:56:54.411164
#53 15.32    Compiling libloading v0.8.9
2026-Mar-23 01:56:54.411164
#53 15.36    Compiling regex-automata v0.4.14
2026-Mar-23 01:56:54.411164
#53 15.44    Compiling
2026-Mar-23 01:56:54.539851
unicode-properties v0.1.4
2026-Mar-23 01:56:54.539851
#53 15.48    Compiling native-tls v0.2.18
2026-Mar-23 01:56:54.539851
#53 15.52    Compiling bindgen v0.69.5
2026-Mar-23 01:56:54.539851
#53 15.52    Compiling ecdsa v0.14.8
2026-Mar-23 01:56:54.539851
#53 15.52    Compiling anyhow v1.0.102
2026-Mar-23 01:56:54.664475
#53 15.62    Compiling crunchy v0.2.4
2026-Mar-23 01:56:54.664475
#53 15.65    Compiling unicode-bidi v0.3.18
2026-Mar-23 01:56:54.664475
#53 15.69    Compiling atoi v2.0.0
2026-Mar-23 01:56:54.792618
#53 15.81    Compiling flate2 v1.1.9
2026-Mar-23 01:56:54.899472
#53 15.83    Compiling hkdf v0.12.4
2026-Mar-23 01:56:55.027644
#53 15.94    Compiling ppv-lite86 v0.2.21
2026-Mar-23 01:56:55.199603
#53 16.22    Compiling crypto-bigint v0.5.5
2026-Mar-23 01:56:55.368440
#53 16.29    Compiling hashbrown v0.14.5
2026-Mar-23 01:56:55.368440
#53 16.29    Compiling stringprep v0.1.5
2026-Mar-23 01:56:55.368440
#53 16.29    Compiling cookie v0.18.1
2026-Mar-23 01:56:55.669536
#53 16.54    Compiling brotli v8.0.2
2026-Mar-23 01:56:55.702568
#53 16.71    Compiling rand_chacha v0.3.1
2026-Mar-23 01:56:55.702568
#53 16.71    Compiling p256 v0.11.1
2026-Mar-23 01:56:55.702568
#53 16.71    Compiling ucd-trie v0.1.7
2026-Mar-23 01:56:55.796207
#53 16.82    Compiling lazy_static v1.5.0
2026-Mar-23 01:56:55.952461
#53 16.82    Compiling lazycell v1.3.0
2026-Mar-23 01:56:55.952461
#53 16.82    Compiling dotenvy v0.15.7
2026-Mar-23 01:56:55.952461
#53 16.82    Compiling compression-core v0.4.31
2026-Mar-23 01:56:56.075578
#53 17.06    Compiling synstructure v0.13.2
2026-Mar-23 01:56:56.075578
#53 17.06    Compiling cexpr v0.6.0
2026-Mar-23 01:56:56.192510
#53 17.12    Compiling fastrand v2.3.0
2026-Mar-23 01:56:56.192510
#53 17.12    Compiling rustc-hash v1.1.0
2026-Mar-23 01:56:56.303573
#53 17.25    Compiling rand v0.8.5
2026-Mar-23 01:56:56.303573
#53 17.29    Compiling byteorder v1.5.0
2026-Mar-23 01:56:56.303573
#53 17.29    Compiling adler v1.0.2
2026-Mar-23 01:56:56.451210
#53 17.35    Compiling whoami v1.6.1
2026-Mar-23 01:56:56.451210
#53 17.39    Compiling tiny-keccak v2.0.2
2026-Mar-23 01:56:56.451210
#53 17.48    Compiling
2026-Mar-23 01:56:56.633298
pest v2.8.6
2026-Mar-23 01:56:56.633298
#53 17.53    Compiling miniz_oxide v0.7.4
2026-Mar-23 01:56:56.773180
#53 17.73    Compiling rand_core v0.9.5
2026-Mar-23 01:56:56.949089
#53 17.98    Compiling gzip-header v1.0.0
2026-Mar-23 01:56:57.366482
#53 18.22    Compiling num-bigint v0.4.6
2026-Mar-23 01:56:57.366482
#53 18.22    Compiling fslock v0.2.1
2026-Mar-23 01:56:57.366482
#53 18.22    Compiling encoding_rs v0.8.35
2026-Mar-23 01:56:57.366482
#53 18.27    Compiling paste v1.0.15
2026-Mar-23 01:56:57.366482
#53 18.39    Compiling radium v0.7.0
2026-Mar-23 01:56:57.479487
#53 18.42    Compiling regex v1.12.3
2026-Mar-23 01:56:57.574511
#53 18.55    Compiling heck v0.5.0
2026-Mar-23 01:56:57.574511
#53 18.60    Compiling mime v0.3.17
2026-Mar-23 01:56:57.768491
#53 18.60    Compiling litrs v1.0.0
2026-Mar-23 01:56:57.768491
#53 18.67    Compiling psl-types v2.0.11
2026-Mar-23 01:56:57.926482
#53 18.85    Compiling rand_chacha v0.9.0
2026-Mar-23 01:56:58.121320
#53 19.15    Compiling arc-swap v1.8.2
2026-Mar-23 01:56:58.334475
#53 19.23    Compiling sha1_smol v1.0.1
2026-Mar-23 01:56:58.334475
#53 19.30    Compiling regex-lite v0.1.9
2026-Mar-23 01:56:58.396505
#53 19.30    Compiling iri-string v0.7.10
2026-Mar-23 01:56:58.808158
#53 19.70    Compiling pest_meta v2.8.6
2026-Mar-23 01:56:58.817931
#53 19.76    Compiling portable-atomic v1.13.1
2026-Mar-23 01:56:58.817931
#53 19.78    Compiling tap v1.0.1
2026-Mar-23 01:56:58.817931
#53 19.80    Compiling heck v0.4.1
2026-Mar-23 01:56:58.817931
#53 19.83    Compiling document-features v0.2.12
2026-Mar-23 01:56:58.948397
#53 19.83    Compiling outref v0.1.0
2026-Mar-23 01:56:58.948397
#53 19.92    Compiling wyz v0.5.1
2026-Mar-23 01:56:59.087867
#53 19.99    Compiling rand v0.9.2
2026-Mar-23 01:56:59.087867
#53 20.05    Compiling simd-abstraction v0.7.1
2026-Mar-23 01:56:59.087867
#53 20.07    Compiling const-random-macro v0.1.16
2026-Mar-23 01:56:59.267671
#53 20.21    Compiling memoffset v0.9.1
2026-Mar-23 01:56:59.275657
2026-Mar-23 01:56:59.395031
#53 20.32    Compiling tokio-stream v0.1.18
2026-Mar-23 01:56:59.403584
#53 20.39    Compiling proc-macro-error-attr v1.0.4
2026-Mar-23 01:56:59.403584
#53 20.42    Compiling xmlparser v0.13.6
2026-Mar-23 01:56:59.518117
#53 20.49    Compiling funty v2.0.0
2026-Mar-23 01:56:59.518117
#53 20.54    Compiling syn v1.0.109
2026-Mar-23 01:56:59.691100
#53 20.60    Compiling const-random v0.1.18
2026-Mar-23 01:56:59.691100
#53 20.72    Compiling pest_generator v2.8.6
2026-Mar-23 01:56:59.858900
#53 20.86    Compiling compression-codecs v0.4.37
2026-Mar-23 01:56:59.977433
#53 20.90    Compiling aws-smithy-xml v0.60.15
2026-Mar-23 01:56:59.977433
#53 20.90    Compiling base64-simd v0.7.0
2026-Mar-23 01:56:59.977433
#53 20.95    Compiling event-listener-strategy v0.5.4
2026-Mar-23 01:56:59.977433
#53 ...
2026-Mar-23 01:56:59.977433
2026-Mar-23 01:56:59.977433
#54 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-23 01:56:59.977433
#54 10.97   Downloaded libsqlite3-sys v0.30.1
2026-Mar-23 01:56:59.977433
#54 11.20    Compiling proc-macro2 v1.0.106
2026-Mar-23 01:56:59.977433
#54 11.20    Compiling quote v1.0.44
2026-Mar-23 01:56:59.977433
#54 11.20    Compiling unicode-ident v1.0.24
2026-Mar-23 01:56:59.977433
#54 11.20    Compiling libc v0.2.182
2026-Mar-23 01:56:59.977433
#54 11.20    Compiling cfg-if v1.0.4
2026-Mar-23 01:56:59.977433
#54 11.23    Compiling serde v1.0.228
2026-Mar-23 01:56:59.977433
#54 11.26    Compiling serde_core v1.0.228
2026-Mar-23 01:56:59.977433
#54 11.27    Compiling pin-project-lite v0.2.16
2026-Mar-23 01:56:59.977433
#54 11.28    Compiling parking_lot_core v0.9.12
2026-Mar-23 01:56:59.977433
#54 11.29    Compiling shlex v1.3.0
2026-Mar-23 01:56:59.977433
#54 11.29    Compiling bytes v1.11.1
2026-Mar-23 01:56:59.977433
#54 11.29    Compiling scopeguard v1.2.0
2026-Mar-23 01:56:59.977433
#54 11.31    Compiling futures-core v0.3.32
2026-Mar-23 01:56:59.977433
#54 11.34    Compiling find-msvc-tools v0.1.9
2026-Mar-23 01:56:59.977433
#54 11.37    Compiling version_check v0.9.5
2026-Mar-23 01:56:59.977433
#54 11.37    Compiling memchr v2.8.0
2026-Mar-23 01:56:59.977433
#54 11.37    Compiling itoa v1.0.17
2026-Mar-23 01:56:59.977433
#54 11.40    Compiling once_cell v1.21.3
2026-Mar-23 01:56:59.977433
#54 11.44    Compiling futures-sink v0.3.32
2026-Mar-23 01:56:59.977433
#54 11.44    Compiling typenum v1.19.0
2026-Mar-23 01:56:59.977433
#54 11.44    Compiling autocfg v1.5.0
2026-Mar-23 01:56:59.977433
#54 11.44    Compiling log v0.4.29
2026-Mar-23 01:56:59.977433
#54 11.44    Compiling slab v0.4.12
2026-Mar-23 01:56:59.977433
#54 11.44    Compiling futures-task v0.3.32
2026-Mar-23 01:56:59.977433
#54 11.44    Compiling futures-io v0.3.32
2026-Mar-23 01:56:59.977433
#54 11.44    Compiling zeroize v1.8.2
2026-Mar-23 01:56:59.977433
#54 11.44    Compiling subtle v2.6.1
2026-Mar-23 01:56:59.977433
#54 11.44    Compiling fnv v1.0.7
2026-Mar-23 01:56:59.977433
#54 11.44    Compiling equivalent v1.0.2
2026-Mar-23 01:56:59.977433
#54 11.46    Compiling percent-encoding v2.3.2
2026-Mar-23 01:56:59.977433
#54 11.50    Compiling icu_normalizer_data v2.1.1
2026-Mar-23 01:56:59.977433
#54 11.50    Compiling icu_properties_data v2.1.2
2026-Mar-23 01:56:59.977433
#54 11.52    Compiling lock_api v0.4.14
2026-Mar-23 01:56:59.977433
#54 11.58    Compiling zerocopy v0.8.39
2026-Mar-23 01:56:59.977433
#54 11.58    Compiling hashbrown v0.16.1
2026-Mar-23 01:56:59.977433
#54 11.59    Compiling pin-utils v0.1.0
2026-Mar-23 01:56:59.977433
#54 11.59    Compiling time-core v0.1.8
2026-Mar-23 01:56:59.977433
#54 11.67    Compiling futures-channel v0.3.32
2026-Mar-23 01:56:59.977433
#54 11.67    Compiling powerfmt v0.2.0
2026-Mar-23 01:56:59.977433
#54 11.67    Compiling ryu v1.0.23
2026-Mar-23 01:56:59.977433
#54 11.69    Compiling num-conv v0.2.0
2026-Mar-23 01:56:59.977433
#54 11.79    Compiling tracing-core v0.1.36
2026-Mar-23 01:56:59.977433
#54 11.79    Compiling stable_deref_trait v1.2.1
2026-Mar-23 01:56:59.977433
#54 11.93    Compiling crc32fast v1.5.0
2026-Mar-23 01:56:59.977433
#54 11.93    Compiling untrusted v0.9.0
2026-Mar-23 01:56:59.977433
#54 11.93    Compiling dunce v1.0.5
2026-Mar-23 01:56:59.977433
#54 11.96    Compiling fs_extra v1.3.0
2026-Mar-23 01:56:59.977433
#54 11.99    Compiling generic-array v0.14.7
2026-Mar-23 01:56:59.977433
#54 12.00    Compiling form_urlencoded v1.2.2
2026-Mar-23 01:56:59.977433
#54 12.04    Compiling tower-service v0.3.3
2026-Mar-23 01:56:59.977433
#54 12.04    Compiling litemap v0.8.1
2026-Mar-23 01:56:59.977433
#54 12.04    Compiling writeable v0.6.2
2026-Mar-23 01:56:59.977433
#54 12.11    Compiling num-traits v0.2.19
2026-Mar-23 01:56:59.977433
#54 12.11    Compiling aws-lc-rs v1.16.1
2026-Mar-23 01:56:59.977433
#54 12.11    Compiling http v1.4.0
2026-Mar-23 01:56:59.977433
#54 12.11    Compiling http v0.2.12
2026-Mar-23 01:56:59.977433
#54 12.16    Compiling rustls-pki-types v1.14.0
2026-Mar-23 01:56:59.977433
#54 12.16    Compiling httparse v1.10.1
2026-Mar-23 01:56:59.977433
#54 12.16    Compiling zmij v1.0.21
2026-Mar-23 01:56:59.977433
#54 12.21    Compiling deranged v0.5.8
2026-Mar-23 01:56:59.977433
#54 12.22    Compiling time-macros v0.2.27
2026-Mar-23 01:56:59.977433
#54 12.30    Compiling vsimd v0.8.0
2026-Mar-23 01:56:59.977433
#54 12.32    Compiling outref v0.5.2
2026-Mar-23 01:56:59.977433
#54 12.32    Compiling try-lock v0.2.5
2026-Mar-23 01:56:59.977433
#54 12.48    Compiling base64 v0.22.1
2026-Mar-23 01:56:59.977433
#54 12.58    Compiling httpdate v1.0.3
2026-Mar-23 01:56:59.977433
#54 12.64    Compiling rustls v0.23.37
2026-Mar-23 01:56:59.977433
#54 12.69    Compiling atomic-waker v1.1.2
2026-Mar-23 01:56:59.977433
#54 12.69    Compiling cpufeatures v0.2.17
2026-Mar-23 01:56:59.977433
#54 12.71    Compiling want v0.3.1
2026-Mar-23 01:56:59.977433
#54 12.74    Compiling openssl-probe v0.2.1
2026-Mar-23 01:56:59.977433
#54 12.76    Compiling tower-layer v0.3.3
2026-Mar-23 01:56:59.977433
#54 12.88    Compiling webpki-roots v1.0.6
2026-Mar-23 01:56:59.977433
#54 12.88    Compiling utf8_iter v1.0.4
2026-Mar-23 01:56:59.977433
#54 12.88    Compiling sync_wrapper v1.0.2
2026-Mar-23 01:56:59.977433
#54 12.97    Compiling crossbeam-utils v0.8.21
2026-Mar-23 01:56:59.977433
#54 12.97    Compiling ipnet v2.11.0
2026-Mar-23 01:56:59.977433
#54 13.12    Compiling bitflags v2.11.0
2026-Mar-23 01:56:59.977433
#54 13.15    Compiling rustversion v1.0.22
2026-Mar-23 01:56:59.977433
#54 13.26    Compiling home v0.5.12
2026-Mar-23 01:56:59.977433
#54 13.37    Compiling http-body v0.4.6
2026-Mar-23 01:56:59.977433
#54 13.37    Compiling getrandom v0.4.1
2026-Mar-23 01:56:59.977433
#54 13.40    Compiling http-body v1.0.1
2026-Mar-23 01:56:59.977433
#54 13.42    Compiling rustls-native-certs v0.8.3
2026-Mar-23 01:56:59.977433
#54 13.42    Compiling base64-simd v0.8.0
2026-Mar-23 01:56:59.977433
#54 13.65    Compiling errno v0.3.14
2026-Mar-23 01:56:59.977433
#54 13.65    Compiling socket2 v0.6.2
2026-Mar-23 01:56:59.977433
#54 13.65    Compiling mio v1.1.1
2026-Mar-23 01:56:59.977433
#54 13.65    Compiling getrandom v0.2.17
2026-Mar-23 01:56:59.977433
#54 13.66    Compiling socket2 v0.5.10
2026-Mar-23 01:56:59.977433
#54 13.72    Compiling serde_json v1.0.149
2026-Mar-23 01:56:59.977433
#54 13.72    Compiling hex v0.4.3
2026-Mar-23 01:56:59.977433
#54 13.72    Compiling const-oid v0.9.6
2026-Mar-23 01:56:59.977433
#54 13.73    Compiling thiserror v2.0.18
2026-Mar-23 01:56:59.977433
#54 13.90    Compiling http-body-util v0.1.3
2026-Mar-23 01:56:59.977433
#54 13.97    Compiling syn v2.0.117
2026-Mar-23 01:56:59.977433
#54 13.97    Compiling signal-hook-registry v1.4.8
2026-Mar-23 01:56:59.977433
#54 14.01    Compiling num-integer v0.1.46
2026-Mar-23 01:56:59.977433
#54 14.01    Compiling rand_core v0.6.4
2026-Mar-23 01:56:59.977433
#54 14.07    Compiling jobserver v0.1.34
2026-Mar-23 01:56:59.977433
#54 14.07    Compiling time v0.3.47
2026-Mar-23 01:56:59.977433
#54 14.13    Compiling pkg-config v0.3.32
2026-Mar-23 01:56:59.977433
#54 14.17    Compiling der v0.6.1
2026-Mar-23 01:56:59.977433
#54 14.18    Compiling base64ct v1.8.3
2026-Mar-23 01:56:59.977433
#54 14.22    Compiling allocator-api2 v0.2.21
2026-Mar-23 01:56:59.977433
#54 14.22    Compiling vcpkg v0.2.15
2026-Mar-23 01:56:59.977433
#54 14.29    Compiling uuid v1.21.0
2026-Mar-23 01:56:59.977433
#54 14.34    Compiling rustls v0.21.12
2026-Mar-23 01:56:59.977433
#54 14.42    Compiling ff v0.12.1
2026-Mar-23 01:56:59.977433
#54 14.48    Compiling cc v1.2.56
2026-Mar-23 01:56:59.977433
#54 14.53    Compiling base16ct v0.1.1
2026-Mar-23 01:56:59.977433
#54 14.58    Compiling glob v0.3.3
2026-Mar-23 01:56:59.977433
#54 14.58    Compiling foldhash v0.1.5
2026-Mar-23 01:56:59.977433
#54 14.58    Compiling crc-catalog v2.4.0
2026-Mar-23 01:56:59.977433
#54 14.60    Compiling rustix v0.38.44
2026-Mar-23 01:56:59.977433
#54 14.60    Compiling group v0.12.1
2026-Mar-23 01:56:59.977433
#54 14.81    Compiling concurrent-queue v2.5.0
2026-Mar-23 01:56:59.977433
#54 15.01    Compiling indexmap v2.13.0
2026-Mar-23 01:56:59.977433
#54 15.01    Compiling semver v1.0.27
2026-Mar-23 01:56:59.977433
#54 15.01    Compiling alloc-no-stdlib v2.0.4
2026-Mar-23 01:56:59.977433
#54 15.03    Compiling tinyvec_macros v0.1.1
2026-Mar-23 01:56:59.977433
#54 15.16    Compiling block-buffer v0.10.4
2026-Mar-23 01:56:59.977433
#54 15.16    Compiling crypto-common v0.1.7
2026-Mar-23 01:56:59.977433
#54 15.32    Compiling spki v0.6.0
2026-Mar-23 01:56:59.977433
#54 15.32    Compiling crypto-bigint v0.4.9
2026-Mar-23 01:56:59.977433
#54 15.32    Compiling hashbrown v0.15.5
2026-Mar-23 01:56:59.977433
#54 15.35    Compiling parking v2.2.1
2026-Mar-23 01:56:59.977433
#54 15.35    Compiling linux-raw-sys v0.4.15
2026-Mar-23 01:56:59.977433
#54 15.36    Compiling clang-sys v1.8.1
2026-Mar-23 01:56:59.977433
#54 15.39    Compiling crc v3.4.0
2026-Mar-23 01:56:59.977433
#54 15.58    Compiling digest v0.10.7
2026-Mar-23 01:56:59.977433
#54 15.59    Compiling prettyplease v0.2.37
2026-Mar-23 01:56:59.977433
#54 15.59    Compiling cmake v0.1.57
2026-Mar-23 01:56:59.977433
#54 15.59    Compiling pkcs8 v0.9.0
2026-Mar-23 01:56:59.977433
#54 15.65    Compiling rustc_version v0.4.1
2026-Mar-23 01:56:59.977433
#54 15.65    Compiling tinyvec v1.10.0
2026-Mar-23 01:56:59.977433
#54 15.78    Compiling event-listener v5.4.1
2026-Mar-23 01:56:59.977433
#54 15.78    Compiling alloc-stdlib v0.2.2
2026-Mar-23 01:56:59.977433
#54 15.94    Compiling sec1 v0.3.0
2026-Mar-23 01:56:59.977433
#54 16.07    Compiling aho-corasick v1.1.4
2026-Mar-23 01:56:59.977433
#54 16.15    Compiling regex-syntax v0.8.10
2026-Mar-23 01:56:59.977433
#54 16.24    Compiling simd-adler32 v0.3.8
2026-Mar-23 01:56:59.977433
#54 16.29    Compiling openssl v0.10.75
2026-Mar-23 01:56:59.977433
#54 16.40    Compiling tokio v1.49.0
2026-Mar-23 01:56:59.977433
#54 16.50    Compiling ring v0.17.14
2026-Mar-23 01:56:59.977433
#54 16.50    Compiling aws-lc-sys v0.38.0
2026-Mar-23 01:56:59.977433
#54 16.53    Compiling openssl-sys v0.9.111
2026-Mar-23 01:56:59.977433
#54 16.55    Compiling adler2 v2.0.1
2026-Mar-23 01:56:59.977433
#54 16.78    Compiling minimal-lexical v0.2.1
2026-Mar-23 01:56:59.977433
#54 16.83    Compiling hmac v0.12.1
2026-Mar-23 01:56:59.977433
#54 16.83    Compiling sha2 v0.10.9
2026-Mar-23 01:56:59.977433
#54 16.83    Compiling signature v1.6.4
2026-Mar-23 01:56:59.977433
#54 16.84    Compiling elliptic-curve v0.12.3
2026-Mar-23 01:56:59.977433
#54 16.84    Compiling thiserror v1.0.69
2026-Mar-23 01:56:59.977433
#54 16.86    Compiling foreign-types-shared v0.1.1
2026-Mar-23 01:56:59.977433
#54 16.94    Compiling miniz_oxide v0.8.9
2026-Mar-23 01:56:59.977433
#54 16.99    Compiling aws-types v1.3.14
2026-Mar-23 01:56:59.977433
#54 17.01    Compiling rfc6979 v0.3.1
2026-Mar-23 01:56:59.977433
#54 17.01    Compiling hashlink v0.10.0
2026-Mar-23 01:56:59.977433
#54 17.12    Compiling md-5 v0.10.6
2026-Mar-23 01:56:59.977433
#54 17.23    Compiling futures-util v0.3.32
2026-Mar-23 01:56:59.977433
#54 17.23    Compiling nom v7.1.3
2026-Mar-23 01:56:59.977433
#54 17.23    Compiling crossbeam-queue v0.3.12
2026-Mar-23 01:56:59.977433
#54 17.35    Compiling unicode-normalization v0.1.25
2026-Mar-23 01:56:59.977433
#54 17.40    Compiling brotli-decompressor v5.0.0
2026-Mar-23 01:56:59.977433
#54 17.58    Compiling ecdsa v0.14.8
2026-Mar-23 01:56:59.977433
#54 17.58    Compiling foreign-types v0.3.2
2026-Mar-23 01:56:59.977433
#54 17.58    Compiling webpki-roots v0.26.11
2026-Mar-23 01:56:59.977433
#54 17.68    Compiling libloading v0.8.9
2026-Mar-23 01:56:59.977433
#54 17.68    Compiling getrandom v0.3.4
2026-Mar-23 01:56:59.977433
#54 17.72    Compiling unicode-properties v0.1.4
2026-Mar-23 01:56:59.977433
#54 17.81    Compiling native-tls v0.2.18
2026-Mar-23 01:56:59.977433
#54 17.82    Compiling anyhow v1.0.102
2026-Mar-23 01:56:59.977433
#54 17.82    Compiling unicode-bidi v0.3.18
2026-Mar-23 01:56:59.977433
#54 17.91    Compiling ppv-lite86 v0.2.21
2026-Mar-23 01:56:59.977433
#54 18.05    Compiling bindgen v0.69.5
2026-Mar-23 01:56:59.977433
#54 18.08    Compiling p256 v0.11.1
2026-Mar-23 01:56:59.977433
#54 18.20    Compiling flate2 v1.1.9
2026-Mar-23 01:56:59.977433
#54 18.20    Compiling hkdf v0.12.4
2026-Mar-23 01:56:59.977433
#54 18.44    Compiling rand_chacha v0.3.1
2026-Mar-23 01:56:59.977433
#54 18.52    Compiling crypto-bigint v0.5.5
2026-Mar-23 01:56:59.977433
#54 18.58    Compiling stringprep v0.1.5
2026-Mar-23 01:56:59.977433
#54 18.63    Compiling cookie v0.18.1
2026-Mar-23 01:56:59.977433
#54 18.66    Compiling lazy_static v1.5.0
2026-Mar-23 01:56:59.977433
#54 18.69    Compiling dotenvy v0.15.7
2026-Mar-23 01:56:59.977433
#54 18.81    Compiling atoi v2.0.0
2026-Mar-23 01:56:59.977433
#54 18.84    Compiling brotli v8.0.2
2026-Mar-23 01:56:59.977433
#54 18.91    Compiling fastrand v2.3.0
2026-Mar-23 01:56:59.977433
#54 18.97    Compiling lazycell v1.3.0
2026-Mar-23 01:56:59.977433
#54 19.00    Compiling adler v1.0.2
2026-Mar-23 01:56:59.977433
#54 19.21    Compiling rand v0.8.5
2026-Mar-23 01:56:59.977433
#54 19.23    Compiling regex-automata v0.4.14
2026-Mar-23 01:56:59.977433
#54 19.24    Compiling byteorder v1.5.0
2026-Mar-23 01:56:59.977433
#54 19.25    Compiling compression-core v0.4.31
2026-Mar-23 01:56:59.977433
#54 19.25    Compiling rustc-hash v1.1.0
2026-Mar-23 01:56:59.977433
#54 19.37    Compiling whoami v1.6.1
2026-Mar-23 01:56:59.977433
#54 19.44    Compiling gzip-header v1.0.0
2026-Mar-23 01:56:59.977433
#54 19.53    Compiling cexpr v0.6.0
2026-Mar-23 01:56:59.977433
#54 19.53    Compiling miniz_oxide v0.7.4
2026-Mar-23 01:56:59.977433
#54 19.72    Compiling fslock v0.2.1
2026-Mar-23 01:56:59.977433
#54 19.99    Compiling synstructure v0.13.2
2026-Mar-23 01:56:59.977433
#54 20.08    Compiling ahash v0.8.12
2026-Mar-23 01:56:59.977433
#54 20.13    Compiling litrs v1.0.0
2026-Mar-23 01:56:59.977433
#54 20.13    Compiling ucd-trie v0.1.7
2026-Mar-23 01:56:59.977433
#54 20.18    Compiling psl-types v2.0.11
2026-Mar-23 01:56:59.977433
#54 20.19    Compiling radium v0.7.0
2026-Mar-23 01:56:59.977433
#54 20.31    Compiling heck v0.5.0
2026-Mar-23 01:56:59.977433
#54 20.31    Compiling paste v1.0.15
2026-Mar-23 01:56:59.977433
#54 20.37    Compiling num-bigint v0.4.6
2026-Mar-23 01:56:59.977433
#54 20.39    Compiling mime v0.3.17
2026-Mar-23 01:56:59.977433
#54 20.44    Compiling pest v2.8.6
2026-Mar-23 01:56:59.977433
#54 20.56    Compiling rand_core v0.9.5
2026-Mar-23 01:56:59.977433
#54 20.79    Compiling arc-swap v1.8.2
2026-Mar-23 01:56:59.977433
#54 20.94    Compiling iri-string v0.7.10
2026-Mar-23 01:56:59.977433
#54 21.02    Compiling heck v0.4.1
2026-Mar-23 01:57:00.082891
#54 21.08    Compiling sha1_smol v1.0.1
2026-Mar-23 01:57:00.082891
#54 21.11    Compiling tap v1.0.1
2026-Mar-23 01:57:00.082891
#54 ...
2026-Mar-23 01:57:00.082891
2026-Mar-23 01:57:00.082891
#53 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-23 01:57:00.082891
#53 21.04    Compiling serde_path_to_error v0.1.20
2026-Mar-23 01:57:00.082891
#53 21.06    Compiling sha1 v0.10.6
2026-Mar-23 01:57:00.082891
#53 21.10    Compiling proc-macro-error v1.0.4
2026-Mar-23 01:57:00.082891
#53 21.10    Compiling bitvec v1.0.1
2026-Mar-23 01:57:00.208842
#53 21.23    Compiling if_chain v1.0.3
2026-Mar-23 01:57:00.323792
#53 21.31    Compiling matchit v0.8.4
2026-Mar-23 01:57:00.433961
#53 21.46    Compiling
2026-Mar-23 01:57:00.608883
unicode-id-start v1.4.0
2026-Mar-23 01:57:00.608883
#53 21.52    Compiling urlencoding v2.1.3
2026-Mar-23 01:57:00.608883
#53 21.53    Compiling bit-vec v0.6.3
2026-Mar-23 01:57:00.608883
#53 21.63    Compiling data-encoding v2.10.0
2026-Mar-23 01:57:00.832882
#53 21.79    Compiling bit-set v0.5.3
2026-Mar-23 01:57:00.832882
#53 21.82    Compiling serde_derive v1.0.228
2026-Mar-23 01:57:00.832882
#53 21.83    Compiling tokio-macros v2.6.0
2026-Mar-23 01:57:00.832882
#53 21.85    Compiling zerofrom-derive v0.1.6
2026-Mar-23 01:57:00.973418
#53 21.88    Compiling yoke-derive v0.8.1
2026-Mar-23 01:57:00.973418
#53 21.92    Compiling tracing-attributes v0.1.31
2026-Mar-23 01:57:00.973418
#53 21.99    Compiling futures-macro v0.3.32
2026-Mar-23 01:57:01.105841
#53 22.03    Compiling zerovec-derive v0.11.2
2026-Mar-23 01:57:01.105841
#53 22.06    Compiling displaydoc v0.2.5
2026-Mar-23 01:57:01.219804
#53 22.17    Compiling thiserror-impl v2.0.18
2026-Mar-23 01:57:01.219804
#53 22.24    Compiling openssl-macros v0.1.1
2026-Mar-23 01:57:01.474840
#53 22.26    Compiling thiserror-impl v1.0.69
2026-Mar-23 01:57:01.474840
#53 22.37    Compiling async-trait v0.1.89
2026-Mar-23 01:57:01.474840
#53 22.42    Compiling proc-macro-rules-macros v0.4.0
2026-Mar-23 01:57:01.474840
#53 22.50    Compiling strum_macros v0.25.3
2026-Mar-23 01:57:01.597098
#53 22.62    Compiling pest_derive v2.8.6
2026-Mar-23 01:57:01.713094
#53 22.64    Compiling pin-project-internal v1.1.10
2026-Mar-23 01:57:01.713094
#53 22.68    Compiling async-lock v3.4.2
2026-Mar-23 01:57:02.050640
#53 23.01    Compiling dlv-list v0.5.2
2026-Mar-23 01:57:02.050640
#53 23.07    Compiling
2026-Mar-23 01:57:02.155488
crossbeam-channel v0.5.15
2026-Mar-23 01:57:02.256118
#53 23.25    Compiling crossbeam-epoch v0.9.18
2026-Mar-23 01:57:02.256118
#53 23.26    Compiling toml_write v0.1.2
2026-Mar-23 01:57:02.256118
#53 23.28    Compiling
2026-Mar-23 01:57:02.398099
cooked-waker v5.0.0
2026-Mar-23 01:57:02.398099
#53 23.29    Compiling winnow v0.7.14
2026-Mar-23 01:57:02.525972
#53 23.50    Compiling extractor v0.1.0 (/app/crates/extractor)
2026-Mar-23 01:57:02.627069
#53 23.65    Compiling proc-macro-rules v0.4.0
2026-Mar-23 01:57:02.746829
#53 23.67    Compiling deno_core_icudata v0.0.73
2026-Mar-23 01:57:02.746829
#53 23.67    Compiling tagptr v0.2.0
2026-Mar-23 01:57:02.746829
#53 23.70    Compiling static_assertions v1.1.0
2026-Mar-23 01:57:02.746829
#53 23.70    Compiling ordered-multimap v0.7.3
2026-Mar-23 01:57:02.746829
#53 23.77    Compiling async-stream-impl v0.3.6
2026-Mar-23 01:57:02.856549
#53 23.81    Compiling hashlink v0.8.4
2026-Mar-23 01:57:02.856549
#53 23.86    Compiling raw-cpuid v11.6.0
2026-Mar-23 01:57:02.955518
#53 23.92    Compiling lru v0.12.5
2026-Mar-23 01:57:02.955518
#53 23.96    Compiling num_cpus v1.17.0
2026-Mar-23 01:57:02.955518
#53 23.98    Compiling zerofrom v0.1.6
2026-Mar-23 01:57:03.167199
#53 24.16    Compiling unicode-segmentation v1.12.0
2026-Mar-23 01:57:03.167199
#53 24.19    Compiling arraydeque v0.5.1
2026-Mar-23 01:57:03.274182
#53 24.26    Compiling base64 v0.21.7
2026-Mar-23 01:57:03.283576
#53 24.29    Compiling tracing v0.1.44
2026-Mar-23 01:57:03.388996
#53 24.33    Compiling pin-project v1.1.10
2026-Mar-23 01:57:03.397232
#53 24.33    Compiling async-stream v0.3.6
2026-Mar-23 01:57:03.397232
#53 24.40    Compiling yoke v0.8.1
2026-Mar-23 01:57:03.397232
#53 24.41    Compiling utoipa-gen v4.3.1
2026-Mar-23 01:57:03.489390
#53 24.41    Compiling sharded-slab v0.1.7
2026-Mar-23 01:57:03.504180
#53 24.43    Compiling rust-ini v0.20.0
2026-Mar-23 01:57:03.504180
#53 24.52    Compiling pem v3.0.6
2026-Mar-23 01:57:03.615934
#53 24.56    Compiling tracing-log v0.2.0
2026-Mar-23 01:57:03.625261
#53 24.64    Compiling zerovec v0.11.5
2026-Mar-23 01:57:03.723061
#53 24.67    Compiling zerotrie v0.2.3
2026-Mar-23 01:57:03.723061
#53 24.70    Compiling spinning_top v0.3.0
2026-Mar-23 01:57:03.723061
#53 24.75    Compiling convert_case v0.6.0
2026-Mar-23 01:57:03.854353
#53 24.88    Compiling axum-core v0.5.6
2026-Mar-23 01:57:03.975901
#53 25.00    Compiling tower v0.4.13
2026-Mar-23 01:57:04.119562
#53 25.06    Compiling yaml-rust2 v0.8.1
2026-Mar-23 01:57:04.119562
#53 25.06    Compiling thread_local v1.1.9
2026-Mar-23 01:57:04.119562
#53 25.06    Compiling no-std-compat v0.4.1
2026-Mar-23 01:57:04.119562
#53 25.15    Compiling simple_asn1 v0.6.4
2026-Mar-23 01:57:04.220106
#53 25.16    Compiling strum v0.25.0
2026-Mar-23 01:57:04.220106
#53 25.16    Compiling pathdiff v0.2.3
2026-Mar-23 01:57:04.220106
#53 25.23    Compiling nonzero_ext v0.3.0
2026-Mar-23 01:57:04.220106
#53 25.25    Compiling web-time v1.1.0
2026-Mar-23 01:57:04.381801
#53 25.28    Compiling nu-ansi-term v0.50.3
2026-Mar-23 01:57:04.381801
#53 25.31    Compiling futures-timer v3.0.3
2026-Mar-23 01:57:04.381801
#53 25.34    Compiling deno_ops v0.176.0
2026-Mar-23 01:57:04.483599
#53 25.49    Compiling tinystr v0.8.2
2026-Mar-23 01:57:04.483599
#53 25.49    Compiling
2026-Mar-23 01:57:04.625742
potential_utf v0.1.4
2026-Mar-23 01:57:04.838182
#53 25.65    Compiling icu_collections v2.1.1
2026-Mar-23 01:57:05.011500
#53 25.91    Compiling icu_locale_core v2.1.1
2026-Mar-23 01:57:05.241986
#53 26.27    Compiling matchers v0.2.0
2026-Mar-23 01:57:06.213044
#53 27.24    Compiling icu_provider v2.1.1
2026-Mar-23 01:57:06.364278
#53 27.39    Compiling smallvec v1.15.1
2026-Mar-23 01:57:06.473192
#53 27.43    Compiling either v1.15.0
2026-Mar-23 01:57:06.473192
#53 27.43    Compiling serde_urlencoded v0.7.1
2026-Mar-23 01:57:06.473192
#53 27.46    Compiling debugid v0.8.0
2026-Mar-23 01:57:06.473192
#53 27.46    Compiling json5 v0.4.1
2026-Mar-23 01:57:06.473192
#53 27.46    Compiling toml_datetime v0.6.11
2026-Mar-23 01:57:06.473192
#53 27.50    Compiling serde_spanned v0.6.9
2026-Mar-23 01:57:06.705984
#53 27.58    Compiling bincode v1.3.3
2026-Mar-23 01:57:06.715810
2026-Mar-23 01:57:06.992057
#53 28.01    Compiling icu_properties v2.1.2
2026-Mar-23 01:57:07.165094
#53 28.07    Compiling parking_lot v0.12.5
2026-Mar-23 01:57:07.165094
#53 28.10    Compiling which v4.4.2
2026-Mar-23 01:57:07.165094
#53 28.11    Compiling bytes-utils v0.1.4
2026-Mar-23 01:57:07.165094
#53 28.19    Compiling icu_normalizer v2.1.1
2026-Mar-23 01:57:07.300617
#53 28.27    Compiling itertools v0.12.1
2026-Mar-23 01:57:07.310553
#53 28.27    Compiling which v6.0.3
2026-Mar-23 01:57:07.496881
#53 28.34    Compiling itertools v0.13.0
2026-Mar-23 01:57:07.496881
#53 28.40    Compiling futures-executor v0.3.32
2026-Mar-23 01:57:07.496881
#53 28.52    Compiling futures-intrusive v0.5.0
2026-Mar-23 01:57:07.596105
#53 28.57    Compiling dashmap v6.1.0
2026-Mar-23 01:57:07.817511
#53 28.62    Compiling moka v0.12.13
2026-Mar-23 01:57:07.817511
#53 28.62    Compiling toml_edit v0.22.27
2026-Mar-23 01:57:07.817511
#53 28.68    Compiling quanta v0.12.6
2026-Mar-23 01:57:07.817511
#53 28.71    Compiling ron v0.8.1
2026-Mar-23 01:57:07.817511
#53 28.71    Compiling tracing-subscriber v0.3.22
2026-Mar-23 01:57:07.817511
#53 28.84    Compiling futures v0.3.32
2026-Mar-23 01:57:08.056382
#53 29.08    Compiling rustls-webpki v0.103.9
2026-Mar-23 01:57:08.601554
#53 29.48    Compiling idna_adapter v1.2.1
2026-Mar-23 01:57:08.633647
#53 29.66    Compiling idna v1.1.0
2026-Mar-23 01:57:08.869611
#53 29.90    Compiling utoipa v4.2.3
2026-Mar-23 01:57:09.067247
#53 30.09    Compiling url v2.5.8
2026-Mar-23 01:57:09.230538
#53 30.15    Compiling governor v0.8.1
2026-Mar-23 01:57:09.971431
#53 31.00    Compiling toml v0.8.23
2026-Mar-23 01:57:10.126333
#53 31.07    Compiling publicsuffix v2.3.0
2026-Mar-23 01:57:10.458231
#53 ...
2026-Mar-23 01:57:10.458231
2026-Mar-23 01:57:10.458231
#54 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-23 01:57:10.458231
#54 21.36    Compiling regex-lite v0.1.9
2026-Mar-23 01:57:10.458231
#54 21.36    Compiling outref v0.1.0
2026-Mar-23 01:57:10.458231
#54 21.43    Compiling document-features v0.2.12
2026-Mar-23 01:57:10.458231
#54 21.53    Compiling rand_chacha v0.9.0
2026-Mar-23 01:57:10.458231
#54 21.67    Compiling tokio-stream v0.1.18
2026-Mar-23 01:57:10.458231
#54 21.67    Compiling wyz v0.5.1
2026-Mar-23 01:57:10.458231
#54 21.76    Compiling simd-abstraction v0.7.1
2026-Mar-23 01:57:10.458231
#54 21.91    Compiling memoffset v0.9.1
2026-Mar-23 01:57:10.458231
#54 21.95    Compiling encoding_rs v0.8.35
2026-Mar-23 01:57:10.458231
#54 21.96    Compiling portable-atomic v1.13.1
2026-Mar-23 01:57:10.458231
#54 22.15    Compiling funty v2.0.0
2026-Mar-23 01:57:10.458231
#54 22.19    Compiling xmlparser v0.13.6
2026-Mar-23 01:57:10.458231
#54 22.24    Compiling hashbrown v0.14.5
2026-Mar-23 01:57:10.458231
#54 22.32    Compiling rand v0.9.2
2026-Mar-23 01:57:10.458231
#54 22.34    Compiling sha1 v0.10.6
2026-Mar-23 01:57:10.458231
#54 22.36    Compiling event-listener-strategy v0.5.4
2026-Mar-23 01:57:10.458231
#54 22.57    Compiling base64-simd v0.7.0
2026-Mar-23 01:57:10.458231
#54 22.64    Compiling serde_path_to_error v0.1.20
2026-Mar-23 01:57:10.458231
#54 22.75    Compiling if_chain v1.0.3
2026-Mar-23 01:57:10.458231
#54 22.75    Compiling bit-vec v0.6.3
2026-Mar-23 01:57:10.458231
#54 22.81    Compiling aws-smithy-xml v0.60.15
2026-Mar-23 01:57:10.458231
#54 22.82    Compiling matchit v0.8.4
2026-Mar-23 01:57:10.458231
#54 22.98    Compiling regex v1.12.3
2026-Mar-23 01:57:10.458231
#54 23.01    Compiling bitvec v1.0.1
2026-Mar-23 01:57:10.458231
#54 23.13    Compiling unicode-id-start v1.4.0
2026-Mar-23 01:57:10.458231
#54 23.17    Compiling data-encoding v2.10.0
2026-Mar-23 01:57:10.458231
#54 23.20    Compiling urlencoding v2.1.3
2026-Mar-23 01:57:10.458231
#54 23.44    Compiling pest_meta v2.8.6
2026-Mar-23 01:57:10.458231
#54 23.57    Compiling bit-set v0.5.3
2026-Mar-23 01:57:10.458231
#54 23.61    Compiling async-lock v3.4.2
2026-Mar-23 01:57:10.458231
#54 23.66    Compiling crossbeam-channel v0.5.15
2026-Mar-23 01:57:10.458231
#54 23.69    Compiling serde_derive v1.0.228
2026-Mar-23 01:57:10.458231
#54 23.71    Compiling tokio-macros v2.6.0
2026-Mar-23 01:57:10.458231
#54 23.71    Compiling zerofrom-derive v0.1.6
2026-Mar-23 01:57:10.458231
#54 23.74    Compiling yoke-derive v0.8.1
2026-Mar-23 01:57:10.458231
#54 23.83    Compiling tracing-attributes v0.1.31
2026-Mar-23 01:57:10.458231
#54 23.83    Compiling zerovec-derive v0.11.2
2026-Mar-23 01:57:10.458231
#54 23.89    Compiling futures-macro v0.3.32
2026-Mar-23 01:57:10.458231
#54 24.01    Compiling displaydoc v0.2.5
2026-Mar-23 01:57:10.458231
#54 24.01    Compiling thiserror-impl v2.0.18
2026-Mar-23 01:57:10.458231
#54 24.10    Compiling openssl-macros v0.1.1
2026-Mar-23 01:57:10.458231
#54 24.10    Compiling thiserror-impl v1.0.69
2026-Mar-23 01:57:10.458231
#54 24.10    Compiling async-trait v0.1.89
2026-Mar-23 01:57:10.458231
#54 24.17    Compiling strum_macros v0.25.3
2026-Mar-23 01:57:10.458231
#54 24.32    Compiling proc-macro-rules-macros v0.4.0
2026-Mar-23 01:57:10.458231
#54 24.72    Compiling pest_generator v2.8.6
2026-Mar-23 01:57:10.458231
#54 24.83    Compiling pin-project-internal v1.1.10
2026-Mar-23 01:57:10.458231
#54 24.85    Compiling crossbeam-epoch v0.9.18
2026-Mar-23 01:57:10.458231
#54 24.99    Compiling compression-codecs v0.4.37
2026-Mar-23 01:57:10.458231
#54 25.30    Compiling cooked-waker v5.0.0
2026-Mar-23 01:57:10.458231
#54 25.43    Compiling tagptr v0.2.0
2026-Mar-23 01:57:10.458231
#54 25.43    Compiling static_assertions v1.1.0
2026-Mar-23 01:57:10.458231
#54 25.53    Compiling deno_core_icudata v0.0.73
2026-Mar-23 01:57:10.458231
#54 25.63    Compiling extractor v0.1.0 (/app/crates/extractor)
2026-Mar-23 01:57:10.458231
#54 25.67    Compiling async-stream-impl v0.3.6
2026-Mar-23 01:57:10.458231
#54 25.69    Compiling lru v0.12.5
2026-Mar-23 01:57:10.458231
#54 25.73    Compiling num_cpus v1.17.0
2026-Mar-23 01:57:10.458231
#54 25.89    Compiling tracing-log v0.2.0
2026-Mar-23 01:57:10.458231
#54 26.05    Compiling thread_local v1.1.9
2026-Mar-23 01:57:10.458231
#54 26.12    Compiling tracing v0.1.44
2026-Mar-23 01:57:10.458231
#54 26.18    Compiling nu-ansi-term v0.50.3
2026-Mar-23 01:57:10.458231
#54 26.18    Compiling sharded-slab v0.1.7
2026-Mar-23 01:57:10.458231
#54 26.29    Compiling proc-macro-rules v0.4.0
2026-Mar-23 01:57:10.458231
#54 26.46    Compiling axum-core v0.5.6
2026-Mar-23 01:57:10.458231
#54 26.51    Compiling tower v0.4.13
2026-Mar-23 01:57:10.458231
#54 26.59    Compiling pin-project v1.1.10
2026-Mar-23 01:57:10.458231
#54 26.73    Compiling pest_derive v2.8.6
2026-Mar-23 01:57:10.458231
#54 26.74    Compiling async-stream v0.3.6
2026-Mar-23 01:57:10.458231
#54 26.80    Compiling strum v0.25.0
2026-Mar-23 01:57:10.458231
#54 26.92    Compiling zerofrom v0.1.6
2026-Mar-23 01:57:10.458231
#54 27.14    Compiling deno_ops v0.176.0
2026-Mar-23 01:57:10.458231
#54 27.20    Compiling yoke v0.8.1
2026-Mar-23 01:57:10.458231
#54 27.43    Compiling zerovec v0.11.5
2026-Mar-23 01:57:10.458231
#54 27.43    Compiling zerotrie v0.2.3
2026-Mar-23 01:57:10.458231
#54 27.86    Compiling rustls-webpki v0.103.9
2026-Mar-23 01:57:10.458231
#54 28.29    Compiling tinystr v0.8.2
2026-Mar-23 01:57:10.458231
#54 28.35    Compiling potential_utf v0.1.4
2026-Mar-23 01:57:10.458231
#54 28.39    Compiling either v1.15.0
2026-Mar-23 01:57:10.458231
#54 28.39    Compiling smallvec v1.15.1
2026-Mar-23 01:57:10.458231
#54 28.57    Compiling icu_locale_core v2.1.1
2026-Mar-23 01:57:10.458231
#54 28.61    Compiling icu_collections v2.1.1
2026-Mar-23 01:57:10.458231
#54 28.64    Compiling matchers v0.2.0
2026-Mar-23 01:57:10.458231
#54 28.68    Compiling itertools v0.12.1
2026-Mar-23 01:57:10.458231
#54 28.68    Compiling which v4.4.2
2026-Mar-23 01:57:10.458231
#54 28.71    Compiling which v6.0.3
2026-Mar-23 01:57:10.458231
#54 28.95    Compiling serde_urlencoded v0.7.1
2026-Mar-23 01:57:10.458231
#54 28.96    Compiling debugid v0.8.0
2026-Mar-23 01:57:10.458231
#54 28.96    Compiling bincode v1.3.3
2026-Mar-23 01:57:10.458231
#54 28.96    Compiling json5 v0.4.1
2026-Mar-23 01:57:10.458231
#54 29.41    Compiling bytes-utils v0.1.4
2026-Mar-23 01:57:10.458231
#54 29.42    Compiling itertools v0.13.0
2026-Mar-23 01:57:10.458231
#54 29.48    Compiling tracing-subscriber v0.3.22
2026-Mar-23 01:57:10.458231
#54 29.78    Compiling parking_lot v0.12.5
2026-Mar-23 01:57:10.458231
#54 29.82    Compiling dashmap v6.1.0
2026-Mar-23 01:57:10.458231
#54 30.15    Compiling icu_provider v2.1.1
2026-Mar-23 01:57:10.458231
#54 30.28    Compiling futures-intrusive v0.5.0
2026-Mar-23 01:57:10.458231
#54 30.45    Compiling icu_properties v2.1.2
2026-Mar-23 01:57:10.458231
#54 30.50    Compiling icu_normalizer v2.1.1
2026-Mar-23 01:57:10.458231
#54 31.17    Compiling rustls-webpki v0.101.7
2026-Mar-23 01:57:10.458231
#54 31.20    Compiling sct v0.7.1
2026-Mar-23 01:57:10.609957
#54 ...
2026-Mar-23 01:57:10.609957
2026-Mar-23 01:57:10.609957
#53 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-23 01:57:10.609957
#53 31.63    Compiling rustls-webpki v0.101.7
2026-Mar-23 01:57:10.609957
#53 31.63    Compiling sct v0.7.1
2026-Mar-23 01:57:10.609957
#53 31.63    Compiling
2026-Mar-23 01:57:10.761258
jsonwebtoken v9.3.1
2026-Mar-23 01:57:11.303971
#53 32.33    Compiling config v0.14.1
2026-Mar-23 01:57:11.513992
#53 32.54    Compiling cookie_store v0.22.1
2026-Mar-23 01:57:11.725092
#53 32.54    Compiling sourcemap v8.0.1
2026-Mar-23 01:57:14.140701
#53 35.17    Compiling sqlx-core v0.8.6
2026-Mar-23 01:57:15.867895
#53 36.89    Compiling tokio-util v0.7.18
2026-Mar-23 01:57:16.022884
#53 36.89    Compiling aws-smithy-async v1.2.14
2026-Mar-23 01:57:16.022884
#53 36.89    Compiling tower v0.5.3
2026-Mar-23 01:57:16.022884
#53 36.89    Compiling tokio-rustls v0.24.1
2026-Mar-23 01:57:16.022884
#53 36.89    Compiling async-compression v0.4.41
2026-Mar-23 01:57:16.022884
#53 36.89    Compiling tokio-native-tls v0.3.1
2026-Mar-23 01:57:16.022884
#53 36.89    Compiling deno_unsync v0.4.4
2026-Mar-23 01:57:16.192827
#53 37.22    Compiling v8 v0.101.0
2026-Mar-23 01:57:16.205183
2026-Mar-23 01:57:16.443302
#53 37.40    Compiling aws-smithy-types v1.4.6
2026-Mar-23 01:57:16.443302
#53 37.40    Compiling h2 v0.4.13
2026-Mar-23 01:57:16.443302
#53 37.40    Compiling h2 v0.3.27
2026-Mar-23 01:57:16.443302
#53 37.40    Compiling combine v4.6.7
2026-Mar-23 01:57:16.443302
#53 37.47    Compiling tower-http v0.6.8
2026-Mar-23 01:57:16.779022
#53 37.81    Compiling sqlx-postgres v0.8.6
2026-Mar-23 01:57:17.129298
#53 38.16    Compiling aws-smithy-runtime-api v1.11.6
2026-Mar-23 01:57:17.305946
#53 38.16    Compiling aws-smithy-eventstream v0.60.20
2026-Mar-23 01:57:17.305946
#53 38.16    Compiling aws-smithy-json v0.62.5
2026-Mar-23 01:57:17.305946
#53 38.17    Compiling aws-smithy-query v0.60.15
2026-Mar-23 01:57:17.305946
#53 38.18    Compiling aws-smithy-json v0.61.9
2026-Mar-23 01:57:18.327213
#53 39.35    Compiling aws-smithy-http v0.63.6
2026-Mar-23 01:57:18.478477
#53 39.35    Compiling aws-credential-types v1.2.14
2026-Mar-23 01:57:18.478477
#53 39.35    Compiling aws-smithy-observability v0.2.6
2026-Mar-23 01:57:18.478477
#53 39.35    Compiling aws-smithy-http v0.62.6
2026-Mar-23 01:57:18.606578
#53 39.63    Compiling crc-fast v1.6.0
2026-Mar-23 01:57:18.759469
#53 39.65    Compiling aws-sigv4 v1.4.2
2026-Mar-23 01:57:19.640966
#53 ...
2026-Mar-23 01:57:19.640966
2026-Mar-23 01:57:19.640966
#54 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-23 01:57:19.640966
#54 32.40    Compiling idna_adapter v1.2.1
2026-Mar-23 01:57:19.640966
#54 32.62    Compiling idna v1.1.0
2026-Mar-23 01:57:19.640966
#54 33.02    Compiling futures-executor v0.3.32
2026-Mar-23 01:57:19.640966
#54 33.02    Compiling moka v0.12.13
2026-Mar-23 01:57:19.640966
#54 33.21    Compiling url v2.5.8
2026-Mar-23 01:57:19.640966
#54 33.21    Compiling publicsuffix v2.3.0
2026-Mar-23 01:57:19.640966
#54 33.79    Compiling futures v0.3.32
2026-Mar-23 01:57:19.640966
#54 34.32    Compiling sqlx-core v0.8.6
2026-Mar-23 01:57:19.640966
#54 34.98    Compiling cookie_store v0.22.1
2026-Mar-23 01:57:19.640966
#54 35.08    Compiling sourcemap v8.0.1
2026-Mar-23 01:57:19.640966
#54 36.57    Compiling v8 v0.101.0
2026-Mar-23 01:57:19.640966
#54 37.27    Compiling sqlx-postgres v0.8.6
2026-Mar-23 01:57:19.640966
#54 37.35    Compiling tokio-util v0.7.18
2026-Mar-23 01:57:19.640966
#54 37.35    Compiling aws-smithy-async v1.2.14
2026-Mar-23 01:57:19.640966
#54 37.35    Compiling tower v0.5.3
2026-Mar-23 01:57:19.640966
#54 37.36    Compiling tokio-rustls v0.24.1
2026-Mar-23 01:57:19.640966
#54 37.37    Compiling async-compression v0.4.41
2026-Mar-23 01:57:19.640966
#54 37.39    Compiling tokio-native-tls v0.3.1
2026-Mar-23 01:57:19.640966
#54 37.40    Compiling deno_unsync v0.4.4
2026-Mar-23 01:57:19.640966
#54 37.83    Compiling aws-smithy-types v1.4.6
2026-Mar-23 01:57:19.640966
#54 37.83    Compiling h2 v0.4.13
2026-Mar-23 01:57:19.640966
#54 37.83    Compiling h2 v0.3.27
2026-Mar-23 01:57:19.640966
#54 37.83    Compiling combine v4.6.7
2026-Mar-23 01:57:19.640966
#54 37.89    Compiling tower-http v0.6.8
2026-Mar-23 01:57:19.640966
#54 38.61    Compiling aws-smithy-runtime-api v1.11.6
2026-Mar-23 01:57:19.640966
#54 38.62    Compiling aws-smithy-eventstream v0.60.20
2026-Mar-23 01:57:19.640966
#54 38.62    Compiling aws-smithy-json v0.62.5
2026-Mar-23 01:57:19.640966
#54 38.62    Compiling aws-smithy-query v0.60.15
2026-Mar-23 01:57:19.640966
#54 38.62    Compiling aws-smithy-json v0.61.9
2026-Mar-23 01:57:19.640966
#54 39.80    Compiling aws-smithy-http v0.63.6
2026-Mar-23 01:57:19.640966
#54 39.80    Compiling aws-credential-types v1.2.14
2026-Mar-23 01:57:19.640966
#54 39.80    Compiling aws-smithy-observability v0.2.6
2026-Mar-23 01:57:19.640966
#54 39.80    Compiling aws-smithy-http v0.62.6
2026-Mar-23 01:57:19.640966
#54 40.09    Compiling aws-sigv4 v1.4.2
2026-Mar-23 01:57:19.825013
#54 40.87    Compiling sqlx-macros-core v0.8.6
2026-Mar-23 01:57:20.225659
#54 ...
2026-Mar-23 01:57:20.225659
2026-Mar-23 01:57:20.225659
#53 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-23 01:57:20.225659
#53 41.25    Compiling hyper v0.14.32
2026-Mar-23 01:57:20.435616
#53 41.30    Compiling sqlx-macros-core v0.8.6
2026-Mar-23 01:57:20.435616
#53 41.31    Compiling redis v0.27.6
2026-Mar-23 01:57:20.471165
#53 41.50    Compiling
2026-Mar-23 01:57:20.621721
hyper v1.8.1
2026-Mar-23 01:57:20.848341
#53 41.87    Compiling
2026-Mar-23 01:57:20.958976
aws-smithy-checksums v0.63.12
2026-Mar-23 01:57:21.429668
#53 42.46    Compiling sqlx-macros v0.8.6
2026-Mar-23 01:57:21.748725
#53 42.77    Compiling hyper-util v0.1.20
2026-Mar-23 01:57:22.911789
#53 43.94    Compiling hyper-tls v0.6.0
2026-Mar-23 01:57:22.911789
#53 43.94    Compiling axum v0.8.8
2026-Mar-23 01:57:23.489384
#53 44.37    Compiling hyper-rustls v0.24.2
2026-Mar-23 01:57:25.488889
#53 46.52    Compiling queue v0.1.0 (/app/crates/queue)
2026-Mar-23 01:57:26.435984
#53 47.46    Compiling serde_v8 v0.209.0
2026-Mar-23 01:57:26.735853
#53 47.76    Compiling deno_core v0.300.0
2026-Mar-23 01:57:30.036093
#53 ...
2026-Mar-23 01:57:30.036093
2026-Mar-23 01:57:30.036093
#54 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-23 01:57:30.036093
#54 41.78    Compiling crc-fast v1.6.0
2026-Mar-23 01:57:30.036093
#54 41.79    Compiling hyper v0.14.32
2026-Mar-23 01:57:30.036093
#54 41.93    Compiling redis v0.27.6
2026-Mar-23 01:57:30.036093
#54 41.94    Compiling sqlx-macros v0.8.6
2026-Mar-23 01:57:30.036093
#54 42.00    Compiling hyper v1.8.1
2026-Mar-23 01:57:30.036093
#54 42.14    Compiling serde_v8 v0.209.0
2026-Mar-23 01:57:30.036093
#54 42.50    Compiling deno_core v0.300.0
2026-Mar-23 01:57:30.036093
#54 43.19    Compiling hyper-util v0.1.20
2026-Mar-23 01:57:30.036093
#54 43.84    Compiling aws-smithy-checksums v0.63.12
2026-Mar-23 01:57:30.036093
#54 44.32    Compiling hyper-tls v0.6.0
2026-Mar-23 01:57:30.036093
#54 44.32    Compiling axum v0.8.8
2026-Mar-23 01:57:30.036093
#54 44.78    Compiling hyper-rustls v0.24.2
2026-Mar-23 01:57:30.036093
#54 46.93    Compiling queue v0.1.0 (/app/crates/queue)
2026-Mar-23 01:57:33.420942
#54 ...
2026-Mar-23 01:57:33.420942
2026-Mar-23 01:57:33.420942
#53 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-23 01:57:33.420942
#53 54.45    Compiling tokio-rustls v0.26.4
2026-Mar-23 01:57:33.576935
#53 54.60    Compiling hyper-rustls v0.27.7
2026-Mar-23 01:57:33.718543
#53 54.75    Compiling aws-smithy-http-client v1.1.12
2026-Mar-23 01:57:33.868881
#53 54.75    Compiling reqwest v0.12.28
2026-Mar-23 01:57:35.029886
#53 56.06    Compiling aws-smithy-runtime v1.10.3
2026-Mar-23 01:57:37.029024
#53 58.06    Compiling aws-runtime v1.7.2
2026-Mar-23 01:57:37.704632
#53 58.73    Compiling sqlx v0.8.6
2026-Mar-23 01:57:37.807155
#53 58.83    Compiling proxy v0.1.0 (/app/crates/proxy)
2026-Mar-23 01:57:37.957548
#53 58.83    Compiling job-system v0.1.0 (/app/crates/job-system)
2026-Mar-23 01:57:37.967351
#53 58.99    Compiling aws-sdk-ssooidc v1.98.0
2026-Mar-23 01:57:37.967351
#53 58.99    Compiling aws-sdk-sts v1.100.0
2026-Mar-23 01:57:38.115389
#53 58.99    Compiling aws-sdk-sso v1.96.0
2026-Mar-23 01:57:38.115389
#53 58.99    Compiling aws-sdk-s3 v1.119.0
2026-Mar-23 01:57:40.020980
#53 61.05    Compiling aws-config v1.8.15
2026-Mar-23 01:57:40.357428
#53 61.38    Compiling muxer v0.1.0 (/app/crates/muxer)
2026-Mar-23 01:57:42.635883
#53 ...
2026-Mar-23 01:57:42.635883
2026-Mar-23 01:57:42.635883
#54 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-23 01:57:42.635883
#54 55.57    Compiling tokio-rustls v0.26.4
2026-Mar-23 01:57:42.635883
#54 55.75    Compiling hyper-rustls v0.27.7
2026-Mar-23 01:57:42.635883
#54 55.89    Compiling aws-smithy-http-client v1.1.12
2026-Mar-23 01:57:42.635883
#54 55.89    Compiling reqwest v0.12.28
2026-Mar-23 01:57:42.635883
#54 57.23    Compiling aws-smithy-runtime v1.10.3
2026-Mar-23 01:57:42.635883
#54 59.38    Compiling aws-runtime v1.7.2
2026-Mar-23 01:57:42.635883
#54 60.38    Compiling sqlx v0.8.6
2026-Mar-23 01:57:42.635883
#54 60.45    Compiling aws-sdk-sso v1.96.0
2026-Mar-23 01:57:42.635883
#54 60.45    Compiling aws-sdk-sts v1.100.0
2026-Mar-23 01:57:42.635883
#54 60.45    Compiling aws-sdk-ssooidc v1.98.0
2026-Mar-23 01:57:42.635883
#54 60.45    Compiling aws-sdk-s3 v1.119.0
2026-Mar-23 01:57:42.635883
#54 60.52    Compiling proxy v0.1.0 (/app/crates/proxy)
2026-Mar-23 01:57:42.635883
#54 60.52    Compiling job-system v0.1.0 (/app/crates/job-system)
2026-Mar-23 01:57:42.635883
#54 62.67    Compiling aws-config v1.8.15
2026-Mar-23 01:57:42.635883
#54 63.53    Compiling muxer v0.1.0 (/app/crates/muxer)
2026-Mar-23 01:57:56.023752
#54 ...
2026-Mar-23 01:57:56.023752
2026-Mar-23 01:57:56.023752
#53 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-23 01:57:56.023752
#53 77.05    Compiling object-store v0.1.0 (/app/crates/object-store)
2026-Mar-23 01:58:05.665592
#53 ...
2026-Mar-23 01:58:05.665592
2026-Mar-23 01:58:05.665592
#54 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-23 01:58:05.665592
#54 78.01    Compiling object-store v0.1.0 (/app/crates/object-store)
2026-Mar-23 01:58:31.481268
#54 ...
2026-Mar-23 01:58:31.481268
2026-Mar-23 01:58:31.481268
#53 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-23 01:58:31.481268
#53 112.5    Compiling api v0.1.0 (/app/crates/api)
2026-Mar-23 01:58:41.026275
#53 ...
2026-Mar-23 01:58:41.026275
2026-Mar-23 01:58:41.026275
#54 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-23 01:58:41.026275
#54 113.5    Compiling worker v0.1.0 (/app/crates/worker)
2026-Mar-23 02:00:15.860071
#54 216.9     Finished `release` profile [optimized] target(s) in 3m 36s
2026-Mar-23 02:00:16.078854
#54 DONE 217.1s
2026-Mar-23 02:00:16.078854
2026-Mar-23 02:00:16.078854
#53 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-23 02:00:16.200310
#53 ...
2026-Mar-23 02:00:16.200310
2026-Mar-23 02:00:16.200310
#55 [worker runtime 5/8] RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Mar-23 02:00:16.200310
#55 CACHED
2026-Mar-23 02:00:16.200310
2026-Mar-23 02:00:16.200310
#56 [worker runtime 3/8] RUN apt-get update && apt-get install -y     ca-certificates     curl     libssl3     && rm -rf /var/lib/apt/lists/*
2026-Mar-23 02:00:16.200310
#56 CACHED
2026-Mar-23 02:00:16.200310
2026-Mar-23 02:00:16.200310
#57 [worker runtime 2/8] WORKDIR /app
2026-Mar-23 02:00:16.200310
#57 CACHED
2026-Mar-23 02:00:16.200310
2026-Mar-23 02:00:16.200310
#58 [worker runtime 4/8] RUN set -eux;     arch="$(dpkg --print-architecture)";     case "$arch" in       amd64) ytdlp_asset="yt-dlp_linux" ;;       arm64) ytdlp_asset="yt-dlp_linux_aarch64" ;;       *) echo "Unsupported architecture: $arch" >&2; exit 1 ;;     esac;     curl -fL "https://github.com/yt-dlp/yt-dlp/releases/latest/download/${ytdlp_asset}" -o /usr/local/bin/yt-dlp;     chmod +x /usr/local/bin/yt-dlp;     /usr/local/bin/yt-dlp --version
2026-Mar-23 02:00:16.200310
#58 CACHED
2026-Mar-23 02:00:16.200310
2026-Mar-23 02:00:16.200310
#59 [worker runtime 6/8] COPY --from=builder /app/target/release/mux-worker /usr/local/bin/
2026-Mar-23 02:00:16.368787
#59 CACHED
2026-Mar-23 02:00:16.368787
2026-Mar-23 02:00:16.368787
#60 [worker runtime 7/8] COPY --from=builder /app/crates/api/app-migrations /app/app-migrations
2026-Mar-23 02:00:16.368787
#60 DONE 0.0s
2026-Mar-23 02:00:16.368787
2026-Mar-23 02:00:16.368787
#61 [worker runtime 8/8] RUN mkdir -p /app/extractors /app/proxy-state && chown -R appuser:appuser /app
2026-Mar-23 02:00:16.448494
#61 DONE 0.2s
2026-Mar-23 02:00:16.448494
2026-Mar-23 02:00:16.448494
#53 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-23 02:00:16.617375
#53 ...
2026-Mar-23 02:00:16.617375
2026-Mar-23 02:00:16.617375
#62 [worker] exporting to image
2026-Mar-23 02:00:16.617375
#62 exporting layers 0.0s done
2026-Mar-23 02:00:16.617375
#62 exporting manifest sha256:c55dfb575cf31ae66de85a5eeea477ea52af789d7fbc182ea685971d305d2ee1 done
2026-Mar-23 02:00:16.617375
#62 exporting config sha256:8b3b681f73115e4fbbb8d3a6e7c6be206dd52e6e8ffa13a4cf09f78943882908 done
2026-Mar-23 02:00:16.617375
#62 exporting attestation manifest sha256:593f95b0ba5182702d79fd39bb536915e783c672d1c30d5007ba254c6addff08 done
2026-Mar-23 02:00:16.617375
#62 exporting manifest list sha256:b9d0341d360a34e34348afaa2b34d4b7f9cd79097439f54f4744daf887ec1aef done
2026-Mar-23 02:00:16.617375
#62 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_worker:3bf1238b2350ae4f3732e6368c475a81f97af531 done
2026-Mar-23 02:00:16.617375
#62 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_worker:3bf1238b2350ae4f3732e6368c475a81f97af531 0.0s done
2026-Mar-23 02:00:16.617375
#62 DONE 0.1s
2026-Mar-23 02:00:16.617375
2026-Mar-23 02:00:16.617375
#63 [worker] resolving provenance for metadata file
2026-Mar-23 02:00:16.773338
#63 DONE 0.0s
2026-Mar-23 02:00:16.773338
2026-Mar-23 02:00:16.773338
#53 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-23 02:00:30.062863
========================================
2026-Mar-23 02:00:30.081459
Deployment failed: Command execution failed (exit code 255): docker exec row00kkoog4wc0cc8c400888 bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/build-time.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/row00kkoog4wc0cc8c400888 -f /artifacts/row00kkoog4wc0cc8c400888/docker/docker-compose.server.yml build --pull --build-arg COOLIFY_URL --build-arg COOLIFY_FQDN --build-arg SERVICE_URL_API --build-arg BETTER_AUTH_TRUSTED_ORIGINS --build-arg WHOP_WEBHOOK_SECRET --build-arg SERVICE_FQDN_FRONTEND --build-arg POSTGRES_PASSWORD --build-arg ORIGIN --build-arg SERVICE_FQDN_API --build-arg BETTER_AUTH_SECRET --build-arg GOOGLE_CLIENT_ID --build-arg VITE_API_URL --build-arg WHOP_PLAN_ID --build-arg GOOGLE_CLIENT_SECRET --build-arg S3_REGION --build-arg S3_ENDPOINT --build-arg S3_BUCKET_NAME --build-arg S3_ACCESS_KEY_ID --build-arg S3_SECRET_ACCESS_KEY --build-arg MUX_ARTIFACT_TTL_SECS --build-arg MUX_CLEANUP_INTERVAL_SECS --build-arg MUX_FILE_TICKET_TTL_SECS --build-arg PROXY_QUARANTINE_TTL_SECS --build-arg ADMIN_EMAILS --build-arg SHARED_PROXY_POSTGRES_PASSWORD --build-arg SERVICE_URL_FRONTEND --build-arg MUX_WORKER_CONCURRENCY --build-arg COOLIFY_BUILD_SECRETS_HASH=c8a6670d0bb115c25e090e96cdbe00d1a4714ec95abb323c725abeed4171a126'
2026-Mar-23 02:00:30.081459
Error: #1 [internal] load local bake definitions
2026-Mar-23 02:00:30.081459
#1 reading from stdin 6.33kB done
2026-Mar-23 02:00:30.081459
#1 DONE 0.0s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#2 [worker internal] load build definition from Dockerfile.worker
2026-Mar-23 02:00:30.081459
#2 transferring dockerfile: 4.16kB done
2026-Mar-23 02:00:30.081459
#2 DONE 0.0s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#3 [frontend internal] load build definition from Dockerfile.frontend
2026-Mar-23 02:00:30.081459
#3 transferring dockerfile: 2.69kB done
2026-Mar-23 02:00:30.081459
#3 DONE 0.0s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#4 [api internal] load build definition from Dockerfile.api
2026-Mar-23 02:00:30.081459
#4 transferring dockerfile: 5.30kB done
2026-Mar-23 02:00:30.081459
#4 DONE 0.0s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#5 [api internal] load metadata for docker.io/library/node:22-alpine
2026-Mar-23 02:00:30.081459
#5 ...
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#6 [api internal] load metadata for docker.io/library/rust:1.91-bookworm
2026-Mar-23 02:00:30.081459
#6 DONE 1.1s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#7 [worker internal] load metadata for docker.io/library/debian:bookworm-slim
2026-Mar-23 02:00:30.081459
#7 DONE 1.1s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#5 [api internal] load metadata for docker.io/library/node:22-alpine
2026-Mar-23 02:00:30.081459
#5 DONE 1.6s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#8 [worker internal] load .dockerignore
2026-Mar-23 02:00:30.081459
#8 transferring context: 341B done
2026-Mar-23 02:00:30.081459
#8 DONE 0.0s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#9 [worker runtime 1/8] FROM docker.io/library/debian:bookworm-slim@sha256:f06537653ac770703bc45b4b113475bd402f451e85223f0f2837acbf89ab020a
2026-Mar-23 02:00:30.081459
#9 resolve docker.io/library/debian:bookworm-slim@sha256:f06537653ac770703bc45b4b113475bd402f451e85223f0f2837acbf89ab020a 0.0s done
2026-Mar-23 02:00:30.081459
#9 DONE 0.0s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#10 [worker builder 1/8] FROM docker.io/library/node:22-alpine@sha256:8094c002d08262dba12645a3b4a15cd6cd627d30bc782f53229a2ec13ee22a00
2026-Mar-23 02:00:30.081459
#10 resolve docker.io/library/node:22-alpine@sha256:8094c002d08262dba12645a3b4a15cd6cd627d30bc782f53229a2ec13ee22a00 0.0s done
2026-Mar-23 02:00:30.081459
#10 DONE 0.0s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#11 [worker builder  1/11] FROM docker.io/library/rust:1.91-bookworm@sha256:c1e5f19e773b7878c3f7a805dd00a495e747acbdc76fb2337a4ebf0418896b33
2026-Mar-23 02:00:30.081459
#11 resolve docker.io/library/rust:1.91-bookworm@sha256:c1e5f19e773b7878c3f7a805dd00a495e747acbdc76fb2337a4ebf0418896b33 0.0s done
2026-Mar-23 02:00:30.081459
#11 DONE 0.0s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#12 [api internal] load build context
2026-Mar-23 02:00:30.081459
#12 transferring context: 1.03MB 0.0s done
2026-Mar-23 02:00:30.081459
#12 DONE 0.0s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#13 [api builder  3/11] RUN apt-get update && apt-get install -y     pkg-config     libssl-dev     && rm -rf /var/lib/apt/lists/*
2026-Mar-23 02:00:30.081459
#13 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#14 [api builder  2/11] WORKDIR /app
2026-Mar-23 02:00:30.081459
#14 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#15 [api builder  4/11] COPY Cargo.toml ./
2026-Mar-23 02:00:30.081459
#15 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#16 [api builder  5/11] COPY Cargo.lock ./
2026-Mar-23 02:00:30.081459
#16 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#17 [frontend internal] load build context
2026-Mar-23 02:00:30.081459
#17 transferring context: 2.07MB 0.0s done
2026-Mar-23 02:00:30.081459
#17 DONE 0.0s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#18 [worker js-builder  3/12] RUN npm install -g pnpm
2026-Mar-23 02:00:30.081459
#18 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#19 [worker js-builder  4/12] COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Mar-23 02:00:30.081459
#19 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#20 [worker js-builder  5/12] COPY packages/api-client/package.json ./packages/api-client/
2026-Mar-23 02:00:30.081459
#20 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#21 [worker js-builder  8/11] COPY apps/injector/ ./apps/injector/
2026-Mar-23 02:00:30.081459
#21 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#22 [worker js-builder  9/11] COPY extractors/ ./extractors/
2026-Mar-23 02:00:30.081459
#22 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#23 [worker js-builder 10/11] RUN pnpm install --frozen-lockfile
2026-Mar-23 02:00:30.081459
#23 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#24 [worker js-builder  7/11] COPY packages/ ./packages/
2026-Mar-23 02:00:30.081459
#24 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#25 [worker js-builder  6/12] COPY apps/injector/package.json ./apps/injector/
2026-Mar-23 02:00:30.081459
#25 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#26 [worker builder 2/8] WORKDIR /app
2026-Mar-23 02:00:30.081459
#26 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#27 [worker js-builder 11/11] RUN mkdir -p extractors/dist &&     npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js &&     npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-23 02:00:30.081459
#27 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#28 [worker builder  6/11] COPY crates/ ./crates/
2026-Mar-23 02:00:30.081459
#28 DONE 0.0s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#20 [api js-builder  5/12] COPY packages/api-client/package.json ./packages/api-client/
2026-Mar-23 02:00:30.081459
#20 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#29 [api js-builder  9/12] RUN pnpm install --frozen-lockfile
2026-Mar-23 02:00:30.081459
#29 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#30 [api js-builder 10/12] RUN pnpm --filter @downloadtool/injector build
2026-Mar-23 02:00:30.081459
#30 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#31 [api js-builder  8/12] COPY packages/ ./packages/
2026-Mar-23 02:00:30.081459
#31 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#32 [api js-builder 11/12] COPY extractors/ ./extractors/
2026-Mar-23 02:00:30.081459
#32 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#25 [api js-builder  6/12] COPY apps/injector/package.json ./apps/injector/
2026-Mar-23 02:00:30.081459
#25 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#19 [api js-builder  4/12] COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Mar-23 02:00:30.081459
#19 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#33 [api js-builder  7/12] COPY apps/injector/ ./apps/injector/
2026-Mar-23 02:00:30.081459
#33 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#18 [api js-builder  3/12] RUN npm install -g pnpm
2026-Mar-23 02:00:30.081459
#18 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#34 [api js-builder 12/12] RUN mkdir -p extractors/dist &&     npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js &&     npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-23 02:00:30.081459
#34 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#35 [frontend builder 3/8] COPY frontend/ ./
2026-Mar-23 02:00:30.081459
#35 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#36 [frontend runtime 3/6] COPY --from=builder /app/build ./build
2026-Mar-23 02:00:30.081459
#36 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#37 [frontend runtime 5/6] COPY --from=builder /app/package-lock.json ./
2026-Mar-23 02:00:30.081459
#37 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#38 [frontend builder 8/8] RUN node build-docker.mjs
2026-Mar-23 02:00:30.081459
#38 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#39 [frontend builder 4/8] COPY config/ /config/
2026-Mar-23 02:00:30.081459
#39 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#40 [frontend builder 7/8] RUN npm run paraglide:compile
2026-Mar-23 02:00:30.081459
#40 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#41 [frontend runtime 4/6] COPY --from=builder /app/package.json ./
2026-Mar-23 02:00:30.081459
#41 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#42 [frontend builder 6/8] RUN test -n "https://api.snapvie.com" || (echo "VITE_API_URL build arg is required" && exit 1)
2026-Mar-23 02:00:30.081459
#42 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#26 [frontend builder 2/8] WORKDIR /app
2026-Mar-23 02:00:30.081459
#26 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#43 [frontend builder 5/8] RUN npm install
2026-Mar-23 02:00:30.081459
#43 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#44 [frontend runtime 6/6] RUN npm ci --omit=dev
2026-Mar-23 02:00:30.081459
#44 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#45 [worker builder  7/11] COPY config/ ./config/
2026-Mar-23 02:00:30.081459
#45 DONE 0.0s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#46 [api builder  8/11] COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Mar-23 02:00:30.081459
#46 DONE 0.0s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#47 [frontend] exporting to image
2026-Mar-23 02:00:30.081459
#47 exporting layers done
2026-Mar-23 02:00:30.081459
#47 exporting manifest sha256:07f87ed34eb703cc583b12e79c6e3ed86f8adc4a28124050baa4ca60a1dca2d4 done
2026-Mar-23 02:00:30.081459
#47 exporting config sha256:c7dd330d38a9cb52bf25f78c0dd365b313f4ae4d3763819de3c6eaadf9964f52 done
2026-Mar-23 02:00:30.081459
#47 exporting attestation manifest sha256:b2dc3bcb35515df001a83544ba90e59ebf53a44ca431ff92d04f9e12c5db8cc7 0.0s done
2026-Mar-23 02:00:30.081459
#47 exporting manifest list sha256:9cdedd3caae5d7750f64cde8bef0dc67cf97e152e4bcc8f5c080c1bc020955d7 done
2026-Mar-23 02:00:30.081459
#47 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:3bf1238b2350ae4f3732e6368c475a81f97af531 done
2026-Mar-23 02:00:30.081459
#47 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:3bf1238b2350ae4f3732e6368c475a81f97af531 done
2026-Mar-23 02:00:30.081459
#47 DONE 0.1s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#48 [worker builder  8/10] COPY extractors/ ./extractors/
2026-Mar-23 02:00:30.081459
#48 DONE 0.0s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#49 [api builder  9/11] COPY extractors/ ./extractors/
2026-Mar-23 02:00:30.081459
#49 DONE 0.0s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#50 [api builder 10/11] COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-23 02:00:30.081459
#50 DONE 0.0s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#51 [worker builder  9/10] COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-23 02:00:30.081459
#51 DONE 0.0s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#52 [frontend] resolving provenance for metadata file
2026-Mar-23 02:00:30.081459
#52 DONE 0.0s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#53 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-23 02:00:30.081459
#53 0.270     Updating crates.io index
2026-Mar-23 02:00:30.081459
#53 5.284  Downloading crates ...
2026-Mar-23 02:00:30.081459
#53 5.581   Downloaded adler v1.0.2
2026-Mar-23 02:00:30.081459
#53 5.605   Downloaded alloc-no-stdlib v2.0.4
2026-Mar-23 02:00:30.081459
#53 5.646   Downloaded atomic-waker v1.1.2
2026-Mar-23 02:00:30.081459
#53 5.693   Downloaded crunchy v0.2.4
2026-Mar-23 02:00:30.081459
#53 5.702   Downloaded cfg-if v1.0.4
2026-Mar-23 02:00:30.081459
#53 5.710   Downloaded alloc-stdlib v0.2.2
2026-Mar-23 02:00:30.081459
#53 5.713   Downloaded adler2 v2.0.1
2026-Mar-23 02:00:30.081459
#53 5.738   Downloaded ahash v0.8.12
2026-Mar-23 02:00:30.081459
#53 5.750   Downloaded block-buffer v0.10.4
2026-Mar-23 02:00:30.081459
#53 5.757   Downloaded aws-smithy-observability v0.2.6
2026-Mar-23 02:00:30.081459
#53 5.761   Downloaded cooked-waker v5.0.0
2026-Mar-23 02:00:30.081459
#53 5.763   Downloaded atoi v2.0.0
2026-Mar-23 02:00:30.081459
#53 5.766   Downloaded base64-simd v0.7.0
2026-Mar-23 02:00:30.081459
#53 5.773   Downloaded aws-smithy-query v0.60.15
2026-Mar-23 02:00:30.081459
#53 5.805   Downloaded base16ct v0.1.1
2026-Mar-23 02:00:30.081459
#53 5.809   Downloaded cfg_aliases v0.2.1
2026-Mar-23 02:00:30.081459
#53 5.813   Downloaded crc-catalog v2.4.0
2026-Mar-23 02:00:30.081459
#53 5.816   Downloaded debugid v0.8.0
2026-Mar-23 02:00:30.081459
#53 5.820   Downloaded base64-simd v0.8.0
2026-Mar-23 02:00:30.081459
#53 5.823   Downloaded bit-set v0.5.3
2026-Mar-23 02:00:30.081459
#53 5.857   Downloaded cpufeatures v0.2.17
2026-Mar-23 02:00:30.081459
#53 5.862   Downloaded const-random v0.1.18
2026-Mar-23 02:00:30.081459
#53 5.865   Downloaded compression-core v0.4.31
2026-Mar-23 02:00:30.081459
#53 5.868   Downloaded async-stream-impl v0.3.6
2026-Mar-23 02:00:30.081459
#53 5.871   Downloaded async-stream v0.3.6
2026-Mar-23 02:00:30.081459
#53 5.875   Downloaded crypto-common v0.1.7
2026-Mar-23 02:00:30.081459
#53 5.878   Downloaded const-random-macro v0.1.16
2026-Mar-23 02:00:30.081459
#53 5.885   Downloaded crossbeam-queue v0.3.12
2026-Mar-23 02:00:30.081459
#53 5.889   Downloaded crc v3.4.0
2026-Mar-23 02:00:30.081459
#53 5.894   Downloaded deno_unsync v0.4.4
2026-Mar-23 02:00:30.081459
#53 5.919   Downloaded aws-smithy-checksums v0.63.12
2026-Mar-23 02:00:30.081459
#53 5.924   Downloaded data-encoding v2.10.0
2026-Mar-23 02:00:30.081459
#53 5.927   Downloaded compression-codecs v0.4.37
2026-Mar-23 02:00:30.081459
#53 5.933   Downloaded lru v0.12.5
2026-Mar-23 02:00:30.081459
#53 5.936   Downloaded axum-core v0.5.6
2026-Mar-23 02:00:30.081459
#53 5.940   Downloaded aws-smithy-http v0.62.6
2026-Mar-23 02:00:30.081459
#53 5.944   Downloaded event-listener-strategy v0.5.4
2026-Mar-23 02:00:30.081459
#53 5.947   Downloaded cexpr v0.6.0
2026-Mar-23 02:00:30.081459
#53 5.951   Downloaded aws-credential-types v1.2.14
2026-Mar-23 02:00:30.081459
#53 5.955   Downloaded aws-smithy-json v0.61.9
2026-Mar-23 02:00:30.081459
#53 5.958   Downloaded aws-smithy-eventstream v0.60.20
2026-Mar-23 02:00:30.081459
#53 5.973   Downloaded aws-smithy-xml v0.60.15
2026-Mar-23 02:00:30.081459
#53 5.977   Downloaded cmake v0.1.57
2026-Mar-23 02:00:30.081459
#53 5.981   Downloaded lru-slab v0.1.2
2026-Mar-23 02:00:30.081459
#53 5.984   Downloaded aws-smithy-http v0.63.6
2026-Mar-23 02:00:30.081459
#53 5.988   Downloaded aws-smithy-async v1.2.14
2026-Mar-23 02:00:30.081459
#53 5.993   Downloaded mime v0.3.17
2026-Mar-23 02:00:30.081459
#53 5.996   Downloaded foreign-types v0.3.2
2026-Mar-23 02:00:30.081459
#53 5.999   Downloaded errno v0.3.14
2026-Mar-23 02:00:30.081459
#53 6.003   Downloaded equivalent v1.0.2
2026-Mar-23 02:00:30.081459
#53 6.006   Downloaded byteorder v1.5.0
2026-Mar-23 02:00:30.081459
#53 6.009   Downloaded aws-types v1.3.14
2026-Mar-23 02:00:30.081459
#53 6.013   Downloaded aws-smithy-json v0.62.5
2026-Mar-23 02:00:30.081459
#53 6.016   Downloaded md-5 v0.10.6
2026-Mar-23 02:00:30.081459
#53 6.019   Downloaded matchers v0.2.0
2026-Mar-23 02:00:30.081459
#53 6.022   Downloaded async-lock v3.4.2
2026-Mar-23 02:00:30.081459
#53 6.026   Downloaded cookie_store v0.22.1
2026-Mar-23 02:00:30.081459
#53 6.031   Downloaded crypto-bigint v0.4.9
2026-Mar-23 02:00:30.081459
#53 6.040   Downloaded lock_api v0.4.14
2026-Mar-23 02:00:30.081459
#53 6.043   Downloaded no-std-compat v0.4.1
2026-Mar-23 02:00:30.081459
#53 6.045   Downloaded memoffset v0.9.1
2026-Mar-23 02:00:30.081459
#53 6.047   Downloaded litrs v1.0.0
2026-Mar-23 02:00:30.081459
#53 6.053   Downloaded litemap v0.8.1
2026-Mar-23 02:00:30.081459
#53 6.057   Downloaded dashmap v6.1.0
2026-Mar-23 02:00:30.081459
#53 6.062   Downloaded crossbeam-utils v0.8.21
2026-Mar-23 02:00:30.081459
#53 6.067   Downloaded crc32fast v1.5.0
2026-Mar-23 02:00:30.081459
#53 6.071   Downloaded cookie v0.18.1
2026-Mar-23 02:00:30.081459
#53 6.075   Downloaded bitflags v2.11.0
2026-Mar-23 02:00:30.081459
#53 6.083   Downloaded bincode v1.3.3
2026-Mar-23 02:00:30.081459
#53 6.087   Downloaded config v0.14.1
2026-Mar-23 02:00:30.081459
#53 6.096   Downloaded nonzero_ext v0.3.0
2026-Mar-23 02:00:30.081459
#53 6.104   Downloaded num-iter v0.1.45
2026-Mar-23 02:00:30.081459
#53 6.106   Downloaded futures-sink v0.3.32
2026-Mar-23 02:00:30.081459
#53 6.108   Downloaded nu-ansi-term v0.50.3
2026-Mar-23 02:00:30.081459
#53 6.113   Downloaded httpdate v1.0.3
2026-Mar-23 02:00:30.081459
#53 6.116   Downloaded crossbeam-channel v0.5.15
2026-Mar-23 02:00:30.081459
#53 6.122   Downloaded hex v0.4.3
2026-Mar-23 02:00:30.081459
#53 6.125   Downloaded funty v2.0.0
2026-Mar-23 02:00:30.081459
#53 6.127   Downloaded aws-smithy-types v1.4.6
2026-Mar-23 02:00:30.081459
#53 6.135   Downloaded crypto-bigint v0.5.5
2026-Mar-23 02:00:30.081459
#53 6.147   Downloaded openssl-macros v0.1.1
2026-Mar-23 02:00:30.081459
#53 6.150   Downloaded outref v0.5.2
2026-Mar-23 02:00:30.081459
#53 6.155   Downloaded ordered-multimap v0.7.3
2026-Mar-23 02:00:30.081459
#53 6.159   Downloaded lazycell v1.3.0
2026-Mar-23 02:00:30.081459
#53 6.162   Downloaded crc-fast v1.6.0
2026-Mar-23 02:00:30.081459
#53 6.174   Downloaded parking_lot v0.12.5
2026-Mar-23 02:00:30.081459
#53 6.178   Downloaded brotli-decompressor v5.0.0
2026-Mar-23 02:00:30.081459
#53 6.187   Downloaded heck v0.4.1
2026-Mar-23 02:00:30.081459
#53 6.189   Downloaded fslock v0.2.1
2026-Mar-23 02:00:30.081459
#53 6.193   Downloaded aws-config v1.8.15
2026-Mar-23 02:00:30.081459
#53 6.203   Downloaded percent-encoding v2.3.2
2026-Mar-23 02:00:30.081459
#53 6.206   Downloaded proc-macro-error-attr v1.0.4
2026-Mar-23 02:00:30.081459
#53 6.209   Downloaded pin-utils v0.1.0
2026-Mar-23 02:00:30.081459
#53 6.213   Downloaded proc-macro-rules-macros v0.4.0
2026-Mar-23 02:00:30.081459
#53 6.215   Downloaded proc-macro2 v1.0.106
2026-Mar-23 02:00:30.081459
#53 6.220   Downloaded proc-macro-rules v0.4.0
2026-Mar-23 02:00:30.081459
#53 6.222   Downloaded nom v7.1.3
2026-Mar-23 02:00:30.081459
#53 6.231   Downloaded mio v1.1.1
2026-Mar-23 02:00:30.081459
#53 6.241   Downloaded moka v0.12.13
2026-Mar-23 02:00:30.081459
#53 6.252   Downloaded deno_core v0.300.0
2026-Mar-23 02:00:30.081459
#53 6.266   Downloaded radium v0.7.0
2026-Mar-23 02:00:30.081459
#53 6.269   Downloaded quote v1.0.44
2026-Mar-23 02:00:30.081459
#53 6.274   Downloaded num-bigint-dig v0.8.6
2026-Mar-23 02:00:30.081459
#53 6.283   Downloaded rand_chacha v0.3.1
2026-Mar-23 02:00:30.081459
#53 6.287   Downloaded rand_core v0.9.5
2026-Mar-23 02:00:30.081459
#53 6.290   Downloaded rand_core v0.6.4
2026-Mar-23 02:00:30.081459
#53 6.292   Downloaded heck v0.5.0
2026-Mar-23 02:00:30.081459
#53 6.294   Downloaded idna_adapter v1.2.1
2026-Mar-23 02:00:30.081459
#53 6.296   Downloaded gzip-header v1.0.0
2026-Mar-23 02:00:30.081459
#53 6.298   Downloaded rand_chacha v0.9.0
2026-Mar-23 02:00:30.081459
#53 6.300   Downloaded home v0.5.12
2026-Mar-23 02:00:30.081459
#53 6.302   Downloaded futures-task v0.3.32
2026-Mar-23 02:00:30.081459
#53 6.304   Downloaded brotli v8.0.2
2026-Mar-23 02:00:30.081459
#53 6.320   Downloaded http-body v0.4.6
2026-Mar-23 02:00:30.081459
#53 6.323   Downloaded num-bigint v0.4.6
2026-Mar-23 02:00:30.081459
#53 6.330   Downloaded if_chain v1.0.3
2026-Mar-23 02:00:30.081459
#53 6.332   Downloaded quinn-udp v0.5.14
2026-Mar-23 02:00:30.081459
#53 6.335   Downloaded quanta v0.12.6
2026-Mar-23 02:00:30.081459
#53 6.339   Downloaded psl-types v2.0.11
2026-Mar-23 02:00:30.081459
#53 6.341   Downloaded ppv-lite86 v0.2.21
2026-Mar-23 02:00:30.081459
#53 6.343   Downloaded pkg-config v0.3.32
2026-Mar-23 02:00:30.081459
#53 6.346   Downloaded pkcs8 v0.10.2
2026-Mar-23 02:00:30.081459
#53 6.351   Downloaded pkcs8 v0.9.0
2026-Mar-23 02:00:30.081459
#53 6.358   Downloaded http-body v1.0.1
2026-Mar-23 02:00:30.081459
#53 6.365   Downloaded rustversion v1.0.22
2026-Mar-23 02:00:30.081459
#53 6.371   Downloaded simd-abstraction v0.7.1
2026-Mar-23 02:00:30.081459
#53 6.374   Downloaded deranged v0.5.8
2026-Mar-23 02:00:30.081459
#53 6.376   Downloaded signature v1.6.4
2026-Mar-23 02:00:30.081459
#53 6.379   Downloaded sharded-slab v0.1.7
2026-Mar-23 02:00:30.081459
#53 6.384   Downloaded parking v2.2.1
2026-Mar-23 02:00:30.081459
#53 6.386   Downloaded simple_asn1 v0.6.4
2026-Mar-23 02:00:30.081459
#53 6.388   Downloaded slab v0.4.12
2026-Mar-23 02:00:30.081459
#53 6.390   Downloaded simd-adler32 v0.3.8
2026-Mar-23 02:00:30.081459
#53 6.393   Downloaded sourcemap v8.0.1
2026-Mar-23 02:00:30.081459
#53 6.397   Downloaded dunce v1.0.5
2026-Mar-23 02:00:30.081459
#53 6.398   Downloaded document-features v0.2.12
2026-Mar-23 02:00:30.081459
#53 6.400   Downloaded signal-hook-registry v1.4.8
2026-Mar-23 02:00:30.081459
#53 6.402   Downloaded shlex v1.3.0
2026-Mar-23 02:00:30.081459
#53 6.404   Downloaded socket2 v0.6.2
2026-Mar-23 02:00:30.081459
#53 6.407   Downloaded sha2 v0.10.9
2026-Mar-23 02:00:30.081459
#53 6.411   Downloaded sha1_smol v1.0.1
2026-Mar-23 02:00:30.081459
#53 6.413   Downloaded pest v2.8.6
2026-Mar-23 02:00:30.081459
#53 6.421   Downloaded potential_utf v0.1.4
2026-Mar-23 02:00:30.081459
#53 6.423   Downloaded portable-atomic v1.13.1
2026-Mar-23 02:00:30.081459
#53 6.434   Downloaded spki v0.7.3
2026-Mar-23 02:00:30.081459
#53 6.438   Downloaded tinystr v0.8.2
2026-Mar-23 02:00:30.081459
#53 6.441   Downloaded tiny-keccak v2.0.2
2026-Mar-23 02:00:30.081459
#53 6.446   Downloaded fnv v1.0.7
2026-Mar-23 02:00:30.081459
#53 6.448   Downloaded tinyvec v1.10.0
2026-Mar-23 02:00:30.081459
#53 6.453   Downloaded tinyvec_macros v0.1.1
2026-Mar-23 02:00:30.081459
#53 6.454   Downloaded itoa v1.0.17
2026-Mar-23 02:00:30.081459
#53 6.456   Downloaded lazy_static v1.5.0
2026-Mar-23 02:00:30.081459
#53 6.459   Downloaded tokio-native-tls v0.3.1
2026-Mar-23 02:00:30.081459
#53 6.462   Downloaded tokio-rustls v0.26.4
2026-Mar-23 02:00:30.081459
#53 6.466   Downloaded tokio-rustls v0.24.1
2026-Mar-23 02:00:30.081459
#53 6.470   Downloaded fs_extra v1.3.0
2026-Mar-23 02:00:30.081459
#53 6.472   Downloaded ipnet v2.11.0
2026-Mar-23 02:00:30.081459
#53 6.475   Downloaded regex v1.12.3
2026-Mar-23 02:00:30.081459
#53 6.484   Downloaded toml_write v0.1.2
2026-Mar-23 02:00:30.081459
#53 6.487   Downloaded toml v0.8.23
2026-Mar-23 02:00:30.081459
#53 6.490   Downloaded regex-automata v0.4.14
2026-Mar-23 02:00:30.081459
#53 6.509   Downloaded redis v0.27.6
2026-Mar-23 02:00:30.081459
#53 6.521   Downloaded rsa v0.9.10
2026-Mar-23 02:00:30.081459
#53 6.530   Downloaded rustls-webpki v0.101.7
2026-Mar-23 02:00:30.081459
#53 6.554   Downloaded zerofrom v0.1.6
2026-Mar-23 02:00:30.081459
#53 6.556   Downloaded zeroize v1.8.2
2026-Mar-23 02:00:30.081459
#53 6.560   Downloaded utoipa v4.2.3
2026-Mar-23 02:00:30.081459
#53 6.564   Downloaded zerovec-derive v0.11.2
2026-Mar-23 02:00:30.081459
#53 6.567   Downloaded sqlx-mysql v0.8.6
2026-Mar-23 02:00:30.081459
#53 6.575   Downloaded sqlx v0.8.6
2026-Mar-23 02:00:30.081459
#53 6.594   Downloaded sqlx-sqlite v0.8.6
2026-Mar-23 02:00:30.081459
#53 6.600   Downloaded zmij v1.0.21
2026-Mar-23 02:00:30.081459
#53 6.603   Downloaded time v0.3.47
2026-Mar-23 02:00:30.081459
#53 6.619   Downloaded syn v2.0.117
2026-Mar-23 02:00:30.081459
#53 6.632   Downloaded syn v1.0.109
2026-Mar-23 02:00:30.081459
#53 6.644   Downloaded hyper-rustls v0.27.7
2026-Mar-23 02:00:30.081459
#53 6.647   Downloaded httparse v1.10.1
2026-Mar-23 02:00:30.081459
#53 6.651   Downloaded hmac v0.12.1
2026-Mar-23 02:00:30.081459
#53 6.654   Downloaded getrandom v0.2.17
2026-Mar-23 02:00:30.081459
#53 6.659   Downloaded rustls-native-certs v0.8.3
2026-Mar-23 02:00:30.081459
#53 6.663   Downloaded sqlx-postgres v0.8.6
2026-Mar-23 02:00:30.081459
#53 6.675   Downloaded ring v0.17.14
2026-Mar-23 02:00:30.081459
#53 6.723   Downloaded typenum v1.19.0
2026-Mar-23 02:00:30.081459
#53 6.726   Downloaded url v2.5.8
2026-Mar-23 02:00:30.081459
#53 6.729   Downloaded unicode-segmentation v1.12.0
2026-Mar-23 02:00:30.081459
#53 6.733   Downloaded unicode-normalization v0.1.25
2026-Mar-23 02:00:30.081459
#53 6.737   Downloaded winnow v0.7.14
2026-Mar-23 02:00:30.081459
#53 6.749   Downloaded spin v0.9.8
2026-Mar-23 02:00:30.081459
#53 6.752   Downloaded tracing-subscriber v0.3.22
2026-Mar-23 02:00:30.081459
#53 6.764   Downloaded webpki-roots v1.0.6
2026-Mar-23 02:00:30.081459
#53 6.769   Downloaded tokio-stream v0.1.18
2026-Mar-23 02:00:30.081459
#53 6.777   Downloaded vcpkg v0.2.15
2026-Mar-23 02:00:30.081459
#53 6.870   Downloaded yoke v0.8.1
2026-Mar-23 02:00:30.081459
#53 6.874   Downloaded tracing-attributes v0.1.31
2026-Mar-23 02:00:30.081459
#53 6.879   Downloaded zerocopy v0.8.39
2026-Mar-23 02:00:30.081459
#53 6.916   Downloaded indexmap v2.13.0
2026-Mar-23 02:00:30.081459
#53 6.923   Downloaded hyper-util v0.1.20
2026-Mar-23 02:00:30.081459
#53 6.933   Downloaded http v1.4.0
2026-Mar-23 02:00:30.081459
#53 6.938   Downloaded http v0.2.12
2026-Mar-23 02:00:30.081459
#53 6.943   Downloaded futures-intrusive v0.5.0
2026-Mar-23 02:00:30.081459
#53 6.950   Downloaded rustls-webpki v0.103.9
2026-Mar-23 02:00:30.081459
#53 6.954   Downloaded regex-lite v0.1.9
2026-Mar-23 02:00:30.081459
#53 6.958   Downloaded itertools v0.13.0
2026-Mar-23 02:00:30.081459
#53 6.969   Downloaded sqlx-core v0.8.6
2026-Mar-23 02:00:30.081459
#53 6.982   Downloaded tower v0.5.3
2026-Mar-23 02:00:30.081459
#53 6.998   Downloaded futures-util v0.3.32
2026-Mar-23 02:00:30.081459
#53 7.021   Downloaded tower-http v0.6.8
2026-Mar-23 02:00:30.081459
#53 7.033   Downloaded hyper v0.14.32
2026-Mar-23 02:00:30.081459
#53 7.044   Downloaded h2 v0.4.13
2026-Mar-23 02:00:30.081459
#53 7.054   Downloaded tower v0.4.13
2026-Mar-23 02:00:30.081459
#53 7.070   Downloaded libm v0.2.16
2026-Mar-23 02:00:30.081459
#53 7.089   Downloaded yaml-rust2 v0.8.1
2026-Mar-23 02:00:30.081459
#53 7.137   Downloaded zerovec v0.11.5
2026-Mar-23 02:00:30.081459
#53 7.144   Downloaded itertools v0.12.1
2026-Mar-23 02:00:30.081459
#53 7.154   Downloaded idna v1.1.0
2026-Mar-23 02:00:30.081459
#53 7.158   Downloaded icu_properties_data v2.1.2
2026-Mar-23 02:00:30.081459
#53 7.174   Downloaded regex-syntax v0.8.10
2026-Mar-23 02:00:30.081459
#53 7.183   Downloaded hyper v1.8.1
2026-Mar-23 02:00:30.081459
#53 7.191   Downloaded hkdf v0.12.4
2026-Mar-23 02:00:30.081459
#53 7.194   Downloaded hashbrown v0.16.1
2026-Mar-23 02:00:30.081459
#53 7.201   Downloaded h2 v0.3.27
2026-Mar-23 02:00:30.081459
#53 7.210   Downloaded hashbrown v0.15.5
2026-Mar-23 02:00:30.081459
#53 7.218   Downloaded tracing v0.1.44
2026-Mar-23 02:00:30.081459
#53 7.238   Downloaded hashbrown v0.14.5
2026-Mar-23 02:00:30.081459
#53 7.245   Downloaded governor v0.8.1
2026-Mar-23 02:00:30.081459
#53 7.251   Downloaded iri-string v0.7.10
2026-Mar-23 02:00:30.081459
#53 7.263   Downloaded prettyplease v0.2.37
2026-Mar-23 02:00:30.081459
#53 7.268   Downloaded icu_collections v2.1.1
2026-Mar-23 02:00:30.081459
#53 7.278   Downloaded flate2 v1.1.9
2026-Mar-23 02:00:30.081459
#53 7.286   Downloaded xmlparser v0.13.6
2026-Mar-23 02:00:30.081459
#53 7.290   Downloaded writeable v0.6.2
2026-Mar-23 02:00:30.081459
#53 7.293   Downloaded unicode-id-start v1.4.0
2026-Mar-23 02:00:30.081459
#53 7.297   Downloaded spinning_top v0.3.0
2026-Mar-23 02:00:30.081459
#53 7.299   Downloaded socket2 v0.5.10
2026-Mar-23 02:00:30.081459
#53 7.302   Downloaded smallvec v1.15.1
2026-Mar-23 02:00:30.081459
#53 7.305   Downloaded pest_meta v2.8.6
2026-Mar-23 02:00:30.081459
#53 7.308   Downloaded libc v0.2.182
2026-Mar-23 02:00:30.081459
#53 7.359   Downloaded icu_normalizer_data v2.1.1
2026-Mar-23 02:00:30.081459
#53 7.361   Downloaded icu_locale_core v2.1.1
2026-Mar-23 02:00:30.081459
#53 7.370   Downloaded icu_normalizer v2.1.1
2026-Mar-23 02:00:30.081459
#53 7.375   Downloaded flume v0.11.1
2026-Mar-23 02:00:30.081459
#53 7.381   Downloaded rustls-pki-types v1.14.0
2026-Mar-23 02:00:30.081459
#53 7.386   Downloaded pin-project v1.1.10
2026-Mar-23 02:00:30.081459
#53 7.406   Downloaded num-traits v0.2.19
2026-Mar-23 02:00:30.081459
#53 7.409   Downloaded getrandom v0.3.4
2026-Mar-23 02:00:30.081459
#53 7.415   Downloaded utoipa-gen v4.3.1
2026-Mar-23 02:00:30.081459
#53 7.423   Downloaded icu_provider v2.1.1
2026-Mar-23 02:00:30.081459
#53 7.426   Downloaded getrandom v0.4.1
2026-Mar-23 02:00:30.081459
#53 7.431   Downloaded jsonwebtoken v9.3.1
2026-Mar-23 02:00:30.081459
#53 7.437   Downloaded icu_properties v2.1.2
2026-Mar-23 02:00:30.081459
#53 7.440   Downloaded generic-array v0.14.7
2026-Mar-23 02:00:30.081459
#53 7.442   Downloaded elliptic-curve v0.12.3
2026-Mar-23 02:00:30.081459
#53 7.447   Downloaded futures v0.3.32
2026-Mar-23 02:00:30.081459
#53 7.454   Downloaded tokio-util v0.7.18
2026-Mar-23 02:00:30.081459
#53 7.465   Downloaded event-listener v5.4.1
2026-Mar-23 02:00:30.081459
#53 7.467   Downloaded tower-service v0.3.3
2026-Mar-23 02:00:30.081459
#53 7.468   Downloaded tower-layer v0.3.3
2026-Mar-23 02:00:30.081459
#53 7.470   Downloaded toml_edit v0.22.27
2026-Mar-23 02:00:30.081459
#53 7.476   Downloaded tokio v1.49.0
2026-Mar-23 02:00:30.081459
#53 7.532   Downloaded aws-lc-sys v0.38.0
2026-Mar-23 02:00:30.081459
#53 7.838   Downloaded futures-channel v0.3.32
2026-Mar-23 02:00:30.081459
#53 7.840   Downloaded serde_json v1.0.149
2026-Mar-23 02:00:30.081459
#53 7.849   Downloaded unicode-ident v1.0.24
2026-Mar-23 02:00:30.081459
#53 7.852   Downloaded unicode-bidi v0.3.18
2026-Mar-23 02:00:30.081459
#53 7.856   Downloaded tracing-core v0.1.36
2026-Mar-23 02:00:30.081459
#53 7.859   Downloaded signature v2.2.0
2026-Mar-23 02:00:30.081459
#53 7.861   Downloaded zerofrom-derive v0.1.6
2026-Mar-23 02:00:30.081459
#53 7.862   Downloaded wyz v0.5.1
2026-Mar-23 02:00:30.081459
#53 7.864   Downloaded which v6.0.3
2026-Mar-23 02:00:30.081459
#53 7.866   Downloaded uuid v1.21.0
2026-Mar-23 02:00:30.081459
#53 7.871   Downloaded untrusted v0.9.0
2026-Mar-23 02:00:30.081459
#53 7.873   Downloaded unicode-properties v0.1.4
2026-Mar-23 02:00:30.081459
#53 7.876   Downloaded ucd-trie v0.1.7
2026-Mar-23 02:00:30.081459
#53 7.877   Downloaded rustc_version v0.2.3
2026-Mar-23 02:00:30.081459
#53 7.878   Downloaded rustc-hash v2.1.1
2026-Mar-23 02:00:30.081459
#53 7.880   Downloaded proc-macro-error v1.0.4
2026-Mar-23 02:00:30.081459
#53 7.884   Downloaded glob v0.3.3
2026-Mar-23 02:00:30.081459
#53 7.886   Downloaded futures-executor v0.3.32
2026-Mar-23 02:00:30.081459
#53 7.888   Downloaded dotenvy v0.15.7
2026-Mar-23 02:00:30.081459
#53 7.892   Downloaded displaydoc v0.2.5
2026-Mar-23 02:00:30.081459
#53 7.896   Downloaded digest v0.10.7
2026-Mar-23 02:00:30.081459
#53 7.898   Downloaded yoke-derive v0.8.1
2026-Mar-23 02:00:30.081459
#53 7.899   Downloaded whoami v1.6.1
2026-Mar-23 02:00:30.081459
#53 7.902   Downloaded which v4.4.2
2026-Mar-23 02:00:30.081459
#53 7.903   Downloaded webpki-roots v0.26.11
2026-Mar-23 02:00:30.081459
#53 7.905   Downloaded web-time v1.1.0
2026-Mar-23 02:00:30.081459
#53 7.907   Downloaded encoding_rs v0.8.35
2026-Mar-23 02:00:30.081459
#53 7.927   Downloaded want v0.3.1
2026-Mar-23 02:00:30.081459
#53 7.928   Downloaded vsimd v0.8.0
2026-Mar-23 02:00:30.081459
#53 7.931   Downloaded version_check v0.9.5
2026-Mar-23 02:00:30.081459
#53 7.932   Downloaded utf8_iter v1.0.4
2026-Mar-23 02:00:30.081459
#53 7.933   Downloaded urlencoding v2.1.3
2026-Mar-23 02:00:30.081459
#53 7.935   Downloaded try-lock v0.2.5
2026-Mar-23 02:00:30.081459
#53 7.936   Downloaded tracing-log v0.2.0
2026-Mar-23 02:00:30.081459
#53 7.938   Downloaded serde v1.0.228
2026-Mar-23 02:00:30.081459
#53 7.942   Downloaded rustls v0.23.37
2026-Mar-23 02:00:30.081459
#53 7.956   Downloaded rustix v0.38.44
2026-Mar-23 02:00:30.081459
#53 7.995   Downloaded hyper-rustls v0.24.2
2026-Mar-23 02:00:30.081459
#53 8.000   Downloaded http-body-util v0.1.3
2026-Mar-23 02:00:30.081459
#53 8.002   Downloaded rustls v0.21.12
2026-Mar-23 02:00:30.081459
#53 8.018   Downloaded linux-raw-sys v0.4.15
2026-Mar-23 02:00:30.081459
#53 8.075   Downloaded reqwest v0.12.28
2026-Mar-23 02:00:30.081459
#53 8.081   Downloaded libloading v0.8.9
2026-Mar-23 02:00:30.081459
#53 8.084   Downloaded toml_datetime v0.6.11
2026-Mar-23 02:00:30.081459
#53 8.085   Downloaded raw-cpuid v11.6.0
2026-Mar-23 02:00:30.081459
#53 8.089   Downloaded hashlink v0.8.4
2026-Mar-23 02:00:30.081459
#53 8.092   Downloaded deno_core_icudata v0.0.73
2026-Mar-23 02:00:30.081459
#53 8.128   Downloaded tokio-macros v2.6.0
2026-Mar-23 02:00:30.081459
#53 8.129   Downloaded jobserver v0.1.34
2026-Mar-23 02:00:30.081459
#53 8.131   Downloaded hashlink v0.10.0
2026-Mar-23 02:00:30.081459
#53 8.134   Downloaded group v0.12.1
2026-Mar-23 02:00:30.081459
#53 8.136   Downloaded futures-timer v3.0.3
2026-Mar-23 02:00:30.081459
#53 8.138   Downloaded quinn-proto v0.11.13
2026-Mar-23 02:00:30.081459
#53 8.146   Downloaded foldhash v0.1.5
2026-Mar-23 02:00:30.081459
#53 8.148   Downloaded find-msvc-tools v0.1.9
2026-Mar-23 02:00:30.081459
#53 8.150   Downloaded rand v0.9.2
2026-Mar-23 02:00:30.081459
#53 8.154   Downloaded rand v0.8.5
2026-Mar-23 02:00:30.081459
#53 8.159   Downloaded quinn v0.11.9
2026-Mar-23 02:00:30.081459
#53 8.162   Downloaded publicsuffix v2.3.0
2026-Mar-23 02:00:30.081459
#53 8.165   Downloaded either v1.15.0
2026-Mar-23 02:00:30.081459
#53 8.167   Downloaded ecdsa v0.14.8
2026-Mar-23 02:00:30.081459
#53 8.169   Downloaded dlv-list v0.5.2
2026-Mar-23 02:00:30.081459
#53 8.171   Downloaded time-macros v0.2.27
2026-Mar-23 02:00:30.081459
#53 8.174   Downloaded time-core v0.1.8
2026-Mar-23 02:00:30.081459
#53 8.175   Downloaded thread_local v1.1.9
2026-Mar-23 02:00:30.081459
#53 8.177   Downloaded thiserror-impl v2.0.18
2026-Mar-23 02:00:30.081459
#53 8.179   Downloaded thiserror-impl v1.0.69
2026-Mar-23 02:00:30.081459
#53 8.181   Downloaded thiserror v2.0.18
2026-Mar-23 02:00:30.081459
#53 8.190   Downloaded thiserror v1.0.69
2026-Mar-23 02:00:30.081459
#53 8.197   Downloaded tap v1.0.1
2026-Mar-23 02:00:30.081459
#53 8.198   Downloaded tagptr v0.2.0
2026-Mar-23 02:00:30.081459
#53 8.200   Downloaded synstructure v0.13.2
2026-Mar-23 02:00:30.081459
#53 8.201   Downloaded sync_wrapper v1.0.2
2026-Mar-23 02:00:30.081459
#53 8.202   Downloaded subtle v2.6.1
2026-Mar-23 02:00:30.081459
#53 8.204   Downloaded strum_macros v0.25.3
2026-Mar-23 02:00:30.081459
#53 8.206   Downloaded strum v0.25.0
2026-Mar-23 02:00:30.081459
#53 8.207   Downloaded stringprep v0.1.5
2026-Mar-23 02:00:30.081459
#53 8.209   Downloaded static_assertions v1.1.0
2026-Mar-23 02:00:30.081459
#53 8.211   Downloaded stable_deref_trait v1.2.1
2026-Mar-23 02:00:30.081459
#53 8.212   Downloaded sqlx-macros-core v0.8.6
2026-Mar-23 02:00:30.081459
#53 8.215   Downloaded sqlx-macros v0.8.6
2026-Mar-23 02:00:30.081459
#53 8.216   Downloaded spki v0.6.0
2026-Mar-23 02:00:30.081459
#53 8.218   Downloaded sha1 v0.10.6
2026-Mar-23 02:00:30.081459
#53 8.220   Downloaded serde_v8 v0.209.0
2026-Mar-23 02:00:30.081459
#53 8.223   Downloaded serde_urlencoded v0.7.1
2026-Mar-23 02:00:30.081459
#53 8.226   Downloaded serde_spanned v0.6.9
2026-Mar-23 02:00:30.081459
#53 8.227   Downloaded serde_path_to_error v0.1.20
2026-Mar-23 02:00:30.081459
#53 8.229   Downloaded serde_derive v1.0.228
2026-Mar-23 02:00:30.081459
#53 8.233   Downloaded serde_core v1.0.228
2026-Mar-23 02:00:30.081459
#53 8.237   Downloaded semver-parser v0.7.0
2026-Mar-23 02:00:30.081459
#53 8.238   Downloaded semver v1.0.27
2026-Mar-23 02:00:30.081459
#53 8.241   Downloaded semver v0.9.0
2026-Mar-23 02:00:30.081459
#53 8.243   Downloaded sec1 v0.3.0
2026-Mar-23 02:00:30.081459
#53 8.245   Downloaded sct v0.7.1
2026-Mar-23 02:00:30.081459
#53 8.252   Downloaded scopeguard v1.2.0
2026-Mar-23 02:00:30.081459
#53 8.254   Downloaded ryu v1.0.23
2026-Mar-23 02:00:30.081459
#53 8.259   Downloaded p256 v0.11.1
2026-Mar-23 02:00:30.081459
#53 8.264   Downloaded openssl-sys v0.9.111
2026-Mar-23 02:00:30.081459
#53 8.271   Downloaded aws-sdk-s3 v1.119.0
2026-Mar-23 02:00:30.081459
#53 8.416   Downloaded rustc_version v0.4.1
2026-Mar-23 02:00:30.081459
#53 8.418   Downloaded rustc-hash v1.1.0
2026-Mar-23 02:00:30.081459
#53 8.419   Downloaded zerotrie v0.2.3
2026-Mar-23 02:00:30.081459
#53 8.423   Downloaded rust-ini v0.20.0
2026-Mar-23 02:00:30.081459
#53 8.425   Downloaded ron v0.8.1
2026-Mar-23 02:00:30.081459
#53 8.434   Downloaded rfc6979 v0.3.1
2026-Mar-23 02:00:30.081459
#53 8.436   Downloaded openssl v0.10.75
2026-Mar-23 02:00:30.081459
#53 8.449   Downloaded json5 v0.4.1
2026-Mar-23 02:00:30.081459
#53 8.451   Downloaded pkcs1 v0.7.5
2026-Mar-23 02:00:30.081459
#53 8.455   Downloaded pin-project-lite v0.2.16
2026-Mar-23 02:00:30.081459
#53 8.462   Downloaded pin-project-internal v1.1.10
2026-Mar-23 02:00:30.081459
#53 8.464   Downloaded pest_generator v2.8.6
2026-Mar-23 02:00:30.081459
#53 8.466   Downloaded pest_derive v2.8.6
2026-Mar-23 02:00:30.081459
#53 8.469   Downloaded parking_lot_core v0.9.12
2026-Mar-23 02:00:30.081459
#53 8.471   Downloaded bitvec v1.0.1
2026-Mar-23 02:00:30.081459
#53 8.497   Downloaded bindgen v0.69.5
2026-Mar-23 02:00:30.081459
#53 8.506   Downloaded aws-lc-rs v1.16.1
2026-Mar-23 02:00:30.081459
#53 8.518   Downloaded powerfmt v0.2.0
2026-Mar-23 02:00:30.081459
#53 8.520   Downloaded pem-rfc7468 v0.7.0
2026-Mar-23 02:00:30.081459
#53 8.522   Downloaded pem v3.0.6
2026-Mar-23 02:00:30.081459
#53 8.524   Downloaded paste v1.0.15
2026-Mar-23 02:00:30.081459
#53 8.528   Downloaded aws-sdk-sts v1.100.0
2026-Mar-23 02:00:30.081459
#53 8.544   Downloaded pathdiff v0.2.3
2026-Mar-23 02:00:30.081459
#53 8.545   Downloaded miniz_oxide v0.8.9
2026-Mar-23 02:00:30.081459
#53 8.548   Downloaded futures-core v0.3.32
2026-Mar-23 02:00:30.081459
#53 8.550   Downloaded minimal-lexical v0.2.1
2026-Mar-23 02:00:30.081459
#53 8.556   Downloaded hyper-tls v0.6.0
2026-Mar-23 02:00:30.081459
#53 8.558   Downloaded foreign-types-shared v0.1.1
2026-Mar-23 02:00:30.081459
#53 8.559   Downloaded axum v0.8.8
2026-Mar-23 02:00:30.081459
#53 8.571   Downloaded aho-corasick v1.1.4
2026-Mar-23 02:00:30.081459
#53 8.578   Downloaded combine v4.6.7
2026-Mar-23 02:00:30.081459
#53 8.587   Downloaded aws-smithy-runtime v1.10.3
2026-Mar-23 02:00:30.081459
#53 8.595   Downloaded once_cell v1.21.3
2026-Mar-23 02:00:30.081459
#53 8.598   Downloaded memchr v2.8.0
2026-Mar-23 02:00:30.081459
#53 8.606   Downloaded async-compression v0.4.41
2026-Mar-23 02:00:30.081459
#53 8.617   Downloaded outref v0.1.0
2026-Mar-23 02:00:30.081459
#53 8.618   Downloaded openssl-probe v0.2.1
2026-Mar-23 02:00:30.081459
#53 8.620   Downloaded miniz_oxide v0.7.4
2026-Mar-23 02:00:30.081459
#53 8.623   Downloaded der v0.7.10
2026-Mar-23 02:00:30.081459
#53 8.631   Downloaded cc v1.2.56
2026-Mar-23 02:00:30.081459
#53 8.635   Downloaded base64 v0.21.7
2026-Mar-23 02:00:30.081459
#53 8.641   Downloaded aws-sdk-ssooidc v1.98.0
2026-Mar-23 02:00:30.081459
#53 8.654   Downloaded aws-sdk-sso v1.96.0
2026-Mar-23 02:00:30.081459
#53 8.664   Downloaded aws-runtime v1.7.2
2026-Mar-23 02:00:30.081459
#53 8.670   Downloaded num_cpus v1.17.0
2026-Mar-23 02:00:30.081459
#53 8.674   Downloaded num-integer v0.1.46
2026-Mar-23 02:00:30.081459
#53 8.676   Downloaded crossbeam-epoch v0.9.18
2026-Mar-23 02:00:30.081459
#53 8.679   Downloaded aws-sigv4 v1.4.2
2026-Mar-23 02:00:30.081459
#53 8.748   Downloaded aws-smithy-runtime-api v1.11.6
2026-Mar-23 02:00:30.081459
#53 8.753   Downloaded aws-smithy-http-client v1.1.12
2026-Mar-23 02:00:30.081459
#53 8.758   Downloaded num-conv v0.2.0
2026-Mar-23 02:00:30.081459
#53 8.759   Downloaded log v0.4.29
2026-Mar-23 02:00:30.081459
#53 8.762   Downloaded futures-macro v0.3.32
2026-Mar-23 02:00:30.081459
#53 8.763   Downloaded der v0.6.1
2026-Mar-23 02:00:30.081459
#53 8.771   Downloaded clang-sys v1.8.1
2026-Mar-23 02:00:30.081459
#53 8.774   Downloaded bytes v1.11.1
2026-Mar-23 02:00:30.081459
#53 8.779   Downloaded base64 v0.22.1
2026-Mar-23 02:00:30.081459
#53 8.785   Downloaded native-tls v0.2.18
2026-Mar-23 02:00:30.081459
#53 8.787   Downloaded matchit v0.8.4
2026-Mar-23 02:00:30.081459
#53 8.789   Downloaded futures-io v0.3.32
2026-Mar-23 02:00:30.081459
#53 8.791   Downloaded form_urlencoded v1.2.2
2026-Mar-23 02:00:30.081459
#53 8.792   Downloaded deno_ops v0.176.0
2026-Mar-23 02:00:30.081459
#53 8.803   Downloaded const-oid v0.9.6
2026-Mar-23 02:00:30.081459
#53 8.806   Downloaded arc-swap v1.8.2
2026-Mar-23 02:00:30.081459
#53 8.812   Downloaded allocator-api2 v0.2.21
2026-Mar-23 02:00:30.081459
#53 8.816   Downloaded base64ct v1.8.3
2026-Mar-23 02:00:30.081459
#53 8.820   Downloaded anyhow v1.0.102
2026-Mar-23 02:00:30.081459
#53 8.826   Downloaded ff v0.12.1
2026-Mar-23 02:00:30.081459
#53 8.828   Downloaded fastrand v2.3.0
2026-Mar-23 02:00:30.081459
#53 8.830   Downloaded convert_case v0.6.0
2026-Mar-23 02:00:30.081459
#53 8.832   Downloaded concurrent-queue v2.5.0
2026-Mar-23 02:00:30.081459
#53 8.834   Downloaded async-trait v0.1.89
2026-Mar-23 02:00:30.081459
#53 8.840   Downloaded arraydeque v0.5.1
2026-Mar-23 02:00:30.081459
#53 8.842   Downloaded bytes-utils v0.1.4
2026-Mar-23 02:00:30.081459
#53 8.844   Downloaded bit-vec v0.6.3
2026-Mar-23 02:00:30.081459
#53 8.846   Downloaded autocfg v1.5.0
2026-Mar-23 02:00:30.081459
#53 8.851   Downloaded libsqlite3-sys v0.30.1
2026-Mar-23 02:00:30.081459
#53 9.022   Downloaded v8 v0.101.0
2026-Mar-23 02:00:30.081459
#53 ...
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#54 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-23 02:00:30.081459
#54 0.284     Updating crates.io index
2026-Mar-23 02:00:30.081459
#54 6.290  Downloading crates ...
2026-Mar-23 02:00:30.081459
#54 6.521   Downloaded adler v1.0.2
2026-Mar-23 02:00:30.081459
#54 6.531   Downloaded adler2 v2.0.1
2026-Mar-23 02:00:30.081459
#54 6.596   Downloaded alloc-stdlib v0.2.2
2026-Mar-23 02:00:30.081459
#54 6.715   Downloaded compression-core v0.4.31
2026-Mar-23 02:00:30.081459
#54 6.764   Downloaded crypto-common v0.1.7
2026-Mar-23 02:00:30.081459
#54 6.767   Downloaded const-random-macro v0.1.16
2026-Mar-23 02:00:30.081459
#54 6.770   Downloaded cfg_aliases v0.2.1
2026-Mar-23 02:00:30.081459
#54 6.788   Downloaded async-stream v0.3.6
2026-Mar-23 02:00:30.081459
#54 6.804   Downloaded digest v0.10.7
2026-Mar-23 02:00:30.081459
#54 6.816   Downloaded cooked-waker v5.0.0
2026-Mar-23 02:00:30.081459
#54 6.820   Downloaded crc v3.4.0
2026-Mar-23 02:00:30.081459
#54 6.824   Downloaded crossbeam-queue v0.3.12
2026-Mar-23 02:00:30.081459
#54 6.827   Downloaded async-stream-impl v0.3.6
2026-Mar-23 02:00:30.081459
#54 6.830   Downloaded block-buffer v0.10.4
2026-Mar-23 02:00:30.081459
#54 6.833   Downloaded const-random v0.1.18
2026-Mar-23 02:00:30.081459
#54 6.836   Downloaded cfg-if v1.0.4
2026-Mar-23 02:00:30.081459
#54 6.848   Downloaded crunchy v0.2.4
2026-Mar-23 02:00:30.081459
#54 6.877   Downloaded base64-simd v0.7.0
2026-Mar-23 02:00:30.081459
#54 6.881   Downloaded atomic-waker v1.1.2
2026-Mar-23 02:00:30.081459
#54 6.884   Downloaded base16ct v0.1.1
2026-Mar-23 02:00:30.081459
#54 6.887   Downloaded dunce v1.0.5
2026-Mar-23 02:00:30.081459
#54 6.891   Downloaded cpufeatures v0.2.17
2026-Mar-23 02:00:30.081459
#54 6.904   Downloaded fnv v1.0.7
2026-Mar-23 02:00:30.081459
#54 6.906   Downloaded dotenvy v0.15.7
2026-Mar-23 02:00:30.081459
#54 6.914   Downloaded funty v2.0.0
2026-Mar-23 02:00:30.081459
#54 6.930   Downloaded futures-core v0.3.32
2026-Mar-23 02:00:30.081459
#54 6.936   Downloaded aws-smithy-observability v0.2.6
2026-Mar-23 02:00:30.081459
#54 6.940   Downloaded debugid v0.8.0
2026-Mar-23 02:00:30.081459
#54 6.945   Downloaded base64-simd v0.8.0
2026-Mar-23 02:00:30.081459
#54 6.949   Downloaded cexpr v0.6.0
2026-Mar-23 02:00:30.081459
#54 6.954   Downloaded crc-catalog v2.4.0
2026-Mar-23 02:00:30.081459
#54 6.958   Downloaded aws-smithy-checksums v0.63.12
2026-Mar-23 02:00:30.081459
#54 6.962   Downloaded aws-smithy-async v1.2.14
2026-Mar-23 02:00:30.081459
#54 6.967   Downloaded bit-set v0.5.3
2026-Mar-23 02:00:30.081459
#54 6.970   Downloaded alloc-no-stdlib v2.0.4
2026-Mar-23 02:00:30.081459
#54 6.975   Downloaded aws-smithy-query v0.60.15
2026-Mar-23 02:00:30.081459
#54 6.978   Downloaded atoi v2.0.0
2026-Mar-23 02:00:30.081459
#54 6.996   Downloaded futures-task v0.3.32
2026-Mar-23 02:00:30.081459
#54 6.999   Downloaded foreign-types v0.3.2
2026-Mar-23 02:00:30.081459
#54 7.003   Downloaded byteorder v1.5.0
2026-Mar-23 02:00:30.081459
#54 7.007   Downloaded concurrent-queue v2.5.0
2026-Mar-23 02:00:30.081459
#54 7.012   Downloaded compression-codecs v0.4.37
2026-Mar-23 02:00:30.081459
#54 7.020   Downloaded ff v0.12.1
2026-Mar-23 02:00:30.081459
#54 7.023   Downloaded data-encoding v2.10.0
2026-Mar-23 02:00:30.081459
#54 7.025   Downloaded convert_case v0.6.0
2026-Mar-23 02:00:30.081459
#54 7.028   Downloaded axum-core v0.5.6
2026-Mar-23 02:00:30.081459
#54 7.033   Downloaded dashmap v6.1.0
2026-Mar-23 02:00:30.081459
#54 7.037   Downloaded aws-smithy-xml v0.60.15
2026-Mar-23 02:00:30.081459
#54 7.040   Downloaded cmake v0.1.57
2026-Mar-23 02:00:30.081459
#54 7.043   Downloaded aws-smithy-json v0.61.9
2026-Mar-23 02:00:30.081459
#54 7.046   Downloaded bytes-utils v0.1.4
2026-Mar-23 02:00:30.081459
#54 7.049   Downloaded aws-credential-types v1.2.14
2026-Mar-23 02:00:30.081459
#54 7.054   Downloaded arraydeque v0.5.1
2026-Mar-23 02:00:30.081459
#54 7.058   Downloaded crc32fast v1.5.0
2026-Mar-23 02:00:30.081459
#54 7.063   Downloaded lru v0.12.5
2026-Mar-23 02:00:30.081459
#54 7.067   Downloaded memoffset v0.9.1
2026-Mar-23 02:00:30.081459
#54 7.071   Downloaded fastrand v2.3.0
2026-Mar-23 02:00:30.081459
#54 7.075   Downloaded httpdate v1.0.3
2026-Mar-23 02:00:30.081459
#54 7.078   Downloaded errno v0.3.14
2026-Mar-23 02:00:30.081459
#54 7.082   Downloaded cookie v0.18.1
2026-Mar-23 02:00:30.081459
#54 7.087   Downloaded mime v0.3.17
2026-Mar-23 02:00:30.081459
#54 7.091   Downloaded allocator-api2 v0.2.21
2026-Mar-23 02:00:30.081459
#54 7.095   Downloaded itoa v1.0.17
2026-Mar-23 02:00:30.081459
#54 7.099   Downloaded lock_api v0.4.14
2026-Mar-23 02:00:30.081459
#54 7.102   Downloaded const-oid v0.9.6
2026-Mar-23 02:00:30.081459
#54 7.106   Downloaded aws-smithy-http v0.63.6
2026-Mar-23 02:00:30.081459
#54 7.111   Downloaded crypto-bigint v0.4.9
2026-Mar-23 02:00:30.081459
#54 7.120   Downloaded litrs v1.0.0
2026-Mar-23 02:00:30.081459
#54 7.127   Downloaded cookie_store v0.22.1
2026-Mar-23 02:00:30.081459
#54 7.132   Downloaded crossbeam-epoch v0.9.18
2026-Mar-23 02:00:30.081459
#54 7.137   Downloaded config v0.14.1
2026-Mar-23 02:00:30.081459
#54 7.147   Downloaded num-conv v0.2.0
2026-Mar-23 02:00:30.081459
#54 7.150   Downloaded deno_ops v0.176.0
2026-Mar-23 02:00:30.081459
#54 7.166   Downloaded lazy_static v1.5.0
2026-Mar-23 02:00:30.081459
#54 7.170   Downloaded event-listener-strategy v0.5.4
2026-Mar-23 02:00:30.081459
#54 7.173   Downloaded http-body v0.4.6
2026-Mar-23 02:00:30.081459
#54 7.177   Downloaded der v0.6.1
2026-Mar-23 02:00:30.081459
#54 7.187   Downloaded openssl-probe v0.2.1
2026-Mar-23 02:00:30.081459
#54 7.191   Downloaded parking v2.2.1
2026-Mar-23 02:00:30.081459
#54 7.195   Downloaded percent-encoding v2.3.2
2026-Mar-23 02:00:30.081459
#54 7.198   Downloaded async-compression v0.4.41
2026-Mar-23 02:00:30.081459
#54 7.211   Downloaded pem v3.0.6
2026-Mar-23 02:00:30.081459
#54 7.214   Downloaded parking_lot_core v0.9.12
2026-Mar-23 02:00:30.081459
#54 7.219   Downloaded miniz_oxide v0.8.9
2026-Mar-23 02:00:30.081459
#54 7.224   Downloaded heck v0.5.0
2026-Mar-23 02:00:30.081459
#54 7.227   Downloaded pem-rfc7468 v0.7.0
2026-Mar-23 02:00:30.081459
#54 7.232   Downloaded hyper-tls v0.6.0
2026-Mar-23 02:00:30.081459
#54 7.235   Downloaded json5 v0.4.1
2026-Mar-23 02:00:30.081459
#54 7.239   Downloaded combine v4.6.7
2026-Mar-23 02:00:30.081459
#54 7.248   Downloaded pin-utils v0.1.0
2026-Mar-23 02:00:30.081459
#54 7.252   Downloaded pin-project-internal v1.1.10
2026-Mar-23 02:00:30.081459
#54 7.256   Downloaded pin-project-lite v0.2.16
2026-Mar-23 02:00:30.081459
#54 7.266   Downloaded pkcs8 v0.10.2
2026-Mar-23 02:00:30.081459
#54 7.273   Downloaded pkg-config v0.3.32
2026-Mar-23 02:00:30.081459
#54 7.276   Downloaded psl-types v2.0.11
2026-Mar-23 02:00:30.081459
#54 7.278   Downloaded proc-macro-rules v0.4.0
2026-Mar-23 02:00:30.081459
#54 7.281   Downloaded proc-macro-error-attr v1.0.4
2026-Mar-23 02:00:30.081459
#54 7.283   Downloaded powerfmt v0.2.0
2026-Mar-23 02:00:30.081459
#54 7.285   Downloaded deno_core v0.300.0
2026-Mar-23 02:00:30.081459
#54 7.300   Downloaded proc-macro-error v1.0.4
2026-Mar-23 02:00:30.081459
#54 7.306   Downloaded bitvec v1.0.1
2026-Mar-23 02:00:30.081459
#54 7.338   Downloaded rand_chacha v0.9.0
2026-Mar-23 02:00:30.081459
#54 7.342   Downloaded rand_core v0.6.4
2026-Mar-23 02:00:30.081459
#54 7.345   Downloaded rustc_version v0.4.1
2026-Mar-23 02:00:30.081459
#54 7.347   Downloaded either v1.15.0
2026-Mar-23 02:00:30.081459
#54 7.349   Downloaded rustc-hash v2.1.1
2026-Mar-23 02:00:30.081459
#54 7.351   Downloaded rustc-hash v1.1.0
2026-Mar-23 02:00:30.081459
#54 7.353   Downloaded rfc6979 v0.3.1
2026-Mar-23 02:00:30.081459
#54 7.354   Downloaded http-body v1.0.1
2026-Mar-23 02:00:30.081459
#54 7.357   Downloaded openssl-sys v0.9.111
2026-Mar-23 02:00:30.081459
#54 7.365   Downloaded rustc_version v0.2.3
2026-Mar-23 02:00:30.081459
#54 7.366   Downloaded openssl v0.10.75
2026-Mar-23 02:00:30.081459
#54 7.378   Downloaded rustversion v1.0.22
2026-Mar-23 02:00:30.081459
#54 7.383   Downloaded no-std-compat v0.4.1
2026-Mar-23 02:00:30.081459
#54 7.384   Downloaded brotli v8.0.2
2026-Mar-23 02:00:30.081459
#54 7.404   Downloaded http-body-util v0.1.3
2026-Mar-23 02:00:30.081459
#54 7.408   Downloaded displaydoc v0.2.5
2026-Mar-23 02:00:30.081459
#54 7.414   Downloaded group v0.12.1
2026-Mar-23 02:00:30.081459
#54 7.416   Downloaded p256 v0.11.1
2026-Mar-23 02:00:30.081459
#54 7.421   Downloaded aws-sdk-s3 v1.119.0
2026-Mar-23 02:00:30.081459
#54 7.571   Downloaded sec1 v0.3.0
2026-Mar-23 02:00:30.081459
#54 7.576   Downloaded spki v0.7.3
2026-Mar-23 02:00:30.081459
#54 7.581   Downloaded sqlx-macros-core v0.8.6
2026-Mar-23 02:00:30.081459
#54 7.586   Downloaded tokio-macros v2.6.0
2026-Mar-23 02:00:30.081459
#54 7.588   Downloaded tinyvec v1.10.0
2026-Mar-23 02:00:30.081459
#54 7.593   Downloaded raw-cpuid v11.6.0
2026-Mar-23 02:00:30.081459
#54 7.598   Downloaded regex-lite v0.1.9
2026-Mar-23 02:00:30.081459
#54 7.602   Downloaded try-lock v0.2.5
2026-Mar-23 02:00:30.081459
#54 7.604   Downloaded unicode-properties v0.1.4
2026-Mar-23 02:00:30.081459
#54 7.607   Downloaded regex-syntax v0.8.10
2026-Mar-23 02:00:30.081459
#54 7.615   Downloaded regex-automata v0.4.14
2026-Mar-23 02:00:30.081459
#54 7.632   Downloaded uuid v1.21.0
2026-Mar-23 02:00:30.081459
#54 7.636   Downloaded unicode-bidi v0.3.18
2026-Mar-23 02:00:30.081459
#54 7.640   Downloaded ucd-trie v0.1.7
2026-Mar-23 02:00:30.081459
#54 7.642   Downloaded ring v0.17.14
2026-Mar-23 02:00:30.081459
#54 7.691   Downloaded nu-ansi-term v0.50.3
2026-Mar-23 02:00:30.081459
#54 7.695   Downloaded nonzero_ext v0.3.0
2026-Mar-23 02:00:30.081459
#54 7.704   Downloaded getrandom v0.2.17
2026-Mar-23 02:00:30.081459
#54 7.709   Downloaded tracing-core v0.1.36
2026-Mar-23 02:00:30.081459
#54 7.714   Downloaded tracing-attributes v0.1.31
2026-Mar-23 02:00:30.081459
#54 7.719   Downloaded reqwest v0.12.28
2026-Mar-23 02:00:30.081459
#54 7.725   Downloaded vsimd v0.8.0
2026-Mar-23 02:00:30.081459
#54 7.728   Downloaded version_check v0.9.5
2026-Mar-23 02:00:30.081459
#54 7.730   Downloaded sqlx-sqlite v0.8.6
2026-Mar-23 02:00:30.081459
#54 7.735   Downloaded sqlx-postgres v0.8.6
2026-Mar-23 02:00:30.081459
#54 7.750   Downloaded zeroize v1.8.2
2026-Mar-23 02:00:30.081459
#54 7.752   Downloaded zerovec-derive v0.11.2
2026-Mar-23 02:00:30.081459
#54 7.755   Downloaded zmij v1.0.21
2026-Mar-23 02:00:30.081459
#54 7.758   Downloaded yoke v0.8.1
2026-Mar-23 02:00:30.081459
#54 7.760   Downloaded xmlparser v0.13.6
2026-Mar-23 02:00:30.081459
#54 7.764   Downloaded zerofrom-derive v0.1.6
2026-Mar-23 02:00:30.081459
#54 7.766   Downloaded zerofrom v0.1.6
2026-Mar-23 02:00:30.081459
#54 7.767   Downloaded yoke-derive v0.8.1
2026-Mar-23 02:00:30.081459
#54 7.769   Downloaded time v0.3.47
2026-Mar-23 02:00:30.081459
#54 7.790   Downloaded sqlx-mysql v0.8.6
2026-Mar-23 02:00:30.081459
#54 7.799   Downloaded serde_json v1.0.149
2026-Mar-23 02:00:30.081459
#54 7.809   Downloaded serde v1.0.228
2026-Mar-23 02:00:30.081459
#54 7.814   Downloaded typenum v1.19.0
2026-Mar-23 02:00:30.081459
#54 7.818   Downloaded tower-http v0.6.8
2026-Mar-23 02:00:30.081459
#54 7.831   Downloaded tracing-subscriber v0.3.22
2026-Mar-23 02:00:30.081459
#54 7.845   Downloaded tracing v0.1.44
2026-Mar-23 02:00:30.081459
#54 7.866   Downloaded utoipa-gen v4.3.1
2026-Mar-23 02:00:30.081459
#54 7.875   Downloaded unicode-segmentation v1.12.0
2026-Mar-23 02:00:30.081459
#54 7.879   Downloaded vcpkg v0.2.15
2026-Mar-23 02:00:30.081459
#54 7.959   Downloaded subtle v2.6.1
2026-Mar-23 02:00:30.081459
#54 7.961   Downloaded icu_locale_core v2.1.1
2026-Mar-23 02:00:30.081459
#54 7.971   Downloaded quinn-udp v0.5.14
2026-Mar-23 02:00:30.081459
#54 7.974   Downloaded hmac v0.12.1
2026-Mar-23 02:00:30.081459
#54 7.977   Downloaded getrandom v0.3.4
2026-Mar-23 02:00:30.081459
#54 7.982   Downloaded flume v0.11.1
2026-Mar-23 02:00:30.081459
#54 7.987   Downloaded spki v0.6.0
2026-Mar-23 02:00:30.081459
#54 7.989   Downloaded rust-ini v0.20.0
2026-Mar-23 02:00:30.081459
#54 7.991   Downloaded jsonwebtoken v9.3.1
2026-Mar-23 02:00:30.081459
#54 7.997   Downloaded futures v0.3.32
2026-Mar-23 02:00:30.081459
#54 8.003   Downloaded elliptic-curve v0.12.3
2026-Mar-23 02:00:30.081459
#54 8.008   Downloaded httparse v1.10.1
2026-Mar-23 02:00:30.081459
#54 8.011   Downloaded hashlink v0.8.4
2026-Mar-23 02:00:30.081459
#54 8.014   Downloaded litemap v0.8.1
2026-Mar-23 02:00:30.081459
#54 8.016   Downloaded tower v0.5.3
2026-Mar-23 02:00:30.081459
#54 8.030   Downloaded tokio v1.49.0
2026-Mar-23 02:00:30.081459
#54 8.083   Downloaded aws-lc-sys v0.38.0
2026-Mar-23 02:00:30.081459
#54 8.391   Downloaded toml_write v0.1.2
2026-Mar-23 02:00:30.081459
#54 8.393   Downloaded utf8_iter v1.0.4
2026-Mar-23 02:00:30.081459
#54 8.396   Downloaded which v4.4.2
2026-Mar-23 02:00:30.081459
#54 8.398   Downloaded hashbrown v0.14.5
2026-Mar-23 02:00:30.081459
#54 8.404   Downloaded webpki-roots v0.26.11
2026-Mar-23 02:00:30.081459
#54 8.406   Downloaded want v0.3.1
2026-Mar-23 02:00:30.081459
#54 8.408   Downloaded unicode-id-start v1.4.0
2026-Mar-23 02:00:30.081459
#54 8.411   Downloaded rand v0.9.2
2026-Mar-23 02:00:30.081459
#54 8.416   Downloaded num-bigint v0.4.6
2026-Mar-23 02:00:30.081459
#54 8.422   Downloaded icu_collections v2.1.1
2026-Mar-23 02:00:30.081459
#54 8.429   Downloaded hyper-util v0.1.20
2026-Mar-23 02:00:30.081459
#54 8.437   Downloaded http v1.4.0
2026-Mar-23 02:00:30.081459
#54 8.442   Downloaded http v0.2.12
2026-Mar-23 02:00:30.081459
#54 8.446   Downloaded futures-intrusive v0.5.0
2026-Mar-23 02:00:30.081459
#54 8.452   Downloaded urlencoding v2.1.3
2026-Mar-23 02:00:30.081459
#54 8.453   Downloaded untrusted v0.9.0
2026-Mar-23 02:00:30.081459
#54 8.456   Downloaded toml v0.8.23
2026-Mar-23 02:00:30.081459
#54 8.459   Downloaded rustls-pki-types v1.14.0
2026-Mar-23 02:00:30.081459
#54 8.462   Downloaded ron v0.8.1
2026-Mar-23 02:00:30.081459
#54 8.470   Downloaded indexmap v2.13.0
2026-Mar-23 02:00:30.081459
#54 8.475   Downloaded icu_provider v2.1.1
2026-Mar-23 02:00:30.081459
#54 8.478   Downloaded icu_properties v2.1.2
2026-Mar-23 02:00:30.081459
#54 8.481   Downloaded icu_normalizer_data v2.1.1
2026-Mar-23 02:00:30.081459
#54 8.483   Downloaded getrandom v0.4.1
2026-Mar-23 02:00:30.081459
#54 8.488   Downloaded wyz v0.5.1
2026-Mar-23 02:00:30.081459
#54 8.489   Downloaded num-bigint-dig v0.8.6
2026-Mar-23 02:00:30.081459
#54 8.495   Downloaded itertools v0.12.1
2026-Mar-23 02:00:30.081459
#54 8.504   Downloaded rsa v0.9.10
2026-Mar-23 02:00:30.081459
#54 8.511   Downloaded web-time v1.1.0
2026-Mar-23 02:00:30.081459
#54 8.513   Downloaded writeable v0.6.2
2026-Mar-23 02:00:30.081459
#54 8.516   Downloaded which v6.0.3
2026-Mar-23 02:00:30.081459
#54 8.518   Downloaded governor v0.8.1
2026-Mar-23 02:00:30.081459
#54 8.523   Downloaded hashbrown v0.16.1
2026-Mar-23 02:00:30.081459
#54 8.531   Downloaded rustls-webpki v0.103.9
2026-Mar-23 02:00:30.081459
#54 8.535   Downloaded hyper v1.8.1
2026-Mar-23 02:00:30.081459
#54 8.546   Downloaded hashbrown v0.15.5
2026-Mar-23 02:00:30.081459
#54 8.553   Downloaded utoipa v4.2.3
2026-Mar-23 02:00:30.081459
#54 8.557   Downloaded unicode-ident v1.0.24
2026-Mar-23 02:00:30.081459
#54 8.560   Downloaded iri-string v0.7.10
2026-Mar-23 02:00:30.081459
#54 8.571   Downloaded whoami v1.6.1
2026-Mar-23 02:00:30.081459
#54 8.574   Downloaded idna v1.1.0
2026-Mar-23 02:00:30.081459
#54 8.578   Downloaded itertools v0.13.0
2026-Mar-23 02:00:30.081459
#54 8.587   Downloaded hkdf v0.12.4
2026-Mar-23 02:00:30.081459
#54 8.590   Downloaded futures-util v0.3.32
2026-Mar-23 02:00:30.081459
#54 8.608   Downloaded h2 v0.3.27
2026-Mar-23 02:00:30.081459
#54 8.616   Downloaded sqlx-core v0.8.6
2026-Mar-23 02:00:30.081459
#54 8.626   Downloaded toml_edit v0.22.27
2026-Mar-23 02:00:30.081459
#54 8.632   Downloaded hyper v0.14.32
2026-Mar-23 02:00:30.081459
#54 8.641   Downloaded h2 v0.4.13
2026-Mar-23 02:00:30.081459
#54 8.649   Downloaded libm v0.2.16
2026-Mar-23 02:00:30.081459
#54 8.663   Downloaded icu_properties_data v2.1.2
2026-Mar-23 02:00:30.081459
#54 8.677   Downloaded tower v0.4.13
2026-Mar-23 02:00:30.081459
#54 8.691   Downloaded rustls-webpki v0.101.7
2026-Mar-23 02:00:30.081459
#54 8.715   Downloaded flate2 v1.1.9
2026-Mar-23 02:00:30.081459
#54 8.722   Downloaded sqlx v0.8.6
2026-Mar-23 02:00:30.081459
#54 8.740   Downloaded toml_datetime v0.6.11
2026-Mar-23 02:00:30.081459
#54 8.742   Downloaded thiserror-impl v1.0.69
2026-Mar-23 02:00:30.081459
#54 8.744   Downloaded rustls-native-certs v0.8.3
2026-Mar-23 02:00:30.081459
#54 8.747   Downloaded rand_core v0.9.5
2026-Mar-23 02:00:30.081459
#54 8.749   Downloaded pin-project v1.1.10
2026-Mar-23 02:00:30.081459
#54 8.767   Downloaded icu_normalizer v2.1.1
2026-Mar-23 02:00:30.081459
#54 8.771   Downloaded tracing-log v0.2.0
2026-Mar-23 02:00:30.081459
#54 8.773   Downloaded tower-service v0.3.3
2026-Mar-23 02:00:30.081459
#54 8.774   Downloaded tower-layer v0.3.3
2026-Mar-23 02:00:30.081459
#54 8.775   Downloaded tokio-stream v0.1.18
2026-Mar-23 02:00:30.081459
#54 8.782   Downloaded tokio-rustls v0.26.4
2026-Mar-23 02:00:30.081459
#54 8.785   Downloaded syn v1.0.109
2026-Mar-23 02:00:30.081459
#54 8.796   Downloaded tokio-rustls v0.24.1
2026-Mar-23 02:00:30.081459
#54 8.799   Downloaded tokio-native-tls v0.3.1
2026-Mar-23 02:00:30.081459
#54 8.802   Downloaded regex v1.12.3
2026-Mar-23 02:00:30.081459
#54 8.808   Downloaded redis v0.27.6
2026-Mar-23 02:00:30.081459
#54 8.818   Downloaded deno_core_icudata v0.0.73
2026-Mar-23 02:00:30.081459
#54 8.853   Downloaded rand v0.8.5
2026-Mar-23 02:00:30.081459
#54 8.857   Downloaded rustls v0.21.12
2026-Mar-23 02:00:30.081459
#54 8.871   Downloaded quinn-proto v0.11.13
2026-Mar-23 02:00:30.081459
#54 8.880   Downloaded syn v2.0.117
2026-Mar-23 02:00:30.081459
#54 8.893   Downloaded quinn v0.11.9
2026-Mar-23 02:00:30.081459
#54 8.897   Downloaded rustix v0.38.44
2026-Mar-23 02:00:30.081459
#54 8.933   Downloaded publicsuffix v2.3.0
2026-Mar-23 02:00:30.081459
#54 8.936   Downloaded tinyvec_macros v0.1.1
2026-Mar-23 02:00:30.081459
#54 8.939   Downloaded tinystr v0.8.2
2026-Mar-23 02:00:30.081459
#54 8.943   Downloaded rustls v0.23.37
2026-Mar-23 02:00:30.081459
#54 8.957   Downloaded tiny-keccak v2.0.2
2026-Mar-23 02:00:30.081459
#54 8.960   Downloaded time-macros v0.2.27
2026-Mar-23 02:00:30.081459
#54 8.963   Downloaded time-core v0.1.8
2026-Mar-23 02:00:30.081459
#54 8.964   Downloaded thread_local v1.1.9
2026-Mar-23 02:00:30.081459
#54 8.966   Downloaded thiserror-impl v2.0.18
2026-Mar-23 02:00:30.081459
#54 8.968   Downloaded thiserror v2.0.18
2026-Mar-23 02:00:30.081459
#54 8.976   Downloaded thiserror v1.0.69
2026-Mar-23 02:00:30.081459
#54 8.985   Downloaded tap v1.0.1
2026-Mar-23 02:00:30.081459
#54 8.986   Downloaded tagptr v0.2.0
2026-Mar-23 02:00:30.081459
#54 8.988   Downloaded synstructure v0.13.2
2026-Mar-23 02:00:30.081459
#54 8.989   Downloaded sync_wrapper v1.0.2
2026-Mar-23 02:00:30.081459
#54 8.990   Downloaded strum_macros v0.25.3
2026-Mar-23 02:00:30.081459
#54 8.993   Downloaded strum v0.25.0
2026-Mar-23 02:00:30.081459
#54 8.994   Downloaded zerotrie v0.2.3
2026-Mar-23 02:00:30.081459
#54 8.998   Downloaded stringprep v0.1.5
2026-Mar-23 02:00:30.081459
#54 9.000   Downloaded zerovec v0.11.5
2026-Mar-23 02:00:30.081459
#54 9.007   Downloaded static_assertions v1.1.0
2026-Mar-23 02:00:30.081459
#54 9.009   Downloaded yaml-rust2 v0.8.1
2026-Mar-23 02:00:30.081459
#54 9.050   Downloaded stable_deref_trait v1.2.1
2026-Mar-23 02:00:30.081459
#54 9.052   Downloaded prettyplease v0.2.37
2026-Mar-23 02:00:30.081459
#54 9.057   Downloaded portable-atomic v1.13.1
2026-Mar-23 02:00:30.081459
#54 9.067   Downloaded url v2.5.8
2026-Mar-23 02:00:30.081459
#54 9.069   Downloaded sqlx-macros v0.8.6
2026-Mar-23 02:00:30.081459
#54 9.070   Downloaded spinning_top v0.3.0
2026-Mar-23 02:00:30.081459
#54 9.072   Downloaded zerocopy v0.8.39
2026-Mar-23 02:00:30.081459
#54 9.102   Downloaded spin v0.9.8
2026-Mar-23 02:00:30.081459
#54 9.104   Downloaded unicode-normalization v0.1.25
2026-Mar-23 02:00:30.081459
#54 9.108   Downloaded sourcemap v8.0.1
2026-Mar-23 02:00:30.081459
#54 9.111   Downloaded socket2 v0.6.2
2026-Mar-23 02:00:30.081459
#54 9.112   Downloaded socket2 v0.5.10
2026-Mar-23 02:00:30.081459
#54 9.114   Downloaded smallvec v1.15.1
2026-Mar-23 02:00:30.081459
#54 9.117   Downloaded slab v0.4.12
2026-Mar-23 02:00:30.081459
#54 9.118   Downloaded simple_asn1 v0.6.4
2026-Mar-23 02:00:30.081459
#54 9.120   Downloaded simd-adler32 v0.3.8
2026-Mar-23 02:00:30.081459
#54 9.122   Downloaded simd-abstraction v0.7.1
2026-Mar-23 02:00:30.081459
#54 9.123   Downloaded signature v2.2.0
2026-Mar-23 02:00:30.081459
#54 9.125   Downloaded signature v1.6.4
2026-Mar-23 02:00:30.081459
#54 9.127   Downloaded tokio-util v0.7.18
2026-Mar-23 02:00:30.081459
#54 9.136   Downloaded signal-hook-registry v1.4.8
2026-Mar-23 02:00:30.081459
#54 9.137   Downloaded shlex v1.3.0
2026-Mar-23 02:00:30.081459
#54 9.139   Downloaded sharded-slab v0.1.7
2026-Mar-23 02:00:30.081459
#54 9.143   Downloaded sha2 v0.10.9
2026-Mar-23 02:00:30.081459
#54 9.146   Downloaded sha1_smol v1.0.1
2026-Mar-23 02:00:30.081459
#54 9.148   Downloaded sha1 v0.10.6
2026-Mar-23 02:00:30.081459
#54 9.150   Downloaded serde_v8 v0.209.0
2026-Mar-23 02:00:30.081459
#54 9.153   Downloaded serde_urlencoded v0.7.1
2026-Mar-23 02:00:30.081459
#54 9.155   Downloaded serde_spanned v0.6.9
2026-Mar-23 02:00:30.081459
#54 9.156   Downloaded winnow v0.7.14
2026-Mar-23 02:00:30.081459
#54 9.168   Downloaded serde_path_to_error v0.1.20
2026-Mar-23 02:00:30.081459
#54 9.170   Downloaded serde_derive v1.0.228
2026-Mar-23 02:00:30.081459
#54 9.173   Downloaded serde_core v1.0.228
2026-Mar-23 02:00:30.081459
#54 9.176   Downloaded semver-parser v0.7.0
2026-Mar-23 02:00:30.081459
#54 9.178   Downloaded semver v1.0.27
2026-Mar-23 02:00:30.081459
#54 9.181   Downloaded semver v0.9.0
2026-Mar-23 02:00:30.081459
#54 9.182   Downloaded pest_meta v2.8.6
2026-Mar-23 02:00:30.081459
#54 9.185   Downloaded pest v2.8.6
2026-Mar-23 02:00:30.081459
#54 9.191   Downloaded hyper-rustls v0.27.7
2026-Mar-23 02:00:30.081459
#54 9.194   Downloaded hashlink v0.10.0
2026-Mar-23 02:00:30.081459
#54 9.196   Downloaded futures-executor v0.3.32
2026-Mar-23 02:00:30.081459
#54 9.198   Downloaded futures-channel v0.3.32
2026-Mar-23 02:00:30.081459
#54 9.200   Downloaded fs_extra v1.3.0
2026-Mar-23 02:00:30.081459
#54 9.202   Downloaded foldhash v0.1.5
2026-Mar-23 02:00:30.081459
#54 9.203   Downloaded webpki-roots v1.0.6
2026-Mar-23 02:00:30.081459
#54 9.206   Downloaded find-msvc-tools v0.1.9
2026-Mar-23 02:00:30.081459
#54 9.208   Downloaded event-listener v5.4.1
2026-Mar-23 02:00:30.081459
#54 9.211   Downloaded sct v0.7.1
2026-Mar-23 02:00:30.081459
#54 9.217   Downloaded scopeguard v1.2.0
2026-Mar-23 02:00:30.081459
#54 9.219   Downloaded libc v0.2.182
2026-Mar-23 02:00:30.081459
#54 9.274   Downloaded ryu v1.0.23
2026-Mar-23 02:00:30.081459
#54 9.278   Downloaded proc-macro-rules-macros v0.4.0
2026-Mar-23 02:00:30.081459
#54 9.280   Downloaded potential_utf v0.1.4
2026-Mar-23 02:00:30.081459
#54 9.281   Downloaded outref v0.5.2
2026-Mar-23 02:00:30.081459
#54 9.283   Downloaded outref v0.1.0
2026-Mar-23 02:00:30.081459
#54 9.284   Downloaded libloading v0.8.9
2026-Mar-23 02:00:30.081459
#54 9.287   Downloaded jobserver v0.1.34
2026-Mar-23 02:00:30.081459
#54 9.290   Downloaded ipnet v2.11.0
2026-Mar-23 02:00:30.081459
#54 9.292   Downloaded idna_adapter v1.2.1
2026-Mar-23 02:00:30.081459
#54 9.293   Downloaded hyper-rustls v0.24.2
2026-Mar-23 02:00:30.081459
#54 9.296   Downloaded futures-timer v3.0.3
2026-Mar-23 02:00:30.081459
#54 9.299   Downloaded glob v0.3.3
2026-Mar-23 02:00:30.081459
#54 9.301   Downloaded form_urlencoded v1.2.2
2026-Mar-23 02:00:30.081459
#54 9.302   Downloaded quanta v0.12.6
2026-Mar-23 02:00:30.081459
#54 9.306   Downloaded proc-macro2 v1.0.106
2026-Mar-23 02:00:30.081459
#54 9.310   Downloaded ppv-lite86 v0.2.21
2026-Mar-23 02:00:30.081459
#54 9.312   Downloaded nom v7.1.3
2026-Mar-23 02:00:30.081459
#54 9.319   Downloaded moka v0.12.13
2026-Mar-23 02:00:30.081459
#54 9.328   Downloaded rand_chacha v0.3.1
2026-Mar-23 02:00:30.081459
#54 9.330   Downloaded radium v0.7.0
2026-Mar-23 02:00:30.081459
#54 9.332   Downloaded quote v1.0.44
2026-Mar-23 02:00:30.081459
#54 9.336   Downloaded matchers v0.2.0
2026-Mar-23 02:00:30.081459
#54 9.338   Downloaded lazycell v1.3.0
2026-Mar-23 02:00:30.081459
#54 9.339   Downloaded hex v0.4.3
2026-Mar-23 02:00:30.081459
#54 9.342   Downloaded gzip-header v1.0.0
2026-Mar-23 02:00:30.081459
#54 9.344   Downloaded generic-array v0.14.7
2026-Mar-23 02:00:30.081459
#54 9.346   Downloaded fslock v0.2.1
2026-Mar-23 02:00:30.081459
#54 9.348   Downloaded mio v1.1.1
2026-Mar-23 02:00:30.081459
#54 9.356   Downloaded bindgen v0.69.5
2026-Mar-23 02:00:30.081459
#54 9.364   Downloaded aws-lc-rs v1.16.1
2026-Mar-23 02:00:30.081459
#54 9.377   Downloaded pkcs8 v0.9.0
2026-Mar-23 02:00:30.081459
#54 9.380   Downloaded pkcs1 v0.7.5
2026-Mar-23 02:00:30.081459
#54 9.384   Downloaded foreign-types-shared v0.1.1
2026-Mar-23 02:00:30.081459
#54 9.385   Downloaded brotli-decompressor v5.0.0
2026-Mar-23 02:00:30.081459
#54 9.392   Downloaded axum v0.8.8
2026-Mar-23 02:00:30.081459
#54 9.402   Downloaded aws-sdk-sts v1.100.0
2026-Mar-23 02:00:30.081459
#54 9.420   Downloaded pest_generator v2.8.6
2026-Mar-23 02:00:30.081459
#54 9.422   Downloaded heck v0.4.1
2026-Mar-23 02:00:30.081459
#54 9.424   Downloaded futures-macro v0.3.32
2026-Mar-23 02:00:30.081459
#54 9.426   Downloaded futures-io v0.3.32
2026-Mar-23 02:00:30.081459
#54 9.427   Downloaded aws-config v1.8.15
2026-Mar-23 02:00:30.081459
#54 9.434   Downloaded encoding_rs v0.8.35
2026-Mar-23 02:00:30.081459
#54 9.454   Downloaded aho-corasick v1.1.4
2026-Mar-23 02:00:30.081459
#54 9.462   Downloaded crc-fast v1.6.0
2026-Mar-23 02:00:30.081459
#54 9.473   Downloaded pest_derive v2.8.6
2026-Mar-23 02:00:30.081459
#54 9.477   Downloaded minimal-lexical v0.2.1
2026-Mar-23 02:00:30.081459
#54 9.484   Downloaded memchr v2.8.0
2026-Mar-23 02:00:30.081459
#54 9.491   Downloaded aws-smithy-runtime v1.10.3
2026-Mar-23 02:00:30.081459
#54 9.500   Downloaded pathdiff v0.2.3
2026-Mar-23 02:00:30.081459
#54 9.501   Downloaded paste v1.0.15
2026-Mar-23 02:00:30.081459
#54 9.507   Downloaded parking_lot v0.12.5
2026-Mar-23 02:00:30.081459
#54 9.511   Downloaded num-traits v0.2.19
2026-Mar-23 02:00:30.081459
#54 9.515   Downloaded ordered-multimap v0.7.3
2026-Mar-23 02:00:30.081459
#54 9.517   Downloaded once_cell v1.21.3
2026-Mar-23 02:00:30.081459
#54 9.521   Downloaded num-iter v0.1.45
2026-Mar-23 02:00:30.081459
#54 9.522   Downloaded crossbeam-channel v0.5.15
2026-Mar-23 02:00:30.081459
#54 9.527   Downloaded aws-smithy-http-client v1.1.12
2026-Mar-23 02:00:30.081459
#54 9.532   Downloaded aws-sigv4 v1.4.2
2026-Mar-23 02:00:30.081459
#54 9.611   Downloaded aws-runtime v1.7.2
2026-Mar-23 02:00:30.081459
#54 9.617   Downloaded num-integer v0.1.46
2026-Mar-23 02:00:30.081459
#54 9.619   Downloaded miniz_oxide v0.7.4
2026-Mar-23 02:00:30.081459
#54 9.622   Downloaded der v0.7.10
2026-Mar-23 02:00:30.081459
#54 9.631   Downloaded crypto-bigint v0.5.5
2026-Mar-23 02:00:30.081459
#54 9.642   Downloaded cc v1.2.56
2026-Mar-23 02:00:30.081459
#54 9.646   Downloaded base64 v0.22.1
2026-Mar-23 02:00:30.081459
#54 9.651   Downloaded base64 v0.21.7
2026-Mar-23 02:00:30.081459
#54 9.656   Downloaded aws-smithy-types v1.4.6
2026-Mar-23 02:00:30.081459
#54 9.662   Downloaded aws-sdk-ssooidc v1.98.0
2026-Mar-23 02:00:30.081459
#54 9.673   Downloaded aws-sdk-sso v1.96.0
2026-Mar-23 02:00:30.081459
#54 9.682   Downloaded openssl-macros v0.1.1
2026-Mar-23 02:00:30.081459
#54 9.683   Downloaded num_cpus v1.17.0
2026-Mar-23 02:00:30.081459
#54 9.687   Downloaded log v0.4.29
2026-Mar-23 02:00:30.081459
#54 9.691   Downloaded ecdsa v0.14.8
2026-Mar-23 02:00:30.081459
#54 9.693   Downloaded document-features v0.2.12
2026-Mar-23 02:00:30.081459
#54 9.694   Downloaded dlv-list v0.5.2
2026-Mar-23 02:00:30.081459
#54 9.697   Downloaded deno_unsync v0.4.4
2026-Mar-23 02:00:30.081459
#54 9.700   Downloaded bytes v1.11.1
2026-Mar-23 02:00:30.081459
#54 9.705   Downloaded aws-smithy-runtime-api v1.11.6
2026-Mar-23 02:00:30.081459
#54 9.711   Downloaded arc-swap v1.8.2
2026-Mar-23 02:00:30.081459
#54 9.717   Downloaded native-tls v0.2.18
2026-Mar-23 02:00:30.081459
#54 9.719   Downloaded matchit v0.8.4
2026-Mar-23 02:00:30.081459
#54 9.722   Downloaded linux-raw-sys v0.4.15
2026-Mar-23 02:00:30.081459
#54 9.776   Downloaded home v0.5.12
2026-Mar-23 02:00:30.081459
#54 9.777   Downloaded futures-sink v0.3.32
2026-Mar-23 02:00:30.081459
#54 9.778   Downloaded equivalent v1.0.2
2026-Mar-23 02:00:30.081459
#54 9.779   Downloaded anyhow v1.0.102
2026-Mar-23 02:00:30.081459
#54 9.784   Downloaded aws-smithy-json v0.62.5
2026-Mar-23 02:00:30.081459
#54 9.786   Downloaded crossbeam-utils v0.8.21
2026-Mar-23 02:00:30.081459
#54 9.790   Downloaded base64ct v1.8.3
2026-Mar-23 02:00:30.081459
#54 9.793   Downloaded aws-types v1.3.14
2026-Mar-23 02:00:30.081459
#54 9.795   Downloaded aws-smithy-http v0.62.6
2026-Mar-23 02:00:30.081459
#54 9.798   Downloaded ahash v0.8.12
2026-Mar-23 02:00:30.081459
#54 9.801   Downloaded clang-sys v1.8.1
2026-Mar-23 02:00:30.081459
#54 9.804   Downloaded async-lock v3.4.2
2026-Mar-23 02:00:30.081459
#54 9.806   Downloaded md-5 v0.10.6
2026-Mar-23 02:00:30.081459
#54 9.808   Downloaded lru-slab v0.1.2
2026-Mar-23 02:00:30.081459
#54 9.809   Downloaded deranged v0.5.8
2026-Mar-23 02:00:30.081459
#54 9.811   Downloaded if_chain v1.0.3
2026-Mar-23 02:00:30.081459
#54 9.812   Downloaded bitflags v2.11.0
2026-Mar-23 02:00:30.081459
#54 9.817   Downloaded aws-smithy-eventstream v0.60.20
2026-Mar-23 02:00:30.081459
#54 9.820   Downloaded async-trait v0.1.89
2026-Mar-23 02:00:30.081459
#54 9.825   Downloaded bit-vec v0.6.3
2026-Mar-23 02:00:30.081459
#54 9.827   Downloaded bincode v1.3.3
2026-Mar-23 02:00:30.081459
#54 9.830   Downloaded autocfg v1.5.0
2026-Mar-23 02:00:30.081459
#54 9.858   Downloaded v8 v0.101.0
2026-Mar-23 02:00:30.081459
#54 ...
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#53 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-23 02:00:30.081459
#53 10.27    Compiling proc-macro2 v1.0.106
2026-Mar-23 02:00:30.081459
#53 10.27    Compiling unicode-ident v1.0.24
2026-Mar-23 02:00:30.081459
#53 10.27    Compiling quote v1.0.44
2026-Mar-23 02:00:30.081459
#53 10.27    Compiling libc v0.2.182
2026-Mar-23 02:00:30.081459
#53 10.27    Compiling cfg-if v1.0.4
2026-Mar-23 02:00:30.081459
#53 10.27    Compiling serde v1.0.228
2026-Mar-23 02:00:30.081459
#53 10.28    Compiling serde_core v1.0.228
2026-Mar-23 02:00:30.081459
#53 10.30    Compiling version_check v0.9.5
2026-Mar-23 02:00:30.081459
#53 10.32    Compiling pin-project-lite v0.2.16
2026-Mar-23 02:00:30.081459
#53 10.33    Compiling parking_lot_core v0.9.12
2026-Mar-23 02:00:30.081459
#53 10.33    Compiling shlex v1.3.0
2026-Mar-23 02:00:30.081459
#53 10.33    Compiling memchr v2.8.0
2026-Mar-23 02:00:30.081459
#53 10.34    Compiling once_cell v1.21.3
2026-Mar-23 02:00:30.081459
#53 10.34    Compiling bytes v1.11.1
2026-Mar-23 02:00:30.081459
#53 10.35    Compiling scopeguard v1.2.0
2026-Mar-23 02:00:30.081459
#53 10.37    Compiling itoa v1.0.17
2026-Mar-23 02:00:30.081459
#53 10.37    Compiling futures-core v0.3.32
2026-Mar-23 02:00:30.081459
#53 10.38    Compiling find-msvc-tools v0.1.9
2026-Mar-23 02:00:30.081459
#53 10.41    Compiling futures-sink v0.3.32
2026-Mar-23 02:00:30.081459
#53 10.42    Compiling autocfg v1.5.0
2026-Mar-23 02:00:30.081459
#53 10.42    Compiling typenum v1.19.0
2026-Mar-23 02:00:30.081459
#53 10.43    Compiling log v0.4.29
2026-Mar-23 02:00:30.081459
#53 10.44    Compiling slab v0.4.12
2026-Mar-23 02:00:30.081459
#53 10.45    Compiling futures-task v0.3.32
2026-Mar-23 02:00:30.081459
#53 10.46    Compiling futures-io v0.3.32
2026-Mar-23 02:00:30.081459
#53 10.47    Compiling zeroize v1.8.2
2026-Mar-23 02:00:30.081459
#53 10.47    Compiling zerocopy v0.8.39
2026-Mar-23 02:00:30.081459
#53 10.49    Compiling equivalent v1.0.2
2026-Mar-23 02:00:30.081459
#53 10.50    Compiling subtle v2.6.1
2026-Mar-23 02:00:30.081459
#53 10.51    Compiling fnv v1.0.7
2026-Mar-23 02:00:30.081459
#53 10.53    Compiling hashbrown v0.16.1
2026-Mar-23 02:00:30.081459
#53 10.57    Compiling lock_api v0.4.14
2026-Mar-23 02:00:30.081459
#53 10.58    Compiling generic-array v0.14.7
2026-Mar-23 02:00:30.081459
#53 10.62    Compiling futures-channel v0.3.32
2026-Mar-23 02:00:30.081459
#53 10.64    Compiling tracing-core v0.1.36
2026-Mar-23 02:00:30.081459
#53 10.65    Compiling percent-encoding v2.3.2
2026-Mar-23 02:00:30.081459
#53 10.66    Compiling time-core v0.1.8
2026-Mar-23 02:00:30.081459
#53 10.66    Compiling powerfmt v0.2.0
2026-Mar-23 02:00:30.081459
#53 10.69    Compiling icu_properties_data v2.1.2
2026-Mar-23 02:00:30.081459
#53 10.69    Compiling num-conv v0.2.0
2026-Mar-23 02:00:30.081459
#53 10.69    Compiling icu_normalizer_data v2.1.1
2026-Mar-23 02:00:30.081459
#53 10.70    Compiling pin-utils v0.1.0
2026-Mar-23 02:00:30.081459
#53 10.71    Compiling ryu v1.0.23
2026-Mar-23 02:00:30.081459
#53 10.74    Compiling stable_deref_trait v1.2.1
2026-Mar-23 02:00:30.081459
#53 10.75    Compiling crc32fast v1.5.0
2026-Mar-23 02:00:30.081459
#53 10.77    Compiling untrusted v0.9.0
2026-Mar-23 02:00:30.081459
#53 10.83    Compiling num-traits v0.2.19
2026-Mar-23 02:00:30.081459
#53 10.83    Compiling http v1.4.0
2026-Mar-23 02:00:30.081459
#53 10.83    Compiling dunce v1.0.5
2026-Mar-23 02:00:30.081459
#53 10.91    Compiling time-macros v0.2.27
2026-Mar-23 02:00:30.081459
#53 10.91    Compiling deranged v0.5.8
2026-Mar-23 02:00:30.081459
#53 10.92    Compiling fs_extra v1.3.0
2026-Mar-23 02:00:30.081459
#53 10.94    Compiling http v0.2.12
2026-Mar-23 02:00:30.081459
#53 10.95    Compiling zmij v1.0.21
2026-Mar-23 02:00:30.081459
#53 10.95    Compiling form_urlencoded v1.2.2
2026-Mar-23 02:00:30.081459
#53 10.95    Compiling rustls-pki-types v1.14.0
2026-Mar-23 02:00:30.081459
#53 10.96    Compiling writeable v0.6.2
2026-Mar-23 02:00:30.081459
#53 10.96    Compiling litemap v0.8.1
2026-Mar-23 02:00:30.081459
#53 10.97    Compiling tower-service v0.3.3
2026-Mar-23 02:00:30.081459
#53 10.98    Compiling aws-lc-rs v1.16.1
2026-Mar-23 02:00:30.081459
#53 11.03    Compiling outref v0.5.2
2026-Mar-23 02:00:30.081459
#53 11.07    Compiling base64 v0.22.1
2026-Mar-23 02:00:30.081459
#53 11.07    Compiling httparse v1.10.1
2026-Mar-23 02:00:30.081459
#53 11.07    Compiling vsimd v0.8.0
2026-Mar-23 02:00:30.081459
#53 11.07    Compiling try-lock v0.2.5
2026-Mar-23 02:00:30.081459
#53 11.12    Compiling httpdate v1.0.3
2026-Mar-23 02:00:30.081459
#53 11.16    Compiling rustls v0.23.37
2026-Mar-23 02:00:30.081459
#53 11.16    Compiling crossbeam-utils v0.8.21
2026-Mar-23 02:00:30.081459
#53 11.19    Compiling want v0.3.1
2026-Mar-23 02:00:30.081459
#53 11.23    Compiling allocator-api2 v0.2.21
2026-Mar-23 02:00:30.081459
#53 11.25    Compiling atomic-waker v1.1.2
2026-Mar-23 02:00:30.081459
#53 11.25    Compiling tower-layer v0.3.3
2026-Mar-23 02:00:30.081459
#53 11.25    Compiling utf8_iter v1.0.4
2026-Mar-23 02:00:30.081459
#53 11.30    Compiling openssl-probe v0.2.1
2026-Mar-23 02:00:30.081459
#53 11.32    Compiling cpufeatures v0.2.17
2026-Mar-23 02:00:30.081459
#53 11.40    Compiling webpki-roots v1.0.6
2026-Mar-23 02:00:30.081459
#53 11.41    Compiling sync_wrapper v1.0.2
2026-Mar-23 02:00:30.081459
#53 11.50    Compiling serde_json v1.0.149
2026-Mar-23 02:00:30.081459
#53 11.50    Compiling ipnet v2.11.0
2026-Mar-23 02:00:30.081459
#53 11.57    Compiling http-body v1.0.1
2026-Mar-23 02:00:30.081459
#53 11.58    Compiling syn v2.0.117
2026-Mar-23 02:00:30.081459
#53 11.60    Compiling base64-simd v0.8.0
2026-Mar-23 02:00:30.081459
#53 11.60    Compiling bitflags v2.11.0
2026-Mar-23 02:00:30.081459
#53 11.60    Compiling rustversion v1.0.22
2026-Mar-23 02:00:30.081459
#53 11.60    Compiling thiserror v2.0.18
2026-Mar-23 02:00:30.081459
#53 11.72    Compiling http-body v0.4.6
2026-Mar-23 02:00:30.081459
#53 11.74    Compiling home v0.5.12
2026-Mar-23 02:00:30.081459
#53 11.75    Compiling getrandom v0.4.1
2026-Mar-23 02:00:30.081459
#53 11.78    Compiling rustls-native-certs v0.8.3
2026-Mar-23 02:00:30.081459
#53 11.80    Compiling jobserver v0.1.34
2026-Mar-23 02:00:30.081459
#53 11.82    Compiling http-body-util v0.1.3
2026-Mar-23 02:00:30.081459
#53 11.83    Compiling getrandom v0.2.17
2026-Mar-23 02:00:30.081459
#53 11.83    Compiling const-oid v0.9.6
2026-Mar-23 02:00:30.081459
#53 11.90    Compiling errno v0.3.14
2026-Mar-23 02:00:30.081459
#53 11.90    Compiling socket2 v0.6.2
2026-Mar-23 02:00:30.081459
#53 12.00    Compiling mio v1.1.1
2026-Mar-23 02:00:30.081459
#53 12.00    Compiling socket2 v0.5.10
2026-Mar-23 02:00:30.081459
#53 12.04    Compiling hex v0.4.3
2026-Mar-23 02:00:30.081459
#53 12.06    Compiling der v0.6.1
2026-Mar-23 02:00:30.081459
#53 12.13    Compiling pkg-config v0.3.32
2026-Mar-23 02:00:30.081459
#53 12.15    Compiling base64ct v1.8.3
2026-Mar-23 02:00:30.081459
#53 12.16    Compiling num-integer v0.1.46
2026-Mar-23 02:00:30.081459
#53 12.16    Compiling vcpkg v0.2.15
2026-Mar-23 02:00:30.081459
#53 12.18    Compiling ahash v0.8.12
2026-Mar-23 02:00:30.081459
#53 12.21    Compiling cc v1.2.56
2026-Mar-23 02:00:30.081459
#53 12.22    Compiling signal-hook-registry v1.4.8
2026-Mar-23 02:00:30.081459
#53 12.25    Compiling rand_core v0.6.4
2026-Mar-23 02:00:30.081459
#53 12.28    Compiling time v0.3.47
2026-Mar-23 02:00:30.081459
#53 12.28    Compiling block-buffer v0.10.4
2026-Mar-23 02:00:30.081459
#53 12.28    Compiling crypto-common v0.1.7
2026-Mar-23 02:00:30.081459
#53 12.36    Compiling rustls v0.21.12
2026-Mar-23 02:00:30.081459
#53 12.39    Compiling aho-corasick v1.1.4
2026-Mar-23 02:00:30.081459
#53 12.48    Compiling uuid v1.21.0
2026-Mar-23 02:00:30.081459
#53 12.48    Compiling foldhash v0.1.5
2026-Mar-23 02:00:30.081459
#53 12.52    Compiling rustix v0.38.44
2026-Mar-23 02:00:30.081459
#53 12.59    Compiling digest v0.10.7
2026-Mar-23 02:00:30.081459
#53 12.61    Compiling base16ct v0.1.1
2026-Mar-23 02:00:30.081459
#53 12.61    Compiling crc-catalog v2.4.0
2026-Mar-23 02:00:30.081459
#53 12.61    Compiling glob v0.3.3
2026-Mar-23 02:00:30.081459
#53 12.69    Compiling regex-syntax v0.8.10
2026-Mar-23 02:00:30.081459
#53 12.78    Compiling hashbrown v0.15.5
2026-Mar-23 02:00:30.081459
#53 12.81    Compiling ff v0.12.1
2026-Mar-23 02:00:30.081459
#53 12.81    Compiling crypto-bigint v0.4.9
2026-Mar-23 02:00:30.081459
#53 12.83    Compiling concurrent-queue v2.5.0
2026-Mar-23 02:00:30.081459
#53 12.99    Compiling hmac v0.12.1
2026-Mar-23 02:00:30.081459
#53 12.99    Compiling sha2 v0.10.9
2026-Mar-23 02:00:30.081459
#53 12.99    Compiling crc v3.4.0
2026-Mar-23 02:00:30.081459
#53 13.20    Compiling alloc-no-stdlib v2.0.4
2026-Mar-23 02:00:30.081459
#53 13.26    Compiling cmake v0.1.57
2026-Mar-23 02:00:30.081459
#53 13.29    Compiling clang-sys v1.8.1
2026-Mar-23 02:00:30.081459
#53 13.36    Compiling group v0.12.1
2026-Mar-23 02:00:30.081459
#53 13.47    Compiling tinyvec_macros v0.1.1
2026-Mar-23 02:00:30.081459
#53 13.47    Compiling parking v2.2.1
2026-Mar-23 02:00:30.081459
#53 13.55    Compiling semver v1.0.27
2026-Mar-23 02:00:30.081459
#53 13.58    Compiling prettyplease v0.2.37
2026-Mar-23 02:00:30.081459
#53 13.59    Compiling linux-raw-sys v0.4.15
2026-Mar-23 02:00:30.081459
#53 13.72    Compiling spki v0.6.0
2026-Mar-23 02:00:30.081459
#53 13.80    Compiling indexmap v2.13.0
2026-Mar-23 02:00:30.081459
#53 13.80    Compiling event-listener v5.4.1
2026-Mar-23 02:00:30.081459
#53 13.86    Compiling alloc-stdlib v0.2.2
2026-Mar-23 02:00:30.081459
#53 13.92    Compiling signature v1.6.4
2026-Mar-23 02:00:30.081459
#53 13.93    Compiling tinyvec v1.10.0
2026-Mar-23 02:00:30.081459
#53 13.98    Compiling rfc6979 v0.3.1
2026-Mar-23 02:00:30.081459
#53 14.01    Compiling rustc_version v0.4.1
2026-Mar-23 02:00:30.081459
#53 14.01    Compiling pkcs8 v0.9.0
2026-Mar-23 02:00:30.081459
#53 14.17    Compiling getrandom v0.3.4
2026-Mar-23 02:00:30.081459
#53 14.19    Compiling minimal-lexical v0.2.1
2026-Mar-23 02:00:30.081459
#53 14.27    Compiling adler2 v2.0.1
2026-Mar-23 02:00:30.081459
#53 14.29    Compiling foreign-types-shared v0.1.1
2026-Mar-23 02:00:30.081459
#53 14.31    Compiling simd-adler32 v0.3.8
2026-Mar-23 02:00:30.081459
#53 14.31    Compiling openssl v0.10.75
2026-Mar-23 02:00:30.081459
#53 14.46    Compiling sec1 v0.3.0
2026-Mar-23 02:00:30.081459
#53 14.57    Compiling ring v0.17.14
2026-Mar-23 02:00:30.081459
#53 14.57    Compiling aws-lc-sys v0.38.0
2026-Mar-23 02:00:30.081459
#53 14.57    Compiling openssl-sys v0.9.111
2026-Mar-23 02:00:30.081459
#53 14.60    Compiling tokio v1.49.0
2026-Mar-23 02:00:30.081459
#53 14.65    Compiling thiserror v1.0.69
2026-Mar-23 02:00:30.081459
#53 14.65    Compiling foreign-types v0.3.2
2026-Mar-23 02:00:30.081459
#53 14.71    Compiling aws-types v1.3.14
2026-Mar-23 02:00:30.081459
#53 14.79    Compiling unicode-normalization v0.1.25
2026-Mar-23 02:00:30.081459
#53 14.84    Compiling miniz_oxide v0.8.9
2026-Mar-23 02:00:30.081459
#53 14.84    Compiling futures-util v0.3.32
2026-Mar-23 02:00:30.081459
#53 14.84    Compiling hashlink v0.10.0
2026-Mar-23 02:00:30.081459
#53 14.98    Compiling elliptic-curve v0.12.3
2026-Mar-23 02:00:30.081459
#53 14.98    Compiling nom v7.1.3
2026-Mar-23 02:00:30.081459
#53 15.07    Compiling brotli-decompressor v5.0.0
2026-Mar-23 02:00:30.081459
#53 15.07    Compiling webpki-roots v0.26.11
2026-Mar-23 02:00:30.081459
#53 15.19    Compiling crossbeam-queue v0.3.12
2026-Mar-23 02:00:30.081459
#53 15.23    Compiling md-5 v0.10.6
2026-Mar-23 02:00:30.081459
#53 15.32    Compiling libloading v0.8.9
2026-Mar-23 02:00:30.081459
#53 15.36    Compiling regex-automata v0.4.14
2026-Mar-23 02:00:30.081459
#53 15.44    Compiling unicode-properties v0.1.4
2026-Mar-23 02:00:30.081459
#53 15.48    Compiling native-tls v0.2.18
2026-Mar-23 02:00:30.081459
#53 15.52    Compiling bindgen v0.69.5
2026-Mar-23 02:00:30.081459
#53 15.52    Compiling ecdsa v0.14.8
2026-Mar-23 02:00:30.081459
#53 15.52    Compiling anyhow v1.0.102
2026-Mar-23 02:00:30.081459
#53 15.62    Compiling crunchy v0.2.4
2026-Mar-23 02:00:30.081459
#53 15.65    Compiling unicode-bidi v0.3.18
2026-Mar-23 02:00:30.081459
#53 15.69    Compiling atoi v2.0.0
2026-Mar-23 02:00:30.081459
#53 15.81    Compiling flate2 v1.1.9
2026-Mar-23 02:00:30.081459
#53 15.83    Compiling hkdf v0.12.4
2026-Mar-23 02:00:30.081459
#53 15.94    Compiling ppv-lite86 v0.2.21
2026-Mar-23 02:00:30.081459
#53 16.22    Compiling crypto-bigint v0.5.5
2026-Mar-23 02:00:30.081459
#53 16.29    Compiling hashbrown v0.14.5
2026-Mar-23 02:00:30.081459
#53 16.29    Compiling stringprep v0.1.5
2026-Mar-23 02:00:30.081459
#53 16.29    Compiling cookie v0.18.1
2026-Mar-23 02:00:30.081459
#53 16.54    Compiling brotli v8.0.2
2026-Mar-23 02:00:30.081459
#53 16.71    Compiling rand_chacha v0.3.1
2026-Mar-23 02:00:30.081459
#53 16.71    Compiling p256 v0.11.1
2026-Mar-23 02:00:30.081459
#53 16.71    Compiling ucd-trie v0.1.7
2026-Mar-23 02:00:30.081459
#53 16.82    Compiling lazy_static v1.5.0
2026-Mar-23 02:00:30.081459
#53 16.82    Compiling lazycell v1.3.0
2026-Mar-23 02:00:30.081459
#53 16.82    Compiling dotenvy v0.15.7
2026-Mar-23 02:00:30.081459
#53 16.82    Compiling compression-core v0.4.31
2026-Mar-23 02:00:30.081459
#53 17.06    Compiling synstructure v0.13.2
2026-Mar-23 02:00:30.081459
#53 17.06    Compiling cexpr v0.6.0
2026-Mar-23 02:00:30.081459
#53 17.12    Compiling fastrand v2.3.0
2026-Mar-23 02:00:30.081459
#53 17.12    Compiling rustc-hash v1.1.0
2026-Mar-23 02:00:30.081459
#53 17.25    Compiling rand v0.8.5
2026-Mar-23 02:00:30.081459
#53 17.29    Compiling byteorder v1.5.0
2026-Mar-23 02:00:30.081459
#53 17.29    Compiling adler v1.0.2
2026-Mar-23 02:00:30.081459
#53 17.35    Compiling whoami v1.6.1
2026-Mar-23 02:00:30.081459
#53 17.39    Compiling tiny-keccak v2.0.2
2026-Mar-23 02:00:30.081459
#53 17.48    Compiling pest v2.8.6
2026-Mar-23 02:00:30.081459
#53 17.53    Compiling miniz_oxide v0.7.4
2026-Mar-23 02:00:30.081459
#53 17.73    Compiling rand_core v0.9.5
2026-Mar-23 02:00:30.081459
#53 17.98    Compiling gzip-header v1.0.0
2026-Mar-23 02:00:30.081459
#53 18.22    Compiling num-bigint v0.4.6
2026-Mar-23 02:00:30.081459
#53 18.22    Compiling fslock v0.2.1
2026-Mar-23 02:00:30.081459
#53 18.22    Compiling encoding_rs v0.8.35
2026-Mar-23 02:00:30.081459
#53 18.27    Compiling paste v1.0.15
2026-Mar-23 02:00:30.081459
#53 18.39    Compiling radium v0.7.0
2026-Mar-23 02:00:30.081459
#53 18.42    Compiling regex v1.12.3
2026-Mar-23 02:00:30.081459
#53 18.55    Compiling heck v0.5.0
2026-Mar-23 02:00:30.081459
#53 18.60    Compiling mime v0.3.17
2026-Mar-23 02:00:30.081459
#53 18.60    Compiling litrs v1.0.0
2026-Mar-23 02:00:30.081459
#53 18.67    Compiling psl-types v2.0.11
2026-Mar-23 02:00:30.081459
#53 18.85    Compiling rand_chacha v0.9.0
2026-Mar-23 02:00:30.081459
#53 19.15    Compiling arc-swap v1.8.2
2026-Mar-23 02:00:30.081459
#53 19.23    Compiling sha1_smol v1.0.1
2026-Mar-23 02:00:30.081459
#53 19.30    Compiling regex-lite v0.1.9
2026-Mar-23 02:00:30.081459
#53 19.30    Compiling iri-string v0.7.10
2026-Mar-23 02:00:30.081459
#53 19.70    Compiling pest_meta v2.8.6
2026-Mar-23 02:00:30.081459
#53 19.76    Compiling portable-atomic v1.13.1
2026-Mar-23 02:00:30.081459
#53 19.78    Compiling tap v1.0.1
2026-Mar-23 02:00:30.081459
#53 19.80    Compiling heck v0.4.1
2026-Mar-23 02:00:30.081459
#53 19.83    Compiling document-features v0.2.12
2026-Mar-23 02:00:30.081459
#53 19.83    Compiling outref v0.1.0
2026-Mar-23 02:00:30.081459
#53 19.92    Compiling wyz v0.5.1
2026-Mar-23 02:00:30.081459
#53 19.99    Compiling rand v0.9.2
2026-Mar-23 02:00:30.081459
#53 20.05    Compiling simd-abstraction v0.7.1
2026-Mar-23 02:00:30.081459
#53 20.07    Compiling const-random-macro v0.1.16
2026-Mar-23 02:00:30.081459
#53 20.21    Compiling memoffset v0.9.1
2026-Mar-23 02:00:30.081459
#53 20.32    Compiling tokio-stream v0.1.18
2026-Mar-23 02:00:30.081459
#53 20.39    Compiling proc-macro-error-attr v1.0.4
2026-Mar-23 02:00:30.081459
#53 20.42    Compiling xmlparser v0.13.6
2026-Mar-23 02:00:30.081459
#53 20.49    Compiling funty v2.0.0
2026-Mar-23 02:00:30.081459
#53 20.54    Compiling syn v1.0.109
2026-Mar-23 02:00:30.081459
#53 20.60    Compiling const-random v0.1.18
2026-Mar-23 02:00:30.081459
#53 20.72    Compiling pest_generator v2.8.6
2026-Mar-23 02:00:30.081459
#53 20.86    Compiling compression-codecs v0.4.37
2026-Mar-23 02:00:30.081459
#53 20.90    Compiling aws-smithy-xml v0.60.15
2026-Mar-23 02:00:30.081459
#53 20.90    Compiling base64-simd v0.7.0
2026-Mar-23 02:00:30.081459
#53 20.95    Compiling event-listener-strategy v0.5.4
2026-Mar-23 02:00:30.081459
#53 ...
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#54 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-23 02:00:30.081459
#54 10.97   Downloaded libsqlite3-sys v0.30.1
2026-Mar-23 02:00:30.081459
#54 11.20    Compiling proc-macro2 v1.0.106
2026-Mar-23 02:00:30.081459
#54 11.20    Compiling quote v1.0.44
2026-Mar-23 02:00:30.081459
#54 11.20    Compiling unicode-ident v1.0.24
2026-Mar-23 02:00:30.081459
#54 11.20    Compiling libc v0.2.182
2026-Mar-23 02:00:30.081459
#54 11.20    Compiling cfg-if v1.0.4
2026-Mar-23 02:00:30.081459
#54 11.23    Compiling serde v1.0.228
2026-Mar-23 02:00:30.081459
#54 11.26    Compiling serde_core v1.0.228
2026-Mar-23 02:00:30.081459
#54 11.27    Compiling pin-project-lite v0.2.16
2026-Mar-23 02:00:30.081459
#54 11.28    Compiling parking_lot_core v0.9.12
2026-Mar-23 02:00:30.081459
#54 11.29    Compiling shlex v1.3.0
2026-Mar-23 02:00:30.081459
#54 11.29    Compiling bytes v1.11.1
2026-Mar-23 02:00:30.081459
#54 11.29    Compiling scopeguard v1.2.0
2026-Mar-23 02:00:30.081459
#54 11.31    Compiling futures-core v0.3.32
2026-Mar-23 02:00:30.081459
#54 11.34    Compiling find-msvc-tools v0.1.9
2026-Mar-23 02:00:30.081459
#54 11.37    Compiling version_check v0.9.5
2026-Mar-23 02:00:30.081459
#54 11.37    Compiling memchr v2.8.0
2026-Mar-23 02:00:30.081459
#54 11.37    Compiling itoa v1.0.17
2026-Mar-23 02:00:30.081459
#54 11.40    Compiling once_cell v1.21.3
2026-Mar-23 02:00:30.081459
#54 11.44    Compiling futures-sink v0.3.32
2026-Mar-23 02:00:30.081459
#54 11.44    Compiling typenum v1.19.0
2026-Mar-23 02:00:30.081459
#54 11.44    Compiling autocfg v1.5.0
2026-Mar-23 02:00:30.081459
#54 11.44    Compiling log v0.4.29
2026-Mar-23 02:00:30.081459
#54 11.44    Compiling slab v0.4.12
2026-Mar-23 02:00:30.081459
#54 11.44    Compiling futures-task v0.3.32
2026-Mar-23 02:00:30.081459
#54 11.44    Compiling futures-io v0.3.32
2026-Mar-23 02:00:30.081459
#54 11.44    Compiling zeroize v1.8.2
2026-Mar-23 02:00:30.081459
#54 11.44    Compiling subtle v2.6.1
2026-Mar-23 02:00:30.081459
#54 11.44    Compiling fnv v1.0.7
2026-Mar-23 02:00:30.081459
#54 11.44    Compiling equivalent v1.0.2
2026-Mar-23 02:00:30.081459
#54 11.46    Compiling percent-encoding v2.3.2
2026-Mar-23 02:00:30.081459
#54 11.50    Compiling icu_normalizer_data v2.1.1
2026-Mar-23 02:00:30.081459
#54 11.50    Compiling icu_properties_data v2.1.2
2026-Mar-23 02:00:30.081459
#54 11.52    Compiling lock_api v0.4.14
2026-Mar-23 02:00:30.081459
#54 11.58    Compiling zerocopy v0.8.39
2026-Mar-23 02:00:30.081459
#54 11.58    Compiling hashbrown v0.16.1
2026-Mar-23 02:00:30.081459
#54 11.59    Compiling pin-utils v0.1.0
2026-Mar-23 02:00:30.081459
#54 11.59    Compiling time-core v0.1.8
2026-Mar-23 02:00:30.081459
#54 11.67    Compiling futures-channel v0.3.32
2026-Mar-23 02:00:30.081459
#54 11.67    Compiling powerfmt v0.2.0
2026-Mar-23 02:00:30.081459
#54 11.67    Compiling ryu v1.0.23
2026-Mar-23 02:00:30.081459
#54 11.69    Compiling num-conv v0.2.0
2026-Mar-23 02:00:30.081459
#54 11.79    Compiling tracing-core v0.1.36
2026-Mar-23 02:00:30.081459
#54 11.79    Compiling stable_deref_trait v1.2.1
2026-Mar-23 02:00:30.081459
#54 11.93    Compiling crc32fast v1.5.0
2026-Mar-23 02:00:30.081459
#54 11.93    Compiling untrusted v0.9.0
2026-Mar-23 02:00:30.081459
#54 11.93    Compiling dunce v1.0.5
2026-Mar-23 02:00:30.081459
#54 11.96    Compiling fs_extra v1.3.0
2026-Mar-23 02:00:30.081459
#54 11.99    Compiling generic-array v0.14.7
2026-Mar-23 02:00:30.081459
#54 12.00    Compiling form_urlencoded v1.2.2
2026-Mar-23 02:00:30.081459
#54 12.04    Compiling tower-service v0.3.3
2026-Mar-23 02:00:30.081459
#54 12.04    Compiling litemap v0.8.1
2026-Mar-23 02:00:30.081459
#54 12.04    Compiling writeable v0.6.2
2026-Mar-23 02:00:30.081459
#54 12.11    Compiling num-traits v0.2.19
2026-Mar-23 02:00:30.081459
#54 12.11    Compiling aws-lc-rs v1.16.1
2026-Mar-23 02:00:30.081459
#54 12.11    Compiling http v1.4.0
2026-Mar-23 02:00:30.081459
#54 12.11    Compiling http v0.2.12
2026-Mar-23 02:00:30.081459
#54 12.16    Compiling rustls-pki-types v1.14.0
2026-Mar-23 02:00:30.081459
#54 12.16    Compiling httparse v1.10.1
2026-Mar-23 02:00:30.081459
#54 12.16    Compiling zmij v1.0.21
2026-Mar-23 02:00:30.081459
#54 12.21    Compiling deranged v0.5.8
2026-Mar-23 02:00:30.081459
#54 12.22    Compiling time-macros v0.2.27
2026-Mar-23 02:00:30.081459
#54 12.30    Compiling vsimd v0.8.0
2026-Mar-23 02:00:30.081459
#54 12.32    Compiling outref v0.5.2
2026-Mar-23 02:00:30.081459
#54 12.32    Compiling try-lock v0.2.5
2026-Mar-23 02:00:30.081459
#54 12.48    Compiling base64 v0.22.1
2026-Mar-23 02:00:30.081459
#54 12.58    Compiling httpdate v1.0.3
2026-Mar-23 02:00:30.081459
#54 12.64    Compiling rustls v0.23.37
2026-Mar-23 02:00:30.081459
#54 12.69    Compiling atomic-waker v1.1.2
2026-Mar-23 02:00:30.081459
#54 12.69    Compiling cpufeatures v0.2.17
2026-Mar-23 02:00:30.081459
#54 12.71    Compiling want v0.3.1
2026-Mar-23 02:00:30.081459
#54 12.74    Compiling openssl-probe v0.2.1
2026-Mar-23 02:00:30.081459
#54 12.76    Compiling tower-layer v0.3.3
2026-Mar-23 02:00:30.081459
#54 12.88    Compiling webpki-roots v1.0.6
2026-Mar-23 02:00:30.081459
#54 12.88    Compiling utf8_iter v1.0.4
2026-Mar-23 02:00:30.081459
#54 12.88    Compiling sync_wrapper v1.0.2
2026-Mar-23 02:00:30.081459
#54 12.97    Compiling crossbeam-utils v0.8.21
2026-Mar-23 02:00:30.081459
#54 12.97    Compiling ipnet v2.11.0
2026-Mar-23 02:00:30.081459
#54 13.12    Compiling bitflags v2.11.0
2026-Mar-23 02:00:30.081459
#54 13.15    Compiling rustversion v1.0.22
2026-Mar-23 02:00:30.081459
#54 13.26    Compiling home v0.5.12
2026-Mar-23 02:00:30.081459
#54 13.37    Compiling http-body v0.4.6
2026-Mar-23 02:00:30.081459
#54 13.37    Compiling getrandom v0.4.1
2026-Mar-23 02:00:30.081459
#54 13.40    Compiling http-body v1.0.1
2026-Mar-23 02:00:30.081459
#54 13.42    Compiling rustls-native-certs v0.8.3
2026-Mar-23 02:00:30.081459
#54 13.42    Compiling base64-simd v0.8.0
2026-Mar-23 02:00:30.081459
#54 13.65    Compiling errno v0.3.14
2026-Mar-23 02:00:30.081459
#54 13.65    Compiling socket2 v0.6.2
2026-Mar-23 02:00:30.081459
#54 13.65    Compiling mio v1.1.1
2026-Mar-23 02:00:30.081459
#54 13.65    Compiling getrandom v0.2.17
2026-Mar-23 02:00:30.081459
#54 13.66    Compiling socket2 v0.5.10
2026-Mar-23 02:00:30.081459
#54 13.72    Compiling serde_json v1.0.149
2026-Mar-23 02:00:30.081459
#54 13.72    Compiling hex v0.4.3
2026-Mar-23 02:00:30.081459
#54 13.72    Compiling const-oid v0.9.6
2026-Mar-23 02:00:30.081459
#54 13.73    Compiling thiserror v2.0.18
2026-Mar-23 02:00:30.081459
#54 13.90    Compiling http-body-util v0.1.3
2026-Mar-23 02:00:30.081459
#54 13.97    Compiling syn v2.0.117
2026-Mar-23 02:00:30.081459
#54 13.97    Compiling signal-hook-registry v1.4.8
2026-Mar-23 02:00:30.081459
#54 14.01    Compiling num-integer v0.1.46
2026-Mar-23 02:00:30.081459
#54 14.01    Compiling rand_core v0.6.4
2026-Mar-23 02:00:30.081459
#54 14.07    Compiling jobserver v0.1.34
2026-Mar-23 02:00:30.081459
#54 14.07    Compiling time v0.3.47
2026-Mar-23 02:00:30.081459
#54 14.13    Compiling pkg-config v0.3.32
2026-Mar-23 02:00:30.081459
#54 14.17    Compiling der v0.6.1
2026-Mar-23 02:00:30.081459
#54 14.18    Compiling base64ct v1.8.3
2026-Mar-23 02:00:30.081459
#54 14.22    Compiling allocator-api2 v0.2.21
2026-Mar-23 02:00:30.081459
#54 14.22    Compiling vcpkg v0.2.15
2026-Mar-23 02:00:30.081459
#54 14.29    Compiling uuid v1.21.0
2026-Mar-23 02:00:30.081459
#54 14.34    Compiling rustls v0.21.12
2026-Mar-23 02:00:30.081459
#54 14.42    Compiling ff v0.12.1
2026-Mar-23 02:00:30.081459
#54 14.48    Compiling cc v1.2.56
2026-Mar-23 02:00:30.081459
#54 14.53    Compiling base16ct v0.1.1
2026-Mar-23 02:00:30.081459
#54 14.58    Compiling glob v0.3.3
2026-Mar-23 02:00:30.081459
#54 14.58    Compiling foldhash v0.1.5
2026-Mar-23 02:00:30.081459
#54 14.58    Compiling crc-catalog v2.4.0
2026-Mar-23 02:00:30.081459
#54 14.60    Compiling rustix v0.38.44
2026-Mar-23 02:00:30.081459
#54 14.60    Compiling group v0.12.1
2026-Mar-23 02:00:30.081459
#54 14.81    Compiling concurrent-queue v2.5.0
2026-Mar-23 02:00:30.081459
#54 15.01    Compiling indexmap v2.13.0
2026-Mar-23 02:00:30.081459
#54 15.01    Compiling semver v1.0.27
2026-Mar-23 02:00:30.081459
#54 15.01    Compiling alloc-no-stdlib v2.0.4
2026-Mar-23 02:00:30.081459
#54 15.03    Compiling tinyvec_macros v0.1.1
2026-Mar-23 02:00:30.081459
#54 15.16    Compiling block-buffer v0.10.4
2026-Mar-23 02:00:30.081459
#54 15.16    Compiling crypto-common v0.1.7
2026-Mar-23 02:00:30.081459
#54 15.32    Compiling spki v0.6.0
2026-Mar-23 02:00:30.081459
#54 15.32    Compiling crypto-bigint v0.4.9
2026-Mar-23 02:00:30.081459
#54 15.32    Compiling hashbrown v0.15.5
2026-Mar-23 02:00:30.081459
#54 15.35    Compiling parking v2.2.1
2026-Mar-23 02:00:30.081459
#54 15.35    Compiling linux-raw-sys v0.4.15
2026-Mar-23 02:00:30.081459
#54 15.36    Compiling clang-sys v1.8.1
2026-Mar-23 02:00:30.081459
#54 15.39    Compiling crc v3.4.0
2026-Mar-23 02:00:30.081459
#54 15.58    Compiling digest v0.10.7
2026-Mar-23 02:00:30.081459
#54 15.59    Compiling prettyplease v0.2.37
2026-Mar-23 02:00:30.081459
#54 15.59    Compiling cmake v0.1.57
2026-Mar-23 02:00:30.081459
#54 15.59    Compiling pkcs8 v0.9.0
2026-Mar-23 02:00:30.081459
#54 15.65    Compiling rustc_version v0.4.1
2026-Mar-23 02:00:30.081459
#54 15.65    Compiling tinyvec v1.10.0
2026-Mar-23 02:00:30.081459
#54 15.78    Compiling event-listener v5.4.1
2026-Mar-23 02:00:30.081459
#54 15.78    Compiling alloc-stdlib v0.2.2
2026-Mar-23 02:00:30.081459
#54 15.94    Compiling sec1 v0.3.0
2026-Mar-23 02:00:30.081459
#54 16.07    Compiling aho-corasick v1.1.4
2026-Mar-23 02:00:30.081459
#54 16.15    Compiling regex-syntax v0.8.10
2026-Mar-23 02:00:30.081459
#54 16.24    Compiling simd-adler32 v0.3.8
2026-Mar-23 02:00:30.081459
#54 16.29    Compiling openssl v0.10.75
2026-Mar-23 02:00:30.081459
#54 16.40    Compiling tokio v1.49.0
2026-Mar-23 02:00:30.081459
#54 16.50    Compiling ring v0.17.14
2026-Mar-23 02:00:30.081459
#54 16.50    Compiling aws-lc-sys v0.38.0
2026-Mar-23 02:00:30.081459
#54 16.53    Compiling openssl-sys v0.9.111
2026-Mar-23 02:00:30.081459
#54 16.55    Compiling adler2 v2.0.1
2026-Mar-23 02:00:30.081459
#54 16.78    Compiling minimal-lexical v0.2.1
2026-Mar-23 02:00:30.081459
#54 16.83    Compiling hmac v0.12.1
2026-Mar-23 02:00:30.081459
#54 16.83    Compiling sha2 v0.10.9
2026-Mar-23 02:00:30.081459
#54 16.83    Compiling signature v1.6.4
2026-Mar-23 02:00:30.081459
#54 16.84    Compiling elliptic-curve v0.12.3
2026-Mar-23 02:00:30.081459
#54 16.84    Compiling thiserror v1.0.69
2026-Mar-23 02:00:30.081459
#54 16.86    Compiling foreign-types-shared v0.1.1
2026-Mar-23 02:00:30.081459
#54 16.94    Compiling miniz_oxide v0.8.9
2026-Mar-23 02:00:30.081459
#54 16.99    Compiling aws-types v1.3.14
2026-Mar-23 02:00:30.081459
#54 17.01    Compiling rfc6979 v0.3.1
2026-Mar-23 02:00:30.081459
#54 17.01    Compiling hashlink v0.10.0
2026-Mar-23 02:00:30.081459
#54 17.12    Compiling md-5 v0.10.6
2026-Mar-23 02:00:30.081459
#54 17.23    Compiling futures-util v0.3.32
2026-Mar-23 02:00:30.081459
#54 17.23    Compiling nom v7.1.3
2026-Mar-23 02:00:30.081459
#54 17.23    Compiling crossbeam-queue v0.3.12
2026-Mar-23 02:00:30.081459
#54 17.35    Compiling unicode-normalization v0.1.25
2026-Mar-23 02:00:30.081459
#54 17.40    Compiling brotli-decompressor v5.0.0
2026-Mar-23 02:00:30.081459
#54 17.58    Compiling ecdsa v0.14.8
2026-Mar-23 02:00:30.081459
#54 17.58    Compiling foreign-types v0.3.2
2026-Mar-23 02:00:30.081459
#54 17.58    Compiling webpki-roots v0.26.11
2026-Mar-23 02:00:30.081459
#54 17.68    Compiling libloading v0.8.9
2026-Mar-23 02:00:30.081459
#54 17.68    Compiling getrandom v0.3.4
2026-Mar-23 02:00:30.081459
#54 17.72    Compiling unicode-properties v0.1.4
2026-Mar-23 02:00:30.081459
#54 17.81    Compiling native-tls v0.2.18
2026-Mar-23 02:00:30.081459
#54 17.82    Compiling anyhow v1.0.102
2026-Mar-23 02:00:30.081459
#54 17.82    Compiling unicode-bidi v0.3.18
2026-Mar-23 02:00:30.081459
#54 17.91    Compiling ppv-lite86 v0.2.21
2026-Mar-23 02:00:30.081459
#54 18.05    Compiling bindgen v0.69.5
2026-Mar-23 02:00:30.081459
#54 18.08    Compiling p256 v0.11.1
2026-Mar-23 02:00:30.081459
#54 18.20    Compiling flate2 v1.1.9
2026-Mar-23 02:00:30.081459
#54 18.20    Compiling hkdf v0.12.4
2026-Mar-23 02:00:30.081459
#54 18.44    Compiling rand_chacha v0.3.1
2026-Mar-23 02:00:30.081459
#54 18.52    Compiling crypto-bigint v0.5.5
2026-Mar-23 02:00:30.081459
#54 18.58    Compiling stringprep v0.1.5
2026-Mar-23 02:00:30.081459
#54 18.63    Compiling cookie v0.18.1
2026-Mar-23 02:00:30.081459
#54 18.66    Compiling lazy_static v1.5.0
2026-Mar-23 02:00:30.081459
#54 18.69    Compiling dotenvy v0.15.7
2026-Mar-23 02:00:30.081459
#54 18.81    Compiling atoi v2.0.0
2026-Mar-23 02:00:30.081459
#54 18.84    Compiling brotli v8.0.2
2026-Mar-23 02:00:30.081459
#54 18.91    Compiling fastrand v2.3.0
2026-Mar-23 02:00:30.081459
#54 18.97    Compiling lazycell v1.3.0
2026-Mar-23 02:00:30.081459
#54 19.00    Compiling adler v1.0.2
2026-Mar-23 02:00:30.081459
#54 19.21    Compiling rand v0.8.5
2026-Mar-23 02:00:30.081459
#54 19.23    Compiling regex-automata v0.4.14
2026-Mar-23 02:00:30.081459
#54 19.24    Compiling byteorder v1.5.0
2026-Mar-23 02:00:30.081459
#54 19.25    Compiling compression-core v0.4.31
2026-Mar-23 02:00:30.081459
#54 19.25    Compiling rustc-hash v1.1.0
2026-Mar-23 02:00:30.081459
#54 19.37    Compiling whoami v1.6.1
2026-Mar-23 02:00:30.081459
#54 19.44    Compiling gzip-header v1.0.0
2026-Mar-23 02:00:30.081459
#54 19.53    Compiling cexpr v0.6.0
2026-Mar-23 02:00:30.081459
#54 19.53    Compiling miniz_oxide v0.7.4
2026-Mar-23 02:00:30.081459
#54 19.72    Compiling fslock v0.2.1
2026-Mar-23 02:00:30.081459
#54 19.99    Compiling synstructure v0.13.2
2026-Mar-23 02:00:30.081459
#54 20.08    Compiling ahash v0.8.12
2026-Mar-23 02:00:30.081459
#54 20.13    Compiling litrs v1.0.0
2026-Mar-23 02:00:30.081459
#54 20.13    Compiling ucd-trie v0.1.7
2026-Mar-23 02:00:30.081459
#54 20.18    Compiling psl-types v2.0.11
2026-Mar-23 02:00:30.081459
#54 20.19    Compiling radium v0.7.0
2026-Mar-23 02:00:30.081459
#54 20.31    Compiling heck v0.5.0
2026-Mar-23 02:00:30.081459
#54 20.31    Compiling paste v1.0.15
2026-Mar-23 02:00:30.081459
#54 20.37    Compiling num-bigint v0.4.6
2026-Mar-23 02:00:30.081459
#54 20.39    Compiling mime v0.3.17
2026-Mar-23 02:00:30.081459
#54 20.44    Compiling pest v2.8.6
2026-Mar-23 02:00:30.081459
#54 20.56    Compiling rand_core v0.9.5
2026-Mar-23 02:00:30.081459
#54 20.79    Compiling arc-swap v1.8.2
2026-Mar-23 02:00:30.081459
#54 20.94    Compiling iri-string v0.7.10
2026-Mar-23 02:00:30.081459
#54 21.02    Compiling heck v0.4.1
2026-Mar-23 02:00:30.081459
#54 21.08    Compiling sha1_smol v1.0.1
2026-Mar-23 02:00:30.081459
#54 21.11    Compiling tap v1.0.1
2026-Mar-23 02:00:30.081459
#54 ...
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#53 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-23 02:00:30.081459
#53 21.04    Compiling serde_path_to_error v0.1.20
2026-Mar-23 02:00:30.081459
#53 21.06    Compiling sha1 v0.10.6
2026-Mar-23 02:00:30.081459
#53 21.10    Compiling proc-macro-error v1.0.4
2026-Mar-23 02:00:30.081459
#53 21.10    Compiling bitvec v1.0.1
2026-Mar-23 02:00:30.081459
#53 21.23    Compiling if_chain v1.0.3
2026-Mar-23 02:00:30.081459
#53 21.31    Compiling matchit v0.8.4
2026-Mar-23 02:00:30.081459
#53 21.46    Compiling unicode-id-start v1.4.0
2026-Mar-23 02:00:30.081459
#53 21.52    Compiling urlencoding v2.1.3
2026-Mar-23 02:00:30.081459
#53 21.53    Compiling bit-vec v0.6.3
2026-Mar-23 02:00:30.081459
#53 21.63    Compiling data-encoding v2.10.0
2026-Mar-23 02:00:30.081459
#53 21.79    Compiling bit-set v0.5.3
2026-Mar-23 02:00:30.081459
#53 21.82    Compiling serde_derive v1.0.228
2026-Mar-23 02:00:30.081459
#53 21.83    Compiling tokio-macros v2.6.0
2026-Mar-23 02:00:30.081459
#53 21.85    Compiling zerofrom-derive v0.1.6
2026-Mar-23 02:00:30.081459
#53 21.88    Compiling yoke-derive v0.8.1
2026-Mar-23 02:00:30.081459
#53 21.92    Compiling tracing-attributes v0.1.31
2026-Mar-23 02:00:30.081459
#53 21.99    Compiling futures-macro v0.3.32
2026-Mar-23 02:00:30.081459
#53 22.03    Compiling zerovec-derive v0.11.2
2026-Mar-23 02:00:30.081459
#53 22.06    Compiling displaydoc v0.2.5
2026-Mar-23 02:00:30.081459
#53 22.17    Compiling thiserror-impl v2.0.18
2026-Mar-23 02:00:30.081459
#53 22.24    Compiling openssl-macros v0.1.1
2026-Mar-23 02:00:30.081459
#53 22.26    Compiling thiserror-impl v1.0.69
2026-Mar-23 02:00:30.081459
#53 22.37    Compiling async-trait v0.1.89
2026-Mar-23 02:00:30.081459
#53 22.42    Compiling proc-macro-rules-macros v0.4.0
2026-Mar-23 02:00:30.081459
#53 22.50    Compiling strum_macros v0.25.3
2026-Mar-23 02:00:30.081459
#53 22.62    Compiling pest_derive v2.8.6
2026-Mar-23 02:00:30.081459
#53 22.64    Compiling pin-project-internal v1.1.10
2026-Mar-23 02:00:30.081459
#53 22.68    Compiling async-lock v3.4.2
2026-Mar-23 02:00:30.081459
#53 23.01    Compiling dlv-list v0.5.2
2026-Mar-23 02:00:30.081459
#53 23.07    Compiling crossbeam-channel v0.5.15
2026-Mar-23 02:00:30.081459
#53 23.25    Compiling crossbeam-epoch v0.9.18
2026-Mar-23 02:00:30.081459
#53 23.26    Compiling toml_write v0.1.2
2026-Mar-23 02:00:30.081459
#53 23.28    Compiling cooked-waker v5.0.0
2026-Mar-23 02:00:30.081459
#53 23.29    Compiling winnow v0.7.14
2026-Mar-23 02:00:30.081459
#53 23.50    Compiling extractor v0.1.0 (/app/crates/extractor)
2026-Mar-23 02:00:30.081459
#53 23.65    Compiling proc-macro-rules v0.4.0
2026-Mar-23 02:00:30.081459
#53 23.67    Compiling deno_core_icudata v0.0.73
2026-Mar-23 02:00:30.081459
#53 23.67    Compiling tagptr v0.2.0
2026-Mar-23 02:00:30.081459
#53 23.70    Compiling static_assertions v1.1.0
2026-Mar-23 02:00:30.081459
#53 23.70    Compiling ordered-multimap v0.7.3
2026-Mar-23 02:00:30.081459
#53 23.77    Compiling async-stream-impl v0.3.6
2026-Mar-23 02:00:30.081459
#53 23.81    Compiling hashlink v0.8.4
2026-Mar-23 02:00:30.081459
#53 23.86    Compiling raw-cpuid v11.6.0
2026-Mar-23 02:00:30.081459
#53 23.92    Compiling lru v0.12.5
2026-Mar-23 02:00:30.081459
#53 23.96    Compiling num_cpus v1.17.0
2026-Mar-23 02:00:30.081459
#53 23.98    Compiling zerofrom v0.1.6
2026-Mar-23 02:00:30.081459
#53 24.16    Compiling unicode-segmentation v1.12.0
2026-Mar-23 02:00:30.081459
#53 24.19    Compiling arraydeque v0.5.1
2026-Mar-23 02:00:30.081459
#53 24.26    Compiling base64 v0.21.7
2026-Mar-23 02:00:30.081459
#53 24.29    Compiling tracing v0.1.44
2026-Mar-23 02:00:30.081459
#53 24.33    Compiling pin-project v1.1.10
2026-Mar-23 02:00:30.081459
#53 24.33    Compiling async-stream v0.3.6
2026-Mar-23 02:00:30.081459
#53 24.40    Compiling yoke v0.8.1
2026-Mar-23 02:00:30.081459
#53 24.41    Compiling utoipa-gen v4.3.1
2026-Mar-23 02:00:30.081459
#53 24.41    Compiling sharded-slab v0.1.7
2026-Mar-23 02:00:30.081459
#53 24.43    Compiling rust-ini v0.20.0
2026-Mar-23 02:00:30.081459
#53 24.52    Compiling pem v3.0.6
2026-Mar-23 02:00:30.081459
#53 24.56    Compiling tracing-log v0.2.0
2026-Mar-23 02:00:30.081459
#53 24.64    Compiling zerovec v0.11.5
2026-Mar-23 02:00:30.081459
#53 24.67    Compiling zerotrie v0.2.3
2026-Mar-23 02:00:30.081459
#53 24.70    Compiling spinning_top v0.3.0
2026-Mar-23 02:00:30.081459
#53 24.75    Compiling convert_case v0.6.0
2026-Mar-23 02:00:30.081459
#53 24.88    Compiling axum-core v0.5.6
2026-Mar-23 02:00:30.081459
#53 25.00    Compiling tower v0.4.13
2026-Mar-23 02:00:30.081459
#53 25.06    Compiling yaml-rust2 v0.8.1
2026-Mar-23 02:00:30.081459
#53 25.06    Compiling thread_local v1.1.9
2026-Mar-23 02:00:30.081459
#53 25.06    Compiling no-std-compat v0.4.1
2026-Mar-23 02:00:30.081459
#53 25.15    Compiling simple_asn1 v0.6.4
2026-Mar-23 02:00:30.081459
#53 25.16    Compiling strum v0.25.0
2026-Mar-23 02:00:30.081459
#53 25.16    Compiling pathdiff v0.2.3
2026-Mar-23 02:00:30.081459
#53 25.23    Compiling nonzero_ext v0.3.0
2026-Mar-23 02:00:30.081459
#53 25.25    Compiling web-time v1.1.0
2026-Mar-23 02:00:30.081459
#53 25.28    Compiling nu-ansi-term v0.50.3
2026-Mar-23 02:00:30.081459
#53 25.31    Compiling futures-timer v3.0.3
2026-Mar-23 02:00:30.081459
#53 25.34    Compiling deno_ops v0.176.0
2026-Mar-23 02:00:30.081459
#53 25.49    Compiling tinystr v0.8.2
2026-Mar-23 02:00:30.081459
#53 25.49    Compiling potential_utf v0.1.4
2026-Mar-23 02:00:30.081459
#53 25.65    Compiling icu_collections v2.1.1
2026-Mar-23 02:00:30.081459
#53 25.91    Compiling icu_locale_core v2.1.1
2026-Mar-23 02:00:30.081459
#53 26.27    Compiling matchers v0.2.0
2026-Mar-23 02:00:30.081459
#53 27.24    Compiling icu_provider v2.1.1
2026-Mar-23 02:00:30.081459
#53 27.39    Compiling smallvec v1.15.1
2026-Mar-23 02:00:30.081459
#53 27.43    Compiling either v1.15.0
2026-Mar-23 02:00:30.081459
#53 27.43    Compiling serde_urlencoded v0.7.1
2026-Mar-23 02:00:30.081459
#53 27.46    Compiling debugid v0.8.0
2026-Mar-23 02:00:30.081459
#53 27.46    Compiling json5 v0.4.1
2026-Mar-23 02:00:30.081459
#53 27.46    Compiling toml_datetime v0.6.11
2026-Mar-23 02:00:30.081459
#53 27.50    Compiling serde_spanned v0.6.9
2026-Mar-23 02:00:30.081459
#53 27.58    Compiling bincode v1.3.3
2026-Mar-23 02:00:30.081459
#53 28.01    Compiling icu_properties v2.1.2
2026-Mar-23 02:00:30.081459
#53 28.07    Compiling parking_lot v0.12.5
2026-Mar-23 02:00:30.081459
#53 28.10    Compiling which v4.4.2
2026-Mar-23 02:00:30.081459
#53 28.11    Compiling bytes-utils v0.1.4
2026-Mar-23 02:00:30.081459
#53 28.19    Compiling icu_normalizer v2.1.1
2026-Mar-23 02:00:30.081459
#53 28.27    Compiling itertools v0.12.1
2026-Mar-23 02:00:30.081459
#53 28.27    Compiling which v6.0.3
2026-Mar-23 02:00:30.081459
#53 28.34    Compiling itertools v0.13.0
2026-Mar-23 02:00:30.081459
#53 28.40    Compiling futures-executor v0.3.32
2026-Mar-23 02:00:30.081459
#53 28.52    Compiling futures-intrusive v0.5.0
2026-Mar-23 02:00:30.081459
#53 28.57    Compiling dashmap v6.1.0
2026-Mar-23 02:00:30.081459
#53 28.62    Compiling moka v0.12.13
2026-Mar-23 02:00:30.081459
#53 28.62    Compiling toml_edit v0.22.27
2026-Mar-23 02:00:30.081459
#53 28.68    Compiling quanta v0.12.6
2026-Mar-23 02:00:30.081459
#53 28.71    Compiling ron v0.8.1
2026-Mar-23 02:00:30.081459
#53 28.71    Compiling tracing-subscriber v0.3.22
2026-Mar-23 02:00:30.081459
#53 28.84    Compiling futures v0.3.32
2026-Mar-23 02:00:30.081459
#53 29.08    Compiling rustls-webpki v0.103.9
2026-Mar-23 02:00:30.081459
#53 29.48    Compiling idna_adapter v1.2.1
2026-Mar-23 02:00:30.081459
#53 29.66    Compiling idna v1.1.0
2026-Mar-23 02:00:30.081459
#53 29.90    Compiling utoipa v4.2.3
2026-Mar-23 02:00:30.081459
#53 30.09    Compiling url v2.5.8
2026-Mar-23 02:00:30.081459
#53 30.15    Compiling governor v0.8.1
2026-Mar-23 02:00:30.081459
#53 31.00    Compiling toml v0.8.23
2026-Mar-23 02:00:30.081459
#53 31.07    Compiling publicsuffix v2.3.0
2026-Mar-23 02:00:30.081459
#53 ...
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#54 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-23 02:00:30.081459
#54 21.36    Compiling regex-lite v0.1.9
2026-Mar-23 02:00:30.081459
#54 21.36    Compiling outref v0.1.0
2026-Mar-23 02:00:30.081459
#54 21.43    Compiling document-features v0.2.12
2026-Mar-23 02:00:30.081459
#54 21.53    Compiling rand_chacha v0.9.0
2026-Mar-23 02:00:30.081459
#54 21.67    Compiling tokio-stream v0.1.18
2026-Mar-23 02:00:30.081459
#54 21.67    Compiling wyz v0.5.1
2026-Mar-23 02:00:30.081459
#54 21.76    Compiling simd-abstraction v0.7.1
2026-Mar-23 02:00:30.081459
#54 21.91    Compiling memoffset v0.9.1
2026-Mar-23 02:00:30.081459
#54 21.95    Compiling encoding_rs v0.8.35
2026-Mar-23 02:00:30.081459
#54 21.96    Compiling portable-atomic v1.13.1
2026-Mar-23 02:00:30.081459
#54 22.15    Compiling funty v2.0.0
2026-Mar-23 02:00:30.081459
#54 22.19    Compiling xmlparser v0.13.6
2026-Mar-23 02:00:30.081459
#54 22.24    Compiling hashbrown v0.14.5
2026-Mar-23 02:00:30.081459
#54 22.32    Compiling rand v0.9.2
2026-Mar-23 02:00:30.081459
#54 22.34    Compiling sha1 v0.10.6
2026-Mar-23 02:00:30.081459
#54 22.36    Compiling event-listener-strategy v0.5.4
2026-Mar-23 02:00:30.081459
#54 22.57    Compiling base64-simd v0.7.0
2026-Mar-23 02:00:30.081459
#54 22.64    Compiling serde_path_to_error v0.1.20
2026-Mar-23 02:00:30.081459
#54 22.75    Compiling if_chain v1.0.3
2026-Mar-23 02:00:30.081459
#54 22.75    Compiling bit-vec v0.6.3
2026-Mar-23 02:00:30.081459
#54 22.81    Compiling aws-smithy-xml v0.60.15
2026-Mar-23 02:00:30.081459
#54 22.82    Compiling matchit v0.8.4
2026-Mar-23 02:00:30.081459
#54 22.98    Compiling regex v1.12.3
2026-Mar-23 02:00:30.081459
#54 23.01    Compiling bitvec v1.0.1
2026-Mar-23 02:00:30.081459
#54 23.13    Compiling unicode-id-start v1.4.0
2026-Mar-23 02:00:30.081459
#54 23.17    Compiling data-encoding v2.10.0
2026-Mar-23 02:00:30.081459
#54 23.20    Compiling urlencoding v2.1.3
2026-Mar-23 02:00:30.081459
#54 23.44    Compiling pest_meta v2.8.6
2026-Mar-23 02:00:30.081459
#54 23.57    Compiling bit-set v0.5.3
2026-Mar-23 02:00:30.081459
#54 23.61    Compiling async-lock v3.4.2
2026-Mar-23 02:00:30.081459
#54 23.66    Compiling crossbeam-channel v0.5.15
2026-Mar-23 02:00:30.081459
#54 23.69    Compiling serde_derive v1.0.228
2026-Mar-23 02:00:30.081459
#54 23.71    Compiling tokio-macros v2.6.0
2026-Mar-23 02:00:30.081459
#54 23.71    Compiling zerofrom-derive v0.1.6
2026-Mar-23 02:00:30.081459
#54 23.74    Compiling yoke-derive v0.8.1
2026-Mar-23 02:00:30.081459
#54 23.83    Compiling tracing-attributes v0.1.31
2026-Mar-23 02:00:30.081459
#54 23.83    Compiling zerovec-derive v0.11.2
2026-Mar-23 02:00:30.081459
#54 23.89    Compiling futures-macro v0.3.32
2026-Mar-23 02:00:30.081459
#54 24.01    Compiling displaydoc v0.2.5
2026-Mar-23 02:00:30.081459
#54 24.01    Compiling thiserror-impl v2.0.18
2026-Mar-23 02:00:30.081459
#54 24.10    Compiling openssl-macros v0.1.1
2026-Mar-23 02:00:30.081459
#54 24.10    Compiling thiserror-impl v1.0.69
2026-Mar-23 02:00:30.081459
#54 24.10    Compiling async-trait v0.1.89
2026-Mar-23 02:00:30.081459
#54 24.17    Compiling strum_macros v0.25.3
2026-Mar-23 02:00:30.081459
#54 24.32    Compiling proc-macro-rules-macros v0.4.0
2026-Mar-23 02:00:30.081459
#54 24.72    Compiling pest_generator v2.8.6
2026-Mar-23 02:00:30.081459
#54 24.83    Compiling pin-project-internal v1.1.10
2026-Mar-23 02:00:30.081459
#54 24.85    Compiling crossbeam-epoch v0.9.18
2026-Mar-23 02:00:30.081459
#54 24.99    Compiling compression-codecs v0.4.37
2026-Mar-23 02:00:30.081459
#54 25.30    Compiling cooked-waker v5.0.0
2026-Mar-23 02:00:30.081459
#54 25.43    Compiling tagptr v0.2.0
2026-Mar-23 02:00:30.081459
#54 25.43    Compiling static_assertions v1.1.0
2026-Mar-23 02:00:30.081459
#54 25.53    Compiling deno_core_icudata v0.0.73
2026-Mar-23 02:00:30.081459
#54 25.63    Compiling extractor v0.1.0 (/app/crates/extractor)
2026-Mar-23 02:00:30.081459
#54 25.67    Compiling async-stream-impl v0.3.6
2026-Mar-23 02:00:30.081459
#54 25.69    Compiling lru v0.12.5
2026-Mar-23 02:00:30.081459
#54 25.73    Compiling num_cpus v1.17.0
2026-Mar-23 02:00:30.081459
#54 25.89    Compiling tracing-log v0.2.0
2026-Mar-23 02:00:30.081459
#54 26.05    Compiling thread_local v1.1.9
2026-Mar-23 02:00:30.081459
#54 26.12    Compiling tracing v0.1.44
2026-Mar-23 02:00:30.081459
#54 26.18    Compiling nu-ansi-term v0.50.3
2026-Mar-23 02:00:30.081459
#54 26.18    Compiling sharded-slab v0.1.7
2026-Mar-23 02:00:30.081459
#54 26.29    Compiling proc-macro-rules v0.4.0
2026-Mar-23 02:00:30.081459
#54 26.46    Compiling axum-core v0.5.6
2026-Mar-23 02:00:30.081459
#54 26.51    Compiling tower v0.4.13
2026-Mar-23 02:00:30.081459
#54 26.59    Compiling pin-project v1.1.10
2026-Mar-23 02:00:30.081459
#54 26.73    Compiling pest_derive v2.8.6
2026-Mar-23 02:00:30.081459
#54 26.74    Compiling async-stream v0.3.6
2026-Mar-23 02:00:30.081459
#54 26.80    Compiling strum v0.25.0
2026-Mar-23 02:00:30.081459
#54 26.92    Compiling zerofrom v0.1.6
2026-Mar-23 02:00:30.081459
#54 27.14    Compiling deno_ops v0.176.0
2026-Mar-23 02:00:30.081459
#54 27.20    Compiling yoke v0.8.1
2026-Mar-23 02:00:30.081459
#54 27.43    Compiling zerovec v0.11.5
2026-Mar-23 02:00:30.081459
#54 27.43    Compiling zerotrie v0.2.3
2026-Mar-23 02:00:30.081459
#54 27.86    Compiling rustls-webpki v0.103.9
2026-Mar-23 02:00:30.081459
#54 28.29    Compiling tinystr v0.8.2
2026-Mar-23 02:00:30.081459
#54 28.35    Compiling potential_utf v0.1.4
2026-Mar-23 02:00:30.081459
#54 28.39    Compiling either v1.15.0
2026-Mar-23 02:00:30.081459
#54 28.39    Compiling smallvec v1.15.1
2026-Mar-23 02:00:30.081459
#54 28.57    Compiling icu_locale_core v2.1.1
2026-Mar-23 02:00:30.081459
#54 28.61    Compiling icu_collections v2.1.1
2026-Mar-23 02:00:30.081459
#54 28.64    Compiling matchers v0.2.0
2026-Mar-23 02:00:30.081459
#54 28.68    Compiling itertools v0.12.1
2026-Mar-23 02:00:30.081459
#54 28.68    Compiling which v4.4.2
2026-Mar-23 02:00:30.081459
#54 28.71    Compiling which v6.0.3
2026-Mar-23 02:00:30.081459
#54 28.95    Compiling serde_urlencoded v0.7.1
2026-Mar-23 02:00:30.081459
#54 28.96    Compiling debugid v0.8.0
2026-Mar-23 02:00:30.081459
#54 28.96    Compiling bincode v1.3.3
2026-Mar-23 02:00:30.081459
#54 28.96    Compiling json5 v0.4.1
2026-Mar-23 02:00:30.081459
#54 29.41    Compiling bytes-utils v0.1.4
2026-Mar-23 02:00:30.081459
#54 29.42    Compiling itertools v0.13.0
2026-Mar-23 02:00:30.081459
#54 29.48    Compiling tracing-subscriber v0.3.22
2026-Mar-23 02:00:30.081459
#54 29.78    Compiling parking_lot v0.12.5
2026-Mar-23 02:00:30.081459
#54 29.82    Compiling dashmap v6.1.0
2026-Mar-23 02:00:30.081459
#54 30.15    Compiling icu_provider v2.1.1
2026-Mar-23 02:00:30.081459
#54 30.28    Compiling futures-intrusive v0.5.0
2026-Mar-23 02:00:30.081459
#54 30.45    Compiling icu_properties v2.1.2
2026-Mar-23 02:00:30.081459
#54 30.50    Compiling icu_normalizer v2.1.1
2026-Mar-23 02:00:30.081459
#54 31.17    Compiling rustls-webpki v0.101.7
2026-Mar-23 02:00:30.081459
#54 31.20    Compiling sct v0.7.1
2026-Mar-23 02:00:30.081459
#54 ...
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#53 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-23 02:00:30.081459
#53 31.63    Compiling rustls-webpki v0.101.7
2026-Mar-23 02:00:30.081459
#53 31.63    Compiling sct v0.7.1
2026-Mar-23 02:00:30.081459
#53 31.63    Compiling jsonwebtoken v9.3.1
2026-Mar-23 02:00:30.081459
#53 32.33    Compiling config v0.14.1
2026-Mar-23 02:00:30.081459
#53 32.54    Compiling cookie_store v0.22.1
2026-Mar-23 02:00:30.081459
#53 32.54    Compiling sourcemap v8.0.1
2026-Mar-23 02:00:30.081459
#53 35.17    Compiling sqlx-core v0.8.6
2026-Mar-23 02:00:30.081459
#53 36.89    Compiling tokio-util v0.7.18
2026-Mar-23 02:00:30.081459
#53 36.89    Compiling aws-smithy-async v1.2.14
2026-Mar-23 02:00:30.081459
#53 36.89    Compiling tower v0.5.3
2026-Mar-23 02:00:30.081459
#53 36.89    Compiling tokio-rustls v0.24.1
2026-Mar-23 02:00:30.081459
#53 36.89    Compiling async-compression v0.4.41
2026-Mar-23 02:00:30.081459
#53 36.89    Compiling tokio-native-tls v0.3.1
2026-Mar-23 02:00:30.081459
#53 36.89    Compiling deno_unsync v0.4.4
2026-Mar-23 02:00:30.081459
#53 37.22    Compiling v8 v0.101.0
2026-Mar-23 02:00:30.081459
#53 37.40    Compiling aws-smithy-types v1.4.6
2026-Mar-23 02:00:30.081459
#53 37.40    Compiling h2 v0.4.13
2026-Mar-23 02:00:30.081459
#53 37.40    Compiling h2 v0.3.27
2026-Mar-23 02:00:30.081459
#53 37.40    Compiling combine v4.6.7
2026-Mar-23 02:00:30.081459
#53 37.47    Compiling tower-http v0.6.8
2026-Mar-23 02:00:30.081459
#53 37.81    Compiling sqlx-postgres v0.8.6
2026-Mar-23 02:00:30.081459
#53 38.16    Compiling aws-smithy-runtime-api v1.11.6
2026-Mar-23 02:00:30.081459
#53 38.16    Compiling aws-smithy-eventstream v0.60.20
2026-Mar-23 02:00:30.081459
#53 38.16    Compiling aws-smithy-json v0.62.5
2026-Mar-23 02:00:30.081459
#53 38.17    Compiling aws-smithy-query v0.60.15
2026-Mar-23 02:00:30.081459
#53 38.18    Compiling aws-smithy-json v0.61.9
2026-Mar-23 02:00:30.081459
#53 39.35    Compiling aws-smithy-http v0.63.6
2026-Mar-23 02:00:30.081459
#53 39.35    Compiling aws-credential-types v1.2.14
2026-Mar-23 02:00:30.081459
#53 39.35    Compiling aws-smithy-observability v0.2.6
2026-Mar-23 02:00:30.081459
#53 39.35    Compiling aws-smithy-http v0.62.6
2026-Mar-23 02:00:30.081459
#53 39.63    Compiling crc-fast v1.6.0
2026-Mar-23 02:00:30.081459
#53 39.65    Compiling aws-sigv4 v1.4.2
2026-Mar-23 02:00:30.081459
#53 ...
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#54 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-23 02:00:30.081459
#54 32.40    Compiling idna_adapter v1.2.1
2026-Mar-23 02:00:30.081459
#54 32.62    Compiling idna v1.1.0
2026-Mar-23 02:00:30.081459
#54 33.02    Compiling futures-executor v0.3.32
2026-Mar-23 02:00:30.081459
#54 33.02    Compiling moka v0.12.13
2026-Mar-23 02:00:30.081459
#54 33.21    Compiling url v2.5.8
2026-Mar-23 02:00:30.081459
#54 33.21    Compiling publicsuffix v2.3.0
2026-Mar-23 02:00:30.081459
#54 33.79    Compiling futures v0.3.32
2026-Mar-23 02:00:30.081459
#54 34.32    Compiling sqlx-core v0.8.6
2026-Mar-23 02:00:30.081459
#54 34.98    Compiling cookie_store v0.22.1
2026-Mar-23 02:00:30.081459
#54 35.08    Compiling sourcemap v8.0.1
2026-Mar-23 02:00:30.081459
#54 36.57    Compiling v8 v0.101.0
2026-Mar-23 02:00:30.081459
#54 37.27    Compiling sqlx-postgres v0.8.6
2026-Mar-23 02:00:30.081459
#54 37.35    Compiling tokio-util v0.7.18
2026-Mar-23 02:00:30.081459
#54 37.35    Compiling aws-smithy-async v1.2.14
2026-Mar-23 02:00:30.081459
#54 37.35    Compiling tower v0.5.3
2026-Mar-23 02:00:30.081459
#54 37.36    Compiling tokio-rustls v0.24.1
2026-Mar-23 02:00:30.081459
#54 37.37    Compiling async-compression v0.4.41
2026-Mar-23 02:00:30.081459
#54 37.39    Compiling tokio-native-tls v0.3.1
2026-Mar-23 02:00:30.081459
#54 37.40    Compiling deno_unsync v0.4.4
2026-Mar-23 02:00:30.081459
#54 37.83    Compiling aws-smithy-types v1.4.6
2026-Mar-23 02:00:30.081459
#54 37.83    Compiling h2 v0.4.13
2026-Mar-23 02:00:30.081459
#54 37.83    Compiling h2 v0.3.27
2026-Mar-23 02:00:30.081459
#54 37.83    Compiling combine v4.6.7
2026-Mar-23 02:00:30.081459
#54 37.89    Compiling tower-http v0.6.8
2026-Mar-23 02:00:30.081459
#54 38.61    Compiling aws-smithy-runtime-api v1.11.6
2026-Mar-23 02:00:30.081459
#54 38.62    Compiling aws-smithy-eventstream v0.60.20
2026-Mar-23 02:00:30.081459
#54 38.62    Compiling aws-smithy-json v0.62.5
2026-Mar-23 02:00:30.081459
#54 38.62    Compiling aws-smithy-query v0.60.15
2026-Mar-23 02:00:30.081459
#54 38.62    Compiling aws-smithy-json v0.61.9
2026-Mar-23 02:00:30.081459
#54 39.80    Compiling aws-smithy-http v0.63.6
2026-Mar-23 02:00:30.081459
#54 39.80    Compiling aws-credential-types v1.2.14
2026-Mar-23 02:00:30.081459
#54 39.80    Compiling aws-smithy-observability v0.2.6
2026-Mar-23 02:00:30.081459
#54 39.80    Compiling aws-smithy-http v0.62.6
2026-Mar-23 02:00:30.081459
#54 40.09    Compiling aws-sigv4 v1.4.2
2026-Mar-23 02:00:30.081459
#54 40.87    Compiling sqlx-macros-core v0.8.6
2026-Mar-23 02:00:30.081459
#54 ...
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#53 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-23 02:00:30.081459
#53 41.25    Compiling hyper v0.14.32
2026-Mar-23 02:00:30.081459
#53 41.30    Compiling sqlx-macros-core v0.8.6
2026-Mar-23 02:00:30.081459
#53 41.31    Compiling redis v0.27.6
2026-Mar-23 02:00:30.081459
#53 41.50    Compiling hyper v1.8.1
2026-Mar-23 02:00:30.081459
#53 41.87    Compiling aws-smithy-checksums v0.63.12
2026-Mar-23 02:00:30.081459
#53 42.46    Compiling sqlx-macros v0.8.6
2026-Mar-23 02:00:30.081459
#53 42.77    Compiling hyper-util v0.1.20
2026-Mar-23 02:00:30.081459
#53 43.94    Compiling hyper-tls v0.6.0
2026-Mar-23 02:00:30.081459
#53 43.94    Compiling axum v0.8.8
2026-Mar-23 02:00:30.081459
#53 44.37    Compiling hyper-rustls v0.24.2
2026-Mar-23 02:00:30.081459
#53 46.52    Compiling queue v0.1.0 (/app/crates/queue)
2026-Mar-23 02:00:30.081459
#53 47.46    Compiling serde_v8 v0.209.0
2026-Mar-23 02:00:30.081459
#53 47.76    Compiling deno_core v0.300.0
2026-Mar-23 02:00:30.081459
#53 ...
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#54 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-23 02:00:30.081459
#54 41.78    Compiling crc-fast v1.6.0
2026-Mar-23 02:00:30.081459
#54 41.79    Compiling hyper v0.14.32
2026-Mar-23 02:00:30.081459
#54 41.93    Compiling redis v0.27.6
2026-Mar-23 02:00:30.081459
#54 41.94    Compiling sqlx-macros v0.8.6
2026-Mar-23 02:00:30.081459
#54 42.00    Compiling hyper v1.8.1
2026-Mar-23 02:00:30.081459
#54 42.14    Compiling serde_v8 v0.209.0
2026-Mar-23 02:00:30.081459
#54 42.50    Compiling deno_core v0.300.0
2026-Mar-23 02:00:30.081459
#54 43.19    Compiling hyper-util v0.1.20
2026-Mar-23 02:00:30.081459
#54 43.84    Compiling aws-smithy-checksums v0.63.12
2026-Mar-23 02:00:30.081459
#54 44.32    Compiling hyper-tls v0.6.0
2026-Mar-23 02:00:30.081459
#54 44.32    Compiling axum v0.8.8
2026-Mar-23 02:00:30.081459
#54 44.78    Compiling hyper-rustls v0.24.2
2026-Mar-23 02:00:30.081459
#54 46.93    Compiling queue v0.1.0 (/app/crates/queue)
2026-Mar-23 02:00:30.081459
#54 ...
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#53 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-23 02:00:30.081459
#53 54.45    Compiling tokio-rustls v0.26.4
2026-Mar-23 02:00:30.081459
#53 54.60    Compiling hyper-rustls v0.27.7
2026-Mar-23 02:00:30.081459
#53 54.75    Compiling aws-smithy-http-client v1.1.12
2026-Mar-23 02:00:30.081459
#53 54.75    Compiling reqwest v0.12.28
2026-Mar-23 02:00:30.081459
#53 56.06    Compiling aws-smithy-runtime v1.10.3
2026-Mar-23 02:00:30.081459
#53 58.06    Compiling aws-runtime v1.7.2
2026-Mar-23 02:00:30.081459
#53 58.73    Compiling sqlx v0.8.6
2026-Mar-23 02:00:30.081459
#53 58.83    Compiling proxy v0.1.0 (/app/crates/proxy)
2026-Mar-23 02:00:30.081459
#53 58.83    Compiling job-system v0.1.0 (/app/crates/job-system)
2026-Mar-23 02:00:30.081459
#53 58.99    Compiling aws-sdk-ssooidc v1.98.0
2026-Mar-23 02:00:30.081459
#53 58.99    Compiling aws-sdk-sts v1.100.0
2026-Mar-23 02:00:30.081459
#53 58.99    Compiling aws-sdk-sso v1.96.0
2026-Mar-23 02:00:30.081459
#53 58.99    Compiling aws-sdk-s3 v1.119.0
2026-Mar-23 02:00:30.081459
#53 61.05    Compiling aws-config v1.8.15
2026-Mar-23 02:00:30.081459
#53 61.38    Compiling muxer v0.1.0 (/app/crates/muxer)
2026-Mar-23 02:00:30.081459
#53 ...
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#54 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-23 02:00:30.081459
#54 55.57    Compiling tokio-rustls v0.26.4
2026-Mar-23 02:00:30.081459
#54 55.75    Compiling hyper-rustls v0.27.7
2026-Mar-23 02:00:30.081459
#54 55.89    Compiling aws-smithy-http-client v1.1.12
2026-Mar-23 02:00:30.081459
#54 55.89    Compiling reqwest v0.12.28
2026-Mar-23 02:00:30.081459
#54 57.23    Compiling aws-smithy-runtime v1.10.3
2026-Mar-23 02:00:30.081459
#54 59.38    Compiling aws-runtime v1.7.2
2026-Mar-23 02:00:30.081459
#54 60.38    Compiling sqlx v0.8.6
2026-Mar-23 02:00:30.081459
#54 60.45    Compiling aws-sdk-sso v1.96.0
2026-Mar-23 02:00:30.081459
#54 60.45    Compiling aws-sdk-sts v1.100.0
2026-Mar-23 02:00:30.081459
#54 60.45    Compiling aws-sdk-ssooidc v1.98.0
2026-Mar-23 02:00:30.081459
#54 60.45    Compiling aws-sdk-s3 v1.119.0
2026-Mar-23 02:00:30.081459
#54 60.52    Compiling proxy v0.1.0 (/app/crates/proxy)
2026-Mar-23 02:00:30.081459
#54 60.52    Compiling job-system v0.1.0 (/app/crates/job-system)
2026-Mar-23 02:00:30.081459
#54 62.67    Compiling aws-config v1.8.15
2026-Mar-23 02:00:30.081459
#54 63.53    Compiling muxer v0.1.0 (/app/crates/muxer)
2026-Mar-23 02:00:30.081459
#54 ...
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#53 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-23 02:00:30.081459
#53 77.05    Compiling object-store v0.1.0 (/app/crates/object-store)
2026-Mar-23 02:00:30.081459
#53 ...
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#54 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-23 02:00:30.081459
#54 78.01    Compiling object-store v0.1.0 (/app/crates/object-store)
2026-Mar-23 02:00:30.081459
#54 ...
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#53 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-23 02:00:30.081459
#53 112.5    Compiling api v0.1.0 (/app/crates/api)
2026-Mar-23 02:00:30.081459
#53 ...
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#54 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-23 02:00:30.081459
#54 113.5    Compiling worker v0.1.0 (/app/crates/worker)
2026-Mar-23 02:00:30.081459
#54 216.9     Finished `release` profile [optimized] target(s) in 3m 36s
2026-Mar-23 02:00:30.081459
#54 DONE 217.1s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#53 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-23 02:00:30.081459
#53 ...
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#55 [worker runtime 5/8] RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Mar-23 02:00:30.081459
#55 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#56 [worker runtime 3/8] RUN apt-get update && apt-get install -y     ca-certificates     curl     libssl3     && rm -rf /var/lib/apt/lists/*
2026-Mar-23 02:00:30.081459
#56 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#57 [worker runtime 2/8] WORKDIR /app
2026-Mar-23 02:00:30.081459
#57 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#58 [worker runtime 4/8] RUN set -eux;     arch="$(dpkg --print-architecture)";     case "$arch" in       amd64) ytdlp_asset="yt-dlp_linux" ;;       arm64) ytdlp_asset="yt-dlp_linux_aarch64" ;;       *) echo "Unsupported architecture: $arch" >&2; exit 1 ;;     esac;     curl -fL "https://github.com/yt-dlp/yt-dlp/releases/latest/download/${ytdlp_asset}" -o /usr/local/bin/yt-dlp;     chmod +x /usr/local/bin/yt-dlp;     /usr/local/bin/yt-dlp --version
2026-Mar-23 02:00:30.081459
#58 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#59 [worker runtime 6/8] COPY --from=builder /app/target/release/mux-worker /usr/local/bin/
2026-Mar-23 02:00:30.081459
#59 CACHED
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#60 [worker runtime 7/8] COPY --from=builder /app/crates/api/app-migrations /app/app-migrations
2026-Mar-23 02:00:30.081459
#60 DONE 0.0s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#61 [worker runtime 8/8] RUN mkdir -p /app/extractors /app/proxy-state && chown -R appuser:appuser /app
2026-Mar-23 02:00:30.081459
#61 DONE 0.2s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#53 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-23 02:00:30.081459
#53 ...
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#62 [worker] exporting to image
2026-Mar-23 02:00:30.081459
#62 exporting layers 0.0s done
2026-Mar-23 02:00:30.081459
#62 exporting manifest sha256:c55dfb575cf31ae66de85a5eeea477ea52af789d7fbc182ea685971d305d2ee1 done
2026-Mar-23 02:00:30.081459
#62 exporting config sha256:8b3b681f73115e4fbbb8d3a6e7c6be206dd52e6e8ffa13a4cf09f78943882908 done
2026-Mar-23 02:00:30.081459
#62 exporting attestation manifest sha256:593f95b0ba5182702d79fd39bb536915e783c672d1c30d5007ba254c6addff08 done
2026-Mar-23 02:00:30.081459
#62 exporting manifest list sha256:b9d0341d360a34e34348afaa2b34d4b7f9cd79097439f54f4744daf887ec1aef done
2026-Mar-23 02:00:30.081459
#62 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_worker:3bf1238b2350ae4f3732e6368c475a81f97af531 done
2026-Mar-23 02:00:30.081459
#62 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_worker:3bf1238b2350ae4f3732e6368c475a81f97af531 0.0s done
2026-Mar-23 02:00:30.081459
#62 DONE 0.1s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#63 [worker] resolving provenance for metadata file
2026-Mar-23 02:00:30.081459
#63 DONE 0.0s
2026-Mar-23 02:00:30.081459
2026-Mar-23 02:00:30.081459
#53 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-23 02:00:30.099517
Error type: RuntimeException
2026-Mar-23 02:00:30.117020
Error code: 0
2026-Mar-23 02:00:30.136742
Location: /var/www/html/app/Traits/ExecuteRemoteCommand.php:243
2026-Mar-23 02:00:30.153720
Stack trace (first 5 lines):
2026-Mar-23 02:00:30.169103
#0 /var/www/html/app/Traits/ExecuteRemoteCommand.php(104): App\Jobs\ApplicationDeploymentJob->executeCommandWithProcess()
2026-Mar-23 02:00:30.189058
#1 /var/www/html/vendor/laravel/framework/src/Illuminate/Collections/Traits/EnumeratesValues.php(272): App\Jobs\ApplicationDeploymentJob->{closure:App\Traits\ExecuteRemoteCommand::execute_remote_command():71}()
2026-Mar-23 02:00:30.208021
#2 /var/www/html/app/Traits/ExecuteRemoteCommand.php(71): Illuminate\Support\Collection->each()
2026-Mar-23 02:00:30.227156
#3 /var/www/html/app/Jobs/ApplicationDeploymentJob.php(730): App\Jobs\ApplicationDeploymentJob->execute_remote_command()
2026-Mar-23 02:00:30.243535
#4 /var/www/html/app/Jobs/ApplicationDeploymentJob.php(467): App\Jobs\ApplicationDeploymentJob->deploy_docker_compose_buildpack()
2026-Mar-23 02:00:30.259588
========================================