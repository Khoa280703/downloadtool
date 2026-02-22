---
title: "High-Performance Video Downloader"
description: "Stream-proxy video downloader (YouTube/TikTok) with Rust backend, deno_core V8 extraction, GPU transcoding pipeline, ad monetization"
status: pending
priority: P1
effort: 12-16d
branch: main
tags: [rust, video-downloader, streaming, gpu, anti-bot, monetization]
created: 2026-02-22
---

# Video Downloader — Implementation Plan

## Context
- Brainstorm: `../reports/brainstorm-260222-1238-video-downloader-architecture.md`
- Research (Rust stack): `research/researcher-01-rust-core-stack.md`
- Research (Anti-bot): `reports/researcher-02-antibot-proxy.md`

## Architecture (Revised After Research + Validation)

```
[Svelte Static Frontend] → Cloudflare Pages
         ↓ HTTPS (SSE for batch progress)
┌─────────────────────────────────────┐
│  Cloud VPS (1-10Gbps, cheap)        │  ← 90% traffic
│  Rust API — axum + tokio            │
│  ├── deno_core V8 extractors        │
│  ├── Anti-bot: VPS IP rotation      │  ← No residential proxy needed
│  ├── Stream Proxy (chunked)         │
│  ├── SSE batch/channel endpoint     │  ← Killer feature
│  └── CPU muxer (fMP4)              │
└──────────────┬──────────────────────┘
               │ WireGuard VPN tunnel (10.0.0.x LAN)
               │ gRPC streaming
┌──────────────▼──────────────────────┐
│  Home Server (Threadripper+RTX3090) │  ← 10% GPU requests only
│  GPU Worker — gRPC server           │
│  └── ffmpeg-sys-next NVDEC→NVENC    │
└─────────────────────────────────────┘
```

**Batch/Playlist flow (SSE):**
```
User pastes channel URL → VPS extracts N video links → SSE pushes list to browser
→ Browser JS downloads pool (3 concurrent) → Real-time progress bar
```

## Phases

| # | Phase | Status | Effort |
|---|---|---|---|
| 01 | [Project Scaffold + Infra](phase-01-project-scaffold.md) | pending | 1d |
| 02 | [Extraction Layer — deno_core V8](phase-02-extraction-layer.md) | pending | 2d |
| 03 | [Stream Proxy + SSE Batch](phase-03-stream-proxy.md) | pending | 2d |
| 04 | [Anti-Bot Layer](phase-04-antibot-layer.md) | pending | 1d |
| 05 | [CPU Muxer — fMP4](phase-05-cpu-muxer.md) | pending | 1d |
| 06 | [GPU Pipeline + gRPC Worker](phase-06-gpu-pipeline.md) | pending | 3d |
| 07 | [Frontend — Svelte + SSE Batch UI](phase-07-frontend.md) | pending | 2.5d |
| 08 | [Ad Integration + Monetization](phase-08-ad-integration.md) | pending | 1d |

**Total: ~13-16d** *(revised from 12-16d after batch feature added)*

## Key Dependencies
- Phase 01: WireGuard setup must be done before Phase 06 testing
- Phase 03 requires Phase 02 (extracted URLs to proxy)
- Phase 04 wraps Phase 03 (anti-bot on outbound requests)
- Phase 05 + 06 parallel after Phase 03
- Phase 06 requires WireGuard tunnel (Phase 01)
- Phase 07 requires Phase 03 (SSE endpoint) + Phase 05/06
- Phase 08 requires Phase 07

## Tech Stack
| Layer | Choice | Reason |
|---|---|---|
| Runtime | tokio (standard) | tokio-uring incompatible with axum |
| HTTP server | axum 0.8 | Production-ready, SSE + stream support |
| Extraction | deno_core 0.295+ | V8 JIT, hot-reload TS extractors |
| Outbound HTTP | reqwest + rustls | VPS IP rotation, modern TLS |
| Anti-bot | VPS IP rotation + cookie store | Residential proxy not needed (VPS IP is clean) |
| CPU mux | mp4-stream | fMP4/CMAF streaming |
| GPU | ffmpeg-sys-next | NVDEC/NVENC FFI on Home Server |
| VPS↔GPU tunnel | WireGuard (kernel) | Lowest latency, ChaCha20, LAN IP space |
| VPS↔GPU protocol | gRPC streaming | Typed, bidirectional byte streaming |
| Batch progress | SSE (Server-Sent Events) | Browser-native, no WS overhead |
| Frontend | Svelte + Vite | Lighter than React |
| CDN | Cloudflare Pages | Free, global |

## Validation Log

