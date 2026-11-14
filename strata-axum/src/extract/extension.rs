use axum::{
    extract::{FromRequestParts, rejection::ExtensionRejection},
    http::request::Parts,
};

use crate::response::ApiFailure;

/// Extension 提取器（clone）：从请求扩展中克隆预先注入的值。
pub struct Extension<T>(pub T);

impl<T> Extension<T> {
    /// 取回内部值。
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T, S> FromRequestParts<S> for Extension<T>
where
    axum::extract::Extension<T>: FromRequestParts<S, Rejection = ExtensionRejection>,
    S: Send + Sync,
{
    type Rejection = ApiFailure;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let value = axum::extract::Extension::<T>::from_request_parts(parts, state)
            .await
            .map_err(|_| {
                tracing::debug!(
                    extension.type = %std::any::type_name::<T>(),
                    rejection.detail = %"Missing request extension",
                    "extension extractor rejected"
                );

                ApiFailure::internal_server_error("INTERNAL_ERROR", "系统异常，请稍后再试")
            })?;

        Ok(Self(value.0))
    }
}

/// Extension 提取器（remove）：从请求扩展中移除值并取得所有权，同一请求只能提取一次。
pub struct TakeExtension<T>(pub T);

impl<T> TakeExtension<T> {
    /// 取回内部值。
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T, S> FromRequestParts<S> for TakeExtension<T>
where
    T: Send + Sync + 'static,
    S: Send + Sync,
{
    type Rejection = ApiFailure;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts.extensions.remove::<T>().map(Self).ok_or_else(|| {
            tracing::debug!(
                extension.type = %std::any::type_name::<T>(),
                rejection.detail = %"Missing request extension",
                "extension extractor rejected"
            );

            ApiFailure::internal_server_error("INTERNAL_ERROR", "系统异常，请稍后再试")
        })
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };

    use super::*;

    fn parts_with_extension<T: Clone + Send + Sync + 'static>(value: T) -> Parts {
        let mut request = Request::new(Body::empty());
        request.extensions_mut().insert(value);
        request.into_parts().0
    }

    fn assert_internal_error(failure: &ApiFailure) {
        assert_eq!(failure.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(failure.code(), "INTERNAL_ERROR");
        assert_eq!(failure.message(), "系统异常，请稍后再试");
    }

    #[tokio::test]
    async fn extension_extracts_injected_value() {
        let mut parts = parts_with_extension("hello".to_owned());

        let extension = Extension::<String>::from_request_parts(&mut parts, &())
            .await
            .unwrap();

        assert_eq!(extension.0, "hello");
        assert_eq!(extension.into_inner(), "hello");
    }

    #[tokio::test]
    async fn extension_can_be_extracted_multiple_times() {
        let mut parts = parts_with_extension("hello".to_owned());

        let first = Extension::<String>::from_request_parts(&mut parts, &())
            .await
            .unwrap();
        let second = Extension::<String>::from_request_parts(&mut parts, &())
            .await
            .unwrap();

        assert_eq!(first.0, second.0);
    }

    #[tokio::test]
    async fn missing_extension_rejects_with_internal_error() {
        let mut parts = Request::new(Body::empty()).into_parts().0;

        let rejection = Extension::<String>::from_request_parts(&mut parts, &())
            .await
            .err()
            .unwrap();

        assert_internal_error(&rejection);
    }

    #[tokio::test]
    async fn take_extension_moves_value_out() {
        let mut parts = parts_with_extension("hello".to_owned());

        let taken = TakeExtension::<String>::from_request_parts(&mut parts, &())
            .await
            .unwrap();

        assert_eq!(taken.into_inner(), "hello");
    }

    #[tokio::test]
    async fn take_extension_can_only_be_extracted_once() {
        let mut parts = parts_with_extension("hello".to_owned());

        let first = TakeExtension::<String>::from_request_parts(&mut parts, &())
            .await
            .unwrap();
        assert_eq!(first.0, "hello");

        let rejection = TakeExtension::<String>::from_request_parts(&mut parts, &())
            .await
            .err()
            .unwrap();
        assert_internal_error(&rejection);
    }

    #[tokio::test]
    async fn missing_take_extension_rejects_with_internal_error() {
        let mut parts = Request::new(Body::empty()).into_parts().0;

        let rejection = TakeExtension::<String>::from_request_parts(&mut parts, &())
            .await
            .err()
            .unwrap();

        assert_internal_error(&rejection);
    }
}
