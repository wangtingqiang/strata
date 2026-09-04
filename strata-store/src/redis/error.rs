/// Redis 客户端初始化错误。
#[derive(Debug, thiserror::Error)]
pub enum RedisClientInitError {
    /// 主机为空。
    #[error("redis host is empty")]
    EmptyHost,

    /// 客户端构建失败。
    #[error("failed to build redis client: {0}")]
    BuildClient(#[source] redis::RedisError),

    /// 初始化探活连接失败。
    #[error("failed to connect redis during client initialization: {0}")]
    Connect(#[source] redis::RedisError),
}
