#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: String,
}

#[cfg(feature = "http")]
mod http {
    use axum::{extract::FromRequestParts, http::request::Parts};

    use crate::{authorization::CurrentUser, http::api::ApiFailure};

    impl<S> FromRequestParts<S> for CurrentUser
    where
        S: Send + Sync,
    {
        type Rejection = ApiFailure;

        async fn from_request_parts(
            parts: &mut Parts,
            _state: &S,
        ) -> Result<Self, Self::Rejection> {
            parts
                .extensions
                .get::<CurrentUser>()
                .cloned()
                .ok_or_else(|| {
                    ApiFailure::internal_server_error(
                        "INTERNAL_SERVER_ERROR",
                        "系统异常，请稍后再试",
                    )
                })
        }
    }
}
