use std::time::Duration;

use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use sqlx::mysql::{MySqlConnectOptions, MySqlPool, MySqlPoolOptions};

use super::MySqlPoolInitError;

/// MySQL 连接池配置。
#[derive(Debug, Clone, Deserialize)]
pub struct MySqlPoolConfig {
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

impl MySqlPoolConfig {
    /// 建立 MySQL 连接池，失败返回初始化错误。
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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn deserializes_optional_fields() {
        let config: MySqlPoolConfig = serde_json::from_value(json!({
            "host": "localhost",
            "port": 3306,
            "username": "root",
            "password": "secret",
            "database": "app",
            "max_connections": 10,
            "acquire_timeout_seconds": 5,
        }))
        .unwrap();

        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 3306);
        assert_eq!(config.username, "root");
        assert_eq!(config.password.expose_secret(), "secret");
        assert_eq!(config.database, "app");
        assert_eq!(config.max_connections, Some(10));
        assert_eq!(config.acquire_timeout_seconds, Some(5));
    }

    #[test]
    fn optional_fields_default_to_none() {
        let config: MySqlPoolConfig = serde_json::from_value(json!({
            "host": "localhost",
            "port": 3306,
            "username": "root",
            "password": "secret",
            "database": "app",
        }))
        .unwrap();

        assert_eq!(config.max_connections, None);
        assert_eq!(config.acquire_timeout_seconds, None);
    }

    #[tokio::test]
    async fn connect_rejects_empty_host_before_any_network_call() {
        let config: MySqlPoolConfig = serde_json::from_value(json!({
            "host": " ",
            "port": 3306,
            "username": "root",
            "password": "secret",
            "database": "app",
        }))
        .unwrap();

        assert!(matches!(
            config.connect().await.err(),
            Some(MySqlPoolInitError::EmptyHost)
        ));
    }
}
