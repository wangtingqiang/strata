use argon2::password_hash::PasswordVerifier;
use secrecy::{ExposeSecret, SecretString};

#[derive(Debug, thiserror::Error)]
#[error("password hash computation failed: {0}")]
pub struct PasswordVerifyError(#[from] argon2::password_hash::Error);

pub fn verify_password(
    password: &SecretString,
    hash: &SecretString,
) -> Result<bool, PasswordVerifyError> {
    match argon2::Argon2::default()
        .verify_password(password.expose_secret().as_bytes(), hash.expose_secret())
    {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::PasswordInvalid) => Ok(false),
        Err(error) => Err(PasswordVerifyError(error)),
    }
}
