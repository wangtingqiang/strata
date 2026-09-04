use secrecy::{ExposeSecret, SecretString};
const MIN_LENGTH: usize = 8;
const MAX_LENGTH: usize = 128;

const SPECIAL_CHARS: &str = "!@#$%^&*()-_=+[]{}|;:'\",.<>?/~`";

/// 密码校验错误。
#[derive(Debug, thiserror::Error)]
pub enum PasswordValidationError {
    /// 密码为空。
    #[error("password must not be empty")]
    Empty,
    /// 密码过短。
    #[error("password must be at least {MIN_LENGTH} characters")]
    TooShort,
    /// 密码过长。
    #[error("password must not exceed {MAX_LENGTH} characters")]
    TooLong,
    /// 包含非法字符。
    #[error("password contains invalid character '{0}'")]
    ContainsInvalidChar(char),
    /// 字符类型不足（需含大写/小写/数字/特殊字符中至少三类）。
    #[error("password must contain at least 3 of: uppercase, lowercase, digit, special character")]
    InsufficientCharacterTypes,
}

/// 校验密码强度（`SecretString`）。
pub fn validate_password(password: &SecretString) -> Result<(), PasswordValidationError> {
    validate_raw(password.expose_secret())
}

fn validate_raw(password: &str) -> Result<(), PasswordValidationError> {
    if password.is_empty() {
        return Err(PasswordValidationError::Empty);
    }

    if password.len() < MIN_LENGTH {
        return Err(PasswordValidationError::TooShort);
    }
    if password.len() > MAX_LENGTH {
        return Err(PasswordValidationError::TooLong);
    }

    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;
    let mut has_special = false;

    for c in password.chars() {
        if c.is_uppercase() {
            has_upper = true;
        } else if c.is_lowercase() {
            has_lower = true;
        } else if c.is_ascii_digit() {
            has_digit = true;
        } else if SPECIAL_CHARS.contains(c) {
            has_special = true;
        } else {
            return Err(PasswordValidationError::ContainsInvalidChar(c));
        }
    }

    let type_count = [has_upper, has_lower, has_digit, has_special]
        .iter()
        .filter(|&&x| x)
        .count();

    if type_count < 3 {
        return Err(PasswordValidationError::InsufficientCharacterTypes);
    }

    Ok(())
}

/// 密码校验便捷 trait。
pub trait ValidatePassword {
    /// 校验密码强度。
    fn validate_password(&self) -> Result<(), PasswordValidationError>;
}

impl ValidatePassword for SecretString {
    fn validate_password(&self) -> Result<(), PasswordValidationError> {
        validate_password(self)
    }
}

impl ValidatePassword for String {
    fn validate_password(&self) -> Result<(), PasswordValidationError> {
        validate_raw(self)
    }
}

impl ValidatePassword for &str {
    fn validate_password(&self) -> Result<(), PasswordValidationError> {
        validate_raw(self)
    }
}
