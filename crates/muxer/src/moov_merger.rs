//! Merge two single-track moov boxes into a dual-track moov.
//!
//! YouTube adaptive streams each contain a single `trak` with `track_id=1`.
//! This module merges video moov (track 1) + audio moov (track 1 → patched to 2)
//! into a combined moov suitable for a muxed fMP4 file.

use crate::box_parser::{find_box, find_box_offset, read_box_header, write_u32_be};
use crate::MuxerError;

/// Merge video and audio moov boxes into a single dual-track moov.
///
/// Video track retains `track_id=1`; audio track is patched to `track_id=2`.
/// `mvhd.next_track_id` is set to 3.
///
/// Returns `(merged_moov_bytes, video_timescale, audio_timescale)`.
pub fn merge_moov(
    video_moov: &[u8],
    audio_moov: &[u8],
) -> Result<(Vec<u8>, u32, u32), MuxerError> {
    // Read timescales and durations before borrowing slices.
    // NOTE: YouTube DASH init segments set mvhd.duration = 0 per DASH spec.
    // The actual media duration lives in mdhd boxes (track-level media headers).
    let video_timescale = crate::box_parser::read_timescale(video_moov).unwrap_or(90000);
    let audio_timescale = crate::box_parser::read_timescale(audio_moov).unwrap_or(44100);

    // Extract components from video moov content
    let v_hdr = read_box_header(video_moov)
        .ok_or_else(|| MuxerError::InvalidInput("Invalid video moov".into()))?;
    let v_content = &video_moov[v_hdr.header_size as usize..];

    let mvhd = find_box(v_content, b"mvhd")
        .ok_or_else(|| MuxerError::InvalidInput("No mvhd in video moov".into()))?;
    let video_trak = find_box(v_content, b"trak")
        .ok_or_else(|| MuxerError::InvalidInput("No trak in video moov".into()))?;

    // Extract audio trak
    let a_hdr = read_box_header(audio_moov)
        .ok_or_else(|| MuxerError::InvalidInput("Invalid audio moov".into()))?;
    let a_content = &audio_moov[a_hdr.header_size as usize..];

    let audio_trak_src = find_box(a_content, b"trak")
        .ok_or_else(|| MuxerError::InvalidInput("No trak in audio moov".into()))?;

    // Clone mvhd and only patch next_track_id → 3.
    // Leave mvhd.duration = 0 (empty_moov style, like ffmpeg -movflags empty_moov).
    // QuickTime has a bug where non-zero mvhd.duration + fragment-computed duration = double.
    // Keeping duration=0 forces QT to compute duration purely from moof/traf decode times.
    let mut mvhd_patched = mvhd.to_vec();
    patch_mvhd_next_track_id(&mut mvhd_patched, 3)?;

    // Clone video trak and zero out mdhd.duration (empty_moov style).
    // YouTube DASH init sets mdhd.duration to actual media duration. If left non-zero,
    // QuickTime sums mdhd durations across tracks (video 213s + audio 213s = 426s = 7:06).
    let mut video_trak = video_trak.to_vec();
    patch_trak_mdhd_duration(&mut video_trak, 0).ok();

    // Clone audio trak, patch track_id 1 → 2, and zero mdhd.duration.
    let mut audio_trak = audio_trak_src.to_vec();
    patch_trak_track_id(&mut audio_trak, 2)?;
    patch_trak_mdhd_duration(&mut audio_trak, 0).ok();

    // Build mvex: trex for both tracks. No mehd — let QT compute duration from fragments.
    // Adding mehd with the computed duration causes QT to add it to fragment durations (double).
    let mvex = build_mvex(1, 2);

    // Assemble moov = 8-byte header + content
    let content_len = mvhd_patched.len() + video_trak.len() + audio_trak.len() + mvex.len();
    let moov_size = 8 + content_len;

    let mut result = Vec::with_capacity(moov_size);
    result.extend_from_slice(&(moov_size as u32).to_be_bytes());
    result.extend_from_slice(b"moov");
    result.extend_from_slice(&mvhd_patched);
    result.extend_from_slice(&video_trak);
    result.extend_from_slice(&audio_trak);
    result.extend_from_slice(&mvex);

    Ok((result, video_timescale, audio_timescale))
}

/// Patch `mvhd.next_track_id` to `next_track_id`.
///
/// mvhd v0: `next_track_id` at content offset 96 (= box offset 104).
/// mvhd v1: `next_track_id` at content offset 108.
fn patch_mvhd_next_track_id(mvhd: &mut [u8], next_track_id: u32) -> Result<(), MuxerError> {
    let hdr = read_box_header(mvhd)
        .ok_or_else(|| MuxerError::InvalidInput("Invalid mvhd".into()))?;
    let cs = hdr.header_size as usize; // content start

    if mvhd.len() < cs + 1 {
        return Err(MuxerError::InvalidInput("mvhd too short".into()));
    }

    // First byte of FullBox content is version
    let version = mvhd[cs];
    // v0 layout before next_track_id: ver(1)+flags(3)+creation(4)+mod(4)+timescale(4)+duration(4)
    //   + rate(4)+volume(2)+reserved(2+8)+matrix(36)+predefined(24) = 96 bytes
    // v1: creation(8)+mod(8)+timescale(4)+duration(8) adds 12 bytes → offset 108
    let nti_off = cs + if version == 0 { 96 } else { 108 };

    if mvhd.len() < nti_off + 4 {
        return Err(MuxerError::InvalidInput("mvhd too short for next_track_id".into()));
    }

    write_u32_be(mvhd, nti_off, next_track_id);
    Ok(())
}

