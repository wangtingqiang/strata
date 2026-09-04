use axum::{extract::FromRequestParts, http::request::Parts};

use crate::response::ApiFailure;

#[derive(Debug, Clone)]
pub struct Context<T>(pub T);

impl<T> std::ops::Deref for Context<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, S> FromRequestParts<S> for Context<T>
where
    T: Send + Sync + 'static,
    S: Send + Sync,
{
    type Rejection = ApiFailure;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts.extensions.remove::<Context<T>>().ok_or_else(|| {
            ApiFailure::internal_server_error("INTERNAL_ERROR", "系统异常，请稍后再试")
        })
    }
}
