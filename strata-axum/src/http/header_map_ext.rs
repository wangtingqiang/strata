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

        let mut parts = value.split_ascii_whitespace();
        let scheme = parts
            .next()
            .ok_or_else(|| ApiFailure::unauthorized("INVALID_AUTHORIZATION", "认证信息无效"))?;
        let token = parts
            .next()
            .ok_or_else(|| ApiFailure::unauthorized("INVALID_AUTHORIZATION", "认证信息无效"))?;

        if !scheme.eq_ignore_ascii_case("bearer") || parts.next().is_some() {
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
    fn whitespace_only_token_rejects() {
        for value in ["Bearer  ", "Bearer   ", "Bearer\t\t"] {
            let rejection = header_map(Some(value)).bearer_token().unwrap_err();
            assert_unauthorized(rejection, "INVALID_AUTHORIZATION");
        }
    }

    #[test]
    fn multiple_spaces_around_token_are_accepted() {
        for value in ["Bearer  abc-123", "Bearer   abc-123", "  Bearer  abc-123  "] {
            let token = header_map(Some(value)).bearer_token().unwrap();
            assert_eq!(token, "abc-123");
        }
    }

    #[test]
    fn trailing_whitespace_is_ignored() {
        for value in ["Bearer abc-123 ", "Bearer abc-123\t"] {
            let token = header_map(Some(value)).bearer_token().unwrap();
            assert_eq!(token, "abc-123");
        }
    }

    #[test]
    fn extra_token_fields_reject() {
        let rejection = header_map(Some("Bearer abc-123 def-456"))
            .bearer_token()
            .unwrap_err();
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
