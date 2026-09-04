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
