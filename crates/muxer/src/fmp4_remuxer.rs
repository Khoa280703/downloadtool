//! True streaming fMP4 remuxer with dual-traf QuickTime-compatible output.
//!
//! Collects all video and audio fragments, then groups them using a video-led
//! strategy: each video fragment is merged with all audio fragments that fall
//! within its time window into a single moof+mdat pair.
//!
//! Output structure:
//!   ftyp → merged moov → [moof{mfhd + traf_V + traf_A...}][mdat{V+A data}] ...
//!
//! This single-moof-dual-traf format is what QuickTime/AVFoundation requires to
//! correctly identify simultaneous tracks and show the right total duration.

use crate::box_parser::{find_box, iter_boxes, read_box_header, write_u32_be};
use crate::fragment_stream::{Fragment, FragmentReader};
use crate::moov_merger::merge_moov;
use crate::traf_merger::merge_fragments;
use crate::MuxerError;
use bytes::Bytes;
use futures::{Stream, StreamExt};
use std::pin::Pin;
use tracing::{info};

/// Pinned stream of muxed fMP4 bytes.
pub type MuxedStream = Pin<Box<dyn Stream<Item = Result<Bytes, MuxerError>> + Send>>;

/// Remux separate video and audio fMP4 streams into a single muxed fMP4.
///
/// Collects all fragments then emits single-moof-dual-traf output for QuickTime
/// compatibility. QuickTime groups separate moofs by track_id and sums durations;
/// dual-traf format tells it both tracks are simultaneous.
pub fn remux_streams<V, A, VE, AE>(video: V, audio: A) -> MuxedStream
where
    VE: std::error::Error + Send + Sync + 'static,
    AE: std::error::Error + Send + Sync + 'static,
    V: Stream<Item = Result<Bytes, VE>> + Send + 'static,
    A: Stream<Item = Result<Bytes, AE>> + Send + 'static,
{
    let video_mapped = video.map(|r| r.map_err(|e| MuxerError::StreamFetchError(e.to_string())));
    let audio_mapped = audio.map(|r| r.map_err(|e| MuxerError::StreamFetchError(e.to_string())));

    Box::pin(async_stream::try_stream! {
        // Phase A: buffer both complete streams
        let (video_result, audio_result) = tokio::join!(
            collect_stream(video_mapped),
            collect_stream(audio_mapped)
        );
        let video_data = video_result?;
        let audio_data = audio_result?;

        info!(
            "Remuxer: collected video={} bytes, audio={} bytes",
            video_data.len(), audio_data.len()
        );

        // Phase B: extract init segment boxes
        let ftyp = find_box(&video_data, b"ftyp")
            .ok_or_else(|| MuxerError::InvalidInput("No ftyp in video stream".into()))?;
        let video_moov = find_box(&video_data, b"moov")
            .ok_or_else(|| MuxerError::InvalidInput("No moov in video stream".into()))?;
        let audio_moov = find_box(&audio_data, b"moov").ok_or_else(|| {
            MuxerError::InvalidInput("No moov in audio stream (WebM not supported for fMP4 remux)".into())
        })?;

        // Phase C: merge moov, get timescales
        let (merged_moov, video_timescale, audio_timescale) = merge_moov(video_moov, audio_moov)?;

        info!("Timescales: video={}, audio={}", video_timescale, audio_timescale);

        // Phase D: emit init segment.
        // Patch ftyp.major_brand from 'dash' → 'isom' so QuickTime treats this as
        // standard MP4 instead of entering DASH streaming heuristics.
        let ftyp_bytes = {
            let mut b = ftyp.to_vec();
            if b.len() >= 12 {
                b[8..12].copy_from_slice(b"isom");
            }
            b
        };
        yield Bytes::from(ftyp_bytes);
        yield Bytes::from(merged_moov);

        // Phase E: collect ALL fragments from both streams
        let video_frag_data = skip_init_bytes(&video_data);
        let audio_frag_data = skip_init_bytes(&audio_data);

        let video_frag_stream = futures::stream::iter(
            std::iter::once(Ok::<Bytes, MuxerError>(Bytes::copy_from_slice(video_frag_data)))
        );
        let audio_frag_stream = futures::stream::iter(
            std::iter::once(Ok::<Bytes, MuxerError>(Bytes::copy_from_slice(audio_frag_data)))
        );

        let mut video_reader = FragmentReader::new(video_frag_stream);
        let mut audio_reader = FragmentReader::new(audio_frag_stream);

        // Collect all video fragments
        let mut video_frags: Vec<Fragment> = Vec::new();
        loop {
            match video_reader.next_fragment().await {
                Some(Ok(f)) => video_frags.push(f),
                Some(Err(e)) => Err(e)?,
                None => break,
            }
        }

        // Collect all audio fragments; patch track_id to 2 in moof
        let mut audio_frags: Vec<Fragment> = Vec::new();
        loop {
            match audio_reader.next_fragment().await {
                Some(Ok(f)) => {
                    // Patch audio track_id to 2 in its moof
                    let mut moof = f.moof.to_vec();
                    patch_tfhd_track_id(&mut moof, 2)?;
                    audio_frags.push(Fragment {
                        moof: Bytes::from(moof),
                        mdat: f.mdat,
                        tfdt: f.tfdt,
                    });
                }
                Some(Err(e)) => Err(e)?,
                None => break,
            }
        }

        info!(
            "Collected {} video fragments and {} audio fragments",
            video_frags.len(), audio_frags.len()
        );

        // Phase F: video-led grouping → emit merged dual-traf fragments
        //
        // For each video fragment Vi:
        //   next_v_tfdt_norm = tfdt_norm of V(i+1), or u128::MAX if last
        //   audio_group = all audio fragments Aj where tfdt_norm(Aj) < next_v_tfdt_norm
        //   emit merge_fragments(Vi, audio_group, seq)
        //
        // Remaining audio fragments (after all video) → emit as-is patched with seq.
        //
        // Normalization to avoid float:
        //   video tfdt_norm = tfdt * audio_timescale
        //   audio tfdt_norm = tfdt * video_timescale

        let vts = video_timescale as u128;
        let ats = audio_timescale as u128;

        // Build normalized video decode times
        let v_tfdt_norms: Vec<u128> = video_frags.iter()
            .map(|f| f.tfdt as u128 * ats)
            .collect();

        // Build normalized audio decode times
        let a_tfdt_norms: Vec<u128> = audio_frags.iter()
            .map(|f| f.tfdt as u128 * vts)
            .collect();

        let mut audio_cursor = 0usize; // index into audio_frags
        let mut seq = 1u32;

        for (vi, v_frag) in video_frags.iter().enumerate() {
            // Determine the upper bound: normalized tfdt of the NEXT video fragment
            let next_v_norm = v_tfdt_norms.get(vi + 1).copied().unwrap_or(u128::MAX);

            // Collect audio fragments whose normalized tfdt < next_v_norm
            let audio_group_start = audio_cursor;
            while audio_cursor < audio_frags.len()
                && a_tfdt_norms[audio_cursor] < next_v_norm
            {
                audio_cursor += 1;
            }
            let audio_group = &audio_frags[audio_group_start..audio_cursor];

            // Build slice of (moof, mdat) references for traf_merger
            let audio_pairs: Vec<(&[u8], &[u8])> = audio_group
                .iter()
                .map(|f| (f.moof.as_ref(), f.mdat.as_ref()))
                .collect();

            // merge_fragments handles mfhd sequence patching internally
            let merged = merge_fragments(
                v_frag.moof.as_ref(),
                v_frag.mdat.as_ref(),
                &audio_pairs,
                seq,
            );
            seq += 1;

            yield Bytes::from(merged);
        }

        // Emit any remaining audio fragments that follow all video
        while audio_cursor < audio_frags.len() {
            let a_frag = &audio_frags[audio_cursor];
            let mut moof = a_frag.moof.to_vec();
            patch_mfhd_sequence(&mut moof, seq)?;
            seq += 1;
            yield Bytes::from(moof);
            yield a_frag.mdat.clone();
            audio_cursor += 1;
        }

        info!("Remux complete: {} total output fragments", seq - 1);
    })
}

