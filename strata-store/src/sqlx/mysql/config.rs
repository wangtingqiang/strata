use std::time::Duration;

use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use sqlx::mysql::{MySqlConnectOptions, MySqlPool, MySqlPoolOptions};

use super::MySqlPoolInitError;

#[derive(Debug, Clone, Deserialize)]
pub struct MySqlPoolConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: SecretString,
    pub database: String,
    pub max_connections: Option<u32>,
    pub acquire_timeout_seconds: Option<u64>,
}

impl MySqlPoolConfig {
    pub async fn connect(&self) -> Result<MySqlPool, MySqlPoolInitError> {
        if self.host.trim().is_empty() {
            return Err(MySqlPoolInitError::EmptyHost);
        }

        let options = MySqlConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(self.password.expose_secret())
            .database(&self.database);

        let mut pool_options = MySqlPoolOptions::new();

        if let Some(n) = self.max_connections {
            pool_options = pool_options.max_connections(n);
        }

        if let Some(t) = self.acquire_timeout_seconds {
            pool_options = pool_options.acquire_timeout(Duration::from_secs(t));
        }

        pool_options
            .connect_with(options)
            .await
            .map_err(MySqlPoolInitError::Connect)
    }
}
