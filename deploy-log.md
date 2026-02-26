2026-Feb-26 05:18:35.668768
Starting deployment of khoa280703/downloadtool:main-zoscg4oc04gkwkssg0kw8w8w to localhost.
2026-Feb-26 05:18:35.829647
Preparing container with helper image: ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Feb-26 05:18:35.922883
[CMD]: docker stop -t 30 dccg880ogw040c0k8oc4g804
2026-Feb-26 05:18:35.922883
Error response from daemon: No such container: dccg880ogw040c0k8oc4g804
2026-Feb-26 05:18:36.039725
[CMD]: docker run -d --network coolify --name dccg880ogw040c0k8oc4g804  --rm -v /var/run/docker.sock:/var/run/docker.sock ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Feb-26 05:18:36.039725
3c24d577b0caf27aadb988f1e1650c6f75b240743b9cb011a6128a3525407619
2026-Feb-26 05:18:37.192865
[CMD]: docker exec dccg880ogw040c0k8oc4g804 bash -c 'GIT_SSH_COMMAND="ssh -o ConnectTimeout=30 -p 22 -o Port=22 -o LogLevel=ERROR -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git ls-remote https://github.com/Khoa280703/downloadtool refs/heads/main'
2026-Feb-26 05:18:37.192865
1dd1833466b0e016e695c744fde76176c17d118b	refs/heads/main
2026-Feb-26 05:18:37.203011
----------------------------------------
2026-Feb-26 05:18:37.206807
Importing Khoa280703/downloadtool:main (commit sha 1dd1833466b0e016e695c744fde76176c17d118b) to /artifacts/dccg880ogw040c0k8oc4g804.
2026-Feb-26 05:18:37.355660
[CMD]: docker exec dccg880ogw040c0k8oc4g804 bash -c 'git clone --depth=1 --recurse-submodules --shallow-submodules -b 'main' 'https://github.com/Khoa280703/downloadtool' '/artifacts/dccg880ogw040c0k8oc4g804' && cd '/artifacts/dccg880ogw040c0k8oc4g804' && if [ -f .gitmodules ]; then sed -i "s#git@\(.*\):#https://\1/#g" '/artifacts/dccg880ogw040c0k8oc4g804'/.gitmodules || true && git submodule sync && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git submodule update --init --recursive --depth=1; fi && cd '/artifacts/dccg880ogw040c0k8oc4g804' && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git lfs pull'
2026-Feb-26 05:18:37.355660
Cloning into '/artifacts/dccg880ogw040c0k8oc4g804'...
2026-Feb-26 05:18:39.226081
[CMD]: docker exec dccg880ogw040c0k8oc4g804 bash -c 'cd /artifacts/dccg880ogw040c0k8oc4g804 && git log -1 1dd1833466b0e016e695c744fde76176c17d118b --pretty=%B'
2026-Feb-26 05:18:39.226081
fix(deploy): prevent api startup crash when extractor dir is not bundle file
2026-Feb-26 05:18:42.786958
[CMD]: docker exec dccg880ogw040c0k8oc4g804 bash -c 'test -f /artifacts/dccg880ogw040c0k8oc4g804/docker/Dockerfile.api && echo 'exists' || echo 'not found''
2026-Feb-26 05:18:42.786958
exists
2026-Feb-26 05:18:42.929993
[CMD]: docker exec dccg880ogw040c0k8oc4g804 bash -c 'cat /artifacts/dccg880ogw040c0k8oc4g804/docker/Dockerfile.api'
2026-Feb-26 05:18:42.929993
# Dockerfile for API service deployment
2026-Feb-26 05:18:42.929993
# Builds the API server and related components without GPU support
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
# Stage 0: Build injector JS (embedded into api crate via include_str! at compile time)
2026-Feb-26 05:18:42.929993
FROM node:22-alpine AS js-builder
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
WORKDIR /app
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
RUN npm install -g pnpm
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
# Copy workspace manifests for pnpm resolution
2026-Feb-26 05:18:42.929993
COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Feb-26 05:18:42.929993
COPY packages/api-client/package.json ./packages/api-client/
2026-Feb-26 05:18:42.929993
COPY apps/injector/package.json ./apps/injector/
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
# Copy injector source and shared packages
2026-Feb-26 05:18:42.929993
COPY apps/injector/ ./apps/injector/
2026-Feb-26 05:18:42.929993
COPY packages/ ./packages/
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
# Install deps and build injector (produces dist/bm.js and dist/youtube-downloader.user.js)
2026-Feb-26 05:18:42.929993
RUN pnpm install --frozen-lockfile
2026-Feb-26 05:18:42.929993
RUN pnpm --filter @downloadtool/injector build
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
# Build extractor TypeScript to IIFE format (required by crates/extractor/build.rs)
2026-Feb-26 05:18:42.929993
COPY extractors/ ./extractors/
2026-Feb-26 05:18:42.929993
RUN mkdir -p extractors/dist && \
2026-Feb-26 05:18:42.929993
npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js && \
2026-Feb-26 05:18:42.929993
npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
# Stage 1: Rust builder
2026-Feb-26 05:18:42.929993
FROM rust:1.88-bookworm AS builder
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
WORKDIR /app
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
# Install dependencies
2026-Feb-26 05:18:42.929993
RUN apt-get update && apt-get install -y \
2026-Feb-26 05:18:42.929993
pkg-config \
2026-Feb-26 05:18:42.929993
libssl-dev \
2026-Feb-26 05:18:42.929993
&& rm -rf /var/lib/apt/lists/*
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
# Copy workspace configuration
2026-Feb-26 05:18:42.929993
COPY Cargo.toml ./
2026-Feb-26 05:18:42.929993
COPY Cargo.lock ./
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
# Copy all crates
2026-Feb-26 05:18:42.929993
COPY crates/ ./crates/
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
# Copy injector dist (required by include_str! in crates/api/src/routes/static_files.rs)
2026-Feb-26 05:18:42.929993
COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
# Copy extractor source + pre-built IIFE dist (built by js-builder stage)
2026-Feb-26 05:18:42.929993
COPY extractors/ ./extractors/
2026-Feb-26 05:18:42.929993
COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
# Build the release binary
2026-Feb-26 05:18:42.929993
RUN cargo build --release --bin api-server
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
# Stage 2: Runtime
2026-Feb-26 05:18:42.929993
FROM debian:bookworm-slim AS runtime
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
WORKDIR /app
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
# Install runtime dependencies
2026-Feb-26 05:18:42.929993
RUN apt-get update && apt-get install -y \
2026-Feb-26 05:18:42.929993
ca-certificates \
2026-Feb-26 05:18:42.929993
curl \
2026-Feb-26 05:18:42.929993
libssl3 \
2026-Feb-26 05:18:42.929993
&& rm -rf /var/lib/apt/lists/*
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
# Create non-root user
2026-Feb-26 05:18:42.929993
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
# Copy binary from builder
2026-Feb-26 05:18:42.929993
COPY --from=builder /app/target/release/api-server /usr/local/bin/
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
# Create directories
2026-Feb-26 05:18:42.929993
RUN mkdir -p /app/extractors /app/data && chown -R appuser:appuser /app
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
# Switch to non-root user
2026-Feb-26 05:18:42.929993
USER appuser
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
# Environment variables
2026-Feb-26 05:18:42.929993
ENV PORT=3068
2026-Feb-26 05:18:42.929993
ENV EXTRACTOR_DIR=/app/extractors
2026-Feb-26 05:18:42.929993
ENV RUST_LOG=info
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
# Expose port
2026-Feb-26 05:18:42.929993
EXPOSE 3068
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
# Health check
2026-Feb-26 05:18:42.929993
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Feb-26 05:18:42.929993
CMD curl -f http://localhost:3068/health || exit 1
2026-Feb-26 05:18:42.929993
2026-Feb-26 05:18:42.929993
# Run the server
2026-Feb-26 05:18:42.929993
CMD ["api-server"]
2026-Feb-26 05:18:43.075827
Added 18 ARG declarations to Dockerfile for service api (multi-stage build, added to 3 stages).
2026-Feb-26 05:18:43.211290
[CMD]: docker exec dccg880ogw040c0k8oc4g804 bash -c 'test -f /artifacts/dccg880ogw040c0k8oc4g804/docker/Dockerfile.frontend && echo 'exists' || echo 'not found''
2026-Feb-26 05:18:43.211290
exists
2026-Feb-26 05:18:43.358443
[CMD]: docker exec dccg880ogw040c0k8oc4g804 bash -c 'cat /artifacts/dccg880ogw040c0k8oc4g804/docker/Dockerfile.frontend'
2026-Feb-26 05:18:43.358443
# Dockerfile for frontend (SvelteKit Node server)
2026-Feb-26 05:18:43.358443
# Copy ALL source files BEFORE npm install so svelte-kit sync (prepare script)
2026-Feb-26 05:18:43.358443
# can find svelte.config.js and generate .svelte-kit/ correctly.
2026-Feb-26 05:18:43.358443
2026-Feb-26 05:18:43.358443
FROM node:22-alpine AS builder
2026-Feb-26 05:18:43.358443
2026-Feb-26 05:18:43.358443
WORKDIR /app
2026-Feb-26 05:18:43.358443
2026-Feb-26 05:18:43.358443
# Copy all frontend source files first (node_modules excluded via .dockerignore)
2026-Feb-26 05:18:43.358443
COPY frontend/ ./
2026-Feb-26 05:18:43.358443
2026-Feb-26 05:18:43.358443
# Install — prepare script runs svelte-kit sync with svelte.config.js available
2026-Feb-26 05:18:43.358443
RUN npm install
2026-Feb-26 05:18:43.358443
2026-Feb-26 05:18:43.358443
# Build-time public API URL (embedded into client bundle by Vite)
2026-Feb-26 05:18:43.358443
# Runtime env is too late for import.meta.env in browser bundle.
2026-Feb-26 05:18:43.358443
ARG VITE_API_URL
2026-Feb-26 05:18:43.358443
ENV VITE_API_URL=${VITE_API_URL}
2026-Feb-26 05:18:43.358443
RUN test -n "$VITE_API_URL" || (echo "VITE_API_URL build arg is required" && exit 1)
2026-Feb-26 05:18:43.358443
2026-Feb-26 05:18:43.358443
# Build via programmatic API — bypasses vite.config.ts loading (esbuild issues on Alpine)
2026-Feb-26 05:18:43.358443
RUN node build-docker.mjs
2026-Feb-26 05:18:43.358443
2026-Feb-26 05:18:43.358443
# Runtime
2026-Feb-26 05:18:43.358443
FROM node:22-alpine AS runtime
2026-Feb-26 05:18:43.358443
2026-Feb-26 05:18:43.358443
WORKDIR /app
2026-Feb-26 05:18:43.358443
2026-Feb-26 05:18:43.358443
COPY --from=builder /app/build ./build
2026-Feb-26 05:18:43.358443
COPY --from=builder /app/package.json ./
2026-Feb-26 05:18:43.358443
2026-Feb-26 05:18:43.358443
ENV PORT=5168
2026-Feb-26 05:18:43.358443
ENV HOST=0.0.0.0
2026-Feb-26 05:18:43.358443
2026-Feb-26 05:18:43.358443
EXPOSE 5168
2026-Feb-26 05:18:43.358443
2026-Feb-26 05:18:43.358443
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Feb-26 05:18:43.358443
CMD wget -qO- http://localhost:5168 || exit 1
2026-Feb-26 05:18:43.358443
2026-Feb-26 05:18:43.358443
CMD ["node", "build"]
2026-Feb-26 05:18:43.500370
Added 12 ARG declarations to Dockerfile for service frontend (multi-stage build, added to 2 stages).
2026-Feb-26 05:18:43.504611
Pulling & building required images.
2026-Feb-26 05:18:43.510563
Creating build-time .env file in /artifacts (outside Docker context).
2026-Feb-26 05:18:43.649701
Adding build arguments to Docker Compose build command.
2026-Feb-26 05:18:43.916892
[CMD]: docker exec dccg880ogw040c0k8oc4g804 bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/build-time.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/dccg880ogw040c0k8oc4g804 -f /artifacts/dccg880ogw040c0k8oc4g804/docker/docker-compose.server.yml build --pull --build-arg COOLIFY_URL --build-arg COOLIFY_FQDN --build-arg SERVICE_URL_FRONTEND --build-arg SERVICE_FQDN_FRONTEND --build-arg SERVICE_URL_API --build-arg SERVICE_FQDN_API --build-arg COOLIFY_BUILD_SECRETS_HASH=32f198a9a75e162000e91bc16bafad93fb124329f6d6764b64249da4c9668423'
2026-Feb-26 05:18:43.916892
#1 [internal] load local bake definitions
2026-Feb-26 05:18:44.117303
#1 reading from stdin 1.81kB done
2026-Feb-26 05:18:44.117303
#1 DONE 0.0s
2026-Feb-26 05:18:44.117303
2026-Feb-26 05:18:44.117303
#2 [frontend internal] load build definition from Dockerfile.frontend
2026-Feb-26 05:18:44.117303
#2 transferring dockerfile: 1.45kB done
2026-Feb-26 05:18:44.117303
#2 DONE 0.0s
2026-Feb-26 05:18:44.117303
2026-Feb-26 05:18:44.117303
#3 [api internal] load build definition from Dockerfile.api
2026-Feb-26 05:18:44.117303
#3 transferring dockerfile: 3.20kB done
2026-Feb-26 05:18:44.117303
#3 DONE 0.0s
2026-Feb-26 05:18:44.117303
2026-Feb-26 05:18:44.117303
#4 [api internal] load metadata for docker.io/library/node:22-alpine
2026-Feb-26 05:18:44.728793
#4 ...
2026-Feb-26 05:18:44.728793
2026-Feb-26 05:18:44.728793
#5 [api internal] load metadata for docker.io/library/rust:1.88-bookworm
2026-Feb-26 05:18:44.728793
#5 DONE 0.8s
2026-Feb-26 05:18:44.829002
#4 [frontend internal] load metadata for docker.io/library/node:22-alpine
2026-Feb-26 05:18:44.829002
#4 DONE 0.8s
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#6 [api internal] load metadata for docker.io/library/debian:bookworm-slim
2026-Feb-26 05:18:44.829002
#6 DONE 0.8s
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#7 [api internal] load .dockerignore
2026-Feb-26 05:18:44.829002
#7 transferring context: 341B done
2026-Feb-26 05:18:44.829002
#7 DONE 0.0s
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#8 [api builder 1/6] FROM docker.io/library/node:22-alpine@sha256:e4bf2a82ad0a4037d28035ae71529873c069b13eb0455466ae0bc13363826e34
2026-Feb-26 05:18:44.829002
#8 resolve docker.io/library/node:22-alpine@sha256:e4bf2a82ad0a4037d28035ae71529873c069b13eb0455466ae0bc13363826e34 0.0s done
2026-Feb-26 05:18:44.829002
#8 DONE 0.0s
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#9 [api runtime 1/6] FROM docker.io/library/debian:bookworm-slim@sha256:74d56e3931e0d5a1dd51f8c8a2466d21de84a271cd3b5a733b803aa91abf4421
2026-Feb-26 05:18:44.829002
#9 resolve docker.io/library/debian:bookworm-slim@sha256:74d56e3931e0d5a1dd51f8c8a2466d21de84a271cd3b5a733b803aa91abf4421 0.0s done
2026-Feb-26 05:18:44.829002
#9 DONE 0.0s
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#10 [api builder  1/10] FROM docker.io/library/rust:1.88-bookworm@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0
2026-Feb-26 05:18:44.829002
#10 resolve docker.io/library/rust:1.88-bookworm@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0 0.0s done
2026-Feb-26 05:18:44.829002
#10 DONE 0.0s
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#11 [frontend internal] load build context
2026-Feb-26 05:18:44.829002
#11 transferring context: 409.89kB 0.0s done
2026-Feb-26 05:18:44.829002
#11 DONE 0.0s
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#12 [api internal] load build context
2026-Feb-26 05:18:44.829002
#12 transferring context: 502.75kB 0.0s done
2026-Feb-26 05:18:44.829002
#12 DONE 0.0s
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#13 [frontend builder 5/6] RUN test -n "https://api-download.khoadangbui.online" || (echo "VITE_API_URL build arg is required" && exit 1)
2026-Feb-26 05:18:44.829002
#13 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#14 [frontend builder 4/6] RUN npm install
2026-Feb-26 05:18:44.829002
#14 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#15 [frontend builder 3/6] COPY frontend/ ./
2026-Feb-26 05:18:44.829002
#15 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#16 [frontend builder 6/6] RUN node build-docker.mjs
2026-Feb-26 05:18:44.829002
#16 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#17 [frontend runtime 3/4] COPY --from=builder /app/build ./build
2026-Feb-26 05:18:44.829002
#17 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#18 [frontend runtime 4/4] COPY --from=builder /app/package.json ./
2026-Feb-26 05:18:44.829002
#18 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#19 [api js-builder  6/12] COPY apps/injector/package.json ./apps/injector/
2026-Feb-26 05:18:44.829002
#19 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#20 [api js-builder 10/12] RUN pnpm --filter @downloadtool/injector build
2026-Feb-26 05:18:44.829002
#20 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#21 [api js-builder  4/12] COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Feb-26 05:18:44.829002
#21 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#22 [api builder  9/10] COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Feb-26 05:18:44.829002
#22 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#23 [api js-builder  7/12] COPY apps/injector/ ./apps/injector/
2026-Feb-26 05:18:44.829002
#23 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#24 [api builder 10/10] RUN cargo build --release --bin api-server
2026-Feb-26 05:18:44.829002
#24 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#25 [api js-builder  8/12] COPY packages/ ./packages/
2026-Feb-26 05:18:44.829002
#25 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#26 [api builder  7/10] COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Feb-26 05:18:44.829002
#26 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#27 [api builder  8/10] COPY extractors/ ./extractors/
2026-Feb-26 05:18:44.829002
#27 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#28 [api builder  6/10] COPY crates/ ./crates/
2026-Feb-26 05:18:44.829002
#28 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#29 [api runtime 3/6] RUN apt-get update && apt-get install -y     ca-certificates     curl     libssl3     && rm -rf /var/lib/apt/lists/*
2026-Feb-26 05:18:44.829002
#29 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#30 [api js-builder  9/12] RUN pnpm install --frozen-lockfile
2026-Feb-26 05:18:44.829002
#30 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#31 [api js-builder  5/12] COPY packages/api-client/package.json ./packages/api-client/
2026-Feb-26 05:18:44.829002
#31 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#32 [api builder  3/10] RUN apt-get update && apt-get install -y     pkg-config     libssl-dev     && rm -rf /var/lib/apt/lists/*
2026-Feb-26 05:18:44.829002
#32 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#33 [api builder  2/10] WORKDIR /app
2026-Feb-26 05:18:44.829002
#33 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#34 [api builder 2/6] WORKDIR /app
2026-Feb-26 05:18:44.829002
#34 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#35 [api js-builder 11/12] COPY extractors/ ./extractors/
2026-Feb-26 05:18:44.829002
#35 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#36 [api builder  5/10] COPY Cargo.lock ./
2026-Feb-26 05:18:44.829002
#36 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#37 [api runtime 2/6] WORKDIR /app
2026-Feb-26 05:18:44.829002
#37 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#38 [api runtime 5/6] COPY --from=builder /app/target/release/api-server /usr/local/bin/
2026-Feb-26 05:18:44.829002
#38 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#39 [api js-builder  3/12] RUN npm install -g pnpm
2026-Feb-26 05:18:44.829002
#39 CACHED
2026-Feb-26 05:18:44.829002
2026-Feb-26 05:18:44.829002
#40 [api builder  4/10] COPY Cargo.toml ./
2026-Feb-26 05:18:44.829002
#40 CACHED
2026-Feb-26 05:18:44.831197
#41 [api runtime 4/6] RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-26 05:18:44.831197
#41 CACHED
2026-Feb-26 05:18:44.831197
2026-Feb-26 05:18:44.831197
#42 [api js-builder 12/12] RUN mkdir -p extractors/dist &&     npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js &&     npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Feb-26 05:18:44.831197
#42 CACHED
2026-Feb-26 05:18:44.831197
2026-Feb-26 05:18:44.831197
#43 [api runtime 6/6] RUN mkdir -p /app/extractors /app/data && chown -R appuser:appuser /app
2026-Feb-26 05:18:44.831197
#43 CACHED
2026-Feb-26 05:18:44.831197
2026-Feb-26 05:18:44.831197
#44 [api] exporting to image
2026-Feb-26 05:18:44.831197
#44 exporting layers
2026-Feb-26 05:18:44.939944
#44 exporting layers done
2026-Feb-26 05:18:44.939944
#44 exporting manifest sha256:1c824be6d3fe584ed94457e1f1f5c63ee1bbe570434e3b73c26d6e88d440161b done
2026-Feb-26 05:18:44.939944
#44 exporting config sha256:f15d43237bed1525b98d0e1e012d3e95fe86fe711b6dca4b100ddb5d15994e11 done
2026-Feb-26 05:18:44.939944
#44 exporting attestation manifest sha256:072b387069c2ce4b1f8637267e2194a5c1b1751d9827fe022d86c00762ab35da 0.0s done
2026-Feb-26 05:18:44.939944
#44 exporting manifest list sha256:b208a03673572243f025e473bca90855ea8069f08ec9e78b31115c2c947eff5e done
2026-Feb-26 05:18:44.939944
#44 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_api:1dd1833466b0e016e695c744fde76176c17d118b done
2026-Feb-26 05:18:44.939944
#44 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_api:1dd1833466b0e016e695c744fde76176c17d118b done
2026-Feb-26 05:18:44.939944
#44 DONE 0.0s
2026-Feb-26 05:18:44.939944
2026-Feb-26 05:18:44.939944
#45 [frontend] exporting to image
2026-Feb-26 05:18:44.939944
#45 exporting layers done
2026-Feb-26 05:18:44.939944
#45 exporting manifest sha256:3d171c76b8dc351d3fb3f956347f22cc5cbc0b2976f5612f380780d3a3ccf193 done
2026-Feb-26 05:18:44.939944
#45 exporting config sha256:a5fc215f5b05f116e7c79e9e67253200c475fdb41339244368454a5cab41c8d8 done
2026-Feb-26 05:18:44.939944
#45 exporting attestation manifest sha256:5d726c5b94c0bd38aa0443f2c8cec5645f21278e430ed0c8bcd9dde7418c9725 0.0s done
2026-Feb-26 05:18:44.939944
#45 exporting manifest list sha256:f0af4cc654c060fb10508310b310aa180ba29e1552190bf3704de4ec0230d806 done
2026-Feb-26 05:18:44.939944
#45 naming to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:1dd1833466b0e016e695c744fde76176c17d118b done
2026-Feb-26 05:18:44.939944
#45 unpacking to docker.io/library/o8kccgkgwsockoocow8sg88s_frontend:1dd1833466b0e016e695c744fde76176c17d118b done
2026-Feb-26 05:18:44.939944
#45 DONE 0.1s
2026-Feb-26 05:18:44.939944
2026-Feb-26 05:18:44.939944
#46 [frontend] resolving provenance for metadata file
2026-Feb-26 05:18:44.955234
#46 DONE 0.0s
2026-Feb-26 05:18:44.955234
2026-Feb-26 05:18:44.955234
#47 [api] resolving provenance for metadata file
2026-Feb-26 05:18:44.955234
#47 DONE 0.0s
2026-Feb-26 05:18:44.957446
frontend  Built
2026-Feb-26 05:18:44.957446
api  Built
2026-Feb-26 05:18:44.968727
Creating .env file with runtime variables for container.
2026-Feb-26 05:18:45.194554
Removing old containers.
2026-Feb-26 05:18:45.670072
[CMD]: docker stop -t 30 frontend-o8kccgkgwsockoocow8sg88s-051423339008
2026-Feb-26 05:18:45.670072
frontend-o8kccgkgwsockoocow8sg88s-051423339008
2026-Feb-26 05:18:45.793982
[CMD]: docker rm -f frontend-o8kccgkgwsockoocow8sg88s-051423339008
2026-Feb-26 05:18:45.793982
frontend-o8kccgkgwsockoocow8sg88s-051423339008
2026-Feb-26 05:19:16.170642
[CMD]: docker stop -t 30 api-o8kccgkgwsockoocow8sg88s-051423333436
2026-Feb-26 05:19:16.170642
api-o8kccgkgwsockoocow8sg88s-051423333436
2026-Feb-26 05:19:16.295339
[CMD]: docker rm -f api-o8kccgkgwsockoocow8sg88s-051423333436
2026-Feb-26 05:19:16.295339
api-o8kccgkgwsockoocow8sg88s-051423333436
2026-Feb-26 05:19:16.297838
Starting new application.
2026-Feb-26 05:19:16.718432
[CMD]: docker exec dccg880ogw040c0k8oc4g804 bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/dccg880ogw040c0k8oc4g804/.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/dccg880ogw040c0k8oc4g804 -f /artifacts/dccg880ogw040c0k8oc4g804/docker/docker-compose.server.yml up -d'
2026-Feb-26 05:19:16.718432
Container api-o8kccgkgwsockoocow8sg88s-051842507751  Creating
2026-Feb-26 05:19:16.749387
Container api-o8kccgkgwsockoocow8sg88s-051842507751  Created
2026-Feb-26 05:19:16.749387
Container frontend-o8kccgkgwsockoocow8sg88s-051842512873  Creating
2026-Feb-26 05:19:16.774673
Container frontend-o8kccgkgwsockoocow8sg88s-051842512873  Created
2026-Feb-26 05:19:16.780931
Container api-o8kccgkgwsockoocow8sg88s-051842507751  Starting
2026-Feb-26 05:19:16.942353
Container api-o8kccgkgwsockoocow8sg88s-051842507751  Started
2026-Feb-26 05:19:16.942353
Container frontend-o8kccgkgwsockoocow8sg88s-051842512873  Starting
2026-Feb-26 05:19:17.120931
Container frontend-o8kccgkgwsockoocow8sg88s-051842512873  Started
2026-Feb-26 05:19:17.393526
New container started.