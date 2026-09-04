use axum::{
    extract::{FromRequestParts, rejection::QueryRejection},
    http::request::Parts,
};

use crate::response::ApiFailure;

/// 查询参数提取器（拒绝时返回统一错误响应）。
pub struct Query<T>(pub T);

impl<T> Query<T> {
    /// 取回内部值。
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<S, T> FromRequestParts<S> for Query<T>
where
    axum::extract::Query<T>: FromRequestParts<S, Rejection = QueryRejection>,
    S: Send + Sync,
{
    type Rejection = ApiFailure;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let value = axum::extract::Query::<T>::from_request_parts(parts, state)
            .await
            .map_err(|rejection| {
                let code = match rejection {
                    QueryRejection::FailedToDeserializeQueryString(_) => {
                        "FAILED_TO_DESERIALIZE_QUERY_STRING"
                    }
                    _ => "QUERY_REJECTION",
                };

                tracing::debug!(
                    rejection.kind = %code,
                    rejection.detail = %rejection.body_text(),
                    "query extractor rejected"
                );

                ApiFailure::new(rejection.status(), code, "请求参数错误")
            })?;

        Ok(Self(value.0))
    }
}
