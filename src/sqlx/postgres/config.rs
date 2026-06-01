use serde::Deserialize;
use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::sqlx::postgres::PgPoolInitError;

#[derive(Debug, Clone, Deserialize)]
pub struct PgPoolConfig {
    pub url: String,
    pub max_connections: u32,
}

impl PgPoolConfig {
    pub async fn connect(&self) -> Result<PgPool, PgPoolInitError> {
        if self.max_connections == 0 {
            return Err(PgPoolInitError::InvalidMaxConnections {
                value: self.max_connections,
            });
        }

        PgPoolOptions::new()
            .max_connections(self.max_connections)
            .connect(self.url.as_str())
            .await
            .map_err(|source| PgPoolInitError::Connect {
                url: self.url.clone(),
                source,
            })
    }
}
