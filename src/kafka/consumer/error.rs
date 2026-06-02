use thiserror::Error;

#[derive(Debug, Error)]
pub enum KafkaConsumerInitError {
    #[error("kafka brokers is empty")]
    EmptyBrokers,
    #[error("kafka group id is empty")]
    EmptyGroupId,
    #[error("failed to create kafka consumer: {source}")]
    CreateConsumer {
        #[source]
        source: rdkafka::error::KafkaError,
    },
    #[error("failed to connect kafka consumer: {source}")]
    Connect {
        #[source]
        source: rdkafka::error::KafkaError,
    },
}
