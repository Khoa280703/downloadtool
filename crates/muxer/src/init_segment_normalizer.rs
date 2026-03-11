use crate::box_parser::{iter_boxes, read_box_header, write_u32_be};
use crate::MuxerError;

pub fn normalize_fragmented_mp4_moov(moov: &[u8]) -> Result<Vec<u8>, MuxerError> {
    let moov_header =
        read_box_header(moov).ok_or_else(|| MuxerError::InvalidInput("Invalid moov box".into()))?;
    if &moov_header.box_type != b"moov" {
        return Err(MuxerError::InvalidInput("Expected moov box".into()));
    }

    let mut patched = moov.to_vec();
    patch_mvhd_duration(&mut patched, 0)?;
    patch_all_trak_mdhd_durations(&mut patched, 0)?;
    Ok(patched)
}

fn patch_mvhd_duration(moov: &mut [u8], duration: u32) -> Result<(), MuxerError> {
    let moov_header = read_box_header(moov)
        .ok_or_else(|| MuxerError::InvalidInput("Invalid moov for mvhd patch".into()))?;
    let moov_content_start = moov_header.header_size as usize;
    let moov_content = &moov[moov_content_start..];
    let (mvhd_off, mvhd_header) = iter_boxes(moov_content)
        .find(|(_, header)| &header.box_type == b"mvhd")
        .ok_or_else(|| MuxerError::InvalidInput("No mvhd in moov".into()))?;

    let mvhd_abs = moov_content_start + mvhd_off;
    let mvhd_content_start = mvhd_abs + mvhd_header.header_size as usize;
    if moov.len() < mvhd_content_start + 1 {
        return Err(MuxerError::InvalidInput("mvhd too short".into()));
    }

    let version = moov[mvhd_content_start];
    if version == 0 {
        let duration_offset = mvhd_content_start + 16;
        if moov.len() < duration_offset + 4 {
            return Err(MuxerError::InvalidInput("mvhd v0 too short".into()));
        }
        write_u32_be(moov, duration_offset, duration);
    } else {
        let duration_offset = mvhd_content_start + 24;
        if moov.len() < duration_offset + 8 {
            return Err(MuxerError::InvalidInput("mvhd v1 too short".into()));
        }
        write_u32_be(moov, duration_offset, 0);
        write_u32_be(moov, duration_offset + 4, duration);
    }

    Ok(())
}

fn patch_all_trak_mdhd_durations(moov: &mut [u8], duration: u32) -> Result<(), MuxerError> {
    let moov_header = read_box_header(moov)
        .ok_or_else(|| MuxerError::InvalidInput("Invalid moov for trak patch".into()))?;
    let moov_content_start = moov_header.header_size as usize;
    let moov_content_end = (moov_header.total_size as usize).min(moov.len());
    let moov_content = &moov[moov_content_start..moov_content_end];

    let trak_offsets: Vec<usize> = iter_boxes(moov_content)
        .filter_map(|(offset, header)| (&header.box_type == b"trak").then_some(offset))
        .collect();

    if trak_offsets.is_empty() {
        return Err(MuxerError::InvalidInput("No trak in moov".into()));
    }

    for trak_off in trak_offsets {
        patch_trak_mdhd_duration(&mut moov[moov_content_start + trak_off..], duration)?;
    }

    Ok(())
}

fn patch_trak_mdhd_duration(trak: &mut [u8], duration: u32) -> Result<(), MuxerError> {
    let trak_header = read_box_header(trak)
        .ok_or_else(|| MuxerError::InvalidInput("Invalid trak for mdhd patch".into()))?;
    let trak_content_start = trak_header.header_size as usize;
    let trak_content_end = (trak_header.total_size as usize).min(trak.len());
    let trak_content = &trak[trak_content_start..trak_content_end];

    let (mdia_off, mdia_header) = iter_boxes(trak_content)
        .find(|(_, header)| &header.box_type == b"mdia")
        .ok_or_else(|| MuxerError::InvalidInput("No mdia in trak".into()))?;
    let mdia_abs = trak_content_start + mdia_off;
    let mdia_content_start = mdia_abs + mdia_header.header_size as usize;
    let mdia_end = (mdia_abs + mdia_header.total_size as usize).min(trak.len());
    let mdia_content = &trak[mdia_content_start..mdia_end];

    let (mdhd_off, mdhd_header) = iter_boxes(mdia_content)
        .find(|(_, header)| &header.box_type == b"mdhd")
        .ok_or_else(|| MuxerError::InvalidInput("No mdhd in mdia".into()))?;
    let mdhd_abs = mdia_content_start + mdhd_off;
    let mdhd_content_start = mdhd_abs + mdhd_header.header_size as usize;
    if trak.len() < mdhd_content_start + 1 {
        return Err(MuxerError::InvalidInput("mdhd too short".into()));
    }

    let version = trak[mdhd_content_start];
    if version == 0 {
        let duration_offset = mdhd_content_start + 16;
        if trak.len() < duration_offset + 4 {
            return Err(MuxerError::InvalidInput("mdhd v0 too short".into()));
        }
        write_u32_be(trak, duration_offset, duration);
    } else {
        let duration_offset = mdhd_content_start + 24;
        if trak.len() < duration_offset + 8 {
            return Err(MuxerError::InvalidInput("mdhd v1 too short".into()));
        }
        write_u32_be(trak, duration_offset, 0);
        write_u32_be(trak, duration_offset + 4, duration);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::box_parser::read_u32_be;

    fn make_box(box_type: &[u8; 4], payload: &[u8]) -> Vec<u8> {
        let size = 8 + payload.len() as u32;
        let mut bytes = Vec::with_capacity(size as usize);
        bytes.extend_from_slice(&size.to_be_bytes());
        bytes.extend_from_slice(box_type);
        bytes.extend_from_slice(payload);
        bytes
    }

    fn build_moov_with_durations(mvhd_duration: u32, mdhd_duration: u32) -> Vec<u8> {
        let mut mvhd_payload = vec![0u8; 100];
        mvhd_payload[0] = 0;
        mvhd_payload[16..20].copy_from_slice(&mvhd_duration.to_be_bytes());
        let mvhd = make_box(b"mvhd", &mvhd_payload);

        let mut mdhd_payload = vec![0u8; 24];
        mdhd_payload[0] = 0;
        mdhd_payload[12..16].copy_from_slice(&48_000u32.to_be_bytes());
        mdhd_payload[16..20].copy_from_slice(&mdhd_duration.to_be_bytes());
        let mdhd = make_box(b"mdhd", &mdhd_payload);
        let mdia = make_box(b"mdia", &mdhd);
        let trak = make_box(b"trak", &mdia);

        let mut moov_payload = Vec::new();
        moov_payload.extend_from_slice(&mvhd);
        moov_payload.extend_from_slice(&trak);
        make_box(b"moov", &moov_payload)
    }

    #[test]
    fn normalize_fragmented_mp4_moov_zeros_mvhd_and_mdhd_duration() {
        let moov = build_moov_with_durations(1_000, 2_000);
        let patched = normalize_fragmented_mp4_moov(&moov).unwrap();

        assert_eq!(read_u32_be(&patched, 28), 0);
        assert_eq!(read_u32_be(&patched, 68), 0);
    }
}
