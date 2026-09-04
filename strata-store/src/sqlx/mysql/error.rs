/// MySQL 连接池初始化错误。
#[derive(Debug, thiserror::Error)]
pub enum MySqlPoolInitError {
    /// 主机为空。
    #[error("mysql host is empty")]
    EmptyHost,

    /// 连接池建立失败。
    #[error("failed to connect mysql pool: {0}")]
    Connect(#[source] sqlx::Error),
}
