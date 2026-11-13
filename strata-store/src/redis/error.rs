/// Redis 初始化错误。
#[derive(Debug, thiserror::Error)]
pub enum RedisInitError {
    /// 主机为空。
    #[error("redis host is empty")]
    EmptyHost,
    /// 客户端构建失败。
    #[error("failed to build redis client: {0}")]
    BuildClient(#[source] redis::RedisError),
    /// 初始化连接失败。
    #[error("failed to connect redis during initialization: {0}")]
    Connect(#[source] redis::RedisError),
}
