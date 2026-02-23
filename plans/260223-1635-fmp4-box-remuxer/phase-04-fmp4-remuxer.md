# Phase 4: fMP4 Remuxer (replaces fmp4_muxer.rs)

## Context
- [box_parser.rs](./phase-02-box-parser.md) -- Phase 2 dependency
- [moov_merger.rs](./phase-03-moov-merger.md) -- Phase 3 dependency
- [fmp4_muxer.rs](../../crates/muxer/src/fmp4_muxer.rs) -- to be replaced (1293 lines)
- [stream.rs](../../crates/api/src/routes/stream.rs) -- consumer, needs update

## Overview
- **Priority:** P1
- **Status:** pending
- **Description:** Stream-based fMP4 remuxer that copies box data from YouTube CMAF streams, merges moov, patches track IDs in fragments

## Key Insights
- YouTube adaptive fMP4 structure: `[ftyp][moov][moof+mdat][moof+mdat]...`
- Each stream has track_id=1 in all traf/tfhd boxes
- Output: `[ftyp][merged_moov][video_fragments...][audio_fragments...]`
- For fragments: need to patch tfhd.track_id (audio 1->2) and renumber mfhd.sequence_number
- Phase 1 approach: non-interleaved (all video then all audio) -- correct for file download
- mfhd layout: `[4B size][4B "mfhd"][4B ver+flags][4B sequence_number]` => seq at offset 12
- tfhd layout: `[4B size][4B "tfhd"][4B ver+flags][4B track_id]...` => track_id at offset 12
- Need to buffer both full streams to memory first (to extract moov from init segment), then stream fragments

## Requirements
### Functional
- `remux_streams(video, audio) -> impl Stream<Item=Result<Bytes, MuxerError>>`
- Buffer video stream until moov box fully received
- Buffer audio stream until moov box fully received (in parallel)
- Emit: ftyp (from video) + merged_moov
- Stream video fragments (moof+mdat pairs), renumbering mfhd.sequence_number starting from 1
- Stream audio fragments, patching tfhd.track_id=2 + renumbering mfhd.sequence_number continuing from video
- Handle errors from source streams gracefully

### Non-functional
- No codec parameter needed (copy-based)
- Under 250 lines
- Replace `fmp4_muxer.rs` export in lib.rs

## Architecture
```
remux_streams(video_stream, audio_stream)
  │
  ├─ Phase A: Buffer init segments (parallel)
  │   ├─ Collect video bytes until ftyp+moov found
  │   └─ Collect audio bytes until moov found
  │
  ├─ Phase B: Emit init segment
  │   ├─ video ftyp → output
  │   └─ merge_moov(video_moov, audio_moov) → output
  │
  ├─ Phase C: Stream video fragments
  │   ├─ Remaining buffered video data → find moof+mdat pairs
  │   ├─ Patch mfhd.sequence_number (1, 2, 3...)
  │   ├─ Continue reading video stream for more fragments
  │   └─ Yield each fragment
  │
  └─ Phase D: Stream audio fragments
      ├─ Remaining buffered audio data → find moof+mdat pairs
      ├─ Patch tfhd.track_id=2 + mfhd.sequence_number (continuing)
      ├─ Continue reading audio stream for more fragments
      └─ Yield each fragment
```

## Related Code Files
- **Create:** `crates/muxer/src/fmp4_remuxer.rs`
- **Delete:** `crates/muxer/src/fmp4_muxer.rs` (after migration)
- **Modify:** `crates/muxer/src/lib.rs` (swap exports)
- **Modify:** `crates/api/src/routes/stream.rs` (use `remux_streams`, remove codec params)
- **Depends on:** `box_parser.rs`, `moov_merger.rs`

## Implementation Steps

### Step 1: Create `fmp4_remuxer.rs` with public API
```rust
use crate::box_parser::{find_box, read_box_header, write_u32_be, iter_boxes};
use crate::moov_merger::merge_moov;
use crate::MuxerError;
use bytes::{Bytes, BytesMut};
use futures::{Stream, StreamExt};
use std::pin::Pin;
use tracing::{debug, error, info};

pub type MuxedStream = Pin<Box<dyn Stream<Item = Result<Bytes, MuxerError>> + Send>>;

/// Remux separate video and audio fMP4 streams into a single muxed fMP4.
///
/// Both input streams must be valid fMP4/CMAF (ftyp+moov+fragments).
/// Output: merged init segment + all video fragments + all audio fragments.
pub fn remux_streams<V, A, VE, AE>(video: V, audio: A) -> MuxedStream
where
    VE: std::error::Error + Send + Sync + 'static,
    AE: std::error::Error + Send + Sync + 'static,
    V: Stream<Item = Result<Bytes, VE>> + Send + 'static,
    A: Stream<Item = Result<Bytes, AE>> + Send + 'static,
{
    Box::pin(async_stream::try_stream! {
        // ... implementation below
    })
}
```

