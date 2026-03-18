2026-Mar-17 00:56:27.499146
Starting deployment of khoa280703/downloadtool:main-zoscg4oc04gkwkssg0kw8w8w to localhost.
2026-Mar-17 00:56:27.654370
Preparing container with helper image: ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Mar-17 00:56:27.746033
[CMD]: docker stop -t 30 sosk0sgskc8sockk0gs8owso
2026-Mar-17 00:56:27.746033
Error response from daemon: No such container: sosk0sgskc8sockk0gs8owso
2026-Mar-17 00:56:27.860774
[CMD]: docker run -d --network coolify --name sosk0sgskc8sockk0gs8owso  --rm -v /var/run/docker.sock:/var/run/docker.sock ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Mar-17 00:56:27.860774
f7c8cb7cd3525c758873aa62ad891fd7f8d376055be097742c2c786b78ad468b
2026-Mar-17 00:56:28.838278
[CMD]: docker exec sosk0sgskc8sockk0gs8owso bash -c 'GIT_SSH_COMMAND="ssh -o ConnectTimeout=30 -p 22 -o Port=22 -o LogLevel=ERROR -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git ls-remote https://github.com/Khoa280703/downloadtool refs/heads/main'
2026-Mar-17 00:56:28.838278
b412264047fb7da5cc4228d3f1ac389528038ad3	refs/heads/main
2026-Mar-17 00:56:28.848178
----------------------------------------
2026-Mar-17 00:56:28.852610
Importing Khoa280703/downloadtool:main (commit sha b412264047fb7da5cc4228d3f1ac389528038ad3) to /artifacts/sosk0sgskc8sockk0gs8owso.
2026-Mar-17 00:56:28.981776
[CMD]: docker exec sosk0sgskc8sockk0gs8owso bash -c 'git clone --depth=1 --recurse-submodules --shallow-submodules -b 'main' 'https://github.com/Khoa280703/downloadtool' '/artifacts/sosk0sgskc8sockk0gs8owso' && cd '/artifacts/sosk0sgskc8sockk0gs8owso' && if [ -f .gitmodules ]; then sed -i "s#git@\(.*\):#https://\1/#g" '/artifacts/sosk0sgskc8sockk0gs8owso'/.gitmodules || true && git submodule sync && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git submodule update --init --recursive --depth=1; fi && cd '/artifacts/sosk0sgskc8sockk0gs8owso' && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git lfs pull'
2026-Mar-17 00:56:28.981776
Cloning into '/artifacts/sosk0sgskc8sockk0gs8owso'...
2026-Mar-17 00:56:30.630929
[CMD]: docker exec sosk0sgskc8sockk0gs8owso bash -c 'cd /artifacts/sosk0sgskc8sockk0gs8owso && git log -1 b412264047fb7da5cc4228d3f1ac389528038ad3 --pretty=%B'
2026-Mar-17 00:56:30.630929
chore: move shared proxy into app compose
2026-Mar-17 00:56:34.621985
[CMD]: docker exec sosk0sgskc8sockk0gs8owso bash -c 'test -f /artifacts/sosk0sgskc8sockk0gs8owso/docker/Dockerfile.api && echo 'exists' || echo 'not found''
2026-Mar-17 00:56:34.621985
exists
2026-Mar-17 00:56:34.759066
[CMD]: docker exec sosk0sgskc8sockk0gs8owso bash -c 'cat /artifacts/sosk0sgskc8sockk0gs8owso/docker/Dockerfile.api'
2026-Mar-17 00:56:34.759066
# Dockerfile for API service deployment
2026-Mar-17 00:56:34.759066
# Builds the API server and related components without GPU support
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
# Stage 0: Build injector JS (embedded into api crate via include_str! at compile time)
2026-Mar-17 00:56:34.759066
FROM node:22-alpine AS js-builder
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
WORKDIR /app
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
RUN npm install -g pnpm
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
# Copy workspace manifests for pnpm resolution
2026-Mar-17 00:56:34.759066
COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Mar-17 00:56:34.759066
COPY packages/api-client/package.json ./packages/api-client/
2026-Mar-17 00:56:34.759066
COPY apps/injector/package.json ./apps/injector/
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
# Copy injector source and shared packages
2026-Mar-17 00:56:34.759066
COPY apps/injector/ ./apps/injector/
2026-Mar-17 00:56:34.759066
COPY packages/ ./packages/
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
# Install deps and build injector (produces dist/bm.js and dist/youtube-downloader.user.js)
2026-Mar-17 00:56:34.759066
RUN pnpm install --frozen-lockfile
2026-Mar-17 00:56:34.759066
RUN pnpm --filter @downloadtool/injector build
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
# Build extractor TypeScript to IIFE format (required by crates/extractor/build.rs)
2026-Mar-17 00:56:34.759066
COPY extractors/ ./extractors/
2026-Mar-17 00:56:34.759066
RUN mkdir -p extractors/dist && \
2026-Mar-17 00:56:34.759066
npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js && \
2026-Mar-17 00:56:34.759066
npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
# Stage 1: Rust builder
2026-Mar-17 00:56:34.759066
FROM rust:1.91-bookworm AS builder
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
WORKDIR /app
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
# Install dependencies
2026-Mar-17 00:56:34.759066
RUN apt-get update && apt-get install -y \
2026-Mar-17 00:56:34.759066
pkg-config \
2026-Mar-17 00:56:34.759066
libssl-dev \
2026-Mar-17 00:56:34.759066
&& rm -rf /var/lib/apt/lists/*
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
# Copy workspace configuration
2026-Mar-17 00:56:34.759066
COPY Cargo.toml ./
2026-Mar-17 00:56:34.759066
COPY Cargo.lock ./
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
# Copy all crates
2026-Mar-17 00:56:34.759066
COPY crates/ ./crates/
2026-Mar-17 00:56:34.759066
COPY config/ ./config/
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
# Copy injector dist (required by include_str! in crates/api/src/routes/static_files.rs)
2026-Mar-17 00:56:34.759066
COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
# Copy extractor source + pre-built IIFE dist (built by js-builder stage)
2026-Mar-17 00:56:34.759066
COPY extractors/ ./extractors/
2026-Mar-17 00:56:34.759066
COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
# Build the release binary
2026-Mar-17 00:56:34.759066
RUN cargo build --release --bin api-server
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
# Stage 2: Runtime
2026-Mar-17 00:56:34.759066
FROM debian:bookworm-slim AS runtime
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
WORKDIR /app
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
# Install runtime dependencies
2026-Mar-17 00:56:34.759066
RUN apt-get update && apt-get install -y \
2026-Mar-17 00:56:34.759066
ca-certificates \
2026-Mar-17 00:56:34.759066
curl \
2026-Mar-17 00:56:34.759066
libssl3 \
2026-Mar-17 00:56:34.759066
&& rm -rf /var/lib/apt/lists/*
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
# Install latest yt-dlp binary (newer than Debian package).
2026-Mar-17 00:56:34.759066
RUN set -eux; \
2026-Mar-17 00:56:34.759066
arch="$(dpkg --print-architecture)"; \
2026-Mar-17 00:56:34.759066
case "$arch" in \
2026-Mar-17 00:56:34.759066
amd64) ytdlp_asset="yt-dlp_linux" ;; \
2026-Mar-17 00:56:34.759066
arm64) ytdlp_asset="yt-dlp_linux_aarch64" ;; \
2026-Mar-17 00:56:34.759066
*) echo "Unsupported architecture: $arch" >&2; exit 1 ;; \
2026-Mar-17 00:56:34.759066
esac; \
2026-Mar-17 00:56:34.759066
curl -fL "https://github.com/yt-dlp/yt-dlp/releases/latest/download/${ytdlp_asset}" -o /usr/local/bin/yt-dlp; \
2026-Mar-17 00:56:34.759066
chmod +x /usr/local/bin/yt-dlp; \
2026-Mar-17 00:56:34.759066
/usr/local/bin/yt-dlp --version
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
# Create non-root user
2026-Mar-17 00:56:34.759066
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
# Copy binary from builder
2026-Mar-17 00:56:34.759066
COPY --from=builder /app/target/release/api-server /usr/local/bin/
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
# Create directories
2026-Mar-17 00:56:34.759066
RUN mkdir -p /app/extractors /app/data /app/proxy-state && chown -R appuser:appuser /app
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
# Switch to non-root user
2026-Mar-17 00:56:34.759066
USER appuser
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
# Environment variables
2026-Mar-17 00:56:34.759066
ENV PORT=3068
2026-Mar-17 00:56:34.759066
ENV EXTRACTOR_DIR=/app/extractors
2026-Mar-17 00:56:34.759066
ENV YTDLP_PATH=/usr/local/bin/yt-dlp
2026-Mar-17 00:56:34.759066
ENV RUST_LOG=info
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
# Expose port
2026-Mar-17 00:56:34.759066
EXPOSE 3068
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
# Health check
2026-Mar-17 00:56:34.759066
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Mar-17 00:56:34.759066
CMD curl -f http://localhost:3068/health || exit 1
2026-Mar-17 00:56:34.759066
2026-Mar-17 00:56:34.759066
# Run the server
2026-Mar-17 00:56:34.759066
CMD ["api-server"]
2026-Mar-17 00:56:34.907656
Added 105 ARG declarations to Dockerfile for service api (multi-stage build, added to 3 stages).
2026-Mar-17 00:56:35.046684
[CMD]: docker exec sosk0sgskc8sockk0gs8owso bash -c 'test -f /artifacts/sosk0sgskc8sockk0gs8owso/docker/Dockerfile.worker && echo 'exists' || echo 'not found''
2026-Mar-17 00:56:35.046684
exists
2026-Mar-17 00:56:35.181095
[CMD]: docker exec sosk0sgskc8sockk0gs8owso bash -c 'cat /artifacts/sosk0sgskc8sockk0gs8owso/docker/Dockerfile.worker'
2026-Mar-17 00:56:35.181095
# Dockerfile for mux worker deployment
2026-Mar-17 00:56:35.181095
2026-Mar-17 00:56:35.181095
# Stage 0: Build extractor TypeScript to IIFE format (required by crates/extractor/build.rs)
2026-Mar-17 00:56:35.181095
FROM node:22-alpine AS js-builder
2026-Mar-17 00:56:35.181095
2026-Mar-17 00:56:35.181095
WORKDIR /app
2026-Mar-17 00:56:35.181095
2026-Mar-17 00:56:35.181095
RUN npm install -g pnpm
2026-Mar-17 00:56:35.181095
2026-Mar-17 00:56:35.181095
COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Mar-17 00:56:35.181095
COPY packages/api-client/package.json ./packages/api-client/
2026-Mar-17 00:56:35.181095
COPY apps/injector/package.json ./apps/injector/
2026-Mar-17 00:56:35.181095
COPY packages/ ./packages/
2026-Mar-17 00:56:35.181095
COPY apps/injector/ ./apps/injector/
2026-Mar-17 00:56:35.181095
COPY extractors/ ./extractors/
2026-Mar-17 00:56:35.181095
2026-Mar-17 00:56:35.181095
RUN pnpm install --frozen-lockfile
2026-Mar-17 00:56:35.181095
RUN mkdir -p extractors/dist && \
2026-Mar-17 00:56:35.181095
npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js && \
2026-Mar-17 00:56:35.181095
npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-17 00:56:35.181095
2026-Mar-17 00:56:35.181095
# Stage 1: Rust builder
2026-Mar-17 00:56:35.181095
FROM rust:1.91-bookworm AS builder
2026-Mar-17 00:56:35.181095
2026-Mar-17 00:56:35.181095
WORKDIR /app
2026-Mar-17 00:56:35.181095
2026-Mar-17 00:56:35.181095
RUN apt-get update && apt-get install -y \
2026-Mar-17 00:56:35.181095
pkg-config \
2026-Mar-17 00:56:35.181095
libssl-dev \
2026-Mar-17 00:56:35.181095
&& rm -rf /var/lib/apt/lists/*
2026-Mar-17 00:56:35.181095
2026-Mar-17 00:56:35.181095
COPY Cargo.toml ./
2026-Mar-17 00:56:35.181095
COPY Cargo.lock ./
2026-Mar-17 00:56:35.181095
COPY crates/ ./crates/
2026-Mar-17 00:56:35.181095
COPY config/ ./config/
2026-Mar-17 00:56:35.181095
COPY extractors/ ./extractors/
2026-Mar-17 00:56:35.181095
COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-17 00:56:35.181095
2026-Mar-17 00:56:35.181095
RUN cargo build --release --bin mux-worker
2026-Mar-17 00:56:35.181095
2026-Mar-17 00:56:35.181095
# Stage 2: Runtime
2026-Mar-17 00:56:35.181095
FROM debian:bookworm-slim AS runtime
2026-Mar-17 00:56:35.181095
2026-Mar-17 00:56:35.181095
WORKDIR /app
2026-Mar-17 00:56:35.181095
2026-Mar-17 00:56:35.181095
RUN apt-get update && apt-get install -y \
2026-Mar-17 00:56:35.181095
ca-certificates \
2026-Mar-17 00:56:35.181095
curl \
2026-Mar-17 00:56:35.181095
libssl3 \
2026-Mar-17 00:56:35.181095
&& rm -rf /var/lib/apt/lists/*
2026-Mar-17 00:56:35.181095
2026-Mar-17 00:56:35.181095
RUN set -eux; \
2026-Mar-17 00:56:35.181095
arch="$(dpkg --print-architecture)"; \
2026-Mar-17 00:56:35.181095
case "$arch" in \
2026-Mar-17 00:56:35.181095
amd64) ytdlp_asset="yt-dlp_linux" ;; \
2026-Mar-17 00:56:35.181095
arm64) ytdlp_asset="yt-dlp_linux_aarch64" ;; \
2026-Mar-17 00:56:35.181095
*) echo "Unsupported architecture: $arch" >&2; exit 1 ;; \
2026-Mar-17 00:56:35.181095
esac; \
2026-Mar-17 00:56:35.181095
curl -fL "https://github.com/yt-dlp/yt-dlp/releases/latest/download/${ytdlp_asset}" -o /usr/local/bin/yt-dlp; \
2026-Mar-17 00:56:35.181095
chmod +x /usr/local/bin/yt-dlp; \
2026-Mar-17 00:56:35.181095
/usr/local/bin/yt-dlp --version
2026-Mar-17 00:56:35.181095
2026-Mar-17 00:56:35.181095
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Mar-17 00:56:35.181095
2026-Mar-17 00:56:35.181095
COPY --from=builder /app/target/release/mux-worker /usr/local/bin/
2026-Mar-17 00:56:35.181095
2026-Mar-17 00:56:35.181095
RUN mkdir -p /app/extractors /app/mux-artifacts /app/proxy-state && chown -R appuser:appuser /app
2026-Mar-17 00:56:35.181095
2026-Mar-17 00:56:35.181095
USER appuser
2026-Mar-17 00:56:35.181095
2026-Mar-17 00:56:35.181095
ENV EXTRACTOR_DIR=/app/extractors
2026-Mar-17 00:56:35.181095
ENV YTDLP_PATH=/usr/local/bin/yt-dlp
2026-Mar-17 00:56:35.181095
ENV RUST_LOG=info
2026-Mar-17 00:56:35.181095
ENV MUX_JOB_OUTPUT_DIR=/app/mux-artifacts
2026-Mar-17 00:56:35.181095
2026-Mar-17 00:56:35.181095
CMD ["mux-worker"]
2026-Mar-17 00:56:35.325334
Added 105 ARG declarations to Dockerfile for service worker (multi-stage build, added to 3 stages).
2026-Mar-17 00:56:35.458932
[CMD]: docker exec sosk0sgskc8sockk0gs8owso bash -c 'test -f /artifacts/sosk0sgskc8sockk0gs8owso/docker/Dockerfile.frontend && echo 'exists' || echo 'not found''
2026-Mar-17 00:56:35.458932
exists
2026-Mar-17 00:56:35.600285
[CMD]: docker exec sosk0sgskc8sockk0gs8owso bash -c 'cat /artifacts/sosk0sgskc8sockk0gs8owso/docker/Dockerfile.frontend'
2026-Mar-17 00:56:35.600285
# Dockerfile for frontend (SvelteKit Node server)
2026-Mar-17 00:56:35.600285
# Copy ALL source files BEFORE npm install so svelte-kit sync (prepare script)
2026-Mar-17 00:56:35.600285
# can find svelte.config.js and generate .svelte-kit/ correctly.
2026-Mar-17 00:56:35.600285
2026-Mar-17 00:56:35.600285
FROM node:22-alpine AS builder
2026-Mar-17 00:56:35.600285
2026-Mar-17 00:56:35.600285
WORKDIR /app
2026-Mar-17 00:56:35.600285
2026-Mar-17 00:56:35.600285
# Copy all frontend source files first (node_modules excluded via .dockerignore)
2026-Mar-17 00:56:35.600285
COPY frontend/ ./
2026-Mar-17 00:56:35.600285
COPY config/ /config/
2026-Mar-17 00:56:35.600285
2026-Mar-17 00:56:35.600285
# Install — prepare script runs svelte-kit sync with svelte.config.js available
2026-Mar-17 00:56:35.600285
RUN npm install
2026-Mar-17 00:56:35.600285
2026-Mar-17 00:56:35.600285
# Build-time public API URL (embedded into client bundle by Vite)
2026-Mar-17 00:56:35.600285
# Runtime env is too late for import.meta.env in browser bundle.
2026-Mar-17 00:56:35.600285
ARG VITE_API_URL
2026-Mar-17 00:56:35.600285
ENV VITE_API_URL=${VITE_API_URL}
2026-Mar-17 00:56:35.600285
RUN test -n "$VITE_API_URL" || (echo "VITE_API_URL build arg is required" && exit 1)
2026-Mar-17 00:56:35.600285
2026-Mar-17 00:56:35.600285
# Generate Paraglide runtime/messages from frontend/messages/* before Vite build
2026-Mar-17 00:56:35.600285
RUN npm run paraglide:compile
2026-Mar-17 00:56:35.600285
2026-Mar-17 00:56:35.600285
# Build via programmatic API — bypasses vite.config.ts loading (esbuild issues on Alpine)
2026-Mar-17 00:56:35.600285
RUN node build-docker.mjs
2026-Mar-17 00:56:35.600285
2026-Mar-17 00:56:35.600285
# Runtime
2026-Mar-17 00:56:35.600285
FROM node:22-alpine AS runtime
2026-Mar-17 00:56:35.600285
2026-Mar-17 00:56:35.600285
WORKDIR /app
2026-Mar-17 00:56:35.600285
2026-Mar-17 00:56:35.600285
COPY --from=builder /app/build ./build
2026-Mar-17 00:56:35.600285
COPY --from=builder /app/package.json ./
2026-Mar-17 00:56:35.600285
COPY --from=builder /app/package-lock.json ./
2026-Mar-17 00:56:35.600285
2026-Mar-17 00:56:35.600285
# Runtime needs server-side deps (better-auth, pg) used by hooks/routes
2026-Mar-17 00:56:35.600285
RUN npm ci --omit=dev
2026-Mar-17 00:56:35.600285
2026-Mar-17 00:56:35.600285
ENV PORT=5168
2026-Mar-17 00:56:35.600285
ENV HOST=0.0.0.0
2026-Mar-17 00:56:35.600285
2026-Mar-17 00:56:35.600285
EXPOSE 5168
2026-Mar-17 00:56:35.600285
2026-Mar-17 00:56:35.600285
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Mar-17 00:56:35.600285
CMD wget -qO- http://127.0.0.1:5168 || exit 1
2026-Mar-17 00:56:35.600285
2026-Mar-17 00:56:35.600285
CMD ["node", "build"]
2026-Mar-17 00:56:35.753838
Added 70 ARG declarations to Dockerfile for service frontend (multi-stage build, added to 2 stages).
2026-Mar-17 00:56:35.758493
Pulling & building required images.
2026-Mar-17 00:56:35.813057
Creating build-time .env file in /artifacts (outside Docker context).
2026-Mar-17 00:56:35.959292
Adding build arguments to Docker Compose build command.
2026-Mar-17 00:56:36.236363
[CMD]: docker exec sosk0sgskc8sockk0gs8owso bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/build-time.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/sosk0sgskc8sockk0gs8owso -f /artifacts/sosk0sgskc8sockk0gs8owso/docker/docker-compose.server.yml build --pull --build-arg COOLIFY_URL --build-arg COOLIFY_FQDN --build-arg SERVICE_URL_API --build-arg SERVICE_FQDN_FRONTEND --build-arg SERVICE_FQDN_API --build-arg ORIGIN --build-arg SERVICE_URL_FRONTEND --build-arg WHOP_WEBHOOK_SECRET --build-arg BETTER_AUTH_TRUSTED_ORIGINS --build-arg POSTGRES_PASSWORD --build-arg BETTER_AUTH_SECRET --build-arg SOCKS5_PROXY_URL --build-arg GOOGLE_CLIENT_ID --build-arg GOOGLE_CLIENT_SECRET --build-arg WHOP_PLAN_ID --build-arg VITE_API_URL --build-arg DOCKER_REDIS_URL --build-arg PROXY_LIST --build-arg MUX_QUEUE_STREAM --build-arg MUX_DIRECT_DOWNLOAD --build-arg MUX_ARTIFACT_BACKEND --build-arg S3_BUCKET_NAME --build-arg S3_ACCESS_KEY_ID --build-arg S3_SECRET_ACCESS_KEY --build-arg S3_REGION --build-arg S3_ENDPOINT --build-arg ADMIN_EMAILS --build-arg MUX_ARTIFACT_TTL_SECS --build-arg MUX_CLEANUP_INTERVAL_SECS --build-arg REDIS_URL --build-arg MUX_FILE_TICKET_TTL_SECS --build-arg PROXY_REDIS_URL --build-arg PROXY_DATABASE_URL --build-arg PROXY_QUARANTINE_TTL_SECS --build-arg SHARED_PROXY_POSTGRES_PASSWORD --build-arg COOLIFY_BUILD_SECRETS_HASH=cb761edbba0452729454e675a446fa41ec9d978a0f423c3bc074dd67ed8f6f1c'
2026-Mar-17 00:56:36.236363
#1 [internal] load local bake definitions
2026-Mar-17 00:56:36.450862
#1 reading from stdin 14.01kB done
2026-Mar-17 00:56:36.450862
#1 DONE 0.0s
2026-Mar-17 00:56:36.450862
2026-Mar-17 00:56:36.450862
#2 [worker internal] load build definition from Dockerfile.worker
2026-Mar-17 00:56:36.450862
#2 transferring dockerfile: 4.62kB done
2026-Mar-17 00:56:36.450862
#2 DONE 0.0s
2026-Mar-17 00:56:36.450862
2026-Mar-17 00:56:36.450862
#3 [frontend internal] load build definition from Dockerfile.frontend
2026-Mar-17 00:56:36.450862
#3 transferring dockerfile: 3.00kB done
2026-Mar-17 00:56:36.450862
#3 DONE 0.0s
2026-Mar-17 00:56:36.450862
2026-Mar-17 00:56:36.450862
#4 [api internal] load build definition from Dockerfile.api
2026-Mar-17 00:56:36.450862
#4 transferring dockerfile: 5.70kB done
2026-Mar-17 00:56:36.450862
#4 DONE 0.0s
2026-Mar-17 00:56:36.450862
2026-Mar-17 00:56:36.450862
#5 [api internal] load metadata for docker.io/library/node:22-alpine
2026-Mar-17 00:56:37.616092
#5 DONE 1.2s
2026-Mar-17 00:56:37.616092
2026-Mar-17 00:56:37.616092
#6 [worker internal] load metadata for docker.io/library/rust:1.91-bookworm
2026-Mar-17 00:56:37.616092
#6 DONE 1.2s
2026-Mar-17 00:56:37.616092
2026-Mar-17 00:56:37.616092
#7 [worker internal] load metadata for docker.io/library/debian:bookworm-slim
2026-Mar-17 00:56:37.616092
#7 DONE 1.2s
2026-Mar-17 00:56:37.616092
2026-Mar-17 00:56:37.616092
#8 [api internal] load .dockerignore
2026-Mar-17 00:56:37.616092
#8 transferring context: 341B done
2026-Mar-17 00:56:37.616092
#8 DONE 0.0s
2026-Mar-17 00:56:37.616092
2026-Mar-17 00:56:37.616092
#9 [api builder 1/8] FROM docker.io/library/node:22-alpine@sha256:8094c002d08262dba12645a3b4a15cd6cd627d30bc782f53229a2ec13ee22a00
2026-Mar-17 00:56:37.616092
#9 resolve docker.io/library/node:22-alpine@sha256:8094c002d08262dba12645a3b4a15cd6cd627d30bc782f53229a2ec13ee22a00 0.0s done
2026-Mar-17 00:56:37.616092
#9 DONE 0.0s
2026-Mar-17 00:56:37.616092
2026-Mar-17 00:56:37.616092
#10 [api builder  1/10] FROM docker.io/library/rust:1.91-bookworm@sha256:c1e5f19e773b7878c3f7a805dd00a495e747acbdc76fb2337a4ebf0418896b33
2026-Mar-17 00:56:37.616092
#10 resolve docker.io/library/rust:1.91-bookworm@sha256:c1e5f19e773b7878c3f7a805dd00a495e747acbdc76fb2337a4ebf0418896b33 0.0s done
2026-Mar-17 00:56:37.616092
#10 DONE 0.0s
2026-Mar-17 00:56:37.616092
2026-Mar-17 00:56:37.616092
#11 [worker runtime 1/7] FROM docker.io/library/debian:bookworm-slim@sha256:f06537653ac770703bc45b4b113475bd402f451e85223f0f2837acbf89ab020a
2026-Mar-17 00:56:37.616092
#11 resolve docker.io/library/debian:bookworm-slim@sha256:f06537653ac770703bc45b4b113475bd402f451e85223f0f2837acbf89ab020a 0.0s done
2026-Mar-17 00:56:37.616092
#11 DONE 0.0s
2026-Mar-17 00:56:37.616092
2026-Mar-17 00:56:37.616092
#12 [api runtime 2/7] WORKDIR /app
2026-Mar-17 00:56:37.616092
#12 CACHED
2026-Mar-17 00:56:37.616092
2026-Mar-17 00:56:37.616092
#13 [api builder 2/8] WORKDIR /app
2026-Mar-17 00:56:37.616092
#13 CACHED
2026-Mar-17 00:56:37.616092
2026-Mar-17 00:56:37.616092
#14 [api builder  2/10] WORKDIR /app
2026-Mar-17 00:56:37.616092
#14 CACHED
2026-Mar-17 00:56:37.616092
2026-Mar-17 00:56:37.616092
#15 [api internal] load build context
2026-Mar-17 00:56:37.616092
#15 transferring context: 937.04kB 0.0s done
2026-Mar-17 00:56:37.779511
#15 transferring context: 937.04kB 0.0s done
2026-Mar-17 00:56:37.779511
#15 DONE 0.0s
2026-Mar-17 00:56:37.779511
2026-Mar-17 00:56:37.779511
#16 [frontend internal] load build context
2026-Mar-17 00:56:37.779511
#16 transferring context: 2.04MB 0.0s done
2026-Mar-17 00:56:37.779511
#16 DONE 0.1s
2026-Mar-17 00:56:37.779511
2026-Mar-17 00:56:37.779511
#17 [frontend builder 3/8] COPY frontend/ ./
2026-Mar-17 00:56:37.779511
#17 CACHED
2026-Mar-17 00:56:37.779511
2026-Mar-17 00:56:37.779511
#18 [frontend builder 4/8] COPY config/ /config/
2026-Mar-17 00:56:37.779511
#18 CACHED
2026-Mar-17 00:56:37.779511
2026-Mar-17 00:56:37.779511
#19 [worker builder  3/10] RUN apt-get update && apt-get install -y     pkg-config     libssl-dev     && rm -rf /var/lib/apt/lists/*
2026-Mar-17 00:56:38.015895
#19 0.446 Get:1 http://deb.debian.org/debian bookworm InRelease [151 kB]
2026-Mar-17 00:56:38.281947
#19 0.642 Get:2 http://deb.debian.org/debian bookworm-updates InRelease [55.4 kB]
2026-Mar-17 00:56:38.281947
#19 0.712 Get:3 http://deb.debian.org/debian-security bookworm-security InRelease [48.0 kB]
2026-Mar-17 00:56:38.503373
#19 0.783 Get:4 http://deb.debian.org/debian bookworm/main amd64 Packages [8792 kB]
2026-Mar-17 00:56:38.930495
#19 1.361 Get:5 http://deb.debian.org/debian bookworm-updates/main amd64 Packages [6924 B]
2026-Mar-17 00:56:39.142437
#19 1.423 Get:6 http://deb.debian.org/debian-security bookworm-security/main amd64 Packages [294 kB]
2026-Mar-17 00:56:39.638775
#19 ...
2026-Mar-17 00:56:39.638775
2026-Mar-17 00:56:39.638775
#20 [api js-builder  3/11] RUN npm install -g pnpm
2026-Mar-17 00:56:39.638775
#20 1.850
2026-Mar-17 00:56:39.638775
#20 1.850 added 1 package in 2s
2026-Mar-17 00:56:39.638775
#20 1.850
2026-Mar-17 00:56:39.638775
#20 1.850 1 package is looking for funding
2026-Mar-17 00:56:39.638775
#20 1.850   run `npm fund` for details
2026-Mar-17 00:56:39.638775
#20 1.851 npm notice
2026-Mar-17 00:56:39.638775
#20 1.851 npm notice New major version of npm available! 10.9.4 -> 11.11.1
2026-Mar-17 00:56:39.638775
#20 1.851 npm notice Changelog: https://github.com/npm/cli/releases/tag/v11.11.1
2026-Mar-17 00:56:39.638775
#20 1.851 npm notice To update run: npm install -g npm@11.11.1
2026-Mar-17 00:56:39.638775
#20 1.851 npm notice
2026-Mar-17 00:56:39.638775
#20 DONE 2.0s
2026-Mar-17 00:56:39.638775
2026-Mar-17 00:56:39.638775
#21 [worker js-builder  4/11] COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Mar-17 00:56:39.638775
#21 DONE 0.1s
2026-Mar-17 00:56:39.638775
2026-Mar-17 00:56:39.638775
#22 [worker js-builder  5/11] COPY packages/api-client/package.json ./packages/api-client/
2026-Mar-17 00:56:39.638775
#22 DONE 0.0s
2026-Mar-17 00:56:39.789832
#23 [worker js-builder  6/11] COPY apps/injector/package.json ./apps/injector/
2026-Mar-17 00:56:39.789832
#23 DONE 0.0s
2026-Mar-17 00:56:39.789832
2026-Mar-17 00:56:39.789832
#24 [api js-builder  7/12] COPY apps/injector/ ./apps/injector/
2026-Mar-17 00:56:39.794717
#24 ...
2026-Mar-17 00:56:39.794717
2026-Mar-17 00:56:39.794717
#25 [worker js-builder  7/11] COPY packages/ ./packages/
2026-Mar-17 00:56:39.794717
#25 DONE 0.2s
2026-Mar-17 00:56:40.040034
#24 [api js-builder  7/12] COPY apps/injector/ ./apps/injector/
2026-Mar-17 00:56:40.040034
#24 DONE 0.2s
2026-Mar-17 00:56:40.040034
2026-Mar-17 00:56:40.040034
#26 [worker js-builder  8/11] COPY apps/injector/ ./apps/injector/
2026-Mar-17 00:56:40.040034
#26 DONE 0.1s
2026-Mar-17 00:56:40.040034
2026-Mar-17 00:56:40.040034
#27 [api js-builder  8/12] COPY packages/ ./packages/
2026-Mar-17 00:56:40.040034
#27 DONE 0.1s
2026-Mar-17 00:56:40.040034
2026-Mar-17 00:56:40.040034
#28 [worker js-builder  9/11] COPY extractors/ ./extractors/
2026-Mar-17 00:56:40.040034
#28 DONE 0.0s
2026-Mar-17 00:56:40.040034
2026-Mar-17 00:56:40.040034
#29 [worker js-builder 10/11] RUN pnpm install --frozen-lockfile
2026-Mar-17 00:56:40.625439
#29 0.648 Scope: all 3 workspace projects
2026-Mar-17 00:56:40.625439
#29 0.736 Lockfile is up to date, resolution step is skipped
2026-Mar-17 00:56:40.738118
#29 0.774 Progress: resolved 1, reused 0, downloaded 0, added 0
2026-Mar-17 00:56:40.947619
#29 0.850 Packages: +105
2026-Mar-17 00:56:40.947619
#29 0.850 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
2026-Mar-17 00:56:41.846116
#29 1.774 Progress: resolved 105, reused 0, downloaded 38, added 37
2026-Mar-17 00:56:42.488520
#29 2.549 Progress: resolved 105, reused 0, downloaded 105, added 105, done
2026-Mar-17 00:56:42.589709
#29 2.615 .../esbuild@0.25.12/node_modules/esbuild postinstall$ node install.js
2026-Mar-17 00:56:42.589709
#29 2.686 .../esbuild@0.25.12/node_modules/esbuild postinstall: Done
2026-Mar-17 00:56:42.690701
#29 2.746
2026-Mar-17 00:56:42.856788
#29 2.852 Done in 2.6s using pnpm v10.32.1
2026-Mar-17 00:56:42.856788
#29 DONE 3.0s
2026-Mar-17 00:56:42.856788
2026-Mar-17 00:56:42.856788
#30 [api runtime 3/7] RUN apt-get update && apt-get install -y     ca-certificates     curl     libssl3     && rm -rf /var/lib/apt/lists/*
2026-Mar-17 00:56:42.856788
#30 0.342 Get:1 http://deb.debian.org/debian bookworm InRelease [151 kB]
2026-Mar-17 00:56:42.856788
#30 0.506 Get:2 http://deb.debian.org/debian bookworm-updates InRelease [55.4 kB]
2026-Mar-17 00:56:42.856788
#30 0.565 Get:3 http://deb.debian.org/debian-security bookworm-security InRelease [48.0 kB]
2026-Mar-17 00:56:42.856788
#30 0.622 Get:4 http://deb.debian.org/debian bookworm/main amd64 Packages [8792 kB]
2026-Mar-17 00:56:42.856788
#30 1.093 Get:5 http://deb.debian.org/debian bookworm-updates/main amd64 Packages [6924 B]
2026-Mar-17 00:56:42.856788
#30 1.148 Get:6 http://deb.debian.org/debian-security bookworm-security/main amd64 Packages [294 kB]
2026-Mar-17 00:56:42.856788
#30 1.816 Fetched 9348 kB in 2s (5912 kB/s)
2026-Mar-17 00:56:42.856788
#30 1.816 Reading package lists...
2026-Mar-17 00:56:42.856788
#30 2.297 Reading package lists...
2026-Mar-17 00:56:42.856788
#30 2.793 Building dependency tree...
2026-Mar-17 00:56:42.856788
#30 2.923 Reading state information...
2026-Mar-17 00:56:42.856788
#30 3.072 The following additional packages will be installed:
2026-Mar-17 00:56:42.856788
#30 3.072   krb5-locales libbrotli1 libcurl4 libgssapi-krb5-2 libk5crypto3 libkeyutils1
2026-Mar-17 00:56:42.856788
#30 3.072   libkrb5-3 libkrb5support0 libldap-2.5-0 libldap-common libnghttp2-14 libpsl5
2026-Mar-17 00:56:42.856788
#30 3.072   librtmp1 libsasl2-2 libsasl2-modules libsasl2-modules-db libssh2-1 openssl
2026-Mar-17 00:56:42.856788
#30 3.073   publicsuffix
2026-Mar-17 00:56:42.856788
#30 3.074 Suggested packages:
2026-Mar-17 00:56:42.856788
#30 3.074   krb5-doc krb5-user libsasl2-modules-gssapi-mit
2026-Mar-17 00:56:42.856788
#30 3.074   | libsasl2-modules-gssapi-heimdal libsasl2-modules-ldap libsasl2-modules-otp
2026-Mar-17 00:56:42.856788
#30 3.074   libsasl2-modules-sql
2026-Mar-17 00:56:42.856788
#30 3.228 The following NEW packages will be installed:
2026-Mar-17 00:56:42.856788
#30 3.229   ca-certificates curl krb5-locales libbrotli1 libcurl4 libgssapi-krb5-2
2026-Mar-17 00:56:42.856788
#30 3.229   libk5crypto3 libkeyutils1 libkrb5-3 libkrb5support0 libldap-2.5-0
2026-Mar-17 00:56:42.856788
#30 3.229   libldap-common libnghttp2-14 libpsl5 librtmp1 libsasl2-2 libsasl2-modules
2026-Mar-17 00:56:42.856788
#30 3.230   libsasl2-modules-db libssh2-1 libssl3 openssl publicsuffix
2026-Mar-17 00:56:42.856788
#30 3.379 0 upgraded, 22 newly installed, 0 to remove and 0 not upgraded.
2026-Mar-17 00:56:42.856788
#30 3.379 Need to get 6111 kB of archives.
2026-Mar-17 00:56:42.856788
#30 3.379 After this operation, 15.7 MB of additional disk space will be used.
2026-Mar-17 00:56:42.856788
#30 3.379 Get:1 http://deb.debian.org/debian-security bookworm-security/main amd64 libssl3 amd64 3.0.18-1~deb12u2 [2030 kB]
2026-Mar-17 00:56:42.856788
#30 3.865 Get:2 http://deb.debian.org/debian-security bookworm-security/main amd64 openssl amd64 3.0.18-1~deb12u2 [1433 kB]
2026-Mar-17 00:56:42.856788
#30 3.924 Get:3 http://deb.debian.org/debian bookworm/main amd64 ca-certificates all 20230311+deb12u1 [155 kB]
2026-Mar-17 00:56:42.856788
#30 3.981 Get:4 http://deb.debian.org/debian bookworm/main amd64 krb5-locales all 1.20.1-2+deb12u4 [63.4 kB]
2026-Mar-17 00:56:42.856788
#30 4.077 Get:5 http://deb.debian.org/debian bookworm/main amd64 libbrotli1 amd64 1.0.9-2+b6 [275 kB]
2026-Mar-17 00:56:42.856788
#30 4.080 Get:6 http://deb.debian.org/debian bookworm/main amd64 libkrb5support0 amd64 1.20.1-2+deb12u4 [33.2 kB]
2026-Mar-17 00:56:42.856788
#30 4.080 Get:7 http://deb.debian.org/debian bookworm/main amd64 libk5crypto3 amd64 1.20.1-2+deb12u4 [79.8 kB]
2026-Mar-17 00:56:42.856788
#30 4.081 Get:8 http://deb.debian.org/debian bookworm/main amd64 libkeyutils1 amd64 1.6.3-2 [8808 B]
2026-Mar-17 00:56:42.856788
#30 4.081 Get:9 http://deb.debian.org/debian bookworm/main amd64 libkrb5-3 amd64 1.20.1-2+deb12u4 [334 kB]
2026-Mar-17 00:56:42.856788
#30 4.085 Get:10 http://deb.debian.org/debian bookworm/main amd64 libgssapi-krb5-2 amd64 1.20.1-2+deb12u4 [135 kB]
2026-Mar-17 00:56:42.856788
#30 4.086 Get:11 http://deb.debian.org/debian bookworm/main amd64 libsasl2-modules-db amd64 2.1.28+dfsg-10 [20.3 kB]
2026-Mar-17 00:56:42.856788
#30 4.086 Get:12 http://deb.debian.org/debian bookworm/main amd64 libsasl2-2 amd64 2.1.28+dfsg-10 [59.7 kB]
2026-Mar-17 00:56:42.856788
#30 4.087 Get:13 http://deb.debian.org/debian bookworm/main amd64 libldap-2.5-0 amd64 2.5.13+dfsg-5 [183 kB]
2026-Mar-17 00:56:42.856788
#30 4.089 Get:14 http://deb.debian.org/debian bookworm/main amd64 libnghttp2-14 amd64 1.52.0-1+deb12u2 [73.0 kB]
2026-Mar-17 00:56:42.856788
#30 4.141 Get:15 http://deb.debian.org/debian bookworm/main amd64 libpsl5 amd64 0.21.2-1 [58.7 kB]
2026-Mar-17 00:56:42.856788
#30 4.142 Get:16 http://deb.debian.org/debian bookworm/main amd64 librtmp1 amd64 2.4+20151223.gitfa8646d.1-2+b2 [60.8 kB]
2026-Mar-17 00:56:42.856788
#30 4.143 Get:17 http://deb.debian.org/debian bookworm/main amd64 libssh2-1 amd64 1.10.0-3+b1 [179 kB]
2026-Mar-17 00:56:42.856788
#30 4.145 Get:18 http://deb.debian.org/debian bookworm/main amd64 libcurl4 amd64 7.88.1-10+deb12u14 [392 kB]
2026-Mar-17 00:56:42.856788
#30 4.151 Get:19 http://deb.debian.org/debian bookworm/main amd64 curl amd64 7.88.1-10+deb12u14 [316 kB]
2026-Mar-17 00:56:42.856788
#30 4.156 Get:20 http://deb.debian.org/debian bookworm/main amd64 libldap-common all 2.5.13+dfsg-5 [29.3 kB]
2026-Mar-17 00:56:42.856788
#30 4.157 Get:21 http://deb.debian.org/debian bookworm/main amd64 libsasl2-modules amd64 2.1.28+dfsg-10 [66.6 kB]
2026-Mar-17 00:56:42.856788
#30 4.158 Get:22 http://deb.debian.org/debian bookworm/main amd64 publicsuffix all 20230209.2326-1 [126 kB]
2026-Mar-17 00:56:42.856788
#30 4.278 debconf: delaying package configuration, since apt-utils is not installed
2026-Mar-17 00:56:42.856788
#30 4.312 Fetched 6111 kB in 1s (6687 kB/s)
2026-Mar-17 00:56:42.856788
#30 4.333 Selecting previously unselected package libssl3:amd64.
2026-Mar-17 00:56:42.856788
#30 4.333 (Reading database ... 
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
2026-Mar-17 00:56:42.856788
#30 4.338 Preparing to unpack .../00-libssl3_3.0.18-1~deb12u2_amd64.deb ...
2026-Mar-17 00:56:42.856788
#30 4.342 Unpacking libssl3:amd64 (3.0.18-1~deb12u2) ...
2026-Mar-17 00:56:42.856788
#30 4.469 Selecting previously unselected package openssl.
2026-Mar-17 00:56:42.856788
#30 4.470 Preparing to unpack .../01-openssl_3.0.18-1~deb12u2_amd64.deb ...
2026-Mar-17 00:56:42.856788
#30 4.471 Unpacking openssl (3.0.18-1~deb12u2) ...
2026-Mar-17 00:56:42.856788
#30 4.559 Selecting previously unselected package ca-certificates.
2026-Mar-17 00:56:42.856788
#30 4.560 Preparing to unpack .../02-ca-certificates_20230311+deb12u1_all.deb ...
2026-Mar-17 00:56:42.856788
#30 4.561 Unpacking ca-certificates (20230311+deb12u1) ...
2026-Mar-17 00:56:42.856788
#30 4.602 Selecting previously unselected package krb5-locales.
2026-Mar-17 00:56:42.856788
#30 4.603 Preparing to unpack .../03-krb5-locales_1.20.1-2+deb12u4_all.deb ...
2026-Mar-17 00:56:42.856788
#30 4.604 Unpacking krb5-locales (1.20.1-2+deb12u4) ...
2026-Mar-17 00:56:42.856788
#30 4.625 Selecting previously unselected package libbrotli1:amd64.
2026-Mar-17 00:56:42.856788
#30 4.626 Preparing to unpack .../04-libbrotli1_1.0.9-2+b6_amd64.deb ...
2026-Mar-17 00:56:42.856788
#30 4.627 Unpacking libbrotli1:amd64 (1.0.9-2+b6) ...
2026-Mar-17 00:56:42.856788
#30 4.662 Selecting previously unselected package libkrb5support0:amd64.
2026-Mar-17 00:56:42.856788
#30 4.663 Preparing to unpack .../05-libkrb5support0_1.20.1-2+deb12u4_amd64.deb ...
2026-Mar-17 00:56:42.856788
#30 4.664 Unpacking libkrb5support0:amd64 (1.20.1-2+deb12u4) ...
2026-Mar-17 00:56:42.856788
#30 4.680 Selecting previously unselected package libk5crypto3:amd64.
2026-Mar-17 00:56:42.856788
#30 4.681 Preparing to unpack .../06-libk5crypto3_1.20.1-2+deb12u4_amd64.deb ...
2026-Mar-17 00:56:42.856788
#30 4.682 Unpacking libk5crypto3:amd64 (1.20.1-2+deb12u4) ...
2026-Mar-17 00:56:42.856788
#30 4.704 Selecting previously unselected package libkeyutils1:amd64.
2026-Mar-17 00:56:42.856788
#30 4.705 Preparing to unpack .../07-libkeyutils1_1.6.3-2_amd64.deb ...
2026-Mar-17 00:56:42.856788
#30 4.706 Unpacking libkeyutils1:amd64 (1.6.3-2) ...
2026-Mar-17 00:56:42.856788
#30 4.721 Selecting previously unselected package libkrb5-3:amd64.
2026-Mar-17 00:56:42.856788
#30 4.722 Preparing to unpack .../08-libkrb5-3_1.20.1-2+deb12u4_amd64.deb ...
2026-Mar-17 00:56:42.856788
#30 4.723 Unpacking libkrb5-3:amd64 (1.20.1-2+deb12u4) ...
2026-Mar-17 00:56:42.856788
#30 4.762 Selecting previously unselected package libgssapi-krb5-2:amd64.
2026-Mar-17 00:56:42.856788
#30 4.763 Preparing to unpack .../09-libgssapi-krb5-2_1.20.1-2+deb12u4_amd64.deb ...
2026-Mar-17 00:56:42.856788
#30 4.764 Unpacking libgssapi-krb5-2:amd64 (1.20.1-2+deb12u4) ...
2026-Mar-17 00:56:42.856788
#30 4.787 Selecting previously unselected package libsasl2-modules-db:amd64.
2026-Mar-17 00:56:42.856788
#30 4.788 Preparing to unpack .../10-libsasl2-modules-db_2.1.28+dfsg-10_amd64.deb ...
2026-Mar-17 00:56:42.860079
#30 4.789 Unpacking libsasl2-modules-db:amd64 (2.1.28+dfsg-10) ...
2026-Mar-17 00:56:42.860079
#30 4.804 Selecting previously unselected package libsasl2-2:amd64.
2026-Mar-17 00:56:42.860079
#30 4.805 Preparing to unpack .../11-libsasl2-2_2.1.28+dfsg-10_amd64.deb ...
2026-Mar-17 00:56:42.860079
#30 4.806 Unpacking libsasl2-2:amd64 (2.1.28+dfsg-10) ...
2026-Mar-17 00:56:42.860079
#30 4.825 Selecting previously unselected package libldap-2.5-0:amd64.
2026-Mar-17 00:56:42.860079
#30 4.826 Preparing to unpack .../12-libldap-2.5-0_2.5.13+dfsg-5_amd64.deb ...
2026-Mar-17 00:56:42.860079
#30 4.828 Unpacking libldap-2.5-0:amd64 (2.5.13+dfsg-5) ...
2026-Mar-17 00:56:42.860079
#30 4.855 Selecting previously unselected package libnghttp2-14:amd64.
2026-Mar-17 00:56:42.860079
#30 4.856 Preparing to unpack .../13-libnghttp2-14_1.52.0-1+deb12u2_amd64.deb ...
2026-Mar-17 00:56:42.860079
#30 4.857 Unpacking libnghttp2-14:amd64 (1.52.0-1+deb12u2) ...
2026-Mar-17 00:56:42.860079
#30 4.879 Selecting previously unselected package libpsl5:amd64.
2026-Mar-17 00:56:42.860079
#30 4.880 Preparing to unpack .../14-libpsl5_0.21.2-1_amd64.deb ...
2026-Mar-17 00:56:42.860079
#30 4.881 Unpacking libpsl5:amd64 (0.21.2-1) ...
2026-Mar-17 00:56:42.860079
#30 4.900 Selecting previously unselected package librtmp1:amd64.
2026-Mar-17 00:56:42.860079
#30 4.901 Preparing to unpack .../15-librtmp1_2.4+20151223.gitfa8646d.1-2+b2_amd64.deb ...
2026-Mar-17 00:56:42.860079
#30 4.902 Unpacking librtmp1:amd64 (2.4+20151223.gitfa8646d.1-2+b2) ...
2026-Mar-17 00:56:42.860079
#30 4.923 Selecting previously unselected package libssh2-1:amd64.
2026-Mar-17 00:56:42.860079
#30 4.924 Preparing to unpack .../16-libssh2-1_1.10.0-3+b1_amd64.deb ...
2026-Mar-17 00:56:42.860079
#30 4.926 Unpacking libssh2-1:amd64 (1.10.0-3+b1) ...
2026-Mar-17 00:56:42.860079
#30 4.954 Selecting previously unselected package libcurl4:amd64.
2026-Mar-17 00:56:42.860079
#30 4.955 Preparing to unpack .../17-libcurl4_7.88.1-10+deb12u14_amd64.deb ...
2026-Mar-17 00:56:42.860079
#30 4.956 Unpacking libcurl4:amd64 (7.88.1-10+deb12u14) ...
2026-Mar-17 00:56:42.860079
#30 4.990 Selecting previously unselected package curl.
2026-Mar-17 00:56:42.860079
#30 4.991 Preparing to unpack .../18-curl_7.88.1-10+deb12u14_amd64.deb ...
2026-Mar-17 00:56:42.860079
#30 4.992 Unpacking curl (7.88.1-10+deb12u14) ...
2026-Mar-17 00:56:42.860079
#30 5.021 Selecting previously unselected package libldap-common.
2026-Mar-17 00:56:42.860079
#30 5.023 Preparing to unpack .../19-libldap-common_2.5.13+dfsg-5_all.deb ...
2026-Mar-17 00:56:42.860079
#30 5.024 Unpacking libldap-common (2.5.13+dfsg-5) ...
2026-Mar-17 00:56:42.860079
#30 5.040 Selecting previously unselected package libsasl2-modules:amd64.
2026-Mar-17 00:56:42.860079
#30 5.041 Preparing to unpack .../20-libsasl2-modules_2.1.28+dfsg-10_amd64.deb ...
2026-Mar-17 00:56:42.860079
#30 5.046 Unpacking libsasl2-modules:amd64 (2.1.28+dfsg-10) ...
2026-Mar-17 00:56:42.860079
#30 5.068 Selecting previously unselected package publicsuffix.
2026-Mar-17 00:56:42.860079
#30 5.069 Preparing to unpack .../21-publicsuffix_20230209.2326-1_all.deb ...
2026-Mar-17 00:56:42.860079
#30 5.070 Unpacking publicsuffix (20230209.2326-1) ...
2026-Mar-17 00:56:42.860079
#30 5.099 Setting up libkeyutils1:amd64 (1.6.3-2) ...
2026-Mar-17 00:56:42.860079
#30 5.101 Setting up libpsl5:amd64 (0.21.2-1) ...
2026-Mar-17 00:56:42.860079
#30 5.104 Setting up libbrotli1:amd64 (1.0.9-2+b6) ...
2026-Mar-17 00:56:42.860079
#30 5.108 Setting up libssl3:amd64 (3.0.18-1~deb12u2) ...
2026-Mar-17 00:56:42.860079
#30 5.111 Setting up libnghttp2-14:amd64 (1.52.0-1+deb12u2) ...
2026-Mar-17 00:56:42.860079
#30 5.113 Setting up krb5-locales (1.20.1-2+deb12u4) ...
2026-Mar-17 00:56:42.860079
#30 5.116 Setting up libldap-common (2.5.13+dfsg-5) ...
2026-Mar-17 00:56:42.860079
#30 5.119 Setting up libkrb5support0:amd64 (1.20.1-2+deb12u4) ...
2026-Mar-17 00:56:42.860079
#30 5.123 Setting up libsasl2-modules-db:amd64 (2.1.28+dfsg-10) ...
2026-Mar-17 00:56:42.860079
#30 5.126 Setting up librtmp1:amd64 (2.4+20151223.gitfa8646d.1-2+b2) ...
2026-Mar-17 00:56:42.860079
#30 5.129 Setting up libk5crypto3:amd64 (1.20.1-2+deb12u4) ...
2026-Mar-17 00:56:42.860079
#30 5.131 Setting up libsasl2-2:amd64 (2.1.28+dfsg-10) ...
2026-Mar-17 00:56:42.860079
#30 5.134 Setting up libssh2-1:amd64 (1.10.0-3+b1) ...
2026-Mar-17 00:56:42.860079
#30 5.137 Setting up libkrb5-3:amd64 (1.20.1-2+deb12u4) ...
2026-Mar-17 00:56:42.860079
#30 5.140 Setting up openssl (3.0.18-1~deb12u2) ...
2026-Mar-17 00:56:42.860079
#30 5.145 Setting up publicsuffix (20230209.2326-1) ...
2026-Mar-17 00:56:42.860079
#30 5.148 Setting up libsasl2-modules:amd64 (2.1.28+dfsg-10) ...
2026-Mar-17 00:56:42.860079
#30 5.155 Setting up libldap-2.5-0:amd64 (2.5.13+dfsg-5) ...
2026-Mar-17 00:56:42.860079
#30 5.157 Setting up ca-certificates (20230311+deb12u1) ...
2026-Mar-17 00:56:42.860079
#30 5.217 debconf: unable to initialize frontend: Dialog
2026-Mar-17 00:56:42.860079
#30 5.217 debconf: (TERM is not set, so the dialog frontend is not usable.)
2026-Mar-17 00:56:42.860079
#30 5.217 debconf: falling back to frontend: Readline
2026-Mar-17 00:56:42.860079
#30 5.217 debconf: unable to initialize frontend: Readline
2026-Mar-17 00:56:42.860079
#30 5.217 debconf: (Can't locate Term/ReadLine.pm in @INC (you may need to install the Term::ReadLine module) (@INC contains: /etc/perl /usr/local/lib/x86_64-linux-gnu/perl/5.36.0 /usr/local/share/perl/5.36.0 /usr/lib/x86_64-linux-gnu/perl5/5.36 /usr/share/perl5 /usr/lib/x86_64-linux-gnu/perl-base /usr/lib/x86_64-linux-gnu/perl/5.36 /usr/share/perl/5.36 /usr/local/lib/site_perl) at /usr/share/perl5/Debconf/FrontEnd/Readline.pm line 7.)
2026-Mar-17 00:56:42.860079
#30 5.217 debconf: falling back to frontend: Teletype
2026-Mar-17 00:56:43.008893
#30 ...
2026-Mar-17 00:56:43.008893
2026-Mar-17 00:56:43.008893
#31 [api js-builder  9/12] RUN pnpm install --frozen-lockfile
2026-Mar-17 00:56:43.008893
#31 0.680 Scope: all 3 workspace projects
2026-Mar-17 00:56:43.008893
#31 0.760 Lockfile is up to date, resolution step is skipped
2026-Mar-17 00:56:43.008893
#31 0.795 Progress: resolved 1, reused 0, downloaded 0, added 0
2026-Mar-17 00:56:43.008893
#31 0.849 Packages: +105
2026-Mar-17 00:56:43.008893
#31 0.849 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
2026-Mar-17 00:56:43.008893
#31 1.796 Progress: resolved 105, reused 0, downloaded 48, added 48
2026-Mar-17 00:56:43.008893
#31 2.553 Progress: resolved 105, reused 0, downloaded 105, added 105, done
2026-Mar-17 00:56:43.008893
#31 2.620 .../esbuild@0.25.12/node_modules/esbuild postinstall$ node install.js
2026-Mar-17 00:56:43.008893
#31 2.687 .../esbuild@0.25.12/node_modules/esbuild postinstall: Done
2026-Mar-17 00:56:43.008893
#31 2.749
2026-Mar-17 00:56:43.008893
#31 2.858 Done in 2.6s using pnpm v10.32.1
2026-Mar-17 00:56:43.008893
#31 DONE 3.0s
2026-Mar-17 00:56:43.008893
2026-Mar-17 00:56:43.008893
#32 [api js-builder 10/12] RUN pnpm --filter @downloadtool/injector build
2026-Mar-17 00:56:43.552945
#32 0.694
2026-Mar-17 00:56:43.552945
#32 0.694 > @downloadtool/injector@0.0.1 build /app/apps/injector
2026-Mar-17 00:56:43.552945
#32 0.694 > vite build && vite build --config vite.userscript.config.ts
2026-Mar-17 00:56:43.552945
#32 0.694
2026-Mar-17 00:56:43.882259
#32 0.928 vite v6.4.1 building for production...
2026-Mar-17 00:56:43.882259
#32 0.974 transforming...
2026-Mar-17 00:56:43.882259
#32 1.024 ✓ 4 modules transformed.
2026-Mar-17 00:56:44.059727
#32 1.045 rendering chunks...
2026-Mar-17 00:56:44.059727
#32 1.048 computing gzip size...
2026-Mar-17 00:56:44.059727
#32 1.051 dist/bm.js  6.20 kB │ gzip: 2.34 kB
2026-Mar-17 00:56:44.059727
#32 1.051 ✓ built in 95ms
2026-Mar-17 00:56:44.186817
#32 1.328 vite v6.4.1 building for production...
2026-Mar-17 00:56:44.348127
#32 1.360 transforming...
2026-Mar-17 00:56:44.348127
#32 1.409 ✓ 4 modules transformed.
2026-Mar-17 00:56:44.348127
#32 1.425 rendering chunks...
2026-Mar-17 00:56:44.348127
#32 1.489 computing gzip size...
2026-Mar-17 00:56:44.471817
#32 1.492 dist/youtube-downloader.user.js  10.42 kB │ gzip: 3.20 kB
2026-Mar-17 00:56:44.471817
#32 1.492 ✓ built in 153ms
2026-Mar-17 00:56:44.471817
#32 DONE 1.6s
2026-Mar-17 00:56:44.471817
2026-Mar-17 00:56:44.471817
#30 [api runtime 3/7] RUN apt-get update && apt-get install -y     ca-certificates     curl     libssl3     && rm -rf /var/lib/apt/lists/*
2026-Mar-17 00:56:44.471817
#30 5.649 Updating certificates in /etc/ssl/certs...
2026-Mar-17 00:56:44.471817
#30 6.207 142 added, 0 removed; done.
2026-Mar-17 00:56:44.471817
#30 6.222 Setting up libgssapi-krb5-2:amd64 (1.20.1-2+deb12u4) ...
2026-Mar-17 00:56:44.471817
#30 6.227 Setting up libcurl4:amd64 (7.88.1-10+deb12u14) ...
2026-Mar-17 00:56:44.471817
#30 6.230 Setting up curl (7.88.1-10+deb12u14) ...
2026-Mar-17 00:56:44.471817
#30 6.234 Processing triggers for libc-bin (2.36-9+deb12u13) ...
2026-Mar-17 00:56:44.471817
#30 6.245 Processing triggers for ca-certificates (20230311+deb12u1) ...
2026-Mar-17 00:56:44.471817
#30 6.249 Updating certificates in /etc/ssl/certs...
2026-Mar-17 00:56:44.471817
#30 6.691 0 added, 0 removed; done.
2026-Mar-17 00:56:44.471817
#30 6.691 Running hooks in /etc/ca-certificates/update.d...
2026-Mar-17 00:56:44.471817
#30 6.692 done.
2026-Mar-17 00:56:44.471817
#30 DONE 6.9s
2026-Mar-17 00:56:44.471817
2026-Mar-17 00:56:44.471817
#33 [api js-builder 11/12] COPY extractors/ ./extractors/
2026-Mar-17 00:56:44.471817
#33 DONE 0.0s
2026-Mar-17 00:56:44.471817
2026-Mar-17 00:56:44.471817
#34 [worker runtime 4/7] RUN set -eux;     arch="$(dpkg --print-architecture)";     case "$arch" in       amd64) ytdlp_asset="yt-dlp_linux" ;;       arm64) ytdlp_asset="yt-dlp_linux_aarch64" ;;       *) echo "Unsupported architecture: $arch" >&2; exit 1 ;;     esac;     curl -fL "https://github.com/yt-dlp/yt-dlp/releases/latest/download/${ytdlp_asset}" -o /usr/local/bin/yt-dlp;     chmod +x /usr/local/bin/yt-dlp;     /usr/local/bin/yt-dlp --version
2026-Mar-17 00:56:44.673393
#34 0.234 + dpkg --print-architecture
2026-Mar-17 00:56:44.911262
#34 0.235 + arch=amd64
2026-Mar-17 00:56:44.911262
#34 0.235 + ytdlp_asset=yt-dlp_linux
2026-Mar-17 00:56:44.911262
#34 0.235 + curl -fL https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux -o /usr/local/bin/yt-dlp
2026-Mar-17 00:56:44.911262
#34 0.241   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2026-Mar-17 00:56:44.911262
#34 0.241                                  Dload  Upload   Total   Spent    Left  Speed
2026-Mar-17 00:56:44.911262
#34 0.241 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2026-Mar-17 00:56:45.174647
0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2026-Mar-17 00:56:45.478722
#34 0.885 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2026-Mar-17 00:56:46.095432
#34 1.505 
  2 34.4M    2 1022k    0     0   808k      0  0:00:43  0:00:01  0:00:42  808k
2026-Mar-17 00:56:46.305129
#34 ...
2026-Mar-17 00:56:46.305129
2026-Mar-17 00:56:46.305129
#19 [worker builder  3/10] RUN apt-get update && apt-get install -y     pkg-config     libssl-dev     && rm -rf /var/lib/apt/lists/*
2026-Mar-17 00:56:46.305129
#19 2.228 Fetched 9348 kB in 2s (5123 kB/s)
2026-Mar-17 00:56:46.305129
#19 2.228 Reading package lists...
2026-Mar-17 00:56:46.305129
#19 2.707 Reading package lists...
2026-Mar-17 00:56:46.305129
#19 3.185 Building dependency tree...
2026-Mar-17 00:56:46.305129
#19 3.379 Reading state information...
2026-Mar-17 00:56:46.305129
#19 3.584 pkg-config is already the newest version (1.8.1-1).
2026-Mar-17 00:56:46.305129
#19 3.584 pkg-config set to manually installed.
2026-Mar-17 00:56:46.305129
#19 3.584 The following additional packages will be installed:
2026-Mar-17 00:56:46.305129
#19 3.584   libssl3 openssl
2026-Mar-17 00:56:46.305129
#19 3.584 Suggested packages:
2026-Mar-17 00:56:46.305129
#19 3.584   libssl-doc
2026-Mar-17 00:56:46.305129
#19 3.614 The following packages will be upgraded:
2026-Mar-17 00:56:46.305129
#19 3.614   libssl-dev libssl3 openssl
2026-Mar-17 00:56:46.305129
#19 3.771 3 upgraded, 0 newly installed, 0 to remove and 53 not upgraded.
2026-Mar-17 00:56:46.305129
#19 3.771 Need to get 5906 kB of archives.
2026-Mar-17 00:56:46.305129
#19 3.771 After this operation, 15.4 kB of additional disk space will be used.
2026-Mar-17 00:56:46.305129
#19 3.771 Get:1 http://deb.debian.org/debian-security bookworm-security/main amd64 libssl-dev amd64 3.0.18-1~deb12u2 [2444 kB]
2026-Mar-17 00:56:46.305129
#19 6.321 Get:2 http://deb.debian.org/debian-security bookworm-security/main amd64 libssl3 amd64 3.0.18-1~deb12u2 [2030 kB]
2026-Mar-17 00:56:46.305129
#19 7.191 Get:3 http://deb.debian.org/debian-security bookworm-security/main amd64 openssl amd64 3.0.18-1~deb12u2 [1433 kB]
2026-Mar-17 00:56:46.305129
#19 7.686 debconf: delaying package configuration, since apt-utils is not installed
2026-Mar-17 00:56:46.305129
#19 7.719 Fetched 5906 kB in 4s (1496 kB/s)
2026-Mar-17 00:56:46.305129
#19 7.737 (Reading database ... 
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
2026-Mar-17 00:56:46.305129
#19 7.755 Preparing to unpack .../libssl-dev_3.0.18-1~deb12u2_amd64.deb ...
2026-Mar-17 00:56:46.305129
#19 7.759 Unpacking libssl-dev:amd64 (3.0.18-1~deb12u2) over (3.0.17-1~deb12u3) ...
2026-Mar-17 00:56:46.305129
#19 7.947 Preparing to unpack .../libssl3_3.0.18-1~deb12u2_amd64.deb ...
2026-Mar-17 00:56:46.305129
#19 7.951 Unpacking libssl3:amd64 (3.0.18-1~deb12u2) over (3.0.17-1~deb12u3) ...
2026-Mar-17 00:56:46.305129
#19 8.082 Preparing to unpack .../openssl_3.0.18-1~deb12u2_amd64.deb ...
2026-Mar-17 00:56:46.305129
#19 8.085 Unpacking openssl (3.0.18-1~deb12u2) over (3.0.17-1~deb12u3) ...
2026-Mar-17 00:56:46.305129
#19 8.321 Setting up libssl3:amd64 (3.0.18-1~deb12u2) ...
2026-Mar-17 00:56:46.305129
#19 8.324 Setting up libssl-dev:amd64 (3.0.18-1~deb12u2) ...
2026-Mar-17 00:56:46.305129
#19 8.327 Setting up openssl (3.0.18-1~deb12u2) ...
2026-Mar-17 00:56:46.305129
#19 8.333 Processing triggers for libc-bin (2.36-9+deb12u13) ...
2026-Mar-17 00:56:46.305129
#19 DONE 8.6s
2026-Mar-17 00:56:46.305129
2026-Mar-17 00:56:46.305129
#35 [api builder  4/10] COPY Cargo.toml ./
2026-Mar-17 00:56:46.305129
#35 DONE 0.0s
2026-Mar-17 00:56:46.410276
#36 [api builder  5/10] COPY Cargo.lock ./
2026-Mar-17 00:56:46.410276
#36 DONE 0.1s
2026-Mar-17 00:56:46.410276
2026-Mar-17 00:56:46.410276
#37 [api builder  6/10] COPY crates/ ./crates/
2026-Mar-17 00:56:46.410276
#37 DONE 0.1s
2026-Mar-17 00:56:46.410276
2026-Mar-17 00:56:46.410276
#38 [worker builder  7/10] COPY config/ ./config/
2026-Mar-17 00:56:46.410276
#38 DONE 0.0s
2026-Mar-17 00:56:46.410276
2026-Mar-17 00:56:46.410276
#39 [worker builder  8/10] COPY extractors/ ./extractors/
2026-Mar-17 00:56:46.410276
#39 DONE 0.0s
2026-Mar-17 00:56:46.410276
2026-Mar-17 00:56:46.410276
#40 [worker js-builder 11/11] RUN mkdir -p extractors/dist &&     npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js &&     npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-17 00:56:46.410276
#40 3.342
2026-Mar-17 00:56:46.410276
#40 3.342   extractors/dist/types.js  1.2kb
2026-Mar-17 00:56:46.410276
#40 3.342
2026-Mar-17 00:56:46.410276
#40 3.342 ⚡ Done in 3ms
2026-Mar-17 00:56:47.084378
#40 4.205
2026-Mar-17 00:56:47.084378
#40 4.205   extractors/dist/youtube.js  22.4kb
2026-Mar-17 00:56:47.084378
#40 4.205
2026-Mar-17 00:56:47.084378
#40 4.205 ⚡ Done in 3ms
2026-Mar-17 00:56:47.084378
#40 DONE 4.2s
2026-Mar-17 00:56:47.084378
2026-Mar-17 00:56:47.084378
#34 [worker runtime 4/7] RUN set -eux;     arch="$(dpkg --print-architecture)";     case "$arch" in       amd64) ytdlp_asset="yt-dlp_linux" ;;       arm64) ytdlp_asset="yt-dlp_linux_aarch64" ;;       *) echo "Unsupported architecture: $arch" >&2; exit 1 ;;     esac;     curl -fL "https://github.com/yt-dlp/yt-dlp/releases/latest/download/${ytdlp_asset}" -o /usr/local/bin/yt-dlp;     chmod +x /usr/local/bin/yt-dlp;     /usr/local/bin/yt-dlp --version
2026-Mar-17 00:56:47.084378
#34 1.505 
  2 34.4M    2 1022k    0     0   808k      0  0:00:43  0:00:01  0:00:42  808k
 99 34.4M   99 34.3M    0     0  14.9M      0  0:00:02  0:00:02 --:--:-- 32.1M
2026-Mar-17 00:56:47.255560
#34 ...
2026-Mar-17 00:56:47.255560
2026-Mar-17 00:56:47.255560
#41 [worker builder  9/10] COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-17 00:56:47.255560
#41 DONE 0.2s
2026-Mar-17 00:56:47.255560
2026-Mar-17 00:56:47.255560
#34 [worker runtime 4/7] RUN set -eux;     arch="$(dpkg --print-architecture)";     case "$arch" in       amd64) ytdlp_asset="yt-dlp_linux" ;;       arm64) ytdlp_asset="yt-dlp_linux_aarch64" ;;       *) echo "Unsupported architecture: $arch" >&2; exit 1 ;;     esac;     curl -fL "https://github.com/yt-dlp/yt-dlp/releases/latest/download/${ytdlp_asset}" -o /usr/local/bin/yt-dlp;     chmod +x /usr/local/bin/yt-dlp;     /usr/local/bin/yt-dlp --version
2026-Mar-17 00:56:47.255560
#34 1.505 
  2 34.4M    2 1022k    0     0   808k      0  0:00:43  0:00:01  0:00:42  808k
 99 34.4M   99 34.3M    0     0  14.9M      0  0:00:02  0:00:02 --:--:-- 32.1M
100 34.4M  100 34.4M    0     0  14.1M      0  0:00:02  0:00:02 --:--:-- 28.7M
2026-Mar-17 00:56:47.255560
#34 2.671 + chmod +x /usr/local/bin/yt-dlp
2026-Mar-17 00:56:47.255560
#34 2.671 + /usr/local/bin/yt-dlp --version
2026-Mar-17 00:56:47.699693
#34 ...
2026-Mar-17 00:56:47.699693
2026-Mar-17 00:56:47.699693
#42 [frontend builder 5/8] RUN npm install
2026-Mar-17 00:56:47.800911
#42 ...
2026-Mar-17 00:56:47.800911
2026-Mar-17 00:56:47.800911
#34 [worker runtime 4/7] RUN set -eux;     arch="$(dpkg --print-architecture)";     case "$arch" in       amd64) ytdlp_asset="yt-dlp_linux" ;;       arm64) ytdlp_asset="yt-dlp_linux_aarch64" ;;       *) echo "Unsupported architecture: $arch" >&2; exit 1 ;;     esac;     curl -fL "https://github.com/yt-dlp/yt-dlp/releases/latest/download/${ytdlp_asset}" -o /usr/local/bin/yt-dlp;     chmod +x /usr/local/bin/yt-dlp;     /usr/local/bin/yt-dlp --version
2026-Mar-17 00:56:47.800911
#34 3.362 2026.03.13
2026-Mar-17 00:56:48.044386
#34 DONE 3.5s
2026-Mar-17 00:56:48.044386
2026-Mar-17 00:56:48.044386
#43 [api js-builder 12/12] RUN mkdir -p extractors/dist &&     npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js &&     npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-17 00:56:48.044386
#43 2.699
2026-Mar-17 00:56:48.044386
#43 2.699   extractors/dist/types.js  1.2kb
2026-Mar-17 00:56:48.044386
#43 2.699
2026-Mar-17 00:56:48.044386
#43 2.699 ⚡ Done in 3ms
2026-Mar-17 00:56:48.044386
#43 3.471
2026-Mar-17 00:56:48.044386
#43 3.471   extractors/dist/youtube.js  22.4kb
2026-Mar-17 00:56:48.044386
#43 3.471
2026-Mar-17 00:56:48.044386
#43 3.471 ⚡ Done in 3ms
2026-Mar-17 00:56:48.044386
#43 DONE 3.5s
2026-Mar-17 00:56:48.044386
2026-Mar-17 00:56:48.044386
#44 [api builder  8/11] COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Mar-17 00:56:48.044386
#44 DONE 0.1s
2026-Mar-17 00:56:48.044386
2026-Mar-17 00:56:48.044386
#45 [api runtime 5/7] RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Mar-17 00:56:48.200704
#45 ...
2026-Mar-17 00:56:48.200704
2026-Mar-17 00:56:48.200704
#46 [api builder  9/11] COPY extractors/ ./extractors/
2026-Mar-17 00:56:48.200704
#46 DONE 0.0s
2026-Mar-17 00:56:48.200704
2026-Mar-17 00:56:48.200704
#47 [api builder 10/11] COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-17 00:56:48.200704
#47 DONE 0.0s
2026-Mar-17 00:56:48.200704
2026-Mar-17 00:56:48.200704
#45 [api runtime 5/7] RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Mar-17 00:56:48.347319
#45 DONE 0.3s
2026-Mar-17 00:56:48.347319
2026-Mar-17 00:56:48.347319
#48 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-17 00:56:48.347319
#48 0.249     Updating crates.io index
2026-Mar-17 00:56:51.114823
#48 ...
2026-Mar-17 00:56:51.114823
2026-Mar-17 00:56:51.114823
#42 [frontend builder 5/8] RUN npm install
2026-Mar-17 00:56:51.114823
#42 12.34
2026-Mar-17 00:56:51.114823
#42 12.34 > frontend@0.0.1 prepare
2026-Mar-17 00:56:51.114823
#42 12.34 > svelte-kit sync || echo ''
2026-Mar-17 00:56:51.114823
#42 12.34
2026-Mar-17 00:56:51.114823
#42 13.26
2026-Mar-17 00:56:51.114823
#42 13.26 added 237 packages, and audited 238 packages in 13s
2026-Mar-17 00:56:51.114823
#42 13.26
2026-Mar-17 00:56:51.114823
#42 13.26 43 packages are looking for funding
2026-Mar-17 00:56:51.114823
#42 13.26   run `npm fund` for details
2026-Mar-17 00:56:51.114823
#42 13.33
2026-Mar-17 00:56:51.114823
#42 13.33 7 vulnerabilities (2 low, 2 moderate, 3 high)
2026-Mar-17 00:56:51.114823
#42 13.33
2026-Mar-17 00:56:51.114823
#42 13.33 To address all issues, run:
2026-Mar-17 00:56:51.114823
#42 13.33   npm audit fix
2026-Mar-17 00:56:51.114823
#42 13.33
2026-Mar-17 00:56:51.114823
#42 13.33 Run `npm audit` for details.
2026-Mar-17 00:56:51.114823
#42 13.33 npm notice
2026-Mar-17 00:56:51.114823
#42 13.33 npm notice New major version of npm available! 10.9.4 -> 11.11.1
2026-Mar-17 00:56:51.114823
#42 13.33 npm notice Changelog: https://github.com/npm/cli/releases/tag/v11.11.1
2026-Mar-17 00:56:51.114823
#42 13.33 npm notice To update run: npm install -g npm@11.11.1
2026-Mar-17 00:56:51.114823
#42 13.33 npm notice
2026-Mar-17 00:56:51.114823
#42 DONE 13.5s
2026-Mar-17 00:56:51.266099
#49 [frontend builder 6/8] RUN test -n "https://api-download.khoadangbui.online" || (echo "VITE_API_URL build arg is required" && exit 1)
2026-Mar-17 00:56:51.398966
#49 DONE 0.3s
2026-Mar-17 00:56:51.398966
2026-Mar-17 00:56:51.398966
#50 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-17 00:56:51.398966
#50 0.291     Updating crates.io index
2026-Mar-17 00:56:53.197269
#50 5.939  Downloading crates ...
2026-Mar-17 00:56:53.447811
#50 6.190   Downloaded adler v1.0.2
2026-Mar-17 00:56:53.557122
#50 6.199   Downloaded adler2 v2.0.1
2026-Mar-17 00:56:53.557122
#50 6.245   Downloaded ahash v0.8.12
2026-Mar-17 00:56:53.557122
#50 6.255   Downloaded alloc-no-stdlib v2.0.4
2026-Mar-17 00:56:53.557122
#50 6.299   Downloaded alloc-stdlib v0.2.2
2026-Mar-17 00:56:53.677060
#50 6.334   Downloaded crc v3.4.0
2026-Mar-17 00:56:53.677060
#50 6.345   Downloaded cfg-if v1.0.4
2026-Mar-17 00:56:53.677060
#50 6.380   Downloaded crunchy v0.2.4
2026-Mar-17 00:56:53.677060
#50 6.383   Downloaded atomic-waker v1.1.2
2026-Mar-17 00:56:53.677060
#50 6.419   Downloaded base16ct v0.1.1
2026-Mar-17 00:56:53.777523
#50 6.423   Downloaded const-random-macro v0.1.16
2026-Mar-17 00:56:53.777523
#50 6.426   Downloaded crc-catalog v2.4.0
2026-Mar-17 00:56:53.777523
#50 6.429   Downloaded crypto-common v0.1.7
2026-Mar-17 00:56:53.777523
#50 6.432   Downloaded block-buffer v0.10.4
2026-Mar-17 00:56:53.777523
#50 6.435   Downloaded async-stream v0.3.6
2026-Mar-17 00:56:53.777523
#50 6.440   Downloaded const-random v0.1.18
2026-Mar-17 00:56:53.777523
#50 6.443   Downloaded compression-core v0.4.31
2026-Mar-17 00:56:53.777523
#50 6.446   Downloaded base64-simd v0.7.0
2026-Mar-17 00:56:53.777523
#50 6.449   Downloaded cfg_aliases v0.2.1
2026-Mar-17 00:56:53.777523
#50 6.452   Downloaded aws-smithy-query v0.60.15
2026-Mar-17 00:56:53.777523
#50 6.455   Downloaded async-stream-impl v0.3.6
2026-Mar-17 00:56:53.777523
#50 6.470   Downloaded crossbeam-queue v0.3.12
2026-Mar-17 00:56:53.777523
#50 6.476   Downloaded cooked-waker v5.0.0
2026-Mar-17 00:56:53.777523
#50 6.479   Downloaded anyhow v1.0.102
2026-Mar-17 00:56:53.777523
#50 6.488   Downloaded base64-simd v0.8.0
2026-Mar-17 00:56:53.777523
#50 6.519   Downloaded aws-smithy-observability v0.2.6
2026-Mar-17 00:56:53.879335
#50 6.526   Downloaded bit-set v0.5.3
2026-Mar-17 00:56:53.879335
#50 6.561   Downloaded atoi v2.0.0
2026-Mar-17 00:56:53.879335
#50 6.566   Downloaded cpufeatures v0.2.17
2026-Mar-17 00:56:53.879335
#50 6.569   Downloaded cmake v0.1.57
2026-Mar-17 00:56:53.879335
#50 6.575   Downloaded dunce v1.0.5
2026-Mar-17 00:56:53.879335
#50 6.577   Downloaded allocator-api2 v0.2.21
2026-Mar-17 00:56:53.879335
#50 6.584   Downloaded axum-core v0.5.6
2026-Mar-17 00:56:53.879335
#50 6.588   Downloaded aws-smithy-async v1.2.14
2026-Mar-17 00:56:53.879335
#50 6.592   Downloaded autocfg v1.5.0
2026-Mar-17 00:56:53.879335
#50 6.595   Downloaded data-encoding v2.10.0
2026-Mar-17 00:56:53.879335
#50 6.598   Downloaded dashmap v6.1.0
2026-Mar-17 00:56:53.879335
#50 6.602   Downloaded bit-vec v0.6.3
2026-Mar-17 00:56:53.879335
#50 6.604   Downloaded aws-smithy-checksums v0.63.12
2026-Mar-17 00:56:53.879335
#50 6.607   Downloaded aws-smithy-http v0.62.6
2026-Mar-17 00:56:53.879335
#50 6.612   Downloaded errno v0.3.14
2026-Mar-17 00:56:53.879335
#50 6.615   Downloaded displaydoc v0.2.5
2026-Mar-17 00:56:53.879335
#50 6.621   Downloaded no-std-compat v0.4.1
2026-Mar-17 00:56:53.980665
#50 6.625   Downloaded arc-swap v1.8.2
2026-Mar-17 00:56:53.980665
#50 6.632   Downloaded mime v0.3.17
2026-Mar-17 00:56:53.980665
#50 6.635   Downloaded aws-types v1.3.14
2026-Mar-17 00:56:53.980665
#50 6.639   Downloaded aws-smithy-xml v0.60.15
2026-Mar-17 00:56:53.980665
#50 6.642   Downloaded async-trait v0.1.89
2026-Mar-17 00:56:53.980665
#50 6.649   Downloaded md-5 v0.10.6
2026-Mar-17 00:56:53.980665
#50 6.652   Downloaded dotenvy v0.15.7
2026-Mar-17 00:56:53.980665
#50 6.658   Downloaded const-oid v0.9.6
2026-Mar-17 00:56:53.980665
#50 6.663   Downloaded nonzero_ext v0.3.0
2026-Mar-17 00:56:53.980665
#50 6.672   Downloaded config v0.14.1
2026-Mar-17 00:56:53.980665
#50 6.681   Downloaded native-tls v0.2.18
2026-Mar-17 00:56:53.980665
#50 6.684   Downloaded cookie_store v0.22.1
2026-Mar-17 00:56:53.980665
#50 6.689   Downloaded clang-sys v1.8.1
2026-Mar-17 00:56:53.980665
#50 6.693   Downloaded bitflags v2.11.0
2026-Mar-17 00:56:53.980665
#50 6.701   Downloaded deno_ops v0.176.0
2026-Mar-17 00:56:53.980665
#50 6.719   Downloaded num-conv v0.2.0
2026-Mar-17 00:56:53.980665
#50 6.722   Downloaded parking v2.2.1
2026-Mar-17 00:56:54.081945
#50 6.725   Downloaded outref v0.1.0
2026-Mar-17 00:56:54.081945
#50 6.728   Downloaded openssl-probe v0.2.1
2026-Mar-17 00:56:54.081945
#50 6.731   Downloaded openssl-macros v0.1.1
2026-Mar-17 00:56:54.081945
#50 6.733   Downloaded num_cpus v1.17.0
2026-Mar-17 00:56:54.081945
#50 6.739   Downloaded num-iter v0.1.45
2026-Mar-17 00:56:54.081945
#50 6.741   Downloaded num-integer v0.1.46
2026-Mar-17 00:56:54.081945
#50 6.745   Downloaded miniz_oxide v0.7.4
2026-Mar-17 00:56:54.081945
#50 6.749   Downloaded memchr v2.8.0
2026-Mar-17 00:56:54.081945
#50 6.758   Downloaded der v0.7.10
2026-Mar-17 00:56:54.081945
#50 6.770   Downloaded pathdiff v0.2.3
2026-Mar-17 00:56:54.081945
#50 6.773   Downloaded pem-rfc7468 v0.7.0
2026-Mar-17 00:56:54.081945
#50 6.779   Downloaded brotli-decompressor v5.0.0
2026-Mar-17 00:56:54.081945
#50 6.788   Downloaded bitvec v1.0.1
2026-Mar-17 00:56:54.081945
#50 6.810   Downloaded percent-encoding v2.3.2
2026-Mar-17 00:56:54.081945
#50 6.812   Downloaded bindgen v0.69.5
2026-Mar-17 00:56:54.081945
#50 6.821   Downloaded psl-types v2.0.11
2026-Mar-17 00:56:54.081945
#50 6.824   Downloaded quanta v0.12.6
2026-Mar-17 00:56:54.187185
#50 6.829   Downloaded rand_chacha v0.3.1
2026-Mar-17 00:56:54.187185
#50 6.831   Downloaded quote v1.0.44
2026-Mar-17 00:56:54.187185
#50 6.836   Downloaded quinn-udp v0.5.14
2026-Mar-17 00:56:54.187185
#50 6.839   Downloaded num-bigint-dig v0.8.6
2026-Mar-17 00:56:54.187185
#50 6.845   Downloaded radium v0.7.0
2026-Mar-17 00:56:54.187185
#50 6.847   Downloaded proc-macro2 v1.0.106
2026-Mar-17 00:56:54.187185
#50 6.850   Downloaded pin-project v1.1.10
2026-Mar-17 00:56:54.187185
#50 6.869   Downloaded num-bigint v0.4.6
2026-Mar-17 00:56:54.187185
#50 6.877   Downloaded rand_chacha v0.9.0
2026-Mar-17 00:56:54.187185
#50 6.879   Downloaded rand_core v0.9.5
2026-Mar-17 00:56:54.187185
#50 6.884   Downloaded rand_core v0.6.4
2026-Mar-17 00:56:54.187185
#50 6.886   Downloaded brotli v8.0.2
2026-Mar-17 00:56:54.187185
#50 6.902   Downloaded either v1.15.0
2026-Mar-17 00:56:54.187185
#50 6.904   Downloaded event-listener-strategy v0.5.4
2026-Mar-17 00:56:54.187185
#50 6.906   Downloaded equivalent v1.0.2
2026-Mar-17 00:56:54.187185
#50 6.908   Downloaded proc-macro-error v1.0.4
2026-Mar-17 00:56:54.187185
#50 6.913   Downloaded ppv-lite86 v0.2.21
2026-Mar-17 00:56:54.187185
#50 6.915   Downloaded pkg-config v0.3.32
2026-Mar-17 00:56:54.187185
#50 6.919   Downloaded pkcs8 v0.10.2
2026-Mar-17 00:56:54.187185
#50 6.924   Downloaded pkcs8 v0.9.0
2026-Mar-17 00:56:54.187185
#50 6.929   Downloaded p256 v0.11.1
2026-Mar-17 00:56:54.348056
#50 6.937   Downloaded sec1 v0.3.0
2026-Mar-17 00:56:54.348056
#50 6.940   Downloaded rustc_version v0.4.1
2026-Mar-17 00:56:54.348056
#50 6.942   Downloaded rustc-hash v2.1.1
2026-Mar-17 00:56:54.348056
#50 6.944   Downloaded ron v0.8.1
2026-Mar-17 00:56:54.348056
#50 6.952   Downloaded generic-array v0.14.7
2026-Mar-17 00:56:54.348056
#50 6.954   Downloaded foreign-types-shared v0.1.1
2026-Mar-17 00:56:54.348056
#50 6.955   Downloaded rustversion v1.0.22
2026-Mar-17 00:56:54.348056
#50 6.959   Downloaded aws-sdk-s3 v1.119.0
2026-Mar-17 00:56:54.348056
#50 7.090   Downloaded sct v0.7.1
2026-Mar-17 00:56:54.457277
#50 7.098   Downloaded semver-parser v0.7.0
2026-Mar-17 00:56:54.457277
#50 7.103   Downloaded socket2 v0.5.10
2026-Mar-17 00:56:54.457277
#50 7.108   Downloaded tagptr v0.2.0
2026-Mar-17 00:56:54.457277
#50 7.112   Downloaded thiserror-impl v2.0.18
2026-Mar-17 00:56:54.457277
#50 7.116   Downloaded tiny-keccak v2.0.2
2026-Mar-17 00:56:54.457277
#50 7.119   Downloaded time-macros v0.2.27
2026-Mar-17 00:56:54.457277
#50 7.122   Downloaded time-core v0.1.8
2026-Mar-17 00:56:54.457277
#50 7.123   Downloaded deno_core_icudata v0.0.73
2026-Mar-17 00:56:54.457277
#50 7.157   Downloaded thiserror-impl v1.0.69
2026-Mar-17 00:56:54.457277
#50 7.160   Downloaded thiserror v2.0.18
2026-Mar-17 00:56:54.457277
#50 7.171   Downloaded tokio-stream v0.1.18
2026-Mar-17 00:56:54.457277
#50 7.180   Downloaded rustls v0.23.37
2026-Mar-17 00:56:54.457277
#50 7.199   Downloaded regex-automata v0.4.14
2026-Mar-17 00:56:54.558819
#50 7.220   Downloaded ring v0.17.14
2026-Mar-17 00:56:54.558819
#50 7.280   Downloaded rustls-webpki v0.103.9
2026-Mar-17 00:56:54.558819
#50 7.286   Downloaded webpki-roots v0.26.11
2026-Mar-17 00:56:54.558819
#50 7.290   Downloaded whoami v1.6.1
2026-Mar-17 00:56:54.558819
#50 7.294   Downloaded zmij v1.0.21
2026-Mar-17 00:56:54.558819
#50 7.298   Downloaded zeroize v1.8.2
2026-Mar-17 00:56:54.558819
#50 7.301   Downloaded zerofrom-derive v0.1.6
2026-Mar-17 00:56:54.661175
#50 7.303   Downloaded tokio-util v0.7.18
2026-Mar-17 00:56:54.661175
#50 7.314   Downloaded yoke-derive v0.8.1
2026-Mar-17 00:56:54.661175
#50 7.316   Downloaded yoke v0.8.1
2026-Mar-17 00:56:54.661175
#50 7.318   Downloaded xmlparser v0.13.6
2026-Mar-17 00:56:54.661175
#50 7.322   Downloaded wyz v0.5.1
2026-Mar-17 00:56:54.661175
#50 7.324   Downloaded writeable v0.6.2
2026-Mar-17 00:56:54.661175
#50 7.327   Downloaded syn v2.0.117
2026-Mar-17 00:56:54.661175
#50 7.342   Downloaded syn v1.0.109
2026-Mar-17 00:56:54.661175
#50 7.355   Downloaded sqlx-postgres v0.8.6
2026-Mar-17 00:56:54.661175
#50 7.368   Downloaded zerovec-derive v0.11.2
2026-Mar-17 00:56:54.661175
#50 7.371   Downloaded zerofrom v0.1.6
2026-Mar-17 00:56:54.661175
#50 7.372   Downloaded sqlx-sqlite v0.8.6
2026-Mar-17 00:56:54.661175
#50 7.379   Downloaded sqlx-mysql v0.8.6
2026-Mar-17 00:56:54.661175
#50 7.388   Downloaded sqlx-core v0.8.6
2026-Mar-17 00:56:54.661175
#50 7.400   Downloaded which v6.0.3
2026-Mar-17 00:56:54.661175
#50 7.403   Downloaded which v4.4.2
2026-Mar-17 00:56:54.794132
#50 7.405   Downloaded web-time v1.1.0
2026-Mar-17 00:56:54.794132
#50 7.408   Downloaded want v0.3.1
2026-Mar-17 00:56:54.794132
#50 7.409   Downloaded vsimd v0.8.0
2026-Mar-17 00:56:54.794132
#50 7.412   Downloaded version_check v0.9.5
2026-Mar-17 00:56:54.794132
#50 7.414   Downloaded uuid v1.21.0
2026-Mar-17 00:56:54.794132
#50 7.419   Downloaded if_chain v1.0.3
2026-Mar-17 00:56:54.794132
#50 7.420   Downloaded utoipa v4.2.3
2026-Mar-17 00:56:54.794132
#50 7.424   Downloaded urlencoding v2.1.3
2026-Mar-17 00:56:54.794132
#50 7.426   Downloaded unicode-ident v1.0.24
2026-Mar-17 00:56:54.794132
#50 7.430   Downloaded unicode-id-start v1.4.0
2026-Mar-17 00:56:54.794132
#50 7.434   Downloaded unicode-bidi v0.3.18
2026-Mar-17 00:56:54.794132
#50 7.438   Downloaded ucd-trie v0.1.7
2026-Mar-17 00:56:54.794132
#50 7.440   Downloaded try-lock v0.2.5
2026-Mar-17 00:56:54.794132
#50 7.441   Downloaded tracing-core v0.1.36
2026-Mar-17 00:56:54.794132
#50 7.445   Downloaded sqlx v0.8.6
2026-Mar-17 00:56:54.794132
#50 7.468   Downloaded utf8_iter v1.0.4
2026-Mar-17 00:56:54.794132
#50 7.470   Downloaded tokio v1.49.0
2026-Mar-17 00:56:54.794132
#50 7.536   Downloaded toml_edit v0.22.27
2026-Mar-17 00:56:54.948773
#50 7.543   Downloaded url v2.5.8
2026-Mar-17 00:56:54.948773
#50 7.547   Downloaded unicode-segmentation v1.12.0
2026-Mar-17 00:56:54.948773
#50 7.551   Downloaded webpki-roots v1.0.6
2026-Mar-17 00:56:54.948773
#50 7.555   Downloaded tracing-subscriber v0.3.22
2026-Mar-17 00:56:54.948773
#50 7.568   Downloaded httpdate v1.0.3
2026-Mar-17 00:56:54.948773
#50 7.570   Downloaded unicode-normalization v0.1.25
2026-Mar-17 00:56:54.948773
#50 7.575   Downloaded http-body v0.4.6
2026-Mar-17 00:56:54.948773
#50 7.577   Downloaded typenum v1.19.0
2026-Mar-17 00:56:54.948773
#50 7.582   Downloaded winnow v0.7.14
2026-Mar-17 00:56:54.948773
#50 7.597   Downloaded heck v0.5.0
2026-Mar-17 00:56:54.948773
#50 7.599   Downloaded vcpkg v0.2.15
2026-Mar-17 00:56:54.948773
#50 7.690   Downloaded matchers v0.2.0
2026-Mar-17 00:56:55.072850
#50 7.692   Downloaded futures-sink v0.3.32
2026-Mar-17 00:56:55.072850
#50 7.694   Downloaded home v0.5.12
2026-Mar-17 00:56:55.072850
#50 7.696   Downloaded futures-core v0.3.32
2026-Mar-17 00:56:55.072850
#50 7.698   Downloaded foreign-types v0.3.2
2026-Mar-17 00:56:55.072850
#50 7.700   Downloaded tower-http v0.6.8
2026-Mar-17 00:56:55.072850
#50 7.712   Downloaded tower v0.5.3
2026-Mar-17 00:56:55.072850
#50 7.728   Downloaded utoipa-gen v4.3.1
2026-Mar-17 00:56:55.072850
#50 7.736   Downloaded tower v0.4.13
2026-Mar-17 00:56:55.072850
#50 7.752   Downloaded fastrand v2.3.0
2026-Mar-17 00:56:55.072850
#50 7.755   Downloaded zerotrie v0.2.3
2026-Mar-17 00:56:55.072850
#50 7.760   Downloaded lazy_static v1.5.0
2026-Mar-17 00:56:55.072850
#50 7.763   Downloaded zerovec v0.11.5
2026-Mar-17 00:56:55.072850
#50 7.771   Downloaded yaml-rust2 v0.8.1
2026-Mar-17 00:56:55.072850
#50 7.814   Downloaded fslock v0.2.1
2026-Mar-17 00:56:55.174063
#50 7.817   Downloaded fnv v1.0.7
2026-Mar-17 00:56:55.174063
#50 7.818   Downloaded untrusted v0.9.0
2026-Mar-17 00:56:55.174063
#50 7.820   Downloaded unicode-properties v0.1.4
2026-Mar-17 00:56:55.174063
#50 7.823   Downloaded tracing-log v0.2.0
2026-Mar-17 00:56:55.174063
#50 7.828   Downloaded zerocopy v0.8.39
2026-Mar-17 00:56:55.174063
#50 7.858   Downloaded itoa v1.0.17
2026-Mar-17 00:56:55.174063
#50 7.860   Downloaded tower-layer v0.3.3
2026-Mar-17 00:56:55.174063
#50 7.861   Downloaded toml_write v0.1.2
2026-Mar-17 00:56:55.174063
#50 7.863   Downloaded serde_json v1.0.149
2026-Mar-17 00:56:55.174063
#50 7.873   Downloaded serde v1.0.228
2026-Mar-17 00:56:55.174063
#50 7.877   Downloaded rustls-webpki v0.101.7
2026-Mar-17 00:56:55.174063
#50 7.897   Downloaded lazycell v1.3.0
2026-Mar-17 00:56:55.174063
#50 7.899   Downloaded futures-task v0.3.32
2026-Mar-17 00:56:55.174063
#50 7.900   Downloaded rustls v0.21.12
2026-Mar-17 00:56:55.174063
#50 7.914   Downloaded hex v0.4.3
2026-Mar-17 00:56:55.174063
#50 7.916   Downloaded rustix v0.38.44
2026-Mar-17 00:56:55.384304
#50 7.948   Downloaded json5 v0.4.1
2026-Mar-17 00:56:55.384304
#50 7.951   Downloaded regex-syntax v0.8.10
2026-Mar-17 00:56:55.384304
#50 7.958   Downloaded regex v1.12.3
2026-Mar-17 00:56:55.384304
#50 7.964   Downloaded raw-cpuid v11.6.0
2026-Mar-17 00:56:55.384304
#50 7.968   Downloaded rsa v0.9.10
2026-Mar-17 00:56:55.384304
#50 7.974   Downloaded gzip-header v1.0.0
2026-Mar-17 00:56:55.384304
#50 7.976   Downloaded aws-lc-sys v0.38.0
2026-Mar-17 00:56:55.511956
#50 8.254   Downloaded http-body v1.0.1
2026-Mar-17 00:56:55.620199
#50 8.255   Downloaded heck v0.4.1
2026-Mar-17 00:56:55.620199
#50 8.257   Downloaded futures-macro v0.3.32
2026-Mar-17 00:56:55.620199
#50 8.258   Downloaded futures-io v0.3.32
2026-Mar-17 00:56:55.620199
#50 8.259   Downloaded funty v2.0.0
2026-Mar-17 00:56:55.620199
#50 8.260   Downloaded toml_datetime v0.6.11
2026-Mar-17 00:56:55.620199
#50 8.262   Downloaded form_urlencoded v1.2.2
2026-Mar-17 00:56:55.620199
#50 8.263   Downloaded reqwest v0.12.28
2026-Mar-17 00:56:55.620199
#50 8.268   Downloaded redis v0.27.6
2026-Mar-17 00:56:55.620199
#50 8.277   Downloaded tokio-rustls v0.26.4
2026-Mar-17 00:56:55.620199
#50 8.280   Downloaded tokio-rustls v0.24.1
2026-Mar-17 00:56:55.620199
#50 8.282   Downloaded tokio-native-tls v0.3.1
2026-Mar-17 00:56:55.620199
#50 8.285   Downloaded tokio-macros v2.6.0
2026-Mar-17 00:56:55.620199
#50 8.286   Downloaded tinyvec_macros v0.1.1
2026-Mar-17 00:56:55.620199
#50 8.287   Downloaded tinyvec v1.10.0
2026-Mar-17 00:56:55.620199
#50 8.291   Downloaded regex-lite v0.1.9
2026-Mar-17 00:56:55.620199
#50 8.293   Downloaded thiserror v1.0.69
2026-Mar-17 00:56:55.620199
#50 8.301   Downloaded tap v1.0.1
2026-Mar-17 00:56:55.620199
#50 8.302   Downloaded synstructure v0.13.2
2026-Mar-17 00:56:55.620199
#50 8.303   Downloaded sqlx-macros-core v0.8.6
2026-Mar-17 00:56:55.620199
#50 8.305   Downloaded spinning_top v0.3.0
2026-Mar-17 00:56:55.620199
#50 8.307   Downloaded socket2 v0.6.2
2026-Mar-17 00:56:55.620199
#50 8.309   Downloaded quinn-proto v0.11.13
2026-Mar-17 00:56:55.620199
#50 8.317   Downloaded sync_wrapper v1.0.2
2026-Mar-17 00:56:55.620199
#50 8.319   Downloaded subtle v2.6.1
2026-Mar-17 00:56:55.620199
#50 8.320   Downloaded strum_macros v0.25.3
2026-Mar-17 00:56:55.620199
#50 8.323   Downloaded strum v0.25.0
2026-Mar-17 00:56:55.620199
#50 8.324   Downloaded stringprep v0.1.5
2026-Mar-17 00:56:55.620199
#50 8.326   Downloaded static_assertions v1.1.0
2026-Mar-17 00:56:55.620199
#50 8.328   Downloaded stable_deref_trait v1.2.1
2026-Mar-17 00:56:55.620199
#50 8.330   Downloaded sqlx-macros v0.8.6
2026-Mar-17 00:56:55.620199
#50 8.331   Downloaded spki v0.7.3
2026-Mar-17 00:56:55.620199
#50 8.334   Downloaded spki v0.6.0
2026-Mar-17 00:56:55.620199
#50 8.337   Downloaded spin v0.9.8
2026-Mar-17 00:56:55.620199
#50 8.340   Downloaded sourcemap v8.0.1
2026-Mar-17 00:56:55.620199
#50 8.343   Downloaded rand v0.9.2
2026-Mar-17 00:56:55.620199
#50 8.349   Downloaded rand v0.8.5
2026-Mar-17 00:56:55.620199
#50 8.354   Downloaded portable-atomic v1.13.1
2026-Mar-17 00:56:55.620199
#50 8.362   Downloaded smallvec v1.15.1
2026-Mar-17 00:56:55.723134
#50 8.365   Downloaded slab v0.4.12
2026-Mar-17 00:56:55.723134
#50 8.367   Downloaded simple_asn1 v0.6.4
2026-Mar-17 00:56:55.723134
#50 8.369   Downloaded simd-adler32 v0.3.8
2026-Mar-17 00:56:55.723134
#50 8.371   Downloaded simd-abstraction v0.7.1
2026-Mar-17 00:56:55.723134
#50 8.373   Downloaded signature v2.2.0
2026-Mar-17 00:56:55.723134
#50 8.375   Downloaded signature v1.6.4
2026-Mar-17 00:56:55.723134
#50 8.377   Downloaded signal-hook-registry v1.4.8
2026-Mar-17 00:56:55.723134
#50 8.379   Downloaded shlex v1.3.0
2026-Mar-17 00:56:55.723134
#50 8.380   Downloaded sharded-slab v0.1.7
2026-Mar-17 00:56:55.723134
#50 8.384   Downloaded sha2 v0.10.9
2026-Mar-17 00:56:55.723134
#50 8.387   Downloaded sha1_smol v1.0.1
2026-Mar-17 00:56:55.723134
#50 8.389   Downloaded sha1 v0.10.6
2026-Mar-17 00:56:55.723134
#50 8.391   Downloaded serde_v8 v0.209.0
2026-Mar-17 00:56:55.723134
#50 8.394   Downloaded serde_urlencoded v0.7.1
2026-Mar-17 00:56:55.723134
#50 8.396   Downloaded tower-service v0.3.3
2026-Mar-17 00:56:55.723134
#50 8.397   Downloaded serde_spanned v0.6.9
2026-Mar-17 00:56:55.723134
#50 8.398   Downloaded serde_path_to_error v0.1.20
2026-Mar-17 00:56:55.723134
#50 8.400   Downloaded serde_derive v1.0.228
2026-Mar-17 00:56:55.723134
#50 8.404   Downloaded group v0.12.1
2026-Mar-17 00:56:55.723134
#50 8.406   Downloaded deno_unsync v0.4.4
2026-Mar-17 00:56:55.723134
#50 8.409   Downloaded serde_core v1.0.228
2026-Mar-17 00:56:55.723134
#50 8.412   Downloaded quinn v0.11.9
2026-Mar-17 00:56:55.723134
#50 8.415   Downloaded publicsuffix v2.3.0
2026-Mar-17 00:56:55.723134
#50 8.418   Downloaded prettyplease v0.2.37
2026-Mar-17 00:56:55.723134
#50 8.422   Downloaded pest_meta v2.8.6
2026-Mar-17 00:56:55.723134
#50 8.425   Downloaded pest v2.8.6
2026-Mar-17 00:56:55.723134
#50 8.430   Downloaded semver v1.0.27
2026-Mar-17 00:56:55.723134
#50 8.433   Downloaded semver v0.9.0
2026-Mar-17 00:56:55.723134
#50 8.434   Downloaded ryu v1.0.23
2026-Mar-17 00:56:55.723134
#50 8.438   Downloaded rustls-pki-types v1.14.0
2026-Mar-17 00:56:55.723134
#50 8.442   Downloaded rustls-native-certs v0.8.3
2026-Mar-17 00:56:55.723134
#50 8.445   Downloaded ipnet v2.11.0
2026-Mar-17 00:56:55.723134
#50 8.447   Downloaded scopeguard v1.2.0
2026-Mar-17 00:56:55.723134
#50 8.448   Downloaded rustc_version v0.2.3
2026-Mar-17 00:56:55.723134
#50 8.449   Downloaded rustc-hash v1.1.0
2026-Mar-17 00:56:55.723134
#50 8.451   Downloaded rust-ini v0.20.0
2026-Mar-17 00:56:55.723134
#50 8.452   Downloaded rfc6979 v0.3.1
2026-Mar-17 00:56:55.723134
#50 8.453   Downloaded lru-slab v0.1.2
2026-Mar-17 00:56:55.723134
#50 8.455   Downloaded idna_adapter v1.2.1
2026-Mar-17 00:56:55.723134
#50 8.456   Downloaded ff v0.12.1
2026-Mar-17 00:56:55.723134
#50 8.458   Downloaded openssl-sys v0.9.111
2026-Mar-17 00:56:55.723134
#50 8.465   Downloaded openssl v0.10.75
2026-Mar-17 00:56:55.831047
#50 8.476   Downloaded pkcs1 v0.7.5
2026-Mar-17 00:56:55.831047
#50 8.479   Downloaded pin-project-lite v0.2.16
2026-Mar-17 00:56:55.831047
#50 8.487   Downloaded pin-project-internal v1.1.10
2026-Mar-17 00:56:55.831047
#50 8.488   Downloaded pest_derive v2.8.6
2026-Mar-17 00:56:55.831047
#50 8.491   Downloaded nom v7.1.3
2026-Mar-17 00:56:55.831047
#50 8.497   Downloaded futures-executor v0.3.32
2026-Mar-17 00:56:55.831047
#50 8.499   Downloaded foldhash v0.1.5
2026-Mar-17 00:56:55.831047
#50 8.501   Downloaded deno_core v0.300.0
2026-Mar-17 00:56:55.831047
#50 8.512   Downloaded proc-macro-rules-macros v0.4.0
2026-Mar-17 00:56:55.831047
#50 8.513   Downloaded proc-macro-rules v0.4.0
2026-Mar-17 00:56:55.831047
#50 8.514   Downloaded proc-macro-error-attr v1.0.4
2026-Mar-17 00:56:55.831047
#50 8.515   Downloaded powerfmt v0.2.0
2026-Mar-17 00:56:55.831047
#50 8.517   Downloaded potential_utf v0.1.4
2026-Mar-17 00:56:55.831047
#50 8.518   Downloaded pin-utils v0.1.0
2026-Mar-17 00:56:55.831047
#50 8.520   Downloaded moka v0.12.13
2026-Mar-17 00:56:55.831047
#50 8.528   Downloaded aws-lc-rs v1.16.1
2026-Mar-17 00:56:55.831047
#50 8.541   Downloaded lru v0.12.5
2026-Mar-17 00:56:55.831047
#50 8.542   Downloaded futures-timer v3.0.3
2026-Mar-17 00:56:55.831047
#50 8.545   Downloaded axum v0.8.8
2026-Mar-17 00:56:55.831047
#50 8.554   Downloaded libloading v0.8.9
2026-Mar-17 00:56:55.831047
#50 8.557   Downloaded aws-sdk-sts v1.100.0
2026-Mar-17 00:56:55.831047
#50 8.573   Downloaded http-body-util v0.1.3
2026-Mar-17 00:56:55.937142
#50 8.575   Downloaded hashlink v0.8.4
2026-Mar-17 00:56:55.937142
#50 8.577   Downloaded jobserver v0.1.34
2026-Mar-17 00:56:55.937142
#50 8.580   Downloaded pem v3.0.6
2026-Mar-17 00:56:55.937142
#50 8.582   Downloaded paste v1.0.15
2026-Mar-17 00:56:55.937142
#50 8.586   Downloaded parking_lot_core v0.9.12
2026-Mar-17 00:56:55.937142
#50 8.588   Downloaded parking_lot v0.12.5
2026-Mar-17 00:56:55.937142
#50 8.591   Downloaded hyper-tls v0.6.0
2026-Mar-17 00:56:55.937142
#50 8.593   Downloaded ordered-multimap v0.7.3
2026-Mar-17 00:56:55.937142
#50 8.596   Downloaded once_cell v1.21.3
2026-Mar-17 00:56:55.937142
#50 8.599   Downloaded num-traits v0.2.19
2026-Mar-17 00:56:55.937142
#50 8.602   Downloaded mio v1.1.1
2026-Mar-17 00:56:55.937142
#50 8.609   Downloaded minimal-lexical v0.2.1
2026-Mar-17 00:56:55.937142
#50 8.615   Downloaded crc-fast v1.6.0
2026-Mar-17 00:56:55.937142
#50 8.624   Downloaded aws-smithy-runtime v1.10.3
2026-Mar-17 00:56:55.937142
#50 8.631   Downloaded glob v0.3.3
2026-Mar-17 00:56:55.937142
#50 8.633   Downloaded aws-config v1.8.15
2026-Mar-17 00:56:55.937142
#50 8.641   Downloaded fs_extra v1.3.0
2026-Mar-17 00:56:55.937142
#50 8.643   Downloaded aho-corasick v1.1.4
2026-Mar-17 00:56:55.937142
#50 8.650   Downloaded outref v0.5.2
2026-Mar-17 00:56:55.937142
#50 8.652   Downloaded miniz_oxide v0.8.9
2026-Mar-17 00:56:55.937142
#50 8.656   Downloaded combine v4.6.7
2026-Mar-17 00:56:55.937142
#50 8.663   Downloaded der v0.6.1
2026-Mar-17 00:56:55.937142
#50 8.670   Downloaded crypto-bigint v0.5.5
2026-Mar-17 00:56:55.937142
#50 8.679   Downloaded crossbeam-channel v0.5.15
2026-Mar-17 00:56:56.045681
#50 8.684   Downloaded cc v1.2.56
2026-Mar-17 00:56:56.045681
#50 8.687   Downloaded base64 v0.22.1
2026-Mar-17 00:56:56.045681
#50 8.692   Downloaded base64 v0.21.7
2026-Mar-17 00:56:56.045681
#50 8.696   Downloaded aws-smithy-types v1.4.6
2026-Mar-17 00:56:56.045681
#50 8.702   Downloaded aws-smithy-http-client v1.1.12
2026-Mar-17 00:56:56.045681
#50 8.706   Downloaded aws-sigv4 v1.4.2
2026-Mar-17 00:56:56.045681
#50 8.774   Downloaded lock_api v0.4.14
2026-Mar-17 00:56:56.045681
#50 8.775   Downloaded hashlink v0.10.0
2026-Mar-17 00:56:56.045681
#50 8.778   Downloaded aws-sdk-ssooidc v1.98.0
2026-Mar-17 00:56:56.045681
#50 8.787   Downloaded find-msvc-tools v0.1.9
2026-Mar-17 00:56:56.150324
#50 8.790   Downloaded event-listener v5.4.1
2026-Mar-17 00:56:56.150324
#50 8.792   Downloaded ecdsa v0.14.8
2026-Mar-17 00:56:56.150324
#50 8.794   Downloaded aws-sdk-sso v1.96.0
2026-Mar-17 00:56:56.150324
#50 8.802   Downloaded aws-runtime v1.7.2
2026-Mar-17 00:56:56.150324
#50 8.807   Downloaded async-compression v0.4.41
2026-Mar-17 00:56:56.150324
#50 8.816   Downloaded nu-ansi-term v0.50.3
2026-Mar-17 00:56:56.150324
#50 8.819   Downloaded thread_local v1.1.9
2026-Mar-17 00:56:56.150324
#50 8.821   Downloaded memoffset v0.9.1
2026-Mar-17 00:56:56.150324
#50 8.822   Downloaded matchit v0.8.4
2026-Mar-17 00:56:56.150324
#50 8.825   Downloaded aws-smithy-runtime-api v1.11.6
2026-Mar-17 00:56:56.150324
#50 8.830   Downloaded bytes v1.11.1
2026-Mar-17 00:56:56.150324
#50 8.835   Downloaded aws-smithy-json v0.62.5
2026-Mar-17 00:56:56.150324
#50 8.837   Downloaded document-features v0.2.12
2026-Mar-17 00:56:56.150324
#50 8.839   Downloaded tinystr v0.8.2
2026-Mar-17 00:56:56.150324
#50 8.841   Downloaded dlv-list v0.5.2
2026-Mar-17 00:56:56.150324
#50 8.844   Downloaded crossbeam-epoch v0.9.18
2026-Mar-17 00:56:56.150324
#50 8.847   Downloaded crc32fast v1.5.0
2026-Mar-17 00:56:56.150324
#50 8.850   Downloaded concurrent-queue v2.5.0
2026-Mar-17 00:56:56.150324
#50 8.852   Downloaded base64ct v1.8.3
2026-Mar-17 00:56:56.150324
#50 8.855   Downloaded async-lock v3.4.2
2026-Mar-17 00:56:56.150324
#50 8.858   Downloaded digest v0.10.7
2026-Mar-17 00:56:56.150324
#50 8.860   Downloaded deranged v0.5.8
2026-Mar-17 00:56:56.150324
#50 8.862   Downloaded debugid v0.8.0
2026-Mar-17 00:56:56.150324
#50 8.863   Downloaded crypto-bigint v0.4.9
2026-Mar-17 00:56:56.150324
#50 8.869   Downloaded crossbeam-utils v0.8.21
2026-Mar-17 00:56:56.150324
#50 8.872   Downloaded cookie v0.18.1
2026-Mar-17 00:56:56.150324
#50 8.875   Downloaded byteorder v1.5.0
2026-Mar-17 00:56:56.150324
#50 8.877   Downloaded bytes-utils v0.1.4
2026-Mar-17 00:56:56.150324
#50 8.879   Downloaded aws-smithy-json v0.61.9
2026-Mar-17 00:56:56.150324
#50 8.881   Downloaded aws-smithy-eventstream v0.60.20
2026-Mar-17 00:56:56.150324
#50 8.884   Downloaded arraydeque v0.5.1
2026-Mar-17 00:56:56.150324
#50 8.886   Downloaded convert_case v0.6.0
2026-Mar-17 00:56:56.150324
#50 8.887   Downloaded compression-codecs v0.4.37
2026-Mar-17 00:56:56.150324
#50 8.892   Downloaded cexpr v0.6.0
2026-Mar-17 00:56:56.150324
#50 ...
2026-Mar-17 00:56:56.150324
2026-Mar-17 00:56:56.150324
#48 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-17 00:56:56.150324
#48 8.040  Downloading crates ...
2026-Mar-17 00:56:56.627560
#48 8.529   Downloaded adler v1.0.2
2026-Mar-17 00:56:56.749631
#48 8.546   Downloaded deno_unsync v0.4.4
2026-Mar-17 00:56:56.749631
#48 8.552   Downloaded adler2 v2.0.1
2026-Mar-17 00:56:56.749631
#48 8.651   Downloaded errno v0.3.14
2026-Mar-17 00:56:56.852464
#48 8.658   Downloaded heck v0.4.1
2026-Mar-17 00:56:56.852464
#48 8.662   Downloaded foreign-types-shared v0.1.1
2026-Mar-17 00:56:56.852464
#48 8.670   Downloaded form_urlencoded v1.2.2
2026-Mar-17 00:56:56.852464
#48 8.696   Downloaded fnv v1.0.7
2026-Mar-17 00:56:56.852464
#48 8.699   Downloaded ff v0.12.1
2026-Mar-17 00:56:56.852464
#48 8.702   Downloaded event-listener-strategy v0.5.4
2026-Mar-17 00:56:56.852464
#48 8.720   Downloaded alloc-stdlib v0.2.2
2026-Mar-17 00:56:56.852464
#48 8.743   Downloaded equivalent v1.0.2
2026-Mar-17 00:56:56.852464
#48 8.749   Downloaded fastrand v2.3.0
2026-Mar-17 00:56:56.852464
#48 8.754   Downloaded funty v2.0.0
2026-Mar-17 00:56:56.956423
#48 8.791   Downloaded gzip-header v1.0.0
2026-Mar-17 00:56:56.956423
#48 8.804   Downloaded futures-io v0.3.32
2026-Mar-17 00:56:56.956423
#48 8.806   Downloaded dunce v1.0.5
2026-Mar-17 00:56:56.956423
#48 8.810   Downloaded futures-task v0.3.32
2026-Mar-17 00:56:56.956423
#48 8.817   Downloaded futures-sink v0.3.32
2026-Mar-17 00:56:56.956423
#48 8.819   Downloaded generic-array v0.14.7
2026-Mar-17 00:56:56.956423
#48 8.822   Downloaded lazy_static v1.5.0
2026-Mar-17 00:56:56.956423
#48 8.826   Downloaded futures-core v0.3.32
2026-Mar-17 00:56:56.956423
#48 8.858   Downloaded heck v0.5.0
2026-Mar-17 00:56:57.056857
#48 8.863   Downloaded http-body v1.0.1
2026-Mar-17 00:56:57.056857
#48 8.866   Downloaded json5 v0.4.1
2026-Mar-17 00:56:57.056857
#48 8.870   Downloaded lazycell v1.3.0
2026-Mar-17 00:56:57.056857
#48 8.872   Downloaded block-buffer v0.10.4
2026-Mar-17 00:56:57.056857
#48 8.875   Downloaded hyper-tls v0.6.0
2026-Mar-17 00:56:57.056857
#48 8.878   Downloaded hex v0.4.3
2026-Mar-17 00:56:57.056857
#48 8.881   Downloaded httpdate v1.0.3
2026-Mar-17 00:56:57.056857
#48 8.900   Downloaded idna_adapter v1.2.1
2026-Mar-17 00:56:57.056857
#48 8.904   Downloaded itoa v1.0.17
2026-Mar-17 00:56:57.056857
#48 8.912   Downloaded futures-macro v0.3.32
2026-Mar-17 00:56:57.056857
#48 8.914   Downloaded if_chain v1.0.3
2026-Mar-17 00:56:57.056857
#48 8.917   Downloaded home v0.5.12
2026-Mar-17 00:56:57.056857
#48 8.919   Downloaded document-features v0.2.12
2026-Mar-17 00:56:57.056857
#48 8.922   Downloaded fslock v0.2.1
2026-Mar-17 00:56:57.056857
#48 8.926   Downloaded fs_extra v1.3.0
2026-Mar-17 00:56:57.056857
#48 8.930   Downloaded foreign-types v0.3.2
2026-Mar-17 00:56:57.056857
#48 8.932   Downloaded deranged v0.5.8
2026-Mar-17 00:56:57.056857
#48 8.936   Downloaded either v1.15.0
2026-Mar-17 00:56:57.056857
#48 8.939   Downloaded dotenvy v0.15.7
2026-Mar-17 00:56:57.056857
#48 8.945   Downloaded lru-slab v0.1.2
2026-Mar-17 00:56:57.056857
#48 8.947   Downloaded dlv-list v0.5.2
2026-Mar-17 00:56:57.056857
#48 8.952   Downloaded memoffset v0.9.1
2026-Mar-17 00:56:57.056857
#48 8.955   Downloaded mime v0.3.17
2026-Mar-17 00:56:57.056857
#48 8.958   Downloaded foldhash v0.1.5
2026-Mar-17 00:56:57.162540
#48 8.961   Downloaded group v0.12.1
2026-Mar-17 00:56:57.162540
#48 8.966   Downloaded glob v0.3.3
2026-Mar-17 00:56:57.162540
#48 8.970   Downloaded async-stream v0.3.6
2026-Mar-17 00:56:57.162540
#48 8.974   Downloaded http-body v0.4.6
2026-Mar-17 00:56:57.162540
#48 8.979   Downloaded libloading v0.8.9
2026-Mar-17 00:56:57.162540
#48 8.984   Downloaded ipnet v2.11.0
2026-Mar-17 00:56:57.162540
#48 8.988   Downloaded crc v3.4.0
2026-Mar-17 00:56:57.162540
#48 8.992   Downloaded futures-timer v3.0.3
2026-Mar-17 00:56:57.162540
#48 8.996   Downloaded futures-executor v0.3.32
2026-Mar-17 00:56:57.162540
#48 9.000   Downloaded ecdsa v0.14.8
2026-Mar-17 00:56:57.162540
#48 9.003   Downloaded matchers v0.2.0
2026-Mar-17 00:56:57.162540
#48 9.007   Downloaded hyper-rustls v0.24.2
2026-Mar-17 00:56:57.162540
#48 9.012   Downloaded lock_api v0.4.14
2026-Mar-17 00:56:57.162540
#48 9.016   Downloaded no-std-compat v0.4.1
2026-Mar-17 00:56:57.162540
#48 9.018   Downloaded event-listener v5.4.1
2026-Mar-17 00:56:57.162540
#48 9.022   Downloaded digest v0.10.7
2026-Mar-17 00:56:57.162540
#48 9.026   Downloaded find-msvc-tools v0.1.9
2026-Mar-17 00:56:57.162540
#48 9.029   Downloaded jobserver v0.1.34
2026-Mar-17 00:56:57.162540
#48 9.033   Downloaded hyper-rustls v0.27.7
2026-Mar-17 00:56:57.162540
#48 9.037   Downloaded http-body-util v0.1.3
2026-Mar-17 00:56:57.162540
#48 9.041   Downloaded hmac v0.12.1
2026-Mar-17 00:56:57.162540
#48 9.044   Downloaded futures-channel v0.3.32
2026-Mar-17 00:56:57.162540
#48 9.048   Downloaded hashlink v0.10.0
2026-Mar-17 00:56:57.162540
#48 9.051   Downloaded base16ct v0.1.1
2026-Mar-17 00:56:57.162540
#48 9.055   Downloaded nonzero_ext v0.3.0
2026-Mar-17 00:56:57.162540
#48 9.064   Downloaded native-tls v0.2.18
2026-Mar-17 00:56:57.267235
#48 9.068   Downloaded num-iter v0.1.45
2026-Mar-17 00:56:57.267235
#48 9.071   Downloaded getrandom v0.2.17
2026-Mar-17 00:56:57.267235
#48 9.077   Downloaded icu_provider v2.1.1
2026-Mar-17 00:56:57.267235
#48 9.082   Downloaded futures v0.3.32
2026-Mar-17 00:56:57.267235
#48 9.090   Downloaded num_cpus v1.17.0
2026-Mar-17 00:56:57.267235
#48 9.096   Downloaded flume v0.11.1
2026-Mar-17 00:56:57.267235
#48 9.101   Downloaded nu-ansi-term v0.50.3
2026-Mar-17 00:56:57.267235
#48 9.107   Downloaded icu_properties v2.1.2
2026-Mar-17 00:56:57.267235
#48 9.112   Downloaded parking v2.2.1
2026-Mar-17 00:56:57.267235
#48 9.115   Downloaded num-integer v0.1.46
2026-Mar-17 00:56:57.267235
#48 9.120   Downloaded pathdiff v0.2.3
2026-Mar-17 00:56:57.267235
#48 9.122   Downloaded percent-encoding v2.3.2
2026-Mar-17 00:56:57.267235
#48 9.125   Downloaded once_cell v1.21.3
2026-Mar-17 00:56:57.267235
#48 9.129   Downloaded cfg-if v1.0.4
2026-Mar-17 00:56:57.267235
#48 9.131   Downloaded icu_collections v2.1.1
2026-Mar-17 00:56:57.267235
#48 9.140   Downloaded pem-rfc7468 v0.7.0
2026-Mar-17 00:56:57.267235
#48 9.144   Downloaded indexmap v2.13.0
2026-Mar-17 00:56:57.267235
#48 9.150   Downloaded pin-utils v0.1.0
2026-Mar-17 00:56:57.267235
#48 9.153   Downloaded pin-project-internal v1.1.10
2026-Mar-17 00:56:57.267235
#48 9.156   Downloaded pest_generator v2.8.6
2026-Mar-17 00:56:57.267235
#48 9.159   Downloaded miniz_oxide v0.8.9
2026-Mar-17 00:56:57.267235
#48 9.163   Downloaded pkcs8 v0.9.0
2026-Mar-17 00:56:57.267235
#48 9.169   Downloaded pkcs8 v0.10.2
2026-Mar-17 00:56:57.382320
#48 9.174   Downloaded pkcs1 v0.7.5
2026-Mar-17 00:56:57.382320
#48 9.178   Downloaded libm v0.2.16
2026-Mar-17 00:56:57.382320
#48 9.194   Downloaded powerfmt v0.2.0
2026-Mar-17 00:56:57.382320
#48 9.197   Downloaded pkg-config v0.3.32
2026-Mar-17 00:56:57.382320
#48 9.200   Downloaded icu_properties_data v2.1.2
2026-Mar-17 00:56:57.382320
#48 9.218   Downloaded proc-macro-error v1.0.4
2026-Mar-17 00:56:57.382320
#48 9.225   Downloaded quanta v0.12.6
2026-Mar-17 00:56:57.382320
#48 9.230   Downloaded rand_core v0.9.5
2026-Mar-17 00:56:57.382320
#48 9.233   Downloaded libc v0.2.182
2026-Mar-17 00:56:57.382320
#48 9.284   Downloaded compression-core v0.4.31
2026-Mar-17 00:56:57.488141
#48 9.289   Downloaded rfc6979 v0.3.1
2026-Mar-17 00:56:57.488141
#48 9.292   Downloaded spki v0.7.3
2026-Mar-17 00:56:57.488141
#48 9.295   Downloaded spki v0.6.0
2026-Mar-17 00:56:57.488141
#48 9.298   Downloaded socket2 v0.5.10
2026-Mar-17 00:56:57.488141
#48 9.300   Downloaded slab v0.4.12
2026-Mar-17 00:56:57.488141
#48 9.302   Downloaded simple_asn1 v0.6.4
2026-Mar-17 00:56:57.488141
#48 9.304   Downloaded simd-adler32 v0.3.8
2026-Mar-17 00:56:57.488141
#48 9.306   Downloaded simd-abstraction v0.7.1
2026-Mar-17 00:56:57.488141
#48 9.308   Downloaded signature v2.2.0
2026-Mar-17 00:56:57.488141
#48 9.310   Downloaded signature v1.6.4
2026-Mar-17 00:56:57.488141
#48 9.312   Downloaded signal-hook-registry v1.4.8
2026-Mar-17 00:56:57.488141
#48 9.314   Downloaded sqlx-macros v0.8.6
2026-Mar-17 00:56:57.488141
#48 9.315   Downloaded smallvec v1.15.1
2026-Mar-17 00:56:57.488141
#48 9.318   Downloaded shlex v1.3.0
2026-Mar-17 00:56:57.488141
#48 9.320   Downloaded sharded-slab v0.1.7
2026-Mar-17 00:56:57.488141
#48 9.324   Downloaded sha2 v0.10.9
2026-Mar-17 00:56:57.488141
#48 9.328   Downloaded sha1_smol v1.0.1
2026-Mar-17 00:56:57.488141
#48 9.330   Downloaded sha1 v0.10.6
2026-Mar-17 00:56:57.488141
#48 9.332   Downloaded spin v0.9.8
2026-Mar-17 00:56:57.488141
#48 9.337   Downloaded strum v0.25.0
2026-Mar-17 00:56:57.488141
#48 9.339   Downloaded strum_macros v0.25.3
2026-Mar-17 00:56:57.488141
#48 9.342   Downloaded tagptr v0.2.0
2026-Mar-17 00:56:57.488141
#48 9.345   Downloaded tinystr v0.8.2
2026-Mar-17 00:56:57.488141
#48 9.349   Downloaded tinyvec v1.10.0
2026-Mar-17 00:56:57.488141
#48 9.352   Downloaded regex-lite v0.1.9
2026-Mar-17 00:56:57.488141
#48 9.355   Downloaded reqwest v0.12.28
2026-Mar-17 00:56:57.488141
#48 9.361   Downloaded regex v1.12.3
2026-Mar-17 00:56:57.488141
#48 9.367   Downloaded tiny-keccak v2.0.2
2026-Mar-17 00:56:57.488141
#48 9.371   Downloaded time-macros v0.2.27
2026-Mar-17 00:56:57.488141
#48 9.374   Downloaded time-core v0.1.8
2026-Mar-17 00:56:57.488141
#48 9.375   Downloaded thread_local v1.1.9
2026-Mar-17 00:56:57.488141
#48 9.377   Downloaded thiserror-impl v2.0.18
2026-Mar-17 00:56:57.488141
#48 9.379   Downloaded thiserror-impl v1.0.69
2026-Mar-17 00:56:57.488141
#48 9.381   Downloaded thiserror v2.0.18
2026-Mar-17 00:56:57.488141
#48 9.390   Downloaded thiserror v1.0.69
2026-Mar-17 00:56:57.600374
#48 9.397   Downloaded tap v1.0.1
2026-Mar-17 00:56:57.600374
#48 9.398   Downloaded synstructure v0.13.2
2026-Mar-17 00:56:57.600374
#48 9.400   Downloaded redis v0.27.6
2026-Mar-17 00:56:57.600374
#48 9.409   Downloaded raw-cpuid v11.6.0
2026-Mar-17 00:56:57.600374
#48 9.413   Downloaded rand v0.9.2
2026-Mar-17 00:56:57.600374
#48 9.418   Downloaded cpufeatures v0.2.17
2026-Mar-17 00:56:57.600374
#48 9.420   Downloaded atomic-waker v1.1.2
2026-Mar-17 00:56:57.600374
#48 9.422   Downloaded crc-catalog v2.4.0
2026-Mar-17 00:56:57.600374
#48 9.424   Downloaded rand v0.8.5
2026-Mar-17 00:56:57.600374
#48 9.428   Downloaded debugid v0.8.0
2026-Mar-17 00:56:57.600374
#48 9.430   Downloaded const-random-macro v0.1.16
2026-Mar-17 00:56:57.600374
#48 9.431   Downloaded sync_wrapper v1.0.2
2026-Mar-17 00:56:57.600374
#48 9.432   Downloaded subtle v2.6.1
2026-Mar-17 00:56:57.600374
#48 9.436   Downloaded quinn-proto v0.11.13
2026-Mar-17 00:56:57.600374
#48 9.446   Downloaded rustls-webpki v0.103.9
2026-Mar-17 00:56:57.600374
#48 9.452   Downloaded which v6.0.3
2026-Mar-17 00:56:57.600374
#48 9.455   Downloaded whoami v1.6.1
2026-Mar-17 00:56:57.600374
#48 9.458   Downloaded webpki-roots v0.26.11
2026-Mar-17 00:56:57.600374
#48 9.460   Downloaded web-time v1.1.0
2026-Mar-17 00:56:57.600374
#48 9.462   Downloaded vsimd v0.8.0
2026-Mar-17 00:56:57.600374
#48 9.465   Downloaded zerofrom-derive v0.1.6
2026-Mar-17 00:56:57.600374
#48 9.467   Downloaded zmij v1.0.21
2026-Mar-17 00:56:57.600374
#48 9.470   Downloaded zerovec-derive v0.11.2
2026-Mar-17 00:56:57.600374
#48 9.472   Downloaded time v0.3.47
2026-Mar-17 00:56:57.600374
#48 9.488   Downloaded syn v2.0.117
2026-Mar-17 00:56:57.736820
#48 9.503   Downloaded toml_edit v0.22.27
2026-Mar-17 00:56:57.736820
#48 9.510   Downloaded tower-http v0.6.8
2026-Mar-17 00:56:57.736820
#48 9.521   Downloaded unicode-normalization v0.1.25
2026-Mar-17 00:56:57.736820
#48 9.527   Downloaded url v2.5.8
2026-Mar-17 00:56:57.736820
#48 9.530   Downloaded utoipa-gen v4.3.1
2026-Mar-17 00:56:57.736820
#48 9.537   Downloaded async-stream-impl v0.3.6
2026-Mar-17 00:56:57.736820
#48 9.538   Downloaded tower v0.5.3
2026-Mar-17 00:56:57.736820
#48 9.553   Downloaded webpki-roots v1.0.6
2026-Mar-17 00:56:57.736820
#48 9.558   Downloaded vcpkg v0.2.15
2026-Mar-17 00:56:57.736820
#48 9.638   Downloaded tracing-subscriber v0.3.22
2026-Mar-17 00:56:57.839050
#48 9.648   Downloaded const-random v0.1.18
2026-Mar-17 00:56:57.839050
#48 9.650   Downloaded unicode-segmentation v1.12.0
2026-Mar-17 00:56:57.839050
#48 9.653   Downloaded typenum v1.19.0
2026-Mar-17 00:56:57.839050
#48 9.657   Downloaded winnow v0.7.14
2026-Mar-17 00:56:57.839050
#48 9.668   Downloaded tracing v0.1.44
2026-Mar-17 00:56:57.839050
#48 9.687   Downloaded tower v0.4.13
2026-Mar-17 00:56:57.839050
#48 9.702   Downloaded md-5 v0.10.6
2026-Mar-17 00:56:57.839050
#48 9.704   Downloaded num-conv v0.2.0
2026-Mar-17 00:56:57.839050
#48 9.705   Downloaded ahash v0.8.12
2026-Mar-17 00:56:57.839050
#48 9.708   Downloaded proc-macro-error-attr v1.0.4
2026-Mar-17 00:56:57.839050
#48 9.710   Downloaded autocfg v1.5.0
2026-Mar-17 00:56:57.839050
#48 9.712   Downloaded potential_utf v0.1.4
2026-Mar-17 00:56:57.839050
#48 9.714   Downloaded dashmap v6.1.0
2026-Mar-17 00:56:57.839050
#48 9.717   Downloaded cmake v0.1.57
2026-Mar-17 00:56:57.839050
#48 9.719   Downloaded bincode v1.3.3
2026-Mar-17 00:56:57.839050
#48 9.722   Downloaded aws-smithy-eventstream v0.60.20
2026-Mar-17 00:56:57.839050
#48 9.726   Downloaded aws-smithy-async v1.2.14
2026-Mar-17 00:56:57.839050
#48 9.730   Downloaded bit-set v0.5.3
2026-Mar-17 00:56:57.839050
#48 9.731   Downloaded alloc-no-stdlib v2.0.4
2026-Mar-17 00:56:57.839050
#48 9.733   Downloaded zerovec v0.11.5
2026-Mar-17 00:56:57.839050
#48 9.740   Downloaded zerotrie v0.2.3
2026-Mar-17 00:56:57.939392
#48 9.745   Downloaded yaml-rust2 v0.8.1
2026-Mar-17 00:56:57.939392
#48 9.783   Downloaded tokio-util v0.7.18
2026-Mar-17 00:56:57.939392
#48 9.793   Downloaded arraydeque v0.5.1
2026-Mar-17 00:56:57.939392
#48 9.794   Downloaded crypto-common v0.1.7
2026-Mar-17 00:56:57.939392
#48 9.795   Downloaded zerocopy v0.8.39
2026-Mar-17 00:56:57.939392
#48 9.825   Downloaded aws-credential-types v1.2.14
2026-Mar-17 00:56:57.939392
#48 9.827   Downloaded crossbeam-queue v0.3.12
2026-Mar-17 00:56:57.939392
#48 9.829   Downloaded cooked-waker v5.0.0
2026-Mar-17 00:56:57.939392
#48 9.830   Downloaded aws-smithy-json v0.61.9
2026-Mar-17 00:56:57.939392
#48 9.832   Downloaded async-trait v0.1.89
2026-Mar-17 00:56:57.939392
#48 9.837   Downloaded zeroize v1.8.2
2026-Mar-17 00:56:57.939392
#48 9.838   Downloaded zerofrom v0.1.6
2026-Mar-17 00:56:57.939392
#48 9.840   Downloaded yoke-derive v0.8.1
2026-Mar-17 00:56:57.939392
#48 9.841   Downloaded yoke v0.8.1
2026-Mar-17 00:56:58.039353
#48 9.843   Downloaded syn v1.0.109
2026-Mar-17 00:56:58.039353
#48 9.854   Downloaded sqlx-sqlite v0.8.6
2026-Mar-17 00:56:58.039353
#48 9.860   Downloaded bytes-utils v0.1.4
2026-Mar-17 00:56:58.039353
#48 9.863   Downloaded sqlx-postgres v0.8.6
2026-Mar-17 00:56:58.039353
#48 9.876   Downloaded data-encoding v2.10.0
2026-Mar-17 00:56:58.039353
#48 9.877   Downloaded byteorder v1.5.0
2026-Mar-17 00:56:58.039353
#48 9.879   Downloaded sqlx-mysql v0.8.6
2026-Mar-17 00:56:58.039353
#48 9.887   Downloaded sqlx v0.8.6
2026-Mar-17 00:56:58.039353
#48 9.904   Downloaded convert_case v0.6.0
2026-Mar-17 00:56:58.039353
#48 9.906   Downloaded concurrent-queue v2.5.0
2026-Mar-17 00:56:58.039353
#48 9.908   Downloaded pin-project-lite v0.2.16
2026-Mar-17 00:56:58.039353
#48 9.916   Downloaded anyhow v1.0.102
2026-Mar-17 00:56:58.039353
#48 9.921   Downloaded compression-codecs v0.4.37
2026-Mar-17 00:56:58.039353
#48 9.926   Downloaded semver v0.9.0
2026-Mar-17 00:56:58.039353
#48 9.928   Downloaded async-lock v3.4.2
2026-Mar-17 00:56:58.039353
#48 9.930   Downloaded cexpr v0.6.0
2026-Mar-17 00:56:58.039353
#48 9.933   Downloaded bit-vec v0.6.3
2026-Mar-17 00:56:58.039353
#48 9.934   Downloaded base64ct v1.8.3
2026-Mar-17 00:56:58.039353
#48 9.938   Downloaded xmlparser v0.13.6
2026-Mar-17 00:56:58.039353
#48 9.941   Downloaded wyz v0.5.1
2026-Mar-17 00:56:58.143436
#48 9.942   Downloaded writeable v0.6.2
2026-Mar-17 00:56:58.143436
#48 9.944   Downloaded utoipa v4.2.3
2026-Mar-17 00:56:58.143436
#48 9.948   Downloaded unicode-properties v0.1.4
2026-Mar-17 00:56:58.143436
#48 9.950   Downloaded unicode-ident v1.0.24
2026-Mar-17 00:56:58.143436
#48 9.954   Downloaded crossbeam-epoch v0.9.18
2026-Mar-17 00:56:58.143436
#48 9.957   Downloaded aws-smithy-http v0.63.6
2026-Mar-17 00:56:58.143436
#48 9.960   Downloaded ucd-trie v0.1.7
2026-Mar-17 00:56:58.143436
#48 9.961   Downloaded tracing-log v0.2.0
2026-Mar-17 00:56:58.143436
#48 9.963   Downloaded tracing-attributes v0.1.31
2026-Mar-17 00:56:58.143436
#48 9.966   Downloaded crc32fast v1.5.0
2026-Mar-17 00:56:58.143436
#48 9.968   Downloaded aws-smithy-json v0.62.5
2026-Mar-17 00:56:58.143436
#48 9.970   Downloaded tower-service v0.3.3
2026-Mar-17 00:56:58.143436
#48 9.971   Downloaded which v4.4.2
2026-Mar-17 00:56:58.143436
#48 9.973   Downloaded aws-types v1.3.14
2026-Mar-17 00:56:58.143436
#48 9.975   Downloaded want v0.3.1
2026-Mar-17 00:56:58.143436
#48 9.976   Downloaded cookie_store v0.22.1
2026-Mar-17 00:56:58.143436
#48 9.979   Downloaded version_check v0.9.5
2026-Mar-17 00:56:58.143436
#48 9.980   Downloaded uuid v1.21.0
2026-Mar-17 00:56:58.143436
#48 9.984   Downloaded utf8_iter v1.0.4
2026-Mar-17 00:56:58.143436
#48 9.985   Downloaded urlencoding v2.1.3
2026-Mar-17 00:56:58.143436
#48 9.986   Downloaded untrusted v0.9.0
2026-Mar-17 00:56:58.143436
#48 9.988   Downloaded aws-smithy-http v0.62.6
2026-Mar-17 00:56:58.143436
#48 9.991   Downloaded unicode-id-start v1.4.0
2026-Mar-17 00:56:58.143436
#48 9.994   Downloaded const-oid v0.9.6
2026-Mar-17 00:56:58.143436
#48 9.997   Downloaded unicode-bidi v0.3.18
2026-Mar-17 00:56:58.143436
#48 9.999   Downloaded crossbeam-utils v0.8.21
2026-Mar-17 00:56:58.143436
#48 10.00   Downloaded try-lock v0.2.5
2026-Mar-17 00:56:58.143436
#48 10.00   Downloaded tracing-core v0.1.36
2026-Mar-17 00:56:58.143436
#48 10.01   Downloaded tower-layer v0.3.3
2026-Mar-17 00:56:58.143436
#48 10.01   Downloaded toml_write v0.1.2
2026-Mar-17 00:56:58.143436
#48 10.01   Downloaded aws-smithy-checksums v0.63.12
2026-Mar-17 00:56:58.143436
#48 10.01   Downloaded toml_datetime v0.6.11
2026-Mar-17 00:56:58.143436
#48 10.01   Downloaded toml v0.8.23
2026-Mar-17 00:56:58.143436
#48 10.02   Downloaded tokio-stream v0.1.18
2026-Mar-17 00:56:58.143436
#48 10.02   Downloaded crypto-bigint v0.4.9
2026-Mar-17 00:56:58.143436
#48 10.03   Downloaded bitflags v2.11.0
2026-Mar-17 00:56:58.143436
#48 10.03   Downloaded aws-smithy-xml v0.60.15
2026-Mar-17 00:56:58.143436
#48 10.03   Downloaded sqlx-core v0.8.6
2026-Mar-17 00:56:58.143436
#48 10.04   Downloaded serde_json v1.0.149
2026-Mar-17 00:56:58.250019
#48 10.05   Downloaded clang-sys v1.8.1
2026-Mar-17 00:56:58.250019
#48 10.06   Downloaded litemap v0.8.1
2026-Mar-17 00:56:58.250019
#48 10.06   Downloaded serde v1.0.228
2026-Mar-17 00:56:58.250019
#48 10.06   Downloaded tinyvec_macros v0.1.1
2026-Mar-17 00:56:58.250019
#48 10.06   Downloaded rustls-webpki v0.101.7
2026-Mar-17 00:56:58.250019
#48 10.09   Downloaded axum-core v0.5.6
2026-Mar-17 00:56:58.250019
#48 10.09   Downloaded rustls v0.23.37
2026-Mar-17 00:56:58.250019
#48 10.11   Downloaded rustls v0.21.12
2026-Mar-17 00:56:58.250019
#48 10.12   Downloaded rustix v0.38.44
2026-Mar-17 00:56:58.250019
#48 10.15   Downloaded cookie v0.18.1
2026-Mar-17 00:56:58.359529
#48 10.15   Downloaded config v0.14.1
2026-Mar-17 00:56:58.359529
#48 10.16   Downloaded rsa v0.9.10
2026-Mar-17 00:56:58.359529
#48 10.17   Downloaded regex-syntax v0.8.10
2026-Mar-17 00:56:58.359529
#48 10.17   Downloaded regex-automata v0.4.14
2026-Mar-17 00:56:58.359529
#48 10.19   Downloaded tokio-macros v2.6.0
2026-Mar-17 00:56:58.359529
#48 10.19   Downloaded tokio-native-tls v0.3.1
2026-Mar-17 00:56:58.359529
#48 10.19   Downloaded allocator-api2 v0.2.21
2026-Mar-17 00:56:58.359529
#48 10.20   Downloaded base64 v0.22.1
2026-Mar-17 00:56:58.359529
#48 10.20   Downloaded quinn v0.11.9
2026-Mar-17 00:56:58.359529
#48 10.20   Downloaded publicsuffix v2.3.0
2026-Mar-17 00:56:58.359529
#48 10.21   Downloaded prettyplease v0.2.37
2026-Mar-17 00:56:58.359529
#48 10.21   Downloaded portable-atomic v1.13.1
2026-Mar-17 00:56:58.359529
#48 10.22   Downloaded stringprep v0.1.5
2026-Mar-17 00:56:58.359529
#48 10.22   Downloaded static_assertions v1.1.0
2026-Mar-17 00:56:58.359529
#48 10.22   Downloaded stable_deref_trait v1.2.1
2026-Mar-17 00:56:58.359529
#48 10.23   Downloaded sqlx-macros-core v0.8.6
2026-Mar-17 00:56:58.359529
#48 10.23   Downloaded spinning_top v0.3.0
2026-Mar-17 00:56:58.359529
#48 10.23   Downloaded socket2 v0.6.2
2026-Mar-17 00:56:58.359529
#48 10.23   Downloaded sourcemap v8.0.1
2026-Mar-17 00:56:58.359529
#48 10.23   Downloaded serde_v8 v0.209.0
2026-Mar-17 00:56:58.359529
#48 10.24   Downloaded serde_urlencoded v0.7.1
2026-Mar-17 00:56:58.359529
#48 10.24   Downloaded serde_spanned v0.6.9
2026-Mar-17 00:56:58.359529
#48 10.24   Downloaded serde_path_to_error v0.1.20
2026-Mar-17 00:56:58.359529
#48 10.24   Downloaded serde_derive v1.0.228
2026-Mar-17 00:56:58.359529
#48 10.25   Downloaded serde_core v1.0.228
2026-Mar-17 00:56:58.359529
#48 10.25   Downloaded deno_ops v0.176.0
2026-Mar-17 00:56:58.359529
#48 10.26   Downloaded semver-parser v0.7.0
2026-Mar-17 00:56:58.459662
#48 10.26   Downloaded semver v1.0.27
2026-Mar-17 00:56:58.459662
#48 10.27   Downloaded sec1 v0.3.0
2026-Mar-17 00:56:58.459662
#48 10.27   Downloaded sct v0.7.1
2026-Mar-17 00:56:58.459662
#48 10.27   Downloaded scopeguard v1.2.0
2026-Mar-17 00:56:58.459662
#48 10.27   Downloaded ryu v1.0.23
2026-Mar-17 00:56:58.459662
#48 10.28   Downloaded rustversion v1.0.22
2026-Mar-17 00:56:58.459662
#48 10.28   Downloaded rustls-pki-types v1.14.0
2026-Mar-17 00:56:58.459662
#48 10.28   Downloaded rustls-native-certs v0.8.3
2026-Mar-17 00:56:58.459662
#48 10.29   Downloaded rustc_version v0.4.1
2026-Mar-17 00:56:58.459662
#48 10.29   Downloaded rustc_version v0.2.3
2026-Mar-17 00:56:58.459662
#48 10.29   Downloaded rustc-hash v2.1.1
2026-Mar-17 00:56:58.459662
#48 10.29   Downloaded rustc-hash v1.1.0
2026-Mar-17 00:56:58.459662
#48 10.29   Downloaded rust-ini v0.20.0
2026-Mar-17 00:56:58.459662
#48 10.30   Downloaded ron v0.8.1
2026-Mar-17 00:56:58.459662
#48 10.30   Downloaded linux-raw-sys v0.4.15
2026-Mar-17 00:56:58.459662
#48 10.35   Downloaded tokio-rustls v0.26.4
2026-Mar-17 00:56:58.459662
#48 10.36   Downloaded aws-smithy-http-client v1.1.12
2026-Mar-17 00:56:58.459662
#48 10.36   Downloaded crypto-bigint v0.5.5
2026-Mar-17 00:56:58.564466
#48 10.37   Downloaded bytes v1.11.1
2026-Mar-17 00:56:58.564466
#48 10.38   Downloaded aws-sdk-sso v1.96.0
2026-Mar-17 00:56:58.564466
#48 10.38   Downloaded arc-swap v1.8.2
2026-Mar-17 00:56:58.564466
#48 10.39   Downloaded aws-sdk-ssooidc v1.98.0
2026-Mar-17 00:56:58.564466
#48 10.40   Downloaded tokio-rustls v0.24.1
2026-Mar-17 00:56:58.564466
#48 10.40   Downloaded cc v1.2.56
2026-Mar-17 00:56:58.564466
#48 10.40   Downloaded aws-runtime v1.7.2
2026-Mar-17 00:56:58.564466
#48 10.41   Downloaded pest_meta v2.8.6
2026-Mar-17 00:56:58.564466
#48 10.41   Downloaded async-compression v0.4.41
2026-Mar-17 00:56:58.564466
#48 10.42   Downloaded crossbeam-channel v0.5.15
2026-Mar-17 00:56:58.564466
#48 10.43   Downloaded base64 v0.21.7
2026-Mar-17 00:56:58.564466
#48 10.43   Downloaded aws-smithy-runtime-api v1.11.6
2026-Mar-17 00:56:58.564466
#48 10.44   Downloaded pest v2.8.6
2026-Mar-17 00:56:58.564466
#48 10.44   Downloaded p256 v0.11.1
2026-Mar-17 00:56:58.564466
#48 10.45   Downloaded openssl-sys v0.9.111
2026-Mar-17 00:56:58.564466
#48 10.45   Downloaded openssl v0.10.75
2026-Mar-17 00:56:58.564466
#48 10.47   Downloaded encoding_rs v0.8.35
2026-Mar-17 00:56:58.669373
#48 10.49   Downloaded aws-sigv4 v1.4.2
2026-Mar-17 00:56:58.669373
#48 10.56   Downloaded aws-smithy-types v1.4.6
2026-Mar-17 00:56:58.669373
#48 10.56   Downloaded aws-smithy-runtime v1.10.3
2026-Mar-17 00:56:58.669373
#48 10.57   Downloaded combine v4.6.7
2026-Mar-17 00:56:58.787946
#48 10.58   Downloaded cfg_aliases v0.2.1
2026-Mar-17 00:56:58.787946
#48 10.58   Downloaded base64-simd v0.8.0
2026-Mar-17 00:56:58.787946
#48 10.58   Downloaded base64-simd v0.7.0
2026-Mar-17 00:56:58.787946
#48 10.58   Downloaded aws-smithy-query v0.60.15
2026-Mar-17 00:56:58.787946
#48 10.58   Downloaded aws-smithy-observability v0.2.6
2026-Mar-17 00:56:58.787946
#48 10.59   Downloaded rand_core v0.6.4
2026-Mar-17 00:56:58.787946
#48 10.59   Downloaded rand_chacha v0.9.0
2026-Mar-17 00:56:58.787946
#48 10.59   Downloaded rand_chacha v0.3.1
2026-Mar-17 00:56:58.787946
#48 10.59   Downloaded radium v0.7.0
2026-Mar-17 00:56:58.787946
#48 10.59   Downloaded quote v1.0.44
2026-Mar-17 00:56:58.787946
#48 10.60   Downloaded quinn-udp v0.5.14
2026-Mar-17 00:56:58.787946
#48 10.60   Downloaded psl-types v2.0.11
2026-Mar-17 00:56:58.787946
#48 10.60   Downloaded proc-macro2 v1.0.106
2026-Mar-17 00:56:58.787946
#48 10.60   Downloaded proc-macro-rules-macros v0.4.0
2026-Mar-17 00:56:58.787946
#48 10.61   Downloaded proc-macro-rules v0.4.0
2026-Mar-17 00:56:58.787946
#48 10.61   Downloaded num-bigint-dig v0.8.6
2026-Mar-17 00:56:58.787946
#48 10.61   Downloaded num-bigint v0.4.6
2026-Mar-17 00:56:58.787946
#48 10.62   Downloaded moka v0.12.13
2026-Mar-17 00:56:58.787946
#48 10.63   Downloaded ppv-lite86 v0.2.21
2026-Mar-17 00:56:58.787946
#48 10.63   Downloaded hyper v0.14.32
2026-Mar-17 00:56:58.787946
#48 10.64   Downloaded hkdf v0.12.4
2026-Mar-17 00:56:58.787946
#48 10.64   Downloaded h2 v0.4.13
2026-Mar-17 00:56:58.787946
#48 10.65   Downloaded h2 v0.3.27
2026-Mar-17 00:56:58.787946
#48 10.66   Downloaded crc-fast v1.6.0
2026-Mar-17 00:56:58.787946
#48 10.67   Downloaded futures-util v0.3.32
2026-Mar-17 00:56:58.787946
#48 10.69   Downloaded aws-config v1.8.15
2026-Mar-17 00:56:58.893050
#48 10.70   Downloaded iri-string v0.7.10
2026-Mar-17 00:56:58.893050
#48 10.71   Downloaded axum v0.8.8
2026-Mar-17 00:56:58.893050
#48 10.72   Downloaded aho-corasick v1.1.4
2026-Mar-17 00:56:58.893050
#48 10.72   Downloaded hyper v1.8.1
2026-Mar-17 00:56:58.893050
#48 10.73   Downloaded hashbrown v0.16.1
2026-Mar-17 00:56:58.893050
#48 10.74   Downloaded brotli-decompressor v5.0.0
2026-Mar-17 00:56:58.893050
#48 10.74   Downloaded aws-sdk-sts v1.100.0
2026-Mar-17 00:56:58.893050
#48 10.76   Downloaded hashbrown v0.15.5
2026-Mar-17 00:56:58.893050
#48 10.76   Downloaded nom v7.1.3
2026-Mar-17 00:56:58.893050
#48 10.77   Downloaded itertools v0.13.0
2026-Mar-17 00:56:58.893050
#48 10.78   Downloaded bindgen v0.69.5
2026-Mar-17 00:56:58.893050
#48 10.79   Downloaded itertools v0.12.1
2026-Mar-17 00:56:58.893050
#48 10.79   Downloaded idna v1.1.0
2026-Mar-17 00:56:58.994802
#48 10.80   Downloaded hashbrown v0.14.5
2026-Mar-17 00:56:58.994802
#48 10.80   Downloaded governor v0.8.1
2026-Mar-17 00:56:58.994802
#48 10.81   Downloaded pin-project v1.1.10
2026-Mar-17 00:56:58.994802
#48 10.83   Downloaded bitvec v1.0.1
2026-Mar-17 00:56:58.994802
#48 10.85   Downloaded aws-lc-rs v1.16.1
2026-Mar-17 00:56:58.994802
#48 10.86   Downloaded mio v1.1.1
2026-Mar-17 00:56:58.994802
#48 10.87   Downloaded minimal-lexical v0.2.1
2026-Mar-17 00:56:58.994802
#48 10.87   Downloaded hyper-util v0.1.20
2026-Mar-17 00:56:58.994802
#48 10.88   Downloaded deno_core v0.300.0
2026-Mar-17 00:56:58.994802
#48 10.89   Downloaded http v1.4.0
2026-Mar-17 00:56:58.994802
#48 10.90   Downloaded pest_derive v2.8.6
2026-Mar-17 00:56:59.098383
#48 ...
2026-Mar-17 00:56:59.098383
2026-Mar-17 00:56:59.098383
#51 [frontend builder 7/8] RUN npm run paraglide:compile
2026-Mar-17 00:56:59.098383
#51 0.296
2026-Mar-17 00:56:59.098383
#51 0.296 > frontend@0.0.1 paraglide:compile
2026-Mar-17 00:56:59.098383
#51 0.296 > paraglide-js compile --project ./project.inlang --outdir ./src/lib/paraglide --strategy url cookie globalVariable baseLocale
2026-Mar-17 00:56:59.098383
#51 0.296
2026-Mar-17 00:56:59.098383
#51 0.780 ℹ [paraglide-js] Compiling inlang project ...
2026-Mar-17 00:56:59.098383
#51 7.155 ✔ [paraglide-js] Successfully compiled inlang project.
2026-Mar-17 00:56:59.098383
#51 DONE 7.6s
2026-Mar-17 00:56:59.098383
2026-Mar-17 00:56:59.098383
#48 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-17 00:56:59.098383
#48 10.90   Downloaded parking_lot_core v0.9.12
2026-Mar-17 00:56:59.098383
#48 10.90   Downloaded parking_lot v0.12.5
2026-Mar-17 00:56:59.098383
#48 10.90   Downloaded ordered-multimap v0.7.3
2026-Mar-17 00:56:59.098383
#48 10.91   Downloaded memchr v2.8.0
2026-Mar-17 00:56:59.098383
#48 10.91   Downloaded http v0.2.12
2026-Mar-17 00:56:59.098383
#48 10.92   Downloaded pem v3.0.6
2026-Mar-17 00:56:59.098383
#48 10.92   Downloaded paste v1.0.15
2026-Mar-17 00:56:59.098383
#48 10.92   Downloaded num-traits v0.2.19
2026-Mar-17 00:56:59.098383
#48 10.93   Downloaded futures-intrusive v0.5.0
2026-Mar-17 00:56:59.098383
#48 10.93   Downloaded flate2 v1.1.9
2026-Mar-17 00:56:59.098383
#48 10.94   Downloaded openssl-probe v0.2.1
2026-Mar-17 00:56:59.098383
#48 10.94   Downloaded icu_locale_core v2.1.1
2026-Mar-17 00:56:59.098383
#48 10.95   Downloaded der v0.7.10
2026-Mar-17 00:56:59.098383
#48 10.96   Downloaded outref v0.5.2
2026-Mar-17 00:56:59.098383
#48 10.96   Downloaded outref v0.1.0
2026-Mar-17 00:56:59.098383
#48 10.96   Downloaded openssl-macros v0.1.1
2026-Mar-17 00:56:59.098383
#48 10.96   Downloaded miniz_oxide v0.7.4
2026-Mar-17 00:56:59.098383
#48 10.96   Downloaded icu_normalizer_data v2.1.1
2026-Mar-17 00:56:59.098383
#48 10.97   Downloaded icu_normalizer v2.1.1
2026-Mar-17 00:56:59.098383
#48 10.97   Downloaded httparse v1.10.1
2026-Mar-17 00:56:59.098383
#48 10.98   Downloaded atoi v2.0.0
2026-Mar-17 00:56:59.098383
#48 10.98   Downloaded log v0.4.29
2026-Mar-17 00:56:59.098383
#48 10.98   Downloaded jsonwebtoken v9.3.1
2026-Mar-17 00:56:59.098383
#48 10.99   Downloaded elliptic-curve v0.12.3
2026-Mar-17 00:56:59.098383
#48 10.99   Downloaded getrandom v0.4.1
2026-Mar-17 00:56:59.098383
#48 11.00   Downloaded getrandom v0.3.4
2026-Mar-17 00:56:59.098383
#48 11.00   Downloaded matchit v0.8.4
2026-Mar-17 00:56:59.212081
#48 11.00   Downloaded lru v0.12.5
2026-Mar-17 00:56:59.212081
#48 11.01   Downloaded litrs v1.0.0
2026-Mar-17 00:56:59.212081
#48 11.01   Downloaded der v0.6.1
2026-Mar-17 00:56:59.212081
#48 11.02   Downloaded crunchy v0.2.4
2026-Mar-17 00:56:59.212081
#48 11.02   Downloaded hashlink v0.8.4
2026-Mar-17 00:56:59.212081
#48 11.02   Downloaded displaydoc v0.2.5
2026-Mar-17 00:56:59.329952
#48 11.12   Downloaded v8 v0.101.0
2026-Mar-17 00:57:00.213849
#48 12.11   Downloaded brotli v8.0.2
2026-Mar-17 00:57:00.452039
#48 12.35   Downloaded tokio v1.49.0
2026-Mar-17 00:57:00.909601
#48 12.81   Downloaded ring v0.17.14
2026-Mar-17 00:57:01.041564
#48 12.87   Downloaded aws-sdk-s3 v1.119.0
2026-Mar-17 00:57:02.193322
#48 14.09   Downloaded deno_core_icudata v0.0.73
2026-Mar-17 00:57:02.360665
#48 14.26   Downloaded libsqlite3-sys v0.30.1
2026-Mar-17 00:57:02.960267
#48 14.86   Downloaded aws-lc-sys v0.38.0
2026-Mar-17 00:57:03.357739
#48 15.26    Compiling proc-macro2 v1.0.106
2026-Mar-17 00:57:03.357739
#48 15.26    Compiling unicode-ident v1.0.24
2026-Mar-17 00:57:03.482156
#48 15.26    Compiling quote v1.0.44
2026-Mar-17 00:57:03.482156
#48 15.26    Compiling libc v0.2.182
2026-Mar-17 00:57:03.482156
#48 15.26    Compiling cfg-if v1.0.4
2026-Mar-17 00:57:03.482156
#48 15.26    Compiling serde v1.0.228
2026-Mar-17 00:57:03.482156
#48 15.26    Compiling serde_core v1.0.228
2026-Mar-17 00:57:03.482156
#48 15.29    Compiling version_check v0.9.5
2026-Mar-17 00:57:03.482156
#48 15.30    Compiling parking_lot_core v0.9.12
2026-Mar-17 00:57:03.482156
#48 15.32    Compiling pin-project-lite v0.2.16
2026-Mar-17 00:57:03.482156
#48 15.33    Compiling shlex v1.3.0
2026-Mar-17 00:57:03.482156
#48 15.34    Compiling memchr v2.8.0
2026-Mar-17 00:57:03.482156
#48 15.35    Compiling once_cell v1.21.3
2026-Mar-17 00:57:03.482156
#48 15.35    Compiling scopeguard v1.2.0
2026-Mar-17 00:57:03.482156
#48 15.35    Compiling bytes v1.11.1
2026-Mar-17 00:57:03.482156
#48 15.36    Compiling itoa v1.0.17
2026-Mar-17 00:57:03.482156
#48 15.38    Compiling futures-core v0.3.32
2026-Mar-17 00:57:03.583210
#48 15.40    Compiling find-msvc-tools v0.1.9
2026-Mar-17 00:57:03.583210
#48 15.41    Compiling futures-sink v0.3.32
2026-Mar-17 00:57:03.583210
#48 15.41    Compiling autocfg v1.5.0
2026-Mar-17 00:57:03.583210
#48 15.42    Compiling typenum v1.19.0
2026-Mar-17 00:57:03.583210
#48 15.44    Compiling log v0.4.29
2026-Mar-17 00:57:03.583210
#48 15.45    Compiling slab v0.4.12
2026-Mar-17 00:57:03.583210
#48 15.46    Compiling futures-io v0.3.32
2026-Mar-17 00:57:03.583210
#48 15.46    Compiling futures-task v0.3.32
2026-Mar-17 00:57:03.583210
#48 15.47    Compiling zeroize v1.8.2
2026-Mar-17 00:57:03.583210
#48 15.47    Compiling zerocopy v0.8.39
2026-Mar-17 00:57:03.583210
#48 15.47    Compiling equivalent v1.0.2
2026-Mar-17 00:57:03.583210
#48 15.48    Compiling subtle v2.6.1
2026-Mar-17 00:57:03.692986
#48 15.51    Compiling fnv v1.0.7
2026-Mar-17 00:57:03.692986
#48 15.53    Compiling hashbrown v0.16.1
2026-Mar-17 00:57:03.692986
#48 15.58    Compiling lock_api v0.4.14
2026-Mar-17 00:57:03.692986
#48 15.59    Compiling
2026-Mar-17 00:57:03.793414
futures-channel v0.3.32
2026-Mar-17 00:57:03.793414
#48 15.60    Compiling tracing-core v0.1.36
2026-Mar-17 00:57:03.793414
#48 15.60    Compiling percent-encoding v2.3.2
2026-Mar-17 00:57:03.793414
#48 15.63    Compiling num-conv v0.2.0
2026-Mar-17 00:57:03.793414
#48 15.68    Compiling icu_normalizer_data v2.1.1
2026-Mar-17 00:57:03.793414
#48 15.69    Compiling generic-array v0.14.7
2026-Mar-17 00:57:03.793414
#48 15.69    Compiling num-traits v0.2.19
2026-Mar-17 00:57:03.986358
#48 15.69    Compiling time-core v0.1.8
2026-Mar-17 00:57:03.986358
#48 15.70    Compiling icu_properties_data v2.1.2
2026-Mar-17 00:57:03.986358
#48 15.72    Compiling powerfmt v0.2.0
2026-Mar-17 00:57:03.986358
#48 15.74    Compiling pin-utils v0.1.0
2026-Mar-17 00:57:03.986358
#48 15.76    Compiling ryu v1.0.23
2026-Mar-17 00:57:03.986358
#48 15.76    Compiling stable_deref_trait v1.2.1
2026-Mar-17 00:57:03.986358
#48 15.76    Compiling crc32fast v1.5.0
2026-Mar-17 00:57:03.986358
#48 15.77    Compiling untrusted v0.9.0
2026-Mar-17 00:57:03.986358
#48 15.79    Compiling dunce v1.0.5
2026-Mar-17 00:57:03.986358
#48 15.89    Compiling http v1.4.0
2026-Mar-17 00:57:04.089492
#48 15.90    Compiling fs_extra v1.3.0
2026-Mar-17 00:57:04.089492
#48 15.91    Compiling http v0.2.12
2026-Mar-17 00:57:04.089492
#48 15.95    Compiling zmij v1.0.21
2026-Mar-17 00:57:04.089492
#48 15.98    Compiling deranged v0.5.8
2026-Mar-17 00:57:04.089492
#48 15.98    Compiling time-macros v0.2.27
2026-Mar-17 00:57:04.089492
#48 15.98    Compiling form_urlencoded v1.2.2
2026-Mar-17 00:57:04.089492
#48 15.98    Compiling rustls-pki-types v1.14.0
2026-Mar-17 00:57:04.089492
#48 15.99    Compiling aws-lc-rs v1.16.1
2026-Mar-17 00:57:04.237534
#48 16.01    Compiling tower-service v0.3.3
2026-Mar-17 00:57:04.237534
#48 16.03    Compiling writeable v0.6.2
2026-Mar-17 00:57:04.237534
#48 16.04    Compiling litemap v0.8.1
2026-Mar-17 00:57:04.237534
#48 16.04    Compiling httparse v1.10.1
2026-Mar-17 00:57:04.237534
#48 16.04    Compiling vsimd v0.8.0
2026-Mar-17 00:57:04.237534
#48 16.05    Compiling base64 v0.22.1
2026-Mar-17 00:57:04.237534
#48 16.06    Compiling outref v0.5.2
2026-Mar-17 00:57:04.237534
#48 16.14    Compiling try-lock v0.2.5
2026-Mar-17 00:57:04.342461
#48 16.21    Compiling rustls v0.23.37
2026-Mar-17 00:57:04.342461
#48 16.22    Compiling httpdate v1.0.3
2026-Mar-17 00:57:04.455365
#48 16.25    Compiling allocator-api2 v0.2.21
2026-Mar-17 00:57:04.455365
#48 16.25    Compiling atomic-waker v1.1.2
2026-Mar-17 00:57:04.455365
#48 16.25    Compiling crossbeam-utils v0.8.21
2026-Mar-17 00:57:04.455365
#48 16.26    Compiling tower-layer v0.3.3
2026-Mar-17 00:57:04.455365
#48 16.33    Compiling cpufeatures v0.2.17
2026-Mar-17 00:57:04.455365
#48 16.33    Compiling openssl-probe v0.2.1
2026-Mar-17 00:57:04.455365
#48 16.36    Compiling utf8_iter v1.0.4
2026-Mar-17 00:57:04.560317
#48 16.41    Compiling want v0.3.1
2026-Mar-17 00:57:04.560317
#48 16.41    Compiling webpki-roots v1.0.6
2026-Mar-17 00:57:04.560317
#48 16.45    Compiling base64-simd v0.8.0
2026-Mar-17 00:57:04.560317
#48 16.46    Compiling
2026-Mar-17 00:57:04.665314
sync_wrapper v1.0.2
2026-Mar-17 00:57:04.665314
#48 16.48    Compiling ipnet v2.11.0
2026-Mar-17 00:57:04.665314
#48 16.50    Compiling syn v2.0.117
2026-Mar-17 00:57:04.665314
#48 16.57    Compiling serde_json v1.0.149
2026-Mar-17 00:57:04.796332
#48 16.59    Compiling errno v0.3.14
2026-Mar-17 00:57:04.796332
#48 16.59    Compiling mio v1.1.1
2026-Mar-17 00:57:04.796332
#48 16.60    Compiling socket2 v0.6.2
2026-Mar-17 00:57:04.796332
#48 16.60    Compiling getrandom v0.2.17
2026-Mar-17 00:57:04.796332
#48 16.60    Compiling http-body v1.0.1
2026-Mar-17 00:57:04.796332
#48 16.60    Compiling http-body v0.4.6
2026-Mar-17 00:57:04.796332
#48 16.60    Compiling socket2 v0.5.10
2026-Mar-17 00:57:04.796332
#48 16.64    Compiling bitflags v2.11.0
2026-Mar-17 00:57:04.796332
#48 16.65    Compiling thiserror v2.0.18
2026-Mar-17 00:57:04.796332
#48 16.67    Compiling rustversion v1.0.22
2026-Mar-17 00:57:04.796332
#48 16.69    Compiling getrandom v0.4.1
2026-Mar-17 00:57:04.937955
#48 16.74    Compiling jobserver v0.1.34
2026-Mar-17 00:57:04.937955
#48 16.75    Compiling rand_core v0.6.4
2026-Mar-17 00:57:04.937955
#48 16.77    Compiling signal-hook-registry v1.4.8
2026-Mar-17 00:57:04.937955
#48 16.79    Compiling http-body-util v0.1.3
2026-Mar-17 00:57:04.937955
#48 16.84    Compiling
2026-Mar-17 00:57:05.041145
num-integer v0.1.46
2026-Mar-17 00:57:05.041145
#48 16.90    Compiling home v0.5.12
2026-Mar-17 00:57:05.041145
#48 16.93    Compiling rustls-native-certs v0.8.3
2026-Mar-17 00:57:05.041145
#48 16.93    Compiling const-oid v0.9.6
2026-Mar-17 00:57:05.041145
#48 16.94    Compiling hex v0.4.3
2026-Mar-17 00:57:05.140948
#48 17.02    Compiling base64ct v1.8.3
2026-Mar-17 00:57:05.140948
#48 17.02    Compiling cc v1.2.56
2026-Mar-17 00:57:05.140948
#48 17.02    Compiling time v0.3.47
2026-Mar-17 00:57:05.140948
#48 17.03    Compiling pkg-config v0.3.32
2026-Mar-17 00:57:05.140948
#48 17.04    Compiling vcpkg v0.2.15
2026-Mar-17 00:57:05.273542
#48 17.07    Compiling der v0.6.1
2026-Mar-17 00:57:05.273542
#48 17.08    Compiling ahash v0.8.12
2026-Mar-17 00:57:05.273542
#48 17.14    Compiling block-buffer v0.10.4
2026-Mar-17 00:57:05.273542
#48 17.17    Compiling crypto-common v0.1.7
2026-Mar-17 00:57:05.379467
#48 17.20    Compiling rustls v0.21.12
2026-Mar-17 00:57:05.379467
#48 17.20    Compiling crypto-bigint v0.4.9
2026-Mar-17 00:57:05.379467
#48 17.28    Compiling ff v0.12.1
2026-Mar-17 00:57:05.483388
#48 17.29    Compiling aho-corasick v1.1.4
2026-Mar-17 00:57:05.483388
#48 17.32    Compiling digest v0.10.7
2026-Mar-17 00:57:05.483388
#48 17.33    Compiling rustix v0.38.44
2026-Mar-17 00:57:05.483388
#48 17.33    Compiling glob v0.3.3
2026-Mar-17 00:57:05.483388
#48 17.33    Compiling base16ct v0.1.1
2026-Mar-17 00:57:05.483388
#48 17.33    Compiling crc-catalog v2.4.0
2026-Mar-17 00:57:05.483388
#48 17.38    Compiling
2026-Mar-17 00:57:05.615018
uuid v1.21.0
2026-Mar-17 00:57:05.615018
#48 17.48    Compiling foldhash v0.1.5
2026-Mar-17 00:57:05.615018
#48 17.48    Compiling regex-syntax v0.8.10
2026-Mar-17 00:57:05.615018
#48 17.48    Compiling group v0.12.1
2026-Mar-17 00:57:05.615018
#48 17.52    Compiling indexmap v2.13.0
2026-Mar-17 00:57:05.728905
#48 17.52    Compiling crc v3.4.0
2026-Mar-17 00:57:05.728905
#48 17.53    Compiling concurrent-queue v2.5.0
2026-Mar-17 00:57:05.728905
#48 17.60    Compiling hmac v0.12.1
2026-Mar-17 00:57:05.728905
#48 17.63    Compiling sha2 v0.10.9
2026-Mar-17 00:57:05.835879
#48 17.66    Compiling hashbrown v0.15.5
2026-Mar-17 00:57:05.835879
#48 17.70    Compiling tinyvec_macros v0.1.1
2026-Mar-17 00:57:05.835879
#48 17.70    Compiling parking v2.2.1
2026-Mar-17 00:57:05.835879
#48 17.74    Compiling
2026-Mar-17 00:57:05.960607
cmake v0.1.57
2026-Mar-17 00:57:05.960607
#48 17.75    Compiling clang-sys v1.8.1
2026-Mar-17 00:57:05.960607
#48 17.79    Compiling spki v0.6.0
2026-Mar-17 00:57:05.960607
#48 17.86    Compiling
2026-Mar-17 00:57:06.115538
alloc-no-stdlib v2.0.4
2026-Mar-17 00:57:06.115538
#48 17.87    Compiling semver v1.0.27
2026-Mar-17 00:57:06.115538
#48 17.91    Compiling prettyplease v0.2.37
2026-Mar-17 00:57:06.115538
#48 17.93    Compiling linux-raw-sys v0.4.15
2026-Mar-17 00:57:06.115538
#48 17.93    Compiling event-listener v5.4.1
2026-Mar-17 00:57:06.115538
#48 18.02    Compiling pkcs8 v0.9.0
2026-Mar-17 00:57:06.228410
#48 18.02    Compiling alloc-stdlib v0.2.2
2026-Mar-17 00:57:06.228410
#48 18.02    Compiling tinyvec v1.10.0
2026-Mar-17 00:57:06.228410
#48 18.04    Compiling rfc6979 v0.3.1
2026-Mar-17 00:57:06.228410
#48 18.13    Compiling rustc_version v0.4.1
2026-Mar-17 00:57:06.228410
#48 ...
2026-Mar-17 00:57:06.228410
2026-Mar-17 00:57:06.228410
#50 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-17 00:57:06.228410
#50 8.896   Downloaded bincode v1.3.3
2026-Mar-17 00:57:06.228410
#50 8.899   Downloaded aws-smithy-http v0.63.6
2026-Mar-17 00:57:06.228410
#50 8.903   Downloaded aws-credential-types v1.2.14
2026-Mar-17 00:57:06.228410
#50 8.988   Downloaded futures-channel v0.3.32
2026-Mar-17 00:57:06.228410
#50 8.995   Downloaded hyper-rustls v0.27.7
2026-Mar-17 00:57:06.228410
#50 9.026   Downloaded v8 v0.101.0
2026-Mar-17 00:57:06.228410
#50 10.03   Downloaded tracing-attributes v0.1.31
2026-Mar-17 00:57:06.228410
#50 10.04   Downloaded litrs v1.0.0
2026-Mar-17 00:57:06.228410
#50 10.04   Downloaded icu_properties v2.1.2
2026-Mar-17 00:57:06.228410
#50 10.04   Downloaded httparse v1.10.1
2026-Mar-17 00:57:06.228410
#50 10.04   Downloaded hmac v0.12.1
2026-Mar-17 00:57:06.228410
#50 10.05   Downloaded getrandom v0.3.4
2026-Mar-17 00:57:06.228410
#50 10.05   Downloaded getrandom v0.2.17
2026-Mar-17 00:57:06.228410
#50 10.06   Downloaded elliptic-curve v0.12.3
2026-Mar-17 00:57:06.228410
#50 10.06   Downloaded toml v0.8.23
2026-Mar-17 00:57:06.228410
#50 10.07   Downloaded pest_generator v2.8.6
2026-Mar-17 00:57:06.228410
#50 10.07   Downloaded log v0.4.29
2026-Mar-17 00:57:06.228410
#50 10.07   Downloaded litemap v0.8.1
2026-Mar-17 00:57:06.228410
#50 10.07   Downloaded jsonwebtoken v9.3.1
2026-Mar-17 00:57:06.228410
#50 10.08   Downloaded icu_provider v2.1.1
2026-Mar-17 00:57:06.228410
#50 10.09   Downloaded hyper-rustls v0.24.2
2026-Mar-17 00:57:06.228410
#50 10.34   Downloaded icu_normalizer_data v2.1.1
2026-Mar-17 00:57:06.228410
#50 10.36   Downloaded icu_locale_core v2.1.1
2026-Mar-17 00:57:06.228410
#50 10.37   Downloaded getrandom v0.4.1
2026-Mar-17 00:57:06.228410
#50 10.40   Downloaded flume v0.11.1
2026-Mar-17 00:57:06.228410
#50 10.56   Downloaded futures v0.3.32
2026-Mar-17 00:57:06.228410
#50 10.57   Downloaded icu_normalizer v2.1.1
2026-Mar-17 00:57:06.228410
#50 10.63   Downloaded flate2 v1.1.9
2026-Mar-17 00:57:06.228410
#50 10.77   Downloaded icu_collections v2.1.1
2026-Mar-17 00:57:06.228410
#50 10.79   Downloaded futures-intrusive v0.5.0
2026-Mar-17 00:57:06.228410
#50 11.19   Downloaded hyper-util v0.1.20
2026-Mar-17 00:57:06.228410
#50 11.40   Downloaded http v1.4.0
2026-Mar-17 00:57:06.228410
#50 11.42   Downloaded http v0.2.12
2026-Mar-17 00:57:06.228410
#50 11.44   Downloaded indexmap v2.13.0
2026-Mar-17 00:57:06.228410
#50 11.91   Downloaded hashbrown v0.15.5
2026-Mar-17 00:57:06.228410
#50 11.93   Downloaded itertools v0.13.0
2026-Mar-17 00:57:06.228410
#50 11.95   Downloaded hashbrown v0.16.1
2026-Mar-17 00:57:06.228410
#50 11.99   Downloaded governor v0.8.1
2026-Mar-17 00:57:06.228410
#50 12.01   Downloaded idna v1.1.0
2026-Mar-17 00:57:06.228410
#50 12.07   Downloaded iri-string v0.7.10
2026-Mar-17 00:57:06.228410
#50 12.19   Downloaded libm v0.2.16
2026-Mar-17 00:57:06.228410
#50 12.22   Downloaded itertools v0.12.1
2026-Mar-17 00:57:06.228410
#50 12.25   Downloaded hashbrown v0.14.5
2026-Mar-17 00:57:06.228410
#50 12.27   Downloaded icu_properties_data v2.1.2
2026-Mar-17 00:57:06.228410
#50 12.37   Downloaded h2 v0.3.27
2026-Mar-17 00:57:06.228410
#50 12.42   Downloaded hkdf v0.12.4
2026-Mar-17 00:57:06.228410
#50 12.42   Downloaded h2 v0.4.13
2026-Mar-17 00:57:06.228410
#50 12.43   Downloaded futures-util v0.3.32
2026-Mar-17 00:57:06.228410
#50 12.49   Downloaded hyper v1.8.1
2026-Mar-17 00:57:06.228410
#50 12.50   Downloaded hyper v0.14.32
2026-Mar-17 00:57:06.228410
#50 12.81   Downloaded time v0.3.47
2026-Mar-17 00:57:06.228410
#50 13.78   Downloaded tracing v0.1.44
2026-Mar-17 00:57:06.228410
#50 14.69   Downloaded libc v0.2.182
2026-Mar-17 00:57:06.228410
#50 15.97   Downloaded encoding_rs v0.8.35
2026-Mar-17 00:57:06.228410
#50 17.08   Downloaded linux-raw-sys v0.4.15
2026-Mar-17 00:57:06.340528
#50 ...
2026-Mar-17 00:57:06.340528
2026-Mar-17 00:57:06.340528
#48 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-17 00:57:06.340528
#48 18.16    Compiling sec1 v0.3.0
2026-Mar-17 00:57:06.340528
#48 18.21    Compiling signature v1.6.4
2026-Mar-17 00:57:06.340528
#48 18.24    Compiling getrandom v0.3.4
2026-Mar-17 00:57:06.455282
#48 18.28    Compiling openssl v0.10.75
2026-Mar-17 00:57:06.455282
#48 18.29    Compiling adler2 v2.0.1
2026-Mar-17 00:57:06.455282
#48 18.36    Compiling ring v0.17.14
2026-Mar-17 00:57:06.573580
#48 18.36    Compiling aws-lc-sys v0.38.0
2026-Mar-17 00:57:06.573580
#48 18.37    Compiling openssl-sys v0.9.111
2026-Mar-17 00:57:06.573580
#48 18.37    Compiling tokio v1.49.0
2026-Mar-17 00:57:06.573580
#48 18.38    Compiling minimal-lexical v0.2.1
2026-Mar-17 00:57:06.573580
#48 18.43    Compiling elliptic-curve v0.12.3
2026-Mar-17 00:57:06.573580
#48 18.47    Compiling foreign-types-shared v0.1.1
2026-Mar-17 00:57:06.712571
#48 18.47    Compiling thiserror v1.0.69
2026-Mar-17 00:57:06.712571
#48 18.50    Compiling simd-adler32 v0.3.8
2026-Mar-17 00:57:06.712571
#48 18.57    Compiling unicode-normalization v0.1.25
2026-Mar-17 00:57:06.712571
#48 18.57    Compiling nom v7.1.3
2026-Mar-17 00:57:06.825517
#48 18.63    Compiling hashlink v0.10.0
2026-Mar-17 00:57:06.825517
#48 18.66    Compiling foreign-types v0.3.2
2026-Mar-17 00:57:06.825517
#48 18.68    Compiling aws-types v1.3.14
2026-Mar-17 00:57:06.825517
#48 18.68    Compiling futures-util v0.3.32
2026-Mar-17 00:57:06.825517
#48 18.70    Compiling miniz_oxide v0.8.9
2026-Mar-17 00:57:06.940688
#48 18.76    Compiling webpki-roots v0.26.11
2026-Mar-17 00:57:06.940688
#48 18.79    Compiling ecdsa v0.14.8
2026-Mar-17 00:57:06.940688
#48 18.79    Compiling brotli-decompressor v5.0.0
2026-Mar-17 00:57:06.940688
#48 18.81    Compiling crossbeam-queue v0.3.12
2026-Mar-17 00:57:06.940688
#48 18.84    Compiling md-5 v0.10.6
2026-Mar-17 00:57:07.054829
#48 18.91    Compiling libloading v0.8.9
2026-Mar-17 00:57:07.160558
#48 18.96    Compiling anyhow v1.0.102
2026-Mar-17 00:57:07.160558
#48 19.06    Compiling regex-automata v0.4.14
2026-Mar-17 00:57:07.280511
#48 19.06    Compiling crunchy v0.2.4
2026-Mar-17 00:57:07.280511
#48 19.11    Compiling unicode-properties v0.1.4
2026-Mar-17 00:57:07.280511
#48 19.13    Compiling unicode-bidi v0.3.18
2026-Mar-17 00:57:07.280511
#48 19.18    Compiling native-tls v0.2.18
2026-Mar-17 00:57:07.493719
#48 19.18    Compiling bindgen v0.69.5
2026-Mar-17 00:57:07.493719
#48 19.19    Compiling p256 v0.11.1
2026-Mar-17 00:57:07.493719
#48 19.19    Compiling hkdf v0.12.4
2026-Mar-17 00:57:07.765863
#48 19.51    Compiling flate2 v1.1.9
2026-Mar-17 00:57:07.765863
#48 19.53    Compiling crypto-bigint v0.5.5
2026-Mar-17 00:57:07.765863
#48 19.67    Compiling stringprep v0.1.5
2026-Mar-17 00:57:07.928660
#48 19.67    Compiling atoi v2.0.0
2026-Mar-17 00:57:07.928660
#48 19.71    Compiling cookie v0.18.1
2026-Mar-17 00:57:07.928660
#48 19.72    Compiling dotenvy v0.15.7
2026-Mar-17 00:57:07.928660
#48 19.77    Compiling lazy_static v1.5.0
2026-Mar-17 00:57:08.036997
#48 19.86    Compiling ucd-trie v0.1.7
2026-Mar-17 00:57:08.036997
#48 19.91    Compiling adler v1.0.2
2026-Mar-17 00:57:08.200606
#48 19.95    Compiling whoami v1.6.1
2026-Mar-17 00:57:08.303884
#48 20.14    Compiling synstructure v0.13.2
2026-Mar-17 00:57:08.303884
#48 20.18    Compiling ppv-lite86 v0.2.21
2026-Mar-17 00:57:08.303884
#48 20.20    Compiling
2026-Mar-17 00:57:08.433100
brotli v8.0.2
2026-Mar-17 00:57:08.433100
#48 20.25    Compiling lazycell v1.3.0
2026-Mar-17 00:57:08.572144
#48 20.35    Compiling tiny-keccak v2.0.2
2026-Mar-17 00:57:08.572144
#48 20.35    Compiling cexpr v0.6.0
2026-Mar-17 00:57:08.572144
#48 20.35    Compiling fastrand v2.3.0
2026-Mar-17 00:57:08.572144
#48 20.39    Compiling compression-core v0.4.31
2026-Mar-17 00:57:08.572144
#48 20.39    Compiling rustc-hash v1.1.0
2026-Mar-17 00:57:08.572144
#48 20.47    Compiling hashbrown v0.14.5
2026-Mar-17 00:57:08.718982
#48 20.47    Compiling byteorder v1.5.0
2026-Mar-17 00:57:08.821642
#48 20.66    Compiling pest v2.8.6
2026-Mar-17 00:57:08.821642
#48 20.69    Compiling miniz_oxide v0.7.4
2026-Mar-17 00:57:08.977606
#48 20.75    Compiling rand_core v0.9.5
2026-Mar-17 00:57:08.977606
#48 20.88    Compiling gzip-header v1.0.0
2026-Mar-17 00:57:09.081794
#48 ...
2026-Mar-17 00:57:09.081794
2026-Mar-17 00:57:09.081794
#52 [frontend builder 8/8] RUN node build-docker.mjs
2026-Mar-17 00:57:09.081794
#52 1.064 The following Vite config options will be overridden by SvelteKit:
2026-Mar-17 00:57:09.081794
#52 1.064   - build.outDir
2026-Mar-17 00:57:09.081794
#52 1.094 vite v6.4.1 building SSR bundle for production...
2026-Mar-17 00:57:09.081794
#52 1.120 transforming...
2026-Mar-17 00:57:09.081794
#52 6.660 "optionsMiddleware" is imported from external module "@better-auth/core/api" but never used in "node_modules/better-auth/dist/api/index.mjs" and "node_modules/better-auth/dist/plugins/index.mjs".
2026-Mar-17 00:57:09.081794
#52 6.661 "getTelemetryAuthConfig" is imported from external module "@better-auth/telemetry" but never used in "node_modules/better-auth/dist/index.mjs".
2026-Mar-17 00:57:09.081794
#52 6.661 ✓ 964 modules transformed.
2026-Mar-17 00:57:09.081794
#52 7.028 rendering chunks...
2026-Mar-17 00:57:09.081794
#52 8.821 vite v6.4.1 building for production...
2026-Mar-17 00:57:09.207556
#52 ...
2026-Mar-17 00:57:09.207556
2026-Mar-17 00:57:09.207556
#48 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-17 00:57:09.207556
#48 21.06    Compiling fslock v0.2.1
2026-Mar-17 00:57:09.207556
#48 21.11    Compiling rand_chacha v0.3.1
2026-Mar-17 00:57:09.353544
#48 21.25    Compiling encoding_rs v0.8.35
2026-Mar-17 00:57:09.494498
#48 21.31    Compiling mime v0.3.17
2026-Mar-17 00:57:09.716588
#48 21.42    Compiling rand v0.8.5
2026-Mar-17 00:57:09.716588
#48 21.47    Compiling litrs v1.0.0
2026-Mar-17 00:57:09.751483
#48 21.65    Compiling radium v0.7.0
2026-Mar-17 00:57:09.876771
#48 21.65    Compiling psl-types v2.0.11
2026-Mar-17 00:57:09.876771
#48 21.65    Compiling heck v0.5.0
2026-Mar-17 00:57:09.876771
#48 21.66    Compiling paste v1.0.15
2026-Mar-17 00:57:09.876771
#48 21.77    Compiling regex v1.12.3
2026-Mar-17 00:57:10.005886
#48 21.78    Compiling rand_chacha v0.9.0
2026-Mar-17 00:57:10.114891
#48 21.92    Compiling arc-swap v1.8.2
2026-Mar-17 00:57:10.114891
#48 22.00    Compiling heck v0.4.1
2026-Mar-17 00:57:10.336932
#48 22.14    Compiling iri-string v0.7.10
2026-Mar-17 00:57:10.460570
#48 22.25    Compiling tap v1.0.1
2026-Mar-17 00:57:10.460570
#48 22.34    Compiling sha1_smol v1.0.1
2026-Mar-17 00:57:10.607032
#48 22.41    Compiling num-bigint v0.4.6
2026-Mar-17 00:57:10.607032
#48 22.44    Compiling portable-atomic v1.13.1
2026-Mar-17 00:57:10.723807
#48 22.57    Compiling serde_derive v1.0.228
2026-Mar-17 00:57:10.836418
#48 22.63    Compiling tokio-macros v2.6.0
2026-Mar-17 00:57:10.836418
#48 22.64    Compiling zerofrom-derive v0.1.6
2026-Mar-17 00:57:10.836418
#48 22.69    Compiling yoke-derive v0.8.1
2026-Mar-17 00:57:10.974313
#48 22.76    Compiling tracing-attributes v0.1.31
2026-Mar-17 00:57:10.974313
#48 22.76    Compiling futures-macro v0.3.32
2026-Mar-17 00:57:10.974313
#48 22.83    Compiling zerovec-derive v0.11.2
2026-Mar-17 00:57:10.974313
#48 22.88    Compiling displaydoc v0.2.5
2026-Mar-17 00:57:11.122378
#48 22.91    Compiling thiserror-impl v2.0.18
2026-Mar-17 00:57:11.122378
#48 22.96    Compiling openssl-macros v0.1.1
2026-Mar-17 00:57:11.122378
#48 22.96    Compiling thiserror-impl v1.0.69
2026-Mar-17 00:57:11.260358
#48 23.11    Compiling async-trait v0.1.89
2026-Mar-17 00:57:11.260358
#48 23.11    Compiling document-features v0.2.12
2026-Mar-17 00:57:11.260358
#48 23.16    Compiling pest_meta v2.8.6
2026-Mar-17 00:57:11.426804
#48 23.33    Compiling outref v0.1.0
2026-Mar-17 00:57:11.732637
#48 23.46    Compiling regex-lite v0.1.9
2026-Mar-17 00:57:11.952559
#48 23.76    Compiling tokio-stream v0.1.18
2026-Mar-17 00:57:12.142592
#48 23.90    Compiling simd-abstraction v0.7.1
2026-Mar-17 00:57:12.428578
#48 24.18    Compiling pest_generator v2.8.6
2026-Mar-17 00:57:12.428578
#48 24.21    Compiling strum_macros v0.25.3
2026-Mar-17 00:57:12.428578
#48 24.32    Compiling proc-macro-rules-macros v0.4.0
2026-Mar-17 00:57:12.547482
#48 24.38    Compiling wyz v0.5.1
2026-Mar-17 00:57:12.992359
#48 24.77    Compiling const-random-macro v0.1.16
2026-Mar-17 00:57:12.992359
#48 24.78    Compiling rand v0.9.2
2026-Mar-17 00:57:13.145388
#48 24.91    Compiling proc-macro-error-attr v1.0.4
2026-Mar-17 00:57:13.145388
#48 24.94    Compiling memoffset v0.9.1
2026-Mar-17 00:57:13.145388
#48 25.05    Compiling xmlparser v0.13.6
2026-Mar-17 00:57:13.273354
#48 25.17    Compiling
2026-Mar-17 00:57:13.427532
syn v1.0.109
2026-Mar-17 00:57:13.427532
#48 25.20    Compiling tracing v0.1.44
2026-Mar-17 00:57:13.427532
#48 25.33    Compiling funty v2.0.0
2026-Mar-17 00:57:13.698545
#48 25.53    Compiling const-random v0.1.18
2026-Mar-17 00:57:13.798085
#48 25.64    Compiling proc-macro-rules v0.4.0
2026-Mar-17 00:57:13.798085
#48 25.70    Compiling axum-core v0.5.6
2026-Mar-17 00:57:13.941966
#48 25.70    Compiling aws-smithy-xml v0.60.15
2026-Mar-17 00:57:13.941966
#48 25.75    Compiling base64-simd v0.7.0
2026-Mar-17 00:57:13.941966
#48 25.84    Compiling pin-project-internal v1.1.10
2026-Mar-17 00:57:14.076617
#48 25.84    Compiling event-listener-strategy v0.5.4
2026-Mar-17 00:57:14.076617
#48 25.88    Compiling sha1 v0.10.6
2026-Mar-17 00:57:14.076617
#48 25.92    Compiling serde_path_to_error v0.1.20
2026-Mar-17 00:57:14.076617
#48 25.94    Compiling zerofrom v0.1.6
2026-Mar-17 00:57:14.270673
#48 26.01    Compiling proc-macro-error v1.0.4
2026-Mar-17 00:57:14.270673
#48 26.02    Compiling matchit v0.8.4
2026-Mar-17 00:57:14.282080
#48 26.18    Compiling bitvec v1.0.1
2026-Mar-17 00:57:14.282080
#48 26.18    Compiling if_chain v1.0.3
2026-Mar-17 00:57:14.423002
#48 26.18    Compiling data-encoding v2.10.0
2026-Mar-17 00:57:14.423002
#48 26.18    Compiling pest_derive v2.8.6
2026-Mar-17 00:57:14.423002
#48 26.18    Compiling bit-vec v0.6.3
2026-Mar-17 00:57:14.583498
#48 26.34    Compiling yoke v0.8.1
2026-Mar-17 00:57:14.583498
#48 26.36    Compiling urlencoding v2.1.3
2026-Mar-17 00:57:14.583498
#48 26.36    Compiling unicode-id-start v1.4.0
2026-Mar-17 00:57:14.583498
#48 26.40    Compiling compression-codecs v0.4.37
2026-Mar-17 00:57:14.583498
#48 26.40    Compiling async-lock v3.4.2
2026-Mar-17 00:57:14.738534
#48 26.54    Compiling bit-set v0.5.3
2026-Mar-17 00:57:14.853563
#48 26.69    Compiling zerovec v0.11.5
2026-Mar-17 00:57:14.853563
#48 26.70    Compiling zerotrie v0.2.3
2026-Mar-17 00:57:15.050238
#48 26.75    Compiling tower v0.4.13
2026-Mar-17 00:57:15.050238
#48 26.79    Compiling dlv-list v0.5.2
2026-Mar-17 00:57:15.050238
#48 26.83    Compiling crossbeam-channel v0.5.15
2026-Mar-17 00:57:15.050238
#48 26.84    Compiling crossbeam-epoch v0.9.18
2026-Mar-17 00:57:15.234239
#48 26.95    Compiling static_assertions v1.1.0
2026-Mar-17 00:57:15.234239
#48 26.95    Compiling deno_core_icudata v0.0.73
2026-Mar-17 00:57:15.234239
#48 27.04    Compiling toml_write v0.1.2
2026-Mar-17 00:57:15.234239
#48 27.13    Compiling
2026-Mar-17 00:57:15.372793
extractor v0.1.0 (/app/crates/extractor)
2026-Mar-17 00:57:15.372793
#48 27.15    Compiling cooked-waker v5.0.0
2026-Mar-17 00:57:15.372793
#48 27.17    Compiling tagptr v0.2.0
2026-Mar-17 00:57:15.372793
#48 27.27    Compiling
2026-Mar-17 00:57:15.482545
winnow v0.7.14
2026-Mar-17 00:57:15.482545
#48 27.27    Compiling async-stream-impl v0.3.6
2026-Mar-17 00:57:15.482545
#48 27.33    Compiling hashlink v0.8.4
2026-Mar-17 00:57:15.482545
#48 27.38    Compiling
2026-Mar-17 00:57:15.579786
ordered-multimap v0.7.3
2026-Mar-17 00:57:15.579786
#48 27.45    Compiling lru v0.12.5
2026-Mar-17 00:57:15.579786
#48 27.48    Compiling
2026-Mar-17 00:57:15.701254
raw-cpuid v11.6.0
2026-Mar-17 00:57:15.701254
#48 27.51    Compiling num_cpus v1.17.0
2026-Mar-17 00:57:15.701254
#48 27.51    Compiling strum v0.25.0
2026-Mar-17 00:57:15.701254
#48 27.54    Compiling base64 v0.21.7
2026-Mar-17 00:57:15.701254
#48 27.57    Compiling unicode-segmentation v1.12.0
2026-Mar-17 00:57:15.921729
#48 27.82    Compiling pin-project v1.1.10
2026-Mar-17 00:57:16.040959
#48 27.88    Compiling tinystr v0.8.2
2026-Mar-17 00:57:16.040959
#48 27.94    Compiling potential_utf v0.1.4
2026-Mar-17 00:57:16.179008
#48 27.95    Compiling deno_ops v0.176.0
2026-Mar-17 00:57:16.179008
#48 28.07    Compiling arraydeque v0.5.1
2026-Mar-17 00:57:16.450888
#48 28.25    Compiling icu_locale_core v2.1.1
2026-Mar-17 00:57:16.450888
#48 ...
2026-Mar-17 00:57:16.450888
2026-Mar-17 00:57:16.450888
#50 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-17 00:57:16.450888
#50 19.15   Downloaded libsqlite3-sys v0.30.1
2026-Mar-17 00:57:16.450888
#50 19.35    Compiling proc-macro2 v1.0.106
2026-Mar-17 00:57:16.450888
#50 19.35    Compiling quote v1.0.44
2026-Mar-17 00:57:16.450888
#50 19.35    Compiling unicode-ident v1.0.24
2026-Mar-17 00:57:16.450888
#50 19.35    Compiling libc v0.2.182
2026-Mar-17 00:57:16.450888
#50 19.35    Compiling cfg-if v1.0.4
2026-Mar-17 00:57:16.450888
#50 19.36    Compiling serde v1.0.228
2026-Mar-17 00:57:16.450888
#50 19.39    Compiling serde_core v1.0.228
2026-Mar-17 00:57:16.450888
#50 19.39    Compiling parking_lot_core v0.9.12
2026-Mar-17 00:57:16.450888
#50 19.39    Compiling pin-project-lite v0.2.16
2026-Mar-17 00:57:16.450888
#50 19.39    Compiling shlex v1.3.0
2026-Mar-17 00:57:16.450888
#50 19.39    Compiling bytes v1.11.1
2026-Mar-17 00:57:16.450888
#50 19.41    Compiling scopeguard v1.2.0
2026-Mar-17 00:57:16.450888
#50 19.41    Compiling futures-core v0.3.32
2026-Mar-17 00:57:16.450888
#50 19.45    Compiling find-msvc-tools v0.1.9
2026-Mar-17 00:57:16.450888
#50 19.45    Compiling version_check v0.9.5
2026-Mar-17 00:57:16.450888
#50 19.45    Compiling memchr v2.8.0
2026-Mar-17 00:57:16.450888
#50 19.45    Compiling itoa v1.0.17
2026-Mar-17 00:57:16.450888
#50 19.45    Compiling once_cell v1.21.3
2026-Mar-17 00:57:16.450888
#50 19.45    Compiling futures-sink v0.3.32
2026-Mar-17 00:57:16.450888
#50 19.45    Compiling typenum v1.19.0
2026-Mar-17 00:57:16.450888
#50 19.45    Compiling autocfg v1.5.0
2026-Mar-17 00:57:16.450888
#50 19.45    Compiling log v0.4.29
2026-Mar-17 00:57:16.450888
#50 19.45    Compiling slab v0.4.12
2026-Mar-17 00:57:16.450888
#50 19.49    Compiling zeroize v1.8.2
2026-Mar-17 00:57:16.450888
#50 19.51    Compiling futures-io v0.3.32
2026-Mar-17 00:57:16.450888
#50 19.51    Compiling futures-task v0.3.32
2026-Mar-17 00:57:16.450888
#50 19.51    Compiling subtle v2.6.1
2026-Mar-17 00:57:16.450888
#50 19.51    Compiling fnv v1.0.7
2026-Mar-17 00:57:16.450888
#50 19.53    Compiling equivalent v1.0.2
2026-Mar-17 00:57:16.450888
#50 19.53    Compiling percent-encoding v2.3.2
2026-Mar-17 00:57:16.450888
#50 19.57    Compiling zerocopy v0.8.39
2026-Mar-17 00:57:16.450888
#50 19.57    Compiling icu_normalizer_data v2.1.1
2026-Mar-17 00:57:16.450888
#50 19.61    Compiling icu_properties_data v2.1.2
2026-Mar-17 00:57:16.450888
#50 19.61    Compiling hashbrown v0.16.1
2026-Mar-17 00:57:16.450888
#50 19.61    Compiling pin-utils v0.1.0
2026-Mar-17 00:57:16.450888
#50 19.64    Compiling powerfmt v0.2.0
2026-Mar-17 00:57:16.450888
#50 19.69    Compiling futures-channel v0.3.32
2026-Mar-17 00:57:16.450888
#50 19.69    Compiling ryu v1.0.23
2026-Mar-17 00:57:16.450888
#50 19.77    Compiling lock_api v0.4.14
2026-Mar-17 00:57:16.450888
#50 19.80    Compiling num-conv v0.2.0
2026-Mar-17 00:57:16.450888
#50 19.80    Compiling time-core v0.1.8
2026-Mar-17 00:57:16.450888
#50 19.83    Compiling stable_deref_trait v1.2.1
2026-Mar-17 00:57:16.450888
#50 19.85    Compiling generic-array v0.14.7
2026-Mar-17 00:57:16.450888
#50 19.91    Compiling tracing-core v0.1.36
2026-Mar-17 00:57:16.450888
#50 19.95    Compiling deranged v0.5.8
2026-Mar-17 00:57:16.450888
#50 20.07    Compiling crc32fast v1.5.0
2026-Mar-17 00:57:16.450888
#50 20.07    Compiling untrusted v0.9.0
2026-Mar-17 00:57:16.450888
#50 20.07    Compiling fs_extra v1.3.0
2026-Mar-17 00:57:16.450888
#50 20.07    Compiling dunce v1.0.5
2026-Mar-17 00:57:16.450888
#50 20.07    Compiling form_urlencoded v1.2.2
2026-Mar-17 00:57:16.450888
#50 20.12    Compiling rustls-pki-types v1.14.0
2026-Mar-17 00:57:16.450888
#50 20.23    Compiling litemap v0.8.1
2026-Mar-17 00:57:16.450888
#50 20.27    Compiling tower-service v0.3.3
2026-Mar-17 00:57:16.450888
#50 20.27    Compiling writeable v0.6.2
2026-Mar-17 00:57:16.450888
#50 20.27    Compiling aws-lc-rs v1.16.1
2026-Mar-17 00:57:16.450888
#50 20.27    Compiling zmij v1.0.21
2026-Mar-17 00:57:16.450888
#50 20.31    Compiling httparse v1.10.1
2026-Mar-17 00:57:16.450888
#50 20.34    Compiling num-traits v0.2.19
2026-Mar-17 00:57:16.450888
#50 20.39    Compiling vsimd v0.8.0
2026-Mar-17 00:57:16.450888
#50 20.44    Compiling time-macros v0.2.27
2026-Mar-17 00:57:16.450888
#50 20.44    Compiling outref v0.5.2
2026-Mar-17 00:57:16.450888
#50 20.54    Compiling try-lock v0.2.5
2026-Mar-17 00:57:16.450888
#50 20.57    Compiling rustls v0.23.37
2026-Mar-17 00:57:16.450888
#50 20.66    Compiling http v1.4.0
2026-Mar-17 00:57:16.450888
#50 20.66    Compiling http v0.2.12
2026-Mar-17 00:57:16.450888
#50 20.69    Compiling base64 v0.22.1
2026-Mar-17 00:57:16.450888
#50 20.71    Compiling httpdate v1.0.3
2026-Mar-17 00:57:16.450888
#50 20.71    Compiling atomic-waker v1.1.2
2026-Mar-17 00:57:16.450888
#50 20.72    Compiling want v0.3.1
2026-Mar-17 00:57:16.450888
#50 20.77    Compiling openssl-probe v0.2.1
2026-Mar-17 00:57:16.450888
#50 20.82    Compiling cpufeatures v0.2.17
2026-Mar-17 00:57:16.450888
#50 20.82    Compiling tower-layer v0.3.3
2026-Mar-17 00:57:16.450888
#50 20.94    Compiling utf8_iter v1.0.4
2026-Mar-17 00:57:16.450888
#50 21.01    Compiling sync_wrapper v1.0.2
2026-Mar-17 00:57:16.450888
#50 21.01    Compiling crossbeam-utils v0.8.21
2026-Mar-17 00:57:16.450888
#50 21.03    Compiling ipnet v2.11.0
2026-Mar-17 00:57:16.450888
#50 21.17    Compiling rustversion v1.0.22
2026-Mar-17 00:57:16.450888
#50 21.33    Compiling webpki-roots v1.0.6
2026-Mar-17 00:57:16.450888
#50 21.35    Compiling bitflags v2.11.0
2026-Mar-17 00:57:16.450888
#50 21.35    Compiling getrandom v0.4.1
2026-Mar-17 00:57:16.450888
#50 21.35    Compiling home v0.5.12
2026-Mar-17 00:57:16.450888
#50 21.38    Compiling base64-simd v0.8.0
2026-Mar-17 00:57:16.450888
#50 21.38    Compiling rustls-native-certs v0.8.3
2026-Mar-17 00:57:16.450888
#50 21.46    Compiling hex v0.4.3
2026-Mar-17 00:57:16.450888
#50 21.51    Compiling serde_json v1.0.149
2026-Mar-17 00:57:16.450888
#50 21.56    Compiling const-oid v0.9.6
2026-Mar-17 00:57:16.450888
#50 21.65    Compiling thiserror v2.0.18
2026-Mar-17 00:57:16.450888
#50 21.82    Compiling http-body v0.4.6
2026-Mar-17 00:57:16.450888
#50 21.83    Compiling syn v2.0.117
2026-Mar-17 00:57:16.450888
#50 21.99    Compiling http-body v1.0.1
2026-Mar-17 00:57:16.450888
#50 22.14    Compiling pkg-config v0.3.32
2026-Mar-17 00:57:16.450888
#50 22.16    Compiling der v0.6.1
2026-Mar-17 00:57:16.450888
#50 22.16    Compiling vcpkg v0.2.15
2026-Mar-17 00:57:16.450888
#50 22.16    Compiling allocator-api2 v0.2.21
2026-Mar-17 00:57:16.450888
#50 22.16    Compiling http-body-util v0.1.3
2026-Mar-17 00:57:16.450888
#50 22.17    Compiling num-integer v0.1.46
2026-Mar-17 00:57:16.450888
#50 22.17    Compiling time v0.3.47
2026-Mar-17 00:57:16.450888
#50 22.18    Compiling block-buffer v0.10.4
2026-Mar-17 00:57:16.450888
#50 22.18    Compiling crypto-common v0.1.7
2026-Mar-17 00:57:16.450888
#50 22.24    Compiling base64ct v1.8.3
2026-Mar-17 00:57:16.450888
#50 22.27    Compiling jobserver v0.1.34
2026-Mar-17 00:57:16.450888
#50 22.27    Compiling getrandom v0.2.17
2026-Mar-17 00:57:16.450888
#50 22.50    Compiling rustls v0.21.12
2026-Mar-17 00:57:16.450888
#50 22.68    Compiling errno v0.3.14
2026-Mar-17 00:57:16.450888
#50 22.68    Compiling mio v1.1.1
2026-Mar-17 00:57:16.450888
#50 22.68    Compiling socket2 v0.6.2
2026-Mar-17 00:57:16.450888
#50 22.68    Compiling digest v0.10.7
2026-Mar-17 00:57:16.450888
#50 22.68    Compiling socket2 v0.5.10
2026-Mar-17 00:57:16.450888
#50 22.74    Compiling glob v0.3.3
2026-Mar-17 00:57:16.450888
#50 22.82    Compiling signal-hook-registry v1.4.8
2026-Mar-17 00:57:16.450888
#50 22.86    Compiling cc v1.2.56
2026-Mar-17 00:57:16.450888
#50 22.88    Compiling rand_core v0.6.4
2026-Mar-17 00:57:16.450888
#50 22.92    Compiling foldhash v0.1.5
2026-Mar-17 00:57:16.450888
#50 22.95    Compiling base16ct v0.1.1
2026-Mar-17 00:57:16.450888
#50 22.97    Compiling rustix v0.38.44
2026-Mar-17 00:57:16.450888
#50 22.97    Compiling crc-catalog v2.4.0
2026-Mar-17 00:57:16.450888
#50 23.02    Compiling hmac v0.12.1
2026-Mar-17 00:57:16.450888
#50 23.08    Compiling sha2 v0.10.9
2026-Mar-17 00:57:16.450888
#50 23.08    Compiling uuid v1.21.0
2026-Mar-17 00:57:16.450888
#50 23.16    Compiling crc v3.4.0
2026-Mar-17 00:57:16.450888
#50 23.16    Compiling concurrent-queue v2.5.0
2026-Mar-17 00:57:16.450888
#50 23.20    Compiling parking v2.2.1
2026-Mar-17 00:57:16.450888
#50 23.27    Compiling hashbrown v0.15.5
2026-Mar-17 00:57:16.450888
#50 23.34    Compiling alloc-no-stdlib v2.0.4
2026-Mar-17 00:57:16.450888
#50 23.41    Compiling spki v0.6.0
2026-Mar-17 00:57:16.450888
#50 23.41    Compiling linux-raw-sys v0.4.15
2026-Mar-17 00:57:16.450888
#50 23.46    Compiling crypto-bigint v0.4.9
2026-Mar-17 00:57:16.450888
#50 23.49    Compiling ff v0.12.1
2026-Mar-17 00:57:16.450888
#50 23.58    Compiling clang-sys v1.8.1
2026-Mar-17 00:57:16.450888
#50 23.59    Compiling tinyvec_macros v0.1.1
2026-Mar-17 00:57:16.450888
#50 23.61    Compiling pkcs8 v0.9.0
2026-Mar-17 00:57:16.450888
#50 23.64    Compiling prettyplease v0.2.37
2026-Mar-17 00:57:16.450888
#50 23.74    Compiling indexmap v2.13.0
2026-Mar-17 00:57:16.450888
#50 23.86    Compiling group v0.12.1
2026-Mar-17 00:57:16.450888
#50 23.91    Compiling semver v1.0.27
2026-Mar-17 00:57:16.450888
#50 24.18    Compiling cmake v0.1.57
2026-Mar-17 00:57:16.450888
#50 24.25    Compiling tinyvec v1.10.0
2026-Mar-17 00:57:16.450888
#50 24.27    Compiling event-listener v5.4.1
2026-Mar-17 00:57:16.450888
#50 24.33    Compiling alloc-stdlib v0.2.2
2026-Mar-17 00:57:16.450888
#50 24.35    Compiling signature v1.6.4
2026-Mar-17 00:57:16.450888
#50 24.47    Compiling sec1 v0.3.0
2026-Mar-17 00:57:16.450888
#50 24.51    Compiling rustc_version v0.4.1
2026-Mar-17 00:57:16.450888
#50 24.57    Compiling tokio v1.49.0
2026-Mar-17 00:57:16.450888
#50 24.57    Compiling rfc6979 v0.3.1
2026-Mar-17 00:57:16.450888
#50 24.61    Compiling aho-corasick v1.1.4
2026-Mar-17 00:57:16.450888
#50 24.62    Compiling regex-syntax v0.8.10
2026-Mar-17 00:57:16.450888
#50 24.62    Compiling thiserror v1.0.69
2026-Mar-17 00:57:16.450888
#50 24.67    Compiling simd-adler32 v0.3.8
2026-Mar-17 00:57:16.450888
#50 24.69    Compiling openssl v0.10.75
2026-Mar-17 00:57:16.450888
#50 24.71    Compiling foreign-types-shared v0.1.1
2026-Mar-17 00:57:16.450888
#50 24.88    Compiling minimal-lexical v0.2.1
2026-Mar-17 00:57:16.450888
#50 24.88    Compiling adler2 v2.0.1
2026-Mar-17 00:57:16.450888
#50 25.00    Compiling elliptic-curve v0.12.3
2026-Mar-17 00:57:16.450888
#50 25.05    Compiling unicode-normalization v0.1.25
2026-Mar-17 00:57:16.450888
#50 25.09    Compiling futures-util v0.3.32
2026-Mar-17 00:57:16.450888
#50 25.09    Compiling foreign-types v0.3.2
2026-Mar-17 00:57:16.450888
#50 25.24    Compiling miniz_oxide v0.8.9
2026-Mar-17 00:57:16.450888
#50 25.24    Compiling hashlink v0.10.0
2026-Mar-17 00:57:16.450888
#50 25.25    Compiling nom v7.1.3
2026-Mar-17 00:57:16.450888
#50 25.29    Compiling aws-types v1.3.14
2026-Mar-17 00:57:16.450888
#50 25.29    Compiling ring v0.17.14
2026-Mar-17 00:57:16.450888
#50 25.32    Compiling aws-lc-sys v0.38.0
2026-Mar-17 00:57:16.450888
#50 25.32    Compiling openssl-sys v0.9.111
2026-Mar-17 00:57:16.450888
#50 25.32    Compiling webpki-roots v0.26.11
2026-Mar-17 00:57:16.450888
#50 25.32    Compiling brotli-decompressor v5.0.0
2026-Mar-17 00:57:16.450888
#50 25.35    Compiling crossbeam-queue v0.3.12
2026-Mar-17 00:57:16.450888
#50 25.46    Compiling md-5 v0.10.6
2026-Mar-17 00:57:16.450888
#50 25.60    Compiling ecdsa v0.14.8
2026-Mar-17 00:57:16.450888
#50 25.69    Compiling libloading v0.8.9
2026-Mar-17 00:57:16.450888
#50 25.69    Compiling native-tls v0.2.18
2026-Mar-17 00:57:16.450888
#50 25.73    Compiling unicode-bidi v0.3.18
2026-Mar-17 00:57:16.450888
#50 25.73    Compiling getrandom v0.3.4
2026-Mar-17 00:57:16.450888
#50 25.79    Compiling ppv-lite86 v0.2.21
2026-Mar-17 00:57:16.450888
#50 25.93    Compiling unicode-properties v0.1.4
2026-Mar-17 00:57:16.450888
#50 25.96    Compiling anyhow v1.0.102
2026-Mar-17 00:57:16.450888
#50 26.28    Compiling bindgen v0.69.5
2026-Mar-17 00:57:16.450888
#50 26.28    Compiling p256 v0.11.1
2026-Mar-17 00:57:16.450888
#50 26.34    Compiling rand_chacha v0.3.1
2026-Mar-17 00:57:16.450888
#50 26.44    Compiling flate2 v1.1.9
2026-Mar-17 00:57:16.450888
#50 26.53    Compiling stringprep v0.1.5
2026-Mar-17 00:57:16.450888
#50 26.70    Compiling regex-automata v0.4.14
2026-Mar-17 00:57:16.450888
#50 26.71    Compiling hkdf v0.12.4
2026-Mar-17 00:57:16.450888
#50 26.82    Compiling rand v0.8.5
2026-Mar-17 00:57:16.450888
#50 26.84    Compiling atoi v2.0.0
2026-Mar-17 00:57:16.450888
#50 27.03    Compiling crypto-bigint v0.5.5
2026-Mar-17 00:57:16.450888
#50 27.16    Compiling cookie v0.18.1
2026-Mar-17 00:57:16.450888
#50 27.26    Compiling synstructure v0.13.2
2026-Mar-17 00:57:16.450888
#50 27.26    Compiling adler v1.0.2
2026-Mar-17 00:57:16.450888
#50 27.26    Compiling fastrand v2.3.0
2026-Mar-17 00:57:16.450888
#50 27.32    Compiling compression-core v0.4.31
2026-Mar-17 00:57:16.450888
#50 27.32    Compiling dotenvy v0.15.7
2026-Mar-17 00:57:16.450888
#50 27.38    Compiling brotli v8.0.2
2026-Mar-17 00:57:16.450888
#50 27.47    Compiling rustc-hash v1.1.0
2026-Mar-17 00:57:16.450888
#50 27.47    Compiling lazy_static v1.5.0
2026-Mar-17 00:57:16.450888
#50 27.51    Compiling byteorder v1.5.0
2026-Mar-17 00:57:16.450888
#50 27.59    Compiling whoami v1.6.1
2026-Mar-17 00:57:16.450888
#50 27.61    Compiling lazycell v1.3.0
2026-Mar-17 00:57:16.450888
#50 27.65    Compiling miniz_oxide v0.7.4
2026-Mar-17 00:57:16.450888
#50 27.79    Compiling gzip-header v1.0.0
2026-Mar-17 00:57:16.450888
#50 27.99    Compiling cexpr v0.6.0
2026-Mar-17 00:57:16.450888
#50 27.99    Compiling num-bigint v0.4.6
2026-Mar-17 00:57:16.450888
#50 28.13    Compiling fslock v0.2.1
2026-Mar-17 00:57:16.450888
#50 28.20    Compiling ahash v0.8.12
2026-Mar-17 00:57:16.450888
#50 28.28    Compiling psl-types v2.0.11
2026-Mar-17 00:57:16.450888
#50 28.31    Compiling heck v0.5.0
2026-Mar-17 00:57:16.450888
#50 28.44    Compiling radium v0.7.0
2026-Mar-17 00:57:16.450888
#50 28.50    Compiling mime v0.3.17
2026-Mar-17 00:57:16.450888
#50 28.52    Compiling ucd-trie v0.1.7
2026-Mar-17 00:57:16.450888
#50 28.69    Compiling litrs v1.0.0
2026-Mar-17 00:57:16.450888
#50 28.69    Compiling paste v1.0.15
2026-Mar-17 00:57:16.450888
#50 28.74    Compiling rand_core v0.9.5
2026-Mar-17 00:57:16.450888
#50 28.82    Compiling pest v2.8.6
2026-Mar-17 00:57:16.450888
#50 28.86    Compiling arc-swap v1.8.2
2026-Mar-17 00:57:16.450888
#50 29.05    Compiling sha1_smol v1.0.1
2026-Mar-17 00:57:16.450888
#50 29.10    Compiling tap v1.0.1
2026-Mar-17 00:57:16.450888
#50 29.19    Compiling heck v0.4.1
2026-Mar-17 00:57:16.610663
#50 29.21    Compiling outref v0.1.0
2026-Mar-17 00:57:16.610663
#50 29.26    Compiling iri-string v0.7.10
2026-Mar-17 00:57:16.610663
#50 29.35    Compiling regex-lite v0.1.9
2026-Mar-17 00:57:16.722074
#50 ...
2026-Mar-17 00:57:16.722074
2026-Mar-17 00:57:16.722074
#48 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-17 00:57:16.722074
#48 28.35    Compiling icu_collections v2.1.1
2026-Mar-17 00:57:16.722074
#48 28.38    Compiling futures-executor v0.3.32
2026-Mar-17 00:57:16.722074
#48 28.39    Compiling async-stream v0.3.6
2026-Mar-17 00:57:16.722074
#48 28.55    Compiling convert_case v0.6.0
2026-Mar-17 00:57:16.722074
#48 28.55    Compiling rust-ini v0.20.0
2026-Mar-17 00:57:16.722074
#48 28.55    Compiling simple_asn1 v0.6.4
2026-Mar-17 00:57:16.722074
#48 28.62    Compiling pem v3.0.6
2026-Mar-17 00:57:16.963514
#48 28.82    Compiling futures v0.3.32
2026-Mar-17 00:57:16.963514
#48 28.82    Compiling tracing-log v0.2.0
2026-Mar-17 00:57:16.963514
#48 28.82    Compiling spinning_top v0.3.0
2026-Mar-17 00:57:16.963514
#48 28.86    Compiling sharded-slab v0.1.7
2026-Mar-17 00:57:17.197086
#48 29.00    Compiling thread_local v1.1.9
2026-Mar-17 00:57:17.197086
#48 29.02    Compiling futures-timer v3.0.3
2026-Mar-17 00:57:17.300047
#48 29.18    Compiling icu_provider v2.1.1
2026-Mar-17 00:57:17.300047
#48 29.20    Compiling yaml-rust2 v0.8.1
2026-Mar-17 00:57:17.404610
#48 29.22    Compiling pathdiff v0.2.3
2026-Mar-17 00:57:17.404610
#48 29.27    Compiling web-time v1.1.0
2026-Mar-17 00:57:17.404610
#48 29.30    Compiling utoipa-gen v4.3.1
2026-Mar-17 00:57:17.515044
#48 29.41    Compiling matchers v0.2.0
2026-Mar-17 00:57:17.626210
#48 29.44    Compiling no-std-compat v0.4.1
2026-Mar-17 00:57:17.626210
#48 29.52    Compiling rustls-webpki v0.103.9
2026-Mar-17 00:57:17.938777
#48 29.65    Compiling nu-ansi-term v0.50.3
2026-Mar-17 00:57:17.938777
#48 29.66    Compiling nonzero_ext v0.3.0
2026-Mar-17 00:57:17.938777
#48 29.72    Compiling quanta v0.12.6
2026-Mar-17 00:57:17.938777
#48 29.73    Compiling icu_properties v2.1.2
2026-Mar-17 00:57:19.047581
#48 30.89    Compiling smallvec v1.15.1
2026-Mar-17 00:57:19.164072
#48 30.95    Compiling either v1.15.0
2026-Mar-17 00:57:19.164072
#48 30.95    Compiling serde_urlencoded v0.7.1
2026-Mar-17 00:57:19.164072
#48 31.02    Compiling debugid v0.8.0
2026-Mar-17 00:57:19.164072
#48 31.04    Compiling bincode v1.3.3
2026-Mar-17 00:57:19.164072
#48 31.04    Compiling toml_datetime v0.6.11
2026-Mar-17 00:57:19.289108
#48 31.07    Compiling serde_spanned v0.6.9
2026-Mar-17 00:57:19.289108
#48 31.07    Compiling json5 v0.4.1
2026-Mar-17 00:57:19.289108
#48 31.19    Compiling icu_normalizer v2.1.1
2026-Mar-17 00:57:19.501567
#48 31.19    Compiling ron v0.8.1
2026-Mar-17 00:57:19.501567
#48 31.21    Compiling tracing-subscriber v0.3.22
2026-Mar-17 00:57:19.501567
#48 31.40    Compiling
2026-Mar-17 00:57:19.642583
bytes-utils v0.1.4
2026-Mar-17 00:57:19.642583
#48 31.40    Compiling itertools v0.13.0
2026-Mar-17 00:57:19.642583
#48 31.54    Compiling itertools v0.12.1
2026-Mar-17 00:57:19.767187
#48 31.54    Compiling which v4.4.2
2026-Mar-17 00:57:19.767187
#48 31.56    Compiling which v6.0.3
2026-Mar-17 00:57:19.767187
#48 31.63    Compiling toml_edit v0.22.27
2026-Mar-17 00:57:19.910574
#48 31.69    Compiling parking_lot v0.12.5
2026-Mar-17 00:57:19.910574
#48 31.81    Compiling dashmap v6.1.0
2026-Mar-17 00:57:20.138572
#48 31.89    Compiling idna_adapter v1.2.1
2026-Mar-17 00:57:20.296066
#48 32.10    Compiling futures-intrusive v0.5.0
2026-Mar-17 00:57:20.296066
#48 32.11    Compiling moka v0.12.13
2026-Mar-17 00:57:20.589566
#48 32.46    Compiling idna v1.1.0
2026-Mar-17 00:57:20.954266
#48 32.85    Compiling url v2.5.8
2026-Mar-17 00:57:21.077417
#48 32.98    Compiling publicsuffix v2.3.0
2026-Mar-17 00:57:21.359742
#48 33.20    Compiling governor v0.8.1
2026-Mar-17 00:57:21.944907
#48 33.84    Compiling cookie_store v0.22.1
2026-Mar-17 00:57:22.129952
#48 33.88    Compiling sourcemap v8.0.1
2026-Mar-17 00:57:22.683511
#48 34.58    Compiling toml v0.8.23
2026-Mar-17 00:57:23.235900
#48 35.13    Compiling rustls-webpki v0.101.7
2026-Mar-17 00:57:23.343064
#48 35.16    Compiling sct v0.7.1
2026-Mar-17 00:57:23.538040
#48 35.29    Compiling jsonwebtoken v9.3.1
2026-Mar-17 00:57:23.864240
#48 35.63    Compiling config v0.14.1
2026-Mar-17 00:57:24.286595
#48 36.03    Compiling sqlx-core v0.8.6
2026-Mar-17 00:57:24.618792
#48 ...
2026-Mar-17 00:57:24.618792
2026-Mar-17 00:57:24.618792
#50 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-17 00:57:24.618792
#50 29.48    Compiling simd-abstraction v0.7.1
2026-Mar-17 00:57:24.618792
#50 29.48    Compiling wyz v0.5.1
2026-Mar-17 00:57:24.618792
#50 29.60    Compiling rand_chacha v0.9.0
2026-Mar-17 00:57:24.618792
#50 29.63    Compiling document-features v0.2.12
2026-Mar-17 00:57:24.618792
#50 29.63    Compiling memoffset v0.9.1
2026-Mar-17 00:57:24.618792
#50 29.68    Compiling encoding_rs v0.8.35
2026-Mar-17 00:57:24.618792
#50 29.68    Compiling xmlparser v0.13.6
2026-Mar-17 00:57:24.618792
#50 29.81    Compiling funty v2.0.0
2026-Mar-17 00:57:24.618792
#50 29.89    Compiling regex v1.12.3
2026-Mar-17 00:57:24.618792
#50 29.93    Compiling portable-atomic v1.13.1
2026-Mar-17 00:57:24.618792
#50 29.94    Compiling base64-simd v0.7.0
2026-Mar-17 00:57:24.618792
#50 29.94    Compiling event-listener-strategy v0.5.4
2026-Mar-17 00:57:24.618792
#50 29.94    Compiling serde_path_to_error v0.1.20
2026-Mar-17 00:57:24.618792
#50 30.04    Compiling tokio-stream v0.1.18
2026-Mar-17 00:57:24.618792
#50 30.27    Compiling rand v0.9.2
2026-Mar-17 00:57:24.618792
#50 30.45    Compiling aws-smithy-xml v0.60.15
2026-Mar-17 00:57:24.618792
#50 30.49    Compiling hashbrown v0.14.5
2026-Mar-17 00:57:24.618792
#50 30.52    Compiling sha1 v0.10.6
2026-Mar-17 00:57:24.618792
#50 30.68    Compiling serde_derive v1.0.228
2026-Mar-17 00:57:24.618792
#50 30.68    Compiling tokio-macros v2.6.0
2026-Mar-17 00:57:24.618792
#50 30.68    Compiling zerofrom-derive v0.1.6
2026-Mar-17 00:57:24.618792
#50 30.88    Compiling yoke-derive v0.8.1
2026-Mar-17 00:57:24.618792
#50 30.99    Compiling tracing-attributes v0.1.31
2026-Mar-17 00:57:24.618792
#50 31.01    Compiling futures-macro v0.3.32
2026-Mar-17 00:57:24.618792
#50 31.02    Compiling zerovec-derive v0.11.2
2026-Mar-17 00:57:24.618792
#50 31.06    Compiling displaydoc v0.2.5
2026-Mar-17 00:57:24.618792
#50 31.16    Compiling thiserror-impl v2.0.18
2026-Mar-17 00:57:24.618792
#50 31.22    Compiling openssl-macros v0.1.1
2026-Mar-17 00:57:24.618792
#50 31.26    Compiling thiserror-impl v1.0.69
2026-Mar-17 00:57:24.618792
#50 31.28    Compiling async-trait v0.1.89
2026-Mar-17 00:57:24.618792
#50 31.31    Compiling pest_meta v2.8.6
2026-Mar-17 00:57:24.618792
#50 31.42    Compiling strum_macros v0.25.3
2026-Mar-17 00:57:24.618792
#50 31.45    Compiling proc-macro-rules-macros v0.4.0
2026-Mar-17 00:57:24.618792
#50 31.53    Compiling bitvec v1.0.1
2026-Mar-17 00:57:24.618792
#50 31.67    Compiling pin-project-internal v1.1.10
2026-Mar-17 00:57:24.618792
#50 31.79    Compiling compression-codecs v0.4.37
2026-Mar-17 00:57:24.618792
#50 31.99    Compiling matchit v0.8.4
2026-Mar-17 00:57:24.618792
#50 32.12    Compiling urlencoding v2.1.3
2026-Mar-17 00:57:24.618792
#50 32.51    Compiling if_chain v1.0.3
2026-Mar-17 00:57:24.618792
#50 32.52    Compiling bit-vec v0.6.3
2026-Mar-17 00:57:24.618792
#50 32.57    Compiling data-encoding v2.10.0
2026-Mar-17 00:57:24.618792
#50 32.57    Compiling unicode-id-start v1.4.0
2026-Mar-17 00:57:24.618792
#50 32.70    Compiling pest_generator v2.8.6
2026-Mar-17 00:57:24.618792
#50 32.92    Compiling bit-set v0.5.3
2026-Mar-17 00:57:24.618792
#50 32.95    Compiling async-lock v3.4.2
2026-Mar-17 00:57:24.618792
#50 33.03    Compiling crossbeam-epoch v0.9.18
2026-Mar-17 00:57:24.618792
#50 33.06    Compiling crossbeam-channel v0.5.15
2026-Mar-17 00:57:24.618792
#50 33.13    Compiling tagptr v0.2.0
2026-Mar-17 00:57:24.618792
#50 33.20    Compiling proc-macro-rules v0.4.0
2026-Mar-17 00:57:24.618792
#50 33.24    Compiling static_assertions v1.1.0
2026-Mar-17 00:57:24.618792
#50 33.33    Compiling deno_core_icudata v0.0.73
2026-Mar-17 00:57:24.618792
#50 33.42    Compiling cooked-waker v5.0.0
2026-Mar-17 00:57:24.618792
#50 33.51    Compiling zerofrom v0.1.6
2026-Mar-17 00:57:24.618792
#50 33.85    Compiling tracing v0.1.44
2026-Mar-17 00:57:24.618792
#50 33.97    Compiling yoke v0.8.1
2026-Mar-17 00:57:24.618792
#50 34.10    Compiling extractor v0.1.0 (/app/crates/extractor)
2026-Mar-17 00:57:24.618792
#50 34.14    Compiling async-stream-impl v0.3.6
2026-Mar-17 00:57:24.618792
#50 34.14    Compiling lru v0.12.5
2026-Mar-17 00:57:24.618792
#50 34.14    Compiling num_cpus v1.17.0
2026-Mar-17 00:57:24.618792
#50 34.21    Compiling axum-core v0.5.6
2026-Mar-17 00:57:24.618792
#50 34.26    Compiling zerotrie v0.2.3
2026-Mar-17 00:57:24.618792
#50 34.29    Compiling tower v0.4.13
2026-Mar-17 00:57:24.618792
#50 34.44    Compiling strum v0.25.0
2026-Mar-17 00:57:24.618792
#50 34.49    Compiling pest_derive v2.8.6
2026-Mar-17 00:57:24.618792
#50 34.53    Compiling tracing-log v0.2.0
2026-Mar-17 00:57:24.618792
#50 34.55    Compiling sharded-slab v0.1.7
2026-Mar-17 00:57:24.618792
#50 34.55    Compiling thread_local v1.1.9
2026-Mar-17 00:57:24.618792
#50 34.55    Compiling nu-ansi-term v0.50.3
2026-Mar-17 00:57:24.618792
#50 34.58    Compiling pin-project v1.1.10
2026-Mar-17 00:57:24.618792
#50 34.87    Compiling deno_ops v0.176.0
2026-Mar-17 00:57:24.618792
#50 34.90    Compiling zerovec v0.11.5
2026-Mar-17 00:57:24.618792
#50 34.98    Compiling async-stream v0.3.6
2026-Mar-17 00:57:24.618792
#50 35.46    Compiling tinystr v0.8.2
2026-Mar-17 00:57:24.618792
#50 35.46    Compiling potential_utf v0.1.4
2026-Mar-17 00:57:24.618792
#50 35.58    Compiling icu_collections v2.1.1
2026-Mar-17 00:57:24.618792
#50 35.70    Compiling icu_locale_core v2.1.1
2026-Mar-17 00:57:24.618792
#50 35.86    Compiling futures-executor v0.3.32
2026-Mar-17 00:57:24.618792
#50 36.00    Compiling smallvec v1.15.1
2026-Mar-17 00:57:24.618792
#50 36.02    Compiling either v1.15.0
2026-Mar-17 00:57:24.618792
#50 36.02    Compiling serde_urlencoded v0.7.1
2026-Mar-17 00:57:24.618792
#50 36.02    Compiling debugid v0.8.0
2026-Mar-17 00:57:24.618792
#50 36.04    Compiling bincode v1.3.3
2026-Mar-17 00:57:24.618792
#50 36.08    Compiling json5 v0.4.1
2026-Mar-17 00:57:24.618792
#50 36.41    Compiling bytes-utils v0.1.4
2026-Mar-17 00:57:24.618792
#50 36.60    Compiling itertools v0.13.0
2026-Mar-17 00:57:24.618792
#50 36.64    Compiling futures v0.3.32
2026-Mar-17 00:57:24.618792
#50 36.71    Compiling parking_lot v0.12.5
2026-Mar-17 00:57:24.618792
#50 36.71    Compiling dashmap v6.1.0
2026-Mar-17 00:57:24.618792
#50 36.85    Compiling itertools v0.12.1
2026-Mar-17 00:57:24.618792
#50 36.85    Compiling which v4.4.2
2026-Mar-17 00:57:24.618792
#50 36.85    Compiling which v6.0.3
2026-Mar-17 00:57:24.618792
#50 37.19    Compiling futures-intrusive v0.5.0
2026-Mar-17 00:57:24.618792
#50 37.23    Compiling moka v0.12.13
2026-Mar-17 00:57:24.618792
#50 37.36    Compiling icu_provider v2.1.1
2026-Mar-17 00:57:24.794621
#50 37.39    Compiling rustls-webpki v0.103.9
2026-Mar-17 00:57:24.817594
#50 37.56    Compiling matchers v0.2.0
2026-Mar-17 00:57:25.023410
#50 37.76    Compiling tracing-subscriber v0.3.22
2026-Mar-17 00:57:25.179937
#50 37.91    Compiling icu_properties v2.1.2
2026-Mar-17 00:57:25.179937
#50 37.91    Compiling icu_normalizer v2.1.1
2026-Mar-17 00:57:25.669670
#50 ...
2026-Mar-17 00:57:25.669670
2026-Mar-17 00:57:25.669670
#48 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-17 00:57:25.669670
#48 37.16    Compiling utoipa v4.2.3
2026-Mar-17 00:57:25.669670
#48 37.43    Compiling tokio-util v0.7.18
2026-Mar-17 00:57:25.669670
#48 37.43    Compiling aws-smithy-async v1.2.14
2026-Mar-17 00:57:25.669670
#48 37.43    Compiling tower v0.5.3
2026-Mar-17 00:57:25.669670
#48 37.43    Compiling async-compression v0.4.41
2026-Mar-17 00:57:25.669670
#48 37.57    Compiling tokio-native-tls v0.3.1
2026-Mar-17 00:57:25.885174
#48 37.64    Compiling deno_unsync v0.4.4
2026-Mar-17 00:57:26.360019
#48 38.26    Compiling aws-smithy-types v1.4.6
2026-Mar-17 00:57:26.490663
#48 38.26    Compiling h2 v0.4.13
2026-Mar-17 00:57:26.490663
#48 38.29    Compiling h2 v0.3.27
2026-Mar-17 00:57:26.490663
#48 38.39    Compiling combine v4.6.7
2026-Mar-17 00:57:26.641812
#48 38.39    Compiling tower-http v0.6.8
2026-Mar-17 00:57:26.826315
#48 38.72    Compiling tokio-rustls v0.24.1
2026-Mar-17 00:57:26.839614
2026-Mar-17 00:57:27.627754
#48 39.38    Compiling aws-smithy-runtime-api v1.11.6
2026-Mar-17 00:57:27.627754
#48 39.38    Compiling aws-smithy-eventstream v0.60.20
2026-Mar-17 00:57:27.681438
#48 39.58    Compiling aws-smithy-json v0.62.5
2026-Mar-17 00:57:27.872988
#48 39.58    Compiling aws-smithy-query v0.60.15
2026-Mar-17 00:57:27.872988
#48 39.58    Compiling aws-smithy-json v0.61.9
2026-Mar-17 00:57:28.253979
#48 40.00    Compiling sqlx-postgres v0.8.6
2026-Mar-17 00:57:28.852379
#48 ...
2026-Mar-17 00:57:28.852379
2026-Mar-17 00:57:28.852379
#52 [frontend builder 8/8] RUN node build-docker.mjs
2026-Mar-17 00:57:28.852379
#52 19.80 ✔ [paraglide-js] Compilation complete (message-modules)
2026-Mar-17 00:57:28.852379
#52 19.83 transforming...
2026-Mar-17 00:57:28.852379
#52 26.46 ✓ 826 modules transformed.
2026-Mar-17 00:57:28.852379
#52 26.86 rendering chunks...
2026-Mar-17 00:57:28.852379
#52 27.20 computing gzip size...
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/version.json                                                0.03 kB │ gzip:   0.05 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/assets/fredoka-latin-ext.CYrqKuxd.woff2           4.58 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/assets/nunito-normal-vietnamese.U01xdrZh.woff2   13.10 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/.vite/manifest.json                                             13.16 kB │ gzip:   1.74 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/assets/fredoka-latin.DM6njrJ3.woff2              29.73 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/assets/nunito-normal-latin.BzFMHfZw.woff2        39.13 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/assets/2.BCHMhk4V.css                             0.22 kB │ gzip:   0.17 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/assets/AppIcon.BlEdAg33.css                       0.47 kB │ gzip:   0.25 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/assets/11.B6JFksZy.css                            0.79 kB │ gzip:   0.43 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/assets/4.DvJYPIor.css                             1.18 kB │ gzip:   0.38 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/assets/12.CiKCOCFN.css                            2.91 kB │ gzip:   0.75 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/assets/3.BhQSW3U_.css                            17.70 kB │ gzip:   3.72 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/assets/0.trg5kHTH.css                            53.98 kB │ gzip:  10.19 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/entry/start.DA7i99xE.js                           0.08 kB │ gzip:   0.09 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/chunks/DG-T2NuS.js                                0.19 kB │ gzip:   0.17 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/chunks/CngS22Vo.js                                0.37 kB │ gzip:   0.28 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/chunks/BLi0WWPH.js                                0.37 kB │ gzip:   0.26 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/chunks/C5548wLj.js                                0.38 kB │ gzip:   0.27 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/chunks/Dq5kRN6d.js                                0.49 kB │ gzip:   0.32 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/chunks/DB1fBGMd.js                                0.54 kB │ gzip:   0.34 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/nodes/1.BPFvDvvb.js                               0.55 kB │ gzip:   0.35 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/chunks/DM4YYLpb.js                                0.64 kB │ gzip:   0.33 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/chunks/DtMvnJmg.js                                0.81 kB │ gzip:   0.46 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/chunks/CJctej1d.js                                1.13 kB │ gzip:   0.65 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/chunks/BQuzZvA0.js                                1.24 kB │ gzip:   0.61 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/chunks/D2gFui-Z.js                                1.37 kB │ gzip:   0.79 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/chunks/CEySqDN-.js                                2.05 kB │ gzip:   1.06 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/chunks/za0YHZJN.js                                2.59 kB │ gzip:   1.19 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/nodes/6.Cul1qqCx.js                               2.66 kB │ gzip:   1.22 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/chunks/CrozGAN7.js                                2.81 kB │ gzip:   0.64 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/chunks/vRDVPpR9.js                                3.11 kB │ gzip:   1.39 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/chunks/UxeC5sfV.js                                4.15 kB │ gzip:   1.94 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/nodes/7.BqAUA86T.js                               4.43 kB │ gzip:   1.27 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/chunks/Ce8dAbcZ.js                                4.83 kB │ gzip:   2.29 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/nodes/8.CyIdVoAr.js                               5.82 kB │ gzip:   1.98 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/nodes/9.CskTcTTN.js                               7.32 kB │ gzip:   2.48 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/nodes/2.MbRjBDSr.js                               7.45 kB │ gzip:   2.90 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/entry/app.-uR2Z1D3.js                             7.89 kB │ gzip:   3.05 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/chunks/CQATMoOd.js                               10.82 kB │ gzip:   4.57 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/nodes/10.BjsM-dxm.js                             12.66 kB │ gzip:   3.92 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/chunks/BIJYWjct.js                               25.09 kB │ gzip:   9.83 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/chunks/DmOCDnli.js                               25.18 kB │ gzip:   8.21 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/nodes/4.DvjHB7dk.js                              29.44 kB │ gzip:  11.57 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/chunks/CuHEigVt.js                               32.89 kB │ gzip:  12.68 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/chunks/B1SfeCS2.js                               33.13 kB │ gzip:  12.03 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/nodes/0.wHxyQdHL.js                              43.23 kB │ gzip:  10.44 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/chunks/BKmFtAEB.js                               70.17 kB │ gzip:  18.73 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/nodes/11.CkempTBl.js                             79.85 kB │ gzip:  11.00 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/chunks/1kHQH8jj.js                               92.94 kB │ gzip:  19.63 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/chunks/Qa-eO6nc.js                              133.02 kB │ gzip:   7.49 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/nodes/12.DLBNjvqF.js                            267.28 kB │ gzip: 104.84 kB
2026-Mar-17 00:57:28.852379
#52 27.22 .svelte-kit/output/client/_app/immutable/nodes/3.BV1yiX32.js                             368.48 kB │ gzip: 100.72 kB
2026-Mar-17 00:57:28.852379
#52 27.22 ✓ built in 18.40s
2026-Mar-17 00:57:28.852379
#52 28.36 vite v6.4.1 building for production...
2026-Mar-17 00:57:28.852379
#52 28.37 transforming...
2026-Mar-17 00:57:28.852379
#52 28.38 ✓ 2 modules transformed.
2026-Mar-17 00:57:28.852379
#52 28.39 rendering chunks...
2026-Mar-17 00:57:28.852379
#52 28.39 computing gzip size...
2026-Mar-17 00:57:28.852379
#52 28.39 .svelte-kit/output/client/service-worker.mjs  3.84 kB │ gzip: 1.33 kB
2026-Mar-17 00:57:28.852379
#52 28.39 ✓ built in 38ms
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/_app/immutable/assets/fredoka-latin-ext.CYrqKuxd.woff2                4.58 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/_app/immutable/assets/nunito-normal-vietnamese.U01xdrZh.woff2        13.10 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/.vite/manifest.json                                                  19.64 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/_app/immutable/assets/fredoka-latin.DM6njrJ3.woff2                   29.73 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/_app/immutable/assets/nunito-normal-latin.BzFMHfZw.woff2             39.13 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/_app/immutable/assets/_layout.BCHMhk4V.css                            0.22 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/_app/immutable/assets/AppIcon.BlEdAg33.css                            0.47 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/_app/immutable/assets/_page.B6JFksZy.css                              0.79 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/_app/immutable/assets/_page.DvJYPIor.css                              1.18 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/_app/immutable/assets/_page.CiKCOCFN.css                              2.91 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/_app/immutable/assets/_page.Dukwh2u_.css                             10.39 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/_app/immutable/assets/_layout.B_jkZyTO.css                           53.19 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/pages/_page.ts.js                                             0.05 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/false.js                                                       0.05 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/pages/(auth)/admin/_page.server.ts.js                         0.13 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/hooks.universal.js                                            0.15 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/pages/(auth)/admin/jobs/_page.server.ts.js                    0.18 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/index-server.js                                                0.18 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/pages/(auth)/admin/activity/_page.server.ts.js                0.19 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/endpoints/api/auth/_...all_/_server.ts.js                     0.23 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/endpoints/api/checkout/callback/_server.ts.js                 0.24 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/environment.js                                                 0.25 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/endpoints/share-target/_server.ts.js                          0.25 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/shared-server.js                                               0.28 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/endpoints/robots.txt/_server.ts.js                            0.30 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/endpoints/api/proxy/jobs/_jobId_/_server.ts.js                0.33 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/endpoints/api/proxy/jobs/_jobId_/release/_server.ts.js        0.35 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/pages/(auth)/admin/overview/_page.server.ts.js                0.36 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/pages/(auth)/admin/_layout.server.ts.js                       0.39 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/endpoints/api/proxy/jobs/_server.ts.js                        0.40 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/state.svelte.js                                                0.42 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/server.js                                                      0.45 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/internal.js                                                           0.46 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/pages/(auth)/account/_page.server.ts.js                       0.62 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/endpoints/api/checkout/_server.ts.js                          0.65 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/fallbacks/error.svelte.js                                     0.66 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/client.js                                                      0.68 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/stores.js                                                      0.73 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/endpoints/api/proxy/jobs/_jobId_/events/_server.ts.js         0.93 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/admin-access.js                                                1.00 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/endpoints/api/proxy/jobs/_jobId_/file-ticket/_server.ts.js    1.02 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/utils.js                                                       1.15 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/endpoints/api/proxy/jobs/_jobId_/file/_server.ts.js           1.50 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/endpoints/sitemap.xml/_server.ts.js                           1.61 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/AdminStatusBadge.js                                            1.76 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/pages/(auth)/admin/proxies/_page.server.ts.js                 2.01 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/exports.js                                                     2.39 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/index.js                                                       2.65 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/rust-api-proxy.js                                              2.93 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/pages/(auth)/admin/activity/_page.svelte.js                   3.01 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/url.js                                                         3.16 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/endpoints/api/proxy/extract/_server.ts.js                     3.27 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/auth-utils.js                                                  3.31 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/url2.js                                                        3.32 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/internal.js                                                    3.36 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/pages/(auth)/admin/capacity/_page.svelte.js                   4.22 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/bun-sqlite-dialect.js                                          4.29 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/node-sqlite-dialect.js                                         4.30 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/pages/(auth)/admin/jobs/_page.svelte.js                       6.02 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/hooks.server.js                                               6.74 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/pages/(auth)/admin/overview/_page.svelte.js                   9.02 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/query.js                                                       9.11 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/index3.js                                                      9.56 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/pages/(auth)/admin/_layout.svelte.js                          9.59 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/remote-entry.js                                                      10.47 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/pages/_layout.svelte.js                                      11.04 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/admin-dashboard.js                                            11.18 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/runtime.js                                                    13.79 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/pages/(auth)/admin/proxies/_page.svelte.js                   13.81 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/index4.js                                                     14.75 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/shared.js                                                     26.42 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/root.js                                                       29.25 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/pages/download/mux-job/_page.svelte.js                       41.15 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/playlist-download-stream-selection.js                         49.71 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/index2.js                                                     91.55 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/SiteHeader.js                                                109.56 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/index.js                                                            123.18 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/pages/(auth)/account/_page.svelte.js                        141.03 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/AuthModal.js                                                 154.58 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/AppIcon.js                                                   200.60 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/chunks/auth.js                                                      311.75 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/pages/_page.svelte.js                                       664.10 kB
2026-Mar-17 00:57:28.852379
#52 28.41 .svelte-kit/output/server/entries/pages/privacy/_page.svelte.js                               957.30 kB
2026-Mar-17 00:57:28.852379
#52 28.41 ✓ built in 27.31s
2026-Mar-17 00:57:28.852379
#52 28.41
2026-Mar-17 00:57:28.852379
#52 28.41 Run npm run preview to preview your production build locally.
2026-Mar-17 00:57:28.852379
#52 28.41
2026-Mar-17 00:57:28.852379
#52 28.41 > Using @sveltejs/adapter-node
2026-Mar-17 00:57:29.195712
#52 ...
2026-Mar-17 00:57:29.195712
2026-Mar-17 00:57:29.195712
#48 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-17 00:57:29.195712
#48 41.09    Compiling v8 v0.101.0
2026-Mar-17 00:57:30.377986
#48 42.27    Compiling aws-credential-types v1.2.14
2026-Mar-17 00:57:30.530884
#48 42.27    Compiling aws-smithy-http v0.63.6
2026-Mar-17 00:57:30.530884
#48 42.27    Compiling aws-smithy-observability v0.2.6
2026-Mar-17 00:57:30.530884
#48 42.27    Compiling aws-smithy-http v0.62.6
2026-Mar-17 00:57:30.919097
#48 42.82    Compiling aws-sigv4 v1.4.2
2026-Mar-17 00:57:31.618682
#48 43.52    Compiling
2026-Mar-17 00:57:31.719308
redis v0.27.6
2026-Mar-17 00:57:31.734033
2026-Mar-17 00:57:31.907658
#48 43.81    Compiling sqlx-macros-core v0.8.6
2026-Mar-17 00:57:32.116020
#48 44.02    Compiling
2026-Mar-17 00:57:32.299835
hyper v0.14.32
2026-Mar-17 00:57:32.655041
#48 44.41    Compiling hyper v1.8.1
2026-Mar-17 00:57:32.932605
#48 44.83    Compiling sqlx-macros v0.8.6
2026-Mar-17 00:57:33.146891
#48 44.90    Compiling crc-fast v1.6.0
2026-Mar-17 00:57:33.796162
#48 45.70    Compiling hyper-util v0.1.20
2026-Mar-17 00:57:35.012516
#48 ...
2026-Mar-17 00:57:35.012516
2026-Mar-17 00:57:35.012516
#52 [frontend builder 8/8] RUN node build-docker.mjs
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/zod/v4/classic/schemas.js -> node_modules/zod/v4/classic/iso.js -> node_modules/zod/v4/classic/schemas.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/binary-operation-parser.js -> node_modules/kysely/dist/esm/parser/reference-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/parser/reference-parser.js -> node_modules/kysely/dist/esm/parser/order-by-parser.js -> node_modules/kysely/dist/esm/dynamic/dynamic-reference-builder.js -> node_modules/kysely/dist/esm/parser/reference-parser.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/binary-operation-parser.js -> node_modules/kysely/dist/esm/parser/reference-parser.js -> node_modules/kysely/dist/esm/parser/order-by-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/parser/reference-parser.js -> node_modules/kysely/dist/esm/parser/order-by-parser.js -> node_modules/kysely/dist/esm/parser/reference-parser.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/binary-operation-parser.js -> node_modules/kysely/dist/esm/parser/value-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/operation-node/select-query-node.js -> node_modules/kysely/dist/esm/operation-node/query-node.js -> node_modules/kysely/dist/esm/operation-node/select-query-node.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/operation-node/query-node.js -> node_modules/kysely/dist/esm/operation-node/delete-query-node.js -> node_modules/kysely/dist/esm/operation-node/query-node.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/insert-query-builder.js -> node_modules/kysely/dist/esm/parser/select-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/parser/table-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/insert-query-builder.js -> node_modules/kysely/dist/esm/parser/select-parser.js -> node_modules/kysely/dist/esm/parser/table-parser.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/insert-query-builder.js -> node_modules/kysely/dist/esm/parser/select-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/insert-query-builder.js -> node_modules/kysely/dist/esm/parser/insert-values-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/insert-query-builder.js -> node_modules/kysely/dist/esm/parser/insert-values-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/insert-query-builder.js -> node_modules/kysely/dist/esm/parser/update-set-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/insert-query-builder.js -> node_modules/kysely/dist/esm/parser/expression-parser.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/delete-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/parser/table-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/delete-query-builder.js -> node_modules/kysely/dist/esm/parser/table-parser.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/update-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/parser/table-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/update-query-builder.js -> node_modules/kysely/dist/esm/parser/table-parser.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/parser/table-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/parser/table-parser.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/parser/with-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/query-creator.js -> node_modules/kysely/dist/esm/query-builder/merge-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/parser/table-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/parse-utils.js -> node_modules/kysely/dist/esm/parser/table-parser.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/parser/table-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/join-parser.js -> node_modules/kysely/dist/esm/parser/table-parser.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/parser/table-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/table-parser.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/group-by-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/set-operation-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/select-query-builder.js -> node_modules/kysely/dist/esm/parser/set-operation-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/parser/table-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/parser/table-parser.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/parser/table-parser.js -> node_modules/kysely/dist/esm/parser/expression-parser.js -> node_modules/kysely/dist/esm/expression/expression-builder.js -> node_modules/kysely/dist/esm/query-builder/function-module.js -> node_modules/kysely/dist/esm/parser/table-parser.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/kysely/dist/esm/parser/table-parser.js -> node_modules/kysely/dist/esm/dynamic/dynamic-table-builder.js -> node_modules/kysely/dist/esm/parser/table-parser.js
2026-Mar-17 00:57:35.012516
#52 32.69 Circular dependency: node_modules/@better-auth/core/dist/oauth2/validate-authorization-code.mjs -> node_modules/@better-auth/core/dist/oauth2/index.mjs -> node_modules/@better-auth/core/dist/oauth2/validate-authorization-code.mjs
2026-Mar-17 00:57:35.012516
#52 35.00   ✔ done
2026-Mar-17 00:57:35.012516
#52 DONE 36.0s
2026-Mar-17 00:57:35.012516
2026-Mar-17 00:57:35.012516
#50 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-17 00:57:35.012516
#50 39.63    Compiling idna_adapter v1.2.1
2026-Mar-17 00:57:35.012516
#50 39.75    Compiling idna v1.1.0
2026-Mar-17 00:57:35.012516
#50 40.46    Compiling url v2.5.8
2026-Mar-17 00:57:35.012516
#50 40.46    Compiling publicsuffix v2.3.0
2026-Mar-17 00:57:35.012516
#50 40.82    Compiling sct v0.7.1
2026-Mar-17 00:57:35.012516
#50 40.82    Compiling rustls-webpki v0.101.7
2026-Mar-17 00:57:35.012516
#50 41.75    Compiling cookie_store v0.22.1
2026-Mar-17 00:57:35.012516
#50 41.85    Compiling sourcemap v8.0.1
2026-Mar-17 00:57:35.012516
#50 43.82    Compiling sqlx-core v0.8.6
2026-Mar-17 00:57:35.012516
#50 44.55    Compiling v8 v0.101.0
2026-Mar-17 00:57:35.012516
#50 44.89    Compiling tokio-util v0.7.18
2026-Mar-17 00:57:35.012516
#50 44.89    Compiling aws-smithy-async v1.2.14
2026-Mar-17 00:57:35.012516
#50 44.89    Compiling tower v0.5.3
2026-Mar-17 00:57:35.012516
#50 44.89    Compiling tokio-native-tls v0.3.1
2026-Mar-17 00:57:35.012516
#50 44.89    Compiling async-compression v0.4.41
2026-Mar-17 00:57:35.012516
#50 44.89    Compiling deno_unsync v0.4.4
2026-Mar-17 00:57:35.012516
#50 45.21    Compiling tokio-rustls v0.24.1
2026-Mar-17 00:57:35.012516
#50 45.40    Compiling aws-smithy-types v1.4.6
2026-Mar-17 00:57:35.012516
#50 45.40    Compiling h2 v0.4.13
2026-Mar-17 00:57:35.012516
#50 45.40    Compiling h2 v0.3.27
2026-Mar-17 00:57:35.012516
#50 45.40    Compiling combine v4.6.7
2026-Mar-17 00:57:35.012516
#50 45.40    Compiling tower-http v0.6.8
2026-Mar-17 00:57:35.012516
#50 45.91    Compiling sqlx-postgres v0.8.6
2026-Mar-17 00:57:35.012516
#50 46.17    Compiling aws-smithy-runtime-api v1.11.6
2026-Mar-17 00:57:35.012516
#50 46.17    Compiling aws-smithy-eventstream v0.60.20
2026-Mar-17 00:57:35.012516
#50 46.17    Compiling aws-smithy-json v0.62.5
2026-Mar-17 00:57:35.012516
#50 46.17    Compiling aws-smithy-query v0.60.15
2026-Mar-17 00:57:35.012516
#50 46.17    Compiling aws-smithy-json v0.61.9
2026-Mar-17 00:57:35.012516
#50 47.17    Compiling aws-credential-types v1.2.14
2026-Mar-17 00:57:35.012516
#50 47.17    Compiling aws-smithy-http v0.63.6
2026-Mar-17 00:57:35.012516
#50 47.17    Compiling aws-smithy-observability v0.2.6
2026-Mar-17 00:57:35.012516
#50 47.17    Compiling aws-smithy-http v0.62.6
2026-Mar-17 00:57:35.012516
#50 47.46    Compiling aws-sigv4 v1.4.2
2026-Mar-17 00:57:35.296946
#50 ...
2026-Mar-17 00:57:35.296946
2026-Mar-17 00:57:35.296946
#48 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-17 00:57:35.296946
#48 47.20    Compiling hyper-tls v0.6.0
2026-Mar-17 00:57:35.296946
#48 47.20    Compiling axum v0.8.8
2026-Mar-17 00:57:35.405709
#48 47.31    Compiling serde_v8 v0.209.0
2026-Mar-17 00:57:35.639786
#48 47.34    Compiling aws-smithy-checksums v0.63.12
2026-Mar-17 00:57:35.781711
#48 47.68    Compiling hyper-rustls v0.24.2
2026-Mar-17 00:57:35.902455
#48 ...
2026-Mar-17 00:57:35.902455
2026-Mar-17 00:57:35.902455
#53 [frontend runtime 3/6] COPY --from=builder /app/build ./build
2026-Mar-17 00:57:35.902455
#53 DONE 0.4s
2026-Mar-17 00:57:35.902455
2026-Mar-17 00:57:35.902455
#48 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-17 00:57:35.902455
#48 47.70    Compiling deno_core v0.300.0
2026-Mar-17 00:57:36.019317
#48 ...
2026-Mar-17 00:57:36.019317
2026-Mar-17 00:57:36.019317
#54 [frontend runtime 4/6] COPY --from=builder /app/package.json ./
2026-Mar-17 00:57:36.019317
#54 DONE 0.1s
2026-Mar-17 00:57:36.019317
2026-Mar-17 00:57:36.019317
#55 [frontend runtime 5/6] COPY --from=builder /app/package-lock.json ./
2026-Mar-17 00:57:36.019317
#55 DONE 0.1s
2026-Mar-17 00:57:36.173942
#56 [frontend runtime 6/6] RUN npm ci --omit=dev
2026-Mar-17 00:57:40.586141
#56 4.563
2026-Mar-17 00:57:40.586141
#56 4.563 > frontend@0.0.1 prepare
2026-Mar-17 00:57:40.586141
#56 4.563 > svelte-kit sync || echo ''
2026-Mar-17 00:57:40.586141
#56 4.563
2026-Mar-17 00:57:40.815397
#56 4.612 Missing Svelte config file in /app — skipping
2026-Mar-17 00:57:40.815397
#56 4.624
2026-Mar-17 00:57:40.815397
#56 4.624 added 77 packages, and audited 78 packages in 4s
2026-Mar-17 00:57:40.815397
#56 4.624
2026-Mar-17 00:57:40.815397
#56 4.624 10 packages are looking for funding
2026-Mar-17 00:57:40.815397
#56 4.624   run `npm fund` for details
2026-Mar-17 00:57:40.815397
#56 4.641
2026-Mar-17 00:57:40.815397
#56 4.641 4 vulnerabilities (2 low, 2 moderate)
2026-Mar-17 00:57:40.815397
#56 4.641
2026-Mar-17 00:57:40.815397
#56 4.641 To address all issues, run:
2026-Mar-17 00:57:40.815397
#56 4.641   npm audit fix
2026-Mar-17 00:57:40.815397
#56 4.641
2026-Mar-17 00:57:40.815397
#56 4.641 Run `npm audit` for details.
2026-Mar-17 00:57:40.815397
#56 4.642 npm notice
2026-Mar-17 00:57:40.815397
#56 4.642 npm notice New major version of npm available! 10.9.4 -> 11.11.1
2026-Mar-17 00:57:40.815397
#56 4.642 npm notice Changelog: https://github.com/npm/cli/releases/tag/v11.11.1
2026-Mar-17 00:57:40.815397
#56 4.642 npm notice To update run: npm install -g npm@11.11.1
2026-Mar-17 00:57:40.815397
#56 4.642 npm notice
2026-Mar-17 00:57:40.993114
#56 DONE 5.0s
2026-Mar-17 00:57:40.993114
2026-Mar-17 00:57:40.993114
#50 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-17 00:57:40.993114
#50 48.64    Compiling hyper v0.14.32
2026-Mar-17 00:57:40.993114
#50 48.94    Compiling sqlx-macros-core v0.8.6
2026-Mar-17 00:57:40.993114
#50 49.07    Compiling hyper v1.8.1
2026-Mar-17 00:57:40.993114
#50 49.30    Compiling redis v0.27.6
2026-Mar-17 00:57:40.993114
#50 49.51    Compiling serde_v8 v0.209.0
2026-Mar-17 00:57:40.993114
#50 49.55    Compiling crc-fast v1.6.0
2026-Mar-17 00:57:40.993114
#50 49.78    Compiling sqlx-macros v0.8.6
2026-Mar-17 00:57:40.993114
#50 49.83    Compiling deno_core v0.300.0
2026-Mar-17 00:57:40.993114
#50 50.17    Compiling hyper-util v0.1.20
2026-Mar-17 00:57:40.993114
#50 51.31    Compiling hyper-tls v0.6.0
2026-Mar-17 00:57:40.993114
#50 51.31    Compiling axum v0.8.8
2026-Mar-17 00:57:40.993114
#50 51.41    Compiling aws-smithy-checksums v0.63.12
2026-Mar-17 00:57:40.993114
#50 51.54    Compiling hyper-rustls v0.24.2
2026-Mar-17 00:57:41.323556
#50 54.07    Compiling queue v0.1.0 (/app/crates/queue)
2026-Mar-17 00:57:43.460502
#50 ...
2026-Mar-17 00:57:43.460502
2026-Mar-17 00:57:43.460502
#57 [frontend] exporting to image
2026-Mar-17 00:57:43.460502
#57 exporting layers 2.3s done
2026-Mar-17 00:57:43.460502
#57 exporting manifest sha256:47b59c97bb558afe3fbd0d6c942a51227bb1f361132794d27103d27fe2472c87 done
2026-Mar-17 00:57:43.460502
#57 exporting config sha256:8159e00f6b81b4a82b5239ea60697b5f72d9700e5f6f95e522788dce45db6a21 done
2026-Mar-17 00:57:43.460502
#57 exporting attestation manifest sha256:b3da09cad37ad8f1130339ab6d13748c68857f3827e1cd51f59a5b12506fadd0 done
2026-Mar-17 00:57:43.460502
#57 exporting manifest list sha256:16cc03fe93238daf9faca20c058f45b865dfe2967d8e09c7986a5a0751c735cf done
2026-Mar-17 00:57:43.460502
#57 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:b412264047fb7da5cc4228d3f1ac389528038ad3 done
2026-Mar-17 00:57:43.460502
#57 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:b412264047fb7da5cc4228d3f1ac389528038ad3
2026-Mar-17 00:57:45.466247
#57 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:b412264047fb7da5cc4228d3f1ac389528038ad3 2.2s done
2026-Mar-17 00:57:45.567993
#57 DONE 4.5s
2026-Mar-17 00:57:45.567993
2026-Mar-17 00:57:45.567993
#58 [frontend] resolving provenance for metadata file
2026-Mar-17 00:57:45.567993
#58 DONE 0.0s
2026-Mar-17 00:57:45.567993
2026-Mar-17 00:57:45.567993
#48 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-17 00:57:45.567993
#48 49.26    Compiling queue v0.1.0 (/app/crates/queue)
2026-Mar-17 00:57:48.049674
#48 59.95    Compiling tokio-rustls v0.26.4
2026-Mar-17 00:57:48.218637
#48 60.12    Compiling hyper-rustls v0.27.7
2026-Mar-17 00:57:48.360373
#48 60.26    Compiling aws-smithy-http-client v1.1.12
2026-Mar-17 00:57:48.510826
#48 60.26    Compiling reqwest v0.12.28
2026-Mar-17 00:57:49.119530
#48 ...
2026-Mar-17 00:57:49.119530
2026-Mar-17 00:57:49.119530
#50 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-17 00:57:49.119530
#50 61.86    Compiling tokio-rustls v0.26.4
2026-Mar-17 00:57:49.295997
#50 62.04    Compiling hyper-rustls v0.27.7
2026-Mar-17 00:57:49.443368
#50 62.19    Compiling aws-smithy-http-client v1.1.12
2026-Mar-17 00:57:49.593829
#50 62.19    Compiling reqwest v0.12.28
2026-Mar-17 00:57:50.793338
#50 63.53    Compiling aws-smithy-runtime v1.10.3
2026-Mar-17 00:57:52.826814
#50 65.57    Compiling aws-runtime v1.7.2
2026-Mar-17 00:57:53.628184
#50 66.37    Compiling sqlx v0.8.6
2026-Mar-17 00:57:53.764499
#50 66.51    Compiling proxy v0.1.0 (/app/crates/proxy)
2026-Mar-17 00:57:53.939731
#50 66.51    Compiling job-system v0.1.0 (/app/crates/job-system)
2026-Mar-17 00:57:53.939731
#50 66.53    Compiling aws-sdk-sso v1.96.0
2026-Mar-17 00:57:53.939731
#50 66.53    Compiling aws-sdk-sts v1.100.0
2026-Mar-17 00:57:53.939731
#50 66.53    Compiling aws-sdk-ssooidc v1.98.0
2026-Mar-17 00:57:53.939731
#50 66.53    Compiling aws-sdk-s3 v1.119.0
2026-Mar-17 00:57:55.672183
#50 68.41    Compiling aws-config v1.8.15
2026-Mar-17 00:57:56.285257
#50 69.03    Compiling muxer v0.1.0 (/app/crates/muxer)
2026-Mar-17 00:57:59.735786
#50 ...
2026-Mar-17 00:57:59.735786
2026-Mar-17 00:57:59.735786
#48 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-17 00:57:59.735786
#48 61.58    Compiling aws-smithy-runtime v1.10.3
2026-Mar-17 00:57:59.735786
#48 63.66    Compiling aws-runtime v1.7.2
2026-Mar-17 00:57:59.735786
#48 64.24    Compiling sqlx v0.8.6
2026-Mar-17 00:57:59.735786
#48 64.34    Compiling proxy v0.1.0 (/app/crates/proxy)
2026-Mar-17 00:57:59.735786
#48 64.34    Compiling job-system v0.1.0 (/app/crates/job-system)
2026-Mar-17 00:57:59.735786
#48 64.63    Compiling aws-sdk-sso v1.96.0
2026-Mar-17 00:57:59.735786
#48 64.63    Compiling aws-sdk-ssooidc v1.98.0
2026-Mar-17 00:57:59.735786
#48 64.63    Compiling aws-sdk-sts v1.100.0
2026-Mar-17 00:57:59.735786
#48 64.63    Compiling aws-sdk-s3 v1.119.0
2026-Mar-17 00:57:59.735786
#48 66.47    Compiling aws-config v1.8.15
2026-Mar-17 00:57:59.735786
#48 66.80    Compiling muxer v0.1.0 (/app/crates/muxer)
2026-Mar-17 00:58:09.736739
#48 81.64    Compiling object-store v0.1.0 (/app/crates/object-store)
2026-Mar-17 00:58:10.769908
#48 ...
2026-Mar-17 00:58:10.769908
2026-Mar-17 00:58:10.769908
#50 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-17 00:58:10.769908
#50 83.51    Compiling object-store v0.1.0 (/app/crates/object-store)
2026-Mar-17 00:58:42.363110
#50 ...
2026-Mar-17 00:58:42.363110
2026-Mar-17 00:58:42.363110
#48 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-17 00:58:42.363110
#48 114.3    Compiling api v0.1.0 (/app/crates/api)
2026-Mar-17 00:58:51.773889
#48 ...
2026-Mar-17 00:58:51.773889
2026-Mar-17 00:58:51.773889
#50 [worker builder 10/10] RUN cargo build --release --bin mux-worker
2026-Mar-17 00:58:51.773889
#50 116.3    Compiling worker v0.1.0 (/app/crates/worker)
2026-Mar-17 01:00:16.226314
#50 209.0     Finished `release` profile [optimized] target(s) in 3m 28s
2026-Mar-17 01:00:16.434667
#50 DONE 209.2s
2026-Mar-17 01:00:16.434667
2026-Mar-17 01:00:16.434667
#48 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-17 01:00:16.548627
#48 ...
2026-Mar-17 01:00:16.548627
2026-Mar-17 01:00:16.548627
#59 [worker runtime 6/7] COPY --from=builder /app/target/release/mux-worker /usr/local/bin/
2026-Mar-17 01:00:16.740238
#59 DONE 0.0s
2026-Mar-17 01:00:16.740238
2026-Mar-17 01:00:16.740238
#60 [worker runtime 7/7] RUN mkdir -p /app/extractors /app/mux-artifacts /app/proxy-state && chown -R appuser:appuser /app
2026-Mar-17 01:00:16.740238
#60 DONE 0.1s
2026-Mar-17 01:00:16.740238
2026-Mar-17 01:00:16.740238
#48 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-17 01:00:16.892111
#48 ...
2026-Mar-17 01:00:16.892111
2026-Mar-17 01:00:16.892111
#61 [worker] exporting to image
2026-Mar-17 01:00:16.892111
#61 exporting layers
2026-Mar-17 01:00:18.443575
#61 exporting layers 1.7s done
2026-Mar-17 01:00:18.620704
#61 exporting manifest sha256:d11083e2568dbd6ad1758722e50bf5a4dc6cab3709c4246e57e853c511af2068 done
2026-Mar-17 01:00:18.620704
#61 exporting config sha256:8cc940a552d32194dda92f644520d25da06259441903db00b9f5e075a783a78b done
2026-Mar-17 01:00:18.620704
#61 exporting attestation manifest sha256:eda9633fb2e6d47df680a7bea948394e6d5dd58d511aad7e31958d53dab5b784 done
2026-Mar-17 01:00:18.620704
#61 exporting manifest list sha256:7889def4afa6f0ddbc0d25538c5f010b9e09e06833abe629958c38008a5b3e87 done
2026-Mar-17 01:00:18.620704
#61 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_worker:b412264047fb7da5cc4228d3f1ac389528038ad3 done
2026-Mar-17 01:00:18.620704
#61 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_worker:b412264047fb7da5cc4228d3f1ac389528038ad3
2026-Mar-17 01:00:19.361302
#61 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_worker:b412264047fb7da5cc4228d3f1ac389528038ad3 0.9s done
2026-Mar-17 01:00:19.588267
#61 DONE 2.6s
2026-Mar-17 01:00:19.588267
2026-Mar-17 01:00:19.588267
#62 [worker] resolving provenance for metadata file
2026-Mar-17 01:00:19.588267
#62 DONE 0.0s
2026-Mar-17 01:00:19.588267
2026-Mar-17 01:00:19.588267
#48 [api builder 11/11] RUN cargo build --release --bin api-server
2026-Mar-17 01:00:42.094890
#48 234.0     Finished `release` profile [optimized] target(s) in 3m 53s
2026-Mar-17 01:00:42.341448
#48 DONE 234.2s
2026-Mar-17 01:00:42.467344
#63 [api runtime 6/7] COPY --from=builder /app/target/release/api-server /usr/local/bin/
2026-Mar-17 01:00:42.786107
#63 DONE 0.3s
2026-Mar-17 01:00:42.928036
#64 [api runtime 7/7] RUN mkdir -p /app/extractors /app/data /app/proxy-state && chown -R appuser:appuser /app
2026-Mar-17 01:00:42.928036
#64 DONE 0.1s
2026-Mar-17 01:00:43.080877
#65 [api] exporting to image
2026-Mar-17 01:00:43.080877
#65 exporting layers
2026-Mar-17 01:00:45.026648
#65 exporting layers 2.1s done
2026-Mar-17 01:00:45.200334
#65 exporting manifest sha256:3660ea118bf16cdf3f53d603aa20fa369e02c1808d7bc51711d4db2410106108 done
2026-Mar-17 01:00:45.200334
#65 exporting config sha256:41445181f980176112b5d156a0c3f0997aa94a3043ab58ccc947f78c4c91e280 done
2026-Mar-17 01:00:45.200334
#65 exporting attestation manifest sha256:4e3ae7acd650f8d251ce7a370e0dc7acb94d85d3e080527944c3278cf471fe2f done
2026-Mar-17 01:00:45.200334
#65 exporting manifest list sha256:ac3207c874cab86f83c625ff9198142e9477ec1b6700aab0626e8840c6312550 done
2026-Mar-17 01:00:45.200334
#65 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_api:b412264047fb7da5cc4228d3f1ac389528038ad3 done
2026-Mar-17 01:00:45.200334
#65 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_api:b412264047fb7da5cc4228d3f1ac389528038ad3
2026-Mar-17 01:00:45.708182
#65 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_api:b412264047fb7da5cc4228d3f1ac389528038ad3 0.7s done
2026-Mar-17 01:00:45.880897
#65 DONE 2.8s
2026-Mar-17 01:00:45.880897
2026-Mar-17 01:00:45.880897
#66 [api] resolving provenance for metadata file
2026-Mar-17 01:00:45.880897
#66 DONE 0.0s
2026-Mar-17 01:00:45.893275
frontend  Built
2026-Mar-17 01:00:45.893275
api  Built
2026-Mar-17 01:00:45.893275
worker  Built
2026-Mar-17 01:00:45.954049
Creating .env file with runtime variables for container.
2026-Mar-17 01:00:46.196594
Removing old containers.
2026-Mar-17 01:00:46.418168
[CMD]: docker stop -t 30 frontend-o8kccgkgwsockoocow8sg88s-005001454761
2026-Mar-17 01:00:46.418168
frontend-o8kccgkgwsockoocow8sg88s-005001454761
2026-Mar-17 01:00:46.881406
[CMD]: docker rm -f frontend-o8kccgkgwsockoocow8sg88s-005001454761
2026-Mar-17 01:00:46.881406
frontend-o8kccgkgwsockoocow8sg88s-005001454761
2026-Mar-17 01:00:46.985381
[CMD]: docker stop -t 30 worker-o8kccgkgwsockoocow8sg88s-005001446397
2026-Mar-17 01:00:46.985381
worker-o8kccgkgwsockoocow8sg88s-005001446397
2026-Mar-17 01:00:47.119793
[CMD]: docker rm -f worker-o8kccgkgwsockoocow8sg88s-005001446397
2026-Mar-17 01:00:47.119793
worker-o8kccgkgwsockoocow8sg88s-005001446397
2026-Mar-17 01:00:47.223353
[CMD]: docker stop -t 30 api-o8kccgkgwsockoocow8sg88s-005001435837
2026-Mar-17 01:00:47.223353
api-o8kccgkgwsockoocow8sg88s-005001435837
2026-Mar-17 01:00:47.380128
[CMD]: docker rm -f api-o8kccgkgwsockoocow8sg88s-005001435837
2026-Mar-17 01:00:47.380128
api-o8kccgkgwsockoocow8sg88s-005001435837
2026-Mar-17 01:00:47.638218
[CMD]: docker stop -t 30 postgres-o8kccgkgwsockoocow8sg88s-005001426571
2026-Mar-17 01:00:47.638218
postgres-o8kccgkgwsockoocow8sg88s-005001426571
2026-Mar-17 01:00:47.764589
[CMD]: docker rm -f postgres-o8kccgkgwsockoocow8sg88s-005001426571
2026-Mar-17 01:00:47.764589
postgres-o8kccgkgwsockoocow8sg88s-005001426571
2026-Mar-17 01:00:47.867333
[CMD]: docker stop -t 30 shared-proxy-postgres-o8kccgkgwsockoocow8sg88s-005001416897
2026-Mar-17 01:00:47.867333
shared-proxy-postgres-o8kccgkgwsockoocow8sg88s-005001416897
2026-Mar-17 01:00:47.987492
[CMD]: docker rm -f shared-proxy-postgres-o8kccgkgwsockoocow8sg88s-005001416897
2026-Mar-17 01:00:47.987492
shared-proxy-postgres-o8kccgkgwsockoocow8sg88s-005001416897
2026-Mar-17 01:00:48.315007
[CMD]: docker stop -t 30 redis-o8kccgkgwsockoocow8sg88s-005001431518
2026-Mar-17 01:00:48.315007
redis-o8kccgkgwsockoocow8sg88s-005001431518
2026-Mar-17 01:00:48.440279
[CMD]: docker rm -f redis-o8kccgkgwsockoocow8sg88s-005001431518
2026-Mar-17 01:00:48.440279
redis-o8kccgkgwsockoocow8sg88s-005001431518
2026-Mar-17 01:00:48.543586
[CMD]: docker stop -t 30 shared-proxy-redis-o8kccgkgwsockoocow8sg88s-005001422263
2026-Mar-17 01:00:48.543586
shared-proxy-redis-o8kccgkgwsockoocow8sg88s-005001422263
2026-Mar-17 01:00:48.665538
[CMD]: docker rm -f shared-proxy-redis-o8kccgkgwsockoocow8sg88s-005001422263
2026-Mar-17 01:00:48.665538
shared-proxy-redis-o8kccgkgwsockoocow8sg88s-005001422263
2026-Mar-17 01:00:48.678276
Starting new application.
2026-Mar-17 01:00:49.133158
[CMD]: docker exec sosk0sgskc8sockk0gs8owso bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/sosk0sgskc8sockk0gs8owso/.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/sosk0sgskc8sockk0gs8owso -f /artifacts/sosk0sgskc8sockk0gs8owso/docker/docker-compose.server.yml up -d'
2026-Mar-17 01:00:49.133158
time="2026-03-17T01:00:49Z" level=warning msg="volume \"o8kccgkgwsockoocow8sg88s_postgres-data\" already exists but was not created by Docker Compose. Use `external: true` to use an existing volume"
2026-Mar-17 01:00:49.146268
Container shared-proxy-postgres-o8kccgkgwsockoocow8sg88s-005634289481  Creating
2026-Mar-17 01:00:49.146268
Container postgres-o8kccgkgwsockoocow8sg88s-005634299312  Creating
2026-Mar-17 01:00:49.146268
Container redis-o8kccgkgwsockoocow8sg88s-005634304068  Creating
2026-Mar-17 01:00:49.146268
Container shared-proxy-redis-o8kccgkgwsockoocow8sg88s-005634294930  Creating
2026-Mar-17 01:00:49.403806
Container shared-proxy-postgres-o8kccgkgwsockoocow8sg88s-005634289481  Created
2026-Mar-17 01:00:49.416773
Container redis-o8kccgkgwsockoocow8sg88s-005634304068  Created
2026-Mar-17 01:00:49.416773
Container shared-proxy-redis-o8kccgkgwsockoocow8sg88s-005634294930  Created
2026-Mar-17 01:00:49.416773
Container postgres-o8kccgkgwsockoocow8sg88s-005634299312  Created
2026-Mar-17 01:00:49.416773
Container worker-o8kccgkgwsockoocow8sg88s-005634318635  Creating
2026-Mar-17 01:00:49.416773
Container api-o8kccgkgwsockoocow8sg88s-005634308337  Creating
2026-Mar-17 01:00:49.439593
Container api-o8kccgkgwsockoocow8sg88s-005634308337  Created
2026-Mar-17 01:00:49.439593
Container frontend-o8kccgkgwsockoocow8sg88s-005634326676  Creating
2026-Mar-17 01:00:49.452362
Container worker-o8kccgkgwsockoocow8sg88s-005634318635  Created
2026-Mar-17 01:00:49.465976
Container frontend-o8kccgkgwsockoocow8sg88s-005634326676  Created
2026-Mar-17 01:00:49.480714
Container redis-o8kccgkgwsockoocow8sg88s-005634304068  Starting
2026-Mar-17 01:00:49.480714
Container shared-proxy-postgres-o8kccgkgwsockoocow8sg88s-005634289481  Starting
2026-Mar-17 01:00:49.480714
Container shared-proxy-redis-o8kccgkgwsockoocow8sg88s-005634294930  Starting
2026-Mar-17 01:00:49.494433
Container postgres-o8kccgkgwsockoocow8sg88s-005634299312  Starting
2026-Mar-17 01:00:49.625313
Container redis-o8kccgkgwsockoocow8sg88s-005634304068  Started
2026-Mar-17 01:00:49.638212
Container postgres-o8kccgkgwsockoocow8sg88s-005634299312  Started
2026-Mar-17 01:00:49.775886
Error response from daemon: failed to set up container networking: driver failed programming external connectivity on endpoint shared-proxy-postgres-o8kccgkgwsockoocow8sg88s-005634289481 (a6ac5d0b7891ab825fe31f87381aa6846c9fbdc2228c45fcad7d6cab994d72c7): Bind for 127.0.0.1:15432 failed: port is already allocated
2026-Mar-17 01:00:49.788712
exit status 1
2026-Mar-17 01:00:49.839735
========================================
2026-Mar-17 01:00:49.858210
Deployment failed: Command execution failed (exit code 1): docker exec sosk0sgskc8sockk0gs8owso bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/sosk0sgskc8sockk0gs8owso/.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/sosk0sgskc8sockk0gs8owso -f /artifacts/sosk0sgskc8sockk0gs8owso/docker/docker-compose.server.yml up -d'
2026-Mar-17 01:00:49.858210
Error: time="2026-03-17T01:00:49Z" level=warning msg="volume \"o8kccgkgwsockoocow8sg88s_postgres-data\" already exists but was not created by Docker Compose. Use `external: true` to use an existing volume"
2026-Mar-17 01:00:49.858210
Container shared-proxy-postgres-o8kccgkgwsockoocow8sg88s-005634289481  Creating
2026-Mar-17 01:00:49.858210
Container postgres-o8kccgkgwsockoocow8sg88s-005634299312  Creating
2026-Mar-17 01:00:49.858210
Container redis-o8kccgkgwsockoocow8sg88s-005634304068  Creating
2026-Mar-17 01:00:49.858210
Container shared-proxy-redis-o8kccgkgwsockoocow8sg88s-005634294930  Creating
2026-Mar-17 01:00:49.858210
Container shared-proxy-postgres-o8kccgkgwsockoocow8sg88s-005634289481  Created
2026-Mar-17 01:00:49.858210
Container redis-o8kccgkgwsockoocow8sg88s-005634304068  Created
2026-Mar-17 01:00:49.858210
Container shared-proxy-redis-o8kccgkgwsockoocow8sg88s-005634294930  Created
2026-Mar-17 01:00:49.858210
Container postgres-o8kccgkgwsockoocow8sg88s-005634299312  Created
2026-Mar-17 01:00:49.858210
Container worker-o8kccgkgwsockoocow8sg88s-005634318635  Creating
2026-Mar-17 01:00:49.858210
Container api-o8kccgkgwsockoocow8sg88s-005634308337  Creating
2026-Mar-17 01:00:49.858210
Container api-o8kccgkgwsockoocow8sg88s-005634308337  Created
2026-Mar-17 01:00:49.858210
Container frontend-o8kccgkgwsockoocow8sg88s-005634326676  Creating
2026-Mar-17 01:00:49.858210
Container worker-o8kccgkgwsockoocow8sg88s-005634318635  Created
2026-Mar-17 01:00:49.858210
Container frontend-o8kccgkgwsockoocow8sg88s-005634326676  Created
2026-Mar-17 01:00:49.858210
Container redis-o8kccgkgwsockoocow8sg88s-005634304068  Starting
2026-Mar-17 01:00:49.858210
Container shared-proxy-postgres-o8kccgkgwsockoocow8sg88s-005634289481  Starting
2026-Mar-17 01:00:49.858210
Container shared-proxy-redis-o8kccgkgwsockoocow8sg88s-005634294930  Starting
2026-Mar-17 01:00:49.858210
Container postgres-o8kccgkgwsockoocow8sg88s-005634299312  Starting
2026-Mar-17 01:00:49.858210
Container redis-o8kccgkgwsockoocow8sg88s-005634304068  Started
2026-Mar-17 01:00:49.858210
Container postgres-o8kccgkgwsockoocow8sg88s-005634299312  Started
2026-Mar-17 01:00:49.858210
Error response from daemon: failed to set up container networking: driver failed programming external connectivity on endpoint shared-proxy-postgres-o8kccgkgwsockoocow8sg88s-005634289481 (a6ac5d0b7891ab825fe31f87381aa6846c9fbdc2228c45fcad7d6cab994d72c7): Bind for 127.0.0.1:15432 failed: port is already allocated
2026-Mar-17 01:00:49.858210
exit status 1
2026-Mar-17 01:00:49.876618
Error type: RuntimeException
2026-Mar-17 01:00:49.894114
Error code: 0
2026-Mar-17 01:00:49.912171
Location: /var/www/html/app/Traits/ExecuteRemoteCommand.php:243
2026-Mar-17 01:00:49.929686
Stack trace (first 5 lines):
2026-Mar-17 01:00:49.947672
#0 /var/www/html/app/Traits/ExecuteRemoteCommand.php(104): App\Jobs\ApplicationDeploymentJob->executeCommandWithProcess()
2026-Mar-17 01:00:49.965317
#1 /var/www/html/vendor/laravel/framework/src/Illuminate/Collections/Traits/EnumeratesValues.php(272): App\Jobs\ApplicationDeploymentJob->{closure:App\Traits\ExecuteRemoteCommand::execute_remote_command():71}()
2026-Mar-17 01:00:49.982738
#2 /var/www/html/app/Traits/ExecuteRemoteCommand.php(71): Illuminate\Support\Collection->each()
2026-Mar-17 01:00:50.000045
#3 /var/www/html/app/Jobs/ApplicationDeploymentJob.php(816): App\Jobs\ApplicationDeploymentJob->execute_remote_command()
2026-Mar-17 01:00:50.017428
#4 /var/www/html/app/Jobs/ApplicationDeploymentJob.php(467): App\Jobs\ApplicationDeploymentJob->deploy_docker_compose_buildpack()
2026-Mar-17 01:00:50.035064
========================================
2026-Mar-17 01:00:50.310281
Gracefully shutting down build container: sosk0sgskc8sockk0gs8owso
2026-Mar-17 01:00:50.682583
[CMD]: docker stop -t 30 sosk0sgskc8sockk0gs8owso
2026-Mar-17 01:00:50.682583
sosk0sgskc8sockk0gs8owso