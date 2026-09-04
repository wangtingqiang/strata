use axum::extract::{FromRequest, Request, rejection::JsonRejection};

use crate::api::ApiFailure;

pub struct Json<T>(pub T);

impl<T> Json<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<S, T> FromRequest<S> for Json<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = ApiFailure;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let value = axum::Json::<T>::from_request(req, state)
            .await
            .map_err(|rejection| {
                let code = match rejection {
                    JsonRejection::JsonDataError(_) => "JSON_DATA_ERROR",
                    JsonRejection::JsonSyntaxError(_) => "JSON_SYNTAX_ERROR",
                    JsonRejection::MissingJsonContentType(_) => "MISSING_JSON_CONTENT_TYPE",
                    JsonRejection::BytesRejection(_) => "BYTES_REJECTION",
                    _ => "JSON_REJECTION",
                };

                tracing::debug!(
                    rejection.kind = %code,
                    rejection.detail = %rejection.body_text(),
                    "json extractor rejected"
                );

                ApiFailure::new(rejection.status(), code, "请求参数错误")
            })?;

        Ok(Self(value.0))
    }
}
