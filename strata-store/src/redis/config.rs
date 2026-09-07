use redis::IntoConnectionInfo;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

use super::RedisClientInitError;

/// Redis 客户端配置。
#[derive(Debug, Clone, Deserialize)]
pub struct RedisClientConfig {
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
}

impl RedisClientConfig {
    /// 建立 Redis 连接（同步）并探活，失败返回初始化错误。
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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn deserializes_from_json() {
        let config: RedisClientConfig = serde_json::from_value(json!({
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

    #[test]
    fn connect_rejects_empty_host_before_any_network_call() {
        let config: RedisClientConfig = serde_json::from_value(json!({
            "host": "   ",
            "port": 6379,
            "username": "default",
            "password": "secret",
            "database": 0,
        }))
        .unwrap();

        assert!(matches!(
            config.connect().err(),
            Some(RedisClientInitError::EmptyHost)
        ));
    }
}
