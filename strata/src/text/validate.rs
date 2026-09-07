/// 校验字符串是否符合小写蛇形机器标识规则
///
/// 规则如下：
/// - 首字符必须是小写英文字母
/// - 后续字符只允许小写英文字母、数字或下划线
/// - 不负责去除首尾空白
pub fn is_snake_lower_alphanumeric(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };

    if !first.is_ascii_lowercase() {
        return false;
    }

    chars.all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_string() {
        assert!(!is_snake_lower_alphanumeric(""));
    }

    #[test]
    fn rejects_non_lowercase_first_char() {
        assert!(!is_snake_lower_alphanumeric("Abc"));
        assert!(!is_snake_lower_alphanumeric("1abc"));
        assert!(!is_snake_lower_alphanumeric("_abc"));
    }

    #[test]
    fn accepts_valid_identifiers() {
        assert!(is_snake_lower_alphanumeric("abc"));
        assert!(is_snake_lower_alphanumeric("a"));
        assert!(is_snake_lower_alphanumeric("a1_b2"));
        assert!(is_snake_lower_alphanumeric("abc_"));
    }

    #[test]
    fn rejects_invalid_characters() {
        assert!(!is_snake_lower_alphanumeric("abc-def"));
        assert!(!is_snake_lower_alphanumeric("abc def"));
        assert!(!is_snake_lower_alphanumeric("abcDef"));
        assert!(!is_snake_lower_alphanumeric("中文"));
    }
}
