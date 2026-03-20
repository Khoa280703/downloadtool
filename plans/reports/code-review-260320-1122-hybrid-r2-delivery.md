# Code Review: Hybrid R2 Direct Delivery

**Date:** 2026-03-20
**Reviewer:** code-reviewer
**Scope:** 7 files, ~1050 LOC reviewed (frontend delivery pipeline)

---

## Overall Assessment

**PASS** -- Implementation is solid, all 5 critical constraints are satisfied. The delivery mode resolver is clean and well-documented. Fallback logic is correct. No security vulnerabilities found.

---

## Constraint Verification

### 1. `appendCacheBuster()` skipped for absolute R2 URLs -- PASS
**File:** `frontend/src/lib/api.ts` lines 547-552

```ts
const isAbsoluteUrl = /^https?:\/\//i.test(ticket.download_url);
const downloadUrl = isAbsoluteUrl
    ? ticket.download_url
    : toAbsoluteDownloadUrl(appendCacheBuster(ticket.download_url, cacheBuster));
```

Correctly branches: absolute (R2 signed) URLs bypass cache buster. Relative proxy paths still get `?t=` param. Comment on line 547 explicitly documents the 403 risk.

### 2. Direct R2 download ONLY for FSAA path -- PASS
**File:** `frontend/src/lib/playlist-download-file-saver.ts` lines 81-86

```ts
const requestedDirect = isCrossOriginUrl(url);
const downgraded = !hasFsaa && options.fallbackUrl && requestedDirect;
const effectiveUrl = downgraded ? options.fallbackUrl! : url;
```

When no FSAA handle exists and URL is cross-origin (R2), automatically downgrades to proxy. Additional FSAA failure fallback at lines 98-104 retries with `options.fallbackUrl`. Anchor fallback also uses `effectiveUrl` (already proxy-downgraded).

### 3. `$env/dynamic/private` for server env -- PASS
**File:** `frontend/src/lib/server/delivery-mode-resolver.ts` line 1

```ts
import { env } from '$env/dynamic/private';
```

Correct SvelteKit pattern. Consistent with all other server modules (auth.ts, rust-api-proxy.ts, etc.).

### 4. API response `ticket_delivery` (snake) / Audit `ticketDelivery` (camel) -- PASS
**File:** `frontend/src/routes/api/proxy/jobs/[jobId]/file-ticket/+server.ts`
- Line 78: `ticketDelivery: decision.deliveryMode` (audit payload, camelCase)
- Line 89: `ticket_delivery: decision.deliveryMode` (API response, snake_case)

Type definition at `api.ts:72`: `ticket_delivery?: 'direct' | 'proxy'` matches.

### 5. R2 signed URL NOT logged in audit -- PASS
**File:** `frontend/src/routes/api/proxy/jobs/[jobId]/file-ticket/+server.ts` lines 61-62

```ts
const auditDownloadUrl = decision.deliveryMode === 'direct' ? '[r2-signed]' : proxyPath;
```

Signed URL is masked. Also in `file/+server.ts` line 72: only logs `new URL(upstreamUrl).host`, not the full signed URL.

---

## Edge Cases Scouted

### E1. RegExp creation on every request (LOW)
**File:** `delivery-mode-resolver.ts` line 21

`getDeliveryConfig()` creates a new `RegExp(pattern, 'i')` on every call to `resolveDeliveryMode`. Not cached. For a download ticket endpoint (low RPS), this is negligible. If RPS grows, consider module-level caching.

**Impact:** Minimal. No action needed now.

### E2. `new URL()` in audit can throw on malformed backend URLs (LOW)
**File:** `file-ticket/+server.ts` line 64

```ts
(() => { try { return new URL(backendUrl).hostname; } catch { return 'unknown'; } })()
```

Already wrapped in try/catch. Safe.

### E3. Proxy fallback file endpoint doesn't mask signed URL in audit hostname (LOW)
**File:** `file/+server.ts` line 72

```ts
upstreamTarget: viaSignedUrl ? new URL(upstreamUrl).host : rustFallbackPath,
```

