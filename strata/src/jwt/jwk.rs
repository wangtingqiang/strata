use base64::prelude::{BASE64_URL_SAFE_NO_PAD, Engine as _};
use ed25519_dalek::{VerifyingKey, pkcs8::DecodePublicKey};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwkExporterBuildError {
    #[error("invalid Ed25519 PEM")]
    InvalidPem {
        #[source]
        source: ed25519_dalek::pkcs8::spki::Error,
    },
}

#[derive(Debug, Error)]
pub enum JwkExporterError {
    #[error("export JWKS failed")]
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
    pub fn new(public_key_pem: &str, kid: String) -> Result<Self, JwkExporterBuildError> {
        let public_key = VerifyingKey::from_public_key_pem(public_key_pem.trim())
            .map_err(|source| JwkExporterBuildError::InvalidPem { source })?;
        Ok(Self { public_key, kid })
    }

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
