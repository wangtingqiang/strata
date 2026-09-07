use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use strata_error::{ErrorInfo, ErrorKind};

use crate::response::{ResponseBody, body_factory::failure_body};

/// 统一失败响应类型，保证 API 错误体结构稳定。
#[derive(Debug)]
pub struct ApiFailure {
    status: StatusCode,
    body: ResponseBody<()>,
}

impl ApiFailure {
    /// 构建带指定状态码、错误码与消息的失败响应。
    pub fn new(status: StatusCode, code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            status,
            body: failure_body(code, message),
        }
    }

    /// 400
    pub fn bad_request(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, code, message)
    }

    /// 401
    pub fn unauthorized(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, code, message)
    }

    /// 403
    pub fn forbidden(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusCode::FORBIDDEN, code, message)
    }

    /// 404
    pub fn not_found(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, code, message)
    }

    /// 409
    pub fn conflict(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusCode::CONFLICT, code, message)
    }

    /// 422
    pub fn unprocessable_entity(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNPROCESSABLE_ENTITY, code, message)
    }

    /// 429
    pub fn too_many_requests(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusCode::TOO_MANY_REQUESTS, code, message)
    }

    /// 500
    pub fn internal_server_error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, code, message)
    }

    /// 503
    pub fn service_unavailable(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusCode::SERVICE_UNAVAILABLE, code, message)
    }

    /// 覆盖 HTTP 状态码。
    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    /// 覆盖业务错误码。
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.body.code = code.into();
        self
    }

    /// 覆盖消息。
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.body.message = message.into();
        self
    }

    /// HTTP 状态码。
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// 业务错误码。
    pub fn code(&self) -> &str {
        &self.body.code
    }

    /// 消息。
    pub fn message(&self) -> &str {
        &self.body.message
    }
}

impl IntoResponse for ApiFailure {
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
}

impl<E: ErrorInfo + std::fmt::Display> From<E> for ApiFailure {
    fn from(error: E) -> Self {
        let kind = error.kind();
        let code = error.code();
        let message = error.message();

        let status = match kind {
            ErrorKind::Unauthenticated => StatusCode::UNAUTHORIZED,
            ErrorKind::AccessDenied => StatusCode::FORBIDDEN,
            ErrorKind::Validation => StatusCode::BAD_REQUEST,
            ErrorKind::Business => StatusCode::UNPROCESSABLE_ENTITY,
            ErrorKind::NotFound => StatusCode::NOT_FOUND,
            ErrorKind::Conflict => StatusCode::CONFLICT,
            ErrorKind::RateLimited => StatusCode::TOO_MANY_REQUESTS,
            ErrorKind::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::Technical => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
        };

        if status.is_server_error() {
            tracing::error!(
                error.kind = %kind,
                error.code = %code,
                error.message = %message,
                error.detail = %error,
                "server error"
            );
        } else {
            tracing::warn!(
                error.kind = %kind,
                error.code = %code,
                error.message = %message,
                error.detail = %error,
                "client error"
            );
        }

        Self::new(status, code, message)
    }
}

#[cfg(test)]
mod tests {
    use http_body_util::BodyExt;
    use serde_json::Value;

    use super::*;

