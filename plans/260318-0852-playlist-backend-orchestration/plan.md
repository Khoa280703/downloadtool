---
title: "Playlist Backend Orchestration"
description: "Chuyển playlist từ client-orchestrated sang backend-orchestrated, vẫn just-in-time extract và giữ nguyên single download flow."
status: pending
priority: P1
effort: 4d
branch: main
tags: [playlist, backend, jobs, orchestration]
created: 2026-03-18
---

# Playlist Backend Orchestration

Goal: browser chỉ tạo và theo dõi playlist job; backend điều phối item queue, just-in-time extract, retry, progress, cancel/resume.

Current pain:
- Playlist fetch đã ở backend (`/api/batch`) nhưng orchestration tải vẫn ở frontend.
- Frontend worker vẫn gọi `extract()` từng video.
- Rate limit/proxy/backoff bị dính vào browser flow.
- User reload/close tab là mất orchestration cục bộ.

Target:
- Single download giữ nguyên.
- Playlist có parent job + item jobs bền vững.
- Worker quyết định per-item: direct stream, direct copy artifact, hoặc mux artifact.
- Progress realtime cho cả playlist và từng item.

Phases:
| Phase | Status | Summary |
| --- | --- | --- |
| [Phase 1](phase-01-data-model-and-api-contract.md) | pending | Thêm parent/item model + API contract |
| [Phase 2](phase-02-backend-playlist-control-plane.md) | pending | Tạo playlist control plane, lifecycle, cancel/resume |
| [Phase 3](phase-03-playlist-worker-and-item-execution.md) | pending | Worker orchestration + just-in-time extract + retry |
| [Phase 4](phase-04-frontend-migration-and-realtime-progress.md) | pending | Bỏ client worker, chuyển UI sang server progress |
| [Phase 5](phase-05-rollout-tests-and-observability.md) | pending | Test, admin, rollout an toàn |

Key decisions:
- Reuse `mux_jobs` hiện có cho item cần artifact.
- Không phá `/api/jobs` và single `DownloadBtn`.
- Không lấy trước toàn bộ stream URL từ playlist fetch.
- Parent playlist job chỉ lưu metadata + desired mode/quality; item mới extract khi tới lượt.

Dependencies:
- `job_system` hiện có
- Redis/pubsub progress hiện có
- DB migrations mới cho playlist parent/items
- Proxy health + audit hiện có

Done when:
- User tạo playlist job bằng 1 request.
- Reload tab vẫn thấy playlist đang chạy.
- Per-item retry/cancel/resume hoạt động.
- Single flow không regression.

Unresolved questions:
- V1 có cần auto-download lần lượt ngay khi item ready, hay chỉ hiện nút tải từng item?
- V1 có cần artifact hóa cả direct-stream item, hay cho phép `ready_stream` + refresh-on-demand?
