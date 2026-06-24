use argon2::password_hash::phc;
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
    pub fn expose_secret(&self) -> &str {
        self.0.expose_secret()
    }

    pub fn verify(&self, password: &str) -> Result<bool, PasswordVerifyError> {
        verify_password(self.0.expose_secret(), password)
    }
}

impl TryFrom<&str> for PasswordHash {
    type Error = PasswordVerifyError;

    fn try_from(hash: &str) -> Result<Self, Self::Error> {
        phc::PasswordHash::new(hash)
            .map_err(|e| PasswordVerifyError::InvalidHashFormat(e.into()))?;

        Ok(Self(hash.into()))
    }
}
