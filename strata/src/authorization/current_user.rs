#[derive(Debug, Clone)]
pub struct CurrentUser<T>(pub T);

impl<T> std::ops::Deref for CurrentUser<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "http")]
mod http {
    use axum::{extract::FromRequestParts, http::request::Parts};

    use crate::{authorization::CurrentUser, http::api::ApiFailure};

    impl<T, S> FromRequestParts<S> for CurrentUser<T>
    where
        T: Clone + Send + Sync + 'static,
        S: Send + Sync,
    {
        type Rejection = ApiFailure;

        async fn from_request_parts(
            parts: &mut Parts,
            _state: &S,
        ) -> Result<Self, Self::Rejection> {
            parts
                .extensions
                .get::<CurrentUser<T>>()
                .cloned()
                .ok_or_else(|| {
                    ApiFailure::internal_server_error("INTERNAL_ERROR", "系统异常，请稍后再试")
                })
        }
    }
}
