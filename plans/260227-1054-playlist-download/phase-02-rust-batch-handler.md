---
title: "Phase 2 - Rust Batch Handler"
status: pending
---

# Phase 2: Rust Batch Handler (SSE)

## Overview
Implement real `create_batch_stream` trong `crates/api/src/routes/batch.rs` thay thế stub.

## File
- **Modify:** `crates/api/src/routes/batch.rs`

## Architecture

```
GET /api/batch?url=<playlist_url>
  → Rust spawns std::thread → calls extractor["youtube"].extractPlaylist(url)
  → TS returns BatchVideoInfo[] JSON array
  → Rust deserializes, iterates, sends SSE events:
      { type: "link", videoId, title, thumbnail, index, total }
      { type: "progress", current, total }  (every 10 items)
      { type: "done", total }
  → SSE keepalive every 15s (KeepAlive::new().interval(Duration::from_secs(15)))
```

## SSE Event Types
```json
// link event
{ "type": "link", "videoId": "abc123", "title": "...", "thumbnail": "...", "index": 1, "total": 42 }

// progress event
{ "type": "progress", "current": 10, "total": 42 }

// done event
{ "type": "done", "total": 42 }

// error event
{ "type": "error", "message": "..." }
```

## Key Design
- `videoId` sử dụng `#[serde(rename = "videoId")]` — camelCase trong SSE JSON
- Extractor trả về `BatchVideoInfo[] JSON array` (không phải stream/generator)
- `std::thread` chạy extractor đồng bộ, gửi qua `mpsc::channel` vào SSE stream
- Nếu client disconnect → `async_stream` drop SSE loop ngay; extractor thread chạy đến hết (kết quả bỏ đi — acceptable vì extract xong trong vài giây)

## Success Criteria
- SSE stream gửi đúng camelCase field names
- `cargo check --workspace` clean
- Frontend nhận được videoId, title, thumbnail đúng format
