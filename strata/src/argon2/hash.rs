use argon2::password_hash::PasswordHasher;

/// 密码哈希错误。
#[derive(Debug, thiserror::Error)]
#[error("password hash failed: {0}")]
pub struct PasswordHashError(#[from] argon2::password_hash::Error);

/// 使用 Argon2 默认参数对密码进行哈希。
pub fn hash_password(password: &str) -> Result<String, PasswordHashError> {
    let hash = argon2::Argon2::default().hash_password(password.as_bytes())?;

    Ok(hash.to_string())
}
