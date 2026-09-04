use jsonwebtoken::{Algorithm, EncodingKey, Header};

/// 签名器构建错误。
#[derive(Debug, thiserror::Error)]
pub enum JwtSignerBuildError {
    /// Ed25519 PEM 无效。
    #[error("invalid Ed25519 PEM: {0}")]
    InvalidPem(#[source] jsonwebtoken::errors::Error),
}

/// 签名错误。
#[derive(Debug, thiserror::Error)]
pub enum JwtSignerError {
    /// JWT 编码失败。
    #[error("JWT encode failed: {0}")]
    EncodeFailed(#[source] jsonwebtoken::errors::Error),
}

/// JWT 签名器。
pub trait JwtSigner: Send + Sync + 'static {
    /// 对 payload 签名，返回 JWT 字符串。
    fn sign(&self, payload: serde_json::Value) -> Result<String, JwtSignerError>;
}

/// 基于 Ed25519 的 JWT 签名器。
pub struct Ed25519JwtSigner {
    encoding_key: EncodingKey,
}

impl Ed25519JwtSigner {
    /// 从 Ed25519 私钥 PEM 构建签名器。
    pub fn try_from_pem(pem: impl AsRef<[u8]>) -> Result<Self, JwtSignerBuildError> {
        let encoding_key =
            EncodingKey::from_ed_pem(pem.as_ref()).map_err(JwtSignerBuildError::InvalidPem)?;

        Ok(Self { encoding_key })
    }
}

impl JwtSigner for Ed25519JwtSigner {
    fn sign(&self, payload: serde_json::Value) -> Result<String, JwtSignerError> {
        let header = Header::new(Algorithm::EdDSA);

        jsonwebtoken::encode(&header, &payload, &self.encoding_key)
            .map_err(JwtSignerError::EncodeFailed)
    }
}

#[cfg(test)]
mod tests {
    use base64::prelude::{BASE64_URL_SAFE_NO_PAD, Engine};

    use super::*;

    const PRIVATE_PEM: &str = r#"-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIAnbWbKwbQDD8fqnEa6ub3kv2a9XKC9a5w5iKR8vVqK4
-----END PRIVATE KEY-----"#;

    fn signer() -> Ed25519JwtSigner {
        Ed25519JwtSigner::try_from_pem(PRIVATE_PEM).unwrap()
    }

    fn payload() -> serde_json::Value {
        serde_json::json!({"sub": "user-1", "exp": 4_102_444_800i64})
    }

    #[test]
    fn signs_payload_into_eddsa_jwt() {
        let token = signer().sign(payload()).unwrap();

        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(parts.len(), 3);

        let header: serde_json::Value =
            serde_json::from_slice(&BASE64_URL_SAFE_NO_PAD.decode(parts[0]).unwrap()).unwrap();
        assert_eq!(header["alg"], "EdDSA");
    }

    #[test]
    fn sign_is_deterministic() {
        let signer = signer();
        assert_eq!(
            signer.sign(payload()).unwrap(),
            signer.sign(payload()).unwrap()
        );
    }

    #[test]
    fn rejects_invalid_pem() {
        assert!(matches!(
            Ed25519JwtSigner::try_from_pem("not a pem"),
            Err(JwtSignerBuildError::InvalidPem(_))
        ));
    }
}