Logs only `.host` (hostname + port), not the full signed URL. This is acceptable -- host is not secret (it's the public R2 bucket domain).

### E4. Chrome UA detection handles Edge correctly
**File:** `delivery-mode-resolver.ts` lines 60-62

Edge UA includes both `Chrome/` and `Edg/`. The check tests `Edg/` first in `classifyUserAgentFamily` (line 76) but in `resolveDeliveryMode` the order doesn't matter -- both Chrome and Edge bypass proxy. Correct.

### E5. Non-mux direct stream path has no `muxJobId` -- correct
**File:** `playlist-download-worker.ts` line 417

Direct stream entries don't set `muxJobId`, so `saveReadyEntry` (line 278) gets `fallbackUrl = undefined`. Since direct streams go through `/api/stream` (same origin), `isCrossOriginUrl` returns false, no fallback needed. Correct.

### E6. `ticket.download_url` could be undefined from backend (MEDIUM)
**File:** `file-ticket/+server.ts` line 50

```ts
const backendUrl: string = ticket.download_url ?? '';
```

If backend returns `download_url: undefined` or missing, `backendUrl = ''`. The resolver sees it's not absolute -> returns `proxy` mode. The client gets `proxyPath` as `download_url`. **This is safe** -- degrades gracefully to proxy.

### E7. Race: FSAA fallback retry doesn't re-check FSAA handle availability
**File:** `playlist-download-file-saver.ts` line 103

If `saveDirectoryHandle` becomes null between first attempt and fallback (theoretically impossible since it's module-scoped and only set by `pickSaveDirectory`), the fallback still uses the captured `saveDirectoryHandle` from the outer scope. Safe.

### E8. Direct stream downloads (non-mux) in DownloadBtn don't get R2 URLs
**File:** `DownloadBtn.svelte` lines 453-466

`buildStreamUrl()` returns a relative `/api/stream?...` URL. This is same-origin, so `isCrossOriginUrl` returns false, no cross-origin issues. `fallbackUrl` is only set for mux jobs. Correct.

---

## High Priority

### H1. Duplicate code: `formatMuxStatus`, `clampProgressPercent`, `resolveMuxPhaseLabel`, `resolveQueuedMuxLabel`
**Files:** `DownloadBtn.svelte` + `playlist-download-worker.ts`

These 4 functions are copied verbatim between both files (~100 lines duplicated). DRY violation.

**Recommendation:** Extract to a shared module like `$lib/mux-status-format.ts`. Both consumers import from there.

**Impact:** Maintainability. If status labels change, must update two places.

---

## Medium Priority

### M1. `getDeliveryConfig()` not called at module scope -- repeated parsing
**File:** `delivery-mode-resolver.ts`

The function reads `env.DOWNLOAD_DELIVERY_MODE` and constructs a RegExp each invocation. Since env vars don't change at runtime in SvelteKit, this could be a lazy singleton.

**Recommendation:** Low priority. Only matters at high RPS.

### M2. `file/+server.ts` proxy endpoint doesn't send Rust auth headers to R2 signed URLs -- correct but fragile
**File:** `file/+server.ts` line 53

```ts
headers: viaSignedUrl ? undefined : rustHeaders,
```

Correctly omits auth headers for R2 signed URLs (they'd cause signature mismatch). But if future middleware adds default headers, this could break. The current approach is correct.

### M3. No explicit CORS validation on `backendUrl` in resolver
**File:** `delivery-mode-resolver.ts`

The resolver decides `direct` without knowing if R2 bucket has CORS configured. This is by design (Phase 3 of rollout plan handles CORS). But if deployed before CORS setup with `DOWNLOAD_DELIVERY_MODE=direct`, direct downloads will fail with CORS errors.

**Mitigation:** Default `hybrid` mode + client-side `isCrossOriginUrl` downgrade in file-saver provides safety net. The plan addresses this in Phase 5 rollout.

---

## Low Priority

### L1. Console logging in file-saver includes full URL for direct downloads
**File:** `playlist-download-file-saver.ts` lines 93, 100-101

Logs `effectiveUrl` which for direct mode contains the signed R2 URL. This is browser console only (not server audit), but could expose signed URLs in error reporting tools (Sentry, etc.).

**Recommendation:** Consider masking cross-origin URLs in console logs, similar to audit masking.

### L2. `buildFilename` in DownloadBtn doesn't sanitize for filesystem
**File:** `DownloadBtn.svelte` line 241

```ts
const safeTitle = title.replace(/[^a-z0-9]/gi, '_');
```

This is fine for most cases but aggressive (replaces spaces, unicode). The playlist worker uses `safeFilename()` from stream-selection module. Consider unifying.

---

## Positive Observations

1. **Clear separation of concerns**: Server-side delivery decision (`delivery-mode-resolver.ts`) is isolated, testable, well-documented
2. **Defense in depth**: Even if server returns `direct`, client-side `isCrossOriginUrl` check in file-saver prevents broken downloads without FSAA
3. **Graceful degradation chain**: Direct -> FSAA fallback to proxy -> anchor fallback. Three levels of resilience
4. **Kill switch**: `DOWNLOAD_DELIVERY_MODE=proxy` env var instantly reverts all traffic. No deploy needed
5. **Audit hygiene**: Signed URLs consistently masked, delivery mode and decision reason logged for debugging
6. **UA handling**: Chrome/Edge "Safari" substring false-positive correctly handled

---

## Recommended Actions

1. **[HIGH]** Extract duplicated mux status formatting to shared module (~100 LOC saved)
2. **[LOW]** Consider masking signed URLs in browser console logs
3. **[LOW]** Unify filename sanitization between DownloadBtn and playlist worker
4. **[INFO]** Ensure CORS on R2 bucket is configured before switching to `hybrid` mode (Phase 3/5 of rollout plan)

---

## Metrics

- Type Coverage: Good -- `FileTicketApiResponse` typed, `DeliveryDecision` exported, `ticket_delivery` union typed
- Test Coverage: Not assessed (no test files in scope)
- Linting Issues: 0 critical, 0 syntax errors detected
- Security Issues: 0 (signed URL masking, no SSRF vectors, proper env handling)

---

## Unresolved Questions

1. Is CORS on the R2 bucket already configured? If not, `hybrid` mode will cause FSAA direct fetches to fail (client will auto-fallback to proxy, but with latency penalty)
2. Should the duplicated mux status formatting be addressed in this PR or deferred?
