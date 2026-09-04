/// 文本处理错误。
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TextError {
    /// 文本为空。
    #[error("value must not be empty")]
    Empty,
    /// 文本过长。
    #[error("value is too long")]
    TooLong,
}

/// 去除首尾空白；空串返回 `None`。
pub fn trim_non_empty(value: &str) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_owned())
    }
}

/// 去除可选文本的首尾空白。
pub fn trim_optional(value: Option<String>) -> Option<String> {
    value.and_then(|value| trim_non_empty(&value))
}

/// 去除首尾空白并校验长度上限。
pub fn trim_non_empty_bounded(value: &str, max_chars: usize) -> Result<String, TextError> {
    let value = trim_non_empty(value).ok_or(TextError::Empty)?;
    validate_max_chars(&value, max_chars)?;
    Ok(value)
}

/// 去除可选文本空白并校验长度上限。
pub fn trim_optional_bounded(
    value: Option<String>,
    max_chars: usize,
) -> Result<Option<String>, TextError> {
    trim_optional(value)
        .map(|value| {
            validate_max_chars(&value, max_chars)?;
            Ok(value)
        })
        .transpose()
}

fn validate_max_chars(value: &str, max_chars: usize) -> Result<(), TextError> {
    if value.chars().count() > max_chars {
        Err(TextError::TooLong)
    } else {
        Ok(())
    }
}
