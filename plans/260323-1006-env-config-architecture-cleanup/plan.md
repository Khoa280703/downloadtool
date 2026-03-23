---
title: "Clean Env and Config Architecture"
description: "Rút gọn contract cấu hình giữa runtime profiles, env và docker-compose để operator dễ quản lý trên Coolify."
status: pending
priority: P2
effort: 2h
branch: main
tags: [config, env, docker, coolify, cleanup]
created: 2026-03-23
---

# Env/Config Cleanup

## Mục tiêu

Làm rõ 3 lớp config, bỏ duplicate/hardcode sai chỗ, và chốt một env contract gọn cho operator:

1. `config/runtime-limit-profiles.json` chỉ giữ app behavior limits.
2. Env thật giữ toàn bộ operator knobs cần chỉnh khi vận hành.
3. `docker/docker-compose.server.yml` chỉ giữ internal wiring giữa service trong production stack.

## Hiện trạng ngắn

- `runtime-limit-profiles.json` đang là nơi hợp lý cho retry/timeout/concurrency phía app.
- `.env` và `.env.production` đã giữ phần lớn runtime knobs nhưng chưa đầy đủ contract vận hành.
- `docker-compose.server.yml` vẫn còn một số `INTERNAL_MUX_*` đang đóng vai operator knob:
  - `INTERNAL_MUX_JOB_MAX_ATTEMPTS`
  - `INTERNAL_MUX_WORKER_LEASE_SECS`
  - `INTERNAL_MUX_WORKER_RECLAIM_LIMIT`
  - `INTERNAL_MUX_QUEUE_STREAM`
  - `INTERNAL_MUX_QUEUE_GROUP`

## Kiến trúc đích

### 1. Runtime profiles

Giữ nguyên ở `config/runtime-limit-profiles.json`:

- backend stream/extract guards
- frontend retry/backoff/polling/jitter/circuit cooldown
- playlist orchestration behavior limits

Không chuyển các giá trị này sang Coolify vì đây là app-level behavior, không phải wiring hạ tầng.

### 2. Env operator knobs

Đưa về env thật, được khai báo trong `.env`, `.env.production`, và Coolify nếu dùng production:

- `PROXY_QUARANTINE_TTL_SECS`
- `MUX_ARTIFACT_TTL_SECS`
- `MUX_CLEANUP_INTERVAL_SECS`
- `MUX_CLEANUP_BATCH_LIMIT`
- `MUX_FILE_TICKET_TTL_SECS`
- `MUX_WORKER_CONCURRENCY`
- `MUX_JOB_MAX_ATTEMPTS`
- `MUX_WORKER_LEASE_SECS`
- `MUX_WORKER_RECLAIM_LIMIT`

Quy ước:

- Đây là các knob vận hành, không nên hidden default trong code nếu đội vẫn muốn chủ động chỉnh khi production có tải.
- `.env.production` là reference snapshot cho operator, không phải runtime source of truth.

### 3. Docker compose server

`docker/docker-compose.server.yml` chỉ nên giữ internal wiring:

- `INTERNAL_DATABASE_URL`
- `INTERNAL_PROXY_DATABASE_URL`
- `INTERNAL_REDIS_URL`
- `INTERNAL_PROXY_REDIS_URL`
- `INTERNAL_RUST_API_URL`

Không giữ các `INTERNAL_MUX_*` nếu chúng thực chất là operator knobs.

## Phases

| Phase | Mục tiêu | Output |
|-------|----------|--------|
| 01 | Inventory toàn bộ env/config hiện có | Bảng phân loại `runtime-profile` / `env` / `compose` |
| 02 | Chốt env contract mới | Danh sách biến operator knobs và biến internal wiring |
| 03 | Đồng bộ file quản lý env | `.env` và `.env.production` phản ánh đúng contract mới |
| 04 | Chốt Coolify keep/delete list | Checklist env cần giữ, env có thể xóa |

## Quy tắc phân loại

- Nếu giá trị dùng để tinh chỉnh hành vi app theo môi trường và có thể cần đổi lúc vận hành: để ở env.
- Nếu giá trị chỉ là địa chỉ nội bộ service/container: để ở compose.
- Nếu giá trị là retry/concurrency/backoff guard thuộc logic ứng dụng: để ở runtime profiles.
- Tránh 1 biến xuất hiện đồng thời ở code default + compose + Coolify nếu nó là cùng một knob.

## Deliverables

- Một env contract cuối cùng, không mơ hồ.
- `.env` sạch cho local operator.
- `.env.production` sạch để tham chiếu production.
- Danh sách env Coolify nên giữ.
- Danh sách env Coolify nên xóa.

## Success Criteria

- Không còn `INTERNAL_*` nào trong compose mà thực chất là operator knob.
- Không còn duplicate config giữa compose và env cho cùng một trách nhiệm.
- Operator có thể nhìn `.env.production` là biết đầy đủ knobs production cần quản lý.
- Coolify chỉ giữ secret, public endpoint, và operator knobs thật sự dùng ở runtime production.

## Rủi ro / Lưu ý

- `MUX_QUEUE_STREAM` và `MUX_QUEUE_GROUP` là điểm cần chốt cuối: nếu xem là implementation constant thì hardcode; nếu muốn operator quản lý hoàn toàn thì đưa ra env. Nên ưu tiên hardcode nếu gần như không đổi giữa môi trường.
- `.env.production` hiện chứa secret thật; nếu tiếp tục dùng như snapshot tham chiếu thì phải chấp nhận đây là file nhạy cảm nội bộ.

## Unresolved Questions

- Có muốn lộ `MUX_QUEUE_STREAM` và `MUX_QUEUE_GROUP` thành env thật không, hay chốt là implementation constant?
