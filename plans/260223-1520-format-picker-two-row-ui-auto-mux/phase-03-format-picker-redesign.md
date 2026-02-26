# Phase 3: FormatPicker Redesign

## Context

- [frontend/src/components/FormatPicker.svelte](/home/khoa2807/working-sources/downloadtool/frontend/src/components/FormatPicker.svelte) -- current flat list UI
- [frontend/src/routes/+page.svelte](/home/khoa2807/working-sources/downloadtool/frontend/src/routes/+page.svelte) -- parent component

## Overview

- **Priority:** High
- **Status:** pending
- **Effort:** 1h

Complete redesign from flat stream list to 2-row picker: resolution row + codec row. Also manages audio stream selection for auto-muxing.

## Key Insights

- Current FormatPicker: flat list of buttons, each showing quality + format
- YouTube returns multiple codecs per resolution (H.264, AV1, VP9 at 1080p)
- Audio-only streams must be hidden from UI but accessible for muxing
- `quality` field from extractor: `"1080p"`, `"1080p60"`, `"1080p (video only)"`, `"Audio 128kbps"`

## Target UI

```
Row 1: [360p] [720p] [1080p ✓] [1440p] [4K]      <- resolution buttons
Row 2: [H.264 MP4 ✓] [AV1 MP4] [VP9 WebM]        <- codec options for selected resolution
```

## Related Code Files

### Files to modify:
- `frontend/src/components/FormatPicker.svelte`
- `frontend/src/routes/+page.svelte` (adapt to new onSelect signature)

## Implementation Steps

### Step 1: New Props interface

```typescript
interface Props {
  streams: Stream[];
  platform: Platform;
  selectedStream: Stream | null;
  onSelect: (videoStream: Stream, audioStream: Stream | null) => void;
}
```

Changed: `onSelect` now passes both video stream and best audio stream (null if video has audio).

### Step 2: Derived data structures

```typescript
/** Filter: video streams only (not audio-only) */
const videoStreams = $derived(
  streams.filter(s => !s.isAudioOnly)
);

/** All audio-only streams, sorted by bitrate desc */
const audioStreams = $derived(
  streams
    .filter(s => s.isAudioOnly)
    .sort((a, b) => (b.bitrate || 0) - (a.bitrate || 0))
);

/** Best audio stream (highest bitrate) */
const bestAudio = $derived(audioStreams[0] || null);

/** Unique resolutions, ordered low->high */
const resolutions = $derived(() => {
  const resSet = new Map<string, number>();
  for (const s of videoStreams) {
    // Normalize: strip " (video only)", "60" fps suffix for grouping
    const base = s.quality.replace(/ \(video only\)/, '').replace(/p\d+/, 'p');
    const h = s.height || 0;
    if (!resSet.has(base) || h > (resSet.get(base) || 0)) {
      resSet.set(base, h);
    }
  }
  // Sort by height ascending
  return [...resSet.entries()]
    .sort((a, b) => a[1] - b[1])
    .map(([label]) => label);
});

/** Currently selected resolution */
let selectedRes = $state<string>('');

/** Streams for selected resolution */
const codecOptions = $derived(
  videoStreams.filter(s => {
    const base = s.quality.replace(/ \(video only\)/, '').replace(/p\d+/, 'p');
    return base === selectedRes;
  })
);
```

### Step 3: Auto-select logic

On mount / when streams change:
```typescript
$effect(() => {
  if (videoStreams.length === 0) return;
  const resList = resolutions;

  // Default: prefer 1080p, fallback to highest available
  const defaultRes = resList.includes('1080p') ? '1080p' : resList[resList.length - 1];
  selectedRes = defaultRes;
});

// When resolution changes, auto-select best codec
$effect(() => {
  if (codecOptions.length === 0) return;

  // Priority: H.264 > VP9 > AV1 > first available
  const priority = ['H.264', 'VP9', 'AV1'];
  let best = codecOptions[0];
  for (const p of priority) {
    const match = codecOptions.find(s => s.codecLabel === p);
    if (match) { best = match; break; }
  }

  const audio = best.hasAudio ? null : bestAudio;
  onSelect(best, audio);
});
```

### Step 4: Template markup

