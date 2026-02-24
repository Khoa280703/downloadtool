# Project Roadmap

**Last Updated:** 2026-02-24
**Project Status:** Actively Developed

## Overview

YouTube downloader platform with anti-bot protection, GPU-accelerated transcoding, and multi-channel distribution (Web PWA, Bookmarklet, Browser Extension, UserScript).

## Phase 1: Core Infrastructure & Extraction (Completed)

**Status:** ✅ Complete | **Completion Date:** Q4 2025

### Milestones
- [x] Backend API server (Rust + Tokio + Axum)
- [x] Video extraction engine (Deno runtime + TypeScript extractors)
- [x] Proxy/anti-bot layer (proxy rotation, header randomization, throttling)
- [x] Cookie store & header builder
- [x] WebSocket stream handler

### Key Deliverables
- `POST /api/extract` → Returns video metadata + stream list
- `GET /api/stream/muxed` → Streams muxed fMP4 video
- Hot-reloadable extractor scripts (`/extractors/youtube.ts`)
- n-parameter transform for full CDN speed (bypasses 100 KB/s throttle)

### Testing Status
- Core API routes tested and stable
- Extraction engine validated on live YouTube content
- Anti-bot evasion working in production

---

## Phase 2: Muxer & Container Format (Completed)

**Status:** ✅ Complete | **Completion Date:** Q1 2026

### Milestones
- [x] fMP4 (Fragmented MP4) dual-track muxer
- [x] Video+Audio stream merging with proper sync
- [x] QuickTime compatibility (zero mdhd.duration fix)
- [x] WebM video-only stream exclusion
- [x] Traf merger with precise data_offset patching

### Key Deliverables
- `crates/muxer/` (9 files, 3,205 LOC)
  - `fmp4_remuxer.rs`: Video-led fragment grouping
  - `moov_merger.rs`: Merge moov boxes, zero duration
  - `traf_merger.rs`: Merge track fragments with correct offsets
  - `box_parser.rs`: BMFF box parsing
  - `stream_fetcher.rs`: Stream buffering & collection

### Quality Status
- Duration accuracy verified in QuickTime + macOS player
- Fragment merging validated with 38 video + 22 audio fragments
- Codec classification working for H.264, AV1, VP9

### Testing Status
- All muxing scenarios tested (various aspect ratios, bitrates, durations)
- Compatibility verified in Safari, QuickTime, VLC

---

## Phase 3: Omnichannel Distribution (Completed)

**Status:** ✅ Complete | **Completion Date:** 2026-02-24

### Milestones

#### Phase 3.0: Monorepo Setup & API Client ✅
- [x] pnpm workspaces configuration
- [x] Root `package.json` with build scripts
- [x] utoipa OpenAPI generation from Rust
- [x] openapi-ts client generation (types + fetch SDK)
- [x] `packages/api-client/` workspace package

#### Phase 3.1: Bookmarklet (apps/injector) ✅
- [x] Vite IIFE build pipeline
- [x] Shadow DOM injection + CSS isolation
- [x] Quality picker modal
- [x] Stream filtering (exclude WebM video-only)
- [x] `GET /bm.js` route (compile-time embed)
- [x] Loader bookmarklet (< 200 chars)

#### Phase 3.2: Web PWA (apps/web — SvelteKit) ✅
- [x] SvelteKit app scaffold
- [x] Web Share Target API integration
- [x] Background Fetch API support
- [x] Service worker (offline cache + fetch interception)
- [x] `beforeinstallprompt` handling
- [x] Clipboard auto-read on tab focus
- [x] `POST /share-target` handler (CSRF-safe)

#### Phase 3.3: Browser Extension (apps/extension) ✅
- [x] MV3 (Manifest V3) content script
- [x] Shadow DOM button injection
- [x] Background service worker with `chrome.downloads` API
- [x] Popup quality picker
- [x] SPA navigation detection (MutationObserver)
- [x] Dual manifests (Firefox + Edge)
- [x] Build script for 2 distribution zips

#### Phase 3.4: UserScript (apps/injector) ✅
- [x] `userscript.ts` entry point
- [x] vite-plugin-monkey for header block generation
- [x] `GM_xmlhttpRequest` wrapper for CORS bypass
- [x] Code reuse from Phase 3.1 shared modules
- [x] `GET /userscript` route (compile-time embed)
- [x] Tampermonkey auto-update support

### Key Deliverables

| Channel | Size | Distribution | Installation |
|---------|------|--------------|--------------|
| **Web PWA** | ~500 KB | Self-hosted | PWA install prompt / web.app.com |
| **Bookmarklet** | 6 KB | `GET /bm.js` | Copy-paste bookmark link |
| **Firefox Extension** | 7.5 KB | AMO (Add-ons for Mozilla) | Firefox Add-ons store |
| **Edge Extension** | 7.5 KB | Edge Add-ons Partner Dashboard | Microsoft Edge store |
| **UserScript** | 10 KB | `GET /userscript` | Tampermonkey/Greasemonkey |

### Codebase Impact
- Monorepo structure with 5 packages: api-client + 4 apps
- Total new code: ~2000 LOC (TypeScript/Svelte)
- Code reuse: 3 shared modules (inject-button, quality-picker, stream-utils) used by bookmarklet, userscript, extension
- No backend API changes required (reuses existing endpoints)

