use std::collections::HashMap;

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use redis::{streams::StreamReadReply, RedisResult, Value};

use crate::{ClaimedQueueMessage, JobQueueConsumer, JobQueuePublisher, QueueJobMessage};

#[derive(Clone)]
pub struct RedisStreamsQueue {
    client: redis::Client,
    stream_key: String,
    group_name: String,
    consumer_name: String,
}

impl RedisStreamsQueue {
    pub fn new(
        redis_url: &str,
        stream_key: &str,
        group_name: &str,
        consumer_name: &str,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            client: redis::Client::open(redis_url).context("failed to create redis client")?,
            stream_key: stream_key.to_string(),
            group_name: group_name.to_string(),
            consumer_name: consumer_name.to_string(),
        })
    }

    async fn connection(&self) -> anyhow::Result<redis::aio::MultiplexedConnection> {
        self.client
            .get_multiplexed_async_connection()
            .await
            .context("failed to open redis connection")
    }

    fn parse_message(values: &HashMap<String, Value>) -> anyhow::Result<QueueJobMessage> {
        let json = values
            .get("payload")
            .ok_or_else(|| anyhow!("redis message missing payload"))?;
        let payload: String =
            redis::from_redis_value(json).context("invalid redis payload field")?;
        serde_json::from_str(&payload).context("invalid queue payload json")
    }
}

#[async_trait]
impl JobQueuePublisher for RedisStreamsQueue {
    async fn publish(&self, message: QueueJobMessage) -> anyhow::Result<()> {
        let payload =
            serde_json::to_string(&message).context("failed to serialize queue payload")?;
        let mut conn = self.connection().await?;
        let _: String = redis::cmd("XADD")
            .arg(&self.stream_key)
            .arg("*")
            .arg("payload")
            .arg(payload)
            .query_async(&mut conn)
            .await
            .context("failed to publish redis stream job")?;
        Ok(())
    }
}

#[async_trait]
impl JobQueueConsumer for RedisStreamsQueue {
    async fn ensure_group(&self) -> anyhow::Result<()> {
        let mut conn = self.connection().await?;
        let result: RedisResult<Value> = redis::cmd("XGROUP")
            .arg("CREATE")
            .arg(&self.stream_key)
            .arg(&self.group_name)
            .arg("0")
            .arg("MKSTREAM")
            .query_async(&mut conn)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(error) if error.to_string().contains("BUSYGROUP") => Ok(()),
            Err(error) => Err(anyhow!(error)).context("failed to ensure redis stream group"),
        }
    }

    async fn consume(&self, block_ms: usize) -> anyhow::Result<Option<ClaimedQueueMessage>> {
        let mut conn = self.connection().await?;
        let reply: StreamReadReply = redis::cmd("XREADGROUP")
            .arg("GROUP")
            .arg(&self.group_name)
            .arg(&self.consumer_name)
            .arg("COUNT")
            .arg(1)
            .arg("BLOCK")
            .arg(block_ms)
            .arg("STREAMS")
            .arg(&self.stream_key)
            .arg(">")
            .query_async(&mut conn)
            .await
            .context("failed to read redis stream job")?;

        let Some(stream) = reply.keys.first() else {
            return Ok(None);
        };
        let Some(message) = stream.ids.first() else {
            return Ok(None);
        };

        Ok(Some(ClaimedQueueMessage {
            stream_id: message.id.clone(),
            message: Self::parse_message(&message.map)?,
        }))
    }

    async fn ack(&self, stream_id: &str) -> anyhow::Result<()> {
        let mut conn = self.connection().await?;
        let _: i64 = redis::cmd("XACK")
            .arg(&self.stream_key)
            .arg(&self.group_name)
            .arg(stream_id)
            .query_async(&mut conn)
            .await
            .context("failed to ack redis stream job")?;
        Ok(())
    }
}
