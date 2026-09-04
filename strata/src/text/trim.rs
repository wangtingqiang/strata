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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_and_filters_empty() {
        assert_eq!(trim_non_empty("  hello  ").as_deref(), Some("hello"));
        assert_eq!(trim_non_empty("   "), None);
        assert_eq!(
            trim_optional(Some("  x  ".to_owned())).as_deref(),
            Some("x")
        );
        assert_eq!(trim_optional(None), None);
    }

    #[test]
    fn bounded_trims_validate_length() {
        assert_eq!(trim_non_empty_bounded("hello", 10).unwrap(), "hello");
        assert!(matches!(
            trim_non_empty_bounded("hello", 3),
            Err(TextError::TooLong)
        ));
        assert!(matches!(
            trim_non_empty_bounded("   ", 10),
            Err(TextError::Empty)
        ));
        assert!(matches!(
            trim_optional_bounded(Some("hello".to_owned()), 3),
            Err(TextError::TooLong)
        ));
        assert_eq!(trim_optional_bounded(None, 10).unwrap(), None);
    }
}
