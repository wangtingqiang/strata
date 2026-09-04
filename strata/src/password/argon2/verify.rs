use argon2::password_hash::PasswordVerifier;

/// 密码验证错误。
#[derive(Debug, thiserror::Error)]
#[error("password verification failed: {0}")]
pub struct PasswordVerifyError(#[from] argon2::password_hash::Error);

/// 验证密码与哈希是否匹配（密码不匹配返回 `Ok(false)`）。
pub fn verify_password(password: &str, hash: &str) -> Result<bool, PasswordVerifyError> {
    match argon2::Argon2::default().verify_password(password.as_bytes(), hash) {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::PasswordInvalid) => Ok(false),
        Err(error) => Err(PasswordVerifyError(error)),
    }
}

#[cfg(test)]
mod tests {
    use crate::password::argon2::{hash::hash_password, validate::validate_password_hash};

    use super::*;

    #[test]
    fn roundtrip_matches_hashed_password() {
        let hash = hash_password("correct horse battery staple").unwrap();
        assert!(verify_password("correct horse battery staple", &hash).unwrap());
    }

    #[test]
    fn wrong_password_returns_false_not_error() {
        let hash = hash_password("correct horse battery staple").unwrap();
        assert!(!verify_password("wrong password", &hash).unwrap());
    }

    #[test]
    fn rejects_corrupted_hash_format() {
        assert!(validate_password_hash("not-a-hash").is_err());
        assert!(verify_password("whatever", "not-a-hash").is_err());
    }

    #[test]
    fn accepts_valid_hash_format() {
        let hash = hash_password("correct horse battery staple").unwrap();
        assert!(validate_password_hash(&hash).is_ok());
    }
}
