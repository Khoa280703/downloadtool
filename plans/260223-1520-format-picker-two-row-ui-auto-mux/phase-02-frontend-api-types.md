# Phase 2: Frontend API + Types

## Context

- [frontend/src/lib/types.ts](/home/khoa2807/working-sources/downloadtool/frontend/src/lib/types.ts) -- frontend Stream interface
- [frontend/src/lib/api.ts](/home/khoa2807/working-sources/downloadtool/frontend/src/lib/api.ts) -- extract() + buildStreamUrl()
- [crates/api/src/routes/stream.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/stream.rs) -- MuxedStreamParams

## Overview

- **Priority:** High (blocks Phases 3 & 4)
- **Status:** pending
- **Effort:** 30min

Fix `extract()` field mapping (currently uses yt-dlp format fields that don't exist), add new Stream fields, add `buildMuxedStreamUrl()`.

## Key Insights

- Current `api.ts` extract() maps `f.acodec`/`f.vcodec` -- these are yt-dlp fields. Backend returns InnerTube format with `f.codec`, `f.mime`, `f.format`, `f.quality`
- Backend response structure: `{ status, metadata: { title, formats: [...], thumbnail, duration } }`
- Each format from backend has: `url`, `quality`, `format`, `mime`, `bitrate`, `codec`, `width`, `height`, `hasAudio`, `isAudioOnly`, `codecLabel` (after Phase 1)
- Rust `MuxedStreamParams` expects: `video_url: String`, `audio_url: String`, `title: Option<String>`

## Related Code Files

### Files to modify:
- `frontend/src/lib/types.ts`
- `frontend/src/lib/api.ts`

## Implementation Steps

### Step 1: Update `frontend/src/lib/types.ts` Stream interface

Current:
```typescript
export interface Stream {
  url: string;
  quality: string;
  format: string;
  hasAudio: boolean;
  size?: number;
}
```

Change to:
```typescript
export interface Stream {
  url: string;
  quality: string;
  format: string;
  hasAudio: boolean;
  isAudioOnly: boolean;
  codecLabel?: string;
  codec?: string;
  width?: number;
  height?: number;
  bitrate?: number;
  size?: number;
}
```

### Step 2: Fix `extract()` mapping in `api.ts`

The current mapping uses `f.acodec`/`f.vcodec` which are yt-dlp fields. After Phase 1, backend formats already contain `hasAudio`, `isAudioOnly`, `codecLabel` directly.

Replace the stream mapping block (lines 34-47):
```typescript
const streams = (meta.formats || []).map((f: any) => ({
  url: f.url,
  quality: f.quality || 'Unknown',
  format: f.format || 'mp4',
  hasAudio: f.hasAudio ?? true,
  isAudioOnly: f.isAudioOnly ?? false,
  codecLabel: f.codecLabel,
  codec: f.codec,
  width: f.width,
  height: f.height,
  bitrate: f.bitrate,
  size: f.filesize,
}));
```

This directly maps backend fields 1:1 instead of re-deriving them incorrectly.

### Step 3: Add `buildMuxedStreamUrl()` function

Add after `buildStreamUrl()`:
```typescript
/**
 * Build muxed stream URL for video-only + audio streams
 * @param videoUrl - Video stream URL
 * @param audioUrl - Audio stream URL
 * @param title - Video title for filename
 * @returns Full muxed download URL
 */
export function buildMuxedStreamUrl(
  videoUrl: string,
  audioUrl: string,
  title: string
): string {
  const params = new URLSearchParams({
    video_url: videoUrl,
    audio_url: audioUrl,
    title,
  });
  return `${API_BASE}/api/stream/muxed?${params.toString()}`;
}
```

## Todo List

- [ ] Add `isAudioOnly`, `codecLabel`, `codec`, `width`, `height`, `bitrate` to frontend Stream type
- [ ] Fix extract() to map backend fields directly (remove yt-dlp field references)
- [ ] Add buildMuxedStreamUrl() function
- [ ] Verify TypeScript compiles: `cd frontend && npx svelte-check`

## Success Criteria

- Frontend Stream type matches backend extractor output
- extract() correctly maps all fields including `hasAudio`, `isAudioOnly`, `codecLabel`
- `buildMuxedStreamUrl()` generates URL matching Rust `MuxedStreamParams` query format
- No TypeScript errors
