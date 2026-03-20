use std::sync::Arc;

use anyhow::{anyhow, Result};
use aws_config::BehaviorVersion;
use aws_sdk_s3::config::{Builder as S3ConfigBuilder, Credentials, Region};

use crate::s3_storage_backend::S3StorageBackend;
use crate::StorageBackend;

#[derive(Clone, Debug)]
pub struct StorageBackendConfig {
    pub s3_bucket: Option<String>,
    pub s3_region: Option<String>,
    pub s3_endpoint: Option<String>,
    pub s3_access_key_id: Option<String>,
    pub s3_secret_access_key: Option<String>,
    pub s3_force_path_style: bool,
    pub multipart_part_size_bytes: usize,
}

pub async fn build_storage_backend(
    config: &StorageBackendConfig,
) -> Result<Arc<dyn StorageBackend>> {
    let bucket = config
        .s3_bucket
        .clone()
        .ok_or_else(|| anyhow!("S3_BUCKET_NAME is required for mux artifact backend"))?;
    let shared = aws_config::defaults(BehaviorVersion::latest()).load().await;
    let mut builder = S3ConfigBuilder::from(&shared);

    if let Some(region) = config.s3_region.as_deref() {
        builder = builder.region(Region::new(region.to_string()));
    }
    if let Some(endpoint) = config.s3_endpoint.as_deref() {
        builder = builder.endpoint_url(endpoint);
    }
    builder = builder.force_path_style(config.s3_force_path_style);

    if let (Some(access_key), Some(secret_key)) = (
        config.s3_access_key_id.clone(),
        config.s3_secret_access_key.clone(),
    ) {
        builder = builder
            .credentials_provider(Credentials::new(access_key, secret_key, None, None, "env"));
    }

    Ok(Arc::new(S3StorageBackend::new(
        aws_sdk_s3::Client::from_conf(builder.build()),
        bucket,
        config.multipart_part_size_bytes,
    )))
}
