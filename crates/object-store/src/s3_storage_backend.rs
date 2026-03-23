use std::time::Duration;

use anyhow::{Context, Result};
use async_trait::async_trait;
use aws_sdk_s3::Client;

use crate::s3_multipart_upload;
use crate::{DownloadTicket, StorageBackend, StoredArtifact, UploadStream};

#[derive(Clone)]
pub struct S3StorageBackend {
    client: Client,
    bucket: String,
    part_size_bytes: usize,
}

impl S3StorageBackend {
    pub fn new(client: Client, bucket: String, part_size_bytes: usize) -> Self {
        Self {
            client,
            bucket,
            part_size_bytes: part_size_bytes.max(5 * 1024 * 1024),
        }
    }
}

#[async_trait]
impl StorageBackend for S3StorageBackend {
    fn backend_name(&self) -> &'static str {
        "s3"
    }

    async fn store_stream(
        &self,
        artifact_key: &str,
        content_type: &str,
        mut stream: UploadStream,
    ) -> Result<StoredArtifact> {
        let create = self
            .client
            .create_multipart_upload()
            .bucket(&self.bucket)
            .key(artifact_key)
            .content_type(content_type)
            .send()
            .await
            .with_context(|| format!("failed to create multipart upload for {artifact_key}"))?;
        let upload_id = create
            .upload_id()
            .context("multipart upload missing upload_id")?;

        let upload_result = self
            .upload_stream_parts(artifact_key, upload_id, &mut stream)
            .await;
        // Drop the upstream stream as soon as multipart ingestion finishes so any
        // attached proxy/download leases are released before S3 completion.
        drop(stream);

        let (completed_parts, total_bytes) = match upload_result {
            Ok(result) => result,
            Err(error) => return self.abort_after_error(artifact_key, upload_id, error).await,
        };

        if let Err(error) = self
            .complete_upload(artifact_key, upload_id, completed_parts)
            .await
        {
            return self.abort_after_error(artifact_key, upload_id, error).await;
        }

        Ok(StoredArtifact {
            backend: self.backend_name().to_string(),
            local_path: None,
            storage_bucket: Some(self.bucket.clone()),
            object_key: Some(artifact_key.to_string()),
            size_bytes: total_bytes,
            etag: None,
            content_type: content_type.to_string(),
        })
    }

    async fn presign_get(
        &self,
        artifact: &StoredArtifact,
        expires_secs: u64,
        content_disposition: Option<&str>,
    ) -> Result<DownloadTicket> {
        let key = artifact
            .object_key
            .as_deref()
            .context("missing object key for s3 presign")?;
        let config =
            aws_sdk_s3::presigning::PresigningConfig::expires_in(Duration::from_secs(expires_secs))
                .context("invalid presign expiration")?;
        let mut request = self.client.get_object().bucket(&self.bucket).key(key);
        if let Some(value) = content_disposition {
            request = request.response_content_disposition(value);
        }
        let request = request
            .presigned(config)
            .await
            .with_context(|| format!("failed to presign get for {key}"))?;

        Ok(DownloadTicket {
            url: request.uri().to_string(),
            expires_at_ms: crate_expires_at_ms(expires_secs),
        })
    }

    async fn delete(&self, artifact: &StoredArtifact) -> Result<()> {
        let key = artifact
            .object_key
            .as_deref()
            .context("missing object key for s3 delete")?;
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .with_context(|| format!("failed to delete object {key}"))?;
        Ok(())
    }
}

impl S3StorageBackend {
    async fn upload_stream_parts(
        &self,
        artifact_key: &str,
        upload_id: &str,
        stream: &mut UploadStream,
    ) -> Result<(Vec<aws_sdk_s3::types::CompletedPart>, i64)> {
        s3_multipart_upload::upload_stream_parts(
            &self.client,
            &self.bucket,
            artifact_key,
            upload_id,
            self.part_size_bytes,
            stream,
        )
        .await
    }

    async fn complete_upload(
        &self,
        artifact_key: &str,
        upload_id: &str,
        completed_parts: Vec<aws_sdk_s3::types::CompletedPart>,
    ) -> Result<()> {
        s3_multipart_upload::complete_upload(
            &self.client,
            &self.bucket,
            artifact_key,
            upload_id,
            completed_parts,
        )
        .await
    }

    async fn abort(&self, artifact_key: &str, upload_id: &str) -> Result<()> {
        s3_multipart_upload::abort_upload(&self.client, &self.bucket, artifact_key, upload_id).await
    }

    async fn abort_after_error<T>(
        &self,
        artifact_key: &str,
        upload_id: &str,
        error: anyhow::Error,
    ) -> Result<T> {
        if let Err(abort_error) = self.abort(artifact_key, upload_id).await {
            return Err(error.context(format!(
                "cleanup abort failed for {artifact_key}: {abort_error}"
            )));
        }

        Err(error)
    }
}

fn crate_expires_at_ms(expires_secs: u64) -> i64 {
    (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        + (expires_secs as u128 * 1000)) as i64
}
