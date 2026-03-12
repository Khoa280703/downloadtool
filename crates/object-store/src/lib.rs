pub mod backend_factory;
pub mod local_fs_storage_backend;
mod s3_multipart_upload;
pub mod s3_storage_backend;

use std::pin::Pin;

use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;

pub type UploadStream = Pin<Box<dyn Stream<Item = Result<Bytes>> + Send>>;

#[derive(Clone, Debug)]
pub struct StoredArtifact {
    pub backend: String,
    pub local_path: Option<String>,
    pub storage_bucket: Option<String>,
    pub object_key: Option<String>,
    pub size_bytes: i64,
    pub etag: Option<String>,
    pub content_type: String,
}

#[derive(Clone, Debug)]
pub struct DownloadTicket {
    pub url: String,
    pub expires_at_ms: i64,
}

#[async_trait]
pub trait StorageBackend: Send + Sync {
    fn backend_name(&self) -> &'static str;
    async fn store_stream(
        &self,
        artifact_key: &str,
        content_type: &str,
        stream: UploadStream,
    ) -> Result<StoredArtifact>;
    async fn presign_get(
        &self,
        artifact: &StoredArtifact,
        expires_secs: u64,
        content_disposition: Option<&str>,
    ) -> Result<DownloadTicket>;
    async fn delete(&self, artifact: &StoredArtifact) -> Result<()>;
}