/// Patch `tkhd.track_id` inside a `trak` box to `new_track_id`.
///
/// tkhd v0: `track_id` at content offset 12.
/// tkhd v1: `track_id` at content offset 20.
fn patch_trak_track_id(trak: &mut [u8], new_track_id: u32) -> Result<(), MuxerError> {
    let trak_hdr = read_box_header(trak)
        .ok_or_else(|| MuxerError::InvalidInput("Invalid trak".into()))?;
    let trak_content_start = trak_hdr.header_size as usize;

    // Find tkhd offset within trak content
    let trak_content = &trak[trak_content_start..];
    let tkhd_off_in_content = find_box_offset(trak_content, b"tkhd")
        .ok_or_else(|| MuxerError::InvalidInput("No tkhd in trak".into()))?;

    let tkhd_abs = trak_content_start + tkhd_off_in_content;
    let tkhd_hdr = read_box_header(&trak[tkhd_abs..])
        .ok_or_else(|| MuxerError::InvalidInput("Invalid tkhd".into()))?;

    let tkhd_content_start = tkhd_abs + tkhd_hdr.header_size as usize;
    if trak.len() < tkhd_content_start + 1 {
        return Err(MuxerError::InvalidInput("tkhd too short".into()));
    }

    let version = trak[tkhd_content_start];
    // v0: ver(1)+flags(3)+creation(4)+mod(4)+track_id(4) → offset 12
    // v1: ver(1)+flags(3)+creation(8)+mod(8)+track_id(4) → offset 20
    let tid_off = tkhd_content_start + if version == 0 { 12 } else { 20 };

    if trak.len() < tid_off + 4 {
        return Err(MuxerError::InvalidInput("tkhd too short for track_id".into()));
    }

    write_u32_be(trak, tid_off, new_track_id);
    Ok(())
}

/// Zero out `mdhd.duration` inside a `trak` box (empty_moov style).
///
/// Path: trak → mdia → mdhd
/// mdhd v0: duration is u32 at content offset 16.
/// mdhd v1: duration is u64 at content offset 24.
fn patch_trak_mdhd_duration(trak: &mut [u8], duration: u32) -> Result<(), MuxerError> {
    let trak_hdr = read_box_header(trak)
        .ok_or_else(|| MuxerError::InvalidInput("Invalid trak for mdhd patch".into()))?;
    let trak_content_start = trak_hdr.header_size as usize;

    // Find mdia in trak
    let mdia_off = find_box_offset(&trak[trak_content_start..], b"mdia")
        .ok_or_else(|| MuxerError::InvalidInput("No mdia in trak".into()))?;
    let mdia_abs = trak_content_start + mdia_off;

    let mdia_hdr = read_box_header(&trak[mdia_abs..])
        .ok_or_else(|| MuxerError::InvalidInput("Invalid mdia".into()))?;
    let mdia_content_start = mdia_abs + mdia_hdr.header_size as usize;

    // Find mdhd in mdia
    let mdhd_off = find_box_offset(&trak[mdia_content_start..], b"mdhd")
        .ok_or_else(|| MuxerError::InvalidInput("No mdhd in mdia".into()))?;
    let mdhd_abs = mdia_content_start + mdhd_off;

    let mdhd_hdr = read_box_header(&trak[mdhd_abs..])
        .ok_or_else(|| MuxerError::InvalidInput("Invalid mdhd".into()))?;
    let mdhd_cs = mdhd_abs + mdhd_hdr.header_size as usize; // content start (= version byte)

    if trak.len() < mdhd_cs + 1 {
        return Err(MuxerError::InvalidInput("mdhd too short".into()));
    }

    let version = trak[mdhd_cs];
    // v0 layout after FullBox header: ver(1)+flags(3)+creation(4)+mod(4)+timescale(4)+duration(4)
    //   → duration at content offset 16
    // v1: creation(8)+mod(8)+timescale(4)+duration(8) → duration at content offset 24
    if version == 0 {
        let dur_off = mdhd_cs + 16;
        if trak.len() < dur_off + 4 {
            return Err(MuxerError::InvalidInput("mdhd v0 too short".into()));
        }
        write_u32_be(trak, dur_off, duration);
    } else {
        let dur_off = mdhd_cs + 24;
        if trak.len() < dur_off + 8 {
            return Err(MuxerError::InvalidInput("mdhd v1 too short".into()));
        }
        // Write u64 as two u32 big-endian halves; duration fits in u32 so high word = 0.
        write_u32_be(trak, dur_off, 0);
        write_u32_be(trak, dur_off + 4, duration);
    }

    Ok(())
}

