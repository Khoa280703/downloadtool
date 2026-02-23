# Phase 2: MP4 Box Parser

## Context
- [ISO 14496-12 (ISOBMFF)](https://www.iso.org/standard/68960.html) -- MP4 box format spec
- New file: `crates/muxer/src/box_parser.rs`

## Overview
- **Priority:** P1
- **Status:** pending
- **Description:** Zero-copy MP4 box boundary parser. No codec parsing. Just size+type+children.

## Key Insights
- MP4 box = `[4B size BE][4B type ASCII][payload...]`
- If size==1: extended size in next 8 bytes `[4B size=1][4B type][8B extended_size][payload...]`
- If size==0: box extends to EOF (only valid for last box)
- Header is 8 bytes (normal) or 16 bytes (extended)
- We only need: iterate boxes, find box by type, read/write u32 BE at offset
- No need to parse box contents except for traversing container boxes (moov, trak, moof, traf)

## Requirements
### Functional
- Parse box header: return (total_size, type_4cc, header_size)
- Iterate top-level boxes in a byte slice
- Find first box of given type (non-recursive)
- Find box by path (recursive): e.g., `["moov", "trak", "tkhd"]`
- Read/write u32 BE at arbitrary offset
- Handle extended size (size==1)

### Non-functional
- Pure functions, no allocation except return values
- Work on `&[u8]` slices (zero-copy)
- Under 150 lines

## Architecture
```rust
// box_parser.rs public API

/// Parsed box header info
pub struct BoxHeader {
    pub box_type: [u8; 4],
    pub total_size: u64,    // includes header
    pub header_size: u8,    // 8 or 16
}

/// Parse box header at start of data
pub fn read_box_header(data: &[u8]) -> Option<BoxHeader>

/// Iterate boxes: yields (offset, BoxHeader) for each top-level box
pub fn iter_boxes(data: &[u8]) -> impl Iterator<Item = (usize, BoxHeader)>

/// Find first box of given type (top-level only)
pub fn find_box<'a>(data: &'a [u8], box_type: &[u8; 4]) -> Option<&'a [u8]>

/// Find box by path: e.g., find_box_path(data, &[b"moov", b"trak", b"tkhd"])
pub fn find_box_path<'a>(data: &'a [u8], path: &[&[u8; 4]]) -> Option<&'a [u8]>

/// Read u32 big-endian at offset
pub fn read_u32_be(data: &[u8], offset: usize) -> u32

/// Write u32 big-endian at offset
pub fn write_u32_be(data: &mut [u8], offset: usize, value: u32)
```

## Related Code Files
- **Create:** `crates/muxer/src/box_parser.rs`
- **Modify:** `crates/muxer/src/lib.rs` (add `pub mod box_parser;`)

## Implementation Steps

### Step 1: Create `box_parser.rs` with header parsing
```rust
/// MP4 box header
#[derive(Debug, Clone)]
pub struct BoxHeader {
    pub box_type: [u8; 4],
    pub total_size: u64,
    pub header_size: u8,
}

pub fn read_box_header(data: &[u8]) -> Option<BoxHeader> {
    if data.len() < 8 {
        return None;
    }
    let size = u32::from_be_bytes(data[0..4].try_into().ok()?) as u64;
    let box_type: [u8; 4] = data[4..8].try_into().ok()?;

    match size {
        0 => {
            // Box extends to end of data
            Some(BoxHeader { box_type, total_size: data.len() as u64, header_size: 8 })
        }
        1 => {
            // Extended size
            if data.len() < 16 { return None; }
            let ext_size = u64::from_be_bytes(data[8..16].try_into().ok()?);
            Some(BoxHeader { box_type, total_size: ext_size, header_size: 16 })
        }
        _ => {
            Some(BoxHeader { box_type, total_size: size, header_size: 8 })
        }
    }
}
```

### Step 2: Box iterator
```rust
pub fn iter_boxes(data: &[u8]) -> impl Iterator<Item = (usize, BoxHeader)> + '_ {
    let mut offset = 0usize;
    std::iter::from_fn(move || {
        if offset >= data.len() { return None; }
        let header = read_box_header(&data[offset..])?;
        let current_offset = offset;
        offset += header.total_size as usize;
        Some((current_offset, header))
    })
}
```

### Step 3: Find box by type
```rust
pub fn find_box<'a>(data: &'a [u8], box_type: &[u8; 4]) -> Option<&'a [u8]> {
    for (offset, header) in iter_boxes(data) {
        if &header.box_type == box_type {
            let end = offset + header.total_size as usize;
            return Some(&data[offset..end.min(data.len())]);
        }
    }
    None
}
```

### Step 4: Find box by path (recursive descent into container boxes)
```rust
pub fn find_box_path<'a>(data: &'a [u8], path: &[&[u8; 4]]) -> Option<&'a [u8]> {
    if path.is_empty() { return Some(data); }

    let box_data = find_box(data, path[0])?;
    if path.len() == 1 { return Some(box_data); }

    // Get content after header
    let header = read_box_header(box_data)?;
    let content = &box_data[header.header_size as usize..];
    find_box_path(content, &path[1..])
}
```

### Step 5: u32 BE helpers
```rust
pub fn read_u32_be(data: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap())
}

pub fn write_u32_be(data: &mut [u8], offset: usize, value: u32) {
    data[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
}
```

### Step 6: Register in lib.rs
Add `pub mod box_parser;` to `crates/muxer/src/lib.rs`.

### Step 7: Unit tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn make_box(box_type: &[u8; 4], payload: &[u8]) -> Vec<u8> {
        let size = (8 + payload.len()) as u32;
        let mut data = size.to_be_bytes().to_vec();
        data.extend_from_slice(box_type);
        data.extend_from_slice(payload);
        data
    }

    #[test]
    fn test_read_box_header() {
        let data = make_box(b"ftyp", &[0u8; 12]);
        let h = read_box_header(&data).unwrap();
        assert_eq!(&h.box_type, b"ftyp");
        assert_eq!(h.total_size, 20);
        assert_eq!(h.header_size, 8);
    }

    #[test]
    fn test_iter_boxes() {
        let mut data = make_box(b"ftyp", &[0u8; 4]);
        data.extend(make_box(b"moov", &[0u8; 8]));
        let types: Vec<[u8;4]> = iter_boxes(&data).map(|(_, h)| h.box_type).collect();
        assert_eq!(types, vec![*b"ftyp", *b"moov"]);
    }

    #[test]
    fn test_find_box() {
        let mut data = make_box(b"ftyp", &[0u8; 4]);
        data.extend(make_box(b"moov", &[0u8; 8]));
        assert!(find_box(&data, b"moov").is_some());
        assert!(find_box(&data, b"mdat").is_none());
    }

    #[test]
    fn test_extended_size() {
        // size=1 means extended
        let mut data = vec![0, 0, 0, 1]; // size=1
        data.extend_from_slice(b"mdat");
        data.extend_from_slice(&24u64.to_be_bytes()); // extended size = 24
        data.extend_from_slice(&[0u8; 8]); // payload
        let h = read_box_header(&data).unwrap();
        assert_eq!(h.total_size, 24);
        assert_eq!(h.header_size, 16);
    }
}
```

## Todo
- [ ] Create `box_parser.rs`
- [ ] Implement `read_box_header` with extended size support
- [ ] Implement `iter_boxes`
- [ ] Implement `find_box` and `find_box_path`
- [ ] Implement `read_u32_be` / `write_u32_be`
- [ ] Register module in `lib.rs`
- [ ] Write unit tests
- [ ] Verify compile

## Success Criteria
- Can parse box headers (normal + extended size)
- Can iterate top-level boxes
- Can find nested box by path (e.g., moov > trak > tkhd)
- All tests pass
- Under 150 lines

## Risk
- Malformed boxes could cause slice panics -- use bounds checking
- Need to handle size=0 (extends to EOF) for robustness
