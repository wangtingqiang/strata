use clickhouse::Client;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

use super::ChClientInitError;

/// ClickHouse 客户端配置。
#[derive(Debug, Clone, Deserialize)]
pub struct ChClientConfig {
    /// ClickHouse 服务地址。
    pub url: String,
    /// 默认数据库。
    pub database: String,
    /// 用户名。
    pub username: String,
    /// 密码。
    pub password: SecretString,
}

impl ChClientConfig {
    /// 建立 ClickHouse 连接并执行 `SELECT 1` 探活，失败返回初始化错误。
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
            .map_err(ChClientInitError::Connect)?;

        Ok(client)
    }
}
