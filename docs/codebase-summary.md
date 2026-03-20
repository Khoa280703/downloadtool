# Codebase Summary

**Generated:** 2026-03-18
**Total Files:** 404 | **Total Tokens:** ~810,000 (repomix)

## Project Overview

A high-performance video downloader platform supporting YouTube and other platforms with anti-bot protection, fMP4 remuxing, and full-speed CDN downloads via YouTube n-parameter transformation.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                   Frontend (Svelte/SvelteKit)               │
│           (/frontend) - Web UI for downloads                │
└──────────────┬────────────────────────────────┬─────────────┘
               │                                │
       ┌───────▼────────┐           ┌───────────▼──────┐
       │   API Layer    │           │  WebSocket Stream│
       │ (crates/api)   │           │   (crates/proxy) │
       │  - Extract     │           │  - Anti-bot      │
       │  - Batch       │           │  - Throttle      │
       │  - Stream      │           │  - Cookie/Header │
       │  - Jobs        │           │  - Proxy Rotation│
       └────┬──────┬────┘           └──────────────────┘
            │      │
    ┌───────▼──┐   └──────────────────────────┐
    │Extractor │                  ┌────────────▼───────────┐
    │(crates/  │                  │  Muxer (crates/muxer)  │
    │extractor)│                  │  - fMP4 Format         │
    │          │                  │  - Stream Fetcher      │
    │- yt-dlp  │                  │  - Codec Config        │
    │- Hot     │                  │  - Mux Router          │
    │  reload  │                  └────────────────────────┘
    │- Pooling │                             │
    └──────────┘                  ┌──────────▼─────────────┐
                                  │  Worker (crates/worker) │
                                  │  - Concurrent job pool  │
                                  │  - Artifact upload      │
                                  └─────────────────────────┘
