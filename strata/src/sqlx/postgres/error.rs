use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgPoolInitError {
    #[error("postgres host is empty")]
    EmptyHost,

    #[error("failed to connect postgres pool")]
    Connect(#[source] sqlx::Error),
}
