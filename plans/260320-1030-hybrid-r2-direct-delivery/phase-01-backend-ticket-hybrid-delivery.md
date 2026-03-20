# Phase 01: Backend Ticket + Hybrid Delivery Config

## Context Links
- [plan.md](./plan.md)
- Current file-ticket route: `frontend/src/routes/api/proxy/jobs/[jobId]/file-ticket/+server.ts`
- Current file proxy route: `frontend/src/routes/api/proxy/jobs/[jobId]/file/+server.ts`
- Rust ticket service: `crates/api/src/services/storage_ticket_service.rs`

## Overview
- **Priority**: P1
- **Status**: pending
- **Effort**: 1.5h

Modify the SvelteKit `file-ticket` route to implement hybrid delivery decision. The Rust backend already returns presigned R2 URLs when `MUX_DIRECT_DOWNLOAD=true`. Currently the frontend file-ticket route ignores this and always returns a proxy URL.

## Key Insights

1. Rust `job_file_ticket_handler` (jobs.rs:268-295) calls `build_ticket(&job, state.mux_direct_download)` which returns either a presigned R2 URL or `/api/jobs/:id/file` path
2. Frontend file-ticket route (line 61-64) **always overrides** with `/api/proxy/jobs/:id/file` -- this is the single point of change
3. Frontend file route already handles signed URLs correctly (line 46-51) -- no change needed there
4. The decision logic (direct vs proxy) belongs in the frontend file-ticket route because it has access to `User-Agent` header

## Requirements

### Functional
- file-ticket route returns R2 signed URL for compatible browsers (Chrome, Edge, Firefox desktop/Android)
- file-ticket route returns proxy URL for incompatible browsers (Safari, iOS)
- `DOWNLOAD_DELIVERY_MODE=proxy` disables direct delivery entirely (kill switch)
- `DOWNLOAD_DELIVERY_MODE=direct` skips UA filter — server always returns R2 URL in ticket (but client may still downgrade to proxy if no FSAA — see Phase 02)
- Response payload includes `ticket_delivery` field = **server's decision** (`direct` or `proxy`). This is NOT the actual download path — client may override based on FSAA capability.

### Non-functional
- Zero additional network hops for direct path
- No new npm dependencies
- Backward compatible -- existing clients that only read `download_url` still work

## Architecture

```
file-ticket route (SvelteKit):
  1. Fetch ticket from Rust backend (unchanged)
  2. Read DOWNLOAD_DELIVERY_MODE env
  3. If mode=proxy OR (mode=hybrid AND UA matches DOWNLOAD_PROXY_UA_REGEX):
       Return { download_url: "/api/proxy/jobs/:id/file", ticket_delivery: "proxy" }
  4. Else if backend returned absolute R2 URL:
       Return { download_url: "<signed-r2-url>", ticket_delivery: "direct" }
  5. Else (backend returned relative path, e.g. localfs):
       Return { download_url: "/api/proxy/jobs/:id/file", ticket_delivery: "proxy" }
```

## Related Code Files

### Modify
- `frontend/src/routes/api/proxy/jobs/[jobId]/file-ticket/+server.ts` -- add hybrid decision logic

### Create
- `frontend/src/lib/server/delivery-mode-resolver.ts` -- extract delivery decision logic (testable, reusable for playlist)

### No Change
- `frontend/src/routes/api/proxy/jobs/[jobId]/file/+server.ts` -- already works as proxy fallback
- `crates/api/src/services/storage_ticket_service.rs` -- already returns presigned URLs
- `crates/api/src/routes/jobs.rs` -- no change needed

## Implementation Steps

### Step 1: Create `delivery-mode-resolver.ts`

Location: `frontend/src/lib/server/delivery-mode-resolver.ts`

