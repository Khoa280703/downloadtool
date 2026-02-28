2026-Feb-28 12:26:40.892142
Starting deployment of khoa280703/downloadtool:main-zoscg4oc04gkwkssg0kw8w8w to localhost.
2026-Feb-28 12:26:41.458019
Preparing container with helper image: ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Feb-28 12:26:41.757286
[CMD]: docker stop -t 30 ac8g4k4wkw4wsow088cwsgwo
2026-Feb-28 12:26:41.757286
Error response from daemon: No such container: ac8g4k4wkw4wsow088cwsgwo
2026-Feb-28 12:26:42.106724
[CMD]: docker run -d --network coolify --name ac8g4k4wkw4wsow088cwsgwo  --rm -v /var/run/docker.sock:/var/run/docker.sock ghcr.io/coollabsio/coolify-helper:1.0.12
2026-Feb-28 12:26:42.106724
77d7c85fe4d01c34d39ec026521943fc3c6d4b15369ef8471a1dfce7b2563bd7
2026-Feb-28 12:26:43.776925
[CMD]: docker exec ac8g4k4wkw4wsow088cwsgwo bash -c 'GIT_SSH_COMMAND="ssh -o ConnectTimeout=30 -p 22 -o Port=22 -o LogLevel=ERROR -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git ls-remote https://github.com/Khoa280703/downloadtool refs/heads/main'
2026-Feb-28 12:26:43.776925
27d4a0dcbf97d8fc90f54f4d0ef4bbb4491653db	refs/heads/main
2026-Feb-28 12:26:43.787525
----------------------------------------
2026-Feb-28 12:26:43.791163
Importing Khoa280703/downloadtool:main (commit sha 27d4a0dcbf97d8fc90f54f4d0ef4bbb4491653db) to /artifacts/ac8g4k4wkw4wsow088cwsgwo.
2026-Feb-28 12:26:44.151758
[CMD]: docker exec ac8g4k4wkw4wsow088cwsgwo bash -c 'git clone --depth=1 --recurse-submodules --shallow-submodules -b 'main' 'https://github.com/Khoa280703/downloadtool' '/artifacts/ac8g4k4wkw4wsow088cwsgwo' && cd '/artifacts/ac8g4k4wkw4wsow088cwsgwo' && if [ -f .gitmodules ]; then sed -i "s#git@\(.*\):#https://\1/#g" '/artifacts/ac8g4k4wkw4wsow088cwsgwo'/.gitmodules || true && git submodule sync && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git submodule update --init --recursive --depth=1; fi && cd '/artifacts/ac8g4k4wkw4wsow088cwsgwo' && GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" git lfs pull'
2026-Feb-28 12:26:44.151758
Cloning into '/artifacts/ac8g4k4wkw4wsow088cwsgwo'...
2026-Feb-28 12:26:47.040111
[CMD]: docker exec ac8g4k4wkw4wsow088cwsgwo bash -c 'cd /artifacts/ac8g4k4wkw4wsow088cwsgwo && git log -1 27d4a0dcbf97d8fc90f54f4d0ef4bbb4491653db --pretty=%B'
2026-Feb-28 12:26:47.040111
fix: avoid default better auth secret during frontend build
2026-Feb-28 12:26:52.146398
[CMD]: docker exec ac8g4k4wkw4wsow088cwsgwo bash -c 'test -f /artifacts/ac8g4k4wkw4wsow088cwsgwo/docker/Dockerfile.api && echo 'exists' || echo 'not found''
2026-Feb-28 12:26:52.146398
exists
2026-Feb-28 12:26:52.488265
[CMD]: docker exec ac8g4k4wkw4wsow088cwsgwo bash -c 'cat /artifacts/ac8g4k4wkw4wsow088cwsgwo/docker/Dockerfile.api'
2026-Feb-28 12:26:52.488265
# Dockerfile for API service deployment
2026-Feb-28 12:26:52.488265
# Builds the API server and related components without GPU support
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
# Stage 0: Build injector JS (embedded into api crate via include_str! at compile time)
2026-Feb-28 12:26:52.488265
FROM node:22-alpine AS js-builder
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
WORKDIR /app
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
RUN npm install -g pnpm
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
# Copy workspace manifests for pnpm resolution
2026-Feb-28 12:26:52.488265
COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Feb-28 12:26:52.488265
COPY packages/api-client/package.json ./packages/api-client/
2026-Feb-28 12:26:52.488265
COPY apps/injector/package.json ./apps/injector/
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
# Copy injector source and shared packages
2026-Feb-28 12:26:52.488265
COPY apps/injector/ ./apps/injector/
2026-Feb-28 12:26:52.488265
COPY packages/ ./packages/
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
# Install deps and build injector (produces dist/bm.js and dist/youtube-downloader.user.js)
2026-Feb-28 12:26:52.488265
RUN pnpm install --frozen-lockfile
2026-Feb-28 12:26:52.488265
RUN pnpm --filter @downloadtool/injector build
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
# Build extractor TypeScript to IIFE format (required by crates/extractor/build.rs)
2026-Feb-28 12:26:52.488265
COPY extractors/ ./extractors/
2026-Feb-28 12:26:52.488265
RUN mkdir -p extractors/dist && \
2026-Feb-28 12:26:52.488265
npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js && \
2026-Feb-28 12:26:52.488265
npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
# Stage 1: Rust builder
2026-Feb-28 12:26:52.488265
FROM rust:1.88-bookworm AS builder
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
WORKDIR /app
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
# Install dependencies
2026-Feb-28 12:26:52.488265
RUN apt-get update && apt-get install -y \
2026-Feb-28 12:26:52.488265
pkg-config \
2026-Feb-28 12:26:52.488265
libssl-dev \
2026-Feb-28 12:26:52.488265
&& rm -rf /var/lib/apt/lists/*
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
# Copy workspace configuration
2026-Feb-28 12:26:52.488265
COPY Cargo.toml ./
2026-Feb-28 12:26:52.488265
COPY Cargo.lock ./
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
# Copy all crates
2026-Feb-28 12:26:52.488265
COPY crates/ ./crates/
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
# Copy injector dist (required by include_str! in crates/api/src/routes/static_files.rs)
2026-Feb-28 12:26:52.488265
COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
# Copy extractor source + pre-built IIFE dist (built by js-builder stage)
2026-Feb-28 12:26:52.488265
COPY extractors/ ./extractors/
2026-Feb-28 12:26:52.488265
COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
# Build the release binary
2026-Feb-28 12:26:52.488265
RUN cargo build --release --bin api-server
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
# Stage 2: Runtime
2026-Feb-28 12:26:52.488265
FROM debian:bookworm-slim AS runtime
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
WORKDIR /app
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
# Install runtime dependencies
2026-Feb-28 12:26:52.488265
RUN apt-get update && apt-get install -y \
2026-Feb-28 12:26:52.488265
ca-certificates \
2026-Feb-28 12:26:52.488265
curl \
2026-Feb-28 12:26:52.488265
libssl3 \
2026-Feb-28 12:26:52.488265
&& rm -rf /var/lib/apt/lists/*
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
# Create non-root user
2026-Feb-28 12:26:52.488265
RUN useradd -m -u 1000 -s /bin/bash appuser
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
# Copy binary from builder
2026-Feb-28 12:26:52.488265
COPY --from=builder /app/target/release/api-server /usr/local/bin/
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
# Create directories
2026-Feb-28 12:26:52.488265
RUN mkdir -p /app/extractors /app/data && chown -R appuser:appuser /app
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
# Switch to non-root user
2026-Feb-28 12:26:52.488265
USER appuser
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
# Environment variables
2026-Feb-28 12:26:52.488265
ENV PORT=3068
2026-Feb-28 12:26:52.488265
ENV EXTRACTOR_DIR=/app/extractors
2026-Feb-28 12:26:52.488265
ENV RUST_LOG=info
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
# Expose port
2026-Feb-28 12:26:52.488265
EXPOSE 3068
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
# Health check
2026-Feb-28 12:26:52.488265
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Feb-28 12:26:52.488265
CMD curl -f http://localhost:3068/health || exit 1
2026-Feb-28 12:26:52.488265
2026-Feb-28 12:26:52.488265
# Run the server
2026-Feb-28 12:26:52.488265
CMD ["api-server"]
2026-Feb-28 12:26:52.856559
Added 42 ARG declarations to Dockerfile for service api (multi-stage build, added to 3 stages).
2026-Feb-28 12:26:53.194605
[CMD]: docker exec ac8g4k4wkw4wsow088cwsgwo bash -c 'test -f /artifacts/ac8g4k4wkw4wsow088cwsgwo/docker/Dockerfile.frontend && echo 'exists' || echo 'not found''
2026-Feb-28 12:26:53.194605
exists
2026-Feb-28 12:26:53.542285
[CMD]: docker exec ac8g4k4wkw4wsow088cwsgwo bash -c 'cat /artifacts/ac8g4k4wkw4wsow088cwsgwo/docker/Dockerfile.frontend'
2026-Feb-28 12:26:53.542285
# Dockerfile for frontend (SvelteKit Node server)
2026-Feb-28 12:26:53.542285
# Copy ALL source files BEFORE npm install so svelte-kit sync (prepare script)
2026-Feb-28 12:26:53.542285
# can find svelte.config.js and generate .svelte-kit/ correctly.
2026-Feb-28 12:26:53.542285
2026-Feb-28 12:26:53.542285
FROM node:22-alpine AS builder
2026-Feb-28 12:26:53.542285
2026-Feb-28 12:26:53.542285
WORKDIR /app
2026-Feb-28 12:26:53.542285
2026-Feb-28 12:26:53.542285
# Copy all frontend source files first (node_modules excluded via .dockerignore)
2026-Feb-28 12:26:53.542285
COPY frontend/ ./
2026-Feb-28 12:26:53.542285
2026-Feb-28 12:26:53.542285
# Install — prepare script runs svelte-kit sync with svelte.config.js available
2026-Feb-28 12:26:53.542285
RUN npm install
2026-Feb-28 12:26:53.542285
2026-Feb-28 12:26:53.542285
# Build-time public API URL (embedded into client bundle by Vite)
2026-Feb-28 12:26:53.542285
# Runtime env is too late for import.meta.env in browser bundle.
2026-Feb-28 12:26:53.542285
ARG VITE_API_URL
2026-Feb-28 12:26:53.542285
ENV VITE_API_URL=${VITE_API_URL}
2026-Feb-28 12:26:53.542285
RUN test -n "$VITE_API_URL" || (echo "VITE_API_URL build arg is required" && exit 1)
2026-Feb-28 12:26:53.542285
2026-Feb-28 12:26:53.542285
# Build via programmatic API — bypasses vite.config.ts loading (esbuild issues on Alpine)
2026-Feb-28 12:26:53.542285
RUN node build-docker.mjs
2026-Feb-28 12:26:53.542285
2026-Feb-28 12:26:53.542285
# Runtime
2026-Feb-28 12:26:53.542285
FROM node:22-alpine AS runtime
2026-Feb-28 12:26:53.542285
2026-Feb-28 12:26:53.542285
WORKDIR /app
2026-Feb-28 12:26:53.542285
2026-Feb-28 12:26:53.542285
COPY --from=builder /app/build ./build
2026-Feb-28 12:26:53.542285
COPY --from=builder /app/package.json ./
2026-Feb-28 12:26:53.542285
COPY --from=builder /app/package-lock.json ./
2026-Feb-28 12:26:53.542285
2026-Feb-28 12:26:53.542285
# Runtime needs server-side deps (better-auth, pg) used by hooks/routes
2026-Feb-28 12:26:53.542285
RUN npm ci --omit=dev
2026-Feb-28 12:26:53.542285
2026-Feb-28 12:26:53.542285
ENV PORT=5168
2026-Feb-28 12:26:53.542285
ENV HOST=0.0.0.0
2026-Feb-28 12:26:53.542285
2026-Feb-28 12:26:53.542285
EXPOSE 5168
2026-Feb-28 12:26:53.542285
2026-Feb-28 12:26:53.542285
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
2026-Feb-28 12:26:53.542285
CMD wget -qO- http://127.0.0.1:5168 || exit 1
2026-Feb-28 12:26:53.542285
2026-Feb-28 12:26:53.542285
CMD ["node", "build"]
2026-Feb-28 12:26:53.895541
Added 28 ARG declarations to Dockerfile for service frontend (multi-stage build, added to 2 stages).
2026-Feb-28 12:26:53.901149
Pulling & building required images.
2026-Feb-28 12:26:53.923245
Creating build-time .env file in /artifacts (outside Docker context).
2026-Feb-28 12:26:54.269198
Adding build arguments to Docker Compose build command.
2026-Feb-28 12:26:54.762492
[CMD]: docker exec ac8g4k4wkw4wsow088cwsgwo bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/build-time.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/ac8g4k4wkw4wsow088cwsgwo -f /artifacts/ac8g4k4wkw4wsow088cwsgwo/docker/docker-compose.server.yml build --pull --build-arg COOLIFY_URL --build-arg COOLIFY_FQDN --build-arg SERVICE_URL_API --build-arg SERVICE_FQDN_FRONTEND --build-arg SERVICE_FQDN_API --build-arg POSTGRES_PASSWORD --build-arg SERVICE_URL_FRONTEND --build-arg BETTER_AUTH_SECRET --build-arg WHOP_WEBHOOK_SECRET --build-arg ORIGIN --build-arg BETTER_AUTH_TRUSTED_ORIGINS --build-arg GOOGLE_CLIENT_ID --build-arg GOOGLE_CLIENT_SECRET --build-arg WHOP_PLAN_ID --build-arg COOLIFY_BUILD_SECRETS_HASH=a21afc02cd414908989fcb2d54f5c09f1d5a4fa0c1e8a2e802b2b03e2c30bacd'
2026-Feb-28 12:26:54.762492
#1 [internal] load local bake definitions
2026-Feb-28 12:26:54.964411
#1 reading from stdin 2.34kB done
2026-Feb-28 12:26:54.964411
#1 DONE 0.0s
2026-Feb-28 12:26:54.964411
2026-Feb-28 12:26:54.964411
#2 [api internal] load build definition from Dockerfile.api
2026-Feb-28 12:26:54.964411
#2 transferring dockerfile: 3.72kB done
2026-Feb-28 12:26:54.964411
#2 DONE 0.0s
2026-Feb-28 12:26:54.964411
2026-Feb-28 12:26:54.964411
#3 [frontend internal] load build definition from Dockerfile.frontend
2026-Feb-28 12:26:54.964411
#3 transferring dockerfile: 1.94kB done
2026-Feb-28 12:26:54.964411
#3 DONE 0.0s
2026-Feb-28 12:26:54.964411
2026-Feb-28 12:26:54.964411
#4 [frontend internal] load metadata for docker.io/library/node:22-alpine
2026-Feb-28 12:26:55.912997
#4 ...
2026-Feb-28 12:26:55.912997
2026-Feb-28 12:26:55.912997
#5 [api internal] load metadata for docker.io/library/debian:bookworm-slim
2026-Feb-28 12:26:55.912997
#5 DONE 1.1s
2026-Feb-28 12:26:56.015092
#6 [api internal] load metadata for docker.io/library/rust:1.88-bookworm
2026-Feb-28 12:26:56.015092
#6 DONE 1.1s
2026-Feb-28 12:26:56.015092
2026-Feb-28 12:26:56.015092
#4 [api internal] load metadata for docker.io/library/node:22-alpine
2026-Feb-28 12:26:56.015092
#4 DONE 1.1s
2026-Feb-28 12:26:56.015092
2026-Feb-28 12:26:56.015092
#7 [api internal] load .dockerignore
2026-Feb-28 12:26:56.015092
#7 transferring context: 341B done
2026-Feb-28 12:26:56.015092
#7 DONE 0.0s
2026-Feb-28 12:26:56.015092
2026-Feb-28 12:26:56.015092
#8 [api runtime 1/6] FROM docker.io/library/debian:bookworm-slim@sha256:74d56e3931e0d5a1dd51f8c8a2466d21de84a271cd3b5a733b803aa91abf4421
2026-Feb-28 12:26:56.015092
#8 resolve docker.io/library/debian:bookworm-slim@sha256:74d56e3931e0d5a1dd51f8c8a2466d21de84a271cd3b5a733b803aa91abf4421 0.0s done
2026-Feb-28 12:26:56.015092
#8 DONE 0.0s
2026-Feb-28 12:26:56.015092
2026-Feb-28 12:26:56.015092
#9 [api builder 1/6] FROM docker.io/library/node:22-alpine@sha256:e4bf2a82ad0a4037d28035ae71529873c069b13eb0455466ae0bc13363826e34
2026-Feb-28 12:26:56.015092
#9 resolve docker.io/library/node:22-alpine@sha256:e4bf2a82ad0a4037d28035ae71529873c069b13eb0455466ae0bc13363826e34 0.0s done
2026-Feb-28 12:26:56.015092
#9 DONE 0.0s
2026-Feb-28 12:26:56.015092
2026-Feb-28 12:26:56.015092
#10 [api builder  1/10] FROM docker.io/library/rust:1.88-bookworm@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0
2026-Feb-28 12:26:56.015092
#10 resolve docker.io/library/rust:1.88-bookworm@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0 0.0s done
2026-Feb-28 12:26:56.015092
#10 DONE 0.0s
2026-Feb-28 12:26:56.015092
2026-Feb-28 12:26:56.015092
#11 [api runtime 2/6] WORKDIR /app
2026-Feb-28 12:26:56.015092
#11 CACHED
2026-Feb-28 12:26:56.015092
2026-Feb-28 12:26:56.015092
#12 [api internal] load build context
2026-Feb-28 12:26:56.015092
#12 transferring context: 608.57kB 0.0s done
2026-Feb-28 12:26:56.191257
#12 DONE 0.0s
2026-Feb-28 12:26:56.191257
2026-Feb-28 12:26:56.191257
#13 [frontend internal] load build context
2026-Feb-28 12:26:56.191257
#13 transferring context: 1.90MB 0.0s done
2026-Feb-28 12:26:56.191257
#13 DONE 0.0s
2026-Feb-28 12:26:56.191257
2026-Feb-28 12:26:56.191257
#14 [api js-builder 12/12] RUN mkdir -p extractors/dist &&     npx --yes esbuild extractors/types.ts --bundle --format=iife --global-name=types --platform=neutral --target=es2020 --outfile=extractors/dist/types.js &&     npx --yes esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
2026-Feb-28 12:26:56.191257
#14 CACHED
2026-Feb-28 12:26:56.191257
2026-Feb-28 12:26:56.191257
#15 [api js-builder  5/12] COPY packages/api-client/package.json ./packages/api-client/
2026-Feb-28 12:26:56.191257
#15 CACHED
2026-Feb-28 12:26:56.191257
2026-Feb-28 12:26:56.191257
#16 [api js-builder  3/12] RUN npm install -g pnpm
2026-Feb-28 12:26:56.191257
#16 CACHED
2026-Feb-28 12:26:56.191257
2026-Feb-28 12:26:56.191257
#17 [api builder  7/10] COPY --from=js-builder /app/apps/injector/dist ./apps/injector/dist
2026-Feb-28 12:26:56.191257
#17 CACHED
2026-Feb-28 12:26:56.191257
2026-Feb-28 12:26:56.191257
#18 [api js-builder  4/12] COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
2026-Feb-28 12:26:56.191257
#18 CACHED
2026-Feb-28 12:26:56.191257
2026-Feb-28 12:26:56.191257
#19 [api builder  6/10] COPY crates/ ./crates/
2026-Feb-28 12:26:56.191257
#19 CACHED
2026-Feb-28 12:26:56.191257
2026-Feb-28 12:26:56.191257
#20 [api builder  8/10] COPY extractors/ ./extractors/
2026-Feb-28 12:26:56.191257
#20 CACHED
2026-Feb-28 12:26:56.191257
2026-Feb-28 12:26:56.191257
#21 [api builder  3/10] RUN apt-get update && apt-get install -y     pkg-config     libssl-dev     && rm -rf /var/lib/apt/lists/*
2026-Feb-28 12:26:56.191257
#21 CACHED
2026-Feb-28 12:26:56.191257
2026-Feb-28 12:26:56.191257
#22 [api js-builder 10/12] RUN pnpm --filter @downloadtool/injector build
2026-Feb-28 12:26:56.191257
#22 CACHED
2026-Feb-28 12:26:56.191257
2026-Feb-28 12:26:56.191257
#23 [api builder  5/10] COPY Cargo.lock ./
2026-Feb-28 12:26:56.191257
#23 CACHED
2026-Feb-28 12:26:56.191257
2026-Feb-28 12:26:56.191257
#24 [api js-builder  7/12] COPY apps/injector/ ./apps/injector/
2026-Feb-28 12:26:56.191257
#24 CACHED
2026-Feb-28 12:26:56.191257
2026-Feb-28 12:26:56.191257
#25 [api js-builder  9/12] RUN pnpm install --frozen-lockfile
2026-Feb-28 12:26:56.191257
#25 CACHED
2026-Feb-28 12:26:56.191257
2026-Feb-28 12:26:56.191257
#26 [api builder  4/10] COPY Cargo.toml ./
2026-Feb-28 12:26:56.191257
#26 CACHED
2026-Feb-28 12:26:56.191257
2026-Feb-28 12:26:56.191257
#27 [api builder  2/10] WORKDIR /app
2026-Feb-28 12:26:56.191257
#27 CACHED
2026-Feb-28 12:26:56.191257
2026-Feb-28 12:26:56.191257
#28 [api js-builder  6/12] COPY apps/injector/package.json ./apps/injector/
2026-Feb-28 12:26:56.191257
#28 CACHED
2026-Feb-28 12:26:56.191257
2026-Feb-28 12:26:56.191257
#29 [api js-builder 11/12] COPY extractors/ ./extractors/
2026-Feb-28 12:26:56.191257
#29 CACHED
2026-Feb-28 12:26:56.191257
2026-Feb-28 12:26:56.191257
#30 [api js-builder  8/12] COPY packages/ ./packages/
2026-Feb-28 12:26:56.191257
#30 CACHED
2026-Feb-28 12:26:56.191257
2026-Feb-28 12:26:56.191257
#31 [api builder  9/10] COPY --from=js-builder /app/extractors/dist ./extractors/dist/
2026-Feb-28 12:26:56.191257
#31 CACHED
2026-Feb-28 12:26:56.191257
2026-Feb-28 12:26:56.191257
#32 [api builder 2/6] WORKDIR /app
2026-Feb-28 12:26:56.191257
#32 CACHED
2026-Feb-28 12:26:56.191257
2026-Feb-28 12:26:56.191257
#33 [frontend builder 3/6] COPY frontend/ ./
2026-Feb-28 12:26:56.202705
#33 DONE 0.2s
2026-Feb-28 12:26:56.202705
2026-Feb-28 12:26:56.202705
#34 [api builder 10/10] RUN cargo build --release --bin api-server
2026-Feb-28 12:26:56.383991
#34 0.346     Updating crates.io index
2026-Feb-28 12:27:01.524551
#34 ...
2026-Feb-28 12:27:01.524551
2026-Feb-28 12:27:01.524551
#35 [frontend builder 4/6] RUN npm install
2026-Feb-28 12:27:01.524551
#35 3.986
2026-Feb-28 12:27:01.524551
#35 3.986 > frontend@0.0.1 prepare
2026-Feb-28 12:27:01.524551
#35 3.986 > svelte-kit sync || echo ''
2026-Feb-28 12:27:01.524551
#35 3.986
2026-Feb-28 12:27:01.524551
#35 4.868
2026-Feb-28 12:27:01.524551
#35 4.868 added 212 packages, and audited 213 packages in 5s
2026-Feb-28 12:27:01.524551
#35 4.868
2026-Feb-28 12:27:01.524551
#35 4.868 41 packages are looking for funding
2026-Feb-28 12:27:01.524551
#35 4.868   run `npm fund` for details
2026-Feb-28 12:27:01.524551
#35 4.883
2026-Feb-28 12:27:01.524551
#35 4.883 3 vulnerabilities (2 low, 1 moderate)
2026-Feb-28 12:27:01.524551
#35 4.883
2026-Feb-28 12:27:01.524551
#35 4.883 To address all issues, run:
2026-Feb-28 12:27:01.524551
#35 4.883   npm audit fix
2026-Feb-28 12:27:01.524551
#35 4.883
2026-Feb-28 12:27:01.524551
#35 4.883 Run `npm audit` for details.
2026-Feb-28 12:27:01.524551
#35 4.884 npm notice
2026-Feb-28 12:27:01.524551
#35 4.884 npm notice New major version of npm available! 10.9.4 -> 11.11.0
2026-Feb-28 12:27:01.524551
#35 4.884 npm notice Changelog: https://github.com/npm/cli/releases/tag/v11.11.0
2026-Feb-28 12:27:01.524551
#35 4.884 npm notice To update run: npm install -g npm@11.11.0
2026-Feb-28 12:27:01.524551
#35 4.884 npm notice
2026-Feb-28 12:27:01.524551
#35 DONE 5.2s
2026-Feb-28 12:27:01.524551
2026-Feb-28 12:27:01.524551
#36 [frontend builder 5/6] RUN test -n "https://api-download.khoadangbui.online" || (echo "VITE_API_URL build arg is required" && exit 1)
2026-Feb-28 12:27:01.610234
#36 DONE 0.2s
2026-Feb-28 12:27:01.610234
2026-Feb-28 12:27:01.610234
#37 [api runtime 3/6] RUN apt-get update && apt-get install -y     ca-certificates     curl     libssl3     && rm -rf /var/lib/apt/lists/*
2026-Feb-28 12:27:01.610234
#37 0.643 Get:1 http://deb.debian.org/debian bookworm InRelease [151 kB]
2026-Feb-28 12:27:01.610234
#37 1.219 Get:2 http://deb.debian.org/debian bookworm-updates InRelease [55.4 kB]
2026-Feb-28 12:27:01.610234
#37 1.429 Get:3 http://deb.debian.org/debian-security bookworm-security InRelease [48.0 kB]
2026-Feb-28 12:27:01.610234
#37 1.636 Get:4 http://deb.debian.org/debian bookworm/main amd64 Packages [8792 kB]
2026-Feb-28 12:27:01.610234
#37 4.258 Get:5 http://deb.debian.org/debian bookworm-updates/main amd64 Packages [6924 B]
2026-Feb-28 12:27:01.610234
#37 4.438 Get:6 http://deb.debian.org/debian-security bookworm-security/main amd64 Packages [297 kB]
2026-Feb-28 12:27:01.610234
#37 4.927 Fetched 9350 kB in 5s (2010 kB/s)
2026-Feb-28 12:27:01.610234
#37 4.927 Reading package lists...
2026-Feb-28 12:27:01.610234
#37 5.359 Reading package lists...
2026-Feb-28 12:27:01.782827
2026-Feb-28 12:27:01.911138
#37 5.813 Building dependency tree...
2026-Feb-28 12:27:01.911138
#37 5.927 Reading state information...
2026-Feb-28 12:27:02.039912
#37 6.056 The following additional packages will be installed:
2026-Feb-28 12:27:02.157602
#37 6.056   krb5-locales libbrotli1 libcurl4 libgssapi-krb5-2 libk5crypto3 libkeyutils1
2026-Feb-28 12:27:02.157602
#37 6.056   libkrb5-3 libkrb5support0 libldap-2.5-0 libldap-common libnghttp2-14 libpsl5
2026-Feb-28 12:27:02.157602
#37 6.056   librtmp1 libsasl2-2 libsasl2-modules libsasl2-modules-db libssh2-1 openssl
2026-Feb-28 12:27:02.157602
#37 6.056   publicsuffix
2026-Feb-28 12:27:02.157602
#37 6.057 Suggested packages:
2026-Feb-28 12:27:02.157602
#37 6.057   krb5-doc krb5-user libsasl2-modules-gssapi-mit
2026-Feb-28 12:27:02.157602
#37 6.057   | libsasl2-modules-gssapi-heimdal libsasl2-modules-ldap libsasl2-modules-otp
2026-Feb-28 12:27:02.157602
#37 6.057   libsasl2-modules-sql
2026-Feb-28 12:27:02.157602
#37 6.173 The following NEW packages will be installed:
2026-Feb-28 12:27:02.308649
#37 6.174   ca-certificates curl krb5-locales libbrotli1 libcurl4 libgssapi-krb5-2
2026-Feb-28 12:27:02.308649
#37 6.174   libk5crypto3 libkeyutils1 libkrb5-3 libkrb5support0 libldap-2.5-0
2026-Feb-28 12:27:02.308649
#37 6.174   libldap-common libnghttp2-14 libpsl5 librtmp1 libsasl2-2 libsasl2-modules
2026-Feb-28 12:27:02.308649
#37 6.174   libsasl2-modules-db libssh2-1 libssl3 openssl publicsuffix
2026-Feb-28 12:27:02.548502
#37 6.564 0 upgraded, 22 newly installed, 0 to remove and 0 not upgraded.
2026-Feb-28 12:27:02.548502
#37 6.564 Need to get 6111 kB of archives.
2026-Feb-28 12:27:02.548502
#37 6.564 After this operation, 15.7 MB of additional disk space will be used.
2026-Feb-28 12:27:02.548502
#37 6.564 Get:1 http://deb.debian.org/debian-security bookworm-security/main amd64 libssl3 amd64 3.0.18-1~deb12u2 [2030 kB]
2026-Feb-28 12:27:04.211140
#37 8.227 Get:2 http://deb.debian.org/debian-security bookworm-security/main amd64 openssl amd64 3.0.18-1~deb12u2 [1433 kB]
2026-Feb-28 12:27:04.588408
#37 8.604 Get:3 http://deb.debian.org/debian bookworm/main amd64 ca-certificates all 20230311+deb12u1 [155 kB]
2026-Feb-28 12:27:04.743200
#37 8.605 Get:4 http://deb.debian.org/debian bookworm/main amd64 krb5-locales all 1.20.1-2+deb12u4 [63.4 kB]
2026-Feb-28 12:27:04.743200
#37 8.606 Get:5 http://deb.debian.org/debian bookworm/main amd64 libbrotli1 amd64 1.0.9-2+b6 [275 kB]
2026-Feb-28 12:27:04.743200
#37 8.608 Get:6 http://deb.debian.org/debian bookworm/main amd64 libkrb5support0 amd64 1.20.1-2+deb12u4 [33.2 kB]
2026-Feb-28 12:27:04.743200
#37 8.608 Get:7 http://deb.debian.org/debian bookworm/main amd64 libk5crypto3 amd64 1.20.1-2+deb12u4 [79.8 kB]
2026-Feb-28 12:27:04.743200
#37 8.609 Get:8 http://deb.debian.org/debian bookworm/main amd64 libkeyutils1 amd64 1.6.3-2 [8808 B]
2026-Feb-28 12:27:04.775278
#37 8.791 Get:9 http://deb.debian.org/debian bookworm/main amd64 libkrb5-3 amd64 1.20.1-2+deb12u4 [334 kB]
2026-Feb-28 12:27:04.960798
#37 8.977 Get:10 http://deb.debian.org/debian bookworm/main amd64 libgssapi-krb5-2 amd64 1.20.1-2+deb12u4 [135 kB]
2026-Feb-28 12:27:05.352067
#37 9.215 Get:11 http://deb.debian.org/debian bookworm/main amd64 libsasl2-modules-db amd64 2.1.28+dfsg-10 [20.3 kB]
2026-Feb-28 12:27:05.352067
#37 9.216 Get:12 http://deb.debian.org/debian bookworm/main amd64 libsasl2-2 amd64 2.1.28+dfsg-10 [59.7 kB]
2026-Feb-28 12:27:05.352067
#37 9.217 Get:13 http://deb.debian.org/debian bookworm/main amd64 libldap-2.5-0 amd64 2.5.13+dfsg-5 [183 kB]
2026-Feb-28 12:27:05.352067
#37 9.218 Get:14 http://deb.debian.org/debian bookworm/main amd64 libnghttp2-14 amd64 1.52.0-1+deb12u2 [73.0 kB]
2026-Feb-28 12:27:05.384675
#37 9.401 Get:15 http://deb.debian.org/debian bookworm/main amd64 libpsl5 amd64 0.21.2-1 [58.7 kB]
2026-Feb-28 12:27:05.567235
#37 9.583 Get:16 http://deb.debian.org/debian bookworm/main amd64 librtmp1 amd64 2.4+20151223.gitfa8646d.1-2+b2 [60.8 kB]
2026-Feb-28 12:27:05.754101
#37 9.770 Get:17 http://deb.debian.org/debian bookworm/main amd64 libssh2-1 amd64 1.10.0-3+b1 [179 kB]
2026-Feb-28 12:27:05.873696
#37 9.773 Get:18 http://deb.debian.org/debian bookworm/main amd64 libcurl4 amd64 7.88.1-10+deb12u14 [392 kB]
2026-Feb-28 12:27:05.873696
#37 9.777 Get:19 http://deb.debian.org/debian bookworm/main amd64 curl amd64 7.88.1-10+deb12u14 [316 kB]
2026-Feb-28 12:27:06.018100
#37 9.962 Get:20 http://deb.debian.org/debian bookworm/main amd64 libldap-common all 2.5.13+dfsg-5 [29.3 kB]
2026-Feb-28 12:27:06.018100
#37 9.963 Get:21 http://deb.debian.org/debian bookworm/main amd64 libsasl2-modules amd64 2.1.28+dfsg-10 [66.6 kB]
2026-Feb-28 12:27:06.018100
#37 9.964 Get:22 http://deb.debian.org/debian bookworm/main amd64 publicsuffix all 20230209.2326-1 [126 kB]
2026-Feb-28 12:27:06.018100
#37 CANCELED
2026-Feb-28 12:27:06.018100
2026-Feb-28 12:27:06.018100
#38 [frontend builder 6/6] RUN node build-docker.mjs
2026-Feb-28 12:27:06.018100
#38 1.092 The following Vite config options will be overridden by SvelteKit:
2026-Feb-28 12:27:06.018100
#38 1.092   - build.outDir
2026-Feb-28 12:27:06.018100
#38 1.120 vite v6.4.1 building SSR bundle for production...
2026-Feb-28 12:27:06.018100
#38 1.138 transforming...
2026-Feb-28 12:27:06.018100
#38 3.411 "optionsMiddleware" is imported from external module "@better-auth/core/api" but never used in "node_modules/better-auth/dist/api/index.mjs" and "node_modules/better-auth/dist/plugins/index.mjs".
2026-Feb-28 12:27:06.018100
#38 3.411 "getTelemetryAuthConfig" is imported from external module "@better-auth/telemetry" but never used in "node_modules/better-auth/dist/index.mjs".
2026-Feb-28 12:27:06.018100
#38 3.412 ✓ 437 modules transformed.
2026-Feb-28 12:27:06.018100
#38 3.533 rendering chunks...
2026-Feb-28 12:27:06.018100
#38 4.263 2026-02-28T12:27:05.872Z WARN [Better Auth]: [better-auth] Base URL could not be determined. Please set a valid base URL using the baseURL config option or the BETTER_AUTH_URL environment variable. Without this, callbacks and redirects may not work correctly.
2026-Feb-28 12:27:06.018100
#38 4.285
2026-Feb-28 12:27:06.018100
#38 4.285 node:internal/event_target:1122
2026-Feb-28 12:27:06.018100
#38 4.285   process.nextTick(() => { throw err; });
2026-Feb-28 12:27:06.018100
#38 4.285                            ^
2026-Feb-28 12:27:06.018100
#38 4.285 [Error [BetterAuthError]: You are using the default secret. Please set `BETTER_AUTH_SECRET` in your environment variables or pass `secret` in your auth config.]
2026-Feb-28 12:27:06.018100
#38 4.285
2026-Feb-28 12:27:06.018100
#38 4.285 Node.js v22.22.0
2026-Feb-28 12:27:06.018100
#38 ERROR: process "/bin/sh -c node build-docker.mjs" did not complete successfully: exit code: 1
2026-Feb-28 12:27:06.018100
2026-Feb-28 12:27:06.018100
#34 [api builder 10/10] RUN cargo build --release --bin api-server
2026-Feb-28 12:27:06.050696
#34 CANCELED
2026-Feb-28 12:27:06.050696
------
2026-Feb-28 12:27:06.050696
> [frontend builder 6/6] RUN node build-docker.mjs:
2026-Feb-28 12:27:06.050696
3.412 ✓ 437 modules transformed.
2026-Feb-28 12:27:06.050696
3.533 rendering chunks...
2026-Feb-28 12:27:06.050696
4.263 2026-02-28T12:27:05.872Z WARN [Better Auth]: [better-auth] Base URL could not be determined. Please set a valid base URL using the baseURL config option or the BETTER_AUTH_URL environment variable. Without this, callbacks and redirects may not work correctly.
2026-Feb-28 12:27:06.050696
4.285
2026-Feb-28 12:27:06.050696
4.285 node:internal/event_target:1122
2026-Feb-28 12:27:06.050696
4.285   process.nextTick(() => { throw err; });
2026-Feb-28 12:27:06.050696
4.285                            ^
2026-Feb-28 12:27:06.050696
4.285 [Error [BetterAuthError]: You are using the default secret. Please set `BETTER_AUTH_SECRET` in your environment variables or pass `secret` in your auth config.]
2026-Feb-28 12:27:06.050696
4.285
2026-Feb-28 12:27:06.050696
4.285 Node.js v22.22.0
2026-Feb-28 12:27:06.050696
------
2026-Feb-28 12:27:06.053688
Dockerfile.frontend:36
2026-Feb-28 12:27:06.053688
2026-Feb-28 12:27:06.053688
--------------------
2026-Feb-28 12:27:06.053688
2026-Feb-28 12:27:06.053688
34 |
2026-Feb-28 12:27:06.053688
2026-Feb-28 12:27:06.053688
35 |     # Build via programmatic API — bypasses vite.config.ts loading (esbuild issues on Alpine)
2026-Feb-28 12:27:06.053688
2026-Feb-28 12:27:06.053688
36 | >>> RUN node build-docker.mjs
2026-Feb-28 12:27:06.053688
2026-Feb-28 12:27:06.053688
37 |
2026-Feb-28 12:27:06.053688
2026-Feb-28 12:27:06.053688
38 |     # Runtime
2026-Feb-28 12:27:06.053688
2026-Feb-28 12:27:06.053688
--------------------
2026-Feb-28 12:27:06.053688
2026-Feb-28 12:27:06.053688
target frontend: failed to solve: process "/bin/sh -c node build-docker.mjs" did not complete successfully: exit code: 1
2026-Feb-28 12:27:06.056167
exit status 1
2026-Feb-28 12:27:06.088158
========================================
2026-Feb-28 12:27:06.092817
Deployment failed: Command execution failed (exit code 1): docker exec ac8g4k4wkw4wsow088cwsgwo bash -c 'COOLIFY_BRANCH=main COOLIFY_RESOURCE_UUID=o8kccgkgwsockoocow8sg88s  docker compose --env-file /artifacts/build-time.env --project-name o8kccgkgwsockoocow8sg88s --project-directory /artifacts/ac8g4k4wkw4wsow088cwsgwo -f /artifacts/ac8g4k4wkw4wsow088cwsgwo/docker/docker-compose.server.yml build --pull --build-arg COOLIFY_URL --build-arg COOLIFY_FQDN --build-arg SERVICE_URL_API --build-arg SERVICE_FQDN_FRONTEND --build-arg SERVICE_FQDN_API --build-arg POSTGRES_PASSWORD --build-arg SERVICE_URL_FRONTEND --build-arg BETTER_AUTH_SECRET --build-arg WHOP_WEBHOOK_SECRET --build-arg ORIGIN --build-arg BETTER_AUTH_TRUSTED_ORIGINS --build-arg GOOGLE_CLIENT_ID --build-arg GOOGLE_CLIENT_SECRET --build-arg WHOP_PLAN_ID --build-arg COOLIFY_BUILD_SECRETS_HASH=a21afc02cd414908989fcb2d54f5c09f1d5a4fa0c1e8a2e802b2b03e2c30bacd'
2026-Feb-28 12:27:06.092817
Error: Dockerfile.frontend:36
2026-Feb-28 12:27:06.092817
2026-Feb-28 12:27:06.092817
--------------------
2026-Feb-28 12:27:06.092817
2026-Feb-28 12:27:06.092817
34 |
2026-Feb-28 12:27:06.092817
2026-Feb-28 12:27:06.092817
35 |     # Build via programmatic API — bypasses vite.config.ts loading (esbuild issues on Alpine)
2026-Feb-28 12:27:06.092817
2026-Feb-28 12:27:06.092817
36 | >>> RUN node build-docker.mjs
2026-Feb-28 12:27:06.092817
2026-Feb-28 12:27:06.092817
37 |
2026-Feb-28 12:27:06.092817
2026-Feb-28 12:27:06.092817
38 |     # Runtime
2026-Feb-28 12:27:06.092817
2026-Feb-28 12:27:06.092817
--------------------
2026-Feb-28 12:27:06.092817
2026-Feb-28 12:27:06.092817
target frontend: failed to solve: process "/bin/sh -c node build-docker.mjs" did not complete successfully: exit code: 1
2026-Feb-28 12:27:06.092817
2026-Feb-28 12:27:06.092817
exit status 1
2026-Feb-28 12:27:06.097722
Error type: RuntimeException
2026-Feb-28 12:27:06.104659
Error code: 0
2026-Feb-28 12:27:06.111109
Location: /var/www/html/app/Traits/ExecuteRemoteCommand.php:243
2026-Feb-28 12:27:06.117259
Stack trace (first 5 lines):
2026-Feb-28 12:27:06.122872
#0 /var/www/html/app/Traits/ExecuteRemoteCommand.php(104): App\Jobs\ApplicationDeploymentJob->executeCommandWithProcess()
2026-Feb-28 12:27:06.128537
#1 /var/www/html/vendor/laravel/framework/src/Illuminate/Collections/Traits/EnumeratesValues.php(272): App\Jobs\ApplicationDeploymentJob->{closure:App\Traits\ExecuteRemoteCommand::execute_remote_command():71}()
2026-Feb-28 12:27:06.133972
#2 /var/www/html/app/Traits/ExecuteRemoteCommand.php(71): Illuminate\Support\Collection->each()
2026-Feb-28 12:27:06.139407
#3 /var/www/html/app/Jobs/ApplicationDeploymentJob.php(730): App\Jobs\ApplicationDeploymentJob->execute_remote_command()
2026-Feb-28 12:27:06.144606
#4 /var/www/html/app/Jobs/ApplicationDeploymentJob.php(467): App\Jobs\ApplicationDeploymentJob->deploy_docker_compose_buildpack()
2026-Feb-28 12:27:06.149379
========================================