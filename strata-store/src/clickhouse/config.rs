use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

use super::ClickHouseInitError;

/// ClickHouse 配置。
#[derive(Debug, Clone, Deserialize)]
pub struct ClickHouseConfig {
    /// ClickHouse 服务地址。
    pub url: String,
    /// 默认数据库。
    pub database: String,
    /// 用户名。
    pub username: String,
    /// 密码。
    pub password: SecretString,
}

impl ClickHouseConfig {
    /// 构建 `clickhouse::Client` 并探活。
    pub async fn connect(&self) -> Result<clickhouse::Client, ClickHouseInitError> {
        if self.url.trim().is_empty() {
            return Err(ClickHouseInitError::EmptyUrl);
        }

        let client = clickhouse::Client::default()
            .with_url(&self.url)
            .with_database(&self.database)
            .with_user(&self.username)
            .with_password(self.password.expose_secret());

        client
            .query("SELECT 1")
            .fetch_one::<u8>()
            .await
            .map_err(ClickHouseInitError::Connect)?;

        Ok(client)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn deserializes_from_json() {
        let config: ClickHouseConfig = serde_json::from_value(json!({
            "url": "http://localhost:8123",
            "database": "default",
            "username": "admin",
            "password": "secret",
        }))
        .unwrap();

        assert_eq!(config.url, "http://localhost:8123");
        assert_eq!(config.database, "default");
        assert_eq!(config.username, "admin");
        assert_eq!(config.password.expose_secret(), "secret");
    }

    #[tokio::test]
    async fn connect_rejects_empty_url_before_any_network_call() {
        let config: ClickHouseConfig = serde_json::from_value(json!({
            "url": "   ",
            "database": "default",
            "username": "admin",
            "password": "secret",
        }))
        .unwrap();

        assert!(matches!(
            config.connect().await.err(),
            Some(ClickHouseInitError::EmptyUrl)
        ));
    }
}
