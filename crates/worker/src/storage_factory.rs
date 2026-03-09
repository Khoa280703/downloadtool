use anyhow::Result;
use object_store::backend_factory::{build_storage_backend as build_backend, StorageBackendConfig};
use object_store::StorageBackend;
use std::path::Path;
use std::sync::Arc;

use crate::worker_config::WorkerConfig;

pub async fn build_storage_backend(config: &WorkerConfig) -> Result<Arc<dyn StorageBackend>> {
    build_backend(&StorageBackendConfig {
        backend: config.artifact_backend.clone(),
        local_dir: config.artifact_dir.clone(),
        s3_bucket: config.s3_bucket.clone(),
        s3_region: config.s3_region.clone(),
        s3_endpoint: config.s3_endpoint.clone(),
        s3_access_key_id: config.s3_access_key_id.clone(),
        s3_secret_access_key: config.s3_secret_access_key.clone(),
        s3_force_path_style: config.s3_force_path_style,
        multipart_part_size_bytes: 8 * 1024 * 1024,
    })
    .await
}

pub async fn init_extractor_bundle(extractor_dir: &str) -> Result<()> {
    let extractor_path = Path::new(extractor_dir);
    if extractor_path.is_file() {
        extractor::init(Some(extractor_path)).await?;
    } else {
        extractor::init(None).await?;
    }
    Ok(())
}
