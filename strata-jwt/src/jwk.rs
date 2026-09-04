use base64::prelude::{BASE64_URL_SAFE_NO_PAD, Engine as _};
use ed25519_dalek::{VerifyingKey, pkcs8::DecodePublicKey};
use serde::Serialize;

/// JWK 导出器构建错误。
#[derive(Debug, thiserror::Error)]
pub enum JwkExporterBuildError {
    /// Ed25519 PEM 无效。
    #[error("invalid Ed25519 PEM: {0}")]
    InvalidPem(#[source] ed25519_dalek::pkcs8::spki::Error),
}

/// JWK 导出错误。
#[derive(Debug, thiserror::Error)]
pub enum JwkExporterError {
    /// JWK 序列化失败。
    #[error("export JWKS failed: {0}")]
    ExportFailed(#[source] serde_json::Error),
}

#[derive(Debug, Serialize)]
struct Ed25519Jwk {
    kty: &'static str,
    crv: &'static str,
    r#use: &'static str,
    alg: &'static str,
    kid: String,
    x: String,
}

/// 基于 Ed25519 的 JWK 导出器。
pub struct Ed25519JwkExporter {
    public_key: VerifyingKey,
    kid: String,
}

impl Clone for Ed25519JwkExporter {
    fn clone(&self) -> Self {
        Self {
            public_key: self.public_key,
            kid: self.kid.clone(),
        }
    }
}

impl Ed25519JwkExporter {
    /// 从 Ed25519 公钥 PEM 与 kid 构建导出器。
    pub fn new(public_key_pem: &str, kid: String) -> Result<Self, JwkExporterBuildError> {
        let public_key = VerifyingKey::from_public_key_pem(public_key_pem.trim())
            .map_err(JwkExporterBuildError::InvalidPem)?;

        Ok(Self { public_key, kid })
    }

    /// 导出 JWK JSON（OKP / Ed25519 / EdDSA）。
    pub fn export(&self) -> Result<String, JwkExporterError> {
        let x = BASE64_URL_SAFE_NO_PAD.encode(self.public_key.as_bytes());
        let jwk = Ed25519Jwk {
            kty: "OKP",
            crv: "Ed25519",
            r#use: "sig",
            alg: "EdDSA",
            kid: self.kid.clone(),
            x,
        };
        serde_json::to_string_pretty(&jwk).map_err(JwkExporterError::ExportFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUBLIC_PEM: &str = r#"-----BEGIN PUBLIC KEY-----
MCowBQYDK2VwAyEAPWTvB+xr+7wmxJfJHOgpwnw7VwZRicW8gJnjT+SckeQ=
-----END PUBLIC KEY-----"#;

    #[test]
    fn exports_expected_jwk() {
        let exporter = Ed25519JwkExporter::new(PUBLIC_PEM, "kid-1".to_owned()).unwrap();
        let jwk: serde_json::Value = serde_json::from_str(&exporter.export().unwrap()).unwrap();

        assert_eq!(jwk["kty"], "OKP");
        assert_eq!(jwk["crv"], "Ed25519");
        assert_eq!(jwk["use"], "sig");
        assert_eq!(jwk["alg"], "EdDSA");
        assert_eq!(jwk["kid"], "kid-1");

        let public_key = VerifyingKey::from_public_key_pem(PUBLIC_PEM.trim()).unwrap();
        let expected_x = BASE64_URL_SAFE_NO_PAD.encode(public_key.as_bytes());
        assert_eq!(jwk["x"], expected_x);
    }

    #[test]
    fn rejects_invalid_pem() {
        assert!(matches!(
            Ed25519JwkExporter::new("not a pem", "kid".to_owned()),
            Err(JwkExporterBuildError::InvalidPem(_))
        ));
    }
}
