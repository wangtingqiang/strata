#[derive(Debug, thiserror::Error)]
pub enum ChClientInitError {
    #[error("clickhouse url is empty")]
    EmptyUrl,

    #[error("failed to connect to clickhouse: {0}")]
    Connect(#[source] clickhouse::error::Error),
}
