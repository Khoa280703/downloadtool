# Phase 05: Rollout + Testing

## Context Links
- [plan.md](./plan.md)
- Phases [01](./phase-01-backend-ticket-hybrid-delivery.md), [02](./phase-02-client-consume-hybrid-ticket.md), [03](./phase-03-cors-r2-verification.md), [04](./phase-04-observability-audit-logging.md)

## Overview
- **Priority**: P2
- **Status**: pending
- **Effort**: 1h

Define rollout plan, test matrix, and verification steps for hybrid R2 direct delivery.

## Rollout Plan

### Stage 1: Deploy Code (Day 0)
- Deploy Phases 01-04 with `DOWNLOAD_DELIVERY_MODE=proxy` (kill switch ON)
- Verify no regression -- all downloads go through proxy as before
- Verify audit logs show `ticketDelivery: proxy`, `decisionReason: config_force_proxy`

### Stage 2: Enable CORS (Day 0)
- Configure R2 CORS (Phase 03)
- Verify with manual curl tests
- Keep `DOWNLOAD_DELIVERY_MODE=proxy`

### Stage 3: Enable Hybrid (Day 1)
- Set `DOWNLOAD_DELIVERY_MODE=hybrid`
- Default regex `Safari|iPhone|iPad` routes Safari/iOS to proxy
- Desktop Chrome/Edge/Firefox get direct R2 URLs in ticket responses; no-FSAA clients may still downgrade to proxy in Phase 02
- Monitor for 24h

### Stage 4: Monitor (Day 1-3)
- Check audit logs:
  - Fallback rate (`isFallback=true` / total proxy requests)
  - Direct vs proxy split by UA family
  - Download duration comparison (direct vs proxy)
  - Any unexpected errors
- Expected: <5% fallback rate for desktop browsers

### Stage 5: Expand (Day 3+)
- If stable, narrow proxy regex: `iPhone|iPad` (allow Safari macOS)
- Monitor another 24h
- If still stable, consider `DOWNLOAD_DELIVERY_MODE=direct` to skip UA filtering entirely
- Note: In this phase, actual browser-direct remains FSAA-only. Non-FSAA/anchor clients still downgrade to proxy until a later anchor-direct phase is implemented

### Kill Switch
At any point: `DOWNLOAD_DELIVERY_MODE=proxy` reverts all traffic to proxy. No code deploy needed, just env var change + restart.

## Test Matrix

### Manual Testing

| Scenario | Browser | Expected | Verify |
|----------|---------|----------|--------|
| Mux + FSAA | Chrome Desktop (FSAA on) | Direct R2 URL | Network tab shows R2 domain |
| Mux + no FSAA (anchor) | Chrome Desktop (no FSAA) | Proxy URL | Network tab shows same-origin (anchor path always proxy) |
| Mux + FSAA | Edge Desktop (FSAA on) | Direct R2 URL | File saves correctly |
| Mux (any) | Safari macOS | Proxy URL | Network tab shows same-origin (UA blocked) |
| Mux + FSAA | Chrome Android (FSAA on) | Direct R2 URL | File saves correctly |
| Mux (any) | Safari iPhone | Proxy URL | File saves correctly (UA blocked) |
| Single direct stream | Any | `/api/stream` URL | Unchanged behavior |
| Playlist + FSAA | Chrome Desktop (FSAA on) | Direct R2 URLs per item | All items download |
| Playlist + no FSAA | Chrome Desktop (no FSAA) | Proxy URLs per item | Anchor path → proxy |
| FSAA direct fail | Chrome + block R2 | Retry via proxy | Console shows fallback log |
| No FSAA direct fail | Chrome + block R2 | N/A — already using proxy | No fallback needed |
| Kill switch ON | Any | Proxy URL | Audit shows config_force_proxy |
| FSAA + direct R2 progress | Chrome + FSAA | Progress bar works | Content-Length exposed via CORS |

### Automated Verification

