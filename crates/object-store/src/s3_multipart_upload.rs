use std::time::Instant;

use anyhow::{anyhow, Context, Result};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart};
use aws_sdk_s3::Client;
use futures::StreamExt;
use tracing::{info, warn};

use crate::UploadStream;

pub(crate) async fn upload_stream_parts(
    client: &Client,
    bucket: &str,
    artifact_key: &str,
    upload_id: &str,
    part_size_bytes: usize,
    stream: &mut UploadStream,
) -> Result<(Vec<CompletedPart>, i64)> {
    let mut completed_parts = Vec::new();
    let mut next_part_number = 1i32;
    let mut buffer = MultipartUploadBuffer::new(part_size_bytes);
    let started_at = Instant::now();

    info!(
        bucket,
        artifact_key, upload_id, part_size_bytes, "Starting multipart upload stream ingestion"
    );

    while let Some(chunk) = stream.next().await {
        buffer.push(&chunk?)?;
        while let Some(part) = buffer.take_ready_part() {
            let part_len = part.len();
            completed_parts.push(
                upload_part(
                    client,
                    bucket,
                    artifact_key,
                    upload_id,
                    next_part_number,
                    &part,
                )
                .await?,
            );
            info!(
                bucket,
                artifact_key,
                upload_id,
                part_number = next_part_number,
                part_size_bytes = part_len,
                uploaded_parts = completed_parts.len(),
                buffered_total_bytes = buffer.total_bytes,
                elapsed_ms = started_at.elapsed().as_millis() as u64,
                "Uploaded multipart part"
            );
            next_part_number += 1;
        }
    }

    let (final_part, total_bytes) = buffer.finish()?;
    if let Some(part) = final_part {
        let part_len = part.len();
        completed_parts.push(
            upload_part(
                client,
                bucket,
                artifact_key,
                upload_id,
                next_part_number,
                &part,
            )
            .await?,
        );
        info!(
            bucket,
            artifact_key,
            upload_id,
            part_number = next_part_number,
            part_size_bytes = part_len,
            uploaded_parts = completed_parts.len(),
            total_bytes,
            elapsed_ms = started_at.elapsed().as_millis() as u64,
            "Uploaded final multipart part"
        );
    }

    info!(
        bucket,
        artifact_key,
        upload_id,
        total_bytes,
        total_parts = completed_parts.len(),
        elapsed_ms = started_at.elapsed().as_millis() as u64,
        "Finished multipart upload stream ingestion"
    );

    Ok((completed_parts, total_bytes))
}

pub(crate) async fn complete_upload(
    client: &Client,
    bucket: &str,
    artifact_key: &str,
    upload_id: &str,
    completed_parts: Vec<CompletedPart>,
) -> Result<()> {
    let started_at = Instant::now();
    let parts_len = completed_parts.len();
    client
        .complete_multipart_upload()
        .bucket(bucket)
        .key(artifact_key)
        .upload_id(upload_id)
        .multipart_upload(
            CompletedMultipartUpload::builder()
                .set_parts(Some(completed_parts))
                .build(),
        )
        .send()
        .await
        .with_context(|| format!("failed to complete multipart upload for {artifact_key}"))?;

    info!(
        bucket,
        artifact_key,
        upload_id,
        parts = parts_len,
        elapsed_ms = started_at.elapsed().as_millis() as u64,
        "Completed multipart upload"
    );

    Ok(())
}

pub(crate) async fn abort_upload(
    client: &Client,
    bucket: &str,
    artifact_key: &str,
    upload_id: &str,
) -> Result<()> {
    let started_at = Instant::now();
    client
        .abort_multipart_upload()
        .bucket(bucket)
        .key(artifact_key)
        .upload_id(upload_id)
        .send()
        .await
        .with_context(|| format!("failed to abort multipart upload for {artifact_key}"))?;

    warn!(
        bucket,
        artifact_key,
        upload_id,
        elapsed_ms = started_at.elapsed().as_millis() as u64,
        "Aborted multipart upload"
    );

    Ok(())
}

async fn upload_part(
    client: &Client,
    bucket: &str,
    artifact_key: &str,
    upload_id: &str,
    part_number: i32,
    bytes: &[u8],
) -> Result<CompletedPart> {
    let started_at = Instant::now();
    let response = client
        .upload_part()
        .bucket(bucket)
        .key(artifact_key)
        .upload_id(upload_id)
        .part_number(part_number)
        .body(ByteStream::from(bytes.to_vec()))
        .send()
        .await
        .with_context(|| format!("failed to upload part {part_number} for {artifact_key}"))?;

    info!(
        bucket,
        artifact_key,
        upload_id,
        part_number,
        part_size_bytes = bytes.len(),
        elapsed_ms = started_at.elapsed().as_millis() as u64,
        "upload_part request finished"
    );

    Ok(CompletedPart::builder()
        .part_number(part_number)
        .set_e_tag(response.e_tag().map(ToString::to_string))
        .build())
}

#[derive(Debug)]
struct MultipartUploadBuffer {
    part_size_bytes: usize,
    total_bytes: i64,
    buffer: Vec<u8>,
}

impl MultipartUploadBuffer {
    fn new(part_size_bytes: usize) -> Self {
        Self {
            part_size_bytes,
            total_bytes: 0,
            buffer: Vec::with_capacity(part_size_bytes),
        }
    }

    fn push(&mut self, chunk: &[u8]) -> Result<()> {
        self.total_bytes += i64::try_from(chunk.len()).context("chunk length exceeds i64")?;
        self.buffer.extend_from_slice(chunk);
        Ok(())
    }

    fn take_ready_part(&mut self) -> Option<Vec<u8>> {
        if self.buffer.len() < self.part_size_bytes {
            return None;
        }

        let remainder = self.buffer.split_off(self.part_size_bytes);
        Some(std::mem::replace(&mut self.buffer, remainder))
    }

    fn finish(self) -> Result<(Option<Vec<u8>>, i64)> {
        if self.total_bytes <= 0 {
            return Err(anyhow!("multipart upload produced empty output"));
        }

        if self.buffer.is_empty() {
            Ok((None, self.total_bytes))
        } else {
            Ok((Some(self.buffer), self.total_bytes))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MultipartUploadBuffer;

    #[test]
    fn keeps_fixed_size_parts_and_overflow() {
        let mut buffer = MultipartUploadBuffer::new(8);
        buffer.push(&[1; 20]).expect("push");

        let first = buffer.take_ready_part().expect("first part");
        let second = buffer.take_ready_part().expect("second part");
        let (tail, total_bytes) = buffer.finish().expect("finish");

        assert_eq!(first.len(), 8);
        assert_eq!(second.len(), 8);
        assert_eq!(tail.expect("tail").len(), 4);
        assert_eq!(total_bytes, 20);
    }

    #[test]
    fn returns_empty_error_when_no_bytes_were_uploaded() {
        let buffer = MultipartUploadBuffer::new(8);
        let error = buffer.finish().expect_err("empty buffer must fail");
        assert!(error.to_string().contains("empty output"));
    }
}
