//! Dual-traf moof merger for QuickTime-compatible fMP4 output.
//!
//! Merges a video fragment and one or more audio fragments into a single
//! moof+mdat pair with multiple traf children:
//!
//!   [moof { mfhd + traf_V + traf_A1 + traf_A2... }]
//!   [mdat { v_data + a_data1 + a_data2... }]
//!
//! QuickTime requires this "single-moof-dual-traf" format to correctly
//! identify simultaneous tracks. Separate moof-per-track causes QT to
//! sum fragment durations per track → shows double duration.

use crate::box_parser::{iter_boxes, read_box_header, write_u32_be};

/// Merge a video fragment + one or more audio fragments into a single moof+mdat.
///
/// # Arguments
/// - `v_moof` – raw bytes of the video moof box
/// - `v_mdat` – raw bytes of the video mdat box
/// - `audio_frags` – slice of `(a_moof, a_mdat)` pairs (may be empty)
/// - `sequence_number` – sequence number for the merged mfhd
///
/// # Returns
/// Concatenated bytes: `[merged_moof][merged_mdat]`
///
/// If `audio_frags` is empty, returns the video fragment with the mfhd
/// sequence number patched (no structural changes).
pub fn merge_fragments(
    v_moof: &[u8],
    v_mdat: &[u8],
    audio_frags: &[(&[u8], &[u8])],
    sequence_number: u32,
) -> Vec<u8> {
    if audio_frags.is_empty() {
        // Fast path: no audio to merge, just patch sequence number and return.
        let mut out = Vec::with_capacity(v_moof.len() + v_mdat.len());
        let mut moof_patched = v_moof.to_vec();
        patch_mfhd_sequence_number(&mut moof_patched, sequence_number);
        out.extend_from_slice(&moof_patched);
        out.extend_from_slice(v_mdat);
        return out;
    }

    // Extract traf from video moof: skip 8B moof header + 16B mfhd = bytes[24..]
    // moof layout: [8B header][16B mfhd][traf...]
    let v_traf = extract_traf_bytes(v_moof);

    // Extract each audio traf
    let audio_trafs: Vec<&[u8]> = audio_frags
        .iter()
        .map(|(a_moof, _)| extract_traf_bytes(a_moof))
        .collect();

    // Compute merged moof size:
    //   8B moof header + 16B mfhd + traf_V.len() + sum(traf_Ai.len())
    let traf_total: usize = audio_trafs.iter().map(|t| t.len()).sum::<usize>() + v_traf.len();
    let merged_moof_size = 8 + 16 + traf_total;

    // v_mdat payload size (mdat total minus 8B header)
    let v_data_size = v_mdat.len().saturating_sub(8);

    // Patch trun.data_offset in video traf.
    // The offset is from the start of moof to the first byte of the video payload in mdat.
    // mdat layout: [8B header][payload...]
    // data_offset = merged_moof_size + 8  (8 = mdat header)
    let v_data_offset = (merged_moof_size + 8) as u32;
    let mut v_traf_patched = v_traf.to_vec();
    patch_trun_data_offset(&mut v_traf_patched, v_data_offset);

    // Patch trun.data_offset in each audio traf.
    // Each audio track's payload starts after: moof + mdat_header + v_data + prior_a_data
    let mut audio_trafs_patched: Vec<Vec<u8>> = Vec::with_capacity(audio_frags.len());
    let mut cumulative_audio_offset: usize = 0;
    for (i, a_traf) in audio_trafs.iter().enumerate() {
        let a_data_offset = (merged_moof_size + 8 + v_data_size + cumulative_audio_offset) as u32;
        let mut patched = a_traf.to_vec();
        patch_trun_data_offset(&mut patched, a_data_offset);
        audio_trafs_patched.push(patched);
        // Accumulate audio payload sizes for subsequent tracks
        let a_mdat = audio_frags[i].1;
        cumulative_audio_offset += a_mdat.len().saturating_sub(8);
    }

    // Build merged moof:
    //   [4B size][4B "moof"][mfhd_from_v_moof_patched][traf_V_patched][traf_A1_patched]...
    let mut merged_moof = Vec::with_capacity(merged_moof_size);
    merged_moof.extend_from_slice(&(merged_moof_size as u32).to_be_bytes());
    merged_moof.extend_from_slice(b"moof");

    // Copy mfhd from video moof (bytes [8..24]) and patch sequence number.
    // mfhd: [4B size=16][4B "mfhd"][4B ver+flags=0][4B sequence_number]
    let mut mfhd = v_moof[8..24].to_vec(); // 16 bytes
    write_u32_be(&mut mfhd, 12, sequence_number);
    merged_moof.extend_from_slice(&mfhd);

    merged_moof.extend_from_slice(&v_traf_patched);
    for a_traf in &audio_trafs_patched {
        merged_moof.extend_from_slice(a_traf);
    }

    debug_assert_eq!(merged_moof.len(), merged_moof_size, "moof size mismatch");

    // Build merged mdat:
    //   [4B size][4B "mdat"][v_payload][a_payload1][a_payload2]...
    let total_payload = v_data_size + cumulative_audio_offset;
    let merged_mdat_size = 8 + total_payload;
    let mut merged_mdat = Vec::with_capacity(merged_mdat_size);
    merged_mdat.extend_from_slice(&(merged_mdat_size as u32).to_be_bytes());
    merged_mdat.extend_from_slice(b"mdat");
    // Append video payload (skip 8B mdat header)
    if v_mdat.len() > 8 {
        merged_mdat.extend_from_slice(&v_mdat[8..]);
    }
    // Append each audio payload
    for (_, a_mdat) in audio_frags {
        if a_mdat.len() > 8 {
            merged_mdat.extend_from_slice(&a_mdat[8..]);
        }
    }

    let mut out = Vec::with_capacity(merged_moof.len() + merged_mdat.len());
    out.extend_from_slice(&merged_moof);
    out.extend_from_slice(&merged_mdat);
    out
}

