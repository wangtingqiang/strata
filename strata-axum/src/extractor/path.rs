use axum::{
    extract::{FromRequestParts, rejection::PathRejection},
    http::request::Parts,
};

use crate::response::ApiFailure;

pub struct Path<T>(pub T);

impl<T> Path<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<S, T> FromRequestParts<S> for Path<T>
where
    axum::extract::Path<T>: FromRequestParts<S, Rejection = PathRejection>,
    S: Send + Sync,
{
    type Rejection = ApiFailure;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let value = axum::extract::Path::<T>::from_request_parts(parts, state)
            .await
            .map_err(|rejection| {
                let code = match rejection {
                    PathRejection::FailedToDeserializePathParams(_) => {
                        "FAILED_TO_DESERIALIZE_PATH_PARAMS"
                    }
                    PathRejection::MissingPathParams(_) => "MISSING_PATH_PARAMS",
                    _ => "PATH_REJECTION",
                };

                tracing::debug!(
                    rejection.kind = %code,
                    rejection.detail = %rejection.body_text(),
                    "path extractor rejected"
                );

                ApiFailure::new(rejection.status(), code, "请求参数错误")
            })?;

        Ok(Self(value.0))
    }
}