```svelte
<div class="format-picker">
  <h4>Select Quality</h4>

  <!-- Row 1: Resolution -->
  <div class="resolution-row" role="radiogroup" aria-label="Video resolution">
    {#each resolutions as res}
      <button
        class="res-btn"
        class:selected={selectedRes === res}
        onclick={() => selectedRes = res}
        role="radio"
        aria-checked={selectedRes === res}
      >
        {res}
      </button>
    {/each}
  </div>

  <!-- Row 2: Codec/format for selected resolution -->
  {#if codecOptions.length > 0}
    <div class="codec-row" role="radiogroup" aria-label="Video codec">
      {#each codecOptions as stream}
        <button
          class="codec-btn"
          class:selected={selectedStream?.url === stream.url}
          onclick={() => {
            const audio = stream.hasAudio ? null : bestAudio;
            onSelect(stream, audio);
          }}
          role="radio"
          aria-checked={selectedStream?.url === stream.url}
        >
          <span class="codec-label">{stream.codecLabel || 'Unknown'}</span>
          <span class="format-label">{stream.format.toUpperCase()}</span>
          {#if stream.hasAudio}
            <span class="audio-icon" title="Includes audio">&#9835;</span>
          {/if}
        </button>
      {/each}
    </div>
  {/if}

  <!-- YouTube toggle (keep existing) -->
  {#if platform === 'youtube'}
    <!-- ... existing watermark toggle ... -->
  {/if}
</div>
```

### Step 5: Styles

- `.resolution-row` and `.codec-row`: `display: flex; flex-wrap: wrap; gap: 0.5rem;`
- `.res-btn`, `.codec-btn`: similar to current `.stream-option` styling
- `.selected` state: blue border + subtle blue background
- Responsive: buttons wrap naturally on small screens

### Step 6: Update parent `+page.svelte`

Current `handleFormatSelect`:
```typescript
function handleFormatSelect(stream: ExtractResult['streams'][0]): void { ... }
```

Change to:
```typescript
let selectedAudioStream = $state<Stream | null>(null);

function handleFormatSelect(videoStream: Stream, audioStream: Stream | null): void {
  selectedAudioStream = audioStream;
  if (extractResult) {
    extractResult = { ...extractResult, streams: extractResult.streams };
    currentDownload.update(s => ({ ...s, selectedStream: videoStream }));
    trackFormatSelected(extractResult.platform, videoStream.quality, videoStream.format, videoStream.hasAudio);
  }
}
```

Update FormatPicker usage:
```svelte
<FormatPicker
  streams={extractResult.streams}
  platform={extractResult.platform}
  selectedStream={$currentDownload.selectedStream}
  onSelect={handleFormatSelect}
/>
```

Update DownloadBtn usage:
```svelte
<DownloadBtn
  stream={$currentDownload.selectedStream}
  audioStream={selectedAudioStream}
  title={extractResult.title}
/>
```

## Todo List

- [ ] Refactor FormatPicker Props to include new onSelect signature
- [ ] Add derived data: videoStreams, audioStreams, bestAudio, resolutions, codecOptions
- [ ] Add auto-select logic (1080p default, H.264 priority)
- [ ] Build 2-row template (resolution row + codec row)
- [ ] Style resolution and codec buttons
- [ ] Keep YouTube watermark toggle
- [ ] Update +page.svelte handleFormatSelect to accept (videoStream, audioStream)
- [ ] Add selectedAudioStream state in +page.svelte
- [ ] Pass audioStream prop to DownloadBtn
- [ ] Test: streams with mixed codecs display correctly
- [ ] Test: resolution switch updates codec row
- [ ] Test: auto-select defaults to 1080p + H.264

## Success Criteria

- No duplicate entries visible (grouped by resolution, each codec shows once)
- Resolution row shows unique resolutions low-to-high
- Codec row updates when resolution changes
- Auto-selects 1080p + H.264 by default
- Audio-only streams hidden from UI
- onSelect passes both video and audio streams to parent

## Risk Assessment

- **Svelte 5 reactivity** -- using `$derived` and `$effect`; ensure no circular dependencies between selectedRes and codecOptions
- **Empty codec row** -- if a resolution has only 1 codec, still show the row (single button, pre-selected)
- **FPS variants** -- "1080p60" vs "1080p"; grouping normalizes by stripping fps suffix
