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
    // Read timescales before borrowing slices
    let video_timescale = crate::box_parser::read_timescale(video_moov).unwrap_or(90000);
    let audio_timescale = crate::box_parser::read_timescale(audio_moov).unwrap_or(44100);
    // Read audio mdhd.duration (in audio timescale) to convert tkhd.duration to movie timescale
    let audio_mdhd_duration = crate::box_parser::read_mdhd_duration(audio_moov).unwrap_or(0);

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

    // Clone mvhd, patch next_track_id → 3, and set duration = max(video, audio)
    let mut mvhd_patched = mvhd.to_vec();
    patch_mvhd_next_track_id(&mut mvhd_patched, 3)?;
    if audio_timescale > 0 && audio_mdhd_duration > 0 {
        let audio_dur_movie_ts = ((audio_mdhd_duration as u128 * video_timescale as u128)
            / audio_timescale as u128) as u32;
        // mvhd.duration should be max of all track durations in movie timescale
        let video_tkhd_dur = read_video_tkhd_duration(v_content).unwrap_or(0);
        let movie_dur = audio_dur_movie_ts.max(video_tkhd_dur);
        patch_mvhd_duration(&mut mvhd_patched, movie_dur)?;
    }

    // Clone audio trak, patch track_id 1 → 2, fix tkhd.duration timescale, and add edts/elst.
    // audio.tkhd.duration is in audio timescale; merged moov uses video timescale,
    // so convert: new_dur = audio_mdhd_duration * video_timescale / audio_timescale
    // edts/elst is required so QuickTime does not sum video+audio durations (→ 7:06 bug).
    let mut audio_trak = audio_trak_src.to_vec();
    patch_trak_track_id(&mut audio_trak, 2)?;
    if audio_timescale > 0 && audio_mdhd_duration > 0 {
        let new_dur = ((audio_mdhd_duration as u128 * video_timescale as u128)
            / audio_timescale as u128) as u32;
        patch_trak_tkhd_duration(&mut audio_trak, new_dur)?;
        insert_trak_edts(&mut audio_trak, new_dur)?;
    }

    // Build mvex with mehd (total duration) + trex for both tracks.
    // mehd.fragment_duration tells QuickTime the total movie duration for fMP4 files,
    // overriding any fragment-scanning heuristics.
    let movie_dur = if audio_timescale > 0 && audio_mdhd_duration > 0 {
        let audio_dur_movie_ts = ((audio_mdhd_duration as u128 * video_timescale as u128)
            / audio_timescale as u128) as u32;
        let video_tkhd_dur = read_video_tkhd_duration(v_content).unwrap_or(0);
        audio_dur_movie_ts.max(video_tkhd_dur)
    } else {
        0
    };
    let mvex = build_mvex(1, 2, movie_dur);

    // Assemble moov = 8-byte header + content
    let content_len = mvhd_patched.len() + video_trak.len() + audio_trak.len() + mvex.len();
    let moov_size = 8 + content_len;

    let mut result = Vec::with_capacity(moov_size);
    result.extend_from_slice(&(moov_size as u32).to_be_bytes());
    result.extend_from_slice(b"moov");
    result.extend_from_slice(&mvhd_patched);
    result.extend_from_slice(video_trak);
    result.extend_from_slice(&audio_trak);
    result.extend_from_slice(&mvex);

    Ok((result, video_timescale, audio_timescale))
}

/// Read `tkhd.duration` from the first `trak` in moov content (already in movie timescale).
fn read_video_tkhd_duration(moov_content: &[u8]) -> Option<u32> {
    let trak = find_box(moov_content, b"trak")?;
    let trak_hdr = read_box_header(trak)?;
    let trak_content = &trak[trak_hdr.header_size as usize..];
    let tkhd_off = find_box_offset(trak_content, b"tkhd")?;
    let tkhd_hdr = read_box_header(&trak_content[tkhd_off..])?;
    let tkhd_cs = tkhd_off + tkhd_hdr.header_size as usize;
    if trak_content.len() < tkhd_cs + 1 {
        return None;
    }
    let version = trak_content[tkhd_cs];
    // v0: duration at content offset 20; v1: at content offset 28
    let dur_off = tkhd_cs + if version == 0 { 20 } else { 28 };
    if trak_content.len() < dur_off + 4 {
        return None;
    }
    Some(crate::box_parser::read_u32_be(trak_content, dur_off))
}

