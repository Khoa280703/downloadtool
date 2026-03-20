# Phase 04: Observability + Audit Logging

## Context Links
- [plan.md](./plan.md)
- Audit log module: `frontend/src/lib/server/audit-log.ts`
- file-ticket route: `frontend/src/routes/api/proxy/jobs/[jobId]/file-ticket/+server.ts`
- file route: `frontend/src/routes/api/proxy/jobs/[jobId]/file/+server.ts`

## Overview
- **Priority**: P2
- **Status**: pending
- **Effort**: 1h

Add structured logging to track delivery mode decisions, fallback events, and download performance. Enable admin visibility into direct vs proxy traffic split.

## Key Insights

1. Audit log system already exists (`logAuditEvent`) with `payload_json` JSONB column
2. file-ticket route already logs -- just need to enrich payload (done in Phase 01)
3. file route logs `viaSignedUrl` -- this means **proxy server** fetched upstream via signed URL, NOT browser direct. Browser-direct downloads bypass this route → no `job_file` row.
4. **Two-level delivery tracking**:
   - `job_file_ticket.ticket_delivery` = **server's decision** (direct or proxy based on UA + config)
   - Client may **downgrade** direct→proxy if no FSAA (anchor path). Client logs `actualDelivery` to console.
   - `job_file` rows = **always proxy traffic** (direct never hits this route). `isFallback` flag distinguishes primary proxy vs FSAA-failed-then-proxy.
   - **No single authoritative source for actual browser delivery mode.** Server knows what it offered; proxy route knows what went through it; neither knows if browser actually fetched R2 directly. Accept this gap — approximate with: `ticket_delivery=direct` AND no `job_file` row for same jobId within window = likely browser-direct.
5. Client-side logging `console.info('[downloadtool] saveDownload', { actualDelivery, ... })` captures real path (done in Phase 02)

## Requirements

### Server-Side (file-ticket route)
Already covered in Phase 01 implementation:
- `ticketDelivery`: 'direct' | 'proxy'
- `decision_reason`: why that mode was chosen
- `download_host`: R2 bucket host or 'proxy'

### Server-Side (file route -- proxy fallback)
Enrich existing audit log:
- `is_fallback`: boolean -- true when client used proxy after direct failed
- `bytes_sent`: from Content-Length if available
- `duration_ms`: request duration
- `effective_mbps`: bytes_sent / duration

### Client-Side
Console logging (already in Phase 02 implementation):
- `[downloadtool] direct download failed, falling back to proxy` with error details
- `[downloadtool] download completed` with delivery mode

## Implementation Steps

### Step 1: Enrich file route audit log

In `frontend/src/routes/api/proxy/jobs/[jobId]/file/+server.ts`:

```typescript
const startTime = Date.now();
// ... existing fetch logic ...
const durationMs = Date.now() - startTime;
const contentLength = upstream.headers.get('content-length');
const bytesSent = contentLength ? parseInt(contentLength, 10) : null;
const isFallback = url.searchParams.has('fallback');

await logAuditEvent(
  { request, locals, cookies, url },
  {
    scope: 'download',
    eventType: 'job_file',
    entityId: params.jobId,
    statusCode: upstream.status,
    outcome: deriveAuditOutcome(upstream.status),
    payload: {
      viaSignedUrl,
      upstreamTarget: viaSignedUrl ? new URL(upstreamUrl).host : rustFallbackPath,
      isFallback,
      durationMs,
      bytesSent,
      effectiveMbps: bytesSent && durationMs > 0
        ? ((bytesSent * 8) / (durationMs / 1000) / 1_000_000).toFixed(2)
        : null
    }
  }
);
```

### Step 2: Add `?fallback=1` param in client fallback

When client falls back to proxy (Phase 02), append `?fallback=1` to proxy URL:

```typescript
// In saveDownload fallback path:
const fallbackWithMarker = options.fallbackUrl.includes('?')
  ? `${options.fallbackUrl}&fallback=1`
  : `${options.fallbackUrl}?fallback=1`;
```

This allows the proxy route to distinguish organic proxy requests from fallback requests.

### Step 3: Add UA family to file-ticket audit log

Parse UA into family for aggregation:

```typescript
function classifyUserAgentFamily(ua: string | null): string {
  if (!ua) return 'unknown';
  if (/Edg\//i.test(ua)) return 'edge';
  if (/Chrome\//i.test(ua) && !/Edg\//i.test(ua)) return 'chrome';
  if (/Firefox\//i.test(ua)) return 'firefox';
  if (/Safari\//i.test(ua)) return 'safari';
  return 'other';
}
```

Add to file-ticket audit payload:
```typescript
payload: {
  ticketDelivery: decision.deliveryMode,
  decisionReason: decision.reason,
  userAgentFamily: classifyUserAgentFamily(userAgent),
  downloadHost: decision.deliveryMode === 'direct' ? 'r2' : 'proxy'
}
```

## Related Code Files

### Modify
- `frontend/src/routes/api/proxy/jobs/[jobId]/file/+server.ts` -- add duration/bytes/fallback logging
- `frontend/src/lib/server/delivery-mode-resolver.ts` -- add `classifyUserAgentFamily()` (from Phase 01)

### No Change
- `frontend/src/lib/server/audit-log.ts` -- existing infra sufficient

## Todo List
- [ ] Add duration + bytes tracking to file proxy route
- [ ] Add `?fallback=1` query param in client fallback path
- [ ] Add UA family classification to file-ticket audit log
- [ ] Add `classifyUserAgentFamily()` helper
- [ ] Verify audit_events table can be queried for delivery mode stats

## Success Criteria
- Admin can query: `SELECT payload_json->>'ticketDelivery', COUNT(*) FROM audit_events WHERE event_type='job_file_ticket' GROUP BY 1` — shows server decisions (client may downgrade direct→proxy if no FSAA)
- Fallback rate visible: `SELECT COUNT(*) FILTER (WHERE payload_json->>'isFallback' = 'true') FROM audit_events WHERE event_type='job_file'`
- Download performance per mode queryable

## Risk Assessment
- **Audit log volume**: file-ticket + file routes already log. Adding fields to existing payload -- no extra rows.
- **Duration measurement**: Measures server-side proxy duration, not client-side. Good enough for comparison.
