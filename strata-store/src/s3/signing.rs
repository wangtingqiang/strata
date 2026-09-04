use hmac::{Hmac, KeyInit, Mac};
use sha2::{Digest, Sha256};
use url::Url;

use super::SigningError;

type HmacSha256 = Hmac<Sha256>;

/// 按 AWS SigV4 规范从密钥、日期、地域派生签名密钥（service 固定为 s3），date 需为 YYYYMMDD。
pub fn s3_signing_key(secret_key: &str, date: &str, region: &str) -> Result<Vec<u8>, SigningError> {
    let date_key = HmacSha256::new_from_slice(&[b"AWS4", secret_key.as_bytes()].concat())?
        .chain_update(date.as_bytes())
        .finalize()
        .into_bytes();

    let region_key = HmacSha256::new_from_slice(&date_key)?
        .chain_update(region.as_bytes())
        .finalize()
        .into_bytes();

    let service_key = HmacSha256::new_from_slice(&region_key)?
        .chain_update(b"s3")
        .finalize()
        .into_bytes();

    let signing_key = HmacSha256::new_from_slice(&service_key)?
        .chain_update(b"aws4_request")
        .finalize()
        .into_bytes();

    Ok(signing_key.to_vec())
}

/// 生成 AWS SigV4 Authorization 请求头，amz_date 需为 `YYYYMMDD'T'HHMMSS'Z'` 格式，
/// payload 按 UNSIGNED-PAYLOAD 处理。
pub fn s3_authorization(
    method: &str,
    url: &str,
    amz_date: &str,
    region: &str,
    access_key: &str,
    secret_key: &str,
) -> Result<String, SigningError> {
    if amz_date.len() != 16 {
        return Err(SigningError::InvalidAmzDate);
    }

    let url = Url::parse(url)?;
    let host = url.host_str().ok_or(SigningError::MissingHost)?;
    let host = url
        .port()
        .map_or_else(|| host.to_owned(), |port| format!("{host}:{port}"));
    let path = url.path();

    let date = &amz_date[..8];

    let canonical_request = format!(
        r#"{method}
{path}

host:{host}
x-amz-content-sha256:UNSIGNED-PAYLOAD
x-amz-date:{amz_date}

host;x-amz-content-sha256;x-amz-date
UNSIGNED-PAYLOAD"#,
    );

    let canonical_hash = hex::encode(Sha256::digest(canonical_request.as_bytes()));

    let credential_scope = format!("{date}/{region}/s3/aws4_request");

    let string_to_sign = format!(
        r#"AWS4-HMAC-SHA256
{amz_date}
{credential_scope}
{canonical_hash}"#,
    );

    let signing_key = s3_signing_key(secret_key, date, region)?;
    let signature = HmacSha256::new_from_slice(&signing_key)?
        .chain_update(string_to_sign.as_bytes())
        .finalize()
        .into_bytes();
    let signature = hex::encode(signature);

    let authorization = format!(
        "AWS4-HMAC-SHA256 Credential={access_key}/{credential_scope}, SignedHeaders=host;x-amz-content-sha256;x-amz-date, Signature={signature}"
    );

    Ok(authorization)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ACCESS_KEY: &str = "AKIDEXAMPLE";
    const SECRET_KEY: &str = "wJalrXUtnFEMI/K7MDENG+bPxRfiCYEXAMPLEKEY";

    #[test]
    fn signing_key_matches_known_answer() {
        // AWS SigV4 规范算法（secret=示例凭据, date=20150830, region=us-east-1, service=s3），
        // 期望值由 Python hmac 与 OpenSSL 两个独立实现演算交叉验证得出。
        let key = s3_signing_key(SECRET_KEY, "20150830", "us-east-1").unwrap();

        assert_eq!(
            hex::encode(key),
            "32f78051dcde24c552811d654f4a769112bb834b03975cdd6b1fd7d16248c269"
        );
    }

    #[test]
    fn signing_key_is_deterministic() {
        let first = s3_signing_key(SECRET_KEY, "20150830", "us-east-1").unwrap();
        let second = s3_signing_key(SECRET_KEY, "20150830", "us-east-1").unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn rejects_amz_date_with_wrong_length() {
        let result = s3_authorization(
            "GET",
            "https://example.com/path/to/object",
            "20150830",
            "us-east-1",
            ACCESS_KEY,
            SECRET_KEY,
        );

        assert!(matches!(result, Err(SigningError::InvalidAmzDate)));
    }

    #[test]
    fn rejects_malformed_url() {
        let result = s3_authorization(
            "GET",
            "not a url",
            "20150830T123600Z",
            "us-east-1",
            ACCESS_KEY,
            SECRET_KEY,
        );

        assert!(matches!(result, Err(SigningError::ParseUrl(_))));
    }

    #[test]
    fn rejects_url_without_host() {
        let result = s3_authorization(
            "GET",
            "file:///tmp/object",
            "20150830T123600Z",
            "us-east-1",
            ACCESS_KEY,
            SECRET_KEY,
        );

        assert!(matches!(result, Err(SigningError::MissingHost)));
    }

    #[test]
    fn authorization_matches_known_answer() {
        // 期望值由 Python hmac + OpenSSL 独立演算交叉验证（与 signing_key 同源）。
        let authorization = s3_authorization(
            "GET",
            "https://example.com/path/to/object",
            "20150830T123600Z",
            "us-east-1",
            ACCESS_KEY,
            SECRET_KEY,
        )
        .unwrap();

        assert_eq!(
            authorization,
            "AWS4-HMAC-SHA256 Credential=AKIDEXAMPLE/20150830/us-east-1/s3/aws4_request, \
             SignedHeaders=host;x-amz-content-sha256;x-amz-date, \
             Signature=29a92096cc12c9aaa9f64408afa2f7ad41d491965404b15b14bb88bf77355c40"
        );
    }
}
