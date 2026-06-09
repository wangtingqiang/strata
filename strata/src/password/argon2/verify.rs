use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordVerifier},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordVerifyError {
    #[error("password hash format is corrupted: {0}")]
    InvalidHashFormat(argon2::password_hash::Error),

    #[error("password hash computation failed: {0}")]
    HashComputationFailed(argon2::password_hash::Error),
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool, PasswordVerifyError> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|e| PasswordVerifyError::InvalidHashFormat(e))?;

    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false),
        Err(e) => Err(PasswordVerifyError::HashComputationFailed(e)),
    }
}
