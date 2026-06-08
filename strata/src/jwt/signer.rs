use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwtSignerBuildError {
    #[error("invalid Ed25519 PEM")]
    InvalidPem {
        #[source]
        source: jsonwebtoken::errors::Error,
    },
}

#[derive(Debug, Error)]
pub enum JwtSignerError {
    #[error("JWT sign failed")]
    SignFailed(#[source] jsonwebtoken::errors::Error),
}

pub struct JwtSigner {
    encoding_key: EncodingKey,
}

impl JwtSigner {
    pub fn from_ed25519_pem(ed25519_pem: impl AsRef<[u8]>) -> Result<Self, JwtSignerBuildError> {
        let encoding_key = EncodingKey::from_ed_pem(ed25519_pem.as_ref())
            .map_err(|source| JwtSignerBuildError::InvalidPem { source })?;
        Ok(Self { encoding_key })
    }

    pub fn sign<C: Serialize>(&self, claims: &C) -> Result<String, JwtSignerError> {
        jsonwebtoken::encode(&Header::new(Algorithm::EdDSA), claims, &self.encoding_key)
            .map_err(JwtSignerError::SignFailed)
    }
}
