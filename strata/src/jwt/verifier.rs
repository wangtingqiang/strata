use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwtVerifierBuildError {
    #[error("invalid Ed25519 PEM")]
    InvalidPem(#[source] jsonwebtoken::errors::Error),
}

#[derive(Debug, Error)]
pub enum JwtVerifierError {
    #[error("JWT verify failed")]
    VerifyFailed(#[source] jsonwebtoken::errors::Error),
    #[error("JWT claims serialization failed")]
    ClaimsSerializeFailed(#[source] serde_json::Error),
}

pub trait JwtVerifier: Send + Sync + 'static {
    fn verify(&self, token: &str) -> Result<String, JwtVerifierError>;
}

pub struct Ed25519JwtVerifier {
    decoding_key: DecodingKey,
    validation: Validation,
}

impl Ed25519JwtVerifier {
    pub fn try_from_pem(pem: impl AsRef<[u8]>) -> Result<Self, JwtVerifierBuildError> {
        let decoding_key = DecodingKey::from_ed_pem(pem.as_ref())
            .map_err(|source| JwtVerifierBuildError::InvalidPem(source))?;

        Ok(Self {
            decoding_key,
            validation: Validation::new(Algorithm::EdDSA),
        })
    }
}

impl JwtVerifier for Ed25519JwtVerifier {
    fn verify(&self, token: &str) -> Result<String, JwtVerifierError> {
        let data =
            jsonwebtoken::decode::<serde_json::Value>(token, &self.decoding_key, &self.validation)
                .map_err(JwtVerifierError::VerifyFailed)?;

        serde_json::to_string(&data.claims).map_err(JwtVerifierError::ClaimsSerializeFailed)
    }
}
