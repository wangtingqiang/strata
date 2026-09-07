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

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode, request::Parts},
    };
    use serde::Deserialize;

    use super::*;

    #[derive(Debug, Deserialize, PartialEq)]
    struct PageQuery {
        page: u32,
        size: u32,
    }

    fn parts_with_query(query: &str) -> Parts {
        Request::builder()
            .uri(format!("/items{query}"))
            .body(Body::empty())
            .unwrap()
            .into_parts()
            .0
    }

    #[tokio::test]
    async fn extracts_valid_query_params() {
        let mut parts = parts_with_query("?page=2&size=10");

        let query = Query::<PageQuery>::from_request_parts(&mut parts, &())
            .await
            .unwrap();

        assert_eq!(query.0, PageQuery { page: 2, size: 10 });
        assert_eq!(query.into_inner().page, 2);
    }

    #[tokio::test]
    async fn rejects_missing_required_field() {
        let mut parts = parts_with_query("?page=2");

        let rejection = Query::<PageQuery>::from_request_parts(&mut parts, &())
            .await
            .err()
            .unwrap();

        assert_eq!(rejection.status(), StatusCode::BAD_REQUEST);
        assert_eq!(rejection.code(), "FAILED_TO_DESERIALIZE_QUERY_STRING");
    }

    #[tokio::test]
    async fn rejects_wrong_type() {
        let mut parts = parts_with_query("?page=abc&size=10");

        let rejection = Query::<PageQuery>::from_request_parts(&mut parts, &())
            .await
            .err()
            .unwrap();

        assert_eq!(rejection.status(), StatusCode::BAD_REQUEST);
        assert_eq!(rejection.code(), "FAILED_TO_DESERIALIZE_QUERY_STRING");
    }
}
