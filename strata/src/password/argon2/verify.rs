use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordVerifier},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasswordVerifyError {
    InvalidHash,
    InvalidPassword,
}

impl std::fmt::Display for PasswordVerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidHash => write!(f, "password hash parse failed"),
            Self::InvalidPassword => write!(f, "password mismatch"),
        }
    }
}

impl std::error::Error for PasswordVerifyError {}

pub fn verify_password(hash: &str, password: &str) -> Result<(), PasswordVerifyError> {
    let parsed_hash = PasswordHash::new(hash).map_err(|_| PasswordVerifyError::InvalidHash)?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| PasswordVerifyError::InvalidPassword)
}
