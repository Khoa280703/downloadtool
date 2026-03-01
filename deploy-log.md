2026-Mar-01 04:41:49.740232
Starting deployment of khoa280703/downloadtool:main-zoscg4oc04gkwkssg0kw8w8w to localhost.
2026-Mar-01 04:41:50.311508
Preparing container with helper image: ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Mar-01 04:41:50.614815
[CMD]: docker stop -t 30 ywscgkkkkwkc4o8w8c40cogc
2026-Mar-01 04:41:50.614815
Error response from daemon: No such container: ywscgkkkkwkc4o8w8c40cogc
2026-Mar-01 04:41:50.937083
[CMD]: docker run -d --network coolify --name ywscgkkkkwkc4o8w8c40cogc  --rm -v /var/run/docker.sock:/var/run/docker.sock ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Mar-01 04:41:50.937083
704d67d93374e15bc83bdb945221f0a1b3fdcc13a6e0118c3c84e47cee095d2e
2026-Mar-01 04:41:52.561922
[CMD]: docker exec ywscgkkkkwkc4o8w8c40cogc bash -c 'GIT_SSH_COMMAND="ssh -o ConnectTimeout=30 -p 22 -o Port=22 -o LogLevel=ERROR -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git ls-remote https://github.com/Khoa280703/downloadtool refs/heads/main'
2026-Mar-01 04:41:52.561922
dcc856454f8e5fcafa7c6df17c1aae344d490065	refs/heads/main
2026-Mar-01 04:41:52.572018
----------------------------------------
2026-Mar-01 04:41:52.575709
Importing Khoa280703/downloadtool:main (commit sha dcc856454f8e5fcafa7c6df17c1aae344d490065) to /artifacts/ywscgkkkkwkc4o8w8c40cogc.
2026-Mar-01 04:41:52.911010
[CMD]: docker exec ywscgkkkkwkc4o8w8c40cogc bash -c 'git clone --depth=1 --recurse-submodules --shallow-submodules -b 'main' 'https://github.com/Khoa280703/downloadtool' '/artifacts/ywscgkkkkwkc4o8w8c40cogc' && cd '/artifacts/ywscgkkkkwkc4o8w8c40cogc' && if [ -f .gitmodules ]; then sed -i "s#git@\(.*\):#https://\1/#g" '/artifacts/ywscgkkkkwkc4o8w8c40cogc'/.gitmodules || true && git submodule sync && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git submodule update --init --recursive --depth=1; fi && cd '/artifacts/ywscgkkkkwkc4o8w8c40cogc' && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git lfs pull'
2026-Mar-01 04:41:52.911010
Cloning into '/artifacts/ywscgkkkkwkc4o8w8c40cogc'...
2026-Mar-01 04:41:55.801285
[CMD]: docker exec ywscgkkkkwkc4o8w8c40cogc bash -c 'cd /artifacts/ywscgkkkkwkc4o8w8c40cogc && git log -1 dcc856454f8e5fcafa7c6df17c1aae344d490065 --pretty=%B'
2026-Mar-01 04:41:55.801285
perf(vite): pre-compile SSR modules on dev server start via ssrFiles warmup
2026-Mar-01 04:42:00.630692
[CMD]: docker exec ywscgkkkkwkc4o8w8c40cogc bash -c 'test -f /artifacts/ywscgkkkkwkc4o8w8c40cogc/docker/Dockerfile.api && echo 'exists' || echo 'not found''
2026-Mar-01 04:42:00.630692
exists
2026-Mar-01 04:42:01.007639
[CMD]: docker exec ywscgkkkkwkc4o8w8c40cogc bash -c 'cat /artifacts/ywscgkkkkwkc4o8w8c40cogc/docker/Dockerfile.api'
2026-Mar-01 04:42:01.007639
# Dockerfile for API service deployment
2026-Mar-01 04:42:01.007639
# Builds the API server and related components without GPU support
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
# Stage 0: Build injector JS (embedded into api crate via include_str! at compile time)
2026-Mar-01 04:42:01.007639
FROM node:22-alpine AS js-builder
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
WORKDIR /app
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
RUN npm install -g pnpm
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
# Copy workspace manifests for pnpm resolution
2026-Mar-01 04:42:01.007639
COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Mar-01 04:42:01.007639
COPY packages/api-client/package.json ./packages/api-client/
2026-Mar-01 04:42:01.007639
COPY apps/injector/package.json ./apps/injector/
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
# Copy injector source and shared packages
2026-Mar-01 04:42:01.007639
COPY apps/injector/ ./apps/injector/
2026-Mar-01 04:42:01.007639
COPY packages/ ./packages/
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
# Install deps and build injector (produces dist/bm.js and dist/youtube-downloader.user.js)
2026-Mar-01 04:42:01.007639
RUN pnpm install --frozen-lockfile
2026-Mar-01 04:42:01.007639
RUN pnpm --filter @downloadtool/injector build
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
# Build extractor TypeScript to IIFE format (required by crates/extractor/build.rs)
2026-Mar-01 04:42:01.007639
COPY extractors/ ./extractors/
2026-Mar-01 04:42:01.007639
RUN mkdir -p extractors/dist && \
2026-Mar-01 04:42:01.007639
npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js && \
2026-Mar-01 04:42:01.007639
npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
# Stage 1: Rust builder
2026-Mar-01 04:42:01.007639
FROM rust:1.88-bookworm AS builder
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
WORKDIR /app
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
# Install dependencies
2026-Mar-01 04:42:01.007639
RUN apt-get update && apt-get install -y \
2026-Mar-01 04:42:01.007639
pkg-config \
2026-Mar-01 04:42:01.007639
libssl-dev \
2026-Mar-01 04:42:01.007639
&& rm -rf /var/lib/apt/lists/*
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
# Copy workspace configuration
2026-Mar-01 04:42:01.007639
COPY Cargo.toml ./
2026-Mar-01 04:42:01.007639
COPY Cargo.lock ./
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
# Copy all crates
2026-Mar-01 04:42:01.007639
COPY crates/ ./crates/
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
# Copy injector dist (required by include_str! in crates/api/src/routes/static_files.rs)
2026-Mar-01 04:42:01.007639
COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
# Copy extractor source + pre-built IIFE dist (built by js-builder stage)
2026-Mar-01 04:42:01.007639
COPY extractors/ ./extractors/
2026-Mar-01 04:42:01.007639
COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
# Build the release binary
2026-Mar-01 04:42:01.007639
RUN cargo build --release --bin api-server
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
# Stage 2: Runtime
2026-Mar-01 04:42:01.007639
FROM debian:bookworm-slim AS runtime
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
WORKDIR /app
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
# Install runtime dependencies
2026-Mar-01 04:42:01.007639
RUN apt-get update && apt-get install -y \
2026-Mar-01 04:42:01.007639
ca-certificates \
2026-Mar-01 04:42:01.007639
curl \
2026-Mar-01 04:42:01.007639
libssl3 \
2026-Mar-01 04:42:01.007639
&& rm -rf /var/lib/apt/lists/*
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
# Install latest yt-dlp binary (newer than Debian package).
2026-Mar-01 04:42:01.007639
RUN set -eux; \
2026-Mar-01 04:42:01.007639
arch="$(dpkg --print-architecture)"; \
2026-Mar-01 04:42:01.007639
case "$arch" in \
2026-Mar-01 04:42:01.007639
amd64) ytdlp_asset="yt-dlp_linux" ;; \
2026-Mar-01 04:42:01.007639
arm64) ytdlp_asset="yt-dlp_linux_aarch64" ;; \
2026-Mar-01 04:42:01.007639
*) echo "Unsupported architecture: $arch" >&2; exit 1 ;; \
2026-Mar-01 04:42:01.007639
esac; \
2026-Mar-01 04:42:01.007639
curl -fL "https://github.com/yt-dlp/yt-dlp/releases/latest/download/${ytdlp_asset}" -o /usr/local/bin/yt-dlp; \
2026-Mar-01 04:42:01.007639
chmod +x /usr/local/bin/yt-dlp; \
2026-Mar-01 04:42:01.007639
/usr/local/bin/yt-dlp --version
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
# Create non-root user
2026-Mar-01 04:42:01.007639
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
# Copy binary from builder
2026-Mar-01 04:42:01.007639
COPY --from=builder /app/target/release/api-server /usr/local/bin/
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
# Create directories
2026-Mar-01 04:42:01.007639
RUN mkdir -p /app/extractors /app/data && chown -R appuser:appuser /app
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
# Switch to non-root user
2026-Mar-01 04:42:01.007639
USER appuser
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
# Environment variables
2026-Mar-01 04:42:01.007639
ENV PORT=3068
2026-Mar-01 04:42:01.007639
ENV EXTRACTOR_DIR=/app/extractors
2026-Mar-01 04:42:01.007639
ENV YTDLP_PATH=/usr/local/bin/yt-dlp
2026-Mar-01 04:42:01.007639
ENV RUST_LOG=info
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
# Expose port
2026-Mar-01 04:42:01.007639
EXPOSE 3068
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
# Health check
2026-Mar-01 04:42:01.007639
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Mar-01 04:42:01.007639
CMD curl -f http://localhost:3068/health || exit 1
2026-Mar-01 04:42:01.007639
2026-Mar-01 04:42:01.007639
# Run the server
2026-Mar-01 04:42:01.007639
CMD ["api-server"]
2026-Mar-01 04:42:01.359114
Added 42 ARG declarations to Dockerfile for service api (multi-stage build, added to 3 stages).
2026-Mar-01 04:42:01.705726
[CMD]: docker exec ywscgkkkkwkc4o8w8c40cogc bash -c 'test -f /artifacts/ywscgkkkkwkc4o8w8c40cogc/docker/Dockerfile.frontend && echo 'exists' || echo 'not found''
2026-Mar-01 04:42:01.705726
exists
2026-Mar-01 04:42:02.063387
[CMD]: docker exec ywscgkkkkwkc4o8w8c40cogc bash -c 'cat /artifacts/ywscgkkkkwkc4o8w8c40cogc/docker/Dockerfile.frontend'
2026-Mar-01 04:42:02.063387
# Dockerfile for frontend (SvelteKit Node server)
2026-Mar-01 04:42:02.063387
# Copy ALL source files BEFORE npm install so svelte-kit sync (prepare script)
2026-Mar-01 04:42:02.063387
# can find svelte.config.js and generate .svelte-kit/ correctly.
2026-Mar-01 04:42:02.063387
2026-Mar-01 04:42:02.063387
FROM node:22-alpine AS builder
2026-Mar-01 04:42:02.063387
2026-Mar-01 04:42:02.063387
WORKDIR /app
2026-Mar-01 04:42:02.063387
2026-Mar-01 04:42:02.063387
# Copy all frontend source files first (node_modules excluded via .dockerignore)
2026-Mar-01 04:42:02.063387
COPY frontend/ ./
2026-Mar-01 04:42:02.063387
2026-Mar-01 04:42:02.063387
# Install — prepare script runs svelte-kit sync with svelte.config.js available
2026-Mar-01 04:42:02.063387
RUN npm install
2026-Mar-01 04:42:02.063387
2026-Mar-01 04:42:02.063387
# Build-time public API URL (embedded into client bundle by Vite)
2026-Mar-01 04:42:02.063387
# Runtime env is too late for import.meta.env in browser bundle.
2026-Mar-01 04:42:02.063387
ARG VITE_API_URL
2026-Mar-01 04:42:02.063387
ENV VITE_API_URL=${VITE_API_URL}
2026-Mar-01 04:42:02.063387
RUN test -n "$VITE_API_URL" || (echo "VITE_API_URL build arg is required" && exit 1)
2026-Mar-01 04:42:02.063387
2026-Mar-01 04:42:02.063387
# Build via programmatic API — bypasses vite.config.ts loading (esbuild issues on Alpine)
2026-Mar-01 04:42:02.063387
RUN node build-docker.mjs
2026-Mar-01 04:42:02.063387
2026-Mar-01 04:42:02.063387
# Runtime
2026-Mar-01 04:42:02.063387
FROM node:22-alpine AS runtime
2026-Mar-01 04:42:02.063387
2026-Mar-01 04:42:02.063387
WORKDIR /app
2026-Mar-01 04:42:02.063387
2026-Mar-01 04:42:02.063387
COPY --from=builder /app/build ./build
2026-Mar-01 04:42:02.063387
COPY --from=builder /app/package.json ./
2026-Mar-01 04:42:02.063387
COPY --from=builder /app/package-lock.json ./
2026-Mar-01 04:42:02.063387
2026-Mar-01 04:42:02.063387
# Runtime needs server-side deps (better-auth, pg) used by hooks/routes
2026-Mar-01 04:42:02.063387
RUN npm ci --omit=dev
2026-Mar-01 04:42:02.063387
2026-Mar-01 04:42:02.063387
ENV PORT=5168
2026-Mar-01 04:42:02.063387
ENV HOST=0.0.0.0
2026-Mar-01 04:42:02.063387
2026-Mar-01 04:42:02.063387
EXPOSE 5168
2026-Mar-01 04:42:02.063387
2026-Mar-01 04:42:02.063387
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Mar-01 04:42:02.063387
CMD wget -qO- http://127.0.0.1:5168 || exit 1
2026-Mar-01 04:42:02.063387
2026-Mar-01 04:42:02.063387
CMD ["node", "build"]
2026-Mar-01 04:42:02.455164
Added 28 ARG declarations to Dockerfile for service frontend (multi-stage build, added to 2 stages).
2026-Mar-01 04:42:02.458799
Pulling & building required images.
2026-Mar-01 04:42:02.476019
Creating build-time .env file in /artifacts (outside Docker context).
2026-Mar-01 04:42:02.832257
Adding build arguments to Docker Compose build command.
2026-Mar-01 04:42:03.306062
[CMD]: docker exec ywscgkkkkwkc4o8w8c40cogc bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/build-time.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/ywscgkkkkwkc4o8w8c40cogc -f /artifacts/ywscgkkkkwkc4o8w8c40cogc/docker/docker-compose.server.yml build --pull --build-arg COOLIFY_URL --build-arg COOLIFY_FQDN --build-arg SERVICE_URL_API --build-arg SERVICE_FQDN_FRONTEND --build-arg SERVICE_FQDN_API --build-arg POSTGRES_PASSWORD --build-arg SERVICE_URL_FRONTEND --build-arg BETTER_AUTH_SECRET --build-arg WHOP_WEBHOOK_SECRET --build-arg ORIGIN --build-arg BETTER_AUTH_TRUSTED_ORIGINS --build-arg GOOGLE_CLIENT_ID --build-arg GOOGLE_CLIENT_SECRET --build-arg WHOP_PLAN_ID --build-arg COOLIFY_BUILD_SECRETS_HASH=a21afc02cd414908989fcb2d54f5c09f1d5a4fa0c1e8a2e802b2b03e2c30bacd'
2026-Mar-01 04:42:03.306062
#1 [internal] load local bake definitions
2026-Mar-01 04:42:03.507013
#1 reading from stdin 2.34kB done
2026-Mar-01 04:42:03.507013
#1 DONE 0.0s
2026-Mar-01 04:42:03.507013
2026-Mar-01 04:42:03.507013
#2 [frontend internal] load build definition from Dockerfile.frontend
2026-Mar-01 04:42:03.507013
#2 transferring dockerfile: 1.94kB done
2026-Mar-01 04:42:03.507013
#2 DONE 0.0s
2026-Mar-01 04:42:03.507013
2026-Mar-01 04:42:03.507013
#3 [api internal] load build definition from Dockerfile.api
2026-Mar-01 04:42:03.507013
#3 transferring dockerfile: 4.27kB done
2026-Mar-01 04:42:03.507013
#3 DONE 0.0s
2026-Mar-01 04:42:03.507013
2026-Mar-01 04:42:03.507013
#4 [api internal] load metadata for docker.io/library/node:22-alpine
2026-Mar-01 04:42:04.118752
#4 ...
2026-Mar-01 04:42:04.118752
2026-Mar-01 04:42:04.118752
#5 [api internal] load metadata for docker.io/library/rust:1.88-bookworm
2026-Mar-01 04:42:04.118752
#5 DONE 0.8s
2026-Mar-01 04:42:04.222332
#4 [api internal] load metadata for docker.io/library/node:22-alpine
2026-Mar-01 04:42:04.222332
#4 DONE 0.8s
2026-Mar-01 04:42:04.222332
2026-Mar-01 04:42:04.222332
#6 [api internal] load metadata for docker.io/library/debian:bookworm-slim
2026-Mar-01 04:42:04.222332
#6 DONE 0.8s
2026-Mar-01 04:42:04.222332
2026-Mar-01 04:42:04.222332
#7 [api internal] load .dockerignore
2026-Mar-01 04:42:04.222332
#7 transferring context: 341B done
2026-Mar-01 04:42:04.222332
#7 DONE 0.0s
2026-Mar-01 04:42:04.222332
2026-Mar-01 04:42:04.222332
#8 [api builder 1/6] FROM docker.io/library/node:22-alpine@sha256:e4bf2a82ad0a4037d28035ae71529873c069b13eb0455466ae0bc13363826e34
2026-Mar-01 04:42:04.222332
#8 resolve docker.io/library/node:22-alpine@sha256:e4bf2a82ad0a4037d28035ae71529873c069b13eb0455466ae0bc13363826e34 0.0s done
2026-Mar-01 04:42:04.222332
#8 DONE 0.0s
2026-Mar-01 04:42:04.222332
2026-Mar-01 04:42:04.222332
#9 [api runtime 1/7] FROM docker.io/library/debian:bookworm-slim@sha256:74d56e3931e0d5a1dd51f8c8a2466d21de84a271cd3b5a733b803aa91abf4421
2026-Mar-01 04:42:04.222332
#9 resolve docker.io/library/debian:bookworm-slim@sha256:74d56e3931e0d5a1dd51f8c8a2466d21de84a271cd3b5a733b803aa91abf4421 0.0s done
2026-Mar-01 04:42:04.222332
#9 DONE 0.0s
2026-Mar-01 04:42:04.222332
2026-Mar-01 04:42:04.222332
#10 [api builder  1/10] FROM docker.io/library/rust:1.88-bookworm@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0
2026-Mar-01 04:42:04.222332
#10 resolve docker.io/library/rust:1.88-bookworm@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0 0.0s done
2026-Mar-01 04:42:04.222332
#10 DONE 0.0s
2026-Mar-01 04:42:04.222332
2026-Mar-01 04:42:04.222332
#11 [api internal] load build context
2026-Mar-01 04:42:04.222332
#11 transferring context: 610.41kB 0.0s done
2026-Mar-01 04:42:04.222332
#11 DONE 0.0s
2026-Mar-01 04:42:04.222332
2026-Mar-01 04:42:04.222332
#12 [frontend internal] load build context
2026-Mar-01 04:42:04.222332
#12 transferring context: 807.52kB 0.0s done
2026-Mar-01 04:42:04.222332
#12 DONE 0.0s
2026-Mar-01 04:42:04.222332
2026-Mar-01 04:42:04.222332
#13 [frontend builder 3/6] COPY frontend/ ./
2026-Mar-01 04:42:04.222332
#13 CACHED
2026-Mar-01 04:42:04.222332
2026-Mar-01 04:42:04.222332
#14 [frontend builder 4/6] RUN npm install
2026-Mar-01 04:42:04.222332
#14 CACHED
2026-Mar-01 04:42:04.222332
2026-Mar-01 04:42:04.222332
#15 [frontend builder 6/6] RUN node build-docker.mjs
2026-Mar-01 04:42:04.222332
#15 CACHED
2026-Mar-01 04:42:04.222332
2026-Mar-01 04:42:04.222332
#16 [frontend runtime 4/6] COPY --from=builder /app/package.json ./
2026-Mar-01 04:42:04.222332
#16 CACHED
2026-Mar-01 04:42:04.222332
2026-Mar-01 04:42:04.222332
#17 [frontend runtime 5/6] COPY --from=builder /app/package-lock.json ./
2026-Mar-01 04:42:04.222332
#17 CACHED
2026-Mar-01 04:42:04.222332
2026-Mar-01 04:42:04.222332
#18 [frontend builder 2/6] WORKDIR /app
2026-Mar-01 04:42:04.222332
#18 CACHED
2026-Mar-01 04:42:04.222332
2026-Mar-01 04:42:04.222332
#19 [frontend builder 5/6] RUN test -n "https://api-download.khoadangbui.online" || (echo "VITE_API_URL build arg is required" && exit 1)
2026-Mar-01 04:42:04.222332
#19 CACHED
2026-Mar-01 04:42:04.222332
2026-Mar-01 04:42:04.222332
#20 [frontend runtime 3/6] COPY --from=builder /app/build ./build
2026-Mar-01 04:42:04.222332
#20 CACHED
2026-Mar-01 04:42:04.222332
2026-Mar-01 04:42:04.222332
#21 [frontend runtime 6/6] RUN npm ci --omit=dev
2026-Mar-01 04:42:04.370032
#21 CACHED
2026-Mar-01 04:42:04.370032
2026-Mar-01 04:42:04.370032
#22 [api builder  7/10] COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Mar-01 04:42:04.370032
#22 CACHED
2026-Mar-01 04:42:04.370032
2026-Mar-01 04:42:04.370032
#23 [api runtime 5/7] RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Mar-01 04:42:04.370032
#23 CACHED
2026-Mar-01 04:42:04.370032
2026-Mar-01 04:42:04.370032
#24 [api builder  4/10] COPY Cargo.toml ./
2026-Mar-01 04:42:04.370032
#24 CACHED
2026-Mar-01 04:42:04.370032
2026-Mar-01 04:42:04.370032
#25 [api js-builder  6/12] COPY apps/injector/package.json ./apps/injector/
2026-Mar-01 04:42:04.370032
#25 CACHED
2026-Mar-01 04:42:04.370032
2026-Mar-01 04:42:04.370032
#26 [api builder  8/10] COPY extractors/ ./extractors/
2026-Mar-01 04:42:04.370032
#26 CACHED
2026-Mar-01 04:42:04.370032
2026-Mar-01 04:42:04.370032
#27 [api builder  6/10] COPY crates/ ./crates/
2026-Mar-01 04:42:04.370032
#27 CACHED
2026-Mar-01 04:42:04.370032
2026-Mar-01 04:42:04.370032
#28 [api builder  2/10] WORKDIR /app
2026-Mar-01 04:42:04.370032
#28 CACHED
2026-Mar-01 04:42:04.370032
2026-Mar-01 04:42:04.370032
#29 [api runtime 6/7] COPY --from=builder /app/target/release/api-server /usr/local/bin/
2026-Mar-01 04:42:04.370032
#29 CACHED
2026-Mar-01 04:42:04.370032
2026-Mar-01 04:42:04.370032
#30 [api js-builder 11/12] COPY extractors/ ./extractors/
2026-Mar-01 04:42:04.370032
#30 CACHED
2026-Mar-01 04:42:04.370032
2026-Mar-01 04:42:04.370032
#31 [api builder  3/10] RUN apt-get update && apt-get install -y     pkg-config     libssl-dev     && rm -rf /var/lib/apt/lists/*
2026-Mar-01 04:42:04.370032
#31 CACHED
2026-Mar-01 04:42:04.372582
#32 [api runtime 3/7] RUN apt-get update && apt-get install -y     ca-certificates     curl     libssl3     && rm -rf /var/lib/apt/lists/*
2026-Mar-01 04:42:04.372582
#32 CACHED
2026-Mar-01 04:42:04.372582
2026-Mar-01 04:42:04.372582
#18 [api builder 2/6] WORKDIR /app
2026-Mar-01 04:42:04.372582
#18 CACHED
2026-Mar-01 04:42:04.372582
2026-Mar-01 04:42:04.372582
#33 [api builder 10/10] RUN cargo build --release --bin api-server
2026-Mar-01 04:42:04.372582
#33 CACHED
2026-Mar-01 04:42:04.372582
2026-Mar-01 04:42:04.372582
#34 [api js-builder  7/12] COPY apps/injector/ ./apps/injector/
2026-Mar-01 04:42:04.372582
#34 CACHED
2026-Mar-01 04:42:04.372582
2026-Mar-01 04:42:04.372582
#35 [api js-builder  3/12] RUN npm install -g pnpm
2026-Mar-01 04:42:04.372582
#35 CACHED
2026-Mar-01 04:42:04.372582
2026-Mar-01 04:42:04.372582
#36 [api js-builder 10/12] RUN pnpm --filter @downloadtool/injector build
2026-Mar-01 04:42:04.372582
#36 CACHED
2026-Mar-01 04:42:04.372582
2026-Mar-01 04:42:04.372582
#37 [api js-builder  4/12] COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Mar-01 04:42:04.372582
#37 CACHED
2026-Mar-01 04:42:04.372582
2026-Mar-01 04:42:04.372582
#38 [api builder  9/10] COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-01 04:42:04.372582
#38 CACHED
2026-Mar-01 04:42:04.372582
2026-Mar-01 04:42:04.372582
#39 [api js-builder 12/12] RUN mkdir -p extractors/dist &&     npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js &&     npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-01 04:42:04.372582
#39 CACHED
2026-Mar-01 04:42:04.372582
2026-Mar-01 04:42:04.372582
#40 [api runtime 2/7] WORKDIR /app
2026-Mar-01 04:42:04.372582
#40 CACHED
2026-Mar-01 04:42:04.372582
2026-Mar-01 04:42:04.372582
#41 [api js-builder  5/12] COPY packages/api-client/package.json ./packages/api-client/
2026-Mar-01 04:42:04.372582
#41 CACHED
2026-Mar-01 04:42:04.372582
2026-Mar-01 04:42:04.372582
#42 [api js-builder  8/12] COPY packages/ ./packages/
2026-Mar-01 04:42:04.372582
#42 CACHED
2026-Mar-01 04:42:04.372582
2026-Mar-01 04:42:04.372582
#43 [api js-builder  9/12] RUN pnpm install --frozen-lockfile
2026-Mar-01 04:42:04.372582
#43 CACHED
2026-Mar-01 04:42:04.372582
2026-Mar-01 04:42:04.372582
#44 [api builder  5/10] COPY Cargo.lock ./
2026-Mar-01 04:42:04.372582
#44 CACHED
2026-Mar-01 04:42:04.372582
2026-Mar-01 04:42:04.372582
#45 [api runtime 4/7] RUN set -eux;     arch="$(dpkg --print-architecture)";     case "$arch" in       amd64) ytdlp_asset="yt-dlp_linux" ;;       arm64) ytdlp_asset="yt-dlp_linux_aarch64" ;;       *) echo "Unsupported architecture: $arch" >&2; exit 1 ;;     esac;     curl -fL "https://github.com/yt-dlp/yt-dlp/releases/latest/download/${ytdlp_asset}" -o /usr/local/bin/yt-dlp;     chmod +x /usr/local/bin/yt-dlp;     /usr/local/bin/yt-dlp --version
2026-Mar-01 04:42:04.372582
#45 CACHED
2026-Mar-01 04:42:04.372582
2026-Mar-01 04:42:04.372582
#46 [api runtime 7/7] RUN mkdir -p /app/extractors /app/data && chown -R appuser:appuser /app
2026-Mar-01 04:42:04.372582
#46 CACHED
2026-Mar-01 04:42:04.372582
2026-Mar-01 04:42:04.372582
#47 [api] exporting to image
2026-Mar-01 04:42:04.372582
#47 exporting layers done
2026-Mar-01 04:42:04.372582
#47 exporting manifest sha256:bffcf744b9472ca0745c1d62e22f4a68d7eedb2bfde8ac8eb2d8a124df48ce6d done
2026-Mar-01 04:42:04.372582
#47 exporting config sha256:751842678012b3822724420d2a93214d02b64418734508c97e91d045cfd2a7ad done
2026-Mar-01 04:42:04.372582
#47 exporting attestation manifest sha256:7da0f27efc89e9d0c4c4e0277fef6e1997259a4148354e1081e58aefd8597c3b 0.0s done
2026-Mar-01 04:42:04.372582
#47 exporting manifest list sha256:ead82aa294fd5dee3ed0dfbf07aab65109ff8427bc0da81a78f8f7a5916e53ca done
2026-Mar-01 04:42:04.372582
#47 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_api:dcc856454f8e5fcafa7c6df17c1aae344d490065 done
2026-Mar-01 04:42:04.372582
#47 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_api:dcc856454f8e5fcafa7c6df17c1aae344d490065 done
2026-Mar-01 04:42:04.372582
#47 DONE 0.0s
2026-Mar-01 04:42:04.372582
2026-Mar-01 04:42:04.372582
#48 [frontend] exporting to image
2026-Mar-01 04:42:04.372582
#48 exporting layers done
2026-Mar-01 04:42:04.372582
#48 exporting manifest sha256:dc94223e2eda0f360ba9ece312595d5ba9f8b2a515ce27065de7e7202d48c7b1 done
2026-Mar-01 04:42:04.372582
#48 exporting config sha256:031a1fe0e8c420fa90377fb25fbf3fa61f8b05d09d10121fa7b38504f8bae971 done
2026-Mar-01 04:42:04.372582
#48 exporting attestation manifest sha256:eb7b0e2cb73d0e16d3dbaad16ea9e4caa807e2dcb272492f257a44f9a5686b70 0.0s done
2026-Mar-01 04:42:04.372582
#48 exporting manifest list sha256:99f00b04e961a71b1a3cfd4cd66c3db3fb9f67df19e7d4eb115645d64fbcda79 done
2026-Mar-01 04:42:04.372582
#48 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:dcc856454f8e5fcafa7c6df17c1aae344d490065 done
2026-Mar-01 04:42:04.372582
#48 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:dcc856454f8e5fcafa7c6df17c1aae344d490065 done
2026-Mar-01 04:42:04.372582
#48 DONE 0.1s
2026-Mar-01 04:42:04.372582
2026-Mar-01 04:42:04.372582
#49 [frontend] resolving provenance for metadata file
2026-Mar-01 04:42:04.411461
#49 DONE 0.0s
2026-Mar-01 04:42:04.411461
2026-Mar-01 04:42:04.411461
#50 [api] resolving provenance for metadata file
2026-Mar-01 04:42:04.411461
#50 DONE 0.0s
2026-Mar-01 04:42:04.413802
api  Built
2026-Mar-01 04:42:04.413802
frontend  Built
2026-Mar-01 04:42:04.439683
Creating .env file with runtime variables for container.
2026-Mar-01 04:42:05.097136
Removing old containers.
2026-Mar-01 04:42:05.707989
[CMD]: docker stop -t 30 frontend-o8kccgkgwsockoocow8sg88s-043818350105
2026-Mar-01 04:42:05.707989
frontend-o8kccgkgwsockoocow8sg88s-043818350105
2026-Mar-01 04:42:06.024394
[CMD]: docker rm -f frontend-o8kccgkgwsockoocow8sg88s-043818350105
2026-Mar-01 04:42:06.024394
frontend-o8kccgkgwsockoocow8sg88s-043818350105
2026-Mar-01 04:42:06.323425
[CMD]: docker stop -t 30 postgres-o8kccgkgwsockoocow8sg88s-043818340368
2026-Mar-01 04:42:06.323425
postgres-o8kccgkgwsockoocow8sg88s-043818340368
2026-Mar-01 04:42:06.656180
[CMD]: docker rm -f postgres-o8kccgkgwsockoocow8sg88s-043818340368
2026-Mar-01 04:42:06.656180
postgres-o8kccgkgwsockoocow8sg88s-043818340368
2026-Mar-01 04:42:06.973148
[CMD]: docker stop -t 30 api-o8kccgkgwsockoocow8sg88s-043818345257
2026-Mar-01 04:42:06.973148
api-o8kccgkgwsockoocow8sg88s-043818345257
2026-Mar-01 04:42:07.285094
[CMD]: docker rm -f api-o8kccgkgwsockoocow8sg88s-043818345257
2026-Mar-01 04:42:07.285094
api-o8kccgkgwsockoocow8sg88s-043818345257
2026-Mar-01 04:42:07.287848
Starting new application.
2026-Mar-01 04:42:08.334912
[CMD]: docker exec ywscgkkkkwkc4o8w8c40cogc bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/ywscgkkkkwkc4o8w8c40cogc/.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/ywscgkkkkwkc4o8w8c40cogc -f /artifacts/ywscgkkkkwkc4o8w8c40cogc/docker/docker-compose.server.yml up -d'
2026-Mar-01 04:42:08.334912
Container postgres-o8kccgkgwsockoocow8sg88s-044159898764  Creating
2026-Mar-01 04:42:08.334912
Container api-o8kccgkgwsockoocow8sg88s-044159904107  Creating
2026-Mar-01 04:42:08.374613
Container api-o8kccgkgwsockoocow8sg88s-044159904107  Created
2026-Mar-01 04:42:08.377879
Container postgres-o8kccgkgwsockoocow8sg88s-044159898764  Created
2026-Mar-01 04:42:08.379915
Container frontend-o8kccgkgwsockoocow8sg88s-044159909515  Creating
2026-Mar-01 04:42:08.565429
Container frontend-o8kccgkgwsockoocow8sg88s-044159909515  Created
2026-Mar-01 04:42:08.576843
Container api-o8kccgkgwsockoocow8sg88s-044159904107  Starting
2026-Mar-01 04:42:08.576843
Container postgres-o8kccgkgwsockoocow8sg88s-044159898764  Starting
2026-Mar-01 04:42:08.762412
Container api-o8kccgkgwsockoocow8sg88s-044159904107  Started
2026-Mar-01 04:42:08.821604
Error response from daemon: failed to set up container networking: driver failed programming external connectivity on endpoint postgres-o8kccgkgwsockoocow8sg88s-044159898764 (a88ab595e35f4598109574070693d8647641f1f5b4c74364ad49e76b3a13cc4d): Bind for 127.0.0.1:5432 failed: port is already allocated
2026-Mar-01 04:42:08.825625
exit status 1
2026-Mar-01 04:42:08.859682
========================================
2026-Mar-01 04:42:08.864420
Deployment failed: Command execution failed (exit code 1): docker exec ywscgkkkkwkc4o8w8c40cogc bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/ywscgkkkkwkc4o8w8c40cogc/.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/ywscgkkkkwkc4o8w8c40cogc -f /artifacts/ywscgkkkkwkc4o8w8c40cogc/docker/docker-compose.server.yml up -d'
2026-Mar-01 04:42:08.864420
Error:  Container postgres-o8kccgkgwsockoocow8sg88s-044159898764  Creating
2026-Mar-01 04:42:08.864420
Container api-o8kccgkgwsockoocow8sg88s-044159904107  Creating
2026-Mar-01 04:42:08.864420
Container api-o8kccgkgwsockoocow8sg88s-044159904107  Created
2026-Mar-01 04:42:08.864420
Container postgres-o8kccgkgwsockoocow8sg88s-044159898764  Created
2026-Mar-01 04:42:08.864420
Container frontend-o8kccgkgwsockoocow8sg88s-044159909515  Creating
2026-Mar-01 04:42:08.864420
Container frontend-o8kccgkgwsockoocow8sg88s-044159909515  Created
2026-Mar-01 04:42:08.864420
Container api-o8kccgkgwsockoocow8sg88s-044159904107  Starting
2026-Mar-01 04:42:08.864420
Container postgres-o8kccgkgwsockoocow8sg88s-044159898764  Starting
2026-Mar-01 04:42:08.864420
Container api-o8kccgkgwsockoocow8sg88s-044159904107  Started
2026-Mar-01 04:42:08.864420
Error response from daemon: failed to set up container networking: driver failed programming external connectivity on endpoint postgres-o8kccgkgwsockoocow8sg88s-044159898764 (a88ab595e35f4598109574070693d8647641f1f5b4c74364ad49e76b3a13cc4d): Bind for 127.0.0.1:5432 failed: port is already allocated
2026-Mar-01 04:42:08.864420
exit status 1
2026-Mar-01 04:42:08.868895
Error type: RuntimeException
2026-Mar-01 04:42:08.873333
Error code: 0
2026-Mar-01 04:42:08.877916
Location: /var/www/html/app/Traits/ExecuteRemoteCommand.php:243
2026-Mar-01 04:42:08.882796
Stack trace (first 5 lines):
2026-Mar-01 04:42:08.887639
#0 /var/www/html/app/Traits/ExecuteRemoteCommand.php(104): App\Jobs\ApplicationDeploymentJob->executeCommandWithProcess()
2026-Mar-01 04:42:08.893169
#1 /var/www/html/vendor/laravel/framework/src/Illuminate/Collections/Traits/EnumeratesValues.php(272): App\Jobs\ApplicationDeploymentJob->{closure:App\Traits\ExecuteRemoteCommand::execute_remote_command():71}()
2026-Mar-01 04:42:08.898068
#2 /var/www/html/app/Traits/ExecuteRemoteCommand.php(71): Illuminate\Support\Collection->each()
2026-Mar-01 04:42:08.902424
#3 /var/www/html/app/Jobs/ApplicationDeploymentJob.php(816): App\Jobs\ApplicationDeploymentJob->execute_remote_command()
2026-Mar-01 04:42:08.906690
#4 /var/www/html/app/Jobs/ApplicationDeploymentJob.php(467): App\Jobs\ApplicationDeploymentJob->deploy_docker_compose_buildpack()
2026-Mar-01 04:42:08.910938
========================================
2026-Mar-01 04:42:09.840077
Gracefully shutting down build container: ywscgkkkkwkc4o8w8c40cogc
2026-Mar-01 04:42:10.338893
[CMD]: docker stop -t 30 ywscgkkkkwkc4o8w8c40cogc
2026-Mar-01 04:42:10.338893
ywscgkkkkwkc4o8w8c40cogc