/// Collect all bytes from a stream into a `Vec<u8>`.
async fn collect_stream<S>(stream: S) -> Result<Vec<u8>, MuxerError>
where
    S: Stream<Item = Result<Bytes, MuxerError>> + Send,
{
    futures::pin_mut!(stream);
    let mut buf = Vec::new();
    while let Some(chunk) = stream.next().await {
        let bytes = chunk?;
        buf.extend_from_slice(&bytes);
    }
    Ok(buf)
}

/// Return the byte slice starting at the first `moof` box, skipping init boxes (ftyp + moov).
fn skip_init_bytes(data: &[u8]) -> &[u8] {
    for (offset, header) in iter_boxes(data) {
        if &header.box_type == b"moof" {
            return &data[offset..];
        }
    }
    &data[data.len()..]
}

/// Patch `mfhd.sequence_number` inside a `moof` box.
///
/// mfhd layout: `[4B size][4B "mfhd"][4B ver+flags][4B sequence_number]`
fn patch_mfhd_sequence(moof: &mut [u8], seq: u32) -> Result<(), MuxerError> {
    let hdr = read_box_header(moof)
        .ok_or_else(|| MuxerError::InvalidInput("Invalid moof".into()))?;
    let content_start = hdr.header_size as usize;

    let mfhd_abs: Option<usize> = iter_boxes(&moof[content_start..])
        .find(|(_, h)| &h.box_type == b"mfhd")
        .map(|(off, _)| content_start + off + 12);

    match mfhd_abs {
        Some(abs) if moof.len() >= abs + 4 => {
            write_u32_be(moof, abs, seq);
            Ok(())
        }
        Some(_) => Err(MuxerError::InvalidInput("mfhd too short".into())),
        None => Err(MuxerError::InvalidInput("No mfhd in moof".into())),
    }
}