/// Extract the traf bytes from a moof box.
///
/// Assumes standard YouTube DASH moof layout:
///   [8B moof header][16B mfhd][traf box(es)...]
///
/// Returns the slice starting at the first traf box within the moof.
/// If parsing fails or no traf found, returns an empty slice.
fn extract_traf_bytes(moof: &[u8]) -> &[u8] {
    let moof_hdr = match read_box_header(moof) {
        Some(h) => h,
        None => return &moof[0..0],
    };
    let moof_end = (moof_hdr.total_size as usize).min(moof.len());
    let content_start = moof_hdr.header_size as usize;
    let content = &moof[content_start..moof_end];

    // Find the first traf box in the moof content
    for (off, hdr) in iter_boxes(content) {
        if &hdr.box_type == b"traf" {
            // Return from the start of traf to end of moof content
            // (include all subsequent traf boxes if multiple exist)
            return &content[off..];
        }
    }
    &moof[0..0]
}

/// Patch `trun.data_offset` inside a traf byte slice.
///
/// trun box layout (FullBox):
///   [4B size][4B "trun"][1B version][3B flags][4B sample_count][4B data_offset?][...]
///
/// `data_offset` is present iff `flags & 0x000001 != 0` (bit 0 of the 24-bit flags field).
/// The flags field occupies bytes [9..12] of the trun box (bytes 1..4 of the FullBox content).
/// The `data_offset_present` bit is bit 0 of the LSB of the 3-byte flags, i.e. byte [11].
///
/// If `data_offset` is present, it lives at trun box offset 16:
///   8B (header) + 4B (ver+flags) + 4B (sample_count) = 16B
fn patch_trun_data_offset(traf: &mut Vec<u8>, new_offset: u32) {
    // traf is the full traf box: [8B header][child boxes...]
    // We must skip the outer traf header to iterate its children (tfhd, trun, tfdt...).
    let content_start = match read_box_header(traf.as_slice()) {
        Some(hdr) if &hdr.box_type == b"traf" => hdr.header_size as usize,
        _ => 0, // fallback: treat entire buffer as child boxes
    };

    let mut pos = content_start;
    while pos < traf.len() {
        let remaining = &traf[pos..];
        let hdr = match read_box_header(remaining) {
            Some(h) => h,
            None => break,
        };
        if hdr.total_size < hdr.header_size as u64 {
            break;
        }
        let box_end = pos + hdr.total_size as usize;

        if &hdr.box_type == b"trun" {
            // trun must be at least 16 bytes: 8B header + 4B ver+flags + 4B sample_count
            if traf.len() >= pos + 16 {
                // flags LSB is at box offset 11: [size=4][type=4][ver=1][flags[0]=1][flags[1]=1][flags[2]=1]
                // data_offset_present = flags[2] bit 0, i.e. trun[pos+11] & 0x01
                let flags_lsb = traf[pos + 11];
                if flags_lsb & 0x01 != 0 {
                    // data_offset field at box offset 16
                    if traf.len() >= pos + 20 {
                        write_u32_be(traf, pos + 16, new_offset);
                    }
                }
                // If data_offset_present is not set, YouTube always sets it; skip quietly.
            }
        }

        pos = box_end;
        if pos == 0 {
            break; // guard against zero-size boxes
        }
    }
}