```typescript
import { env } from '$env/dynamic/private';

// Env-based config
type DeliveryMode = 'hybrid' | 'direct' | 'proxy';

interface DeliveryDecision {
  deliveryMode: 'direct' | 'proxy';
  reason: string;
}

function getDeliveryConfig(): { mode: DeliveryMode; proxyUaRegex: RegExp } {
  const mode = (env.DOWNLOAD_DELIVERY_MODE || 'hybrid') as DeliveryMode;
  const pattern = env.DOWNLOAD_PROXY_UA_REGEX || 'Safari|iPhone|iPad';
  return { mode, proxyUaRegex: new RegExp(pattern, 'i') };
}

export function resolveDeliveryMode(
  userAgent: string | null,
  backendDownloadUrl: string
): DeliveryDecision {
  const { mode, proxyUaRegex } = getDeliveryConfig();
  const isAbsoluteUrl = /^https?:\/\//i.test(backendDownloadUrl);

  // Kill switch
  if (mode === 'proxy') {
    return { deliveryMode: 'proxy', reason: 'config_force_proxy' };
  }

  // Backend returned local path (localfs) -- must proxy
  if (!isAbsoluteUrl) {
    return { deliveryMode: 'proxy', reason: 'backend_local_path' };
  }

  // Force direct for all UAs
  if (mode === 'direct') {
    return { deliveryMode: 'direct', reason: 'config_force_direct' };
  }

  // Hybrid: check UA
  if (userAgent && proxyUaRegex.test(userAgent)) {
    // Avoid false positive: Chrome UA also contains "Safari"
    // Chrome: "Mozilla/5.0 ... Chrome/120 Safari/537.36"
    // Real Safari: "Mozilla/5.0 ... Version/17 Safari/605.1.15"
    const isChrome = /Chrome\//i.test(userAgent);
    const isEdge = /Edg\//i.test(userAgent);
    if (!isChrome && !isEdge) {
      return { deliveryMode: 'proxy', reason: 'ua_match_proxy_pattern' };
    }
  }

  return { deliveryMode: 'direct', reason: 'hybrid_default_direct' };
}
```

### Step 2: Update `file-ticket/+server.ts`

Replace the hardcoded proxy URL override (lines 59-68) with hybrid decision:

```typescript
// Parse backend ticket response
const ticketPayload = (await upstream.json()) as { download_url?: string; job_id?: string };
const backendUrl = ticketPayload.download_url?.trim() || '';
const userAgent = request.headers.get('user-agent');

const decision = resolveDeliveryMode(userAgent, backendUrl);
const downloadUrl = decision.deliveryMode === 'direct'
  ? backendUrl
  : `/api/proxy/jobs/${encodeURIComponent(params.jobId)}/file`;

// Audit log with delivery decision
await logAuditEvent(
  { request, locals, cookies, url },
  {
    scope: 'download',
    eventType: 'job_file_ticket',
    entityId: params.jobId,
    targetLabel: params.jobId,
    statusCode: 200,
    outcome: 'success',
    payload: {
      downloadUrl: decision.deliveryMode === 'direct' ? '[r2-signed]' : downloadUrl,
      ticketDelivery: decision.deliveryMode,
      decisionReason: decision.reason
    }
  }
);

return json(
  {
    download_url: downloadUrl,
    ticket_delivery: decision.deliveryMode
  },
  { headers: applyNoStoreCache(new Headers()) }
);
```

### Step 3: Add env vars to `.env.example`

```
# Download delivery mode: hybrid (default), direct, proxy
DOWNLOAD_DELIVERY_MODE=hybrid
# UA regex for proxy fallback in hybrid mode
DOWNLOAD_PROXY_UA_REGEX=Safari|iPhone|iPad
```

## Todo List
- [ ] Create `delivery-mode-resolver.ts` with `resolveDeliveryMode()` function
- [ ] Handle Chrome-contains-Safari UA edge case in regex matching
- [ ] Update `file-ticket/+server.ts` to use hybrid decision
- [ ] Add `ticket_delivery` field to response payload
- [ ] Add env vars to `.env.example`
- [ ] Verify `MUX_DIRECT_DOWNLOAD=true` is set in production Rust backend env

## Success Criteria
- `file-ticket` returns R2 signed URL for Chrome/Edge/Firefox requests
- `file-ticket` returns proxy URL for Safari/iOS requests
- `DOWNLOAD_DELIVERY_MODE=proxy` forces all traffic through proxy
- Audit log shows `ticketDelivery` and `decisionReason` for every ticket request

## Risk Assessment
- **Chrome UA contains "Safari"**: Mitigated by checking for `Chrome/` in UA string before matching Safari pattern
- **Env var not set**: Defaults to `hybrid` mode, safe fallback
- **SvelteKit env pattern**: MUST use `$env/dynamic/private` (not `import.meta.env`) for server-side code. All existing server files follow this pattern (`rust-api-proxy.ts`, `admin-dashboard.ts`, etc.)
- **Backend returns localfs path**: Detected by checking for absolute URL, falls back to proxy

## Security Considerations
- R2 signed URLs have TTL -- same as existing ticket TTL, no new exposure
- Don't log full signed URL in audit (contains auth signature) -- log `[r2-signed]` placeholder
- Proxy route still requires download session cookie -- unchanged
