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
