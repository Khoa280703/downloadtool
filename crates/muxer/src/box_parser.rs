//! Zero-copy MP4 box (ISOBMFF) boundary parser.
//!
//! Parses box headers and navigates box hierarchies without codec-level parsing.
//! All functions work on `&[u8]` slices (zero-copy).
//!
//! Box layout: `[4B size BE][4B type ASCII][payload...]`
//! Extended size: `[4B size=1][4B type][8B ext_size][payload...]`

/// Parsed MP4 box header.
#[derive(Debug, Clone)]
pub struct BoxHeader {
    /// Four-character code (4CC) identifying the box type.
    pub box_type: [u8; 4],
    /// Total box size in bytes, including header.
    pub total_size: u64,
    /// Header size in bytes: 8 (normal) or 16 (extended size).
    pub header_size: u8,
}

/// Parse box header at the start of `data`.
///
/// Returns `None` if data is too short or the header is malformed.
pub fn read_box_header(data: &[u8]) -> Option<BoxHeader> {
    if data.len() < 8 {
        return None;
    }
    let size = u32::from_be_bytes(data[0..4].try_into().ok()?) as u64;
    let box_type: [u8; 4] = data[4..8].try_into().ok()?;

    match size {
        0 => {
            // Box extends to end of data (valid only for last box in stream)
            Some(BoxHeader {
                box_type,
                total_size: data.len() as u64,
                header_size: 8,
            })
        }
        1 => {
            // Extended 64-bit size follows the 8-byte base header
            if data.len() < 16 {
                return None;
            }
            let ext_size = u64::from_be_bytes(data[8..16].try_into().ok()?);
            Some(BoxHeader {
                box_type,
                total_size: ext_size,
                header_size: 16,
            })
        }
        _ => Some(BoxHeader {
            box_type,
            total_size: size,
            header_size: 8,
        }),
    }
}

/// Iterate top-level boxes in `data`, yielding `(offset, BoxHeader)` for each.
pub fn iter_boxes(data: &[u8]) -> impl Iterator<Item = (usize, BoxHeader)> + '_ {
    let mut offset = 0usize;
    std::iter::from_fn(move || {
        if offset >= data.len() {
            return None;
        }
        let header = read_box_header(&data[offset..])?;
        // Guard against zero or tiny total_size to avoid infinite loop
        if header.total_size < header.header_size as u64 {
            return None;
        }
        let current = offset;
        offset += header.total_size as usize;
        Some((current, header))
    })
}

/// Find the first top-level box of `box_type` and return its full byte slice.
pub fn find_box<'a>(data: &'a [u8], box_type: &[u8; 4]) -> Option<&'a [u8]> {
    for (offset, header) in iter_boxes(data) {
        if &header.box_type == box_type {
            let end = (offset + header.total_size as usize).min(data.len());
            return Some(&data[offset..end]);
        }
    }
    None
}

/// Find the offset of the first top-level box of `box_type`.
pub fn find_box_offset(data: &[u8], box_type: &[u8; 4]) -> Option<usize> {
    for (offset, header) in iter_boxes(data) {
        if &header.box_type == box_type {
            return Some(offset);
        }
    }
    None
}

/// Find a box by hierarchical path, descending into container boxes.
///
/// Example: `find_box_path(data, &[b"moov", b"trak", b"tkhd"])`
pub fn find_box_path<'a>(data: &'a [u8], path: &[&[u8; 4]]) -> Option<&'a [u8]> {
    if path.is_empty() {
        return Some(data);
    }
    let box_data = find_box(data, path[0])?;
    if path.len() == 1 {
        return Some(box_data);
    }
    // Descend into this box's content (skip header)
    let header = read_box_header(box_data)?;
    let content = &box_data[header.header_size as usize..];
    find_box_path(content, &path[1..])
}

/// Read a big-endian u32 at `offset` in `data`.
///
/// # Panics
/// Panics if `offset + 4 > data.len()`.
pub fn read_u32_be(data: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap())
}

