# Phase 01 — Project Scaffold

## Context
- Plan: [plan.md](plan.md)
- Next: [phase-02-extraction-layer.md](phase-02-extraction-layer.md)

## Overview
- **Priority**: P0 (blocker for all phases)
- **Status**: pending
- **Effort**: 0.5d
- Setup Rust workspace, CI pipeline, Docker config, project structure

## Architecture

<!-- Updated: Validation Session 1 - Added WireGuard + gRPC + dual deployment -->

```
downloadtool/
├── Cargo.toml              # workspace root
├── crates/
│   ├── api/                # axum HTTP server (VPS)
│   ├── extractor/          # deno_core V8 extraction (VPS)
│   ├── proxy/              # stream proxy + anti-bot (VPS)
│   ├── muxer/              # CPU fMP4 muxer (VPS)
│   ├── gpu-pipeline/       # NVDEC/NVENC (Home Server, feature flag)
│   └── gpu-worker/         # gRPC server for Home Server
├── extractors/             # TypeScript extractor scripts
│   ├── youtube.ts
│   ├── youtube-channel.ts  # channel/playlist batch
│   ├── tiktok.ts
│   └── tiktok-channel.ts   # TikTok profile/hashtag batch
├── proto/
│   └── transcode.proto     # gRPC service definition
├── frontend/               # Svelte app
├── docker/
│   ├── Dockerfile.vps      # VPS build (no GPU)
│   └── Dockerfile.homeserver  # Home Server build (CUDA + GPU)
├── infra/
│   └── wireguard/
│       ├── vps.conf        # WireGuard peer config
│       └── homeserver.conf
└── .github/workflows/ci.yml
```

**Deployment targets:**
- **VPS**: `Dockerfile.vps` — axum API + extractor + proxy + muxer
- **Home Server**: `Dockerfile.homeserver` — gpu-worker gRPC server (CUDA)

## Implementation Steps

1. **Init Cargo workspace**
   ```toml
   [workspace]
   members = ["crates/*"]
   resolver = "2"
   ```

2. **Create crates** — `cargo new --lib` for each: `api`, `extractor`, `proxy`, `muxer`, `gpu-pipeline`

3. **Root Cargo.toml dependencies**
   ```toml
   axum = "0.8"
   tokio = { version = "1", features = ["full"] }
   reqwest = { version = "0.12", features = ["stream", "rustls-tls"] }
   deno_core = "0.295"
   serde = { version = "1", features = ["derive"] }
   serde_json = "1"
   tracing = "0.1"
   tracing-subscriber = "0.3"
   ```

4. **Feature flag for GPU**
   ```toml
   [features]
   gpu = ["gpu-pipeline"]
   ```

5. **WireGuard config** (`infra/wireguard/`)
   - `vps.conf`: WireGuard peer with Home Server pubkey, allowed IP `10.0.0.2/32`
   - `homeserver.conf`: WireGuard peer, listen on port 51820, VPS allowed IP `10.0.0.1/32`
   - VPS IP: `10.0.0.1`, Home Server IP: `10.0.0.2`

6. **gRPC proto** (`proto/transcode.proto`)
   ```protobuf
   service GpuWorker {
     rpc Transcode(stream TranscodeChunk) returns (stream TranscodeChunk);
   }
   message TranscodeChunk { bytes data = 1; TranscodeOptions options = 2; bool eof = 3; }
   message TranscodeOptions { string mode = 1; uint32 target_bitrate = 2; }
   ```

7. **Dockerfile.vps** — `rust:latest` builder + `debian-slim` runtime. No CUDA.

8. **Dockerfile.homeserver** — `nvidia/cuda:12.3-devel` builder + runtime with FFmpeg NVENC.

9. **CI** — GitHub Actions: `cargo build --workspace`, `cargo test`, `cargo clippy`

10. **Config struct** — env vars: `PORT`, `EXTRACTOR_DIR`, `GPU_WORKER_ADDR` (e.g. `10.0.0.2:50051`), `GPU_ENABLED`

## Todo
- [ ] Init Cargo workspace (6 crates)
- [ ] Root Cargo.toml with all deps + feature flags
- [ ] transcode.proto + tonic codegen
- [ ] WireGuard config templates (vps + homeserver)
- [ ] Dockerfile.vps + Dockerfile.homeserver
- [ ] GitHub Actions CI
- [ ] Config loader from env

## Success Criteria
- `cargo build --workspace` compiles cleanly
- CI passes on push
- WireGuard ping VPS↔HomeServer <5ms

## Risks
- FFmpeg NVENC compilation → `nvidia/cuda:12.3-devel` base image
- WireGuard kernel module availability on VPS → check `lsmod | grep wireguard`; fallback: `wireguard-go` userspace
