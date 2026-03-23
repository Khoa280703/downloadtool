# Brainstorm: Fork yt-dlp vs Rust Pipeline

**Date:** 2026-03-23
**Status:** Concluded — No action needed

## Problem Statement
User considered forking yt-dlp to embed Snapvie's proxy/worker/R2/muxer logic into Python, aiming for multi-platform expansion and potential performance gains.

## Evaluated Approaches

### A. Fork yt-dlp + Rewrite Snapvie Logic in Python
- **Pros:** yt-dlp's 1000+ extractors built-in, ffmpeg handles all codecs
- **Cons:**
  - Rewriting Rust→Python (proxy rotation, anti-bot, R2 upload, job queue, progress tracking)
  - Disk-based pipeline (ffmpeg): ~40% slower than streaming Rust muxer
  - Fork maintenance: yt-dlp commits daily, YouTube extractor changes weekly
  - RAM/disk: 10x more RAM, 220MB disk per job vs zero disk
  - Scale: 100 concurrent jobs → OOM/disk risk
- **Verdict:** ❌ High effort, worse performance, maintenance nightmare

### B. yt-dlp Plugin (No Fork)
- **Pros:** No fork conflicts, plugin API more stable
- **Cons:** Still disk-based, still Python overhead, limited hook points
- **Verdict:** ⚠️ Viable but unnecessary for current goals

### C. Keep Rust Pipeline + Platform Adapters (Recommended)
- **Pros:**
  - yt-dlp -J already works for 1000+ sites (no fork needed)
  - Only normalize output → Snapvie VideoInfo format (~few hundred LOC per platform)
  - Streaming pipeline 40% faster, 10x less RAM, zero disk
  - Zero fork maintenance
- **Cons:** Rust muxer only supports fMP4 copy-codec (no transcode)
- **Verdict:** ✅ Best balance of effort/performance/maintainability

## Key Findings

### Performance Prediction (100MB video + 10MB audio)
| Metric | Rust streaming | ffmpeg subprocess |
|--------|---------------|-------------------|
| Time (1 job) | ~15s | ~25s |
| RAM/job | 5-10MB | 50-100MB |
| Disk/job | 0 | ~220MB |
| 100 concurrent | 1GB RAM, 0 disk | OOM/disk risk |

Rust wins because download+mux+upload run **simultaneously** (pipelining) vs **sequential** for ffmpeg.

### Multi-platform Reality
Most platforms (TikTok, Instagram, Facebook, Twitter) serve combined audio+video — **no muxing needed**. Only YouTube and Vimeo separate streams. Multi-platform expansion is primarily an extraction normalization problem, not a muxing problem.

## Decision
Keep current Rust streaming pipeline. No fork needed. When expanding to new platforms, use `yt-dlp -J` for extraction + Rust adapter for normalization.

## Future Consideration
Add ffmpeg as **fallback mux path** only if needed for edge cases (WebM transcode, corrupt stream repair). Not priority now.
