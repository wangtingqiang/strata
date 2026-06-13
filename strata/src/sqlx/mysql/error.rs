use thiserror::Error;

#[derive(Debug, Error)]
pub enum MySqlPoolInitError {
    #[error("mysql host is empty")]
    EmptyHost,

    #[error("failed to connect mysql pool")]
    Connect(#[source] sqlx::Error),
}
