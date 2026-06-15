use axum::extract::FromRequestParts;
use http::request::Parts;

use crate::http::api::ApiFailure;

#[derive(Debug)]
pub struct AppContext<T>(pub T);

impl<T> std::ops::Deref for AppContext<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, S> FromRequestParts<S> for AppContext<T>
where
    T: Send + Sync + 'static,
    S: Send + Sync,
{
    type Rejection = ApiFailure;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts.extensions.remove::<AppContext<T>>().ok_or_else(|| {
            ApiFailure::internal_server_error("INTERNAL_ERROR", "系统异常，请稍后再试")
        })
    }
}
