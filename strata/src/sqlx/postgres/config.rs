use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use sqlx::PgPool;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::sqlx::postgres::PgPoolInitError;

#[derive(Debug, Clone, Deserialize)]
pub struct PgPoolConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: SecretString,
    pub database: String,
    pub max_connections: u32,
}

impl PgPoolConfig {
    pub async fn connect(&self) -> Result<PgPool, PgPoolInitError> {
        if self.host.trim().is_empty() {
            return Err(PgPoolInitError::EmptyHost);
        }
        if self.max_connections == 0 {
            return Err(PgPoolInitError::InvalidMaxConnections {
                value: self.max_connections,
            });
        }

        let options = PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(self.password.expose_secret())
            .database(&self.database);

        PgPoolOptions::new()
            .max_connections(self.max_connections)
            .connect_with(options)
            .await
            .map_err(|source| PgPoolInitError::Connect { source })
    }
}