### Step 2: Buffer both streams to get init segments
```rust
// Inside try_stream!:

// Collect both streams fully (they're already being fetched via chunked bypass)
let (video_data, audio_data) = tokio::join!(
    collect_stream(video),
    collect_stream(audio)
);
let video_data = video_data?;
let audio_data = audio_data?;

info!("Collected video: {} bytes, audio: {} bytes", video_data.len(), audio_data.len());
```

Helper:
```rust
async fn collect_stream<S, E>(stream: S) -> Result<Vec<u8>, MuxerError>
where
    E: std::error::Error + Send + Sync + 'static,
    S: Stream<Item = Result<Bytes, E>> + Send,
{
    futures::pin_mut!(stream);
    let mut buf = Vec::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| MuxerError::StreamFetchError(e.to_string()))?;
        buf.extend_from_slice(&chunk);
    }
    Ok(buf)
}
```

**Note:** This buffers full streams in memory. For a 1080p 10-min video (~150MB video + ~20MB audio), that's ~170MB RAM. Acceptable for download tool. Future optimization: stream fragments after moov is found.

### Step 3: Extract and emit init segment
```rust
// Find ftyp from video
let ftyp = find_box(&video_data, b"ftyp")
    .ok_or_else(|| MuxerError::InvalidInput("No ftyp in video stream".into()))?;

// Find moov boxes
let video_moov = find_box(&video_data, b"moov")
    .ok_or_else(|| MuxerError::InvalidInput("No moov in video stream".into()))?;
let audio_moov = find_box(&audio_data, b"moov")
    .ok_or_else(|| MuxerError::InvalidInput("No moov in audio stream".into()))?;

// Merge moov
let merged_moov = merge_moov(video_moov, audio_moov)?;

// Yield init segment
yield Bytes::copy_from_slice(ftyp);
yield Bytes::from(merged_moov);
```

### Step 4: Stream video fragments with patched sequence numbers
```rust
let mut seq = 1u32;

// Find where fragments start in video data (after ftyp+moov)
let video_frags = skip_init_boxes(&video_data);
for (offset, moof, mdat) in iter_fragment_pairs(video_frags) {
    let mut patched_moof = moof.to_vec();
    patch_mfhd_sequence(&mut patched_moof, seq)?;
    seq += 1;

    yield Bytes::from(patched_moof);
    yield Bytes::copy_from_slice(mdat);
}
```

### Step 5: Stream audio fragments with patched track_id and sequence numbers
```rust
let audio_frags = skip_init_boxes(&audio_data);
for (offset, moof, mdat) in iter_fragment_pairs(audio_frags) {
    let mut patched_moof = moof.to_vec();
    patch_mfhd_sequence(&mut patched_moof, seq)?;
    patch_tfhd_track_id(&mut patched_moof, 2)?;
    seq += 1;

    yield Bytes::from(patched_moof);
    yield Bytes::copy_from_slice(mdat);
}

info!("Remux complete: {} total fragments", seq - 1);
```

