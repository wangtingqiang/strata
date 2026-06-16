use argon2::password_hash::PasswordHash as Argon2Hash;
use secrecy::{ExposeSecret, SecretString};

use crate::password::argon2::{verify::PasswordVerifyError, verify_password};

#[derive(Debug, Clone)]
pub struct PasswordHash(SecretString);

impl PartialEq for PasswordHash {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl Eq for PasswordHash {}

impl PasswordHash {
    pub fn new(hash: impl Into<String>) -> Result<Self, PasswordVerifyError> {
        let hash = hash.into();
        Argon2Hash::new(&hash).map_err(PasswordVerifyError::InvalidHashFormat)?;
        Ok(Self(SecretString::new(hash.into())))
    }

    pub fn expose_secret(&self) -> &str {
        self.0.expose_secret()
    }

    pub fn verify(&self, password: &str) -> Result<bool, PasswordVerifyError> {
        verify_password(self.0.expose_secret(), password)
    }
}
