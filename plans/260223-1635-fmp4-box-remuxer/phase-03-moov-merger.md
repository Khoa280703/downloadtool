# Phase 3: Moov Merger

## Context
- [box_parser.rs](./phase-02-box-parser.md) -- Phase 2 dependency
- YouTube fMP4 streams: each has `[ftyp][moov: {mvhd, trak(id=1), mvex}][moof+mdat]...`
- We merge two single-track moov boxes into one dual-track moov

## Overview
- **Priority:** P1
- **Status:** pending
- **Description:** Merge video moov (trak_id=1) + audio moov (trak_id=1 -> patched to 2) into combined moov

## Key Insights
- YouTube adaptive streams have exactly 1 trak per moov, track_id=1
- moov structure: `[mvhd][trak][mvex{trex}]`
- We need: `[mvhd_from_video][video_trak(id=1)][audio_trak(id=2)][mvex{trex_v(1), trex_a(2)}]`
- tkhd.track_id location: version 0 = byte offset 12 from tkhd content start; version 1 = byte offset 20
- tkhd content starts after 8-byte header (4B size + 4B "tkhd")
- tkhd layout v0: `[1B version][3B flags][4B creation][4B modification][4B track_id]...`
- tkhd layout v1: `[1B version][3B flags][8B creation][8B modification][4B track_id]...`
- trex.track_id at offset 12 from box start (after 4B size + 4B type + 1B ver + 3B flags)
- mvhd.next_track_id: version 0 at byte 96+8=offset 104 from mvhd box start; but simpler to just count from content: last 4 bytes of mvhd content

## Requirements
### Functional
- `merge_moov(video_moov: &[u8], audio_moov: &[u8]) -> Result<Vec<u8>, MuxerError>`
- Extract mvhd from video moov, patch next_track_id to 3
- Extract video trak (keep track_id=1)
- Extract audio trak, patch track_id 1->2 in tkhd
- Build mvex with two trex boxes (track 1 and 2)
- Write correct moov box size

### Non-functional
- Under 200 lines
- Use box_parser functions for all box operations

## Architecture
```
merge_moov(video_moov, audio_moov) -> Vec<u8>
  ├─ Parse video_moov: find mvhd, trak
  ├─ Parse audio_moov: find trak
  ├─ Clone audio trak → patch tkhd.track_id to 2
  ├─ Clone mvhd → patch next_track_id to 3
  ├─ Build mvex: [trex(track=1)][trex(track=2)]
  └─ Assemble: [moov_header][mvhd][video_trak][audio_trak][mvex]
```

## Related Code Files
- **Create:** `crates/muxer/src/moov_merger.rs`
- **Depends on:** `crates/muxer/src/box_parser.rs` (Phase 2)
- **Modify:** `crates/muxer/src/lib.rs` (add `pub mod moov_merger;`)

## Implementation Steps

### Step 1: Create `moov_merger.rs` with merge function
```rust
use crate::box_parser::{find_box, find_box_path, read_box_header, read_u32_be, write_u32_be, iter_boxes};
use crate::MuxerError;

/// Merge video and audio moov boxes into a single dual-track moov.
///
/// Video track gets track_id=1, audio track gets track_id=2.
/// Both input moov boxes are expected to have a single trak with track_id=1.
pub fn merge_moov(video_moov: &[u8], audio_moov: &[u8]) -> Result<Vec<u8>, MuxerError> {
    // 1. Extract components from video moov
    let v_header = read_box_header(video_moov)
        .ok_or_else(|| MuxerError::InvalidInput("Invalid video moov".into()))?;
    let v_content = &video_moov[v_header.header_size as usize..];

    let mvhd = find_box(v_content, b"mvhd")
        .ok_or_else(|| MuxerError::InvalidInput("No mvhd in video moov".into()))?;
    let video_trak = find_box(v_content, b"trak")
        .ok_or_else(|| MuxerError::InvalidInput("No trak in video moov".into()))?;

    // 2. Extract audio trak
    let a_header = read_box_header(audio_moov)
        .ok_or_else(|| MuxerError::InvalidInput("Invalid audio moov".into()))?;
    let a_content = &audio_moov[a_header.header_size as usize..];

    let audio_trak_src = find_box(a_content, b"trak")
        .ok_or_else(|| MuxerError::InvalidInput("No trak in audio moov".into()))?;

    // 3. Clone and patch mvhd: set next_track_id = 3
    let mut mvhd_patched = mvhd.to_vec();
    patch_mvhd_next_track_id(&mut mvhd_patched, 3)?;

    // 4. Clone and patch audio trak: track_id 1 -> 2
    let mut audio_trak = audio_trak_src.to_vec();
    patch_trak_track_id(&mut audio_trak, 2)?;

    // 5. Build mvex
    let mvex = build_mvex(1, 2);

    // 6. Assemble moov
    let moov_content_size = mvhd_patched.len() + video_trak.len() + audio_trak.len() + mvex.len();
    let moov_size = 8 + moov_content_size;

    let mut result = Vec::with_capacity(moov_size);
    result.extend_from_slice(&(moov_size as u32).to_be_bytes());
    result.extend_from_slice(b"moov");
    result.extend_from_slice(&mvhd_patched);
    result.extend_from_slice(video_trak);
    result.extend_from_slice(&audio_trak);
    result.extend_from_slice(&mvex);

    Ok(result)
}
```

