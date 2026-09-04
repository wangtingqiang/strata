/// AWS SigV4 签名错误。
#[derive(Debug, thiserror::Error)]
pub enum SigningError {
    /// HMAC 密钥构建失败。
    #[error("failed to build hmac: {0}")]
    BuildHmac(#[from] hmac::digest::InvalidLength),

    /// URL 解析失败。
    #[error("failed to parse url: {0}")]
    ParseUrl(#[from] url::ParseError),

    /// URL 缺少主机名。
    #[error("url must have a host")]
    MissingHost,

    /// amz_date 格式非法。
    #[error("invalid amz_date format")]
    InvalidAmzDate,
}
