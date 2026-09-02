use argon2::password_hash::PasswordHasher;
use secrecy::{ExposeSecret, SecretString};

#[derive(Debug, thiserror::Error)]
#[error("password hash failed: {0}")]
pub struct PasswordHashError(#[from] argon2::password_hash::Error);

pub fn hash_password(password: &SecretString) -> Result<SecretString, PasswordHashError> {
    let hash = argon2::Argon2::default().hash_password(password.expose_secret().as_bytes())?;

    Ok(hash.to_string().into())
}
