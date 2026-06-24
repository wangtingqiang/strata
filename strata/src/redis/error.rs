use thiserror::Error;

#[derive(Debug, Error)]
pub enum RedisClientInitError {
    #[error("redis host is empty")]
    EmptyHost,

    #[error("failed to build redis client: {0}")]
    BuildClient(#[source] redis::RedisError),

    #[error("failed to connect redis during client initialization: {0}")]
    Connect(#[source] redis::RedisError),
}