### Step 6: Helper functions
```rust
/// Skip ftyp and moov boxes, return remaining data (fragments)
fn skip_init_boxes(data: &[u8]) -> &[u8] {
    let mut offset = 0;
    for (box_offset, header) in iter_boxes(data) {
        if &header.box_type == b"moof" {
            return &data[box_offset..];
        }
        offset = box_offset + header.total_size as usize;
    }
    &data[offset..]
}

/// Iterate moof+mdat pairs from fragment data
fn iter_fragment_pairs(data: &[u8]) -> Vec<(usize, &[u8], &[u8])> {
    let mut result = Vec::new();
    let mut iter = iter_boxes(data).peekable();

    while let Some((moof_offset, moof_header)) = iter.next() {
        if &moof_header.box_type != b"moof" { continue; }
        let moof_end = moof_offset + moof_header.total_size as usize;
        let moof_slice = &data[moof_offset..moof_end.min(data.len())];

        // Next box should be mdat
        if let Some((mdat_offset, mdat_header)) = iter.next() {
            if &mdat_header.box_type == b"mdat" {
                let mdat_end = mdat_offset + mdat_header.total_size as usize;
                let mdat_slice = &data[mdat_offset..mdat_end.min(data.len())];
                result.push((moof_offset, moof_slice, mdat_slice));
            }
        }
    }
    result
}

/// Patch mfhd.sequence_number inside a moof box
fn patch_mfhd_sequence(moof: &mut [u8], seq: u32) -> Result<(), MuxerError> {
    let header = read_box_header(moof)
        .ok_or_else(|| MuxerError::InvalidInput("Invalid moof".into()))?;
    let content = header.header_size as usize;

    // Find mfhd in moof content
    // mfhd: [4B size][4B "mfhd"][4B ver+flags][4B sequence_number]
    // sequence_number at mfhd_start + 12
    for (box_offset, box_header) in iter_boxes(&moof[content..]) {
        if &box_header.box_type == b"mfhd" {
            let abs_offset = content + box_offset + 12;
            write_u32_be(moof, abs_offset, seq);
            return Ok(());
        }
    }
    Err(MuxerError::InvalidInput("No mfhd in moof".into()))
}

/// Patch tfhd.track_id inside a moof box (within traf)
fn patch_tfhd_track_id(moof: &mut [u8], track_id: u32) -> Result<(), MuxerError> {
    let header = read_box_header(moof)
        .ok_or_else(|| MuxerError::InvalidInput("Invalid moof".into()))?;
    let moof_content_start = header.header_size as usize;

    // Find traf in moof
    for (traf_offset, traf_header) in iter_boxes(&moof[moof_content_start..]) {
        if &traf_header.box_type != b"traf" { continue; }

        let traf_abs_start = moof_content_start + traf_offset;
        let traf_hdr = read_box_header(&moof[traf_abs_start..])
            .ok_or_else(|| MuxerError::InvalidInput("Invalid traf".into()))?;
        let traf_content_start = traf_abs_start + traf_hdr.header_size as usize;

        // Find tfhd in traf
        // tfhd: [4B size][4B "tfhd"][4B ver+flags][4B track_id]
        for (tfhd_offset, tfhd_header) in iter_boxes(&moof[traf_content_start..]) {
            if &tfhd_header.box_type == b"tfhd" {
                let abs_offset = traf_content_start + tfhd_offset + 12;
                write_u32_be(moof, abs_offset, track_id);
                return Ok(());
            }
        }
    }
    Err(MuxerError::InvalidInput("No tfhd in moof".into()))
}
```

### Step 7: Add `async-stream` dependency to Cargo.toml
```toml
# In crates/muxer/Cargo.toml
async-stream = "0.3"
```

### Step 8: Update lib.rs
```rust
// Remove old exports, add new
pub mod fmp4_remuxer;
pub mod box_parser;
pub mod moov_merger;
// pub mod fmp4_muxer;  // Remove or keep for backward compat
pub mod mux_router;
pub mod stream_fetcher;
pub mod codec;

pub use fmp4_remuxer::{remux_streams, MuxedStream};
// Remove: pub use fmp4_muxer::{mux_streams, MuxedStream};
```

### Step 9: Update stream.rs in api crate
```rust
// Change imports:
// Before:
use muxer::codec::Codec;
use muxer::{mux_streams, MuxerError};

// After:
use muxer::{remux_streams, MuxerError};

// In muxed_stream_handler, remove codec parsing:
// Remove video_codec and audio_codec params handling
// Change:
let muxed_stream = remux_streams(video_stream, audio_stream);
```

Update `MuxedStreamParams` -- `video_codec` and `audio_codec` become unused but keep for backward compat (just ignore them).

### Step 10: Delete fmp4_muxer.rs
After everything compiles and tests pass, remove `crates/muxer/src/fmp4_muxer.rs`.

