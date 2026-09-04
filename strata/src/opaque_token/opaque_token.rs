use secrecy::{ExposeSecret, SecretString};
/// 不透明令牌错误。
#[derive(Debug, thiserror::Error)]
pub enum OpaqueTokenError {
    /// 令牌为空。
    #[error("token must not be empty")]
    Empty,
}

/// 不透明令牌（随机 UUID）。
#[derive(Debug, Clone)]
pub struct OpaqueToken(SecretString);

impl PartialEq for OpaqueToken {
    fn eq(&self, other: &Self) -> bool {
        self.expose_secret() == other.expose_secret()
    }
}

impl Eq for OpaqueToken {}

impl OpaqueToken {
    /// 生成新令牌。
    pub fn generate() -> Self {
        Self(SecretString::from(
            uuid::Uuid::new_v4().as_simple().to_string(),
        ))
    }

    /// 暴露令牌原文。
    pub fn expose_secret(&self) -> &str {
        self.0.expose_secret()
    }
}

impl TryFrom<String> for OpaqueToken {
    type Error = OpaqueTokenError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(OpaqueTokenError::Empty);
        }

        Ok(Self(SecretString::from(value)))
    }
}

impl TryFrom<&str> for OpaqueToken {
    type Error = OpaqueTokenError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

impl From<OpaqueToken> for SecretString {
    fn from(value: OpaqueToken) -> Self {
        value.0
    }
}
