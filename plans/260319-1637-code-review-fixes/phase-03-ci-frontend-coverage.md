# Phase 03 — CI Frontend Coverage

## Context Links
- CI config: `.github/workflows/ci.yml`
- Frontend build: `frontend/package.json` → `pnpm build`, `pnpm check`
- PM: pnpm

## Overview
- **Priority:** P1 High
- **Status:** pending
- **Effort:** 30m
- CI only has Rust jobs (fmt, clippy, build, test, docs, docker). No frontend checks at all. Broken Svelte/TS code can merge without detection.

## Key Insights
- Frontend has existing scripts: `pnpm build` (Vite build), `pnpm check` (svelte-check + tsc)
- `pnpm check` requires `pnpm run paraglide:compile && svelte-kit sync` first (already in script)
- Frontend job is independent of Rust jobs — can run in parallel
- Node 20 LTS is standard for CI
- **`pnpm-lock.yaml` is at repo ROOT**, not in `frontend/` dir — `pnpm install` must run from root or use `--filter` pattern

## Requirements
- Add `frontend` job to CI workflow
- Install pnpm, install deps, run build, run type check
- Job runs parallel to Rust jobs (no `needs:` dependency)

## Architecture
Single new job in existing `.github/workflows/ci.yml`.

## Related Code Files
- **Modify:** `.github/workflows/ci.yml` — add `frontend` job

## Implementation Steps

1. Add `frontend` job to `.github/workflows/ci.yml` after the `fmt` job block:

```yaml
  # Check frontend build and types
  frontend:
    name: Frontend
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: 20

      - name: Install pnpm
        uses: pnpm/action-setup@v4
        with:
          version: 9

      - name: Install dependencies
        run: pnpm install --frozen-lockfile

      - name: Type check
        run: pnpm --filter frontend check

      - name: Build
        run: pnpm --filter frontend build
```

**Important notes:**
- `pnpm install` runs at **repo root** (where `pnpm-lock.yaml` lives), NOT in `frontend/`
- Use `pnpm --filter frontend` to target frontend workspace package
- Do NOT use `working-directory: frontend` — lockfile is at root

2. Commit and push to trigger CI

## Todo List
- [x] Add `frontend` job to ci.yml (install at root, `--filter frontend` for check/build)
- [ ] Test CI run passes

## Success Criteria
- `frontend` job appears in GitHub Actions
- Runs in parallel with Rust jobs
- Catches TypeScript/Svelte errors before merge
- Does not slow down overall CI (runs ~2-3 min)

## Risk Assessment
- **Low risk**: additive change, does not affect existing Rust CI
- If frontend build fails on CI but passes locally, likely missing env vars or dependencies — fix in CI config

## Security Considerations
- None. CI is read-only build check.

## Next Steps
- Consider adding frontend lint (eslint) if configured
- Consider caching `node_modules` for faster CI runs (pnpm store cache)
