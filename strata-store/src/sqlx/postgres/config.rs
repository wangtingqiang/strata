use std::time::Duration;

use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};

use super::PgPoolInitError;

/// PostgreSQL 连接池配置。
#[derive(Debug, Clone, Deserialize)]
pub struct PgPoolConfig {
    /// 主机地址。
    pub host: String,
    /// 端口。
    pub port: u16,
    /// 用户名。
    pub username: String,
    /// 密码。
    pub password: SecretString,
    /// 库名。
    pub database: String,
    /// 连接池上限。
    pub max_connections: Option<u32>,
    /// 获取连接超时（秒）。
    pub acquire_timeout_seconds: Option<u64>,
}

impl PgPoolConfig {
    /// 建立 PostgreSQL 连接池。
    pub async fn connect(&self) -> Result<PgPool, PgPoolInitError> {
        let options = PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(self.password.expose_secret())
            .database(&self.database);

        let mut pool_options = PgPoolOptions::new();

        if let Some(max_connections) = self.max_connections {
            pool_options = pool_options.max_connections(max_connections);
        }

        if let Some(acquire_timeout) = self.acquire_timeout() {
            pool_options = pool_options.acquire_timeout(acquire_timeout);
        }

        pool_options
            .connect_with(options)
            .await
            .map_err(PgPoolInitError::Connect)
    }

    /// 获取连接超时；未配置时为 `None`。
    fn acquire_timeout(&self) -> Option<Duration> {
        self.acquire_timeout_seconds.map(Duration::from_secs)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn deserializes_optional_fields() {
        let config: PgPoolConfig = serde_json::from_value(json!({
            "host": "localhost",
            "port": 5432,
            "username": "root",
            "password": "secret",
            "database": "app",
            "max_connections": 10,
            "acquire_timeout_seconds": 5,
        }))
        .unwrap();

        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 5432);
        assert_eq!(config.username, "root");
        assert_eq!(config.password.expose_secret(), "secret");
        assert_eq!(config.database, "app");
        assert_eq!(config.max_connections, Some(10));
        assert_eq!(config.acquire_timeout_seconds, Some(5));
    }

    #[test]
    fn optional_fields_default_to_none() {
        let config: PgPoolConfig = serde_json::from_value(json!({
            "host": "localhost",
            "port": 5432,
            "username": "root",
            "password": "secret",
            "database": "app",
        }))
        .unwrap();

        assert_eq!(config.max_connections, None);
        assert_eq!(config.acquire_timeout_seconds, None);
    }
}
