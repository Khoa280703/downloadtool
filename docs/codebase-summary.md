# Codebase Summary

**Generated:** 2026-02-24
**Total Files:** 106 | **Total Tokens:** 146,804

## Project Overview

A high-performance video downloader platform supporting YouTube and other platforms with anti-bot protection, GPU-accelerated transcoding, and full-speed CDN downloads via YouTube n-parameter transformation.

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
       │  - Transcode   │           │  - Proxy Rotation│
       └────┬──────┬────┘           └──────────────────┘
            │      │
    ┌───────▼──┐   └──────────────────┐
    │Extractor │    ┌────────────────┐▼───────────────┐
    │Engine    │    │  GPU Pipeline  │  Muxer         │
    │(crates/  │    │  (crates/gpu-  │  (crates/      │
    │extractor)│    │  pipeline)     │  muxer)        │
    │          │    │ - Decoder      │ - fMP4 Format  │
    │- Hot     │    │ - Encoder      │ - Stream       │
    │  reload  │    │ - Watermark    │   Fetcher      │
    │- Runtime │    │- Frame Queue   │ - Codec Config │
    │  pooling │    │               │ - Mux Router   │
    └──────────┘    └───────────────┘─────────────────┘
         │                    │             │
         │            ┌───────▼─────────────▼──────┐
         └───────────►│  GPU Worker Process        │
                      │  (crates/gpu-worker)       │
                      │  - Transcode Management    │
                      └────────────────────────────┘
```

## Key Components

### 1. **Frontend** (`/frontend`)
- **Framework:** SvelteKit with Svelte components
- **Key Components:**
  - `UrlInput.svelte` - URL input handling
  - `BatchInput.svelte` - Batch download processing
  - `DownloadBtn.svelte` - Download initiation
  - `FormatPicker.svelte` - Stream quality/format selection
  - `BatchProgress.svelte` - Download progress tracking
  - `CookieConsent.svelte` - Privacy compliance
  - `AdBanner.svelte` & `InterstitialAd.svelte` - Ad integration
- **Features:** Responsive design, real-time progress, ad monetization

### 2. **API Layer** (`crates/api`)
- **Entry Point:** `main.rs` - HTTP server (Tokio-based)
- **Routes:**
  - `extract.rs` - Extract video metadata & streams
  - `batch.rs` - Batch download operations
  - `stream.rs` - WebSocket stream handler
  - `transcode.rs` - GPU transcoding requests
- **Config:** Platform-aware settings, environment-based configuration

### 3. **Extractor Engine** (`crates/extractor`)
- **Purpose:** Dynamic extraction of video metadata from various platforms
- **Architecture:**
  - `engine.rs` - Core extraction orchestrator
  - `runtime.rs` - Deno runtime management for JavaScript extractors
  - `pool.rs` - Connection pooling & reuse
  - `hot_reload.rs` - Live reload of extractor scripts
  - `types.rs` - Shared types (Stream, Platform, ExtractionResult)
- **Key Feature:** Dynamically loads TypeScript extractors from `/extractors` directory

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

### 5. **GPU Pipeline** (`crates/gpu-pipeline`)
- **Purpose:** Hardware-accelerated video encoding
- **Components:**
  - `pipeline.rs` - Orchestrates decode → transform → encode flow
  - `decoder.rs` - Hardware video decoding
  - `encoder.rs` - Hardware video encoding (largest component: 12,395 tokens)
  - `frame_queue.rs` - Async frame buffering
  - `watermark.rs` - Watermark overlay processing
  - `ffi.rs` - GPU driver FFI bindings

### 6. **Muxer** (`crates/muxer`)
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

### 7. **GPU Worker** (`crates/gpu-worker`)
- **Purpose:** Standalone process for GPU transcoding
- **Components:**
  - `server.rs` - gRPC server for transcode requests
  - `transcode.rs` - Transcode job execution
- **Protocol:** Protocol Buffers (`proto/transcode.proto`)

### 8. **Extractors** (`/extractors`)
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

1. **Web PWA** (`apps/web/`)
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
  - `stream-utils.ts`: Filters WebM-only streams, builds muxed URLs for all 4 channels
- Extension and injector share identical logic, different delivery mechanisms

### Backend Integration

- `GET /openapi.json`: Serves OpenAPI spec (utoipa auto-generated)
- `GET /bm.js`: Serves bookmarklet (compile-time embed via `include_str!`)
- `GET /userscript`: Serves userscript (compile-time embed via `include_str!`)
- No new API endpoints required; uses existing `POST /api/extract` + `GET /api/stream/muxed`

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
| Total Files | 106 |
| Rust Files | 43 (10,188 LOC) |
| Frontend Files | ~30 (TypeScript + Svelte) |
| Muxer Crate | 9 files, 3,205 LOC (8 modules) |
| Largest File | `crates/muxer/src/traf_merger.rs` (416 LOC) |
| Language Distribution | Rust, TypeScript, Svelte, YAML |
| Key Dependencies | Tokio, reqwest, deno_core, GPU libraries |

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
├── backend/
│   └── crates/              # Rust crates (6 modules)
│       ├── api/             # HTTP server & routes + static file serving
│       ├── extractor/       # Extraction engine
│       ├── gpu-pipeline/    # GPU transcoding
│       ├── gpu-worker/      # GPU worker process
│       ├── muxer/           # Container muxing
│       └── proxy/           # Anti-bot & proxy layer
├── packages/
│   └── api-client/          # Generated TypeScript API client (@downloadtool/api-client)
│       ├── src/
│       │   ├── types.gen.ts     # Auto-generated types from utoipa
│       │   └── sdk.gen.ts       # Auto-generated fetch functions
│       └── generate.sh          # Regenerate client from OpenAPI spec
├── apps/
│   ├── web/                 # SvelteKit PWA (Web UI + Share Target + Background Fetch)
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
3. **Hardware Acceleration:** GPU pipeline for real-time video transcoding
4. **Streaming Architecture:** WebSocket for long-lived downloads, fMP4 for progressive playback
5. **Connection Pooling:** Reused HTTP & extraction engine connections

## Known Limitations & Considerations

- **YouTube Throttling:** Mitigated by n-parameter transform (deployed 2026-02-23)
- **Bot Detection:** Multi-layer defense via proxy rotation, headers, throttling
- **GPU Requirements:** NVIDIA/AMD GPU required for encoding
- **Memory Usage:** Large files may require stream-based processing

## Future Improvements

- Additional platform support (TikTok, Instagram, etc.)
- Distributed GPU worker scaling
- Advanced analytics & monitoring
- API rate limiting & authentication
- Batch download scheduling

---

**Last Updated:** 2026-02-24
**Status:** Complete & Operational (WebM Exclusion + QuickTime Fix Deployed)
