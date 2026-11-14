use serde::Serialize;

/// 统一响应体。
#[derive(Debug, Clone, Serialize)]
pub struct ResponseBody<T> {
    /// 是否成功。
    pub success: bool,
    /// 业务错误码（成功时为 `"0"`）。
    pub code: String,
    /// 人类可读消息。
    pub message: String,
    /// 服务器时间（RFC3339，UTC+8）。
    pub server_time: String,
    /// 业务数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}
