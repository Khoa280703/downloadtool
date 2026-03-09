pub mod redis_streams;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueueJobMessage {
    pub job_id: String,
    pub dedupe_key: String,
    pub requested_at_ms: i64,
}

#[derive(Clone, Debug)]
pub struct ClaimedQueueMessage {
    pub stream_id: String,
    pub message: QueueJobMessage,
}

#[async_trait]
pub trait JobQueuePublisher: Send + Sync {
    async fn publish(&self, message: QueueJobMessage) -> anyhow::Result<()>;
}

#[async_trait]
pub trait JobQueueConsumer: Send + Sync {
    async fn ensure_group(&self) -> anyhow::Result<()>;
    async fn consume(&self, block_ms: usize) -> anyhow::Result<Option<ClaimedQueueMessage>>;
    async fn ack(&self, stream_id: &str) -> anyhow::Result<()>;
}