### Step 2: Implement `patch_mvhd_next_track_id`
```rust
/// Patch mvhd.next_track_id
/// mvhd v0: next_track_id at content offset 96 (after 8B header = box offset 104)
/// mvhd v1: next_track_id at content offset 108
fn patch_mvhd_next_track_id(mvhd: &mut [u8], next_track_id: u32) -> Result<(), MuxerError> {
    let header = read_box_header(mvhd)
        .ok_or_else(|| MuxerError::InvalidInput("Invalid mvhd".into()))?;
    let content_start = header.header_size as usize;

    if mvhd.len() < content_start + 1 {
        return Err(MuxerError::InvalidInput("mvhd too short".into()));
    }

    let version = mvhd[content_start]; // first byte of fullbox is version
    let offset = match version {
        0 => content_start + 4 + 96, // fullbox(4) + fields up to next_track_id
        1 => content_start + 4 + 108,
        _ => return Err(MuxerError::InvalidInput(format!("Unknown mvhd version {}", version))),
    };

    // Actually: mvhd v0 total content = 100 bytes after fullbox header
    // Layout v0: ver(1)+flags(3) + creation(4)+mod(4)+timescale(4)+duration(4) + rate(4)+vol(2)+reserved(10)+matrix(36)+predefined(24)+next_track_id(4)
    // = 4 + 4+4+4+4 + 4+2+10+36+24+4 = 4+16+80 = 100
    // next_track_id offset from content start: 4 + 16 + 76 = 96... let me recount
    // ver+flags=4, creation=4, mod=4, timescale=4, duration=4 => 20
    // rate=4, volume=2, reserved=2, reserved=8, matrix=36, predefined=24 => 76
    // total before next_track_id = 20+76 = 96
    // So: content_start + 96

    // v1: ver+flags=4, creation=8, mod=8, timescale=4, duration=8 => 32
    // + 76 = 108
    let nti_offset = content_start + match version { 0 => 96, _ => 108 };

    if mvhd.len() < nti_offset + 4 {
        return Err(MuxerError::InvalidInput("mvhd too short for next_track_id".into()));
    }

    write_u32_be(mvhd, nti_offset, next_track_id);
    Ok(())
}
```

### Step 3: Implement `patch_trak_track_id`
```rust
/// Patch track_id in trak's tkhd box
fn patch_trak_track_id(trak: &mut [u8], new_track_id: u32) -> Result<(), MuxerError> {
    // Find tkhd within trak content
    let trak_header = read_box_header(trak)
        .ok_or_else(|| MuxerError::InvalidInput("Invalid trak".into()))?;
    let content_start = trak_header.header_size as usize;

    // Find tkhd box offset within trak content
    let trak_content = &trak[content_start..];
    let tkhd_offset_in_content = find_box_offset(trak_content, b"tkhd")
        .ok_or_else(|| MuxerError::InvalidInput("No tkhd in trak".into()))?;

    let tkhd_abs_start = content_start + tkhd_offset_in_content;
    let tkhd_header = read_box_header(&trak[tkhd_abs_start..])
        .ok_or_else(|| MuxerError::InvalidInput("Invalid tkhd".into()))?;

    let tkhd_content_start = tkhd_abs_start + tkhd_header.header_size as usize;
    let version = trak[tkhd_content_start]; // fullbox version

    // tkhd v0: ver(1)+flags(3)+creation(4)+mod(4)+track_id(4) → track_id at +12
    // tkhd v1: ver(1)+flags(3)+creation(8)+mod(8)+track_id(4) → track_id at +20
    let tid_offset = tkhd_content_start + match version { 0 => 12, _ => 20 };

    if trak.len() < tid_offset + 4 {
        return Err(MuxerError::InvalidInput("tkhd too short".into()));
    }

    write_u32_be(trak, tid_offset, new_track_id);
    Ok(())
}
```

### Step 4: Add `find_box_offset` helper to box_parser (or local)
```rust
/// Find offset of first box of given type (not the slice, just the offset)
fn find_box_offset(data: &[u8], box_type: &[u8; 4]) -> Option<usize> {
    for (offset, header) in iter_boxes(data) {
        if &header.box_type == *box_type {
            return Some(offset);
        }
    }
    None
}
```

