# Project Roadmap

**Last Updated:** 2026-03-16
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
- Durable mux downloads via `/api/jobs/*`
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

#### Phase 3.2: Web PWA (frontend — SvelteKit) ✅
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

## Phase 3.5: Frontend Auth Modal & Performance (Completed)

**Status:** ✅ Complete | **Completion Date:** 2026-03-01

### Milestones
- [x] Auth flow migration: `/login` route → modal on homepage
- [x] Font optimization: Material Symbols 1.1 MB → 4.5 KB
- [x] Homepage prerendering: Static HTML generation
- [x] Cookie check optimization: DB query skip for unauthenticated
- [x] Batch download URL fix: Consistent buildStreamUrl

### Key Deliverables
- Centralized AuthModal component on homepage
- `hooks.server.ts` redirect: `/login` → `/?auth=required`
- BFF proxy pattern for JWT security (no client-side token exposure)
- Expected LCP: 7.5s → ~2-3s (pending full metrics)
- Cookie check savings: 95%+ DB query reduction for anonymous users

### Testing Status
- Auth flow tested across all browsers
- Font optimization verified (subset contains all 27 used icons)
- Homepage static generation confirmed (no server request)
- Batch downloads verified with multiple format combinations

---

## Phase 4: GPU Acceleration (Planned)

**Status:** 📋 Planned | **Estimated Completion:** Q2 2026

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

## Phase 5: Internationalization (i18n) (Complete)

**Status:** ✅ Complete | **Completion Date:** 2026-03-16

### Objectives ✅
- Paraglide JS integration for type-safe i18n ✅
- Translation to 24+ languages ✅
- SEO support: hreflang tags + multilingual sitemap.xml ✅
- Language switcher component ✅
- Locale persistence (localStorage + URL) ✅

### Deliverables ✅
- `messages/en.json` with 384 keys ✅
- 24+ language translation files ✅
- LanguageSwitcher component with locale detection ✅
- hreflang implementation for all pages ✅
- Multilingual sitemap.xml generation ✅

### Completed Metrics
- 24+ languages: ar, bg, cs, da, de, el, en, es, et, fi, fr, hu, id, it, ja, ko, lt, lv, nb, nl, pl, pt, pt-BR, ro, ru, sk, sl, sv, tr, uk, vi, zh, zh-TW
- hreflang tags on all language variants ✅
- Locale switching < 200ms UX ✅
- SEO: Proper canonical/hreflang for all pages ✅

---

## Phase 5.1: Dual Download Flow & Job System (Complete)

**Status:** ✅ Complete | **Completion Date:** 2026-03-16

### Milestones
- [x] Dual download flow: Direct browser + background mux job
- [x] Job progress tracking with 7-phase pipeline
- [x] Real-time SSE progress streaming to frontend
- [x] Job control plane with durable storage (PostgreSQL + Redis)
- [x] Standalone worker process for muxing
- [x] S3 multipart upload support

### Key Deliverables
- `DownloadBtn.svelte` - Unified download component (direct + mux paths)
- `AppIcon.svelte` - SVG icon system (60+ Lucide icons + quality badges)
- `/api/proxy/jobs/[jobId]/events` - SSE endpoint for real-time progress
- `crates/job-system/` - Durable job repository
- `crates/worker/` - Standalone mux worker
- `crates/queue/` - Redis Streams abstraction
- `crates/object-store/` - Storage abstraction (LocalFs + S3)
- `crates/muxer/src/init_segment_normalizer.rs` - FMP4 moov patching
- `/download/mux-job` - Progress tracking page

### Job Phases
1. Starting → 2. FetchingStreams → 3. MuxingUploading → 4. CompletingUpload → 5. Ready

### Testing Status
- Dual download flows tested (direct + mux)
- Job progress SSE confirmed working
- S3 multipart upload validated
- Worker lease + heartbeat functioning

---

## Phase 3.6: Runtime Configuration & Telemetry (Completed)

**Status:** ✅ Complete | **Completion Date:** 2026-03-06

### Milestones
- [x] Runtime limits configuration (config/runtime-limit-profiles.json)
- [x] Proxy quarantine system documented
- [x] API access tracing enabled

### Key Deliverables
- Centralized config for backend/frontend limits (zero code changes for tuning)
- Automatic proxy blocking on detection failures
- Structured logging of all API requests with user context

### Testing Status
- Runtime config values applied correctly at startup
- Proxy quarantine persists across restarts
- API tracing logs complete and queryable

---

## Phase 3.7: Job Control Plane / Worker Storage Redesign (Completed)

