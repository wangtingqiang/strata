use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};
use rand_core::OsRng;
use secrecy::SecretString;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasswordHashError {
    HashFailed,
}

impl std::fmt::Display for PasswordHashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HashFailed => write!(f, "password hash failed"),
        }
    }
}

impl std::error::Error for PasswordHashError {}

pub fn hash_password(password: &str) -> Result<SecretString, PasswordHashError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| PasswordHashError::HashFailed)?;

    Ok(SecretString::new(hash.to_string().into()))
}
