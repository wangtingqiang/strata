/// `str` 的便捷方法。
pub trait StrExt {
    /// 文本非空。
    fn is_not_empty(&self) -> bool;

    /// 空文本返回 `None`，否则返回自身。
    fn non_empty(&self) -> Option<&str>;

    /// 去除首尾空白；空串或全空白返回 `None`。
    fn trim_non_empty(&self) -> Option<&str>;

    /// 校验字符串是否符合小写蛇形机器标识规则。
    ///
    /// 规则如下：
    /// - 首字符必须是小写英文字母
    /// - 后续字符只允许小写英文字母、数字或下划线
    fn is_snake_lower_alphanumeric(&self) -> bool;
}

impl StrExt for str {
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    fn non_empty(&self) -> Option<&str> {
        self.is_not_empty().then_some(self)
    }

    fn trim_non_empty(&self) -> Option<&str> {
        self.trim().non_empty()
    }

    fn is_snake_lower_alphanumeric(&self) -> bool {
        let mut chars = self.chars();

        let Some(first) = chars.next() else {
            return false;
        };

        if !first.is_ascii_lowercase() {
            return false;
        }

        chars.all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_not_empty_treats_whitespace_as_text() {
        assert!(!"".is_not_empty());
        assert!("a".is_not_empty());
        assert!("   ".is_not_empty());
    }

    #[test]
    fn non_empty_returns_self_or_none() {
        assert_eq!("".non_empty(), None);
        assert_eq!("a".non_empty(), Some("a"));

        let owned = String::from("a");
        assert_eq!(owned.non_empty(), Some("a"));
    }

    #[test]
    fn trim_non_empty_strips_whitespace() {
        assert_eq!("  a  ".trim_non_empty(), Some("a"));
        assert_eq!("".trim_non_empty(), None);
        assert_eq!("   ".trim_non_empty(), None);
    }

    #[test]
    fn snake_lower_alphanumeric_rejects_empty_string() {
        assert!(!"".is_snake_lower_alphanumeric());
    }

    #[test]
    fn snake_lower_alphanumeric_rejects_non_lowercase_first_char() {
        assert!(!"Abc".is_snake_lower_alphanumeric());
        assert!(!"1abc".is_snake_lower_alphanumeric());
        assert!(!"_abc".is_snake_lower_alphanumeric());
    }

    #[test]
    fn snake_lower_alphanumeric_accepts_valid_identifiers() {
        assert!("abc".is_snake_lower_alphanumeric());
        assert!("a".is_snake_lower_alphanumeric());
        assert!("a1_b2".is_snake_lower_alphanumeric());
        assert!("abc_".is_snake_lower_alphanumeric());
    }

    #[test]
    fn snake_lower_alphanumeric_rejects_invalid_characters() {
        assert!(!"abc-def".is_snake_lower_alphanumeric());
        assert!(!"abc def".is_snake_lower_alphanumeric());
        assert!(!"abcDef".is_snake_lower_alphanumeric());
        assert!(!"中文".is_snake_lower_alphanumeric());
    }
}
