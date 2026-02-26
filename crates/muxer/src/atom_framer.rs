//! MP4 atom-level streaming reader.
//!
//! Reads one complete box at a time from an async byte stream.
//! Used during the init phase to extract `ftyp` and `moov` boxes
//! before handing off the remainder to `FragmentReader`.

use crate::MuxerError;
use bytes::{BufMut, Bytes, BytesMut};
use futures::{Stream, StreamExt};

/// Maximum allowed size for non-mdat boxes (256 MB).
/// Prevents unbounded memory allocation from malformed input.
const MAX_NON_MDAT_BOX_SIZE: u64 = 256 * 1024 * 1024;

/// Reads MP4 boxes one at a time from an async byte stream.
///
/// Only buffers one complete box at a time in its internal `BytesMut`.
/// After collecting the init segment, call `into_remaining_stream()`
/// to get a stream of the unconsumed bytes chained with the original stream.
pub struct AtomFramer<S> {
    stream: S,
    buf: BytesMut,
    done: bool,
}

impl<S, E> AtomFramer<S>
where
    E: std::error::Error + Send + Sync + 'static,
    S: Stream<Item = Result<Bytes, E>> + Unpin,
{
    /// Create a new `AtomFramer` wrapping an async byte stream.
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            buf: BytesMut::with_capacity(64 * 1024),
            done: false,
        }
    }

    /// Fill the internal buffer to at least `n` bytes.
    /// Returns `false` if the stream ends before reaching `n` bytes.
    async fn fill_to(&mut self, n: usize) -> Result<bool, MuxerError> {
        while self.buf.len() < n {
            match self.stream.next().await {
                Some(Ok(chunk)) => self.buf.put(chunk),
                Some(Err(e)) => return Err(MuxerError::StreamFetchError(e.to_string())),
                None => return Ok(false),
            }
        }
        Ok(true)
    }

    /// Read the next complete box from the stream.
    ///
    /// Returns `(box_type, full_box_bytes)` or `None` if the stream is exhausted.
    ///
    /// Handles extended size: when the 32-bit size field is 1, the next 8 bytes
    /// are read as a 64-bit total size.
    pub async fn read_box(&mut self) -> Option<Result<([u8; 4], Bytes), MuxerError>> {
        if self.done {
            return None;
        }

        // Need at least 8 bytes for a standard box header
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

        let size32 = u32::from_be_bytes(self.buf[0..4].try_into().unwrap());
        let box_type: [u8; 4] = self.buf[4..8].try_into().unwrap();

        let (total_size, header_size) = if size32 == 1 {
            // Extended size: next 8 bytes are the real u64 size
            match self.fill_to(16).await {
                Ok(false) => {
                    self.done = true;
                    return Some(Err(MuxerError::InvalidInput(
                        "Stream ended inside extended box header".into(),
                    )));
                }
                Err(e) => {
                    self.done = true;
                    return Some(Err(e));
                }
                Ok(true) => {}
            }
            let ext_size = u64::from_be_bytes(self.buf[8..16].try_into().unwrap());
            (ext_size, 16usize)
        } else if size32 == 0 {
            // size==0 means "box extends to end of file" — not expected in streaming
            self.done = true;
            return Some(Err(MuxerError::InvalidInput(
                "Box with size=0 (extends to EOF) not supported in streaming".into(),
            )));
        } else {
            (size32 as u64, 8usize)
        };

        if total_size < header_size as u64 {
            self.done = true;
            return Some(Err(MuxerError::InvalidInput(format!(
                "Box size {} smaller than header {}",
                total_size, header_size
            ))));
        }

        // Sanity check: non-mdat boxes shouldn't be enormous
        if &box_type != b"mdat" && total_size > MAX_NON_MDAT_BOX_SIZE {
            self.done = true;
            return Some(Err(MuxerError::InvalidInput(format!(
                "Box {:?} size {} exceeds 256MB safety limit",
                std::str::from_utf8(&box_type).unwrap_or("????"),
                total_size
            ))));
        }

        let total = total_size as usize;
        match self.fill_to(total).await {
            Ok(false) => {
                self.done = true;
                return Some(Err(MuxerError::InvalidInput(format!(
                    "Stream ended inside box {:?} (need {} bytes, have {})",
                    std::str::from_utf8(&box_type).unwrap_or("????"),
                    total,
                    self.buf.len()
                ))));
            }
            Err(e) => {
                self.done = true;
                return Some(Err(e));
            }
            Ok(true) => {}
        }

        let box_bytes = self.buf.split_to(total).freeze();
        Some(Ok((box_type, box_bytes)))
    }

    /// Collect `ftyp` and `moov` boxes from the stream, skipping others (e.g. `styp`, `sidx`).
    ///
    /// Returns `(ftyp_bytes, moov_bytes)`.
    pub async fn collect_init_segment(&mut self) -> Result<(Bytes, Bytes), MuxerError> {
        let mut ftyp: Option<Bytes> = None;
        let mut moov: Option<Bytes> = None;

        while ftyp.is_none() || moov.is_none() {
            match self.read_box().await {
                None => {
                    return Err(MuxerError::InvalidInput(
                        "Stream ended before init segment complete (need ftyp + moov)".into(),
                    ))
                }
                Some(Err(e)) => return Err(e),
                Some(Ok((box_type, data))) => match &box_type {
                    b"ftyp" => ftyp = Some(data),
                    b"moov" => moov = Some(data),
                    _ => {} // skip styp, sidx, free, etc.
                },
            }
        }

        Ok((ftyp.unwrap(), moov.unwrap()))
    }

    /// Consume this `AtomFramer` and return a stream that yields any
    /// unconsumed bytes from the internal buffer, followed by the
    /// remainder of the original stream.
    ///
    /// This is used to hand off from init-segment collection to
    /// `FragmentReader` without losing bytes already buffered.
    pub fn into_remaining_stream(
        self,
    ) -> impl Stream<Item = Result<Bytes, MuxerError>> + Unpin + Send
    where
        S: Send + 'static,
        E: 'static,
    {
        let remainder = if self.buf.is_empty() {
            None
        } else {
            Some(Ok(self.buf.freeze()))
        };

        let mapped_stream = self
            .stream
            .map(|r| r.map_err(|e| MuxerError::StreamFetchError(e.to_string())));

        // Chain: first yield leftover buffer, then continue with original stream
        Box::pin(futures::stream::iter(remainder.into_iter()).chain(mapped_stream))
            as std::pin::Pin<Box<dyn Stream<Item = Result<Bytes, MuxerError>> + Send>>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::stream;

    /// Build a minimal MP4 box with the given type and payload.
    fn make_box(box_type: &[u8; 4], payload: &[u8]) -> Vec<u8> {
        let size = 8 + payload.len() as u32;
        let mut data = Vec::with_capacity(size as usize);
        data.extend_from_slice(&size.to_be_bytes());
        data.extend_from_slice(box_type);
        data.extend_from_slice(payload);
        data
    }

    /// Build an extended-size box (size field == 1, followed by 8-byte u64 size).
    fn make_extended_box(box_type: &[u8; 4], payload: &[u8]) -> Vec<u8> {
        let total_size: u64 = 16 + payload.len() as u64;
        let mut data = Vec::with_capacity(total_size as usize);
        data.extend_from_slice(&1u32.to_be_bytes()); // size == 1 signals extended
        data.extend_from_slice(box_type);
        data.extend_from_slice(&total_size.to_be_bytes());
        data.extend_from_slice(payload);
        data
    }

    #[tokio::test]
    async fn test_read_box_normal() {
        let data = make_box(b"ftyp", &[0x01, 0x02, 0x03, 0x04]);
        let s = stream::iter(vec![Ok::<_, std::io::Error>(Bytes::from(data))]);
        let mut framer = AtomFramer::new(s);

        let (box_type, box_bytes) = framer.read_box().await.unwrap().unwrap();
        assert_eq!(&box_type, b"ftyp");
        assert_eq!(box_bytes.len(), 12); // 8 header + 4 payload
        assert_eq!(&box_bytes[8..], &[0x01, 0x02, 0x03, 0x04]);

        // Stream should be exhausted
        assert!(framer.read_box().await.is_none());
    }

    #[tokio::test]
    async fn test_read_box_extended_size() {
        let data = make_extended_box(b"moof", &[0xAA, 0xBB]);
        let s = stream::iter(vec![Ok::<_, std::io::Error>(Bytes::from(data))]);
        let mut framer = AtomFramer::new(s);

        let (box_type, box_bytes) = framer.read_box().await.unwrap().unwrap();
        assert_eq!(&box_type, b"moof");
        assert_eq!(box_bytes.len(), 18); // 16 header + 2 payload
    }

    #[tokio::test]
    async fn test_collect_init_segment_skips_styp() {
        let mut data = Vec::new();
        data.extend(make_box(b"styp", &[0x00; 4])); // should be skipped
        data.extend(make_box(b"ftyp", b"isom\x00\x00\x00\x00isom"));
        data.extend(make_box(b"sidx", &[0x00; 8])); // should be skipped
        data.extend(make_box(b"moov", &[0x00; 20]));
        data.extend(make_box(b"moof", &[0x00; 16])); // leftover

        let s = stream::iter(vec![Ok::<_, std::io::Error>(Bytes::from(data))]);
        let mut framer = AtomFramer::new(s);

        let (ftyp, moov) = framer.collect_init_segment().await.unwrap();
        assert_eq!(&ftyp[4..8], b"ftyp");
        assert_eq!(&moov[4..8], b"moov");
    }

    #[tokio::test]
    async fn test_into_remaining_stream() {
        // Build: ftyp + moov + moof + mdat
        let mut data = Vec::new();
        data.extend(make_box(b"ftyp", &[0x01; 4]));
        data.extend(make_box(b"moov", &[0x02; 8]));
        let moof_bytes = make_box(b"moof", &[0x03; 12]);
        let mdat_bytes = make_box(b"mdat", &[0x04; 6]);
        data.extend(&moof_bytes);
        data.extend(&mdat_bytes);

        let s = stream::iter(vec![Ok::<_, std::io::Error>(Bytes::from(data))]);
        let mut framer = AtomFramer::new(s);

        // Consume init segment
        let (_ftyp, _moov) = framer.collect_init_segment().await.unwrap();

        // Get remaining stream and collect all bytes
        let remaining = framer.into_remaining_stream();
        futures::pin_mut!(remaining);

        let mut leftover = Vec::new();
        while let Some(chunk) = remaining.next().await {
            leftover.extend_from_slice(&chunk.unwrap());
        }

        // Remaining should start with the moof box
        assert!(leftover.len() >= 8, "leftover should have data");
        assert_eq!(&leftover[4..8], b"moof");
        // And contain the mdat too
        let expected_len = moof_bytes.len() + mdat_bytes.len();
        assert_eq!(leftover.len(), expected_len);
    }

    #[tokio::test]
    async fn test_chunked_delivery() {
        // Test that AtomFramer works when bytes arrive in small chunks
        let box_data = make_box(b"ftyp", &[0x01, 0x02, 0x03, 0x04]);
        // Split into 3-byte chunks
        let chunks: Vec<Result<Bytes, std::io::Error>> = box_data
            .chunks(3)
            .map(|c| Ok(Bytes::copy_from_slice(c)))
            .collect();
        let s = stream::iter(chunks);
        let mut framer = AtomFramer::new(s);

        let (box_type, box_bytes) = framer.read_box().await.unwrap().unwrap();
        assert_eq!(&box_type, b"ftyp");
        assert_eq!(box_bytes.len(), 12);
    }
}
