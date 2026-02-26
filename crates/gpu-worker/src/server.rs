//! gRPC server for GPU worker — binds on Home Server, receives jobs from VPS via WireGuard
//!
//! Build with `--features gpu` on Home Server (requires FFmpeg NVENC/NVDEC + CUDA).
//! Without `--features gpu`, compiles to a stub server that rejects all requests.

use std::net::SocketAddr;

use tracing::info;

/// Configuration for the GPU worker server.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Bind address
    pub bind_addr: String,
    /// Maximum concurrent transcoding jobs
    pub max_concurrent_jobs: usize,
    /// CUDA device ID to use
    pub cuda_device_id: i32,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_addr: "0.0.0.0:50051".to_string(),
            max_concurrent_jobs: 6,
            cuda_device_id: 0,
        }
    }
}

impl ServerConfig {
    /// Load configuration from environment variables.
    ///
    /// - `GPU_WORKER_BIND` — bind address (default: "0.0.0.0:50051")
    /// - `GPU_WORKER_MAX_JOBS` — max concurrent jobs (default: 6)
    /// - `CUDA_DEVICE_ID` — CUDA device ID (default: 0)
    pub fn from_env() -> Result<Self, crate::WorkerError> {
        use std::env;
        Ok(Self {
            bind_addr: env::var("GPU_WORKER_BIND")
                .unwrap_or_else(|_| "0.0.0.0:50051".to_string()),
            max_concurrent_jobs: env::var("GPU_WORKER_MAX_JOBS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(6),
            cuda_device_id: env::var("CUDA_DEVICE_ID")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
        })
    }
}

/// GPU Worker gRPC server.
pub struct GpuWorkerServer {
    bind_addr: SocketAddr,
    config: ServerConfig,
}

impl GpuWorkerServer {
    pub fn new(bind_addr: SocketAddr, config: ServerConfig) -> Self {
        Self { bind_addr, config }
    }

    pub fn bind_addr(&self) -> SocketAddr {
        self.bind_addr
    }

    /// Run the gRPC server.
    /// Requires `--features gpu` to enable real GPU transcoding.
    pub async fn run(self) -> Result<(), crate::WorkerError> {
        #[cfg(feature = "gpu")]
        {
            self.run_gpu().await
        }
        #[cfg(not(feature = "gpu"))]
        {
            info!(
                "GPU worker running in STUB mode on {} (build with --features gpu for real GPU support)",
                self.bind_addr
            );
            self.run_stub().await
        }
    }

    /// Run stub server — rejects all transcode requests with UNIMPLEMENTED.
    #[cfg(not(feature = "gpu"))]
    async fn run_stub(self) -> Result<(), crate::WorkerError> {
        use tonic::transport::Server;
        use crate::transcode::gpu_worker_server::GpuWorkerServer as TonicServer;

        let service = StubGpuWorkerService;
        Server::builder()
            .add_service(TonicServer::new(service))
            .serve(self.bind_addr)
            .await
            .map_err(|e| crate::WorkerError::ServerError(e.to_string()))
    }

