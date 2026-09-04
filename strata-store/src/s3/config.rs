use secrecy::SecretString;
use serde::Deserialize;

/// S3 配置。
#[derive(Debug, Clone, Deserialize)]
pub struct S3Config {
    /// 内网访问端点。
    pub internal_endpoint: String,
    /// 公网访问端点。
    pub public_endpoint: String,
    /// 桶名。
    pub bucket: String,
    /// 地域。
    pub region: String,
    /// 访问密钥 ID。
    pub access_key: SecretString,
    /// 访问密钥。
    pub secret_key: SecretString,
}
