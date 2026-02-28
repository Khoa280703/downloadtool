---
title: "Phase 1 - TypeScript Playlist Extractor"
status: pending
---

# Phase 1: TypeScript Playlist Extractor

## Overview
Tạo `extractors/youtube-playlist.ts` — dùng InnerTube browse API (WEB client) để lấy toàn bộ video trong playlist YouTube.

## File
- **Create:** `extractors/youtube-playlist.ts` (~120 lines)

## Architecture

```
parsePlaylistId(url) → listId
  → POST InnerTube /browse (WEB client, browseId: VL{listId})
  → parse videos from contents[].playlistVideoRenderer
  → follow continuationData → continuationCommand.token (max 50 pages)
  → dedup via seenVideoIds Set + seenContinuations Set
  → return BatchVideoInfo[]
```

## Key Design
- **Client:** WEB (not IOS/ANDROID) — playlist browse requires WEB client
- **Return type:** `BatchVideoInfo[]` (not AsyncGenerator) — Rust deserializes full array
- **Dedup:** `seenVideoIds` Set, `seenContinuations` Set
- **Max pages:** 50 (safety limit)
- **Export:** `extractPlaylist(url)` + re-export from `youtube.ts`

## Types
```ts
export interface BatchVideoInfo {
  videoId: string;
  title: string;
  thumbnail?: string;
  index: number; // 1-based position in playlist
}
```

## Implementation Steps
1. `parsePlaylistId(url)` — URL API + regex fallback for `list=` param
2. `fetchPage(listId, token?)` — POST to InnerTube browse endpoint
3. `parseVideos(contents)` — extract from `playlistVideoRenderer`
4. `extractPlaylist(url)` — loop with continuation, return `BatchVideoInfo[]`

## Success Criteria
- Returns all videos in playlist (handles pagination)
- No duplicate entries
- `thumbnail` is highest-res thumbnail URL
- Exported correctly so Rust can call via Deno extractor