    /// Run GPU-enabled server with real NVENC/NVDEC pipeline.
    #[cfg(feature = "gpu")]
    async fn run_gpu(self) -> Result<(), crate::WorkerError> {
        use std::pin::Pin;
        use std::sync::Arc;
        use futures::{Stream, StreamExt};
        use tokio::sync::mpsc;
        use tokio_stream::wrappers::ReceiverStream;
        use tonic::{transport::Server, Request, Response, Status, Streaming};
        use tonic::async_trait;
        use tracing::{debug, error, warn};
        use gpu_pipeline::{GpuPipeline, PipelineConfig, TranscodeMode, VideoCodec, Resolution};
        use crate::transcode::{
            gpu_worker_server::{GpuWorker, GpuWorkerServer as TonicServer},
            TranscodeChunk, HealthRequest, HealthResponse, HealthStatus, GpuInfo,
        };

        struct GpuService { pipeline: Arc<GpuPipeline>, max_concurrent: usize }

        #[async_trait]
        impl GpuWorker for GpuService {
            type TranscodeStream = Pin<Box<dyn Stream<Item = Result<TranscodeChunk, Status>> + Send>>;

            async fn transcode(
                &self,
                request: Request<Streaming<TranscodeChunk>>,
            ) -> Result<Response<Self::TranscodeStream>, Status> {
                if !self.pipeline.has_capacity() {
                    warn!("GPU at capacity, rejecting request");
                    return Err(Status::resource_exhausted("GPU at capacity"));
                }

                let mut stream = request.into_inner();
                let pipeline = Arc::clone(&self.pipeline);
                let (tx, rx) = mpsc::channel(16);

                tokio::spawn(async move {
                    let mut options = None;
                    let mut input_buffer: Vec<bytes::Bytes> = Vec::new();

                    while let Some(result) = stream.next().await {
                        match result {
                            Ok(chunk) => {
                                if chunk.eof { break; }
                                if options.is_none() { options = chunk.options.clone(); }
                                input_buffer.push(bytes::Bytes::from(chunk.data));
                            }
                            Err(e) => {
                                error!("Stream error: {}", e);
                                let _ = tx.send(Err(Status::internal("Stream error"))).await;
                                return;
                            }
                        }
                    }

                    let input = futures::stream::iter(input_buffer);
                    match pipeline.transcode(input) {
                        Ok(mut out) => {
                            let mut seq = 0u64;
                            while let Some(r) = out.next().await {
                                match r {
                                    Ok(data) => {
                                        let c = TranscodeChunk { data: data.to_vec(), options: options.clone(), eof: false, sequence: seq };
                                        if tx.send(Ok(c)).await.is_err() { return; }
                                        seq += 1;
                                    }
                                    Err(e) => { let _ = tx.send(Err(Status::internal(e.to_string()))).await; return; }
                                }
                            }
                            let _ = tx.send(Ok(TranscodeChunk { data: vec![], options: None, eof: true, sequence: seq })).await;
                        }
                        Err(e) => { let _ = tx.send(Err(Status::internal(e.to_string()))).await; }
                    }
                });

                Ok(Response::new(Box::pin(ReceiverStream::new(rx)) as Self::TranscodeStream))
            }

            async fn health_check(
                &self,
                _request: Request<HealthRequest>,
            ) -> Result<Response<HealthResponse>, Status> {
                let slots = self.pipeline.available_slots();
                Ok(Response::new(HealthResponse {
                    status: if slots > 0 { HealthStatus::Healthy as i32 } else { HealthStatus::Degraded as i32 },
                    gpu_info: Some(GpuInfo {
                        name: "NVIDIA RTX 3090".to_string(),
                        cuda_version: "12.3".to_string(),
                        total_memory: 24 * 1024 * 1024 * 1024,
                        available_memory: 20 * 1024 * 1024 * 1024,
                        utilization: 0,
                        max_concurrent_jobs: self.max_concurrent as u32,
                    }),
                    capabilities: vec!["h264_decode".into(), "h264_encode".into(), "watermark".into()],
                    error_message: String::new(),
                }))
            }
        }

        let pipeline_config = PipelineConfig {
            input_codec: VideoCodec::H264,
            output_codec: VideoCodec::H264,
            output_resolution: Resolution::p1080(),
            target_bitrate: 5_000_000,
            mode: TranscodeMode::Recompress,
            watermark_config: None,
            max_concurrent_jobs: self.config.max_concurrent_jobs,
        };
        let pipeline = GpuPipeline::new(pipeline_config)
            .map_err(|e| crate::WorkerError::TranscodeError(e.to_string()))?;
        let service = GpuService { pipeline: Arc::new(pipeline), max_concurrent: self.config.max_concurrent_jobs };

        info!("Starting GPU worker gRPC server on {}", self.bind_addr);
        Server::builder()
            .add_service(TonicServer::new(service))
            .serve(self.bind_addr)
            .await
            .map_err(|e| crate::WorkerError::ServerError(e.to_string()))
    }
}

/// Stub service for non-GPU builds — returns UNIMPLEMENTED for all calls.
#[cfg(not(feature = "gpu"))]
struct StubGpuWorkerService;

#[cfg(not(feature = "gpu"))]
#[tonic::async_trait]
impl crate::transcode::gpu_worker_server::GpuWorker for StubGpuWorkerService {
    type TranscodeStream = std::pin::Pin<Box<dyn futures::Stream<Item = Result<crate::transcode::TranscodeChunk, tonic::Status>> + Send>>;

    async fn transcode(
        &self,
        _request: tonic::Request<tonic::Streaming<crate::transcode::TranscodeChunk>>,
    ) -> Result<tonic::Response<Self::TranscodeStream>, tonic::Status> {
        Err(tonic::Status::unimplemented("GPU support not compiled — rebuild with --features gpu"))
    }

    async fn health_check(
        &self,
        _request: tonic::Request<crate::transcode::HealthRequest>,
    ) -> Result<tonic::Response<crate::transcode::HealthResponse>, tonic::Status> {
        use crate::transcode::{HealthResponse, HealthStatus};
        Ok(tonic::Response::new(HealthResponse {
            status: HealthStatus::Unhealthy as i32,
            gpu_info: None,
            capabilities: vec![],
            error_message: "GPU support not compiled".to_string(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_config_default() {
        let config = ServerConfig::default();
        assert_eq!(config.bind_addr, "0.0.0.0:50051");
        assert_eq!(config.max_concurrent_jobs, 6);
    }

    #[test]
    fn test_server_creation() {
        let addr: SocketAddr = "0.0.0.0:50051".parse().unwrap();
        let config = ServerConfig::default();
        let server = GpuWorkerServer::new(addr, config);
        assert_eq!(server.bind_addr(), addr);
    }
}