```

## Key Components

### 1. **Frontend** (`/frontend`)
- **Framework:** SvelteKit with Svelte components
- **i18n System:** Paraglide JS (24+ languages, 384 keys, `frontend/messages/`)
- **Key Components:**
  - `UrlInput.svelte` - URL input handling
  - `DownloadBtn.svelte` - Unified download (direct + mux job, **NEW 2026-03-16**)
  - `AppIcon.svelte` - SVG icon system (60+ Lucide icons, quality badges, **NEW 2026-03-16**)
  - `BatchProgress.svelte` - Real-time SSE progress tracking
  - `FormatPicker.svelte` - Stream quality/format selection
  - `CookieConsent.svelte` - Privacy compliance
  - `AdBanner.svelte` & `InterstitialAd.svelte` - Ad integration
  - `PlaylistProgress.svelte` - Playlist job status + item list (**NEW 2026-03-18**)
- **API Modules:**
  - `lib/api.ts` - Extraction & download API client
  - `lib/playlist-job-api.ts` - **NEW (2026-03-18):** Client for playlist job orchestration (create, status, SSE events, cancel)
- **Routes:**
  - `routes/api/proxy/playlist-jobs/+server.ts` - BFF proxy for POST create (NEW 2026-03-18)
  - `routes/api/proxy/playlist-jobs/[jobId]/+server.ts` - BFF proxy for GET status (NEW 2026-03-18)
  - `routes/api/proxy/playlist-jobs/[jobId]/events/+server.ts` - SSE event proxy (NEW 2026-03-18)
  - `routes/api/proxy/playlist-jobs/[jobId]/cancel/+server.ts` - BFF proxy for POST cancel (NEW 2026-03-18)
- **Features:** Responsive design, i18n support, real-time progress, ad monetization, backend playlist orchestration

### 2. **API Layer** (`crates/api`)
- **Entry Point:** `main.rs` - HTTP server (Tokio-based), PostgreSQL pool setup
- **Routes:**
  - `extract.rs` - Extract video metadata & streams
  - `batch.rs` - Batch download operations with SSE (Server-Sent Events) streaming
  - `stream.rs` - WebSocket stream handler with 422 WebM validation
  - `transcode.rs` - GPU transcoding requests
  - `whop_webhook.rs` - **NEW (2026-02-28):** Whop subscription webhook handler with HMAC-SHA256 signature verification
  - `openapi.rs` - OpenAPI/Swagger spec generation (utoipa)
  - `static_files.rs` - Frontend static file serving with cache headers
- **Auth System** (`auth/`)  - **NEW (2026-02-28)**
  - `jwt_claims.rs` - JWT token structure (user_id, tier, expiration)
  - `jwt_middleware.rs` - Axum extractor for JWT validation and user context injection
  - `user_tier.rs` - User tier enumeration (Free, Pro, Premium) with rate limit quotas
- **Config:** Environment variables (DATABASE_URL, JWT_SECRET, WHOP_API_KEY, etc.)

### 2.1 **Durable Job Control Plane** (`crates/api/src/routes/jobs.rs`, `crates/api/src/services/*`)
- `job_control_plane.rs` - Owns durable `/api/jobs/*` orchestration over PostgreSQL + Redis Streams
- `job_identity.rs` - Computes request hash / dedupe key for idempotent create
- `storage_ticket_service.rs` - Builds LocalFs proxy ticket or S3-compatible presigned ticket
- `/api/jobs/*` - Create job, poll status, fetch file ticket, stream/redirect ready file, send release hint
- `mux_jobs` rows now persist `preferred_video_proxy` / `preferred_audio_proxy` so worker delivery can stay on the same proxy family used at extract time

### 2.2 **Playlist Backend Orchestration** (`crates/api/src/routes/playlist_jobs.rs`, `crates/api/src/services/playlist_processor.rs`, `crates/job-system/src/playlist_job_*`) — **NEW (2026-03-18)**
- **Purpose:** Move playlist download from client-orchestrated queue to durable backend orchestration
- **Routes:**
  - `POST /api/proxy/playlist-jobs` - Create playlist job from URL, discover items once, queue processor
  - `GET /api/proxy/playlist-jobs/{id}` - Fetch job status + items + download URLs
  - `GET /api/proxy/playlist-jobs/{id}/events` - SSE stream of real-time status snapshots
  - `POST /api/proxy/playlist-jobs/{id}/cancel` - Cancel all pending items
  - `POST /api/proxy/playlist-jobs/{id}/items/{item_id}/retry` - Retry single failed item
- **Models & Storage:**
  - `playlist_job_models.rs` - Rust types (PlaylistJobStatus, PlaylistItemStatus, records)
  - `playlist_jobs` table - Orchestration metadata (source_url, status, counts, quality/mode preference)
  - `playlist_job_items` table - Per-item state (video_id, status, mux_job_id, download_url)
- **Processor:**
  - `playlist_processor.rs` - Discovers items, routes to direct or mux, updates item status
  - Just-in-time stream extraction: no upfront URL collection
  - Auto-selects codec/quality per requested_mode before routing
  - Reuses existing mux job + artifact system for muxing
  - Publishes SSE snapshots as items progress

### 3. **Extractor Engine** (`crates/extractor`)
- **Purpose:** Dynamic extraction of video metadata from various platforms
- **Architecture:**
  - `runtime.rs` - Deno runtime management for JavaScript extractors
  - `pool.rs` - Connection pooling & reuse
  - `hot_reload.rs` - Live reload of extractor scripts
  - `ytdlp.rs` - **NEW (2026-02-28):** yt-dlp subprocess extractor with moka cache (500 items, 300s TTL) and semaphore throttle (max 10 concurrent processes)
  - `types.rs` - Shared types (Stream, Platform, ExtractionResult)
- **Key Feature:** Dual extraction strategy: yt-dlp for primary extraction, Deno fallback for playlists/channels
- **Sticky Proxy Refresh (2026-03-17):** proxy-pinned refresh paths bypass the shared extract cache so refreshed signed URLs stay aligned with the proxy that will fetch them

### 4. **Proxy & Anti-Bot Layer** (`crates/proxy`)
- **Purpose:** Evade YouTube/CDN bot detection and throttling
- **Components:**
  - `anti_bot.rs` - Full anti-bot client with retry logic & proxy rotation
  - `proxy_pool.rs` - Manages proxy health & rotation
  - `cookie_store.rs` - Per-platform cookie persistence
  - `header_builder.rs` - User-Agent and header rotation
  - `throttle.rs` - Domain-level request throttling
  - `client.rs` - HTTP client wrapper
  - `stream.rs` - Streaming response handler
- **Critical Fix (2026-02-23):** Changed `.timeout(30s)` → `.connect_timeout(30s)` to allow long-duration transfers without mid-transfer timeout

### 5. **Muxer** (`crates/muxer`)
- **Purpose:** Container-level multiplexing (fMP4 format) with dual-track support
- **Components:**
  - `fmp4_remuxer.rs` - Core dual-traf remuxer (407 LOC, video-led fragment merging)
  - `moov_merger.rs` - Merge video/audio moov boxes, zero mdhd.duration (305 LOC)
  - `traf_merger.rs` - Merge track fragments, patch trun.data_offset (416 LOC)
  - `box_parser.rs` - BMFF box parsing, timescale reading (301 LOC)
  - `fragment_stream.rs` - Fragment streaming/collection (273 LOC)
  - `stream_fetcher.rs` - Fetch & buffer streams (264 LOC)
  - `mux_router.rs` - Route streams to appropriate muxer (255 LOC)
  - `codec.rs` - Codec identification/classification (189 LOC)
  - **NEW:** `init_segment_normalizer.rs` (158 LOC, **2026-03-16**) - Patch MP4 moov for FMP4 streaming

### 6.1 **Job System & Worker Runtime** (`crates/job-system`, `crates/queue`, `crates/object-store`, `crates/worker`)
- `crates/job-system/` (**NEW 2026-03-16**)
  - Owns durable job/artifact/event repository logic on PostgreSQL
  - Handles request reuse, dedupe lock, worker lease claim/reclaim, artifact ready/fail transitions
- `crates/queue/` (**NEW 2026-03-16**)
  - Redis Streams publisher/consumer abstraction for `mux_jobs`
- `crates/object-store/` (**NEW 2026-03-16**)
  - Shared storage trait with `LocalFs` and S3-compatible multipart implementations
  - **NEW:** `s3_multipart_upload.rs` for S3/MinIO/R2 support
- `crates/worker/` (**NEW 2026-03-16**)
  - Standalone mux worker process
  - Claims jobs, heartbeats leases, uploads artifacts, deletes expired storage objects
  - Keeps sticky proxy affinity through `late_extract` and auth-like URL refresh, rotating only after the current proxy becomes unusable
  - **NEW:** `job_progress_publisher.rs` for 7-phase progress streaming

### 7. **Extractors** (`/extractors`)
- **Format:** TypeScript files dynamically loaded by Deno runtime
- **Key Files:**
  - `youtube.ts` - Main YouTube extractor (strategy: InnerTube → HTML fallback)
  - `youtube-innertube.ts` - YouTube InnerTube API client
  - `youtube-n-transform.ts` - **NEW (2026-02-23):** Applies n-parameter transform for full CDN speed
  - `youtube-channel.ts` - Channel-level extraction
- **YouTube Optimization:**
  - InnerTube API returns plain URLs (no signature decryption)
  - N-parameter transform bypasses YouTube's 100 KB/s CDN throttle
  - Fallback: HTML scraping for restricted videos

## Omnichannel Distribution (2026-02-24)

### 4 Distribution Channels Implemented

The platform now reaches users via 4 independent channels:

1. **Web PWA** (`frontend/`)
   - SvelteKit-based progressive web app
   - Web Share Target API: Android "Share from YouTube app" integration
   - Background Fetch API: Downloads continue even if PWA is closed
   - Installable on Android Chrome, Desktop Chrome, Edge
   - Clipboard auto-read on focus for quick video URL pasting

2. **Bookmarklet** (`apps/injector/` → `GET /bm.js`)
   - Tiny IIFE script (6KB minified)
   - Copy-paste bookmarklet into browser bookmarks
   - Works on any browser: Chrome, Firefox, Safari, Edge
   - Injects Download button via Shadow DOM for CSS isolation
   - No installation required

3. **Browser Extension** (`apps/extension/` → 2 zips)
   - MV3 (Manifest V3) compatible
   - Firefox: 7.5KB zip via AMO (Add-ons for Mozilla)
   - Edge: 7.5KB zip via Edge Add-ons Partner Dashboard
   - Content script with Shadow DOM injection
   - Background service worker uses `chrome.downloads` API for native download dialog
   - Popup auto-detects YouTube tabs, shows quality picker

4. **UserScript** (`apps/injector/` → `GET /userscript`)
   - Tampermonkey/Greasemonkey compatible
   - 10KB IIFE with `==UserScript==` header block
   - Auto-update via `@updateURL` + `@downloadURL`
   - Reuses shared modules from bookmarklet for DRY code
   - `GM_xmlhttpRequest` bypasses CORS restrictions

### Monorepo Architecture

- **pnpm workspaces** (no Turborepo yet): zero-config workspace linking
- **Root workspace tooling** in `package.json`: build, dev, generate scripts
- **API client package** (`packages/api-client/`):
  - Generated from Rust backend via utoipa + openapi-ts
  - Provides type-safe TypeScript types + fetch functions
  - Imported by web, extension, bookmarklet, userscript
  - Zero runtime dependencies (generated types only)

### Code Reuse (DRY)

- `apps/injector/src/shared/`:
  - `inject-button.ts`: Reused by bookmarklet + userscript
  - `quality-picker.ts`: Reused by bookmarklet + userscript + extension
  - `stream-utils.ts`: Filters WebM-only streams and builds mux-job launcher URLs for external channels
- Extension and injector share identical logic, different delivery mechanisms

### Backend Integration

- `GET /openapi.json`: Serves OpenAPI spec (utoipa auto-generated)
- `GET /bm.js`: Serves bookmarklet (compile-time embed via `include_str!`)
- `GET /userscript`: Serves userscript (compile-time embed via `include_str!`)
- External clients use `POST /api/extract` plus app-domain launcher `/download/mux-job`, which then drives durable `/api/jobs/*`

## Recent Changes (2026-03-18 — Playlist Backend Orchestration)

### 1. **Backend Playlist Orchestration** ✅
**Files:** `crates/api/src/routes/playlist_jobs.rs`, `crates/api/src/services/playlist_processor.rs`, `crates/job-system/src/playlist_job_*`, `frontend/src/lib/playlist-job-api.ts`

**New API Endpoints:**
- `POST /api/proxy/playlist-jobs` - Create playlist job, discover items, queue processor
- `GET /api/proxy/playlist-jobs/{id}` - Fetch job + item status, counts, download URLs
- `GET /api/proxy/playlist-jobs/{id}/events` - SSE stream of real-time status snapshots
- `POST /api/proxy/playlist-jobs/{id}/cancel` - Cancel all pending items
- `POST /api/proxy/playlist-jobs/{id}/items/{item_id}/retry` - Retry single failed item

**New Database Tables:**
- `playlist_jobs` - Orchestration metadata (source_url, status, completed/failed counts, quality/mode)
- `playlist_job_items` - Per-item state (video_id, status, mux_job_id, artifact_key, download_url)

**Key Architecture Decisions:**
- Playlist items discovered once at job creation time
- Just-in-time stream extraction per item (not upfront bulk extraction)
- Routes to direct download OR durable mux job based on auto-selected codec
- Reuses existing `mux_jobs` + artifact system for muxing items
- SSE provides real-time progress for browser + auto-save when items ready
- Backend survives tab close; browser can refresh and recover state

**Impact:** Durable playlist downloads, per-item retry/cancel, background processing

---

## Recent Changes (2026-03-16 — i18n Complete + Mux Job Flow + Job System)

### 1. **Internationalization (i18n) Complete** ✅
**Files:** `frontend/messages/` (24+ language files)

**Features:**
- Paraglide JS integration (fully typed i18n system)
- 384 translation keys in `messages/en.json`
- 24+ supported languages: ar, bg, cs, da, de, el, en, es, et, fi, fr, hu, id, it, ja, ko, lt, lv, nb, nl, pl, pt, pt-BR, ro, ru, sk, sl, sv, tr, uk, vi, zh, zh-TW
- URL-based locale prefixes: `/en/` (default, no prefix), `/vi/`, `/de/`, etc.
- hreflang tags for SEO crawlers
- Multilingual sitemap.xml

**Impact:** Global reach, better SEO, 30+ language support

### 2. **Dual Download Flow Implemented** ✅
**Files:** `DownloadBtn.svelte`, `AppIcon.svelte`, `/api/proxy/jobs/[jobId]/events`

**New Components:**
- `DownloadBtn.svelte` - Unified component supporting both download paths
- `AppIcon.svelte` - SVG-based icons (60+ Lucide icons + quality badges)

**Two Download Paths:**
1. **Direct:** Video + Audio combined → instant browser download
2. **Mux Job:** Video-only + Audio-only → background job → 7-phase progress → auto-download

**Job Phases:**
- Starting → FetchingStreams → MuxingUploading → CompletingUpload → Ready

**Impact:** Better UX, background processing, real-time progress feedback

### 3. **Job System & Worker Infrastructure** ✅
**New Crates:**
- `crates/job-system/` - Durable job/artifact repository (PostgreSQL-backed)
- `crates/worker/` - Standalone mux worker process
- `crates/queue/` - Redis Streams pub/sub for job distribution

**New Modules:**
- `crates/job-system/src/job_progress.rs` (172 LOC) - JobProgressPhase (7 states), Redis pub/sub progress
- `crates/worker/src/job_progress_publisher.rs` (155 LOC) - Stream job progress updates
- `crates/muxer/src/init_segment_normalizer.rs` (158 LOC) - FMP4 moov box patching
- `crates/object-store/src/s3_multipart_upload.rs` (74 LOC) - S3 multipart upload support

**New API Endpoint:**
- `GET /api/proxy/jobs/[jobId]/events` - SSE stream for real-time job progress

**Impact:** Durable job pipeline, scalable worker architecture, real-time progress tracking

### 4. **Admin DB Proxy Inventory** ✅
**Removed:** `crates/proxy/src/proxy_inventory_store.rs` (moved to admin PostgreSQL DB)

**Impact:** Persistent proxy inventory management, reduced memory footprint

---

## Recent Changes (2026-03-06 — Runtime Config & Proxy Quarantine)

### 1. **Runtime Limits Configuration Activated** ✅
**File:** `config/runtime-limit-profiles.json`

**Changes:**
- Local profile: Generous limits for development (4 extract retries, 8 batch reconnects, max 1 playlist worker)
- Production profile: Conservative limits for stability
- All values now explicitly set (previously had null defaults)
- Frontend: Extract retry 500-8000ms, batch reconnect 1000-12000ms
- Frontend mux jobs: poll interval + max wait are configurable without redeploy
- Backend runtime profile now only exposes the active guards still read by code

**Impact:** Zero-downtime configuration changes without code deployment

### 2. **Proxy Quarantine System Documented** ✅
**File:** `crates/proxy/src/proxy_pool.rs`

**Feature:**
- Bad proxies automatically blocked after bot-check errors
- Persistent quarantine file: `/tmp/downloadtool-quarantined-proxies.txt`
- Faster failure detection reduces latency

**Impact:** More reliable anti-bot evasion

### 3. **API Access Tracing Added** ✅
**Location:** `crates/api/src/main.rs` (middleware setup)

**Capabilities:**
- All API requests logged with user_id, tier, endpoint, latency
- Structured logging for observability dashboards
- Error tracking with context

---

## Recent Changes (2026-03-01 — Frontend Auth & Performance)

### 1. **Auth Flow Migration to Modal** ✅
**Files:** `frontend/src/routes/(auth)/login/` (DELETED), `frontend/src/hooks.server.ts` (updated), `frontend/src/components/AuthModal.svelte` (NEW)

**What changed:**
- Login route removed entirely (no longer a separate page)
- `hooks.server.ts` detects missing session → redirects to `/?auth=required`
- AuthModal component renders on homepage, triggered by URL param
- No page reload on login, modal pops up over homepage
- Cookie check optimization: skips DB query if `better-auth` cookie absent

**Why:** Better UX (no page flicker), reduced roundtrips, centralized auth UI

### 2. **Font Optimization** ✅
**File:** `frontend/src/app.html`

**Optimization:**
- Material Symbols font: 1.1 MB → 4.5 KB (27-icon subset)
- Added `font-display: swap` for non-blocking text rendering
- Preload CSS with `<link rel="preload">`
- Lazy loading: `loading="lazy"` on external images

**Impact:** LCP expected to improve (was 7.5s before), faster FCP

### 3. **Homepage Prerendering** ✅
**File:** `frontend/src/routes/+page.ts`

**What changed:**
- Added `export const prerender = true` for static generation
- Removed `+page.server.ts` (no server-side data)
- Homepage is now pure HTML, served from cache

**Why:** Instant load, no server request, CDN cacheable

### 4. **Batch Download URL Fix** ✅
**File:** `frontend/src/lib/playlist-download-worker.ts`

**Fix:**
- Single-stream downloads use `buildStreamUrl()`
- Mux-required downloads create durable jobs and wait for ready artifact URLs
- Raw CDN fallback has been removed from playlist worker

**Impact:** Batch downloads reliable across all formats

---

## Recent Changes (2026-02-28)

### 1. **yt-dlp Subprocess Extractor** ✅
- **File:** `crates/extractor/src/ytdlp.rs` (NEW, 536 LOC)
- **Purpose:** Replace Deno in-process extraction with subprocess call to yt-dlp
- **Features:**
  - Calls `yt-dlp -J --no-playlist` for JSON metadata + stream URLs
  - yt-dlp handles PO Token, signature decryption, throttle bypass automatically
  - Moka async cache (500 items, 300s TTL) for repeat URL lookups
  - Tokio Semaphore (max 10 concurrent yt-dlp processes) to prevent resource exhaustion
  - governor rate limiting (per-IP keyed) via middleware
  - Cache metrics: hits/misses tracked for observability
- **Integration:** `lib.rs` calls `ytdlp::extract_via_ytdlp()` as primary; Deno pool kept for playlists only
- **Metrics:** Extract cache hits/misses via `EXTRACT_CACHE_HITS`, `EXTRACT_CACHE_MISSES` atomics

### 2. **Authentication & JWT System** ✅
- **Files:** `crates/api/src/auth/` (NEW)
- **Components:**
  - `jwt_claims.rs` - JWT payload (user_id, tier, exp)
  - `jwt_middleware.rs` - Axum middleware for token validation & user injection (141 LOC)
  - `user_tier.rs` - User tier enum (Free, Pro, Premium) with rate limit quotas
- **Security:** HMAC-SHA256 JWT signing via `jsonwebtoken` crate
- **Tier-Based Features:** Different extraction/batch limits per subscription level
- **BFF Pattern:** SvelteKit frontend proxies Rust API calls with JWT server-side (prevents XSS token exposure)

### 3. **Whop Subscription Integration** ✅
- **File:** `crates/api/src/routes/whop_webhook.rs` (NEW, 187 LOC)
- **Purpose:** Accept Whop webhook events for subscription management
- **Implementation:**
  - Validates HMAC-SHA256 signature via `X-Whop-Signature` header
  - Parses webhook JSON (customer, plan, custom_data with user.id)
  - Updates PostgreSQL `subscriptions` table with user tier
- **Migration:** `crates/api/migrations/0001_create_subscriptions.sql` creates schema
- **Flow:** User purchases on Whop → webhook fires → JWT issued with new tier

### 4. **Batch Operations Enhancement** ✅
- **File:** `crates/api/src/routes/batch.rs` (updated, 274 LOC)
- **New Features:**
  - SSE (Server-Sent Events) streaming of batch progress instead of polling
  - Per-download status events (queued, started, completed, failed)
  - Rate limiting per user tier (Free: 5/day, Pro: 50/day, Premium: unlimited)
  - Database persistence of batch jobs for resume capability
- **Frontend Components:**
  - `BatchInput.svelte` - Multiple URL input, submission
  - `BatchProgress.svelte` - Real-time SSE progress tracking
  - `BatchActiveState.svelte` - NEW: Visual state machine for batch job UI

### 5. **PostgreSQL Integration** ✅
- **New Dependency:** `sqlx` with PostgreSQL driver
- **Schema:**
  - `subscriptions` table: user_id, tier, created_at, expires_at
  - `users` table (implied): email, jwt_secret_hash
  - `batch_jobs` table (implied): user_id, urls[], statuses[], created_at
- **Connection:** Pooled via `sqlx::PgPool` in `AppState`
- **Migrations:** Located at `crates/api/migrations/0001_*`

## Recent Changes (2026-02-24)

### 1. **WebM Video-Only Stream Exclusion** ✅
- **File:** `crates/api/src/routes/stream.rs`
- **Change:** Early 422 UNPROCESSABLE_ENTITY error for WebM video streams
- **Why:** WebM uses EBML container (not ISO BMFF), incompatible with fMP4 remuxing
- **Detection:** `mime=video/webm` or `mime=video%2Fwebm` in URL (YouTube encodes VP9 as WebM)
- **Impact:** Prevents malformed muxing, directs users to H.264/AV1 MP4 streams

### 2. **QuickTime Double-Duration Bug Fixed** ✅
- **File:** `crates/muxer/src/moov_merger.rs`
- **Problem:** YouTube DASH init has `mdhd.duration` per track. When 2 tracks merged, QuickTime summed them (213s+213s=426s=7:06 instead of 3:33)
- **Fix:** Zero out `mdhd.duration` in both video/audio trak boxes (empty_moov style, matching ffmpeg)
- **Affected:** All muxed fMP4 files with dual audio+video tracks
- **Verified:** QuickTime & macOS player now show correct duration

### 3. **Dual-Traf Muxer Implementation** ✅
- **New File:** `crates/muxer/src/traf_merger.rs` (416 LOC)
- **Purpose:** Merge video+audio fragments into single moof with dual traf boxes
- **Compatibility:** QuickTime-compatible fragment structure
- **Key:** Precise `trun.data_offset` patching for correct sample location

### 4. **fMP4 Remuxer with Video-Led Grouping** ✅
- **File:** `crates/muxer/src/fmp4_remuxer.rs` (407 LOC)
- **Strategy:** Video-led fragment grouping (video sets pace, audio fills in)
- **Scale:** 38 video + 22 audio fragments → 38 output fragments
- **Format:** Patches `ftyp.major_brand` from `dash` to `isom`

### 5. **Frontend WebM Filter** ✅
- **File:** `frontend/src/components/FormatPicker.svelte`
- **Change:** VP9/WebM video-only streams excluded from resolution + codec options
- **Priority:** `getDefaultCodec` selects H.264 → AV1 → MP4 (never WebM)
- **UX:** Prevents user confusion, avoids muxing failures

## Codebase Metrics

| Metric | Value |
|--------|-------|
| Total Files | 110 |
| Rust Files | 40+ (~11,500 LOC including ytdlp.rs + auth) |
| TypeScript/Svelte | ~35 files (frontend, extractors) |
| Muxer Crate | 9 files, 3,205 LOC (8 modules) |
| Extractor Crate | 6 files, 700+ LOC (includes new ytdlp.rs 536 LOC) |
| API Crate | 9 routes + auth (includes new jwt middleware, webhooks) |
| Largest File | `crates/muxer/src/traf_merger.rs` (416 LOC) |
| Database Migrations | 1+ SQL files in `crates/api/migrations/` |
| Language Distribution | Rust, TypeScript, Svelte, SQL, YAML |
| Key Dependencies | Tokio, reqwest, deno_core, sqlx, jsonwebtoken, moka, governor |

## Technology Stack

| Layer | Technology |
|-------|-----------|
| Frontend | SvelteKit, Svelte, TypeScript |
| API Server | Rust + Tokio + Axum |
| Extraction | Deno Core + TypeScript |
| HTTP Client | reqwest with cookie jar |
| GPU Encoding | Hardware FFI bindings (NVIDIA/AMD) |
| Muxing | Custom fMP4 implementation |
| IPC | gRPC + Protocol Buffers |
| Containerization | Docker + docker-compose |

## File Organization

```
downloadtool/
├── crates/              # Rust crates (7 modules)
│   ├── api/             # HTTP server & routes + static file serving
│   ├── extractor/       # Extraction engine (yt-dlp + Deno)
│   ├── muxer/           # Container muxing (fMP4)
│   ├── proxy/           # Anti-bot & proxy layer
│   ├── job-system/      # Durable job/artifact repository
│   ├── worker/          # Standalone mux worker process
│   └── object-store/    # S3/local storage abstraction
├── frontend/            # SvelteKit PWA (Web UI + Share Target + Background Fetch)
├── packages/
│   └── api-client/          # Generated TypeScript API client (@downloadtool/api-client)
│       ├── src/
│       │   ├── types.gen.ts     # Auto-generated types from utoipa
│       │   └── sdk.gen.ts       # Auto-generated fetch functions
│       └── generate.sh          # Regenerate client from OpenAPI spec
├── apps/
│   ├── injector/            # Vite IIFE builder for bookmarklet & userscript
│   │   ├── src/
│   │   │   ├── bookmarklet.ts
│   │   │   ├── userscript.ts
│   │   │   └── shared/          # DRY modules (inject-button, quality-picker, stream-utils)
│   │   └── dist/
│   │       ├── bm.js            # Bookmarklet output
│   │       └── youtube-downloader.user.js  # UserScript output
│   └── extension/           # MV3 Extension (Firefox + Edge)
│       ├── src/
│       │   ├── content-script.ts
│       │   ├── background.ts
│       │   └── popup/
│       ├── manifest-firefox.json
│       ├── manifest-edge.json
│       └── build-extension.sh
├── extractors/          # Dynamic extractor scripts (TS)
├── infra/               # Infrastructure configs (WireGuard)
├── proto/               # Protocol Buffer definitions
├── docker/              # Docker images & compose files
├── plans/               # Development plans & research
├── docs/                # Documentation (this folder)
├── pnpm-workspace.yaml  # Monorepo workspace config
└── package.json         # Root workspace tooling
```

## Key Design Patterns

1. **Anti-Bot Evasion:** Proxy rotation, header randomization, cookie persistence, request throttling
2. **Dynamic Extraction:** Deno runtime for hot-reloadable JavaScript/TypeScript extractors
3. **Remuxing Pipeline:** fMP4 muxing for combining separate video+audio streams
4. **Streaming Architecture:** WebSocket for long-lived downloads, fMP4 for progressive playback
5. **Connection Pooling:** Reused HTTP & extraction engine connections

## Known Limitations & Considerations

- **YouTube Throttling:** Mitigated by n-parameter transform (deployed 2026-02-23)
- **Bot Detection:** Multi-layer defense via proxy rotation, headers, throttling
- **Mux Pipeline:** Video+audio streams muxed server-side when direct combined stream unavailable
- **Memory Usage:** Large files may require stream-based processing

## Future Improvements

- Deeper YouTube support (channels, playlists, and richer metadata)
- GPU-accelerated transcoding (planned)
- Advanced analytics & monitoring
- API rate limiting & authentication
- Batch download scheduling

---

**Last Updated:** 2026-03-19 (docs sync: removed phantom gpu-pipeline/gpu-worker refs, fixed apps/web→frontend)
**Status:** Complete & Operational (i18n ✅ | Mux Job Flow ✅ | Job System ✅ | Runtime Config ✅ | Frontend Auth Modal ✅ | Playlist Backend Orchestration ✅ | Admin Visibility ✅ | Reload Resume ✅ | Server Recovery ✅)
