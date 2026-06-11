use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwtVerifierBuildError {
    #[error("invalid Ed25519 PEM")]
    InvalidPem(#[source] jsonwebtoken::errors::Error),
}

#[derive(Debug)]
pub enum JwtVerification {
    Valid(serde_json::Value),
    Invalid,
}

#[derive(Debug, Error)]
pub enum JwtVerifierError {
    #[error("JWT decode failed")]
    DecodeFailed(#[source] jsonwebtoken::errors::Error),
}

pub trait JwtVerifier: Send + Sync + 'static {
    fn verify(&self, token: &str) -> Result<JwtVerification, JwtVerifierError>;
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
    fn verify(&self, token: &str) -> Result<JwtVerification, JwtVerifierError> {
        let data = match jsonwebtoken::decode::<serde_json::Value>(
            token,
            &self.decoding_key,
            &self.validation,
        ) {
            Ok(data) => data,
            Err(e) => {
                use jsonwebtoken::errors::ErrorKind;

                return match e.kind() {
                    ErrorKind::Json(_)
                    | ErrorKind::Base64(_)
                    | ErrorKind::Utf8(_)
                    | ErrorKind::Provider(_) => Err(JwtVerifierError::DecodeFailed(e)),
                    _ => Ok(JwtVerification::Invalid),
                };
            }
        };

        Ok(JwtVerification::Valid(data.claims))
    }
}
