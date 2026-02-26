# Phase 07 — Verify Build & Tests

## Overview
- **Priority:** P1
- **Status:** pending
- **ETA:** 20m
- **Depends on:** Phases 01–06 (all must complete first)

---

## Verification Checklist

### 1. Confirm deleted paths are gone

```bash
ls crates/gpu-worker 2>&1   # expect: No such file or directory
ls crates/gpu-pipeline 2>&1 # expect: No such file or directory
ls proto/transcode.proto 2>&1 # expect: No such file or directory
ls docker/Dockerfile.gpu-worker 2>&1 # expect: No such file or directory
```

### 2. Grep for residual GPU references in Rust sources

```bash
grep -r 'gpu\|transcode\|GPU_WORKER\|GPU_ENABLED\|tonic\|prost' \
  crates/api/src/ Cargo.toml
```

Expected: zero matches.

### 3. Cargo build — workspace

```bash
cargo build --workspace
```

Expected: zero errors, zero warnings about unknown features or missing deps.

### 4. Cargo build — release

```bash
cargo build --workspace --release
```

### 5. Run all tests

```bash
cargo test --workspace --all-targets --verbose
```

Expected: all tests pass. Notably `config::tests::test_default_config` should pass with GPU fields removed.

### 6. Cargo clippy

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

Expected: zero warnings.

### 7. Docker Compose config validation

```bash
docker compose -f docker/docker-compose.server.yml config
```

Expected: valid YAML printed, no references to `gpu-worker`.

### 8. Frontend type check

```bash
cd frontend && pnpm check
```

Expected: zero svelte-check errors.

---

## Rollback Plan

If any step fails:

1. `git diff` to identify what was changed
2. `git checkout -- <file>` to revert individual files
3. Or `git stash` to revert all uncommitted changes

---

## Success Criteria (Definition of Done)

- [ ] All deleted paths confirmed absent
- [ ] `cargo build --workspace --release` exits 0
- [ ] `cargo test --workspace` all pass
- [ ] `cargo clippy --workspace -- -D warnings` exits 0
- [ ] `docker compose config` validates clean
- [ ] `pnpm check` in frontend passes
- [ ] `grep -r 'gpu-worker\|gpu-pipeline\|transcode\|GPU_ENABLED\|GPU_WORKER' .` (excluding `plans/`) returns zero matches
