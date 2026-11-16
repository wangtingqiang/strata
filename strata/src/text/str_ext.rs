/// `str` 的便捷方法。
pub trait StrExt {
    /// 文本非空。
    fn is_not_empty(&self) -> bool;

    /// 空文本返回 `None`，否则返回自身。
    fn non_empty(&self) -> Option<&str>;

    /// 去除首尾空白；空串或全空白返回 `None`。
    fn trim_non_empty(&self) -> Option<&str>;
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
}
