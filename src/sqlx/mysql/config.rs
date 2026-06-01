use serde::Deserialize;
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};

use crate::sqlx::mysql::MySqlPoolInitError;

#[derive(Debug, Clone, Deserialize)]
pub struct MySqlPoolConfig {
    pub url: String,
    pub max_connections: u32,
}

impl MySqlPoolConfig {
    pub async fn connect(&self) -> Result<MySqlPool, MySqlPoolInitError> {
        if self.max_connections == 0 {
            return Err(MySqlPoolInitError::InvalidMaxConnections {
                value: self.max_connections,
            });
        }

        MySqlPoolOptions::new()
            .max_connections(self.max_connections)
            .connect(self.url.as_str())
            .await
            .map_err(|source| MySqlPoolInitError::Connect {
                url: self.url.clone(),
                source,
            })
    }
}
