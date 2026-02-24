//! Video-led fragment alignment for streaming dual-track muxing.
//!
//! Aligns video and audio fragments by timestamp, emitting merged
//! moof+mdat pairs using a sliding audio window. Only buffers
//! 2-5 audio fragments at a time — never the full stream.

use crate::fragment_stream::{Fragment, FragmentReader};
use crate::traf_merger::merge_fragments;
use crate::MuxerError;
use bytes::Bytes;
use futures::Stream;
use std::collections::VecDeque;
use tracing::warn;

/// Maximum audio fragments to buffer. Hard cap — oldest is dropped if exceeded.
const MAX_AUDIO_WINDOW: usize = 30;

/// Aligns video and audio fragments for streaming merge.
///
/// Uses a video-led strategy: for each video fragment, collects all audio
/// fragments that fall within its time window and merges them into a single
/// dual-traf moof+mdat pair.
///
/// Audio fragments are buffered in a small `VecDeque` window (typically 2-5
/// fragments). The video stream drives the pace.
pub struct FragmentAligner<V, A> {
    video_reader: FragmentReader<V>,
    audio_reader: FragmentReader<A>,
    /// Small buffer of upcoming audio fragments (typically 2-5).
    audio_window: VecDeque<Fragment>,
    audio_done: bool,
    /// Video timescale (for cross-multiply normalization).
    vts: u128,
    /// Audio timescale (for cross-multiply normalization).
    ats: u128,
}

