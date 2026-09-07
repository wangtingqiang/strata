use serde::{Deserialize, Deserializer};

/// 反序列化辅助：空字符串按缺失处理（u64）。
pub fn deserialize_optional_u64_or_empty<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;
    match value {
        None => Ok(None),
        Some(value) => {
            let normalized = value.trim();
            if normalized.is_empty() {
                return Ok(None);
            }

            normalized
                .parse::<u64>()
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
    }
}

/// 反序列化辅助：空字符串按缺失处理（String）。
pub fn deserialize_optional_string_or_empty<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;
    match value {
        None => Ok(None),
        Some(value) => {
            let normalized = value.trim();
            if normalized.is_empty() {
                return Ok(None);
            }

            Ok(Some(normalized.to_owned()))
        }
    }
}

/// 反序列化辅助：空字符串按缺失处理（bool）。
pub fn deserialize_optional_bool_or_empty<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;
    match value {
        None => Ok(None),
        Some(value) => {
            let normalized = value.trim();
            if normalized.is_empty() {
                return Ok(None);
            }

            normalized
                .parse::<bool>()
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::json;

    use super::*;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Record {
        #[serde(default, deserialize_with = "deserialize_optional_u64_or_empty")]
        count: Option<u64>,
        #[serde(default, deserialize_with = "deserialize_optional_string_or_empty")]
        name: Option<String>,
        #[serde(default, deserialize_with = "deserialize_optional_bool_or_empty")]
        enabled: Option<bool>,
    }

    #[test]
    fn missing_fields_default_to_none() {
        let record: Record = serde_json::from_value(json!({})).unwrap();
        assert_eq!(
            record,
            Record {
                count: None,
                name: None,
                enabled: None,
            }
        );
    }

    #[test]
    fn explicit_null_maps_to_none() {
        let record: Record =
            serde_json::from_value(json!({ "count": null, "name": null, "enabled": null }))
                .unwrap();
        assert_eq!(
            record,
            Record {
                count: None,
                name: None,
                enabled: None,
            }
        );
    }

    #[test]
    fn empty_strings_map_to_none() {
        let record: Record =
            serde_json::from_value(json!({ "count": "", "name": "", "enabled": "" })).unwrap();
        assert_eq!(
            record,
            Record {
                count: None,
                name: None,
                enabled: None,
            }
        );
    }

    #[test]
    fn whitespace_only_strings_map_to_none() {
        let record: Record = serde_json::from_value(json!({
            "count": "  ",
            "name": " \t ",
            "enabled": "  ",
        }))
        .unwrap();
        assert_eq!(
            record,
            Record {
                count: None,
                name: None,
                enabled: None,
            }
        );
    }

    #[test]
    fn valid_values_are_parsed_and_trimmed() {
        let record: Record = serde_json::from_value(json!({
            "count": "42",
            "name": "  hello world ",
            "enabled": "true",
        }))
        .unwrap();
        assert_eq!(
            record,
            Record {
                count: Some(42),
                name: Some("hello world".to_owned()),
                enabled: Some(true),
            }
        );
    }

    #[test]
    fn bool_false_is_parsed() {
        let record: Record = serde_json::from_value(json!({ "enabled": "false" })).unwrap();
        assert_eq!(
            record,
            Record {
                count: None,
                name: None,
                enabled: Some(false),
            }
        );
    }

    #[test]
    fn invalid_u64_fails_deserialization() {
        let result: Result<Record, _> = serde_json::from_value(json!({ "count": "abc" }));
        assert!(result.is_err());
    }

    #[test]
    fn invalid_bool_fails_deserialization() {
        let result: Result<Record, _> = serde_json::from_value(json!({ "enabled": "yes" }));
        assert!(result.is_err());
    }
}
