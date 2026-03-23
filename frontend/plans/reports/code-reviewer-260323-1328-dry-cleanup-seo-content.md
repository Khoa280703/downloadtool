# Code Review: DRY Cleanup — SEO Content System

**Date:** 2026-03-23 | **Scope:** 3 files | **Focus:** Import correctness, circular deps, missing exports

## Overall Assessment

**PASS** — Clean DRY refactor. `hubPath()` consolidated into `content-types.ts`, `SITE_URL` sourced from single `public-pages.ts`. No circular dependencies, no missing exports.

## Dependency Graph (verified)

```
content-types.ts        (0 imports — leaf node, exports hubPath + types)
  <- build-page-seo.ts  (imports hubPath, types from content-types + SITE_URL from public-pages)
  <- build-page-schema.ts (imports hubPath, types from content-types + contentPageUrl from build-page-seo + SITE_URL from public-pages)
```

- `content-types.ts` has **zero imports** — no cycle risk
- `build-page-schema.ts -> build-page-seo.ts -> content-types.ts` — acyclic, correct

## Findings

### Medium: Residual SITE_URL in structured-data.ts

`src/lib/seo/structured-data.ts:6` still has `const SITE_URL = 'https://snapvie.com'` as a local constant instead of importing from `public-pages.ts`. This is the last remaining duplicate. The DRY cleanup is incomplete until this is also consolidated.

**Recommendation:** Replace line 6 with `import { SITE_URL } from '$lib/seo/public-pages';` and remove the local const.

### Positive Observations

- `hubPath()` placement in `content-types.ts` is correct — it only depends on `ContentPageType` which is defined in the same file
- Import paths use SvelteKit `$lib` alias for cross-directory imports, relative `./` for same-directory — consistent convention
- Both consumer files (`build-page-seo.ts`, `build-page-schema.ts`) separate type imports from value imports — good practice
- No runtime side effects in any of the changed files
- All files well under 200-line limit

## Metrics

| Metric | Value |
|--------|-------|
| Files reviewed | 3 (+2 upstream verified) |
| Circular deps | 0 |
| Missing exports | 0 |
| Remaining SITE_URL duplicates | 1 (`structured-data.ts`) |
| Critical issues | 0 |

## Recommended Actions

1. **[Medium]** Consolidate `SITE_URL` in `structured-data.ts` to import from `public-pages.ts` — last remaining duplicate
