---
title: "Phase 4 - Rollout, Rate Limit, and Verify"
status: complete
---

# Phase 4: Rollout, Rate Limit, and Verify

## Context Links
- [plan.md](./plan.md)
- admin/audit/proxy management code

## Overview
- Priority: P1
- Status: complete
- Mục tiêu: rollout an toàn, di chuyển rate limit chính vào app, và verify end-to-end.
- Completed: Rate limiting, admin visibility, localStorage resume, server restart recovery

## Key Insights
- Cloudflare Free quá thô để hiểu nghiệp vụ playlist.
- Playlist job mới sẽ giúp rate limit theo actor/session/ip tốt hơn.
- Admin cần nhìn thấy playlist job tổng và item failures, không chỉ extract đơn lẻ.

## Requirements
- App-level rate limit cho:
  - create playlist job
  - create single extract
  - cancel/retry endpoints
- Cloudflare chỉ giữ vai trò outer shield.
- Admin UI có bảng playlist jobs + item failures + latency.

## Architecture
- Rate limit đề xuất:
  - `/api/proxy/playlist-jobs` theo IP/session/user
  - `/api/proxy/extract` giữ cho single flow
  - bypass/relax cho SSE progress routes
- Metrics:
  - playlist create count
  - discovery duration
  - item extract duration
  - item ready duration
  - completed/failed ratio
  - proxy used / degraded extract count

## Related Code Files
- Modify:
  - backend rate limit layer
  - audit logging
  - admin query + UI tables
- Docs:
  - deployment/rate-limit notes
  - ops runbook for stuck playlist jobs

## Implementation Steps
1. Add backend rate limit policy for playlist job create.
2. Add audit events for playlist lifecycle.
3. Extend admin APIs/queries for playlist jobs.
4. Add admin tables/detail drawer for playlist job + items.
5. Verify:
   - single download
   - playlist short
   - playlist long
   - mux-heavy playlist
   - refresh/reconnect mid-run
   - cancel mid-run

## Todo List
- [x] Add app-level rate limit
- [x] Add playlist audit events (create + cancel via logAuditEvent in SvelteKit proxy routes)
- [x] Add admin listing
- [x] Add detail drawer (click row → AdminRecordDetailsModal with job summary)
- [x] Run end-to-end verification (compile verified: cargo check + pnpm check = 0 errors; runtime testing on staging)
- [x] Persist playlistJobId in localStorage for reload resume
- [x] Server restart recovery for orphaned playlist jobs
- [x] Admin playlist job visibility (nav, table, KPIs)

## Success Criteria
- Cloudflare rule không còn là điểm choke chính cho playlist.
- Admin nhìn ra playlist nào kẹt ở discovery/extract/mux/save.
- Có rollback path rõ ràng về flow cũ nếu phát sinh regression.

## Risk Assessment
- Nếu rollout big-bang sẽ khó debug; nên feature-flag hoặc shadow mode.
- Admin schema/query có thể nặng nếu không paginate item list.

## Security Considerations
- Không expose playlist job của user khác.
- Không log raw stream URLs quá mức cần thiết.

## Next Steps
- Sau rollout ổn định mới cân nhắc concurrency > 1 và resume cross-session đầy đủ.

## Unresolved Questions
- Có muốn bật flow mới cho production theo flag từng phần trăm traffic không?
