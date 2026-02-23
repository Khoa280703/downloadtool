//! Streaming fMP4 fragment reader.
//!
//! Reads moof+mdat pairs one at a time from an async byte stream.
//! Only buffers one complete fragment at a time — never the full video.

use crate::box_parser::{read_box_header, read_tfdt};
use crate::MuxerError;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use futures::{Stream, StreamExt};

/// A single fMP4 fragment: moof box + mdat box bytes.
pub struct Fragment {
    /// Raw moof box bytes (fully buffered, typically small ~1KB).
    pub moof: Bytes,
    /// Raw mdat box bytes (media payload).
    pub mdat: Bytes,
    /// `base_media_decode_time` parsed from the tfdt box inside moof.
    pub tfdt: u64,
}

/// Reads fMP4 fragments one at a time from an async byte stream.
///
/// Buffers at most one complete moof+mdat pair at a time.
/// The underlying stream may yield bytes in arbitrary chunk sizes.
pub struct FragmentReader<S> {
    stream: S,
    buffer: BytesMut,
    done: bool,
}

impl<S, E> FragmentReader<S>
where
    E: std::error::Error + Send + Sync + 'static,
    S: Stream<Item = Result<Bytes, E>> + Unpin,
{
    /// Create a new reader wrapping the given byte stream.
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            buffer: BytesMut::with_capacity(256 * 1024),
            done: false,
        }
    }

    /// Read bytes from stream until buffer has at least `needed` bytes.
    /// Returns `false` if stream ended before filling the buffer.
    async fn fill_to(&mut self, needed: usize) -> Result<bool, MuxerError> {
        while self.buffer.len() < needed {
            match self.stream.next().await {
                Some(Ok(chunk)) => self.buffer.put(chunk),
                Some(Err(e)) => return Err(MuxerError::StreamFetchError(e.to_string())),
                None => return Ok(false),
            }
        }
        Ok(true)
    }

    /// Return the next complete Fragment (moof + mdat pair), or `None` if stream ended.
    pub async fn next_fragment(&mut self) -> Option<Result<Fragment, MuxerError>> {
        if self.done {
            return None;
        }

        // Skip any non-moof boxes that may appear (e.g. sidx, styp)
        loop {
            // Need at least 8 bytes to read a box header
            match self.fill_to(8).await {
                Ok(false) => {
                    self.done = true;
                    return None;
                }
                Err(e) => {
                    self.done = true;
                    return Some(Err(e));
                }
                Ok(true) => {}
            }

            let box_type = &self.buffer[4..8];
            if box_type == b"moof" {
                break;
            }

            // Skip non-moof box: read its full size then drain it
            let hdr = match read_box_header(&self.buffer) {
                Some(h) => h,
                None => {
                    self.done = true;
                    return Some(Err(MuxerError::InvalidInput("Malformed box header".into())));
                }
            };
            let skip = hdr.total_size as usize;
            if skip < 8 {
                self.done = true;
                return Some(Err(MuxerError::InvalidInput("Box size too small".into())));
            }
            match self.fill_to(skip).await {
                Ok(false) => {
                    self.done = true;
                    return None;
                }
                Err(e) => {
                    self.done = true;
                    return Some(Err(e));
                }
                Ok(true) => {}
            }
            self.buffer.advance(skip);
        }

        // We are positioned at a moof box. Read its declared size.
        let moof_size = u32::from_be_bytes(
            self.buffer[0..4].try_into().expect("buffer has ≥8 bytes"),
        ) as usize;

        if moof_size < 8 {
            self.done = true;
            return Some(Err(MuxerError::InvalidInput("moof size too small".into())));
        }

        match self.fill_to(moof_size).await {
            Ok(false) => {
                self.done = true;
                return Some(Err(MuxerError::InvalidInput("Stream ended inside moof".into())));
            }
            Err(e) => {
                self.done = true;
                return Some(Err(e));
            }
            Ok(true) => {}
        }
        let moof_bytes = self.buffer.split_to(moof_size).freeze();
        let tfdt = read_tfdt(&moof_bytes).unwrap_or(0);

        // Expect mdat to follow immediately after moof
        match self.fill_to(8).await {
            Ok(false) => {
                self.done = true;
                return Some(Err(MuxerError::InvalidInput("Stream ended after moof".into())));
            }
            Err(e) => {
                self.done = true;
                return Some(Err(e));
            }
            Ok(true) => {}
        }

        if &self.buffer[4..8] != b"mdat" {
            self.done = true;
            return Some(Err(MuxerError::InvalidInput(
                "Expected mdat box after moof".into(),
            )));
        }

        let mdat_size = u32::from_be_bytes(
            self.buffer[0..4].try_into().expect("buffer has ≥8 bytes"),
        ) as usize;

        if mdat_size < 8 {
            self.done = true;
            return Some(Err(MuxerError::InvalidInput("mdat size too small".into())));
        }

        match self.fill_to(mdat_size).await {
            Ok(false) => {
                self.done = true;
                return Some(Err(MuxerError::InvalidInput("Truncated mdat".into())));
            }
            Err(e) => {
                self.done = true;
                return Some(Err(e));
            }
            Ok(true) => {}
        }
        let mdat_bytes = self.buffer.split_to(mdat_size).freeze();

        Some(Ok(Fragment {
            moof: moof_bytes,
            mdat: mdat_bytes,
            tfdt,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::box_parser::write_u32_be;
    use futures::stream;

    /// Build a minimal moof box with mfhd + traf{tfhd + tfdt}.
    fn build_test_moof(track_id: u32, decode_time: u32) -> Vec<u8> {
        // mfhd: [16][mfhd][ver+flags=0][seq=1]
        let mut mfhd = vec![0u8; 16];
        mfhd[0..4].copy_from_slice(&16u32.to_be_bytes());
        mfhd[4..8].copy_from_slice(b"mfhd");
        write_u32_be(&mut mfhd, 12, 1);

        // tfhd: [16][tfhd][flags][track_id]
        let mut tfhd = vec![0u8; 16];
        tfhd[0..4].copy_from_slice(&16u32.to_be_bytes());
        tfhd[4..8].copy_from_slice(b"tfhd");
        write_u32_be(&mut tfhd, 12, track_id);

        // tfdt v0: [16][tfdt][ver=0][flags=0][decode_time u32]
        let mut tfdt = vec![0u8; 16];
        tfdt[0..4].copy_from_slice(&16u32.to_be_bytes());
        tfdt[4..8].copy_from_slice(b"tfdt");
        // version=0, flags=0
        tfdt[8..12].copy_from_slice(&[0u8; 4]);
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
    async fn test_read_single_fragment() {
        let mut data = build_test_moof(1, 12345);
        data.extend(build_mdat(&[0xAA, 0xBB, 0xCC]));

        let stream = stream::iter(vec![Ok::<_, std::io::Error>(Bytes::from(data))]);
        let mut reader = FragmentReader::new(stream);

        let frag = reader.next_fragment().await.unwrap().unwrap();
        assert_eq!(frag.tfdt, 12345);
        assert_eq!(&frag.moof[4..8], b"moof");
        assert_eq!(&frag.mdat[4..8], b"mdat");

        // Stream should be exhausted
        assert!(reader.next_fragment().await.is_none());
    }

    #[tokio::test]
    async fn test_read_two_fragments() {
        let mut data = build_test_moof(1, 0);
        data.extend(build_mdat(&[0x01]));
        data.extend(build_test_moof(1, 9000));
        data.extend(build_mdat(&[0x02]));

        let stream = stream::iter(vec![Ok::<_, std::io::Error>(Bytes::from(data))]);
        let mut reader = FragmentReader::new(stream);

        let f1 = reader.next_fragment().await.unwrap().unwrap();
        assert_eq!(f1.tfdt, 0);

        let f2 = reader.next_fragment().await.unwrap().unwrap();
        assert_eq!(f2.tfdt, 9000);

        assert!(reader.next_fragment().await.is_none());
    }
}