**Status:** ✅ Complete | **Completion Date:** 2026-03-06

### Milestones
- [x] Durable `mux_jobs` / `mux_artifacts` / `mux_job_events` schema in PostgreSQL
- [x] New `/api/jobs/*` control-plane contract with JWT ownership checks
- [x] Redis Streams publish path for durable mux worker pipeline
- [x] Standalone `crates/worker` process with lease reclaim + heartbeat
- [x] `object-store` abstraction with `LocalFs` and S3-compatible multipart upload
- [x] Frontend `/api/proxy/jobs/*` cutover for mux download flow
- [x] Direct download ticket support for object storage + TTL cleanup runner

### Key Deliverables
- API can create/reuse durable jobs without relying on in-memory queue state
- Worker claims jobs from Redis-backed queue and updates canonical state in Postgres
- Browser can download ready artifacts through LocalFs proxy or direct presigned object-storage URL
- Expired artifacts are cleaned up server-side even if frontend never sends release callback

### Verification Status
- `cargo test --workspace`
- `cargo check --workspace`
- `pnpm --filter frontend check`

### Residual Follow-up
- Sync mux path was removed; all mux downloads now converge on durable `/api/jobs/*`

---

## Phase 5.2: UI-Preserving SEO (Partial)

**Status:** 🔄 In Progress | **Estimated Completion:** Q2 2026

### Objectives
- [x] Phase 01: Homepage SEO foundation (brand rename, metadata, JSON-LD, cache-control) — COMPLETE
- [x] Phase 02: Homepage content blocks (Why Snapvie, quality, FAQ with schema) — COMPLETE
- [x] Phase 03: 5 EN landing pages (8k-hdr, playlist, shorts, 4k, mp3) + BreadcrumbList — COMPLETE
- [x] Phase 04: Sitemap, robots.txt, internal links, indexability — COMPLETE
- [ ] Phase 05: Measurement, rollout, expansion gate — DEFERRED (need Search Console data)
- [ ] Phase 06: Supporting content cluster — DEFERRED
- [ ] Phase 07: Authority, distribution, link acquisition — DEFERRED
- [ ] Phase 08: SERP CTR optimization loop — DEFERRED
- [ ] Phase 09: Technical renderability, CWV, crawl efficiency — DEFERRED
- [x] Phase 10: Trust pages (about, contact, terms, dmca) — COMPLETE

### Completed Deliverables
- Brand rename: FetchTube → Snapvie (39 files)
- Canonical, OG, Twitter, JSON-LD tags (Organization → WebSite → WebApplication schema)
- Cache-control split (public for SEO pages, private for auth pages)
- hreflang fix, localStorage keys migration
- Homepage content blocks (Why Snapvie, Quality badges, FAQ)
- 5 EN landing pages: `/download-youtube-{8k-hdr, playlist, shorts, 4k, mp3}`
- BreadcrumbList structured data
- Sitemap + robots.txt Disallow rules
- Public trust surface: `/about`, `/contact`, `/terms`, `/privacy`, `/dmca`
- Internal linking structure

### In Progress
- Phase 05: Measurement baseline & expansion gate
- Phases 06-09: Supporting content, authority, CTR optimization, technical refinements

### Success Metrics
- 5 EN landing pages indexed and tracked in Search Console
- Keyword clusters: playlist, shorts, 4K, 8K HDR showing impressions
- Core Web Vitals performance maintained
- Trust pages providing entity signals
- Multilingual technical SEO ready for VI/PT-BR expansion

### Related Plan
- Full details: `plans/260319-0929-ui-preserving-seo-snapvie/plan.md`

---

## Phase 6: Advanced Features (Backlog)

**Status:** 📋 Planned | **Target:** Q3 2026+

### Objectives
- [ ] Batch download scheduling
- [ ] Playlist extraction (YouTube playlists, channels)
- [ ] YouTube-only feature depth (channels, playlists, smarter filtering)
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
| Runtime Config | 1.2.1 | ✅ Released | 2026-03-06 |
| i18n Integration | 1.4.0 | ✅ Released | 2026-03-16 |
| Mux Job Flow & Job System | 1.5.0 | ✅ Released | 2026-03-16 |
| SEO Foundation | 1.6.0 | ✅ Partial | 2026-03-19 |
| GPU Acceleration | 2.0.0 | 📋 Planned | Q2 2026 |
| Batch & Scheduling | 2.1.0 | 📋 Planned | Q3 2026 |

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

**Last Reviewed:** 2026-03-20
**Next Review Date:** 2026-04-20
