use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgPoolInitError {
    #[error("invalid postgres pool configuration: max_connections must be greater than 0")]
    InvalidMaxConnections { value: u32 },

    #[error("postgres host is empty")]
    EmptyHost,

    #[error("failed to connect postgres pool")]
    Connect {
        #[source]
        source: sqlx::Error,
    },
}
