use secrecy::{ExposeSecret, SecretString};

/// 序列化辅助：暴露 SecretString 原文。
pub fn serialize_secret_string<S>(secret: &SecretString, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(secret.expose_secret())
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::*;

    #[derive(Serialize)]
    struct Helper {
        #[serde(serialize_with = "serialize_secret_string")]
        secret: SecretString,
    }

    #[test]
    fn serializes_exposed_value() {
        let helper = Helper {
            secret: SecretString::from("test-secret"),
        };

        assert_eq!(
            serde_json::to_string(&helper).unwrap(),
            r#"{"secret":"test-secret"}"#
        );
    }
}