impl<V, A> FragmentAligner<V, A>
where
    V: Stream<Item = Result<Bytes, MuxerError>> + Unpin,
    A: Stream<Item = Result<Bytes, MuxerError>> + Unpin,
{
    /// Create a new `FragmentAligner`.
    ///
    /// # Arguments
    /// - `video_stream` — remaining byte stream after video init segment
    /// - `audio_stream` — remaining byte stream after audio init segment
    /// - `video_timescale` — timescale from video moov (e.g., 90000)
    /// - `audio_timescale` — timescale from audio moov (e.g., 44100)
    pub fn new(
        video_stream: V,
        audio_stream: A,
        video_timescale: u32,
        audio_timescale: u32,
    ) -> Self {
        Self {
            video_reader: FragmentReader::new(video_stream),
            audio_reader: FragmentReader::new(audio_stream),
            audio_window: VecDeque::with_capacity(8),
            audio_done: false,
            vts: video_timescale as u128,
            ats: audio_timescale as u128,
        }
    }

    /// Pull the next merged fragment.
    ///
    /// Returns `None` when both streams are exhausted and all fragments have
    /// been emitted.
    ///
    /// Each call:
    /// 1. Reads the next video fragment
    /// 2. Peeks ahead for the *following* video fragment's timestamp
    /// 3. Collects audio fragments whose normalized timestamps fall before
    ///    the next video boundary
    /// 4. Merges them via `merge_fragments()` into a single dual-traf output
    /// 5. After video ends, flushes remaining audio fragments
    pub async fn next_merged(&mut self, seq: &mut u32) -> Option<Result<Bytes, MuxerError>> {
        // Pull next video fragment
        let v_frag = match self.video_reader.next_fragment().await {
            Some(Ok(f)) => {
                // Video track_id stays as-is (should be 1)
                Some(f)
            }
            Some(Err(e)) => return Some(Err(e)),
            None => None,
        };

        if let Some(v_frag) = v_frag {
            // Determine normalized timestamp of this video fragment
            let v_tfdt_norm = v_frag.tfdt as u128 * self.ats;

            // Pull audio until one exceeds current video boundary (greedy
            // alignment — works because YouTube DASH A/V are roughly aligned).
            if !self.audio_done {
                loop {
                    // Check if the last audio in window already exceeds current video position
                    if let Some(last) = self.audio_window.back() {
                        let last_norm = last.tfdt as u128 * self.vts;
                        // Keep pulling if last audio is still before or at current video
                        if last_norm > v_tfdt_norm {
                            break;
                        }
                    }

                    match self.audio_reader.next_fragment().await {
                        Some(Ok(mut a_frag)) => {
                            // Patch audio track_id to 2
                            let mut moof = a_frag.moof.to_vec();
                            if let Err(e) = patch_tfhd_track_id(&mut moof, 2) {
                                return Some(Err(e));
                            }
                            a_frag.moof = Bytes::from(moof);

                            let a_norm = a_frag.tfdt as u128 * self.vts;
                            self.audio_window.push_back(a_frag);

                            // Hard cap: drop oldest to prevent unbounded growth
                            if self.audio_window.len() > MAX_AUDIO_WINDOW {
                                warn!(
                                    "Audio window exceeded {} fragments (AV drift?), dropping oldest",
                                    MAX_AUDIO_WINDOW
                                );
                                self.audio_window.pop_front();
                            }

                            if a_norm > v_tfdt_norm {
                                break;
                            }
                        }
                        Some(Err(e)) => return Some(Err(e)),
                        None => {
                            self.audio_done = true;
                            break;
                        }
                    }
                }
            }

            // Drain audio fragments whose normalized tfdt <= v_tfdt_norm
            // (audio that belongs to this video fragment's time window)
            let drain_count = self
                .audio_window
                .iter()
                .take_while(|a| {
                    let a_norm = a.tfdt as u128 * self.vts;
                    a_norm <= v_tfdt_norm
                })
                .count();

            // We need references to the fragments for merge_fragments.
            // Drain them from the deque and collect.
            let drained: Vec<Fragment> = self.audio_window.drain(..drain_count).collect();

            let audio_pairs: Vec<(&[u8], &[u8])> = drained
                .iter()
                .map(|f| (f.moof.as_ref(), f.mdat.as_ref()))
                .collect();

            let merged = merge_fragments(
                v_frag.moof.as_ref(),
                v_frag.mdat.as_ref(),
                &audio_pairs,
                *seq,
            );
            *seq += 1;

            return Some(Ok(Bytes::from(merged)));
        }

        // Video stream exhausted — flush remaining audio fragments
        if let Some(a_frag) = self.audio_window.pop_front() {
            // Audio-only fragment: emit individually with patched sequence number
            let mut moof = a_frag.moof.to_vec();
            if let Err(e) = patch_mfhd_sequence(&mut moof, *seq) {
                return Some(Err(e));
            }
            *seq += 1;
            let mut out = moof;
            out.extend_from_slice(&a_frag.mdat);
            return Some(Ok(Bytes::from(out)));
        }

        // Drain remaining audio from reader
        if !self.audio_done {
            match self.audio_reader.next_fragment().await {
                Some(Ok(a_frag)) => {
                    let mut moof = a_frag.moof.to_vec();
                    if let Err(e) = patch_tfhd_track_id(&mut moof, 2) {
                        return Some(Err(e));
                    }
                    if let Err(e) = patch_mfhd_sequence(&mut moof, *seq) {
                        return Some(Err(e));
                    }
                    *seq += 1;
                    let mut out = moof;
                    out.extend_from_slice(&a_frag.mdat);
                    return Some(Ok(Bytes::from(out)));
                }
                Some(Err(e)) => return Some(Err(e)),
                None => {
                    self.audio_done = true;
                }
            }
        }

        None
    }
}

