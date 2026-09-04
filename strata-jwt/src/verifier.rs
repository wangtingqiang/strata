use jsonwebtoken::{Algorithm, DecodingKey, Validation};

/// 验证器构建错误。
#[derive(Debug, thiserror::Error)]
pub enum JwtVerifierBuildError {
    /// Ed25519 PEM 无效。
    #[error("invalid Ed25519 PEM: {0}")]
    InvalidPem(#[source] jsonwebtoken::errors::Error),
}

/// 验证错误。
#[derive(Debug, thiserror::Error)]
pub enum JwtVerifierError {
    /// JWT 解码失败。
    #[error("JWT decode failed: {0}")]
    DecodeFailed(#[source] jsonwebtoken::errors::Error),
}

/// JWT 验证器。
pub trait JwtVerifier: Send + Sync + 'static {
    /// 验证 token 并返回其 claims。
    fn verify(&self, token: &str) -> Result<serde_json::Value, JwtVerifierError>;
}

/// 基于 Ed25519 的 JWT 验证器。
pub struct Ed25519JwtVerifier {
    decoding_key: DecodingKey,
    validation: Validation,
}

impl Ed25519JwtVerifier {
    /// 从 Ed25519 公钥 PEM 构建验证器（使用 EdDSA 默认校验规则）。
    pub fn try_from_pem(pem: impl AsRef<[u8]>) -> Result<Self, JwtVerifierBuildError> {
        let decoding_key =
            DecodingKey::from_ed_pem(pem.as_ref()).map_err(JwtVerifierBuildError::InvalidPem)?;

        Ok(Self {
            decoding_key,
            validation: Validation::new(Algorithm::EdDSA),
        })
    }
}

impl JwtVerifier for Ed25519JwtVerifier {
    fn verify(&self, token: &str) -> Result<serde_json::Value, JwtVerifierError> {
        match jsonwebtoken::decode::<serde_json::Value>(token, &self.decoding_key, &self.validation)
        {
            Ok(data) => Ok(data.claims),
            Err(error) => Err(JwtVerifierError::DecodeFailed(error)),
        }
    }
}
