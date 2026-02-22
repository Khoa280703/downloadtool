//! gRPC server implementation for GPU worker
//!
//! Provides GPU-accelerated transcoding services via gRPC.

use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;

use futures::{Stream, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status, Streaming};
use tracing::{debug, error, info, warn};

use gpu_pipeline::{
    GpuPipeline, PipelineConfig, TranscodeMode, VideoCodec, Resolution,
    frame_queue::DEFAULT_MAX_CONCURRENT_JOBS,
};

use crate::transcode::{
    gpu_worker_server::{GpuWorker, GpuWorkerServer as TonicGpuWorkerServer},
    TranscodeChunk, TranscodeOptions, HealthRequest, HealthResponse,
    HealthStatus, GpuInfo,
};

/// GPU Worker gRPC service implementation.
pub struct GpuWorkerService {
    /// GPU transcoding pipeline
    pipeline: Arc<GpuPipeline>,
    /// Maximum concurrent transcoding jobs
    max_concurrent: usize,
}

impl GpuWorkerService {
    /// Create a new GPU worker service.
    pub fn new(config: ServerConfig) -> Result<Self, crate::WorkerError> {
        info!("Creating GPU worker service");

        // Create pipeline configuration
        let pipeline_config = PipelineConfig {
            input_codec: VideoCodec::H264,
            output_codec: VideoCodec::H264,
            output_resolution: Resolution::p1080(),
            target_bitrate: 5_000_000,
            mode: TranscodeMode::Recompress,
            watermark_config: None,
            max_concurrent_jobs: config.max_concurrent_jobs,
        };

        let pipeline = GpuPipeline::new(pipeline_config)
            .map_err(|e| crate::WorkerError::TranscodeError(e.to_string()))?;

        Ok(Self {
            pipeline: Arc::new(pipeline),
            max_concurrent: config.max_concurrent_jobs,
        })
    }

    /// Get GPU information for health check.
    fn get_gpu_info(&self) -> GpuInfo {
        GpuInfo {
            name: "NVIDIA GPU".to_string(),
            cuda_version: "12.3".to_string(),
            total_memory: 24 * 1024 * 1024 * 1024, // 24GB placeholder
            available_memory: 20 * 1024 * 1024 * 1024, // 20GB placeholder
            utilization: 0,
            max_concurrent_jobs: self.max_concurrent as u32,
        }
    }
}

#[tonic::async_trait]
impl GpuWorker for GpuWorkerService {
    type TranscodeStream = Pin<Box<dyn Stream<Item = Result<TranscodeChunk, Status>> + Send>>;

    /// Transcode video data in a bidirectional streaming manner.
    async fn transcode(
        &self,
        request: Request<Streaming<TranscodeChunk>>,
    ) -> Result<Response<Self::TranscodeStream>, Status> {
        info!("Received transcoding request");

        // Check GPU capacity
        if !self.pipeline.has_capacity() {
            warn!("GPU at capacity, rejecting transcoding request");
            return Err(Status::resource_exhausted(
                "GPU at capacity, no available job slots"
            ));
        }

        let mut stream = request.into_inner();
        let pipeline = Arc::clone(&self.pipeline);

        // Create channel for output chunks
        let (tx, rx) = mpsc::channel(16);

        // Spawn transcoding task
        tokio::spawn(async move {
            let mut options: Option<TranscodeOptions> = None;
            let mut input_buffer: Vec<bytes::Bytes> = Vec::new();

            // Collect input chunks
            while let Some(chunk_result) = stream.next().await {
                match chunk_result {
                    Ok(chunk) => {
                        if chunk.eof {
                            debug!("Received EOF marker");
                            break;
                        }

                        // Store options from first chunk
                        if options.is_none() && chunk.options.is_some() {
                            options = chunk.options.clone();
                        }

                        input_buffer.push(bytes::Bytes::from(chunk.data));
                    }
                    Err(e) => {
                        error!("Error receiving chunk: {}", e);
                        let _ = tx.send(Err(Status::internal("Stream error"))).await;
                        return;
                    }
                }
            }

            debug!("Received {} input chunks", input_buffer.len());

            // Create input stream from buffer
            let input_stream = futures::stream::iter(input_buffer);

            // Perform transcoding
            match pipeline.transcode(input_stream) {
                Ok(mut output_stream) => {
                    let mut sequence = 0u64;

                    // Send output chunks
                    while let Some(result) = output_stream.next().await {
                        match result {
                            Ok(data) => {
                                let chunk = TranscodeChunk {
                                    data: data.to_vec(),
                                    options: options.clone(),
                                    eof: false,
                                    sequence,
                                };

                                if tx.send(Ok(chunk)).await.is_err() {
                                    debug!("Output channel closed");
                                    return;
                                }
                                sequence += 1;
                            }
                            Err(e) => {
                                error!("Transcoding error: {}", e);
                                let _ = tx.send(Err(Status::internal(e.to_string()))).await;
                                return;
                            }
                        }
                    }

                    // Send EOF marker
                    let eof_chunk = TranscodeChunk {
                        data: vec![],
                        options: None,
                        eof: true,
                        sequence,
                    };
                    let _ = tx.send(Ok(eof_chunk)).await;
                    info!("Transcoding completed, sent {} chunks", sequence);
                }
                Err(e) => {
                    error!("Failed to start transcoding: {}", e);
                    let _ = tx.send(Err(Status::internal(e.to_string()))).await;
                }
            }
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(output_stream) as Self::TranscodeStream))
    }

