use serde::Deserialize;

/// 服务监听配置。
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    /// 监听地址。
    pub host: String,
    /// 监听端口。
    pub port: u16,
}

impl ServerConfig {
    /// 监听地址（`host:port`）。
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
