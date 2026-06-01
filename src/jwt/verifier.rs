use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwtVerifierBuildError {
    #[error("invalid Ed25519 PEM")]
    InvalidPem {
        #[source]
        source: jsonwebtoken::errors::Error,
    },
}

#[derive(Debug, Error)]
pub enum JwtVerifierError {
    #[error("JWT verify failed")]
    VerifyFailed(#[source] jsonwebtoken::errors::Error),
}

pub struct JwtVerifier {
    decoding_key: DecodingKey,
    validation: Validation,
}

impl JwtVerifier {
    pub fn from_ed25519_pem(ed25519_pem: impl AsRef<[u8]>) -> Result<Self, JwtVerifierBuildError> {
        let decoding_key = DecodingKey::from_ed_pem(ed25519_pem.as_ref())
            .map_err(|source| JwtVerifierBuildError::InvalidPem { source })?;
        Ok(Self {
            decoding_key,
            validation: Validation::new(Algorithm::EdDSA),
        })
    }

    pub fn verify<C: DeserializeOwned>(&self, token: &str) -> Result<C, JwtVerifierError> {
        let data = jsonwebtoken::decode::<C>(token, &self.decoding_key, &self.validation)
            .map_err(JwtVerifierError::VerifyFailed)?;
        Ok(data.claims)
    }
}
