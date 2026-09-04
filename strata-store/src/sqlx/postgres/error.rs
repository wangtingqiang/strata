#[derive(Debug, thiserror::Error)]
pub enum PgPoolInitError {
    #[error("postgres host is empty")]
    EmptyHost,

    #[error("failed to connect postgres pool: {0}")]
    Connect(#[source] sqlx::Error),
}
