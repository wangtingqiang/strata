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

#[cfg(test)]
mod tests {
    use crate::signer::{Ed25519JwtSigner, JwtSigner};

    use super::*;

    const PRIVATE_PEM: &str = r#"-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIAnbWbKwbQDD8fqnEa6ub3kv2a9XKC9a5w5iKR8vVqK4
-----END PRIVATE KEY-----"#;

    const PUBLIC_PEM: &str = r#"-----BEGIN PUBLIC KEY-----
MCowBQYDK2VwAyEAPWTvB+xr+7wmxJfJHOgpwnw7VwZRicW8gJnjT+SckeQ=
-----END PUBLIC KEY-----"#;

    const OTHER_PUBLIC_PEM: &str = r#"-----BEGIN PUBLIC KEY-----
MCowBQYDK2VwAyEAo1c9w3gXxuq4congiRcv1MbjI1alHPRzhG1lHO0c9lQ=
-----END PUBLIC KEY-----"#;

    fn verifier(pem: &str) -> Ed25519JwtVerifier {
        Ed25519JwtVerifier::try_from_pem(pem).unwrap()
    }

    fn signed_token() -> String {
        Ed25519JwtSigner::try_from_pem(PRIVATE_PEM)
            .unwrap()
            .sign(serde_json::json!({"sub": "user-1", "exp": 4_102_444_800i64}))
            .unwrap()
    }

    #[test]
    fn verifies_token_signed_by_matching_key() {
        let claims = verifier(PUBLIC_PEM).verify(&signed_token()).unwrap();

        assert_eq!(claims["sub"], "user-1");
    }

    #[test]
    fn rejects_tampered_token() {
        let mut chars: Vec<char> = signed_token().chars().collect();
        let last = chars.last_mut().unwrap();
        *last = if *last == 'a' { 'b' } else { 'a' };
        let tampered: String = chars.into_iter().collect();

        assert!(matches!(
            verifier(PUBLIC_PEM).verify(&tampered),
            Err(JwtVerifierError::DecodeFailed(_))
        ));
    }

    #[test]
    fn rejects_token_from_other_key() {
        assert!(matches!(
            verifier(OTHER_PUBLIC_PEM).verify(&signed_token()),
            Err(JwtVerifierError::DecodeFailed(_))
        ));
    }

    #[test]
    fn rejects_invalid_pem() {
        assert!(matches!(
            Ed25519JwtVerifier::try_from_pem("not a pem"),
            Err(JwtVerifierBuildError::InvalidPem(_))
        ));
    }
}
