# Phase 05 — Docs Sync

## Context Links
- `docs/codebase-summary.md` — references `crates/gpu-pipeline`, `apps/web/`
- `docs/code-standards.md` — references `engine.rs`
- `docs/architecture_review.md` — says worker is single-threaded

## Overview
- **Priority:** P2 Low
- **Status:** completed (2026-03-19)
- **Effort:** 1h
- Docs reference non-existent modules and contain outdated claims about architecture.

## Key Insights
- `crates/gpu-pipeline` does NOT exist in current codebase (phantom reference)
- `apps/web/` does NOT exist — frontend is at `frontend/`
- `engine.rs` reference in code-standards may still exist in extractor crate — verify
- Worker at `crates/worker/src/main.rs:131` uses concurrent processing but docs say single-threaded

## Requirements
- Remove references to non-existent `crates/gpu-pipeline`, `crates/gpu-worker`
- Fix `apps/web/` references to `frontend/`
- Update worker description to reflect concurrent architecture
- Verify `engine.rs` reference accuracy
- Update crate count and LOC metrics

## Architecture
No code change. Documentation update only.

## Related Code Files
- **Modify:** `docs/codebase-summary.md`
- **Modify:** `docs/code-standards.md`
- **Modify:** `docs/architecture_review.md` (if exists)
- **Reference:** `crates/worker/src/main.rs` (verify concurrency model)

## Implementation Steps

1. **Audit `docs/codebase-summary.md`:**
   - Remove `gpu-pipeline` and `gpu-worker` from architecture diagram
   - Fix `apps/web/` → `frontend/`
   - Update crate list to match actual workspace
   - Update file counts and LOC estimates

2. **Audit `docs/code-standards.md`:**
   - Verify `engine.rs` reference — if file exists, keep; if not, remove
   - Update directory structure to match current codebase

3. **Audit `docs/architecture_review.md`:**
   - Fix "single-threaded worker" claim
   - Describe concurrent processing model accurately

4. **Cross-check all docs for other phantom references:**
   - Grep docs/ for file/module names and verify they exist

5. **Update "Last Updated" timestamps**

## Todo List
- [x] Fix codebase-summary.md phantom references
- [x] Fix code-standards.md outdated structure
- [x] Fix architecture_review.md concurrency claim
- [x] Grep for other phantom references
- [x] Update timestamps

## Success Criteria
- All module/file references in docs point to existing code
- Architecture descriptions match actual implementation
- No phantom crate references remain

## Risk Assessment
- **Zero code risk**: docs-only change
- Risk of missing some references — grep thoroughly

## Security Considerations
- None

## Next Steps
- Set up a pre-commit hook or CI check for doc freshness (YAGNI for now)
