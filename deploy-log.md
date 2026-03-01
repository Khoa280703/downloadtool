2026-Mar-01 11:05:07.491946
Starting deployment of khoa280703/downloadtool:main-zoscg4oc04gkwkssg0kw8w8w to localhost.
2026-Mar-01 11:05:07.656910
Preparing container with helper image: ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Mar-01 11:05:07.753946
[CMD]: docker stop -t 30 wsgos4ws0cgwg8kg8s8kgg04
2026-Mar-01 11:05:07.753946
Error response from daemon: No such container: wsgos4ws0cgwg8kg8s8kgg04
2026-Mar-01 11:05:07.885312
[CMD]: docker run -d --network coolify --name wsgos4ws0cgwg8kg8s8kgg04  --rm -v /var/run/docker.sock:/var/run/docker.sock ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Mar-01 11:05:07.885312
71598fade63efc8e1012b3b7f01b22ef3ca8f246c4fb73b76753f560bdb478a4
2026-Mar-01 11:05:09.031596
[CMD]: docker exec wsgos4ws0cgwg8kg8s8kgg04 bash -c 'GIT_SSH_COMMAND="ssh -o ConnectTimeout=30 -p 22 -o Port=22 -o LogLevel=ERROR -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git ls-remote https://github.com/Khoa280703/downloadtool refs/heads/main'
2026-Mar-01 11:05:09.031596
02f799165d0773b805376bd1a9ca3429b83f1489	refs/heads/main
2026-Mar-01 11:05:09.041688
----------------------------------------
2026-Mar-01 11:05:09.045669
Importing Khoa280703/downloadtool:main (commit sha 02f799165d0773b805376bd1a9ca3429b83f1489) to /artifacts/wsgos4ws0cgwg8kg8s8kgg04.
2026-Mar-01 11:05:09.174953
[CMD]: docker exec wsgos4ws0cgwg8kg8s8kgg04 bash -c 'git clone --depth=1 --recurse-submodules --shallow-submodules -b 'main' 'https://github.com/Khoa280703/downloadtool' '/artifacts/wsgos4ws0cgwg8kg8s8kgg04' && cd '/artifacts/wsgos4ws0cgwg8kg8s8kgg04' && if [ -f .gitmodules ]; then sed -i "s#git@\(.*\):#https://\1/#g" '/artifacts/wsgos4ws0cgwg8kg8s8kgg04'/.gitmodules || true && git submodule sync && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git submodule update --init --recursive --depth=1; fi && cd '/artifacts/wsgos4ws0cgwg8kg8s8kgg04' && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git lfs pull'
2026-Mar-01 11:05:09.174953
Cloning into '/artifacts/wsgos4ws0cgwg8kg8s8kgg04'...
2026-Mar-01 11:05:11.488961
[CMD]: docker exec wsgos4ws0cgwg8kg8s8kgg04 bash -c 'cd /artifacts/wsgos4ws0cgwg8kg8s8kgg04 && git log -1 02f799165d0773b805376bd1a9ca3429b83f1489 --pretty=%B'
2026-Mar-01 11:05:11.488961
feat(frontend): finalize i18n rollout and auth ui cleanup
2026-Mar-01 11:05:14.981270
[CMD]: docker exec wsgos4ws0cgwg8kg8s8kgg04 bash -c 'test -f /artifacts/wsgos4ws0cgwg8kg8s8kgg04/docker/Dockerfile.api && echo 'exists' || echo 'not found''
2026-Mar-01 11:05:14.981270
exists
2026-Mar-01 11:05:15.137198
[CMD]: docker exec wsgos4ws0cgwg8kg8s8kgg04 bash -c 'cat /artifacts/wsgos4ws0cgwg8kg8s8kgg04/docker/Dockerfile.api'
2026-Mar-01 11:05:15.137198
# Dockerfile for API service deployment
2026-Mar-01 11:05:15.137198
# Builds the API server and related components without GPU support
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
# Stage 0: Build injector JS (embedded into api crate via include_str! at compile time)
2026-Mar-01 11:05:15.137198
FROM node:22-alpine AS js-builder
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
WORKDIR /app
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
RUN npm install -g pnpm
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
# Copy workspace manifests for pnpm resolution
2026-Mar-01 11:05:15.137198
COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Mar-01 11:05:15.137198
COPY packages/api-client/package.json ./packages/api-client/
2026-Mar-01 11:05:15.137198
COPY apps/injector/package.json ./apps/injector/
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
# Copy injector source and shared packages
2026-Mar-01 11:05:15.137198
COPY apps/injector/ ./apps/injector/
2026-Mar-01 11:05:15.137198
COPY packages/ ./packages/
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
# Install deps and build injector (produces dist/bm.js and dist/youtube-downloader.user.js)
2026-Mar-01 11:05:15.137198
RUN pnpm install --frozen-lockfile
2026-Mar-01 11:05:15.137198
RUN pnpm --filter @downloadtool/injector build
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
# Build extractor TypeScript to IIFE format (required by crates/extractor/build.rs)
2026-Mar-01 11:05:15.137198
COPY extractors/ ./extractors/
2026-Mar-01 11:05:15.137198
RUN mkdir -p extractors/dist && \
2026-Mar-01 11:05:15.137198
npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js && \
2026-Mar-01 11:05:15.137198
npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
# Stage 1: Rust builder
2026-Mar-01 11:05:15.137198
FROM rust:1.88-bookworm AS builder
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
WORKDIR /app
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
# Install dependencies
2026-Mar-01 11:05:15.137198
RUN apt-get update && apt-get install -y \
2026-Mar-01 11:05:15.137198
pkg-config \
2026-Mar-01 11:05:15.137198
libssl-dev \
2026-Mar-01 11:05:15.137198
&& rm -rf /var/lib/apt/lists/*
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
# Copy workspace configuration
2026-Mar-01 11:05:15.137198
COPY Cargo.toml ./
2026-Mar-01 11:05:15.137198
COPY Cargo.lock ./
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
# Copy all crates
2026-Mar-01 11:05:15.137198
COPY crates/ ./crates/
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
# Copy injector dist (required by include_str! in crates/api/src/routes/static_files.rs)
2026-Mar-01 11:05:15.137198
COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
# Copy extractor source + pre-built IIFE dist (built by js-builder stage)
2026-Mar-01 11:05:15.137198
COPY extractors/ ./extractors/
2026-Mar-01 11:05:15.137198
COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
# Build the release binary
2026-Mar-01 11:05:15.137198
RUN cargo build --release --bin api-server
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
# Stage 2: Runtime
2026-Mar-01 11:05:15.137198
FROM debian:bookworm-slim AS runtime
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
WORKDIR /app
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
# Install runtime dependencies
2026-Mar-01 11:05:15.137198
RUN apt-get update && apt-get install -y \
2026-Mar-01 11:05:15.137198
ca-certificates \
2026-Mar-01 11:05:15.137198
curl \
2026-Mar-01 11:05:15.137198
libssl3 \
2026-Mar-01 11:05:15.137198
&& rm -rf /var/lib/apt/lists/*
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
# Install latest yt-dlp binary (newer than Debian package).
2026-Mar-01 11:05:15.137198
RUN set -eux; \
2026-Mar-01 11:05:15.137198
arch="$(dpkg --print-architecture)"; \
2026-Mar-01 11:05:15.137198
case "$arch" in \
2026-Mar-01 11:05:15.137198
amd64) ytdlp_asset="yt-dlp_linux" ;; \
2026-Mar-01 11:05:15.137198
arm64) ytdlp_asset="yt-dlp_linux_aarch64" ;; \
2026-Mar-01 11:05:15.137198
*) echo "Unsupported architecture: $arch" >&2; exit 1 ;; \
2026-Mar-01 11:05:15.137198
esac; \
2026-Mar-01 11:05:15.137198
curl -fL "https://github.com/yt-dlp/yt-dlp/releases/latest/download/${ytdlp_asset}" -o /usr/local/bin/yt-dlp; \
2026-Mar-01 11:05:15.137198
chmod +x /usr/local/bin/yt-dlp; \
2026-Mar-01 11:05:15.137198
/usr/local/bin/yt-dlp --version
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
# Create non-root user
2026-Mar-01 11:05:15.137198
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
# Copy binary from builder
2026-Mar-01 11:05:15.137198
COPY --from=builder /app/target/release/api-server /usr/local/bin/
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
# Create directories
2026-Mar-01 11:05:15.137198
RUN mkdir -p /app/extractors /app/data && chown -R appuser:appuser /app
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
# Switch to non-root user
2026-Mar-01 11:05:15.137198
USER appuser
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
# Environment variables
2026-Mar-01 11:05:15.137198
ENV PORT=3068
2026-Mar-01 11:05:15.137198
ENV EXTRACTOR_DIR=/app/extractors
2026-Mar-01 11:05:15.137198
ENV YTDLP_PATH=/usr/local/bin/yt-dlp
2026-Mar-01 11:05:15.137198
ENV RUST_LOG=info
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
# Expose port
2026-Mar-01 11:05:15.137198
EXPOSE 3068
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
# Health check
2026-Mar-01 11:05:15.137198
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Mar-01 11:05:15.137198
CMD curl -f http://localhost:3068/health || exit 1
2026-Mar-01 11:05:15.137198
2026-Mar-01 11:05:15.137198
# Run the server
2026-Mar-01 11:05:15.137198
CMD ["api-server"]
2026-Mar-01 11:05:15.291057
Added 42 ARG declarations to Dockerfile for service api (multi-stage build, added to 3 stages).
2026-Mar-01 11:05:15.438604
[CMD]: docker exec wsgos4ws0cgwg8kg8s8kgg04 bash -c 'test -f /artifacts/wsgos4ws0cgwg8kg8s8kgg04/docker/Dockerfile.frontend && echo 'exists' || echo 'not found''
2026-Mar-01 11:05:15.438604
exists
2026-Mar-01 11:05:15.584823
[CMD]: docker exec wsgos4ws0cgwg8kg8s8kgg04 bash -c 'cat /artifacts/wsgos4ws0cgwg8kg8s8kgg04/docker/Dockerfile.frontend'
2026-Mar-01 11:05:15.584823
# Dockerfile for frontend (SvelteKit Node server)
2026-Mar-01 11:05:15.584823
# Copy ALL source files BEFORE npm install so svelte-kit sync (prepare script)
2026-Mar-01 11:05:15.584823
# can find svelte.config.js and generate .svelte-kit/ correctly.
2026-Mar-01 11:05:15.584823
2026-Mar-01 11:05:15.584823
FROM node:22-alpine AS builder
2026-Mar-01 11:05:15.584823
2026-Mar-01 11:05:15.584823
WORKDIR /app
2026-Mar-01 11:05:15.584823
2026-Mar-01 11:05:15.584823
# Copy all frontend source files first (node_modules excluded via .dockerignore)
2026-Mar-01 11:05:15.584823
COPY frontend/ ./
2026-Mar-01 11:05:15.584823
2026-Mar-01 11:05:15.584823
# Install — prepare script runs svelte-kit sync with svelte.config.js available
2026-Mar-01 11:05:15.584823
RUN npm install
2026-Mar-01 11:05:15.584823
2026-Mar-01 11:05:15.584823
# Build-time public API URL (embedded into client bundle by Vite)
2026-Mar-01 11:05:15.584823
# Runtime env is too late for import.meta.env in browser bundle.
2026-Mar-01 11:05:15.584823
ARG VITE_API_URL
2026-Mar-01 11:05:15.584823
ENV VITE_API_URL=${VITE_API_URL}
2026-Mar-01 11:05:15.584823
RUN test -n "$VITE_API_URL" || (echo "VITE_API_URL build arg is required" && exit 1)
2026-Mar-01 11:05:15.584823
2026-Mar-01 11:05:15.584823
# Build via programmatic API — bypasses vite.config.ts loading (esbuild issues on Alpine)
2026-Mar-01 11:05:15.584823
RUN node build-docker.mjs
2026-Mar-01 11:05:15.584823
2026-Mar-01 11:05:15.584823
# Runtime
2026-Mar-01 11:05:15.584823
FROM node:22-alpine AS runtime
2026-Mar-01 11:05:15.584823
2026-Mar-01 11:05:15.584823
WORKDIR /app
2026-Mar-01 11:05:15.584823
2026-Mar-01 11:05:15.584823
COPY --from=builder /app/build ./build
2026-Mar-01 11:05:15.584823
COPY --from=builder /app/package.json ./
2026-Mar-01 11:05:15.584823
COPY --from=builder /app/package-lock.json ./
2026-Mar-01 11:05:15.584823
2026-Mar-01 11:05:15.584823
# Runtime needs server-side deps (better-auth, pg) used by hooks/routes
2026-Mar-01 11:05:15.584823
RUN npm ci --omit=dev
2026-Mar-01 11:05:15.584823
2026-Mar-01 11:05:15.584823
ENV PORT=5168
2026-Mar-01 11:05:15.584823
ENV HOST=0.0.0.0
2026-Mar-01 11:05:15.584823
2026-Mar-01 11:05:15.584823
EXPOSE 5168
2026-Mar-01 11:05:15.584823
2026-Mar-01 11:05:15.584823
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Mar-01 11:05:15.584823
CMD wget -qO- http://127.0.0.1:5168 || exit 1
2026-Mar-01 11:05:15.584823
2026-Mar-01 11:05:15.584823
CMD ["node", "build"]
2026-Mar-01 11:05:15.743835
Added 28 ARG declarations to Dockerfile for service frontend (multi-stage build, added to 2 stages).
2026-Mar-01 11:05:15.747820
Pulling & building required images.
2026-Mar-01 11:05:15.764697
Creating build-time .env file in /artifacts (outside Docker context).
2026-Mar-01 11:05:15.907685
Adding build arguments to Docker Compose build command.
2026-Mar-01 11:05:16.173091
[CMD]: docker exec wsgos4ws0cgwg8kg8s8kgg04 bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/build-time.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/wsgos4ws0cgwg8kg8s8kgg04 -f /artifacts/wsgos4ws0cgwg8kg8s8kgg04/docker/docker-compose.server.yml build --pull --build-arg COOLIFY_URL --build-arg COOLIFY_FQDN --build-arg SERVICE_URL_API --build-arg SERVICE_FQDN_FRONTEND --build-arg SERVICE_FQDN_API --build-arg ORIGIN --build-arg SERVICE_URL_FRONTEND --build-arg WHOP_WEBHOOK_SECRET --build-arg POSTGRES_PASSWORD --build-arg BETTER_AUTH_TRUSTED_ORIGINS --build-arg BETTER_AUTH_SECRET --build-arg GOOGLE_CLIENT_ID --build-arg GOOGLE_CLIENT_SECRET --build-arg WHOP_PLAN_ID --build-arg COOLIFY_BUILD_SECRETS_HASH=5211b99f02c8ef84c5b4387c0c0250f5d88df66df08eae10ee3d4d4bf2231816'
2026-Mar-01 11:05:16.173091
#1 [internal] load local bake definitions
2026-Mar-01 11:05:16.376190
#1 reading from stdin 2.99kB done
2026-Mar-01 11:05:16.376190
#1 DONE 0.0s
2026-Mar-01 11:05:16.376190
2026-Mar-01 11:05:16.376190
#2 [api internal] load build definition from Dockerfile.api
2026-Mar-01 11:05:16.376190
#2 transferring dockerfile: 4.27kB done
2026-Mar-01 11:05:16.376190
#2 DONE 0.0s
2026-Mar-01 11:05:16.376190
2026-Mar-01 11:05:16.376190
#3 [frontend internal] load build definition from Dockerfile.frontend
2026-Mar-01 11:05:16.376190
#3 transferring dockerfile: 1.94kB done
2026-Mar-01 11:05:16.376190
#3 DONE 0.0s
2026-Mar-01 11:05:16.376190
2026-Mar-01 11:05:16.376190
#4 [frontend internal] load metadata for docker.io/library/node:22-alpine
2026-Mar-01 11:05:17.554692
#4 ...
2026-Mar-01 11:05:17.554692
2026-Mar-01 11:05:17.554692
#5 [api internal] load metadata for docker.io/library/rust:1.88-bookworm
2026-Mar-01 11:05:17.554692
#5 DONE 1.3s
2026-Mar-01 11:05:17.666276
#4 [api internal] load metadata for docker.io/library/node:22-alpine
2026-Mar-01 11:05:17.666276
#4 DONE 1.3s
2026-Mar-01 11:05:17.666276
2026-Mar-01 11:05:17.666276
#6 [api internal] load metadata for docker.io/library/debian:bookworm-slim
2026-Mar-01 11:05:17.666276
#6 DONE 1.3s
2026-Mar-01 11:05:17.666276
2026-Mar-01 11:05:17.666276
#7 [frontend internal] load .dockerignore
2026-Mar-01 11:05:17.666276
#7 transferring context: 341B done
2026-Mar-01 11:05:17.666276
#7 DONE 0.0s
2026-Mar-01 11:05:17.666276
2026-Mar-01 11:05:17.666276
#8 [frontend builder 1/6] FROM docker.io/library/node:22-alpine@sha256:e4bf2a82ad0a4037d28035ae71529873c069b13eb0455466ae0bc13363826e34
2026-Mar-01 11:05:17.666276
#8 resolve docker.io/library/node:22-alpine@sha256:e4bf2a82ad0a4037d28035ae71529873c069b13eb0455466ae0bc13363826e34 0.0s done
2026-Mar-01 11:05:17.666276
#8 DONE 0.0s
2026-Mar-01 11:05:17.666276
2026-Mar-01 11:05:17.666276
#9 [api runtime 1/7] FROM docker.io/library/debian:bookworm-slim@sha256:74d56e3931e0d5a1dd51f8c8a2466d21de84a271cd3b5a733b803aa91abf4421
2026-Mar-01 11:05:17.666276
#9 resolve docker.io/library/debian:bookworm-slim@sha256:74d56e3931e0d5a1dd51f8c8a2466d21de84a271cd3b5a733b803aa91abf4421 0.0s done
2026-Mar-01 11:05:17.666276
#9 DONE 0.0s
2026-Mar-01 11:05:17.666276
2026-Mar-01 11:05:17.666276
#10 [api builder  1/10] FROM docker.io/library/rust:1.88-bookworm@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0
2026-Mar-01 11:05:17.666276
#10 resolve docker.io/library/rust:1.88-bookworm@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0 0.0s done
2026-Mar-01 11:05:17.666276
#10 DONE 0.0s
2026-Mar-01 11:05:17.666276
2026-Mar-01 11:05:17.666276
#11 [api internal] load build context
2026-Mar-01 11:05:17.666276
#11 transferring context: 660.65kB 0.0s done
2026-Mar-01 11:05:17.666276
#11 DONE 0.0s
2026-Mar-01 11:05:17.666276
2026-Mar-01 11:05:17.666276
#12 [api js-builder  3/12] RUN npm install -g pnpm
2026-Mar-01 11:05:17.666276
#12 CACHED
2026-Mar-01 11:05:17.666276
2026-Mar-01 11:05:17.666276
#13 [frontend internal] load build context
2026-Mar-01 11:05:17.666276
#13 transferring context: 1.33MB 0.0s done
2026-Mar-01 11:05:17.666276
#13 DONE 0.0s
2026-Mar-01 11:05:17.666276
2026-Mar-01 11:05:17.666276
#14 [frontend builder 2/6] WORKDIR /app
2026-Mar-01 11:05:17.666276
#14 CACHED
2026-Mar-01 11:05:17.666276
2026-Mar-01 11:05:17.666276
#15 [frontend builder 3/6] COPY frontend/ ./
2026-Mar-01 11:05:17.666276
#15 DONE 0.0s
2026-Mar-01 11:05:17.666276
2026-Mar-01 11:05:17.666276
#16 [api js-builder  4/12] COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Mar-01 11:05:17.826543
#16 DONE 0.0s
2026-Mar-01 11:05:17.826543
2026-Mar-01 11:05:17.826543
#17 [api js-builder  5/12] COPY packages/api-client/package.json ./packages/api-client/
2026-Mar-01 11:05:17.826543
#17 DONE 0.0s
2026-Mar-01 11:05:17.826543
2026-Mar-01 11:05:17.826543
#18 [api js-builder  6/12] COPY apps/injector/package.json ./apps/injector/
2026-Mar-01 11:05:17.826543
#18 DONE 0.0s
2026-Mar-01 11:05:17.826543
2026-Mar-01 11:05:17.826543
#19 [api js-builder  7/12] COPY apps/injector/ ./apps/injector/
2026-Mar-01 11:05:17.826543
#19 DONE 0.0s
2026-Mar-01 11:05:17.826543
2026-Mar-01 11:05:17.826543
#20 [api js-builder  8/12] COPY packages/ ./packages/
2026-Mar-01 11:05:17.826543
#20 DONE 0.1s
2026-Mar-01 11:05:17.826543
2026-Mar-01 11:05:17.826543
#21 [frontend builder 4/6] RUN npm install
2026-Mar-01 11:05:20.851734
#21 ...
2026-Mar-01 11:05:20.851734
2026-Mar-01 11:05:20.851734
#22 [api js-builder  9/12] RUN pnpm install --frozen-lockfile
2026-Mar-01 11:05:20.851734
#22 0.611 Scope: all 3 workspace projects
2026-Mar-01 11:05:20.851734
#22 0.692 Lockfile is up to date, resolution step is skipped
2026-Mar-01 11:05:20.851734
#22 0.728 Progress: resolved 1, reused 0, downloaded 0, added 0
2026-Mar-01 11:05:20.851734
#22 0.780 Packages: +105
2026-Mar-01 11:05:20.851734
#22 0.780 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
2026-Mar-01 11:05:20.851734
#22 1.729 Progress: resolved 105, reused 0, downloaded 95, added 95
2026-Mar-01 11:05:20.851734
#22 2.467 Progress: resolved 105, reused 0, downloaded 105, added 105, done
2026-Mar-01 11:05:20.851734
#22 2.537 .../esbuild@0.25.12/node_modules/esbuild postinstall$ node install.js
2026-Mar-01 11:05:20.851734
#22 2.602 .../esbuild@0.25.12/node_modules/esbuild postinstall: Done
2026-Mar-01 11:05:20.851734
#22 2.672
2026-Mar-01 11:05:20.851734
#22 2.774 Done in 2.6s using pnpm v10.30.3
2026-Mar-01 11:05:20.851734
#22 DONE 2.9s
2026-Mar-01 11:05:21.001932
#23 [api js-builder 10/12] RUN pnpm --filter @downloadtool/injector build
2026-Mar-01 11:05:21.310850
#23 0.609
2026-Mar-01 11:05:21.310850
#23 0.609 > @downloadtool/injector@0.0.1 build /app/apps/injector
2026-Mar-01 11:05:21.310850
#23 0.609 > vite build && vite build --config vite.userscript.config.ts
2026-Mar-01 11:05:21.310850
#23 0.609
2026-Mar-01 11:05:21.545440
#23 0.844 vite v6.4.1 building for production...
2026-Mar-01 11:05:21.657149
#23 0.889 transforming...
2026-Mar-01 11:05:21.657149
#23 0.935 ✓ 4 modules transformed.
2026-Mar-01 11:05:21.657149
#23 0.956 rendering chunks...
2026-Mar-01 11:05:21.814755
#23 0.960 computing gzip size...
2026-Mar-01 11:05:21.814755
#23 0.963 dist/bm.js  6.00 kB │ gzip: 2.27 kB
2026-Mar-01 11:05:21.814755
#23 0.963 ✓ built in 92ms
2026-Mar-01 11:05:21.930420
#23 1.229 vite v6.4.1 building for production...
2026-Mar-01 11:05:22.094442
#23 1.264 transforming...
2026-Mar-01 11:05:22.094442
#23 1.311 ✓ 4 modules transformed.
2026-Mar-01 11:05:22.094442
#23 1.327 rendering chunks...
2026-Mar-01 11:05:22.094442
#23 1.393 computing gzip size...
2026-Mar-01 11:05:22.313219
#23 1.395 dist/youtube-downloader.user.js  10.03 kB │ gzip: 3.09 kB
2026-Mar-01 11:05:22.313219
#23 1.395 ✓ built in 153ms
2026-Mar-01 11:05:22.313219
#23 DONE 1.4s
2026-Mar-01 11:05:22.313219
2026-Mar-01 11:05:22.313219
#24 [api js-builder 11/12] COPY extractors/ ./extractors/
2026-Mar-01 11:05:22.313219
#24 DONE 0.0s
2026-Mar-01 11:05:22.313219
2026-Mar-01 11:05:22.313219
#25 [api js-builder 12/12] RUN mkdir -p extractors/dist &&     npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js &&     npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Mar-01 11:05:24.378927
#25 2.216
2026-Mar-01 11:05:24.378927
#25 2.216   extractors/dist/types.js  1.2kb
2026-Mar-01 11:05:24.378927
#25 2.216
2026-Mar-01 11:05:24.378927
#25 2.216 ⚡ Done in 2ms
2026-Mar-01 11:05:25.059752
#25 2.897
2026-Mar-01 11:05:25.059752
#25 2.897   extractors/dist/youtube.js  22.4kb
2026-Mar-01 11:05:25.059752
#25 2.897
2026-Mar-01 11:05:25.059752
#25 2.897 ⚡ Done in 3ms
2026-Mar-01 11:05:25.245245
#25 DONE 2.9s
2026-Mar-01 11:05:25.245245
2026-Mar-01 11:05:25.245245
#26 [api builder 10/10] RUN cargo build --release --bin api-server
2026-Mar-01 11:05:25.245245
#26 CACHED
2026-Mar-01 11:05:25.245245
2026-Mar-01 11:05:25.245245
#27 [api builder  2/10] WORKDIR /app
2026-Mar-01 11:05:25.245245
#27 CACHED
2026-Mar-01 11:05:25.245245
2026-Mar-01 11:05:25.245245
#28 [api builder  6/10] COPY crates/ ./crates/
2026-Mar-01 11:05:25.245245
#28 CACHED
2026-Mar-01 11:05:25.245245
2026-Mar-01 11:05:25.245245
#29 [api builder  4/10] COPY Cargo.toml ./
2026-Mar-01 11:05:25.245245
#29 CACHED
2026-Mar-01 11:05:25.245245
2026-Mar-01 11:05:25.245245
#30 [api runtime 4/7] RUN set -eux;     arch="$(dpkg --print-architecture)";     case "$arch" in       amd64) ytdlp_asset="yt-dlp_linux" ;;       arm64) ytdlp_asset="yt-dlp_linux_aarch64" ;;       *) echo "Unsupported architecture: $arch" >&2; exit 1 ;;     esac;     curl -fL "https://github.com/yt-dlp/yt-dlp/releases/latest/download/${ytdlp_asset}" -o /usr/local/bin/yt-dlp;     chmod +x /usr/local/bin/yt-dlp;     /usr/local/bin/yt-dlp --version
2026-Mar-01 11:05:25.245245
#30 CACHED
2026-Mar-01 11:05:25.245245
2026-Mar-01 11:05:25.245245
#31 [api runtime 6/7] COPY --from=builder /app/target/release/api-server /usr/local/bin/
2026-Mar-01 11:05:25.245245
#31 CACHED
2026-Mar-01 11:05:25.245245
2026-Mar-01 11:05:25.245245
#32 [api builder  5/10] COPY Cargo.lock ./
2026-Mar-01 11:05:25.245245
#32 CACHED
2026-Mar-01 11:05:25.245245
2026-Mar-01 11:05:25.245245
#33 [api runtime 3/7] RUN apt-get update && apt-get install -y     ca-certificates     curl     libssl3     && rm -rf /var/lib/apt/lists/*
2026-Mar-01 11:05:25.245245
#33 CACHED
2026-Mar-01 11:05:25.245245
2026-Mar-01 11:05:25.245245
#34 [api builder  3/10] RUN apt-get update && apt-get install -y     pkg-config     libssl-dev     && rm -rf /var/lib/apt/lists/*
2026-Mar-01 11:05:25.245245
#34 CACHED
2026-Mar-01 11:05:25.245245
2026-Mar-01 11:05:25.245245
#35 [api runtime 2/7] WORKDIR /app
2026-Mar-01 11:05:25.245245
#35 CACHED
2026-Mar-01 11:05:25.245245
2026-Mar-01 11:05:25.245245
#36 [api builder  9/10] COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Mar-01 11:05:25.245245
#36 CACHED
2026-Mar-01 11:05:25.245245
2026-Mar-01 11:05:25.245245
#37 [api runtime 5/7] RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Mar-01 11:05:25.245245
#37 CACHED
2026-Mar-01 11:05:25.245245
2026-Mar-01 11:05:25.245245
#38 [api builder  7/10] COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Mar-01 11:05:25.245245
#38 CACHED
2026-Mar-01 11:05:25.245245
2026-Mar-01 11:05:25.245245
#39 [api builder  8/10] COPY extractors/ ./extractors/
2026-Mar-01 11:05:25.245245
#39 CACHED
2026-Mar-01 11:05:25.245245
2026-Mar-01 11:05:25.245245
#40 [api runtime 7/7] RUN mkdir -p /app/extractors /app/data && chown -R appuser:appuser /app
2026-Mar-01 11:05:25.245245
#40 CACHED
2026-Mar-01 11:05:25.245245
2026-Mar-01 11:05:25.245245
#41 [api] exporting to image
2026-Mar-01 11:05:25.245245
#41 exporting layers done
2026-Mar-01 11:05:25.245245
#41 exporting manifest sha256:f5b0bcd070464b28c3580aa37d8711bd150587cdd63791a68c63bccc76af1efa done
2026-Mar-01 11:05:25.245245
#41 exporting config sha256:c130a01b9c696cdc8626f9be4379392bfe9ed4fb0835cee91b8275c8f67e9f16 done
2026-Mar-01 11:05:25.245245
#41 exporting attestation manifest sha256:d79777df5900189c2778afc56b705e11c15f0785501fb96601ab61fafcca21d1 0.0s done
2026-Mar-01 11:05:25.245245
#41 exporting manifest list sha256:103845730051175fe89b44e5a41d73b59bdcc9640528d1b818c84991f51f564b 0.0s done
2026-Mar-01 11:05:25.245245
#41 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_api:02f799165d0773b805376bd1a9ca3429b83f1489 done
2026-Mar-01 11:05:25.245245
#41 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_api:02f799165d0773b805376bd1a9ca3429b83f1489 done
2026-Mar-01 11:05:25.245245
#41 DONE 0.1s
2026-Mar-01 11:05:25.245245
2026-Mar-01 11:05:25.245245
#42 [api] resolving provenance for metadata file
2026-Mar-01 11:05:25.403593
#42 DONE 0.0s
2026-Mar-01 11:05:25.403593
2026-Mar-01 11:05:25.403593
#21 [frontend builder 4/6] RUN npm install
2026-Mar-01 11:05:25.700223
#21 8.031
2026-Mar-01 11:05:25.700223
#21 8.031 > frontend@0.0.1 prepare
2026-Mar-01 11:05:25.700223
#21 8.031 > svelte-kit sync || echo ''
2026-Mar-01 11:05:25.700223
#21 8.031
2026-Mar-01 11:05:26.518207
#21 8.849
2026-Mar-01 11:05:26.518207
#21 8.849 added 237 packages, and audited 238 packages in 9s
2026-Mar-01 11:05:26.518207
#21 8.849
2026-Mar-01 11:05:26.518207
#21 8.849 43 packages are looking for funding
2026-Mar-01 11:05:26.518207
#21 8.849   run `npm fund` for details
2026-Mar-01 11:05:26.669905
#21 8.864
2026-Mar-01 11:05:26.669905
#21 8.864 3 vulnerabilities (2 low, 1 moderate)
2026-Mar-01 11:05:26.669905
#21 8.864
2026-Mar-01 11:05:26.669905
#21 8.864 To address all issues, run:
2026-Mar-01 11:05:26.669905
#21 8.864   npm audit fix
2026-Mar-01 11:05:26.669905
#21 8.864
2026-Mar-01 11:05:26.669905
#21 8.864 Run `npm audit` for details.
2026-Mar-01 11:05:26.669905
#21 8.865 npm notice
2026-Mar-01 11:05:26.669905
#21 8.865 npm notice New major version of npm available! 10.9.4 -> 11.11.0
2026-Mar-01 11:05:26.669905
#21 8.865 npm notice Changelog: https://github.com/npm/cli/releases/tag/v11.11.0
2026-Mar-01 11:05:26.669905
#21 8.865 npm notice To update run: npm install -g npm@11.11.0
2026-Mar-01 11:05:26.669905
#21 8.865 npm notice
2026-Mar-01 11:05:26.669905
#21 DONE 9.0s
2026-Mar-01 11:05:26.820913
#43 [frontend builder 5/6] RUN test -n "https://api-download.khoadangbui.online" || (echo "VITE_API_URL build arg is required" && exit 1)
2026-Mar-01 11:05:26.939157
#43 DONE 0.3s
2026-Mar-01 11:05:27.090206
#44 [frontend builder 6/6] RUN node build-docker.mjs
2026-Mar-01 11:05:28.001909
#44 1.062 The following Vite config options will be overridden by SvelteKit:
2026-Mar-01 11:05:28.001909
#44 1.062   - build.outDir
2026-Mar-01 11:05:28.198196
#44 1.091 vite v6.4.1 building SSR bundle for production...
2026-Mar-01 11:05:28.198196
#44 1.108 transforming...
2026-Mar-01 11:05:28.874322
#44 1.934 ✓ 145 modules transformed.
2026-Mar-01 11:05:28.957360
#44 1.938 ✗ Build failed in 846ms
2026-Mar-01 11:05:28.957360
#44 1.939 node:internal/modules/run_main:123
2026-Mar-01 11:05:28.957360
#44 1.939     triggerUncaughtException(
2026-Mar-01 11:05:28.957360
#44 1.939     ^
2026-Mar-01 11:05:28.957360
#44 1.939
2026-Mar-01 11:05:28.957360
#44 1.939 [vite:load-fallback] Could not load /app/src/lib/paraglide/runtime (imported by src/routes/(auth)/account/+page.svelte): ENOENT: no such file or directory, open '/app/src/lib/paraglide/runtime'
2026-Mar-01 11:05:28.957360
#44 1.939     at async open (node:internal/fs/promises:636:25)
2026-Mar-01 11:05:28.957360
#44 1.939     at async Object.readFile (node:internal/fs/promises:1235:14)
2026-Mar-01 11:05:28.957360
#44 1.939     at async Object.handler (file:///app/node_modules/vite/dist/node/chunks/dep-D4NMHUTW.js:45872:27)
2026-Mar-01 11:05:28.957360
#44 1.939     at async PluginDriver.hookFirstAndGetPlugin (file:///app/node_modules/rollup/dist/es/shared/node-entry.js:22453:28)
2026-Mar-01 11:05:28.957360
#44 1.939     at async file:///app/node_modules/rollup/dist/es/shared/node-entry.js:21445:33
2026-Mar-01 11:05:28.957360
#44 1.939     at async Queue.work (file:///app/node_modules/rollup/dist/es/shared/node-entry.js:22681:32) {
2026-Mar-01 11:05:28.957360
#44 1.939   errno: -2,
2026-Mar-01 11:05:28.957360
#44 1.939   code: 'PLUGIN_ERROR',
2026-Mar-01 11:05:28.957360
#44 1.939   syscall: 'open',
2026-Mar-01 11:05:28.957360
#44 1.939   path: '/app/src/lib/paraglide/runtime',
2026-Mar-01 11:05:28.957360
#44 1.939   pluginCode: 'ENOENT',
2026-Mar-01 11:05:28.957360
#44 1.939   plugin: 'vite:load-fallback',
2026-Mar-01 11:05:28.957360
#44 1.939   hook: 'load',
2026-Mar-01 11:05:28.957360
#44 1.939   watchFiles: [
2026-Mar-01 11:05:28.957360
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/app/server/remote/index.js',
2026-Mar-01 11:05:28.957360
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/server/index.js',
2026-Mar-01 11:05:28.957360
#44 1.939     '/app/src/routes/api/auth/[...all]/+server.ts',
2026-Mar-01 11:05:28.957360
#44 1.939     '/app/.svelte-kit/generated/server/internal.js',
2026-Mar-01 11:05:28.957360
#44 1.939     '/app/src/routes/api/checkout/+server.ts',
2026-Mar-01 11:05:28.957360
#44 1.939     '/app/src/routes/api/checkout/callback/+server.ts',
2026-Mar-01 11:05:28.957360
#44 1.939     '/app/src/routes/api/proxy/extract/+server.ts',
2026-Mar-01 11:05:28.957360
#44 1.939     '/app/src/routes/share-target/+server.ts',
2026-Mar-01 11:05:28.957360
#44 1.939     '/app/src/routes/sitemap.xml/+server.ts',
2026-Mar-01 11:05:28.957360
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/components/svelte-5/error.svelte',
2026-Mar-01 11:05:28.957360
#44 1.939     '/app/src/routes/+layout.svelte',
2026-Mar-01 11:05:28.957360
#44 1.939     '/app/src/routes/+page.ts',
2026-Mar-01 11:05:28.957360
#44 1.939     '/app/src/routes/(auth)/account/+page.svelte',
2026-Mar-01 11:05:28.957360
#44 1.939     '/app/src/routes/(auth)/account/+page.server.ts',
2026-Mar-01 11:05:28.957360
#44 1.939     '/app/src/routes/+page.svelte',
2026-Mar-01 11:05:28.957360
#44 1.939     '/app/src/routes/privacy/+page.svelte',
2026-Mar-01 11:05:28.957360
#44 1.939     '/app/src/hooks.server.ts',
2026-Mar-01 11:05:28.957360
#44 1.939     '/app/src/hooks.ts',
2026-Mar-01 11:05:28.957360
#44 1.939     '/app/package.json',
2026-Mar-01 11:05:28.957360
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/app/server/remote/command.js',
2026-Mar-01 11:05:28.957360
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/app/server/remote/form.js',
2026-Mar-01 11:05:28.957360
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/app/server/remote/prerender.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/app/server/remote/query.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/utils/promise.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/server/respond.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/server/constants.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/shared-server.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/esm-env/index.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/utils/env.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/server/utils.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/server/app.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/app/paths/internal/server.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/.svelte-kit/generated/root.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/app/state/index.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/svelte/src/internal/server/index.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/src/lib/auth-actions.ts',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/app/navigation.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/app/environment/index.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/svelte/src/index-server.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/src/components/AuthModal.svelte',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/src/components/SiteHeader.svelte',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/src/components/SiteFooter.svelte',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/src/components/DownloadBtn.svelte',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/src/components/FormatPicker.svelte',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/src/components/BatchProgress.svelte',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/src/components/BatchInput.svelte',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/src/lib/api.ts',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/src/stores/download.ts',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/src/lib/analytics.ts',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/app/stores.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/src/app.css',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/src/components/CookieConsent.svelte',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/form-utils.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/app/server/remote/shared.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/shared.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/esm-env/true.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/esm-env/false.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/utils/error.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/utils/http.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/utils/escape.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/constants.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/svelte/src/legacy/legacy-server.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/app/state/client.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/.svelte-kit/generated/root.svelte',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/app/state/server.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/svelte/src/internal/shared/utils.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/svelte/src/internal/shared/attributes.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/svelte/src/constants.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/svelte/src/store/utils.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/svelte/src/escaping.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/svelte/src/internal/server/hydration.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/svelte/src/internal/shared/validate.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/svelte/src/internal/server/context.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/svelte/src/internal/server/blocks/html.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/svelte/src/utils.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/svelte/src/internal/server/renderer.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/svelte/src/internal/server/errors.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/svelte/src/internal/server/dev.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/svelte/src/internal/shared/clone.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/svelte/src/internal/server/hydratable.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/svelte/src/internal/server/blocks/snippet.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/svelte/src/internal/server/abort-signal.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/src/lib/auth-client.ts',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/src/components/LanguageSwitcher.svelte',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/src/components/UserMenu.svelte',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/src/stores/batch.ts',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/src/lib/playlist-download-worker.ts',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/src/components/BatchActiveState.svelte',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/src/lib/playlist-download-stream-selection.ts',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/client/client.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/server/endpoint.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/server/page/respond_with_error.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/utils/url.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/server/page/render.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/utils/routing.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/server/page/index.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/server/cookie.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/server/data/index.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/runtime/server/fetch.js',
2026-Mar-01 11:05:28.960110
#44 1.939     '/app/node_modules/@sveltejs/kit/src/utils/page_nodes.js',
2026-Mar-01 11:05:28.960110
#44 1.939     ... 47 more items
2026-Mar-01 11:05:28.960110
#44 1.939   ]
2026-Mar-01 11:05:28.960110
#44 1.939 }
2026-Mar-01 11:05:28.960110
#44 1.939
2026-Mar-01 11:05:28.960110
#44 1.939 Node.js v22.22.0
2026-Mar-01 11:05:28.960110
#44 ERROR: process "/bin/sh -c node build-docker.mjs" did not complete successfully: exit code: 1
2026-Mar-01 11:05:28.960110
------
2026-Mar-01 11:05:28.960110
> [frontend builder 6/6] RUN node build-docker.mjs:
2026-Mar-01 11:05:28.960110
1.939     '/app/node_modules/@sveltejs/kit/src/runtime/server/page/index.js',
2026-Mar-01 11:05:28.960110
1.939     '/app/node_modules/@sveltejs/kit/src/runtime/server/cookie.js',
2026-Mar-01 11:05:28.960110
1.939     '/app/node_modules/@sveltejs/kit/src/runtime/server/data/index.js',
2026-Mar-01 11:05:28.960110
1.939     '/app/node_modules/@sveltejs/kit/src/runtime/server/fetch.js',
2026-Mar-01 11:05:28.960110
1.939     '/app/node_modules/@sveltejs/kit/src/utils/page_nodes.js',
2026-Mar-01 11:05:28.960110
1.939     ... 47 more items
2026-Mar-01 11:05:28.960110
1.939   ]
2026-Mar-01 11:05:28.960110
1.939 }
2026-Mar-01 11:05:28.960110
1.939
2026-Mar-01 11:05:28.960110
1.939 Node.js v22.22.0
2026-Mar-01 11:05:28.960110
------
2026-Mar-01 11:05:28.962310
Dockerfile.frontend:36
2026-Mar-01 11:05:28.962310
2026-Mar-01 11:05:28.962310
--------------------
2026-Mar-01 11:05:28.962310
2026-Mar-01 11:05:28.962310
34 |
2026-Mar-01 11:05:28.962310
2026-Mar-01 11:05:28.962310
35 |     # Build via programmatic API — bypasses vite.config.ts loading (esbuild issues on Alpine)
2026-Mar-01 11:05:28.962310
2026-Mar-01 11:05:28.962310
36 | >>> RUN node build-docker.mjs
2026-Mar-01 11:05:28.962310
2026-Mar-01 11:05:28.962310
37 |
2026-Mar-01 11:05:28.962310
2026-Mar-01 11:05:28.962310
38 |     # Runtime
2026-Mar-01 11:05:28.962310
2026-Mar-01 11:05:28.962310
--------------------
2026-Mar-01 11:05:28.962310
2026-Mar-01 11:05:28.962310
target frontend: failed to solve: process "/bin/sh -c node build-docker.mjs" did not complete successfully: exit code: 1
2026-Mar-01 11:05:28.964481
exit status 1
2026-Mar-01 11:05:28.998839
========================================
2026-Mar-01 11:05:29.005641
Deployment failed: Command execution failed (exit code 1): docker exec wsgos4ws0cgwg8kg8s8kgg04 bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/build-time.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/wsgos4ws0cgwg8kg8s8kgg04 -f /artifacts/wsgos4ws0cgwg8kg8s8kgg04/docker/docker-compose.server.yml build --pull --build-arg COOLIFY_URL --build-arg COOLIFY_FQDN --build-arg SERVICE_URL_API --build-arg SERVICE_FQDN_FRONTEND --build-arg SERVICE_FQDN_API --build-arg ORIGIN --build-arg SERVICE_URL_FRONTEND --build-arg WHOP_WEBHOOK_SECRET --build-arg POSTGRES_PASSWORD --build-arg BETTER_AUTH_TRUSTED_ORIGINS --build-arg BETTER_AUTH_SECRET --build-arg GOOGLE_CLIENT_ID --build-arg GOOGLE_CLIENT_SECRET --build-arg WHOP_PLAN_ID --build-arg COOLIFY_BUILD_SECRETS_HASH=5211b99f02c8ef84c5b4387c0c0250f5d88df66df08eae10ee3d4d4bf2231816'
2026-Mar-01 11:05:29.005641
Error: Dockerfile.frontend:36
2026-Mar-01 11:05:29.005641
2026-Mar-01 11:05:29.005641
--------------------
2026-Mar-01 11:05:29.005641
2026-Mar-01 11:05:29.005641
34 |
2026-Mar-01 11:05:29.005641
2026-Mar-01 11:05:29.005641
35 |     # Build via programmatic API — bypasses vite.config.ts loading (esbuild issues on Alpine)
2026-Mar-01 11:05:29.005641
2026-Mar-01 11:05:29.005641
36 | >>> RUN node build-docker.mjs
2026-Mar-01 11:05:29.005641
2026-Mar-01 11:05:29.005641
37 |
2026-Mar-01 11:05:29.005641
2026-Mar-01 11:05:29.005641
38 |     # Runtime
2026-Mar-01 11:05:29.005641
2026-Mar-01 11:05:29.005641
--------------------
2026-Mar-01 11:05:29.005641
2026-Mar-01 11:05:29.005641
target frontend: failed to solve: process "/bin/sh -c node build-docker.mjs" did not complete successfully: exit code: 1
2026-Mar-01 11:05:29.005641
2026-Mar-01 11:05:29.005641
exit status 1
2026-Mar-01 11:05:29.014812
Error type: RuntimeException
2026-Mar-01 11:05:29.020786
Error code: 0
2026-Mar-01 11:05:29.026704
Location: /var/www/html/app/Traits/ExecuteRemoteCommand.php:243
2026-Mar-01 11:05:29.031791
Stack trace (first 5 lines):
2026-Mar-01 11:05:29.036872
#0 /var/www/html/app/Traits/ExecuteRemoteCommand.php(104): App\Jobs\ApplicationDeploymentJob->executeCommandWithProcess()
2026-Mar-01 11:05:29.042407
#1 /var/www/html/vendor/laravel/framework/src/Illuminate/Collections/Traits/EnumeratesValues.php(272): App\Jobs\ApplicationDeploymentJob->{closure:App\Traits\ExecuteRemoteCommand::execute_remote_command():71}()