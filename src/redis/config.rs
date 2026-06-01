use std::str::FromStr;

use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

use crate::redis::RedisClientInitError;

#[derive(Debug, Clone, Deserialize)]
pub struct RedisClientConfig {
    pub url: String,
    pub username: String,
    pub password: SecretString,
}

impl RedisClientConfig {
    pub async fn connect(&self) -> Result<redis::Client, RedisClientInitError> {
        let url = self.url.trim();
        if url.is_empty() {
            return Err(RedisClientInitError::EmptyUrl);
        }

        let connection_info = redis::ConnectionInfo::from_str(url)
            .map_err(|source| RedisClientInitError::BuildClient { source })?;
        let mut redis_settings = connection_info.redis_settings().clone();

        let username = self.username.trim();
        let password = self.password.expose_secret().trim();
        if !username.is_empty() {
            redis_settings = redis_settings.set_username(username);
        }
        if !password.is_empty() {
            redis_settings = redis_settings.set_password(password);
        }

        let connection_info = connection_info.set_redis_settings(redis_settings);

        let client = redis::Client::open(connection_info)
            .map_err(|source| RedisClientInitError::BuildClient { source })?;

        client
            .get_multiplexed_async_connection()
            .await
            .map_err(|source| RedisClientInitError::Connect { source })?;

        Ok(client)
    }
}
