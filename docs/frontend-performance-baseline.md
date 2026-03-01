# Frontend Performance Baseline

**Last Baseline:** 2026-02-27 (local lab)
**Status Update:** 2026-03-01 (optimizations applied, awaiting new measurement)

## Scope

- URL: `http://127.0.0.1:4173/`
- Build mode: production (`pnpm --filter frontend build` + `pnpm --filter frontend preview`)
- Tool: Lighthouse CLI
- Command:

```bash
npx -y lighthouse http://127.0.0.1:4173 \
  --only-categories=performance \
  --chrome-flags='--headless --no-sandbox' \
  --quiet --output=json --output-path=/tmp/lh-frontend.json
```

## Baseline Result (2026-02-27, median of 2 runs)

- Performance Score: `75`
- LCP (Largest Contentful Paint): `~7.51s` ⚠️ (bottleneck)
- FCP (First Contentful Paint): `~1.66s` ✅
- CLS (Cumulative Layout Shift): `~0.046` ✅ (< 0.1)
- TBT (Total Blocking Time): `0ms` ✅

**Root Cause Analysis:**
- LCP bottleneck: External font + external images loading in series
- Primary issue: Material Symbols font (1.1 MB) blocking text rendering
- Secondary: Large hero image from remote URL

---

## Optimizations Applied (2026-03-01) ✅

### 1. Font Optimization
```
Material Symbols: 1.1 MB → 4.5 KB (27-icon subset)
+ font-display: swap (non-blocking rendering)
+ Preload CSS: <link rel="preload" as="style">
```
**Expected LCP improvement:** 7.51s → ~2-3s (pending measurement)

### 2. Homepage Prerendering
```
export const prerender = true
- Removed +page.server.ts (no server data)
- Static HTML → instant load, CDN cacheable
```
**Expected FCP improvement:** 1.66s → ~0.5s

### 3. Cookie Check Optimization
```
hooks.server.ts: Skip DB query if no better-auth cookie
- Saves ~200ms per unauthenticated visitor
- 95%+ DB query reduction for anonymous users
```

### 4. Lazy Loading on Images
```
<img loading="lazy" fetchpriority="low" src="...">
- External images load on demand
- Non-critical images deprioritized
```

---

## Next Steps

1. **Measure new baseline** (post-optimizations) with Lighthouse
   ```bash
   npx lighthouse http://127.0.0.1:4173 --only-categories=performance --output=json
   ```

2. **Verify LCP target:** < 2.5s (Good: Web Vitals)
3. **Responsive images:** Implement srcset for different viewports (if LCP still high)
4. **CDN cache validation:** Check Cloudflare edge cache (see `docs/cloudflare-cache-checklist.md`)
