# Phase 01 — Delete GPU Crates & Proto

## Overview
- **Priority:** P1 (must run first — other phases depend on absence of these crates)
- **Status:** pending
- **ETA:** 15m

## Files to Delete (entirely)

| Path | Reason |
|------|--------|
| `crates/gpu-worker/` | gRPC GPU worker binary — entire directory |
| `crates/gpu-pipeline/` | Shared GPU processing logic — entire directory |
| `proto/transcode.proto` | Only consumed by gpu-worker; no other crate references it |

## Implementation Steps

1. `rm -rf crates/gpu-worker/`
2. `rm -rf crates/gpu-pipeline/`
3. `rm proto/transcode.proto`

## Success Criteria
- None of the three paths exist on disk
- `cargo metadata` no longer lists `gpu-worker` or `gpu-pipeline` packages (after Phase 2 workspace cleanup)

## Notes
- No other workspace crate imports `gpu-pipeline` directly in its non-optional deps (only `crates/api` does via the optional `gpu` feature — cleaned in Phase 3)
- `proto/transcode.proto` is only referenced in `gpu-worker`'s `build.rs` (tonic-build); deleting the crate makes the proto file dead