/// Patch `tfhd.track_id` inside a `moof` box (within the `traf` child).
///
/// tfhd layout: `[4B size][4B "tfhd"][4B ver+flags][4B track_id]`
fn patch_tfhd_track_id(moof: &mut [u8], track_id: u32) -> Result<(), MuxerError> {
    let hdr = read_box_header(moof)
        .ok_or_else(|| MuxerError::InvalidInput("Invalid moof".into()))?;
    let moof_content = hdr.header_size as usize;

    let traf_abs: Option<usize> = iter_boxes(&moof[moof_content..])
        .find(|(_, h)| &h.box_type == b"traf")
        .map(|(off, _)| moof_content + off);

    let traf_abs = traf_abs.ok_or_else(|| MuxerError::InvalidInput("No traf in moof".into()))?;

    let traf_hdr = read_box_header(&moof[traf_abs..])
        .ok_or_else(|| MuxerError::InvalidInput("Invalid traf".into()))?;
    let traf_content = traf_abs + traf_hdr.header_size as usize;

    let tfhd_abs: Option<usize> = iter_boxes(&moof[traf_content..])
        .find(|(_, h)| &h.box_type == b"tfhd")
        .map(|(off, _)| traf_content + off + 12);

    match tfhd_abs {
        Some(abs) if moof.len() >= abs + 4 => {
            write_u32_be(moof, abs, track_id);
            Ok(())
        }
        Some(_) => Err(MuxerError::InvalidInput("tfhd too short".into())),
        None => Err(MuxerError::InvalidInput("No tfhd in traf".into())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::stream;

    /// Build a minimal valid fMP4 byte sequence with one moof+mdat fragment.
    fn build_test_fmp4(track_id: u32) -> Vec<u8> {
        let mut data = Vec::new();

        // ftyp: 20 bytes
        data.extend_from_slice(&20u32.to_be_bytes());
        data.extend_from_slice(b"ftyp");
        data.extend_from_slice(b"isom");
        data.extend_from_slice(&0u32.to_be_bytes());
        data.extend_from_slice(b"isom");

        // mvhd v0 = 108 bytes
        let mut mvhd = vec![0u8; 108];
        mvhd[0..4].copy_from_slice(&108u32.to_be_bytes());
        mvhd[4..8].copy_from_slice(b"mvhd");
        write_u32_be(&mut mvhd, 104, 2); // next_track_id

        // tkhd v0 = 92 bytes; track_id at offset 20
        let mut tkhd = vec![0u8; 92];
        tkhd[0..4].copy_from_slice(&92u32.to_be_bytes());
        tkhd[4..8].copy_from_slice(b"tkhd");
        write_u32_be(&mut tkhd, 20, track_id);

        let trak_size = 8 + tkhd.len();
        let mut trak = Vec::with_capacity(trak_size);
        trak.extend_from_slice(&(trak_size as u32).to_be_bytes());
        trak.extend_from_slice(b"trak");
        trak.extend_from_slice(&tkhd);

        let moov_size = 8 + mvhd.len() + trak.len();
        data.extend_from_slice(&(moov_size as u32).to_be_bytes());
        data.extend_from_slice(b"moov");
        data.extend_from_slice(&mvhd);
        data.extend_from_slice(&trak);

        // moof: mfhd(16B) + traf{tfhd(16B)}
        let mut mfhd = vec![0u8; 16];
        mfhd[0..4].copy_from_slice(&16u32.to_be_bytes());
        mfhd[4..8].copy_from_slice(b"mfhd");
        write_u32_be(&mut mfhd, 12, 1); // sequence_number=1

        let mut tfhd = vec![0u8; 16];
        tfhd[0..4].copy_from_slice(&16u32.to_be_bytes());
        tfhd[4..8].copy_from_slice(b"tfhd");
        tfhd[8..12].copy_from_slice(&[0, 0x02, 0, 0]); // default-base-is-moof flag
        write_u32_be(&mut tfhd, 12, track_id);

        let traf_size = 8 + tfhd.len();
        let mut traf = Vec::with_capacity(traf_size);
        traf.extend_from_slice(&(traf_size as u32).to_be_bytes());
        traf.extend_from_slice(b"traf");
        traf.extend_from_slice(&tfhd);

        let moof_size = 8 + mfhd.len() + traf.len();
        data.extend_from_slice(&(moof_size as u32).to_be_bytes());
        data.extend_from_slice(b"moof");
        data.extend_from_slice(&mfhd);
        data.extend_from_slice(&traf);

        // mdat: 8-byte header + 4 bytes payload
        let payload = [0xDE, 0xAD, 0xBE, 0xEF];
        data.extend_from_slice(&12u32.to_be_bytes());
        data.extend_from_slice(b"mdat");
        data.extend_from_slice(&payload);

        data
    }

    #[tokio::test]
    async fn test_remux_streams_basic() {
        let video_data = build_test_fmp4(1);
        let audio_data = build_test_fmp4(1);

        let video_stream =
            stream::iter(vec![Ok::<_, std::io::Error>(Bytes::from(video_data))]);
        let audio_stream =
            stream::iter(vec![Ok::<_, std::io::Error>(Bytes::from(audio_data))]);

        let mut muxed = remux_streams(video_stream, audio_stream);
        let mut output = Vec::new();

        while let Some(result) = muxed.next().await {
            output.extend_from_slice(&result.unwrap());
        }

        // ftyp, moov, and moof must all be present in output
        assert!(output.windows(4).any(|w| w == b"ftyp"), "missing ftyp");
        assert!(output.windows(4).any(|w| w == b"moov"), "missing moov");
        assert!(output.windows(4).any(|w| w == b"moof"), "missing moof");
    }

    #[test]
    fn test_skip_init_bytes() {
        let data = build_test_fmp4(1);
        let frags = skip_init_bytes(&data);
        let hdr = read_box_header(frags).unwrap();
        assert_eq!(&hdr.box_type, b"moof");
    }

    #[test]
    fn test_patch_mfhd_sequence() {
        let data = build_test_fmp4(1);
        let frags = skip_init_bytes(&data);
        let mut moof = frags[..read_box_header(frags).unwrap().total_size as usize].to_vec();
        patch_mfhd_sequence(&mut moof, 42).unwrap();

        let hdr = read_box_header(&moof).unwrap();
        let moof_content = hdr.header_size as usize;
        for (off, h) in iter_boxes(&moof[moof_content..]) {
            if &h.box_type == b"mfhd" {
                assert_eq!(
                    crate::box_parser::read_u32_be(&moof, moof_content + off + 12),
                    42
                );
                return;
            }
        }
        panic!("mfhd not found after patch");
    }
}
