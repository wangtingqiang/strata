use jsonwebtoken::{Algorithm, EncodingKey, Header};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwtSignerBuildError {
    #[error("invalid Ed25519 PEM")]
    InvalidPem(#[source] jsonwebtoken::errors::Error),
}

#[derive(Debug, Error)]
pub enum JwtSignerError {
    #[error("JWT sign failed")]
    SignFailed(#[source] jsonwebtoken::errors::Error),
}

pub trait JwtSigner: Send + Sync + 'static {
    fn sign(&self, payload: String) -> Result<String, JwtSignerError>;
}

pub struct Ed25519JwtSigner {
    encoding_key: EncodingKey,
}

impl Ed25519JwtSigner {
    pub fn from_pem(pem: impl AsRef<[u8]>) -> Result<Self, JwtSignerBuildError> {
        let encoding_key = EncodingKey::from_ed_pem(pem.as_ref())
            .map_err(|source| JwtSignerBuildError::InvalidPem(source))?;
        Ok(Self { encoding_key })
    }
}

impl JwtSigner for Ed25519JwtSigner {
    fn sign(&self, payload: String) -> Result<String, JwtSignerError> {
        let header = Header::new(Algorithm::EdDSA);
        jsonwebtoken::encode(&header, &payload, &self.encoding_key)
            .map_err(JwtSignerError::SignFailed)
    }
}
