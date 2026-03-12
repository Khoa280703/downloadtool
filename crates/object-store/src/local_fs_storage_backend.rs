use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use futures::StreamExt;
use tokio::io::AsyncWriteExt;

use crate::{DownloadTicket, StorageBackend, StoredArtifact, UploadStream};

#[derive(Clone)]
pub struct LocalFsStorageBackend {
    base_dir: PathBuf,
}

impl LocalFsStorageBackend {
    pub fn new(base_dir: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&base_dir).with_context(|| {
            format!(
                "failed to create local artifact directory {}",
                base_dir.display()
            )
        })?;
        Ok(Self { base_dir })
    }
}

#[async_trait]
impl StorageBackend for LocalFsStorageBackend {
    fn backend_name(&self) -> &'static str {
        "localfs"
    }

    async fn store_stream(
        &self,
        artifact_key: &str,
        content_type: &str,
        mut stream: UploadStream,
    ) -> Result<StoredArtifact> {
        let temp_path = self.base_dir.join(format!("{artifact_key}.part"));
        let final_path = self.base_dir.join(format!("{artifact_key}.mp4"));
        let _ = tokio::fs::remove_file(&temp_path).await;
        let _ = tokio::fs::remove_file(&final_path).await;

        let mut file = tokio::fs::File::create(&temp_path).await.with_context(|| {
            format!(
                "failed to create local temp artifact {}",
                temp_path.display()
            )
        })?;
        let mut written = 0i64;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk)
                .await
                .context("failed to write local artifact bytes")?;
            written += chunk.len() as i64;
        }

        file.flush()
            .await
            .context("failed to flush local artifact file")?;
        drop(file);

        if written <= 0 {
            let _ = tokio::fs::remove_file(&temp_path).await;
            return Err(anyhow!("local artifact upload produced empty output"));
        }

        tokio::fs::rename(&temp_path, &final_path)
            .await
            .with_context(|| {
                format!("failed to finalize local artifact {}", final_path.display())
            })?;

        Ok(StoredArtifact {
            backend: self.backend_name().to_string(),
            local_path: Some(final_path.to_string_lossy().to_string()),
            storage_bucket: None,
            object_key: None,
            size_bytes: written,
            etag: None,
            content_type: content_type.to_string(),
        })
    }

    async fn presign_get(
        &self,
        _artifact: &StoredArtifact,
        _expires_secs: u64,
        _content_disposition: Option<&str>,
    ) -> Result<DownloadTicket> {
        Err(anyhow!(
            "localfs backend requires API-signed download ticket route"
        ))
    }

    async fn delete(&self, artifact: &StoredArtifact) -> Result<()> {
        if let Some(local_path) = artifact.local_path.as_deref() {
            let _ = tokio::fs::remove_file(local_path).await;
        }
        Ok(())
    }
}
