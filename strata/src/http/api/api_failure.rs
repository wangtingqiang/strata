use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{
    error::{ErrorInfo, ToErrorInfo},
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

    pub fn from_error_info(status: StatusCode, error: ErrorInfo) -> Self {
        Self::new(status, error.code(), error.message())
    }

    /// 400
    pub fn bad_request(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, code, message)
    }

    pub fn bad_request_from(error: impl ToErrorInfo) -> Self {
        Self::from_error_info(StatusCode::BAD_REQUEST, error.to_error_info())
    }

    /// 401
    pub fn unauthorized(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, code, message)
    }

    pub fn unauthorized_from(error: impl ToErrorInfo) -> Self {
        Self::from_error_info(StatusCode::UNAUTHORIZED, error.to_error_info())
    }

    /// 403
    pub fn forbidden(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusCode::FORBIDDEN, code, message)
    }

    pub fn forbidden_from(error: impl ToErrorInfo) -> Self {
        Self::from_error_info(StatusCode::FORBIDDEN, error.to_error_info())
    }

    /// 404
    pub fn not_found(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, code, message)
    }

    pub fn not_found_from(error: impl ToErrorInfo) -> Self {
        Self::from_error_info(StatusCode::NOT_FOUND, error.to_error_info())
    }

    /// 409
    pub fn conflict(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusCode::CONFLICT, code, message)
    }

    pub fn conflict_from(error: impl ToErrorInfo) -> Self {
        Self::from_error_info(StatusCode::CONFLICT, error.to_error_info())
    }

    /// 422
    pub fn unprocessable_entity(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNPROCESSABLE_ENTITY, code, message)
    }

    pub fn unprocessable_entity_from(error: impl ToErrorInfo) -> Self {
        Self::from_error_info(StatusCode::UNPROCESSABLE_ENTITY, error.to_error_info())
    }

    /// 429
    pub fn too_many_requests(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusCode::TOO_MANY_REQUESTS, code, message)
    }

    pub fn too_many_requests_from(error: impl ToErrorInfo) -> Self {
        Self::from_error_info(StatusCode::TOO_MANY_REQUESTS, error.to_error_info())
    }

    /// 500
    pub fn internal_server_error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, code, message)
    }

    pub fn internal_server_error_from(error: impl ToErrorInfo) -> Self {
        Self::from_error_info(StatusCode::INTERNAL_SERVER_ERROR, error.to_error_info())
    }

    /// 503
    pub fn service_unavailable(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusCode::SERVICE_UNAVAILABLE, code, message)
    }

    pub fn service_unavailable_from(error: impl ToErrorInfo) -> Self {
        Self::from_error_info(StatusCode::SERVICE_UNAVAILABLE, error.to_error_info())
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