/// Build an `mvex` box with two `trex` entries for the given track IDs.
///
/// No `mehd` box — leaving duration=0 (empty_moov style) forces QuickTime to compute
/// duration from fragment decode times rather than metadata, avoiding the QT bug where
/// it adds mvhd.duration + fragment duration = doubled duration.
fn build_mvex(video_track_id: u32, audio_track_id: u32) -> Vec<u8> {
    // trex: 8B header + 4B ver/flags + 5×4B fields = 32 bytes
    const TREX_SIZE: u32 = 32;
    let mvex_size = 8 + TREX_SIZE * 2;

    let mut mvex = Vec::with_capacity(mvex_size as usize);
    mvex.extend_from_slice(&mvex_size.to_be_bytes());
    mvex.extend_from_slice(b"mvex");
    write_trex(&mut mvex, video_track_id);
    write_trex(&mut mvex, audio_track_id);
    mvex
}

/// Append a `trex` box for `track_id` to `buf`.
fn write_trex(buf: &mut Vec<u8>, track_id: u32) {
    buf.extend_from_slice(&32u32.to_be_bytes()); // box size
    buf.extend_from_slice(b"trex");
    buf.extend_from_slice(&[0u8; 4]); // version + flags
    buf.extend_from_slice(&track_id.to_be_bytes());
    buf.extend_from_slice(&1u32.to_be_bytes()); // default_sample_description_index
    buf.extend_from_slice(&0u32.to_be_bytes()); // default_sample_duration
    buf.extend_from_slice(&0u32.to_be_bytes()); // default_sample_size
    buf.extend_from_slice(&0u32.to_be_bytes()); // default_sample_flags
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::box_parser;

    /// Build a minimal moov with mvhd v0 + trak{tkhd v0} for testing.
    fn build_simple_moov(track_id: u32) -> Vec<u8> {
        // mvhd v0 = 108 bytes total (8 header + 100 content)
        // next_track_id sits at byte offset 104 (content offset 96)
        let mut mvhd = vec![0u8; 108];
        mvhd[0..4].copy_from_slice(&108u32.to_be_bytes());
        mvhd[4..8].copy_from_slice(b"mvhd");
        box_parser::write_u32_be(&mut mvhd, 104, track_id + 1);

        // tkhd v0 = 92 bytes total (8 header + 84 content)
        // track_id at content offset 12 → box offset 20
        let mut tkhd = vec![0u8; 92];
        tkhd[0..4].copy_from_slice(&92u32.to_be_bytes());
        tkhd[4..8].copy_from_slice(b"tkhd");
        box_parser::write_u32_be(&mut tkhd, 20, track_id);

        // trak = 8-byte header + tkhd
        let trak_size = 8 + tkhd.len();
        let mut trak = Vec::with_capacity(trak_size);
        trak.extend_from_slice(&(trak_size as u32).to_be_bytes());
        trak.extend_from_slice(b"trak");
        trak.extend_from_slice(&tkhd);

        // moov = 8-byte header + mvhd + trak
        let moov_size = 8 + mvhd.len() + trak.len();
        let mut moov = Vec::with_capacity(moov_size);
        moov.extend_from_slice(&(moov_size as u32).to_be_bytes());
        moov.extend_from_slice(b"moov");
        moov.extend_from_slice(&mvhd);
        moov.extend_from_slice(&trak);
        moov
    }

    #[test]
    fn test_merge_moov_structure() {
        let v_moov = build_simple_moov(1);
        let a_moov = build_simple_moov(1);

        let (merged, _vts, _ats) = merge_moov(&v_moov, &a_moov).unwrap();

        // Valid moov box
        let hdr = box_parser::read_box_header(&merged).unwrap();
        assert_eq!(&hdr.box_type, b"moov");
        assert_eq!(hdr.total_size as usize, merged.len());

        // Exactly 2 trak boxes in merged moov content
        let content = &merged[8..];
        let trak_count = box_parser::iter_boxes(content)
            .filter(|(_, h)| &h.box_type == b"trak")
            .count();
        assert_eq!(trak_count, 2);
    }

    #[test]
    fn test_merged_moov_has_mvex() {
        let (merged, _, _) = merge_moov(&build_simple_moov(1), &build_simple_moov(1)).unwrap();
        let content = &merged[8..];
        assert!(box_parser::find_box(content, b"mvex").is_some());
    }

    #[test]
    fn test_patch_mvhd_next_track_id() {
        let moov = build_simple_moov(1);
        // mvhd is first box in moov content (after 8-byte moov header)
        let mvhd_start = 8;
        let mvhd_end = mvhd_start + 108;
        let mut mvhd = moov[mvhd_start..mvhd_end].to_vec();
        patch_mvhd_next_track_id(&mut mvhd, 3).unwrap();
        assert_eq!(box_parser::read_u32_be(&mvhd, 104), 3);
    }
}
