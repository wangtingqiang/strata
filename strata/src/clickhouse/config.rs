use clickhouse::Client;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

use crate::clickhouse::ChClientInitError;

#[derive(Debug, Clone, Deserialize)]
pub struct ChClientConfig {
    pub url: String,
    pub database: String,
    pub username: String,
    pub password: SecretString,
}

impl ChClientConfig {
    pub async fn connect(&self) -> Result<Client, ChClientInitError> {
        if self.url.trim().is_empty() {
            return Err(ChClientInitError::EmptyUrl);
        }

        let client = Client::default()
            .with_url(&self.url)
            .with_database(&self.database)
            .with_user(&self.username)
            .with_password(self.password.expose_secret());

        client
            .query("SELECT 1")
            .fetch_one::<u8>()
            .await
            .map_err(|source| ChClientInitError::Connect { source })?;

        Ok(client)
    }
}
