use thiserror::Error;

#[derive(Debug, Error)]
pub enum KafkaProducerInitError {
    #[error("kafka brokers is empty")]
    EmptyBrokers,
    #[error("failed to create kafka producer: {source}")]
    CreateProducer {
        #[source]
        source: rdkafka::error::KafkaError,
    },
    #[error("failed to connect kafka producer: {source}")]
    Connect {
        #[source]
        source: rdkafka::error::KafkaError,
    },
}
