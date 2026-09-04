use hmac::{Hmac, KeyInit, Mac};
use secrecy::{ExposeSecret, SecretString};
use sha2::{Digest, Sha256};
use url::Url;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, thiserror::Error)]
pub enum SigningError {
    #[error("failed to build hmac")]
    BuildHmac(#[from] hmac::digest::InvalidLength),
    #[error("failed to parse url")]
    ParseUrl(#[from] url::ParseError),
    #[error("url must have a host")]
    MissingHost,
    #[error("invalid amz_date format")]
    InvalidAmzDate,
}

pub fn s3_signing_key(
    secret_key: &SecretString,
    date: &str,
    region: &str,
) -> Result<Vec<u8>, SigningError> {
    let secret_key = secret_key.expose_secret();
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

pub fn s3_authorization(
    method: &str,
    url: &str,
    amz_date: &str,
    region: &str,
    access_key: &SecretString,
    secret_key: &SecretString,
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

    let access_key = access_key.expose_secret();
    let authorization = format!(
        "AWS4-HMAC-SHA256 Credential={access_key}/{credential_scope}, SignedHeaders=host;x-amz-content-sha256;x-amz-date, Signature={signature}"
    );

    Ok(authorization)
}
