#[cfg(feature = "redis-connection-manager")]
use std::time::Duration;

use redis::IntoConnectionInfo;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

use super::RedisInitError;

/// Redis 配置。
#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    /// 主机地址。
    pub host: String,
    /// 端口。
    pub port: u16,
    /// 用户名。
    pub username: String,
    /// 密码。
    pub password: SecretString,
    /// 数据库编号。
    pub database: u8,
    /// 连接管理器参数。
    #[cfg(feature = "redis-connection-manager")]
    #[serde(default)]
    pub connection_manager: RedisConnectionManagerConfig,
}

/// Redis 连接管理器参数。
#[cfg(feature = "redis-connection-manager")]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct RedisConnectionManagerConfig {
    /// 命令响应超时（毫秒）；缺省使用 redis 默认（500ms）。
    pub response_timeout_ms: Option<u64>,
    /// 建连超时（毫秒）；缺省使用 redis 默认（1s）。
    pub connection_timeout_ms: Option<u64>,
    /// 重连尝试次数；缺省使用 redis 默认（6）。
    pub number_of_retries: Option<usize>,
}

impl RedisConfig {
    /// 构建 `redis::Client` 并探活。
    pub fn connect(&self) -> Result<redis::Client, RedisInitError> {
        let client = self.build_client()?;

        let _ = client.get_connection().map_err(RedisInitError::Connect)?;

        Ok(client)
    }

    /// 构建 `redis::aio::ConnectionManager`，初始化即建立连接。
    #[cfg(feature = "redis-connection-manager")]
    pub async fn connection_manager(
        &self,
    ) -> Result<redis::aio::ConnectionManager, RedisInitError> {
        let client = self.build_client()?;

        // redis 的 setter 语义：None 表示关闭超时；因此只在 Some 时覆盖，None 保持 new() 的默认值。
        let mut config = redis::aio::ConnectionManagerConfig::new();

        if let Some(response_timeout) = self.connection_manager.response_timeout() {
            config = config.set_response_timeout(Some(response_timeout));
        }

        if let Some(connection_timeout) = self.connection_manager.connection_timeout() {
            config = config.set_connection_timeout(Some(connection_timeout));
        }

        if let Some(number_of_retries) = self.connection_manager.number_of_retries {
            config = config.set_number_of_retries(number_of_retries);
        }

        redis::aio::ConnectionManager::new_with_config(client, config)
            .await
            .map_err(RedisInitError::Connect)
    }

    /// 构建客户端。
    fn build_client(&self) -> Result<redis::Client, RedisInitError> {
        let redis_settings = redis::RedisConnectionInfo::default()
            .set_db(self.database as i64)
            .set_username(self.username.clone())
            .set_password(self.password.expose_secret());

        let connection_info = redis::ConnectionAddr::Tcp(self.host.clone(), self.port)
            .into_connection_info()
            .map_err(RedisInitError::BuildClient)?
            .set_redis_settings(redis_settings);

        redis::Client::open(connection_info).map_err(RedisInitError::BuildClient)
    }
}

#[cfg(feature = "redis-connection-manager")]
impl RedisConnectionManagerConfig {
    /// 命令响应超时；未配置时为 `None`。
    fn response_timeout(&self) -> Option<Duration> {
        self.response_timeout_ms.map(Duration::from_millis)
    }

    /// 建连超时；未配置时为 `None`。
    fn connection_timeout(&self) -> Option<Duration> {
        self.connection_timeout_ms.map(Duration::from_millis)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn deserializes_from_json() {
        let config: RedisConfig = serde_json::from_value(json!({
            "host": "127.0.0.1",
            "port": 6379,
            "username": "default",
            "password": "secret",
            "database": 0,
        }))
        .unwrap();

        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 6379);
        assert_eq!(config.username, "default");
        assert_eq!(config.password.expose_secret(), "secret");
        assert_eq!(config.database, 0);
    }
}

#[cfg(all(test, feature = "redis-connection-manager"))]
mod connection_manager_tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn deserializes_manager_section() {
        let config: RedisConfig = serde_json::from_value(json!({
            "host": "127.0.0.1",
            "port": 6379,
            "username": "default",
            "password": "secret",
            "database": 0,
            "connection_manager": { "response_timeout_ms": 3000 },
        }))
        .unwrap();

        assert_eq!(config.connection_manager.response_timeout_ms, Some(3000));
    }

    #[test]
    fn missing_manager_section_defaults_to_all_none() {
        let config: RedisConfig = serde_json::from_value(json!({
            "host": "127.0.0.1",
            "port": 6379,
            "username": "default",
            "password": "secret",
            "database": 0,
        }))
        .unwrap();

        assert_eq!(config.connection_manager.response_timeout_ms, None);
        assert_eq!(config.connection_manager.connection_timeout_ms, None);
        assert_eq!(config.connection_manager.number_of_retries, None);
    }
}
