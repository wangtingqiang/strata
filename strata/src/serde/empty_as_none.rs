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
