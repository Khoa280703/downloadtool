# Phase 4: DownloadBtn Auto-Mux

## Context

- [frontend/src/components/DownloadBtn.svelte](/home/khoa2807/working-sources/downloadtool/frontend/src/components/DownloadBtn.svelte) -- download button
- [frontend/src/lib/api.ts](/home/khoa2807/working-sources/downloadtool/frontend/src/lib/api.ts) -- buildStreamUrl, buildMuxedStreamUrl (added in Phase 2)
- [crates/api/src/routes/stream.rs](/home/khoa2807/working-sources/downloadtool/crates/api/src/routes/stream.rs) -- muxed_stream_handler

## Overview

- **Priority:** High
- **Status:** pending
- **Effort:** 30min

Add `audioStream` prop to DownloadBtn. When provided, use `buildMuxedStreamUrl()` instead of `buildStreamUrl()`. User experience is seamless -- no audio warning, file always has audio+video.

## Key Insights

- Current DownloadBtn always calls `buildStreamUrl(stream.url, title, stream.format)` -> `/api/stream`
- Rust `/api/stream/muxed` expects query params: `video_url`, `audio_url`, `title` (optional)
- Muxed endpoint returns `Content-Type: video/mp4` with fMP4 muxed stream
- `buildMuxedStreamUrl()` added in Phase 2

## Related Code Files

### Files to modify:
- `frontend/src/components/DownloadBtn.svelte`

### Already modified in Phase 3:
- `frontend/src/routes/+page.svelte` (passes `audioStream` prop)

## Implementation Steps

### Step 1: Add `audioStream` prop

```typescript
interface Props {
  stream: Stream | null;
  audioStream?: Stream | null;  // NEW
  title: string;
  disabled?: boolean;
}

let { stream, audioStream = null, title, disabled = false }: Props = $props();
```

### Step 2: Update import

```typescript
import { buildStreamUrl, buildMuxedStreamUrl } from '$lib/api';
```

### Step 3: Update `handleDownload()` URL building

Replace:
```typescript
const downloadUrl = buildStreamUrl(stream.url, title, stream.format);
```

With:
```typescript
const downloadUrl = audioStream
  ? buildMuxedStreamUrl(stream.url, audioStream.url, title)
  : buildStreamUrl(stream.url, title, stream.format);
```

### Step 4: Update download filename extension

When muxing, output is always mp4:
```typescript
const ext = audioStream ? 'mp4' : stream.format;
anchor.download = `${title.replace(/[^a-z0-9]/gi, '_')}.${ext}`;
```

### Step 5: Update button label (optional enhancement)

Show "Download 1080p" regardless of muxing -- user doesn't need to know about audio/video separation.

No change needed; current `Download {stream?.quality || ''}` already works.

## Todo List

- [ ] Add `audioStream` prop to DownloadBtn
- [ ] Import `buildMuxedStreamUrl` from api.ts
- [ ] Conditional URL building: muxed if audioStream provided, else normal
- [ ] Fix file extension for muxed downloads (always mp4)
- [ ] Verify parent passes audioStream correctly (done in Phase 3)
- [ ] Test: video-only stream triggers muxed URL
- [ ] Test: muxed stream has both audio and video in downloaded file
- [ ] Test: normal streams (hasAudio=true) still use /api/stream

## Success Criteria

- Video-only stream (1080p+) downloads WITH audio via `/api/stream/muxed`
- Normal muxed streams (360p, 720p with audio) download via `/api/stream` as before
- Downloaded filename has correct `.mp4` extension
- No visible UX difference -- user clicks Download, gets file with audio+video

## Security Considerations

- Stream URLs are user-controlled; backend already validates/proxies them
- No new attack surface; just routing to existing muxed endpoint