    /// Health check endpoint.
    async fn health_check(
        &self,
        request: Request<HealthRequest>,
    ) -> Result<Response<HealthResponse>, Status> {
        let capability = request.into_inner().capability;
        debug!("Health check requested, capability: {:?}", capability);

        let gpu_info = self.get_gpu_info();
        let available_slots = self.pipeline.available_slots();

        // Determine health status based on available capacity
        let status = if available_slots == 0 {
            HealthStatus::Degraded as i32
        } else if available_slots >= self.max_concurrent / 2 {
            HealthStatus::Healthy as i32
        } else {
            HealthStatus::Degraded as i32
        };

        let response = HealthResponse {
            status,
            gpu_info: Some(gpu_info),
            capabilities: vec![
                "h264_decode".to_string(),
                "h265_decode".to_string(),
                "h264_encode".to_string(),
                "h265_encode".to_string(),
                "watermark".to_string(),
            ],
            error_message: if available_slots == 0 {
                "GPU at capacity".to_string()
            } else {
                String::new()
            },
        };

        Ok(Response::new(response))
    }
}

/// GPU Worker gRPC server.
pub struct GpuWorkerServer {
    /// Server bind address
    bind_addr: SocketAddr,
    /// Server configuration
    config: ServerConfig,
}

impl GpuWorkerServer {
    /// Create a new GPU worker server.
    pub fn new(bind_addr: SocketAddr, config: ServerConfig) -> Self {
        Self { bind_addr, config }
    }

    /// Run the gRPC server.
    pub async fn run(self) -> Result<(), crate::WorkerError> {
        info!("Starting GPU worker gRPC server on {}", self.bind_addr);

        let service = GpuWorkerService::new(self.config)?;
        let tonic_service = TonicGpuWorkerServer::new(service);

        Server::builder()
            .add_service(tonic_service)
            .serve(self.bind_addr)
            .await
            .map_err(|e| crate::WorkerError::ServerError(e.to_string()))?;

        Ok(())
    }

    /// Get the server bind address.
    pub fn bind_addr(&self) -> SocketAddr {
        self.bind_addr
    }
}

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
            max_concurrent_jobs: DEFAULT_MAX_CONCURRENT_JOBS,
            cuda_device_id: 0,
        }
    }
}

impl ServerConfig {
    /// Load configuration from environment variables.
    ///
    /// # Environment Variables
    /// - `GPU_WORKER_BIND` - Bind address (default: "0.0.0.0:50051")
    /// - `GPU_WORKER_MAX_JOBS` - Max concurrent jobs (default: 6)
    /// - `CUDA_DEVICE_ID` - CUDA device ID (default: 0)
    pub fn from_env() -> Result<Self, crate::WorkerError> {
        use std::env;

        let bind_addr = env::var("GPU_WORKER_BIND")
            .unwrap_or_else(|_| "0.0.0.0:50051".to_string());

        let max_concurrent_jobs = env::var("GPU_WORKER_MAX_JOBS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(DEFAULT_MAX_CONCURRENT_JOBS);

        let cuda_device_id = env::var("CUDA_DEVICE_ID")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        Ok(Self {
            bind_addr,
            max_concurrent_jobs,
            cuda_device_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_config_default() {
        let config = ServerConfig::default();
        assert_eq!(config.bind_addr, "0.0.0.0:50051");
        assert_eq!(config.max_concurrent_jobs, DEFAULT_MAX_CONCURRENT_JOBS);
        assert_eq!(config.cuda_device_id, 0);
    }

    #[test]
    fn test_server_creation() {
        let addr: SocketAddr = "0.0.0.0:50051".parse().unwrap();
        let config = ServerConfig::default();
        let server = GpuWorkerServer::new(addr, config);
        assert_eq!(server.bind_addr(), addr);
    }

    #[tokio::test]
    async fn test_gpu_info() {
        let config = ServerConfig::default();
        let service = GpuWorkerService::new(config).unwrap();
        let info = service.get_gpu_info();

        assert_eq!(info.max_concurrent_jobs, DEFAULT_MAX_CONCURRENT_JOBS as u32);
        assert!(info.total_memory > 0);
    }
}
