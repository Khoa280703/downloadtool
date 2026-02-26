# Brainstorm Report: Video Downloader Architecture
*Date: 2026-02-22 | Project: downloadtool*

## Problem Statement
Build a high-performance video downloader web service (YouTube, YouTube) monetized via ads. User wants maximum technical quality, not simplicity. Self-hosted on bare-metal (Threadripper CPU + RTX 3090 GPU).

## Requirements
- Ad-revenue commercial website
- High concurrency (target: 10K+ concurrent users)
- Zero-storage stream proxy (no disk)
- Fast extraction (no cold start)
- Anti-bot reliability (differentiator vs ssstik ~70% uptime)
- On-the-fly transcoding/watermark features (GPU differentiation)
- Single binary, self-hostable

## Market Analysis
See: `researcher-260222-1248-video-downloader-market-analysis.md`

Key findings:
- No competitor uses Rust in production
- cobalt.tools is fastest (14x improved) but no monetization
- ssstik.io dominates YouTube but has ~70% uptime, heavy ads
- No web UI for batch/playlist download (CLI-only via yt-dlp)
- GPU transcoding on-the-fly: no competitor offers this

## Evaluated Approaches

### Extraction Layer
| Option | Pros | Cons |
|---|---|---|
| **deno_core V8 embedded** | Zero cold-start, single binary, hot-reload JS | Must self-maintain extractors; V8 API unstable |
| yt-dlp service pool | 1800+ extractors, community maintained, always updated | Python runtime dependency |
| rquickjs | Lighter than V8 | Too immature, smaller ecosystem |

**Decision: deno_core V8** — aligns with single-binary goal + maximum control

### I/O Runtime
| Option | Pros | Cons |
|---|---|---|
| **tokio-uring** | Lower syscall overhead, better for Threadripper many-core | Linux-only, newer API |
| tokio standard | Battle-tested, wide ecosystem | Marginally less efficient at extreme load |

**Decision: tokio-uring** — justified for high-core-count hardware

### TLS
**Decision: rustls** — already uses AES-NI by default, modern, pure Rust. Not a custom optimization needed.

### Media Processing
| Use Case | Technology | GPU needed? |
|---|---|---|
| Audio + Video mux (container) | CPU mp4 crate | ❌ |
| Watermark branding on-the-fly | NVDEC → process → NVENC | ✅ |
| Re-compress 4K → lower bitrate | NVDEC → NVENC | ✅ |
| Format convert (VP9 → H264) | NVDEC → NVENC | ✅ |

**Decision: CPU for muxing, GPU for transcoding features** (key differentiation)

### Anti-Bot
**Decision: Multi-layer**
1. JA3 TLS fingerprint spoofing (custom rustls ClientHello)
2. Residential proxy pool rotation
3. Browser-like request headers simulation
4. Rate limiting + backoff per IP range

## Final Architecture

```
[React/Svelte Static] → Cloudflare Pages (free CDN)
         ↓ HTTPS
[Rust API Server]
  ├── tokio-uring runtime
  ├── axum HTTP framework
  └── rustls TLS (AES-NI auto)
         ↓
[Extraction Orchestrator]
  ├── deno_core V8 (TypeScript extractors, hot-reload)
  └── Fallback: yt-dlp service (long-running, pooled connections)
         ↓
[Anti-Bot Layer]
  ├── JA3 fingerprint spoofing
  ├── Residential proxy rotation
  └── User-Agent + header simulation
         ↓
[Stream Proxy - tokio-uring async]
         ↓
[Media Router]
  ├── Simple mux → CPU (mp4 fragmented, milliseconds)
  └── Transcode/Watermark → GPU Pipeline
        (NVDEC → VRAM processing → NVENC → chunked HTTP stream)
```

## Differentiation vs Market
1. **Reliability** — JA3 spoofing + proxy rotation vs competitors' simple IP rotation
2. **On-the-fly GPU transcoding** — no competitor offers this in web UI
3. **Watermark branding** — add logo to downloaded videos = viral marketing
4. **Future: Batch/Playlist** — first web UI for playlist download

## Implementation Considerations
- deno_core API changes with Deno versions → pin version carefully
- NVENC/NVDEC FFI via `ffmpeg-sys-next` crate (safer than raw CUDA bindings)
- JA3 spoofing requires forking/customizing rustls ClientHello builder
- Fragmented MP4 (fMP4) allows streaming before full mux completion
- Residential proxy costs: ~$10-50/GB depending on provider

## Risk Assessment
| Risk | Probability | Mitigation |
|---|---|---|
| Legal/DMCA takedown | Medium | Terms of service, DMCA response, jurisdictional hosting |
| Platform API changes block extraction | High | deno_core hot-reload + yt-dlp fallback |
| GPU memory exhaustion (many concurrent transcodes) | Medium | Queue system, max concurrent GPU jobs |
| deno_core V8 API instability | Medium | Pin version, integration tests |
| Proxy IP ban cascades | Medium | Multi-provider proxy pool |

## Success Metrics
- Extraction success rate: >99% (vs ssstik ~70% uptime)
- Time to first byte: <500ms
- Concurrent streams: 10K+ on Threadripper
- GPU transcoding latency: <2s for watermark, <5s for re-encode

## Next Steps → Implementation Plan
Phases:
1. Rust project scaffold (axum + tokio-uring)
2. deno_core V8 extraction layer + TypeScript extractors
3. Stream proxy core
4. Anti-bot layer (JA3 + proxy pool)
5. CPU muxer (mp4 fragmented)
6. GPU pipeline (NVDEC → NVENC via FFI)
7. Frontend (React/Svelte)
8. Ad integration + monetization