/// Write a big-endian u32 at `offset` in `data`.
///
/// # Panics
/// Panics if `offset + 4 > data.len()`.
pub fn write_u32_be(data: &mut [u8], offset: usize, value: u32) {
    data[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
}

/// Read a big-endian u64 at `offset` in `data`.
///
/// # Panics
/// Panics if `offset + 8 > data.len()`.
pub fn read_u64_be(data: &[u8], offset: usize) -> u64 {
    u64::from_be_bytes(data[offset..offset + 8].try_into().unwrap_or([0; 8]))
}

/// Read `base_media_decode_time` from a `tfdt` box inside a `moof` box.
///
/// Box path: moof → traf → tfdt
/// tfdt v0: 4-byte decode time; v1: 8-byte decode time.
/// Returns the value promoted to u64.
pub fn read_tfdt(moof: &[u8]) -> Option<u64> {
    let moof_hdr = read_box_header(moof)?;
    let moof_content = &moof[moof_hdr.header_size as usize..];

    // Find traf inside moof content
    let (traf_off, traf_hdr) = iter_boxes(moof_content)
        .find(|(_, h)| &h.box_type == b"traf")?;
    let traf_content_start = traf_off + traf_hdr.header_size as usize;
    let traf_end = (traf_off + traf_hdr.total_size as usize).min(moof_content.len());
    let traf_content = &moof_content[traf_content_start..traf_end];

    // Find tfdt inside traf content
    let (tfdt_off, _) = iter_boxes(traf_content)
        .find(|(_, h)| &h.box_type == b"tfdt")?;
    let tfdt_data = &traf_content[tfdt_off..];

    // tfdt layout: [4B size][4B "tfdt"][1B version][3B flags][4B or 8B base_media_decode_time]
    // Minimum length: 8 (header) + 1 (version) = 9 bytes to read version
    if tfdt_data.len() < 9 {
        return None;
    }
    let version = tfdt_data[8]; // version byte after [size=4][type=4]
    if version == 1 {
        // v1: 8-byte base_media_decode_time at absolute offset 12 within tfdt box
        if tfdt_data.len() < 20 {
            return None;
        }
        Some(read_u64_be(tfdt_data, 12))
    } else {
        // v0: 4-byte base_media_decode_time at absolute offset 12 within tfdt box
        if tfdt_data.len() < 16 {
            return None;
        }
        Some(read_u32_be(tfdt_data, 12) as u64)
    }
}

/// Read the duration from a `moov` box (first trak's mdhd).
///
/// Box path: moov → trak → mdia → mdhd
/// mdhd v0: duration at content offset 16 (after timescale at 12); v1: at content offset 24.
/// Returns the value promoted to u64.
pub fn read_mdhd_duration(moov: &[u8]) -> Option<u64> {
    let moov_hdr = read_box_header(moov)?;
    let moov_end = (moov_hdr.total_size as usize).min(moov.len());
    let moov_content = &moov[moov_hdr.header_size as usize..moov_end];

    let (trak_off, trak_hdr) = iter_boxes(moov_content)
        .find(|(_, h)| &h.box_type == b"trak")?;
    let trak_end = (trak_off + trak_hdr.total_size as usize).min(moov_content.len());
    let trak_slice = &moov_content[trak_off..trak_end];
    let trak_hdr2 = read_box_header(trak_slice)?;
    let trak_content = &trak_slice[trak_hdr2.header_size as usize..];

    let (mdia_off, mdia_hdr) = iter_boxes(trak_content)
        .find(|(_, h)| &h.box_type == b"mdia")?;
    let mdia_end = (mdia_off + mdia_hdr.total_size as usize).min(trak_content.len());
    let mdia_slice = &trak_content[mdia_off..mdia_end];
    let mdia_hdr2 = read_box_header(mdia_slice)?;
    let mdia_content = &mdia_slice[mdia_hdr2.header_size as usize..];

    let (mdhd_off, _) = iter_boxes(mdia_content)
        .find(|(_, h)| &h.box_type == b"mdhd")?;
    let mdhd = &mdia_content[mdhd_off..];

    if mdhd.len() < 9 {
        return None;
    }
    let version = mdhd[8];
    // v0: timescale at offset 20, duration at offset 24 (u32)
    // v1: timescale at offset 28, duration at offset 32 (u64)
    if version == 1 {
        let dur_abs = 8 + 4 + 8 + 8 + 4; // = 32
        if mdhd.len() < dur_abs + 8 {
            return None;
        }
        Some(read_u64_be(mdhd, dur_abs))
    } else {
        let dur_abs = 8 + 4 + 4 + 4 + 4; // = 24
        if mdhd.len() < dur_abs + 4 {
            return None;
        }
        Some(read_u32_be(mdhd, dur_abs) as u64)
    }
}

/// Read the timescale from a `moov` box (first trak's mdhd).
///
/// Box path: moov → trak → mdia → mdhd
/// mdhd v0: timescale at content offset 12; v1: at content offset 20.
pub fn read_timescale(moov: &[u8]) -> Option<u32> {
    let moov_hdr = read_box_header(moov)?;
    let moov_end = (moov_hdr.total_size as usize).min(moov.len());
    let moov_content = &moov[moov_hdr.header_size as usize..moov_end];

    // Find first trak
    let (trak_off, trak_hdr) = iter_boxes(moov_content)
        .find(|(_, h)| &h.box_type == b"trak")?;
    let trak_end = (trak_off + trak_hdr.total_size as usize).min(moov_content.len());
    let trak_slice = &moov_content[trak_off..trak_end];
    let trak_hdr2 = read_box_header(trak_slice)?;
    let trak_content = &trak_slice[trak_hdr2.header_size as usize..];

    // Find mdia inside trak
    let (mdia_off, mdia_hdr) = iter_boxes(trak_content)
        .find(|(_, h)| &h.box_type == b"mdia")?;
    let mdia_end = (mdia_off + mdia_hdr.total_size as usize).min(trak_content.len());
    let mdia_slice = &trak_content[mdia_off..mdia_end];
    let mdia_hdr2 = read_box_header(mdia_slice)?;
    let mdia_content = &mdia_slice[mdia_hdr2.header_size as usize..];

    // Find mdhd inside mdia
    let (mdhd_off, _) = iter_boxes(mdia_content)
        .find(|(_, h)| &h.box_type == b"mdhd")?;
    let mdhd = &mdia_content[mdhd_off..];

    // mdhd FullBox: [4B size][4B type][1B version][3B flags] = 9 bytes minimum
    if mdhd.len() < 9 {
        return None;
    }
    let version = mdhd[8];
    // v0: [ver+flags=4B][creation=4B][modification=4B][timescale=4B] → timescale at content offset 12
    // v1: [ver+flags=4B][creation=8B][modification=8B][timescale=4B] → timescale at content offset 20
    // Content starts at byte 8 (after 4B size + 4B type)
    let timescale_abs = if version == 1 {
        8 + 4 + 8 + 8 // size+type(8) + ver+flags(4) + creation(8) + modification(8)
    } else {
        8 + 4 + 4 + 4 // size+type(8) + ver+flags(4) + creation(4) + modification(4)
    };
    if mdhd.len() < timescale_abs + 4 {
        return None;
    }
    Some(read_u32_be(mdhd, timescale_abs))
}

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
    fn test_read_box_header_normal() {
        let data = make_box(b"ftyp", &[0u8; 12]);
        let h = read_box_header(&data).unwrap();
        assert_eq!(&h.box_type, b"ftyp");
        assert_eq!(h.total_size, 20);
        assert_eq!(h.header_size, 8);
    }

    #[test]
    fn test_read_box_header_extended_size() {
        // size=1 triggers extended 64-bit size
        let mut data = vec![0, 0, 0, 1]; // size=1
        data.extend_from_slice(b"mdat");
        data.extend_from_slice(&24u64.to_be_bytes()); // ext size=24
        data.extend_from_slice(&[0u8; 8]); // payload
        let h = read_box_header(&data).unwrap();
        assert_eq!(&h.box_type, b"mdat");
        assert_eq!(h.total_size, 24);
        assert_eq!(h.header_size, 16);
    }

    #[test]
    fn test_iter_boxes() {
        let mut data = make_box(b"ftyp", &[0u8; 4]);
        data.extend(make_box(b"moov", &[0u8; 8]));
        let types: Vec<[u8; 4]> = iter_boxes(&data).map(|(_, h)| h.box_type).collect();
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
    fn test_find_box_path() {
        // Build moov > trak nested structure
        let trak = make_box(b"trak", &[0u8; 4]);
        let moov = make_box(b"moov", &trak);
        let mut data = make_box(b"ftyp", &[0u8; 4]);
        data.extend(moov);

        let result = find_box_path(&data, &[b"moov", b"trak"]);
        assert!(result.is_some());
        let h = read_box_header(result.unwrap()).unwrap();
        assert_eq!(&h.box_type, b"trak");
    }

    #[test]
    fn test_read_write_u32_be() {
        let mut data = vec![0u8; 8];
        write_u32_be(&mut data, 2, 0xDEAD_BEEF);
        assert_eq!(read_u32_be(&data, 2), 0xDEAD_BEEF);
    }
}
