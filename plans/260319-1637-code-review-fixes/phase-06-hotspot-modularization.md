# Phase 06 — Hotspot Modularization

## Context Links
- `frontend/src/routes/+page.svelte` (~1900 LOC)
- `frontend/src/lib/api.ts`
- `crates/extractor/src/ytdlp.rs`
- `crates/api/src/routes/stream.rs`
- `crates/proxy/src/proxy_pool.rs`

## Overview
- **Priority:** P3 Low (tech debt)
- **Status:** pending (deferred)
- **Effort:** deferred — do when touching files for other reasons
- Several files exceed 200 LOC guideline. Not urgent but increases maintenance burden and context window cost for AI tools.

## Key Insights
- `+page.svelte` at ~1900 LOC is the worst offender — homepage with mixed concerns
- Project rule: keep files under 200 LOC for optimal LLM context management
- Modularization should happen incrementally, not as a big-bang refactor
- Each file needs careful analysis of logical separation boundaries

## Requirements
- When touching a hotspot file for a feature/fix, extract logical sections into modules
- Use kebab-case naming with descriptive names for extracted modules
- Maintain existing functionality — no behavior changes

## Architecture
Extract components and functions following existing patterns:
- Svelte: extract child components, composables, stores
- Rust: extract into sibling modules, use `mod` re-exports

## Related Code Files

| File | LOC | Suggested Split |
|------|-----|-----------------|
| `+page.svelte` | ~1900 | Extract: URL input section, format picker section, playlist section, download actions |
| `api.ts` | large | Split by domain: extract-api, stream-api, batch-api |
| `ytdlp.rs` | ~536 | Extract: cache module, parsing module |
| `stream.rs` | large | Extract: validation, URL building |
| `proxy_pool.rs` | large | Extract: quarantine logic, health checks |

## Implementation Steps

**Do NOT implement as standalone task.** Instead:

1. When a feature/fix touches one of these files, assess if extraction makes sense
2. Extract the most logical section (e.g., if fixing `+page.svelte` playlist logic, extract playlist section to `HomePlaylistSection.svelte`)
3. Keep extractions small — one section per PR
4. Update imports and verify no regressions

## Todo List
- [ ] Track which hotspot files are touched in future PRs
- [ ] Extract sections opportunistically
- [ ] (Deferred — no immediate action required)

## Success Criteria
- Hotspot files gradually shrink below 400 LOC
- No behavior changes from extraction
- New modules follow kebab-case naming convention

## Risk Assessment
- **Low risk** when done incrementally
- **High risk** if attempted as big-bang refactor — avoid this

## Security Considerations
- None

## Next Steps
- Revisit this list quarterly or when files cause merge conflicts
- Prioritize `+page.svelte` first as it's the most impactful