/// Patch `mfhd.sequence_number` in a standalone moof-sized buffer.
/// Used in the fast-path (no audio to merge).
fn patch_mfhd_sequence_number(moof: &mut Vec<u8>, seq: u32) {
    let hdr = match read_box_header(moof) {
        Some(h) => h,
        None => return,
    };
    let content_start = hdr.header_size as usize;
    // Collect the absolute write offset before releasing the immutable borrow.
    let write_abs: Option<usize> = {
        iter_boxes(&moof[content_start..])
            .find(|(_, h)| &h.box_type == b"mfhd")
            .map(|(off, _)| content_start + off + 12)
    };
    if let Some(abs) = write_abs {
        if moof.len() >= abs + 4 {
            write_u32_be(moof, abs, seq);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::box_parser::{read_u32_be, write_u32_be as w32};

    /// Build a minimal moof: mfhd(16B) + traf{tfhd(16B) + trun(24B with data_offset)}.
    fn build_test_moof(track_id: u32, seq: u32) -> Vec<u8> {
        // mfhd
        let mut mfhd = vec![0u8; 16];
        mfhd[0..4].copy_from_slice(&16u32.to_be_bytes());
        mfhd[4..8].copy_from_slice(b"mfhd");
        w32(&mut mfhd, 12, seq);

        // tfhd: [16][tfhd][flags=0x020000][track_id]
        let mut tfhd = vec![0u8; 16];
        tfhd[0..4].copy_from_slice(&16u32.to_be_bytes());
        tfhd[4..8].copy_from_slice(b"tfhd");
        tfhd[8..12].copy_from_slice(&[0x00, 0x02, 0x00, 0x00]); // default-base-is-moof
        w32(&mut tfhd, 12, track_id);

        // trun v0 with data_offset_present (flags=0x000001):
        // [size][trun][ver=0][flags=0x000001][sample_count=1][data_offset=999][sample_duration=3000]
        let mut trun = vec![0u8; 24];
        trun[0..4].copy_from_slice(&24u32.to_be_bytes());
        trun[4..8].copy_from_slice(b"trun");
        trun[8..12].copy_from_slice(&[0x00, 0x00, 0x00, 0x01]); // ver=0, flags=0x000001
        w32(&mut trun, 12, 1); // sample_count = 1
        w32(&mut trun, 16, 999); // data_offset placeholder
        w32(&mut trun, 20, 3000); // sample_duration (arbitrary extra sample field)

        let traf_size = 8 + tfhd.len() + trun.len();
        let mut traf = Vec::with_capacity(traf_size);
        traf.extend_from_slice(&(traf_size as u32).to_be_bytes());
        traf.extend_from_slice(b"traf");
        traf.extend_from_slice(&tfhd);
        traf.extend_from_slice(&trun);

        let moof_size = 8 + mfhd.len() + traf.len();
        let mut moof = Vec::with_capacity(moof_size);
        moof.extend_from_slice(&(moof_size as u32).to_be_bytes());
        moof.extend_from_slice(b"moof");
        moof.extend_from_slice(&mfhd);
        moof.extend_from_slice(&traf);
        moof
    }

    fn build_test_mdat(payload: &[u8]) -> Vec<u8> {
        let size = 8 + payload.len() as u32;
        let mut mdat = Vec::with_capacity(size as usize);
        mdat.extend_from_slice(&size.to_be_bytes());
        mdat.extend_from_slice(b"mdat");
        mdat.extend_from_slice(payload);
        mdat
    }

    #[test]
    fn test_merge_fragments_empty_audio() {
        let v_moof = build_test_moof(1, 5);
        let v_mdat = build_test_mdat(&[0xDE, 0xAD]);

        let out = merge_fragments(&v_moof, &v_mdat, &[], 7);
        // Output must start with a valid moof box
        assert!(out.windows(4).any(|w| w == b"moof"), "no moof in output");
        // Sequence number must be patched to 7
        let moof_end = read_box_header(&out).unwrap().total_size as usize;
        let moof_bytes = &out[..moof_end];
        let content_start = 8;
        for (off, h) in iter_boxes(&moof_bytes[content_start..]) {
            if &h.box_type == b"mfhd" {
                let seq = read_u32_be(moof_bytes, content_start + off + 12);
                assert_eq!(seq, 7, "sequence number not patched");
                break;
            }
        }
    }

    #[test]
    fn test_merge_fragments_with_audio() {
        let v_moof = build_test_moof(1, 1);
        let v_mdat = build_test_mdat(&[0x01, 0x02, 0x03, 0x04]); // 4 bytes payload

        let a_moof = build_test_moof(2, 1);
        let a_mdat = build_test_mdat(&[0xAA, 0xBB]); // 2 bytes payload

        let audio_frags = [(&a_moof[..], &a_mdat[..])];
        let out = merge_fragments(&v_moof, &v_mdat, &audio_frags, 3);

        // Output must have exactly one moof and one mdat
        let moof_count = out.windows(4).filter(|w| *w == b"moof").count();
        let mdat_count = out.windows(4).filter(|w| *w == b"mdat").count();
        assert_eq!(moof_count, 1, "should be exactly 1 moof");
        assert_eq!(mdat_count, 1, "should be exactly 1 mdat");

        // Verify merged moof contains 2 traf boxes (one video, one audio)
        let moof_hdr = read_box_header(&out).unwrap();
        let moof_end = moof_hdr.total_size as usize;
        let moof_content_start = moof_hdr.header_size as usize;
        let moof_bytes = &out[..moof_end];
        let traf_count = iter_boxes(&moof_bytes[moof_content_start..])
            .filter(|(_, h)| &h.box_type == b"traf")
            .count();
        assert_eq!(traf_count, 2, "merged moof should have 2 traf boxes");

        // Verify mfhd sequence number was patched to 3
        for (off, h) in iter_boxes(&moof_bytes[moof_content_start..]) {
            if &h.box_type == b"mfhd" {
                let seq = read_u32_be(moof_bytes, moof_content_start + off + 12);
                assert_eq!(seq, 3, "sequence number should be 3");
                break;
            }
        }

        // Verify merged mdat contains video payload + audio payload
        let mdat_start = moof_end;
        let mdat_bytes = &out[mdat_start..];
        assert!(mdat_bytes.len() >= 8 + 4 + 2, "mdat payload too short");
        assert_eq!(&mdat_bytes[8..12], &[0x01, 0x02, 0x03, 0x04], "video payload");
        assert_eq!(&mdat_bytes[12..14], &[0xAA, 0xBB], "audio payload");
    }

    #[test]
    fn test_data_offset_patching() {
        let v_moof = build_test_moof(1, 1);
        let v_mdat = build_test_mdat(&[0x01, 0x02, 0x03, 0x04]); // 4 bytes

        let a_moof = build_test_moof(2, 1);
        let a_mdat = build_test_mdat(&[0xAA]); // 1 byte

        let audio_frags = [(&a_moof[..], &a_mdat[..])];
        let out = merge_fragments(&v_moof, &v_mdat, &audio_frags, 1);

        let moof_hdr = read_box_header(&out).unwrap();
        let merged_moof_size = moof_hdr.total_size as usize;
        let moof_content_start = moof_hdr.header_size as usize;
        let moof_bytes = &out[..merged_moof_size];

        // Find video traf (first traf)
        let mut traf_iter = iter_boxes(&moof_bytes[moof_content_start..])
            .filter(|(_, h)| &h.box_type == b"traf");

        let (v_traf_off, v_traf_hdr) = traf_iter.next().expect("video traf");
        let v_traf_abs = moof_content_start + v_traf_off;
        let v_traf_content_start = v_traf_abs + v_traf_hdr.header_size as usize;
        let v_traf_end = v_traf_abs + v_traf_hdr.total_size as usize;

        // Find trun in video traf
        for (trun_off, trun_hdr) in iter_boxes(&moof_bytes[v_traf_content_start..v_traf_end]) {
            if &trun_hdr.box_type == b"trun" {
                let trun_abs = v_traf_content_start + trun_off;
                // Check data_offset_present flag
                let flags_lsb = moof_bytes[trun_abs + 11];
                if flags_lsb & 0x01 != 0 {
                    let v_data_offset = read_u32_be(moof_bytes, trun_abs + 16);
                    // Expected: merged_moof_size + 8 (mdat header)
                    let expected = (merged_moof_size + 8) as u32;
                    assert_eq!(
                        v_data_offset, expected,
                        "video trun data_offset should point past mdat header"
                    );
                }
                break;
            }
        }

        // Find audio traf (second traf)
        let (a_traf_off, a_traf_hdr) = traf_iter.next().expect("audio traf");
        let a_traf_abs = moof_content_start + a_traf_off;
        let a_traf_content_start = a_traf_abs + a_traf_hdr.header_size as usize;
        let a_traf_end = a_traf_abs + a_traf_hdr.total_size as usize;

        for (trun_off, trun_hdr) in iter_boxes(&moof_bytes[a_traf_content_start..a_traf_end]) {
            if &trun_hdr.box_type == b"trun" {
                let trun_abs = a_traf_content_start + trun_off;
                let flags_lsb = moof_bytes[trun_abs + 11];
                if flags_lsb & 0x01 != 0 {
                    let a_data_offset = read_u32_be(moof_bytes, trun_abs + 16);
                    // Expected: merged_moof_size + 8 (mdat header) + 4 (video payload)
                    let expected = (merged_moof_size + 8 + 4) as u32;
                    assert_eq!(
                        a_data_offset, expected,
                        "audio trun data_offset should point past video payload"
                    );
                }
                break;
            }
        }
    }
}
