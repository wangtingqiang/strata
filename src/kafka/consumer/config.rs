use rdkafka::{ClientConfig, consumer::StreamConsumer};
use serde::Deserialize;

use crate::kafka::consumer::error::KafkaConsumerInitError;

#[derive(Debug, Clone, Deserialize)]
pub struct KafkaConsumerConfig {
    pub brokers: String,
    pub group_id: String,
    #[serde(default)]
    pub client_id: Option<String>,
}

impl KafkaConsumerConfig {
    pub fn create_consumer(&self) -> Result<StreamConsumer, KafkaConsumerInitError> {
        if self.brokers.trim().is_empty() {
            return Err(KafkaConsumerInitError::EmptyBrokers);
        }
        if self.group_id.trim().is_empty() {
            return Err(KafkaConsumerInitError::EmptyGroupId);
        }
        let mut config = ClientConfig::new();
        config.set("bootstrap.servers", &self.brokers);
        config.set("group.id", &self.group_id);
        config.set("auto.offset.reset", "earliest");
        config.set("enable.auto.commit", "false");
        if let Some(ref client_id) = self.client_id {
            config.set("client.id", client_id);
        }
        config
            .create()
            .map_err(|source| KafkaConsumerInitError::CreateConsumer { source })
    }
}
