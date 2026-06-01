use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TextError {
    #[error("value must not be empty")]
    Empty,
    #[error("value is too long")]
    TooLong,
}

pub fn trim_non_empty(value: &str) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_owned())
    }
}

pub fn trim_optional(value: Option<String>) -> Option<String> {
    value.and_then(|value| trim_non_empty(&value))
}

pub fn trim_non_empty_bounded(value: &str, max_chars: usize) -> Result<String, TextError> {
    let value = trim_non_empty(value).ok_or(TextError::Empty)?;
    validate_max_chars(&value, max_chars)?;
    Ok(value)
}

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
