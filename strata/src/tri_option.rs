/// 三态可选值：区分未提供、显式空值与有值三种状态。
///
/// 相当于 `Option<Option<T>>` 的可读版本：`Missing` 对应 `None`，
/// `Null` 对应 `Some(None)`，`Value` 对应 `Some(Some(v))`。
///
/// 作为 serde 结构体字段时，需 `use strata::serde::deserialize_tri_option;`
/// 并标注 `#[serde(default, deserialize_with = "deserialize_tri_option")]`；
/// 漏写 `default` 会报 `missing field` 错误，而不是被静默解析为 `Null`。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TriOption<T> {
    /// 未提供该值。
    #[default]
    Missing,
    /// 显式提供了空值（如 JSON 的 `null`）。
    Null,
    /// 提供了值。
    Value(T),
}

impl<T> TriOption<T> {
    /// 转为 `Option<Option<T>>`：Missing→None，Null→Some(None)，Value→Some(Some(v))。
    pub fn into_option(self) -> Option<Option<T>> {
        match self {
            Self::Missing => None,
            Self::Null => Some(None),
            Self::Value(value) => Some(Some(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_missing() {
        assert_eq!(TriOption::<i32>::default(), TriOption::Missing);
    }

    #[test]
    fn into_option_maps_three_states() {
        assert_eq!(TriOption::<i32>::Missing.into_option(), None);
        assert_eq!(TriOption::<i32>::Null.into_option(), Some(None));
        assert_eq!(TriOption::<i32>::Value(7).into_option(), Some(Some(7)));
    }
}
