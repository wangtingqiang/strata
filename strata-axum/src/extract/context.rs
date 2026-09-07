use axum::{extract::FromRequestParts, http::request::Parts};

use crate::response::ApiFailure;

/// 请求上下文提取器：从请求扩展中取出预先注入的值。
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

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode, request::Parts},
    };

    use super::*;

    fn parts_with_context<T: Send + Sync + Clone + 'static>(value: T) -> Parts {
        let mut request = Request::new(Body::empty());
        request.extensions_mut().insert(Context(value));
        request.into_parts().0
    }

    #[tokio::test]
    async fn extracts_injected_context() {
        let mut parts = parts_with_context("hello".to_owned());

        let context = Context::<String>::from_request_parts(&mut parts, &())
            .await
            .unwrap();

        assert_eq!(context.0, "hello");
    }

    #[tokio::test]
    async fn missing_context_rejects_with_500() {
        let mut parts = Request::new(Body::empty()).into_parts().0;

        let rejection = Context::<String>::from_request_parts(&mut parts, &())
            .await
            .err()
            .unwrap();

        assert_eq!(rejection.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(rejection.code(), "INTERNAL_ERROR");
        assert_eq!(rejection.message(), "系统异常，请稍后再试");
    }

    #[test]
    fn derefs_to_inner_value() {
        let context = Context("value".to_owned());
        let inner: &String = &context;

        assert_eq!(inner, "value");
    }
}
