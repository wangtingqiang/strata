use std::time::Duration;

use rdkafka::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

use crate::kafka::consumer::error::KafkaConsumerInitError;

#[derive(Debug, Clone, Deserialize)]
pub struct KafkaConsumerConfig {
    pub brokers: String,
    pub group_id: String,
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

impl KafkaConsumerConfig {
    pub fn connect(&self) -> Result<StreamConsumer, KafkaConsumerInitError> {
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
        let consumer: StreamConsumer = config
            .create()
            .map_err(|source| KafkaConsumerInitError::CreateConsumer { source })?;

        consumer
            .fetch_metadata(None, Duration::from_secs(1))
            .map_err(|source| KafkaConsumerInitError::Connect { source })?;

        Ok(consumer)
    }
}
