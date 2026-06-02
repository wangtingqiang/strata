use rdkafka::{ClientConfig, producer::FutureProducer};
use serde::Deserialize;

use crate::kafka::producer::KafkaProducerInitError;

#[derive(Debug, Clone, Deserialize)]
pub struct KafkaProducerConfig {
    pub brokers: String,
    #[serde(default)]
    pub client_id: Option<String>,
}

impl KafkaProducerConfig {
    pub fn create_producer(&self) -> Result<FutureProducer, KafkaProducerInitError> {
        if self.brokers.trim().is_empty() {
            return Err(KafkaProducerInitError::EmptyBrokers);
        }
        let mut config = ClientConfig::new();
        config.set("bootstrap.servers", &self.brokers);
        config.set("acks", "all");
        if let Some(ref client_id) = self.client_id {
            config.set("client.id", client_id);
        }
        config
            .create()
            .map_err(|source| KafkaProducerInitError::CreateProducer { source })
    }
}
