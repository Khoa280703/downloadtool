---
title: "Backend Playlist Orchestration"
description: "Chuyển playlist download từ client-orchestrated sang backend-orchestrated, vẫn giữ just-in-time extract để tránh stream URL hết hạn."
status: complete
priority: P1
effort: 2-4 ngày
branch: main
tags: [playlist, backend, jobs, sse, extract, mux]
created: 2026-03-18
completed: 2026-03-18
---

# Backend Playlist Orchestration

## Goal
- Backend trở thành nơi điều phối playlist.
- Frontend chỉ tạo job, subscribe progress, và lưu file khi item ready.
- Mỗi item chỉ extract khi sắp xử lý, không giữ stream URL quá lâu.
- Không phá flow single download hiện tại.

## Architecture Shift
- Hiện tại: frontend fetch playlist list, frontend queue từng item, frontend gọi `extract()` cho từng video.
- Mục tiêu: frontend tạo `playlist job`, backend discover + queue + retry + track progress, frontend chỉ consume item-ready events.

## Phases
| # | Phase | File | Status |
|---|-------|------|--------|
| 1 | Data model + API contract | [phase-01-data-model-and-api-contract.md](./phase-01-data-model-and-api-contract.md) | complete |
| 2 | Backend orchestration engine | [phase-02-backend-playlist-orchestration-engine.md](./phase-02-backend-playlist-orchestration-engine.md) | complete |
| 3 | Frontend thin client migration | [phase-03-frontend-thin-client-migration.md](./phase-03-frontend-thin-client-migration.md) | complete |
| 4 | Rollout, rate limiting, verify | [phase-04-rollout-rate-limit-and-verify.md](./phase-04-rollout-rate-limit-and-verify.md) | complete |

> **Note:** All 4 phases complete. Backend playlist orchestration live.

## Non-goals
- Không rewrite single download flow.
- Không extract trước toàn bộ stream URLs cho cả playlist.
- Không thêm feature premium/quota mới trong cùng đợt.

## Success Criteria
- Playlist download không còn phụ thuộc browser queue cho orchestration.
- Browser refresh/tab close không làm mất trạng thái backend playlist job.
- Không còn burst `POST /api/proxy/extract` từ frontend cho mỗi playlist.
- Progress admin/UI phản ánh state playlist job và item job theo realtime SSE.

## Related Code
- `crates/api/src/routes/batch.rs`
- `frontend/src/routes/+page.svelte`
- `frontend/src/lib/playlist-download-worker.ts`
- `frontend/src/lib/api.ts`
- `frontend/src/stores/batch.ts`
- existing mux job control plane / worker code

## Risks
- Browser save step vẫn phải ở client, nhất là anchor fallback.
- Playlist auto-download nhiều file có thể bị browser policy chặn nếu không dùng FSAA.
- Refactor đụng cả API contract, persistence, SSE, và admin observability.

## Rollout Note
- Ưu tiên feature-flag hoặc route mới song song, giữ flow cũ để rollback nhanh.

## Unresolved Questions
- Có muốn playlist backend job hỗ trợ resume sau khi user quay lại trang hay chỉ cần theo session hiện tại?
- Có muốn gom playlist thành zip/tar ở server về lâu dài hay vẫn giữ từng file riêng?
