/// ClickHouse 客户端初始化错误。
#[derive(Debug, thiserror::Error)]
pub enum ChClientInitError {
    /// 地址为空。
    #[error("clickhouse url is empty")]
    EmptyUrl,

    /// 建立连接失败。
    #[error("failed to connect to clickhouse: {0}")]
    Connect(#[source] clickhouse::error::Error),
}
