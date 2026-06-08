use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChClientInitError {
    #[error("clickhouse url is empty")]
    EmptyUrl,
    #[error("failed to connect to clickhouse: {source}")]
    Connect {
        #[source]
        source: clickhouse::error::Error,
    },
}
