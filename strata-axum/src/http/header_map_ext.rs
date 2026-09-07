use axum::http::{HeaderMap, header};

use crate::response::ApiFailure;

/// HeaderMap 扩展。
pub trait HeaderMapExt {
    /// 从 Authorization 头解析 Bearer token。
    fn bearer_token(&self) -> Result<String, ApiFailure>;
}

impl HeaderMapExt for HeaderMap {
    fn bearer_token(&self) -> Result<String, ApiFailure> {
        let value = self
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| ApiFailure::unauthorized("MISSING_AUTHORIZATION", "缺少认证信息"))?;

        let (scheme, token) = value
            .split_once(' ')
            .ok_or_else(|| ApiFailure::unauthorized("INVALID_AUTHORIZATION", "认证信息无效"))?;

        if !scheme.eq_ignore_ascii_case("bearer") || token.is_empty() {
            return Err(ApiFailure::unauthorized(
                "INVALID_AUTHORIZATION",
                "认证信息无效",
            ));
        }

        Ok(token.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use axum::http::{HeaderMap, HeaderValue, StatusCode, header};

    use super::*;

    fn header_map(authorization: Option<&str>) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if let Some(value) = authorization {
            headers.insert(header::AUTHORIZATION, HeaderValue::from_str(value).unwrap());
        }
        headers
    }

    fn assert_unauthorized(failure: ApiFailure, expected_code: &str) {
        assert_eq!(failure.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(failure.code(), expected_code);
    }

    #[test]
    fn missing_authorization_header_rejects() {
        let rejection = HeaderMap::new().bearer_token().unwrap_err();
        assert_unauthorized(rejection, "MISSING_AUTHORIZATION");
    }

    #[test]
    fn non_utf8_authorization_header_rejects_as_missing() {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_bytes(&[0xff, 0xfe]).unwrap(),
        );

        let rejection = headers.bearer_token().unwrap_err();

        assert_unauthorized(rejection, "MISSING_AUTHORIZATION");
    }

    #[test]
    fn header_without_scheme_rejects() {
        let rejection = header_map(Some("plain-token")).bearer_token().unwrap_err();
        assert_unauthorized(rejection, "INVALID_AUTHORIZATION");
    }

    #[test]
    fn wrong_scheme_rejects() {
        let rejection = header_map(Some("Basic abc-123"))
            .bearer_token()
            .unwrap_err();
        assert_unauthorized(rejection, "INVALID_AUTHORIZATION");
    }

    #[test]
    fn empty_token_rejects() {
        let rejection = header_map(Some("Bearer ")).bearer_token().unwrap_err();
        assert_unauthorized(rejection, "INVALID_AUTHORIZATION");
    }

    #[test]
    fn bearer_scheme_is_case_insensitive() {
        for value in ["Bearer abc-123", "bearer abc-123", "BEARER abc-123"] {
            let token = header_map(Some(value)).bearer_token().unwrap();
            assert_eq!(token, "abc-123");
        }
    }

    #[test]
    fn valid_bearer_token_extracted() {
        let token = header_map(Some("Bearer abc-123")).bearer_token().unwrap();
        assert_eq!(token, "abc-123");
    }
}
