# Phase 1: Extractor Stream Fields

## Context

- [extractors/types.ts](/home/khoa2807/working-sources/downloadtool/extractors/types.ts) -- Stream interface
- [extractors/youtube-innertube.ts](/home/khoa2807/working-sources/downloadtool/extractors/youtube-innertube.ts) -- parseFormat()

## Overview

- **Priority:** High (blocks all other phases)
- **Status:** pending
- **Effort:** 30min

Add `hasAudio`, `isAudioOnly`, `codecLabel` fields to extractor Stream type and populate them in InnerTube's `parseFormat()`.

## Key Insights

- `parseFormat()` already computes `isVideoOnly` and `isAudioOnly` locally (lines 143-144) but never exposes them
- `codecs` string already parsed from mimeType (line 137), e.g. `"avc1.64001F, mp4a.40.2"`
- The Stream interface currently has: `url, quality, format, mime, bitrate, codec, width, height`

## Related Code Files

### Files to modify:
- `extractors/types.ts` -- add 3 fields to Stream interface
- `extractors/youtube-innertube.ts` -- populate new fields in parseFormat()

### Files to rebuild:
- `extractors/dist/youtube.js` -- esbuild bundle output

## Implementation Steps

### Step 1: Update `extractors/types.ts` Stream interface

Add after `height?: number;`:
```typescript
/** Whether this stream contains audio */
hasAudio: boolean;
/** Whether this is an audio-only stream */
isAudioOnly: boolean;
/** Human-readable codec label (e.g., "H.264", "VP9", "AV1", "AAC", "Opus") */
codecLabel?: string;
```

### Step 2: Add codec label helper in `youtube-innertube.ts`

Add before `parseFormat()`:
```typescript
/** Map codec string prefix to human-readable label */
function getCodecLabel(codecs: string, isAudioOnly: boolean): string {
  if (isAudioOnly) {
    if (codecs.includes('mp4a')) return 'AAC';
    if (codecs.includes('opus')) return 'Opus';
    return 'Audio';
  }
  if (codecs.includes('avc1')) return 'H.264';
  if (codecs.includes('vp09') || codecs.includes('vp9')) return 'VP9';
  if (codecs.includes('av01')) return 'AV1';
  return codecs.split('.')[0] || 'Unknown';
}
```

### Step 3: Update `parseFormat()` return statement

Current return (line 160):
```typescript
return { url, quality, format: ext, mime: mimeType, bitrate, codec: codecs, width, height };
```

Change to:
```typescript
const codecLabel = getCodecLabel(codecs, isAudioOnly);
const hasAudio = !isVideoOnly || isAudioOnly;

return {
  url, quality, format: ext, mime: mimeType, bitrate,
  codec: codecs, width, height,
  hasAudio, isAudioOnly, codecLabel,
};
```

Note: `hasAudio` logic:
- Audio-only stream -> `hasAudio: true`
- Muxed video (has both video+audio codecs) -> `hasAudio: true`
- Video-only (adaptive, no audio codec) -> `hasAudio: false`

### Step 4: Rebuild esbuild bundle

```bash
npx esbuild extractors/youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=extractors/dist/youtube.js
```

## Todo List

- [ ] Add `hasAudio`, `isAudioOnly`, `codecLabel` to Stream interface in types.ts
- [ ] Add `getCodecLabel()` helper function
- [ ] Update `parseFormat()` return to include new fields
- [ ] Rebuild esbuild bundle
- [ ] Verify no TypeScript compilation errors

## Success Criteria

- Stream objects returned from extractor include `hasAudio`, `isAudioOnly`, `codecLabel`
- Video-only streams (1080p+) have `hasAudio: false`, `isAudioOnly: false`
- Audio streams have `hasAudio: true`, `isAudioOnly: true`
- `codecLabel` maps correctly: avc1->H.264, vp09/vp9->VP9, av01->AV1, mp4a->AAC, opus->Opus
- esbuild bundle compiles without errors