/// Patch `tfhd.track_id` inside a `moof` box (within the `traf` child).
///
/// tfhd layout: `[4B size][4B "tfhd"][4B ver+flags][4B track_id]`
fn patch_tfhd_track_id(moof: &mut [u8], track_id: u32) -> Result<(), MuxerError> {
    use crate::box_parser::{iter_boxes, read_box_header, write_u32_be};

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

/// Patch `mfhd.sequence_number` inside a `moof` box.
///
/// mfhd layout: `[4B size][4B "mfhd"][4B ver+flags][4B sequence_number]`
fn patch_mfhd_sequence(moof: &mut [u8], seq: u32) -> Result<(), MuxerError> {
    use crate::box_parser::{iter_boxes, read_box_header, write_u32_be};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::box_parser::write_u32_be;
    use bytes::Bytes;
    use futures::stream;

    /// Build a minimal moof with mfhd + traf{tfhd + tfdt}.
    fn build_test_moof(track_id: u32, decode_time: u32) -> Vec<u8> {
        let mut mfhd = vec![0u8; 16];
        mfhd[0..4].copy_from_slice(&16u32.to_be_bytes());
        mfhd[4..8].copy_from_slice(b"mfhd");
        write_u32_be(&mut mfhd, 12, 1);

        let mut tfhd = vec![0u8; 16];
        tfhd[0..4].copy_from_slice(&16u32.to_be_bytes());
        tfhd[4..8].copy_from_slice(b"tfhd");
        tfhd[8..12].copy_from_slice(&[0x00, 0x02, 0x00, 0x00]);
        write_u32_be(&mut tfhd, 12, track_id);

        let mut tfdt = vec![0u8; 16];
        tfdt[0..4].copy_from_slice(&16u32.to_be_bytes());
        tfdt[4..8].copy_from_slice(b"tfdt");
        tfdt[8..12].copy_from_slice(&[0u8; 4]); // version=0, flags=0
        write_u32_be(&mut tfdt, 12, decode_time);

        let traf_size = 8u32 + tfhd.len() as u32 + tfdt.len() as u32;
        let mut traf = Vec::with_capacity(traf_size as usize);
        traf.extend_from_slice(&traf_size.to_be_bytes());
        traf.extend_from_slice(b"traf");
        traf.extend_from_slice(&tfhd);
        traf.extend_from_slice(&tfdt);

        let moof_size = 8u32 + mfhd.len() as u32 + traf.len() as u32;
        let mut moof = Vec::with_capacity(moof_size as usize);
        moof.extend_from_slice(&moof_size.to_be_bytes());
        moof.extend_from_slice(b"moof");
        moof.extend_from_slice(&mfhd);
        moof.extend_from_slice(&traf);
        moof
    }

    fn build_mdat(payload: &[u8]) -> Vec<u8> {
        let size = 8 + payload.len() as u32;
        let mut mdat = Vec::with_capacity(size as usize);
        mdat.extend_from_slice(&size.to_be_bytes());
        mdat.extend_from_slice(b"mdat");
        mdat.extend_from_slice(payload);
        mdat
    }

    #[tokio::test]
    async fn test_align_2v_3a() {
        // Video: 2 fragments at decode_time 0 and 3000 (timescale 90000)
        // Audio: 3 fragments at decode_time 0, 1024, 2048 (timescale 44100)
        let mut video_data = Vec::new();
        video_data.extend(build_test_moof(1, 0));
        video_data.extend(build_mdat(&[0x01, 0x02]));
        video_data.extend(build_test_moof(1, 3000));
        video_data.extend(build_mdat(&[0x03, 0x04]));

        let mut audio_data = Vec::new();
        audio_data.extend(build_test_moof(1, 0));
        audio_data.extend(build_mdat(&[0xA1]));
        audio_data.extend(build_test_moof(1, 1024));
        audio_data.extend(build_mdat(&[0xA2]));
        audio_data.extend(build_test_moof(1, 2048));
        audio_data.extend(build_mdat(&[0xA3]));

        let video_stream = stream::iter(vec![Ok::<_, MuxerError>(Bytes::from(video_data))]);
        let audio_stream = stream::iter(vec![Ok::<_, MuxerError>(Bytes::from(audio_data))]);

        let mut aligner = FragmentAligner::new(video_stream, audio_stream, 90000, 44100);
        let mut seq = 1u32;
        let mut outputs = Vec::new();

        while let Some(result) = aligner.next_merged(&mut seq).await {
            outputs.push(result.unwrap());
        }

        // Should produce at least 2 merged fragments (one per video fragment)
        assert!(outputs.len() >= 2, "Expected at least 2 outputs, got {}", outputs.len());

        // Each output should contain moof
        for (i, output) in outputs.iter().enumerate() {
            assert!(
                output.windows(4).any(|w| w == b"moof"),
                "Output {} missing moof",
                i
            );
        }
    }

    #[tokio::test]
    async fn test_video_only_no_audio() {
        let mut video_data = Vec::new();
        video_data.extend(build_test_moof(1, 0));
        video_data.extend(build_mdat(&[0x01]));

        let video_stream = stream::iter(vec![Ok::<_, MuxerError>(Bytes::from(video_data))]);
        let audio_stream = stream::iter(vec![] as Vec<Result<Bytes, MuxerError>>);

        let mut aligner = FragmentAligner::new(video_stream, audio_stream, 90000, 44100);
        let mut seq = 1u32;

        let result = aligner.next_merged(&mut seq).await;
        assert!(result.is_some());
        let output = result.unwrap().unwrap();
        assert!(output.windows(4).any(|w| w == b"moof"));

        // No more fragments
        assert!(aligner.next_merged(&mut seq).await.is_none());
    }
}
