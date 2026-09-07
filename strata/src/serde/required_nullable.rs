use serde::{Deserialize, Deserializer};

/// 区分字段缺失与显式 `null` 的三态容器。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RequiredNullable<T> {
    /// 字段缺失。
    #[default]
    Missing,
    /// 显式 `null`。
    Null,
    /// 有值。
    Value(T),
}

impl<T> RequiredNullable<T> {
    /// 转为 `Option<Option<T>>`：Missing→None，Null→Some(None)，Value→Some(Some(v))。
    pub fn into_option(self) -> Option<Option<T>> {
        match self {
            Self::Missing => None,
            Self::Null => Some(None),
            Self::Value(value) => Some(Some(value)),
        }
    }
}

/// 反序列化辅助：字段缺失与显式 `null` 分别映射为 Missing 与 Null。
pub fn deserialize_required_nullable<'de, D, T>(
    deserializer: D,
) -> Result<RequiredNullable<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let value = Option::<T>::deserialize(deserializer)?;
    Ok(match value {
        Some(value) => RequiredNullable::Value(value),
        None => RequiredNullable::Null,
    })
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::json;

    use super::*;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Entry {
        #[serde(default, deserialize_with = "deserialize_required_nullable")]
        nickname: RequiredNullable<String>,
    }

    #[test]
    fn missing_field_is_missing() {
        let entry: Entry = serde_json::from_value(json!({})).unwrap();
        assert_eq!(entry.nickname, RequiredNullable::Missing);
    }

    #[test]
    fn explicit_null_is_null() {
        let entry: Entry = serde_json::from_value(json!({ "nickname": null })).unwrap();
        assert_eq!(entry.nickname, RequiredNullable::Null);
    }

    #[test]
    fn value_is_value() {
        let entry: Entry = serde_json::from_value(json!({ "nickname": "alice" })).unwrap();
        assert_eq!(entry.nickname, RequiredNullable::Value("alice".to_owned()));
    }

    #[test]
    fn into_option_maps_three_states() {
        assert_eq!(RequiredNullable::<i32>::Missing.into_option(), None);
        assert_eq!(RequiredNullable::<i32>::Null.into_option(), Some(None));
        assert_eq!(
            RequiredNullable::<i32>::Value(7).into_option(),
            Some(Some(7))
        );
    }
}
