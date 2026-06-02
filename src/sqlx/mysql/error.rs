use thiserror::Error;

#[derive(Debug, Error)]
pub enum MySqlPoolInitError {
    #[error("invalid mysql pool configuration: max_connections must be greater than 0")]
    InvalidMaxConnections { value: u32 },

    #[error("mysql host is empty")]
    EmptyHost,

    #[error("failed to connect mysql pool")]
    Connect {
        #[source]
        source: sqlx::Error,
    },
}
