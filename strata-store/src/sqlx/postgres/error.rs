/// PostgreSQL 连接池初始化错误。
#[derive(Debug, thiserror::Error)]
pub enum PgPoolInitError {
    /// 主机为空。
    #[error("postgres host is empty")]
    EmptyHost,

    /// 连接池建立失败。
    #[error("failed to connect postgres pool: {0}")]
    Connect(#[source] sqlx::Error),
}