    async fn body_json(response: Response) -> Value {
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&bytes).unwrap()
    }

    #[derive(Debug, strata_error::ErrorInfo)]
    enum TestError {
        #[info(kind = "Unauthenticated", code = "E001", message = "unauthenticated")]
        Unauthenticated,
        #[info(kind = "AccessDenied", code = "E002", message = "access denied")]
        AccessDenied,
        #[info(kind = "Validation", code = "E003", message = "validation error")]
        Validation,
        #[info(kind = "Business", code = "E004", message = "business error")]
        Business,
        #[info(kind = "NotFound", code = "E005", message = "not found")]
        NotFound,
        #[info(kind = "Conflict", code = "E006", message = "conflict")]
        Conflict,
        #[info(kind = "RateLimited", code = "E007", message = "rate limited")]
        RateLimited,
        #[info(kind = "Internal", code = "E008", message = "internal error")]
        Internal,
        #[info(kind = "Technical", code = "E009", message = "technical error")]
        Technical,
        #[info(kind = "Unexpected", code = "E010", message = "unexpected error")]
        Unexpected,
    }

    impl std::fmt::Display for TestError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("test error")
        }
    }

    #[test]
    fn maps_kind_to_status_code() {
        let cases = [
            (TestError::Unauthenticated, StatusCode::UNAUTHORIZED),
            (TestError::AccessDenied, StatusCode::FORBIDDEN),
            (TestError::Validation, StatusCode::BAD_REQUEST),
            (TestError::Business, StatusCode::UNPROCESSABLE_ENTITY),
            (TestError::NotFound, StatusCode::NOT_FOUND),
            (TestError::Conflict, StatusCode::CONFLICT),
            (TestError::RateLimited, StatusCode::TOO_MANY_REQUESTS),
            (TestError::Internal, StatusCode::INTERNAL_SERVER_ERROR),
            (TestError::Technical, StatusCode::INTERNAL_SERVER_ERROR),
            (TestError::Unexpected, StatusCode::INTERNAL_SERVER_ERROR),
        ];

        for (error, expected_status) in cases {
            let expected_code = error.code();
            let expected_message = error.message();

            let failure = ApiFailure::from(error);

            assert_eq!(failure.status(), expected_status);
            assert_eq!(failure.code(), expected_code);
            assert_eq!(failure.message(), expected_message);
        }
    }

    #[test]
    fn constructor_helpers_map_to_expected_status() {
        let cases = [
            (
                ApiFailure::bad_request("E400", "bad"),
                StatusCode::BAD_REQUEST,
            ),
            (
                ApiFailure::unauthorized("E401", "unauthorized"),
                StatusCode::UNAUTHORIZED,
            ),
            (
                ApiFailure::forbidden("E403", "forbidden"),
                StatusCode::FORBIDDEN,
            ),
            (
                ApiFailure::not_found("E404", "not found"),
                StatusCode::NOT_FOUND,
            ),
            (
                ApiFailure::conflict("E409", "conflict"),
                StatusCode::CONFLICT,
            ),
            (
                ApiFailure::unprocessable_entity("E422", "unprocessable"),
                StatusCode::UNPROCESSABLE_ENTITY,
            ),
            (
                ApiFailure::too_many_requests("E429", "too many"),
                StatusCode::TOO_MANY_REQUESTS,
            ),
            (
                ApiFailure::internal_server_error("E500", "internal"),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            (
                ApiFailure::service_unavailable("E503", "unavailable"),
                StatusCode::SERVICE_UNAVAILABLE,
            ),
        ];

        for (failure, expected_status) in cases {
            assert_eq!(failure.status(), expected_status);
        }
    }

    #[test]
    fn constructors_carry_code_and_message() {
        let failure = ApiFailure::not_found("USER_NOT_FOUND", "用户不存在");
        assert_eq!(failure.code(), "USER_NOT_FOUND");
        assert_eq!(failure.message(), "用户不存在");
    }

    #[test]
    fn with_overrides_apply() {
        let failure = ApiFailure::bad_request("A", "a")
            .with_status(StatusCode::CONFLICT)
            .with_code("B")
            .with_message("b");

        assert_eq!(failure.status(), StatusCode::CONFLICT);
        assert_eq!(failure.code(), "B");
        assert_eq!(failure.message(), "b");
    }

    #[tokio::test]
    async fn into_response_has_failure_envelope_shape() {
        let response = ApiFailure::bad_request("E400", "bad request").into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = body_json(response).await;
        assert_eq!(body["success"], false);
        assert_eq!(body["code"], "E400");
        assert_eq!(body["message"], "bad request");
        assert!(body.get("data").is_none());

        let timestamp = body["timestamp"].as_str().unwrap();
        assert!(timestamp.ends_with("+08:00"));
    }
}
