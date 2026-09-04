/// 密码强度校验。
pub mod password;

pub use password::{PasswordValidationError, ValidatePassword, validate_password};
