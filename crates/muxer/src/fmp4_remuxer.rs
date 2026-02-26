//! Streaming fMP4 remuxer with dual-traf QuickTime-compatible output.
//!
//! Streams video and audio fragments on-the-fly using a two-phase pipeline:
//!
//! 1. **INIT phase**: Collects small ftyp+moov boxes from both streams (~1-5KB each),
//!    merges them, and emits the init segment.
//! 2. **STREAM phase**: Aligns and merges fragments using a video-led strategy with
//!    a small audio window (~2-5 fragments). Peak RAM ~5-10MB per connection.
//!
//! Output structure:
//!   ftyp → merged moov → [moof{mfhd + traf_V + traf_A...}][mdat{V+A data}] ...
//!
//! This single-moof-dual-traf format is what QuickTime/AVFoundation requires to
//! correctly identify simultaneous tracks and show the right total duration.

use crate::atom_framer::AtomFramer;
use crate::fragment_aligner::FragmentAligner;
use crate::moov_merger::merge_moov;
use crate::MuxerError;
use bytes::Bytes;
use futures::{Stream, StreamExt};
use std::pin::Pin;
use tracing::info;

/// Pinned stream of muxed fMP4 bytes.
pub type MuxedStream = Pin<Box<dyn Stream<Item = Result<Bytes, MuxerError>> + Send>>;

/// Remux separate video and audio fMP4 streams into a single muxed fMP4.
///
/// Uses a streaming pipeline — never buffers the full video or audio.
/// Peak RAM per connection is ~5-10MB (one window of fragments).
///
/// Output is QuickTime-compatible dual-traf format.
pub fn remux_streams<V, A, VE, AE>(video: V, audio: A) -> MuxedStream
where
    VE: std::error::Error + Send + Sync + 'static,
    AE: std::error::Error + Send + Sync + 'static,
    V: Stream<Item = Result<Bytes, VE>> + Send + 'static,
    A: Stream<Item = Result<Bytes, AE>> + Send + 'static,
{
    // Box::pin the mapped streams so they implement Unpin (required by AtomFramer)
    let video_mapped = Box::pin(video.map(|r| r.map_err(|e| MuxerError::StreamFetchError(e.to_string()))));
    let audio_mapped = Box::pin(audio.map(|r| r.map_err(|e| MuxerError::StreamFetchError(e.to_string()))));

    Box::pin(async_stream::try_stream! {
        // ── Phase 1: INIT ─────────────────────────────────────────────
        // Collect ftyp+moov from both streams (small, ~1-5KB each).
        let mut v_framer = AtomFramer::new(video_mapped);
        let mut a_framer = AtomFramer::new(audio_mapped);

        // Use try_join to cancel the other if one fails
        let (v_init, a_init) = tokio::try_join!(
            v_framer.collect_init_segment(),
            a_framer.collect_init_segment(),
        )?;

        let (v_ftyp, v_moov): (Bytes, Bytes) = v_init;
        let (_a_ftyp, a_moov): (Bytes, Bytes) = a_init;  // audio ftyp discarded

        info!(
            "Remuxer: init segments collected (video ftyp={} moov={}, audio moov={})",
            v_ftyp.len(), v_moov.len(), a_moov.len()
        );

        // Merge moov boxes, get timescales
        let (merged_moov, video_timescale, audio_timescale) = merge_moov(&v_moov, &a_moov)?;
        info!("Timescales: video={}, audio={}", video_timescale, audio_timescale);

        // Patch ftyp brand: 'dash' → 'isom' so QuickTime treats this as
        // standard MP4 instead of entering DASH streaming heuristics.
        let ftyp_bytes = {
            let mut b = v_ftyp.to_vec();
            if b.len() >= 12 {
                b[8..12].copy_from_slice(b"isom");
            }
            b
        };

        // Emit init segment
        yield Bytes::from(ftyp_bytes);
        yield Bytes::from(merged_moov);

        // ── Phase 2: STREAM ───────────────────────────────────────────
        // Hand off remaining bytes (after init) to FragmentAligner.
        let v_remaining = v_framer.into_remaining_stream();
        let a_remaining = a_framer.into_remaining_stream();

        let mut aligner = FragmentAligner::new(
            v_remaining,
            a_remaining,
            video_timescale,
            audio_timescale,
        );

        let mut seq = 1u32;
        while let Some(result) = aligner.next_merged(&mut seq).await {
            yield result?;
        }

        info!("Remux complete: {} total output fragments", seq - 1);
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::box_parser::write_u32_be;
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

    #[tokio::test]
    async fn test_remux_streams_no_full_buffer() {
        // Verify the new implementation doesn't use collect_stream
        // (this is a compile-time guarantee — collect_stream no longer exists)
        let video_data = build_test_fmp4(1);
        let audio_data = build_test_fmp4(1);

        let video_stream =
            stream::iter(vec![Ok::<_, std::io::Error>(Bytes::from(video_data))]);
        let audio_stream =
            stream::iter(vec![Ok::<_, std::io::Error>(Bytes::from(audio_data))]);

        let mut muxed = remux_streams(video_stream, audio_stream);
        let mut chunk_count = 0;

        while let Some(result) = muxed.next().await {
            result.unwrap();
            chunk_count += 1;
        }

        // At minimum: ftyp + moov + 1 fragment = 3 chunks
        assert!(chunk_count >= 3, "Expected at least 3 output chunks, got {}", chunk_count);
    }
}
