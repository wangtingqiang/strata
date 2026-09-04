#[derive(Debug, thiserror::Error)]
pub enum MySqlPoolInitError {
    #[error("mysql host is empty")]
    EmptyHost,

    #[error("failed to connect mysql pool: {0}")]
    Connect(#[source] sqlx::Error),
}
