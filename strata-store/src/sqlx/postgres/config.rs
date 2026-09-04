use std::time::Duration;

use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};

use super::PgPoolInitError;

#[derive(Debug, Clone, Deserialize)]
pub struct PgPoolConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: SecretString,
    pub database: String,
    pub max_connections: Option<u32>,
    pub acquire_timeout_seconds: Option<u64>,
}

impl PgPoolConfig {
    pub async fn connect(&self) -> Result<PgPool, PgPoolInitError> {
        if self.host.trim().is_empty() {
            return Err(PgPoolInitError::EmptyHost);
        }

        let options = PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(self.password.expose_secret())
            .database(&self.database);

        let mut pool_options = PgPoolOptions::new();

        if let Some(n) = self.max_connections {
            pool_options = pool_options.max_connections(n);
        }

        if let Some(t) = self.acquire_timeout_seconds {
            pool_options = pool_options.acquire_timeout(Duration::from_secs(t));
        }

        pool_options
            .connect_with(options)
            .await
            .map_err(PgPoolInitError::Connect)
    }
}
