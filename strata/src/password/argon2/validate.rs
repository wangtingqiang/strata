#[derive(Debug, thiserror::Error)]
#[error("password hash format is corrupted: {0}")]
pub struct PasswordHashFormatError(#[from] argon2::password_hash::phc::Error);

/// 校验密码哈希格式。
pub fn validate_password_hash(hash: &str) -> Result<(), PasswordHashFormatError> {
    argon2::PasswordHash::new(hash)?;

    Ok(())
}
