use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use sqlx::MySqlPool;
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};

use crate::sqlx::mysql::MySqlPoolInitError;

#[derive(Debug, Clone, Deserialize)]
pub struct MySqlPoolConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: SecretString,
    pub database: String,
    pub max_connections: u32,
}

impl MySqlPoolConfig {
    pub async fn connect(&self) -> Result<MySqlPool, MySqlPoolInitError> {
        if self.host.trim().is_empty() {
            return Err(MySqlPoolInitError::EmptyHost);
        }
        if self.max_connections == 0 {
            return Err(MySqlPoolInitError::InvalidMaxConnections {
                value: self.max_connections,
            });
        }

        let options = MySqlConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(self.password.expose_secret())
            .database(&self.database);

        MySqlPoolOptions::new()
            .max_connections(self.max_connections)
            .connect_with(options)
            .await
            .map_err(|source| MySqlPoolInitError::Connect { source })
    }
}