### Session 1 — 2026-02-22
**Trigger:** Initial plan creation validation
**Questions asked:** 7

#### Questions & Answers

1. **[Architecture]** Phase 02: deno_core V8 yêu cầu tự viết và maintain TypeScript extractors. Approach nào phù hợp?
   - Options: deno_core V8 tự maintain | deno_core + port yt-dlp | yt-dlp service pool
   - **Answer:** deno_core V8 tự maintain
   - **Rationale:** Single binary goal + zero cold-start + full control. Team accepts ongoing maintenance burden.

2. **[Architecture]** Phase 04: Chi phí residential proxy ($50-200/ngày ở 10K users). Ai trả?
   - Options: Server owner | Free tier trước | Pass to premium users
   - **Answer:** Hybrid VPS + Home Server architecture — VPS IP is the "proxy"
   - **Custom input:** VPS ($10-20/mo, 1-10Gbps) handles 90% traffic with its own clean IP. Home Server (Threadripper+RTX3090) handles GPU-only 10% via WireGuard tunnel. No residential proxy needed.
   - **Rationale:** Eliminates proxy cost entirely. VPS IP rotation (multiple cheap VPS) replaces residential proxy. Home server IP never exposed to YouTube/TikTok.

3. **[Architecture]** GPU hardware availability?
   - Options: RTX 3090 sẵn có | Thuê cloud | Bỏ GPU khỏi MVP
   - **Answer:** RTX 3090 đã có sẵn trên máy chủ
   - **Rationale:** Phase 06 can proceed immediately. Home Server deployment is primary target.

4. **[Scope]** Batch download trong MVP?
   - Options: Single URL | Batch 10 | Batch 50 | Không giới hạn
   - **Answer:** SSE pipeline + browser-side download pool (unlimited, channel/playlist support)
   - **Custom input:** User pastes channel URL → Deno V8 extracts all video links → SSE pushes to browser → Browser JS pool (3 concurrent downloads) → real-time progress bar. Zero server-side file aggregation.
   - **Rationale:** Killer differentiator feature. No competitor offers channel-level batch in web UI. SSE is stateless server-side (no WS overhead), browser handles download concurrency.

5. **[Architecture]** VPS↔Home Server tunnel?
   - Options: WireGuard | Cloudflare Tunnel
   - **Answer:** Kernel-Mode WireGuard
   - **Custom input:** WireGuard creates real LAN (VPS=10.0.0.1, HomeServer=10.0.0.2). gRPC server binds to WireGuard IP only. Zero external attack surface on home server ports. ChaCha20 encryption, UDP base = no TCP meltdown.
   - **Rationale:** Lowest latency, highest throughput (1-5Gbps), home server never exposed directly to internet.

6. **[Architecture]** VPS→GPU job routing?
   - Options: gRPC stream | Redis queue
   - **Answer:** gRPC streaming direct over WireGuard
   - **Rationale:** Low latency bidirectional streaming. Typed protobuf. No additional Redis dependency.

7. **[Scope]** Batch limit?
   - **Answer:** Unlimited via SSE + browser pool
   - **Rationale:** See answer #4. Browser JS is the download manager, not the server.

#### Confirmed Decisions
- Extraction: deno_core V8 self-maintained TS extractors
- Deployment: VPS (edge, 90%) + Home Server GPU (10%) via WireGuard + gRPC
- Anti-bot: VPS IP rotation — no residential proxy cost
- Batch: SSE endpoint + browser-side pool (3 concurrent), unlimited URLs, channel support
- Tunnel: WireGuard kernel-mode (10.0.0.x LAN)
- GPU: RTX 3090 already available, implement in Phase 06

#### Action Items
- [ ] Add WireGuard setup to Phase 01 (infra)
- [ ] Add SSE batch endpoint to Phase 03
- [ ] Update Phase 04 anti-bot to remove residential proxy requirement
- [ ] Update Phase 06 to include gRPC server on Home Server
- [ ] Update Phase 07 frontend to include SSE batch UI + progress bar
- [ ] Add channel/playlist extractor to Phase 02 TS extractors

#### Impact on Phases
- Phase 01: Add WireGuard config, VPS + Home Server deployment dockerfiles, gRPC scaffold
- Phase 03: Add `GET /api/batch` SSE endpoint for channel/playlist extraction progress
- Phase 04: Remove residential proxy — replace with VPS IP rotation (multiple VPS instances)
- Phase 06: Add gRPC server binding to WireGuard IP, update deployment to Home Server
- Phase 07: Add SSE batch UI, channel URL input, progress bar, browser download pool (JS)
