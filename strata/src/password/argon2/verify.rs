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