/// Patch `mvhd.duration` to `new_duration` (in movie timescale).
///
/// mvhd v0: duration at content offset 16; v1: at content offset 24 (u64, patched as u32).
fn patch_mvhd_duration(mvhd: &mut [u8], new_duration: u32) -> Result<(), MuxerError> {
    let hdr = read_box_header(mvhd)
        .ok_or_else(|| MuxerError::InvalidInput("Invalid mvhd for duration patch".into()))?;
    let cs = hdr.header_size as usize;
    if mvhd.len() < cs + 1 {
        return Err(MuxerError::InvalidInput("mvhd too short for duration".into()));
    }
    let version = mvhd[cs];
    // v0: ver+flags(4)+creation(4)+mod(4)+timescale(4)+duration(4) → duration at content offset 16
    // v1: ver+flags(4)+creation(8)+mod(8)+timescale(4)+duration(8) → at content offset 24
    let dur_off = cs + if version == 0 { 16 } else { 24 };
    if version == 0 {
        if mvhd.len() < dur_off + 4 {
            return Err(MuxerError::InvalidInput("mvhd v0 too short for duration".into()));
        }
        write_u32_be(mvhd, dur_off, new_duration);
    } else {
        if mvhd.len() < dur_off + 8 {
            return Err(MuxerError::InvalidInput("mvhd v1 too short for duration".into()));
        }
        write_u32_be(mvhd, dur_off, 0);
        write_u32_be(mvhd, dur_off + 4, new_duration);
    }
    Ok(())
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

/// Patch `tkhd.duration` inside a `trak` box to `new_duration` (in movie timescale).
///
/// tkhd v0: `duration` at content offset 20 (= box offset 28, after reserved field).
/// tkhd v1: `duration` at content offset 28 (= box offset 36, u64 — patched as u32 high=0).
fn patch_trak_tkhd_duration(trak: &mut [u8], new_duration: u32) -> Result<(), MuxerError> {
    let trak_hdr = read_box_header(trak)
        .ok_or_else(|| MuxerError::InvalidInput("Invalid trak".into()))?;
    let trak_content_start = trak_hdr.header_size as usize;

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
    // tkhd FullBox content (after 8-byte size+type header = tkhd_content_start):
    // v0: [ver+flags(4)][creation(4)][mod(4)][track_id(4)][reserved(4)][duration(4)] → duration at offset 20
    // v1: [ver+flags(4)][creation(8)][mod(8)][track_id(4)][reserved(4)][duration(8)] → duration at offset 28
    let dur_off = tkhd_content_start + if version == 0 { 20 } else { 28 };

    if version == 0 {
        if trak.len() < dur_off + 4 {
            return Err(MuxerError::InvalidInput("tkhd v0 too short for duration".into()));
        }
        write_u32_be(trak, dur_off, new_duration);
    } else {
        // v1 has 8-byte duration: write as 0||new_duration
        if trak.len() < dur_off + 8 {
            return Err(MuxerError::InvalidInput("tkhd v1 too short for duration".into()));
        }
        write_u32_be(trak, dur_off, 0);
        write_u32_be(trak, dur_off + 4, new_duration);
    }
    Ok(())
}

/// Build an `edts` box containing a single `elst` v0 entry.
///
/// `segment_duration` is in movie timescale units.
/// `media_time = 0` means audio starts at the very beginning of the media.
fn build_edts_v0(segment_duration: u32) -> Vec<u8> {
    // elst v0: 8B header + 4B ver/flags + 4B entry_count + 1 entry×(4+4+4) = 28 bytes
    let mut elst = Vec::with_capacity(28);
    elst.extend_from_slice(&28u32.to_be_bytes()); // box size
    elst.extend_from_slice(b"elst");
    elst.extend_from_slice(&[0u8; 4]); // version=0, flags=0
    elst.extend_from_slice(&1u32.to_be_bytes()); // entry_count=1
    elst.extend_from_slice(&segment_duration.to_be_bytes()); // segment_duration (movie ts)
    elst.extend_from_slice(&0u32.to_be_bytes()); // media_time=0
    elst.extend_from_slice(&0x00010000u32.to_be_bytes()); // media_rate=1.0 (16.16 fixed)

    // edts = 8B header + elst = 36 bytes
    let mut edts = Vec::with_capacity(36);
    edts.extend_from_slice(&36u32.to_be_bytes()); // box size
    edts.extend_from_slice(b"edts");
    edts.extend_from_slice(&elst);
    edts
}

/// Insert an `edts` box into `trak` immediately after the `tkhd` box.
///
/// ISO 14496-12 requires order: tkhd → edts → mdia.
/// QuickTime/AVFoundation ignores `edts` if it appears after `mdia`,
/// which causes incorrect duration display (e.g. 7:06 instead of 3:33).
fn insert_trak_edts(trak: &mut Vec<u8>, duration_movie_ts: u32) -> Result<(), MuxerError> {
    let edts = build_edts_v0(duration_movie_ts);

    // Find tkhd end offset to insert edts right after it (before mdia)
    let trak_hdr = read_box_header(trak)
        .ok_or_else(|| MuxerError::InvalidInput("Invalid trak for edts insert".into()))?;
    let trak_content_start = trak_hdr.header_size as usize;
    let trak_content = &trak[trak_content_start..];

    let tkhd_off = find_box_offset(trak_content, b"tkhd")
        .ok_or_else(|| MuxerError::InvalidInput("No tkhd in trak for edts insert".into()))?;
    let tkhd_hdr = read_box_header(&trak_content[tkhd_off..])
        .ok_or_else(|| MuxerError::InvalidInput("Invalid tkhd for edts insert".into()))?;

    // Insert position: right after tkhd (= trak_content_start + tkhd_off + tkhd_size)
    let insert_pos = trak_content_start + tkhd_off + tkhd_hdr.total_size as usize;

    let new_trak_size = (trak.len() + edts.len()) as u32;
    trak.splice(insert_pos..insert_pos, edts);
    trak[0..4].copy_from_slice(&new_trak_size.to_be_bytes());
    Ok(())
}

/// Build an `mvex` box containing `mehd` + two `trex` entries.
///
/// `mehd` (Movie Extends Header) carries the total fMP4 movie duration so
/// QuickTime/AVFoundation uses it directly instead of scanning fragment timestamps.
/// `fragment_duration` is in movie timescale units; 0 means unknown (omits mehd).
fn build_mvex(video_track_id: u32, audio_track_id: u32, fragment_duration: u32) -> Vec<u8> {
    // trex: 8B header + 4B ver/flags + 5×4B fields = 32 bytes
    const TREX_SIZE: u32 = 32;
    // mehd v0: 8B header + 4B ver/flags + 4B duration = 16 bytes
    const MEHD_SIZE: u32 = 16;
    let mvex_size = if fragment_duration > 0 {
        8 + MEHD_SIZE + TREX_SIZE * 2
    } else {
        8 + TREX_SIZE * 2
    };

    let mut mvex = Vec::with_capacity(mvex_size as usize);
    mvex.extend_from_slice(&mvex_size.to_be_bytes());
    mvex.extend_from_slice(b"mvex");
    if fragment_duration > 0 {
        write_mehd(&mut mvex, fragment_duration);
    }
    write_trex(&mut mvex, video_track_id);
    write_trex(&mut mvex, audio_track_id);
    mvex
}

/// Append a `mehd` v0 box to `buf`.
///
/// `fragment_duration` is the total movie duration in movie timescale units.
fn write_mehd(buf: &mut Vec<u8>, fragment_duration: u32) {
    buf.extend_from_slice(&16u32.to_be_bytes()); // box size = 16
    buf.extend_from_slice(b"mehd");
    buf.extend_from_slice(&[0u8; 4]); // version=0, flags=0
    buf.extend_from_slice(&fragment_duration.to_be_bytes());
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
        let mut moov = build_simple_moov(1);
        // mvhd is first box in moov content (after 8-byte moov header)
        let mvhd_start = 8;
        let mvhd_end = mvhd_start + 108;
        let mut mvhd = moov[mvhd_start..mvhd_end].to_vec();
        patch_mvhd_next_track_id(&mut mvhd, 3).unwrap();
        assert_eq!(box_parser::read_u32_be(&mvhd, 104), 3);
    }
}
