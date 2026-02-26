# Phase 02 — Clean Workspace Cargo.toml

## Overview
- **Priority:** P1
- **Status:** pending
- **ETA:** 10m
- **Depends on:** Phase 01

## File to Modify

`/home/khoa2807/working-sources/downloadtool/Cargo.toml`

## Current State

```toml
[workspace]
members = ["crates/*"]   # glob — gpu-worker & gpu-pipeline auto-included

[workspace.dependencies]
tonic = "0.12"
prost = "0.13"
```

The workspace uses a `"crates/*"` glob, so deleting the crate directories (Phase 01) automatically removes them from the member list — **no members edit needed**.

## Changes Required

### Remove workspace-level GPU deps

Remove these two entries from `[workspace.dependencies]` (only used by gpu-worker/gpu-pipeline):

```toml
# gRPC for GPU worker communication   ← remove this comment too
tonic = "0.12"
prost = "0.13"
```

### Verify no remaining references

After removal, confirm no other crate's `Cargo.toml` references `tonic` or `prost` at workspace level. (`crates/api` declared `tonic` as optional — it will be cleaned in Phase 03 to use a direct version pin or removed entirely.)

## Implementation Steps

1. Edit `Cargo.toml` — delete the `tonic` and `prost` lines plus the comment above them
2. Run `cargo metadata --no-deps 2>&1 | grep -E 'tonic|prost|gpu'` — expect no output

## Success Criteria
- `Cargo.toml` has no `tonic`, `prost`, or GPU-related entries
- `cargo check --workspace` proceeds past dependency resolution (full build verified in Phase 07)
