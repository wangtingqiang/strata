use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{
    error::{ErrorInfo, ErrorKind},
    http::api::{ResponseBody, body_factory::failure_body},
};

/// 统一失败响应类型，保证 API 错误体结构稳定。
#[derive(Debug)]
pub struct ApiFailure {
    status: StatusCode,
    body: ResponseBody<()>,
}

impl ApiFailure {
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

    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.body.code = code.into();
        self
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.body.message = message.into();
        self
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn code(&self) -> &str {
        &self.body.code
    }

    pub fn message(&self) -> &str {
        &self.body.message
    }
}

impl IntoResponse for ApiFailure {
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
}

impl<E: ErrorInfo> From<E> for ApiFailure {
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
                http.response.status_code = status.as_u16(),
                error.kind = %kind,
                error.code = code,
                error.message = message,
                %error,
                "request failed"
            );
        } else {
            tracing::warn!(
                http.response.status_code = status.as_u16(),
                error.kind = %kind,
                error.code = code,
                error.message = message,
                %error,
                "request failed"
            );
        }

        Self::new(status, code, message)
    }
}
