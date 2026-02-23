# Codebase Summary

**Generated:** 2026-02-23
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
- **Purpose:** Container-level multiplexing (fMP4 format)
- **Components:**
  - `fmp4_muxer.rs` - fMP4 format writer (largest: 12,395 tokens)
  - `stream_fetcher.rs` - Fetch & buffer streams
  - `codec.rs` - Codec configuration & validation
  - `mux_router.rs` - Route streams to appropriate muxer

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

## Recent Changes (2026-02-23)

### 1. **Timeout Bug Fix** ✅
- **File:** `crates/proxy/src/anti_bot.rs` (line 99)
- **Change:** `.timeout(30s)` → `.connect_timeout(30s)`
- **Impact:** Downloads no longer timeout mid-transfer; only connection establishment is limited to 30s
- **Status:** Deployed

### 2. **YouTube N-Parameter Transform** ✅
- **New File:** `extractors/youtube-n-transform.ts`
- **Purpose:** Extract & apply YouTube player.js n-parameter transform for full-speed downloads
- **How it works:**
  1. Fetches current player.js from YouTube homepage
  2. Extracts n-parameter transform function via regex pattern matching
  3. Caches transform by player version
  4. Applies transform to stream URLs before download
- **Impact:** Achieves ~2-3 Mbps instead of 100 KB/s on YouTube CDN
- **Based on:** yt-dlp technique
- **Files Modified:**
  - `extractors/youtube-innertube.ts` - Added `transformStreamUrls()` call (line 218)
  - `extractors/youtube.ts` - Added `transformStreamUrls()` call (line 103)

## Codebase Metrics

| Metric | Value |
|--------|-------|
| Total Files | 106 |
| Total Tokens | 146,804 |
| Largest File | `crates/muxer/src/fmp4_muxer.rs` (12,395 tokens) |
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
├── crates/              # Rust crates (6 modules)
│   ├── api/             # HTTP server & routes
│   ├── extractor/       # Extraction engine
│   ├── gpu-pipeline/    # GPU transcoding
│   ├── gpu-worker/      # GPU worker process
│   ├── muxer/           # Container muxing
│   └── proxy/           # Anti-bot & proxy layer
├── extractors/          # Dynamic extractor scripts (TS)
├── frontend/            # SvelteKit web UI
├── infra/               # Infrastructure configs (WireGuard)
├── proto/               # Protocol Buffer definitions
├── docker/              # Docker images & compose files
├── plans/               # Development plans & research
└── docs/                # Documentation (this folder)
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

**Last Updated:** 2026-02-23
**Status:** Complete & Operational
