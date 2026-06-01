use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgPoolInitError {
    #[error("invalid postgres pool configuration: max_connections must be greater than 0")]
    InvalidMaxConnections { value: u32 },

    #[error("failed to connect postgres pool")]
    Connect {
        url: String,
        #[source]
        source: sqlx::Error,
    },
}
