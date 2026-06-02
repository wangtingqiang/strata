use thiserror::Error;

#[derive(Debug, Error)]
pub enum RedisClientInitError {
    #[error("redis host is empty")]
    EmptyHost,

    #[error("failed to build redis client")]
    BuildClient {
        #[source]
        source: redis::RedisError,
    },

    #[error("failed to connect redis during client initialization")]
    Connect {
        #[source]
        source: redis::RedisError,
    },
}
