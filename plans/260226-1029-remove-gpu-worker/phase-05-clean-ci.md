# Phase 05 — Clean CI/CD

## Overview
- **Priority:** P2
- **Status:** pending
- **ETA:** 10m
- **Depends on:** none (independent)

## File to Modify

`/home/khoa2807/working-sources/downloadtool/.github/workflows/ci.yml`

---

## Changes Required

### Remove GPU Worker Dockerfile validation step

In the `docker` job, remove the entire step (lines 188–191):

```yaml
# Note: GPU worker Docker build requires CUDA which is not available in GitHub Actions
# This step validates the Dockerfile syntax only
- name: Validate GPU Worker Dockerfile
  run: docker build --file docker/Dockerfile.gpu-worker --target builder .
  continue-on-error: true
```

### Remove `protobuf-compiler` from apt installs

`protobuf-compiler` was only needed because `gpu-worker` and `gpu-pipeline` used `tonic-build` (prost codegen) in their `build.rs`. Once those crates are gone, the `api` crate no longer needs protobuf at build time.

Remove from all three `apt-get install` steps in `clippy`, `build`, `test`, and `docs` jobs:

```yaml
sudo apt-get install -y pkg-config libssl-dev protobuf-compiler
#                                                ↑ remove this
```

Becomes:
```yaml
sudo apt-get install -y pkg-config libssl-dev
```

Affects jobs: `clippy`, `build`, `test`, `docs` (4 occurrences).

> **Note:** Verify first that no remaining crate has a `build.rs` using `tonic-build` or `prost-build`. After Phase 01 deletes both GPU crates, this should be safe.

---

## Implementation Steps

1. Edit `.github/workflows/ci.yml`
2. Remove the `Validate GPU Worker Dockerfile` step block (including comments)
3. Remove `protobuf-compiler` from all 4 `apt-get install` lines
4. Confirm no other step references `gpu`, `cuda`, `nvidia`, or `transcode`

## Success Criteria
- `grep -i 'gpu\|cuda\|nvidia\|transcode\|protobuf' .github/workflows/ci.yml` returns no matches
- CI pipeline still has: fmt, clippy, build, test, docs, docker (API image only)