### Step 5: Build mvex
```rust
fn build_mvex(video_track_id: u32, audio_track_id: u32) -> Vec<u8> {
    let trex_size: u32 = 32; // 8 header + 4 ver/flags + 5*4 fields
    let mvex_size: u32 = 8 + trex_size * 2;

    let mut mvex = Vec::with_capacity(mvex_size as usize);
    mvex.extend_from_slice(&mvex_size.to_be_bytes());
    mvex.extend_from_slice(b"mvex");

    // trex for video
    write_trex(&mut mvex, video_track_id);
    // trex for audio
    write_trex(&mut mvex, audio_track_id);

    mvex
}

fn write_trex(buf: &mut Vec<u8>, track_id: u32) {
    buf.extend_from_slice(&32u32.to_be_bytes()); // size
    buf.extend_from_slice(b"trex");
    buf.extend_from_slice(&[0, 0, 0, 0]); // version + flags
    buf.extend_from_slice(&track_id.to_be_bytes());
    buf.extend_from_slice(&1u32.to_be_bytes()); // default_sample_description_index
    buf.extend_from_slice(&0u32.to_be_bytes()); // default_sample_duration
    buf.extend_from_slice(&0u32.to_be_bytes()); // default_sample_size
    buf.extend_from_slice(&0u32.to_be_bytes()); // default_sample_flags
}
```

### Step 6: Register in lib.rs
Add `pub mod moov_merger;` to `crates/muxer/src/lib.rs`.

### Step 7: Unit tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::box_parser;

    fn build_simple_moov(track_id: u32, handler_type: &[u8; 4]) -> Vec<u8> {
        // Build minimal moov: [mvhd][trak{tkhd}]
        // mvhd v0: 108 bytes total (8 header + 100 content)
        let mut mvhd = vec![0u8; 108];
        mvhd[0..4].copy_from_slice(&108u32.to_be_bytes());
        mvhd[4..8].copy_from_slice(b"mvhd");
        // version=0, flags=0 at [8..12]
        // next_track_id at offset 104 (8+96)
        box_parser::write_u32_be(&mut mvhd, 104, track_id + 1);

        // tkhd v0: 92 bytes total
        let mut tkhd = vec![0u8; 92];
        tkhd[0..4].copy_from_slice(&92u32.to_be_bytes());
        tkhd[4..8].copy_from_slice(b"tkhd");
        // track_id at content offset 12 => box offset 8+12=20
        box_parser::write_u32_be(&mut tkhd, 20, track_id);

        // trak = 8 + tkhd
        let trak_size = 8 + tkhd.len();
        let mut trak = Vec::with_capacity(trak_size);
        trak.extend_from_slice(&(trak_size as u32).to_be_bytes());
        trak.extend_from_slice(b"trak");
        trak.extend_from_slice(&tkhd);

        // moov = 8 + mvhd + trak
        let moov_size = 8 + mvhd.len() + trak.len();
        let mut moov = Vec::with_capacity(moov_size);
        moov.extend_from_slice(&(moov_size as u32).to_be_bytes());
        moov.extend_from_slice(b"moov");
        moov.extend_from_slice(&mvhd);
        moov.extend_from_slice(&trak);

        moov
    }

    #[test]
    fn test_merge_moov() {
        let v_moov = build_simple_moov(1, b"vide");
        let a_moov = build_simple_moov(1, b"soun");

        let merged = merge_moov(&v_moov, &a_moov).unwrap();

        // Verify it's a valid moov box
        let header = box_parser::read_box_header(&merged).unwrap();
        assert_eq!(&header.box_type, b"moov");
        assert_eq!(header.total_size as usize, merged.len());

        // Should contain 2 trak boxes
        let content = &merged[8..];
        let trak_count = box_parser::iter_boxes(content)
            .filter(|(_, h)| &h.box_type == b"trak")
            .count();
        assert_eq!(trak_count, 2);
    }
}
```

## Todo
- [ ] Create `moov_merger.rs`
- [ ] Implement `merge_moov`
- [ ] Implement `patch_mvhd_next_track_id`
- [ ] Implement `patch_trak_track_id`
- [ ] Implement `build_mvex`
- [ ] Add `find_box_offset` to box_parser or local helper
- [ ] Register in lib.rs
- [ ] Write unit tests
- [ ] Verify compile

## Success Criteria
- Can merge two single-track moov boxes into dual-track moov
- Audio trak track_id correctly patched to 2
- mvhd.next_track_id correctly set to 3
- mvex contains trex for both tracks
- Output is valid moov box (correct size field)

## Risk
- Version 1 mvhd/tkhd offsets are different -- must check version byte
- YouTube moov may contain extra boxes (udta, etc.) -- we only extract mvhd+trak, extras are dropped (acceptable for download use case)

## Security
- Bounds-check all offsets before writing to prevent buffer overflows
- Validate box sizes against actual data length
