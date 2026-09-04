use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::response::{ResponseBody, body_factory::success_body};

/// API 成功响应，包含 HTTP 状态码与统一响应体。
#[derive(Debug)]
pub struct ApiSuccess<T = ()> {
    status: StatusCode,
    body: ResponseBody<T>,
}

impl ApiSuccess<()> {
    /// 200
    pub fn ok() -> Self {
        Self {
            status: StatusCode::OK,
            body: success_body("ok"),
        }
    }

    /// 201
    pub fn created() -> Self {
        Self {
            status: StatusCode::CREATED,
            body: success_body("created"),
        }
    }

    /// 202
    pub fn accepted() -> Self {
        Self {
            status: StatusCode::ACCEPTED,
            body: success_body("accepted"),
        }
    }

    /// 为无数据成功响应补充 data，并保持已有 code/message/timestamp。
    pub fn with_data<T>(self, data: T) -> ApiSuccess<T> {
        let ApiSuccess { status, body } = self;

        ApiSuccess {
            status,
            body: ResponseBody {
                success: body.success,
                code: body.code,
                message: body.message,
                time: body.time,
                data: Some(data),
            },
        }
    }
}

impl<T> ApiSuccess<T> {
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

    pub fn body(&self) -> &ResponseBody<T> {
        &self.body
    }
}

impl<T> IntoResponse for ApiSuccess<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
}
