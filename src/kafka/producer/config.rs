use std::time::Duration;

use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, Producer};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

use crate::kafka::producer::KafkaProducerInitError;

#[derive(Debug, Clone, Deserialize)]
pub struct KafkaProducerConfig {
    pub brokers: String,
    #[serde(default)]
    pub client_id: Option<String>,
    #[serde(default)]
    pub security_protocol: Option<String>,
    #[serde(default)]
    pub sasl_mechanism: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<SecretString>,
}

impl KafkaProducerConfig {
    pub fn connect(&self) -> Result<FutureProducer, KafkaProducerInitError> {
        if self.brokers.trim().is_empty() {
            return Err(KafkaProducerInitError::EmptyBrokers);
        }
        let mut config = ClientConfig::new();
        config.set("bootstrap.servers", &self.brokers);
        config.set("acks", "all");
        if let Some(ref client_id) = self.client_id {
            config.set("client.id", client_id);
        }
        if let Some(ref protocol) = self.security_protocol {
            config.set("security.protocol", protocol);
        }
        if let Some(ref mechanism) = self.sasl_mechanism {
            config.set("sasl.mechanism", mechanism);
        }
        if let Some(ref username) = self.username {
            config.set("sasl.username", username);
        }
        if let Some(ref password) = self.password {
            config.set("sasl.password", password.expose_secret());
        }
        let producer: FutureProducer = config
            .create()
            .map_err(|source| KafkaProducerInitError::CreateProducer { source })?;

        producer
            .client()
            .fetch_metadata(None, Duration::from_secs(10))
            .map_err(|source| KafkaProducerInitError::Connect { source })?;

        Ok(producer)
    }
}
