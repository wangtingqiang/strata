use redis::IntoConnectionInfo;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

use crate::redis::RedisClientInitError;

#[derive(Debug, Clone, Deserialize)]
pub struct RedisClientConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: SecretString,
    pub database: u8,
}

impl RedisClientConfig {
    pub fn connect(&self) -> Result<redis::Client, RedisClientInitError> {
        let host = self.host.trim();

        if host.is_empty() {
            return Err(RedisClientInitError::EmptyHost);
        }

        let redis_info = redis::RedisConnectionInfo::default()
            .set_db(self.database as i64)
            .set_username(self.username.clone())
            .set_password(self.password.expose_secret());

        let connection_info = redis::ConnectionAddr::Tcp(host.to_owned(), self.port)
            .into_connection_info()
            .map_err(RedisClientInitError::BuildClient)?
            .set_redis_settings(redis_info);

        let client =
            redis::Client::open(connection_info).map_err(RedisClientInitError::BuildClient)?;

        let _ = client
            .get_connection()
            .map_err(RedisClientInitError::Connect)?;

        Ok(client)
    }
}