### Testing Status
- All 4 channels functional and tested
- Cross-browser compatibility verified: Chrome, Firefox, Safari, Edge
- PWA tested on Android Chrome (Web Share Target, Background Fetch)
- Extension tested in Firefox developer mode + Edge developer mode

---

## Phase 4: GPU Acceleration (Planned)

**Status:** 🔄 In Progress | **Estimated Completion:** Q2 2026

### Objectives
- GPU-accelerated video encoding (NVIDIA/AMD support)
- Hardware decoder for H.264/H.265/VP9
- Real-time watermarking
- Batch transcoding with frame queue management

### Planned Deliverables
- `crates/gpu-pipeline/` expansion (decoder + encoder + frame_queue)
- GPU worker process (gRPC server)
- Transcode API endpoint: `POST /api/transcode`
- Support for MP4, WebM output formats

### Dependencies
- NVIDIA CUDA / AMD ROCm runtime
- FFmpeg GPU libraries

### Success Metrics
- Encoding speed: 10x+ improvement over CPU
- Memory footprint: < 2 GB per transcode job
- Concurrent transcodes: 4+ simultaneous jobs

---

## Phase 5: Advanced Features (Backlog)

**Status:** 📋 Planned | **Target:** Q3 2026+

### Objectives
- [ ] Batch download scheduling
- [ ] Playlist extraction (YouTube playlists, channels)
- [ ] Additional platform support (TikTok, Instagram, etc.)
- [ ] Advanced analytics & monitoring dashboard
- [ ] API authentication & rate limiting
- [ ] Distributed GPU worker scaling
- [ ] WebRTC peer-to-peer download (decentralized)

### Research Topics
- Decentralized storage integration (IPFS)
- Legal compliance (DMCA, copyright, platform ToS)
- Multi-region CDN caching
- Machine learning for auto-format selection

---

## Technical Debt & Improvements

### Completed Fixes
- ✅ QuickTime double-duration bug (zero mdhd.duration)
- ✅ WebM video-only stream exclusion (422 error)
- ✅ YouTube CDN throttle bypass (n-parameter transform)
- ✅ Connection timeout issue (changed to connect_timeout)

### Known Limitations
- YouTube DOM selectors subject to change (mitigated with fallback selectors)
- Bot detection constantly evolving (proxy rotation helps)
- GPU support limited to NVIDIA/AMD (no Intel Arc yet)
- Background Fetch API limited browser support (fallback to fetch)

### Performance Targets
- Bookmarklet injection: < 500ms
- Extract API latency: < 2s
- Muxing throughput: 5-10 Mbps typical
- PWA TTFB (Time to First Byte): < 1s

---

## Release Timeline

| Phase | Version | Status | Date |
|-------|---------|--------|------|
| Core API | 1.0.0 | ✅ Released | Q4 2025 |
| Muxer | 1.1.0 | ✅ Released | 2026-02-15 |
| Omnichannel | 1.2.0 | ✅ Released | 2026-02-24 |
| GPU Acceleration | 1.3.0 | 🔄 In Progress | Q2 2026 |
| Batch & Scheduling | 2.0.0 | 📋 Planned | Q3 2026 |

---

## Success Metrics

### User Acquisition
- **Web PWA:** 1,000+ monthly active users (target Q2 2026)
- **Firefox Extension:** 500+ installs (target Q1 2026, AMO submission pending)
- **Edge Extension:** 500+ installs (target Q1 2026, Edge store submission pending)
- **Bookmarklet:** 200+ daily active users (no submission required)
- **UserScript:** 500+ GreasyFork subscriptions (target Q1 2026)

### Quality Metrics
- 99%+ success rate on extraction (YouTube videos)
- 0 muxing failures on supported codecs
- < 5% failed downloads (network/timeout)
- TTFB < 1s for all platforms

### Operational Metrics
- Server uptime: 99.5%+
- API latency p95: < 3s
- Muxing throughput: 50 GB/day capacity
- Bot detection evasion: 95%+ success rate

---

## Dependencies & Blockers

### External Dependencies
- utoipa (Rust): OpenAPI generation ✅
- openapi-ts (TypeScript): Client generation ✅
- vite-plugin-monkey (UserScript header generation) ✅
- SvelteKit (PWA framework) ✅
- Deno Core (JavaScript runtime) ✅

### Known Issues
- None blocking current phase completion

### Next Phase Dependencies
- GPU driver availability (for Phase 4)
- Kubernetes cluster setup (for scaling, future)

---

## Contributing & Maintenance

### Development Workflow
1. Research topic (via `researcher` agent)
2. Create implementation plan (via `planner` agent)
3. Implement feature (via `backend-developer` / `frontend-developer`)
4. Test thoroughly (via `tester` agent)
5. Code review (via `code-reviewer` agent)
6. Merge to main branch

### Documentation Requirements
- Update this roadmap after each phase completion
- Maintain codebase-summary.md with architecture changes
- Document breaking API changes in CHANGELOG
- Keep security considerations updated

### Monitoring & Alerting
- Server health: CPU, memory, disk usage
- API latency: P50, P95, P99
- Extraction success rate
- Bot detection effectiveness

---

**Last Reviewed:** 2026-02-24
**Next Review Date:** 2026-03-24
