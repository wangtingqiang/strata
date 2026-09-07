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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addr_joins_host_and_port() {
        let config = ServerConfig {
            host: "127.0.0.1".to_owned(),
            port: 8080,
        };

        assert_eq!(config.addr(), "127.0.0.1:8080");
    }
}
