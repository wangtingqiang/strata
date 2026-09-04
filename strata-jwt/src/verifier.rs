use jsonwebtoken::{Algorithm, DecodingKey, Validation};

#[derive(Debug, thiserror::Error)]
pub enum JwtVerifierBuildError {
    #[error("invalid Ed25519 PEM: {0}")]
    InvalidPem(#[source] jsonwebtoken::errors::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum JwtVerifierError {
    #[error("JWT decode failed: {0}")]
    DecodeFailed(#[source] jsonwebtoken::errors::Error),
}

pub trait JwtVerifier: Send + Sync + 'static {
    fn verify(&self, token: &str) -> Result<serde_json::Value, JwtVerifierError>;
}

pub struct Ed25519JwtVerifier {
    decoding_key: DecodingKey,
    validation: Validation,
}

impl Ed25519JwtVerifier {
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
