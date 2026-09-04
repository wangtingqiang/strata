#[derive(Debug, thiserror::Error)]
pub enum SigningError {
    #[error("failed to build hmac: {0}")]
    BuildHmac(#[from] hmac::digest::InvalidLength),

    #[error("failed to parse url: {0}")]
    ParseUrl(#[from] url::ParseError),

    #[error("url must have a host")]
    MissingHost,

    #[error("invalid amz_date format")]
    InvalidAmzDate,
}