### Step 11: Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use futures::stream;

    fn build_test_fmp4(track_id: u32) -> Vec<u8> {
        let mut data = Vec::new();

        // ftyp
        let ftyp_size = 20u32;
        data.extend_from_slice(&ftyp_size.to_be_bytes());
        data.extend_from_slice(b"ftyp");
        data.extend_from_slice(b"isom");
        data.extend_from_slice(&[0, 0, 0, 0]); // minor version
        data.extend_from_slice(b"isom");

        // Minimal moov with mvhd + trak{tkhd}
        // (use build_simple_moov from phase 3 tests)
        // ... construct moov here ...

        // One moof+mdat fragment
        // moof: mfhd + traf{tfhd}
        let mut moof_content = Vec::new();
        // mfhd
        moof_content.extend_from_slice(&16u32.to_be_bytes());
        moof_content.extend_from_slice(b"mfhd");
        moof_content.extend_from_slice(&[0, 0, 0, 0]); // ver+flags
        moof_content.extend_from_slice(&1u32.to_be_bytes()); // seq=1

        // traf with tfhd
        let mut traf_content = Vec::new();
        traf_content.extend_from_slice(&16u32.to_be_bytes()); // tfhd size
        traf_content.extend_from_slice(b"tfhd");
        traf_content.extend_from_slice(&[0, 0x02, 0, 0]); // ver+flags (default-base-is-moof)
        traf_content.extend_from_slice(&track_id.to_be_bytes());

        let traf_size = 8 + traf_content.len();
        moof_content.extend_from_slice(&(traf_size as u32).to_be_bytes());
        moof_content.extend_from_slice(b"traf");
        moof_content.extend_from_slice(&traf_content);

        let moof_size = 8 + moof_content.len();
        data.extend_from_slice(&(moof_size as u32).to_be_bytes());
        data.extend_from_slice(b"moof");
        data.extend_from_slice(&moof_content);

        // mdat
        let mdat_payload = vec![0xDE, 0xAD, 0xBE, 0xEF];
        let mdat_size = 8 + mdat_payload.len();
        data.extend_from_slice(&(mdat_size as u32).to_be_bytes());
        data.extend_from_slice(b"mdat");
        data.extend_from_slice(&mdat_payload);

        data
    }

    #[tokio::test]
    async fn test_remux_streams_basic() {
        let video_data = build_test_fmp4(1);
        let audio_data = build_test_fmp4(1);

        let video_stream = stream::iter(vec![Ok::<_, std::io::Error>(Bytes::from(video_data))]);
        let audio_stream = stream::iter(vec![Ok::<_, std::io::Error>(Bytes::from(audio_data))]);

        let mut muxed = remux_streams(video_stream, audio_stream);
        let mut output = Vec::new();

        while let Some(result) = muxed.next().await {
            output.extend_from_slice(&result.unwrap());
        }

        // Verify ftyp present
        assert!(output.windows(4).any(|w| w == b"ftyp"));
        // Verify moov present
        assert!(output.windows(4).any(|w| w == b"moov"));
        // Verify moof present (fragments)
        assert!(output.windows(4).any(|w| w == b"moof"));
    }
}
```

## Todo
- [ ] Add `async-stream = "0.3"` to Cargo.toml
- [ ] Create `fmp4_remuxer.rs`
- [ ] Implement `remux_streams`
- [ ] Implement `collect_stream`
- [ ] Implement `skip_init_boxes`
- [ ] Implement `iter_fragment_pairs`
- [ ] Implement `patch_mfhd_sequence`
- [ ] Implement `patch_tfhd_track_id`
- [ ] Update `lib.rs` exports
- [ ] Update `stream.rs` to use `remux_streams`
- [ ] Delete `fmp4_muxer.rs`
- [ ] Write unit tests
- [ ] Integration test with constructed fMP4 data
- [ ] Verify compile

## Success Criteria
- Remuxed output starts with valid ftyp + moov (2 traks)
- Video fragments have track_id=1
- Audio fragments have track_id=2
- mfhd sequence numbers are sequential across all fragments
- Output plays correctly in VLC/Chrome
- fmp4_muxer.rs is deleted
- All existing tests updated/passing

## Risk
- Full stream buffering uses ~170MB for 10-min 1080p video -- acceptable for download tool, not for live streaming
- `async-stream` adds a new dependency (small, well-maintained crate)
- `iter_fragment_pairs` assumes moof is always followed by mdat -- true for YouTube CMAF

## Security
- Validate box sizes to prevent integer overflow
- Bounds-check all patch offsets
- Don't trust input sizes blindly (cap at reasonable maximum)

## Next Steps (future optimization)
- Interleave video+audio fragments by tfdt timestamp for better seeking
- Stream fragments as they arrive instead of buffering full stream
- Support WebM remuxing for VP9+Opus
