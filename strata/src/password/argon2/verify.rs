use argon2::password_hash::PasswordVerifier;

#[derive(Debug, thiserror::Error)]
#[error("password verification failed: {0}")]
pub struct PasswordVerifyError(#[from] argon2::password_hash::Error);

pub fn verify_password(password: &str, hash: &str) -> Result<bool, PasswordVerifyError> {
    match argon2::Argon2::default().verify_password(password.as_bytes(), hash) {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::PasswordInvalid) => Ok(false),
        Err(error) => Err(PasswordVerifyError(error)),
    }
}
