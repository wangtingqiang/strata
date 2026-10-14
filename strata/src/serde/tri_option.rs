use serde::{Deserialize, Deserializer};

use crate::tri_option::TriOption;

// 故意不为 TriOption 实现 Deserialize，因为类型内无法区分缺失与显式 null，导致漏写 #[serde(default)] 时缺失会被静默解析为 Null。
/// 反序列化辅助：显式 `null` 映射为 `Null`，有值映射为 `Value`。
///
/// 字段缺失由字段上的 `#[serde(default)]` 映射为 `Missing`（`Default` 为
/// `Missing`）；必须与 `#[serde(default)]` 搭配使用，漏写时缺失字段会报
/// `missing field`，而不是被静默解析为 `Null`。
pub fn deserialize_tri_option<'de, D, T>(deserializer: D) -> Result<TriOption<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let value = Option::<T>::deserialize(deserializer)?;
    Ok(match value {
        Some(value) => TriOption::Value(value),
        None => TriOption::Null,
    })
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::json;

    use crate::tri_option::TriOption;

    use super::deserialize_tri_option;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Entry {
        #[serde(default, deserialize_with = "deserialize_tri_option")]
        nickname: TriOption<String>,
    }

    #[derive(Deserialize)]
    struct EntryWithoutDefault {
        #[serde(deserialize_with = "deserialize_tri_option")]
        nickname: TriOption<String>,
    }

    #[test]
    fn missing_field_is_missing() {
        let entry: Entry = serde_json::from_value(json!({})).unwrap();
        assert_eq!(entry.nickname, TriOption::Missing);
    }

    #[test]
    fn explicit_null_is_null() {
        let entry: Entry = serde_json::from_value(json!({ "nickname": null })).unwrap();
        assert_eq!(entry.nickname, TriOption::Null);
    }

    #[test]
    fn value_is_value() {
        let entry: Entry = serde_json::from_value(json!({ "nickname": "alice" })).unwrap();
        assert_eq!(entry.nickname, TriOption::Value("alice".to_owned()));
    }

    #[test]
    fn missing_field_without_default_fails() {
        let error = match serde_json::from_value::<EntryWithoutDefault>(json!({})) {
            Ok(entry) => panic!(
                "漏写 #[serde(default)] 时应报 missing field，实际得到 {:?}",
                entry.nickname
            ),
            Err(error) => error,
        };
        assert!(error.to_string().contains("missing field"));
    }
}
