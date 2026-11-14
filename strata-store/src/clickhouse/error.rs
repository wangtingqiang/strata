/// ClickHouse 初始化错误。
#[derive(Debug, thiserror::Error)]
pub enum ClickHouseInitError {
    /// 建立连接失败。
    #[error("failed to connect to clickhouse: {0}")]
    Connect(#[source] clickhouse::error::Error),
}