```sql
-- After enabling hybrid, run these queries:

-- 1. Delivery mode distribution
SELECT
  payload_json->>'ticketDelivery' as mode,
  payload_json->>'decisionReason' as reason,
  COUNT(*)
FROM audit_events
WHERE event_type = 'job_file_ticket'
  AND created_at > NOW() - INTERVAL '24 hours'
GROUP BY 1, 2
ORDER BY 3 DESC;

-- 2. Fallback rate
SELECT
  COUNT(*) FILTER (WHERE (payload_json->>'isFallback')::boolean = true) as fallback_count,
  COUNT(*) as total_proxy,
  ROUND(
    100.0 * COUNT(*) FILTER (WHERE (payload_json->>'isFallback')::boolean = true) / NULLIF(COUNT(*), 0),
    2
  ) as fallback_pct
FROM audit_events
WHERE event_type = 'job_file'
  AND created_at > NOW() - INTERVAL '24 hours';

-- 3. Server ticket decisions (what server offered — client may downgrade)
-- ticket_delivery=direct means server returned R2 URL. Client may still
-- use proxy if no FSAA. To estimate actual direct: count ticket_delivery=direct
-- minus job_file rows for same jobIds (those went through proxy anyway).
SELECT
  payload_json->>'ticketDelivery' as ticket_decision,
  payload_json->>'userAgentFamily' as ua_family,
  COUNT(*) as ticket_count
FROM audit_events
WHERE event_type = 'job_file_ticket'
  AND created_at > NOW() - INTERVAL '24 hours'
GROUP BY 1, 2
ORDER BY 3 DESC;

-- 3b. Estimate actual browser-direct downloads
-- Jobs with ticket=direct that never hit proxy route = likely browser-direct
SELECT
  t.cnt as tickets_direct,
  p.cnt as then_proxied,
  t.cnt - COALESCE(p.cnt, 0) as likely_browser_direct
FROM
  (SELECT COUNT(*) as cnt FROM audit_events
   WHERE event_type = 'job_file_ticket'
     AND payload_json->>'ticketDelivery' = 'direct'
     AND created_at > NOW() - INTERVAL '24 hours') t,
  (SELECT COUNT(DISTINCT ae2.entity_id) as cnt FROM audit_events ae2
   WHERE ae2.event_type = 'job_file'
     AND ae2.created_at > NOW() - INTERVAL '24 hours'
     AND ae2.entity_id IN (
       SELECT entity_id FROM audit_events
       WHERE event_type = 'job_file_ticket'
         AND payload_json->>'ticketDelivery' = 'direct'
         AND created_at > NOW() - INTERVAL '24 hours'
     )) p;

-- 4. Proxy-only performance (file route = always proxy traffic)
-- viaSignedUrl here means the PROXY fetched upstream via R2 signed URL,
-- NOT that the browser downloaded direct. All rows in job_file are proxy.
SELECT
  CASE WHEN (payload_json->>'isFallback')::boolean THEN 'proxy_fallback' ELSE 'proxy_primary' END as proxy_type,
  AVG((payload_json->>'durationMs')::numeric) as avg_duration_ms,
  AVG((payload_json->>'effectiveMbps')::numeric) as avg_mbps
FROM audit_events
WHERE event_type = 'job_file'
  AND created_at > NOW() - INTERVAL '24 hours'
GROUP BY 1;
```

## Todo List
- [ ] Deploy with `DOWNLOAD_DELIVERY_MODE=proxy`
- [ ] Configure R2 CORS
- [ ] Verify CORS with curl
- [ ] Switch to `DOWNLOAD_DELIVERY_MODE=hybrid`
- [ ] Run manual test matrix
- [ ] Monitor 24h -- check fallback rate
- [ ] Run SQL verification queries
- [ ] Document results

## Success Criteria
- **FSAA-enabled** desktop browsers download direct from R2 (subset of desktop traffic — users who selected save directory)
- **Non-FSAA** desktop browsers use proxy (anchor path, no JS error signal for fallback)
- Safari/iOS always proxy (UA-blocked)
- Admin sees delivery mode split in `job_file_ticket` audit logs
- Download speed measurably faster for FSAA+direct large files vs proxy
- No regression in single video or playlist download flows
- Kill switch verified working
- **Phase 2 expansion**: After validating direct reliability, consider allowing anchor+direct for desktop allowlist

## Risk Assessment
- **Unexpected browser behavior**: Mitigated by gradual rollout (proxy first, then hybrid)
- **R2 rate limiting**: Unlikely for download volumes. Monitor R2 metrics.
- **Ad blocker interference**: Some blockers may block cross-origin downloads. Client retry handles this automatically.
- **Rapid playlist downloads blocked by browser**: Existing throttling in playlist worker handles this. Direct R2 URLs may actually be better since they come from different origin (no same-origin connection limit).
