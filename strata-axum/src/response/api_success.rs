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
                timestamp: body.timestamp,
                data: Some(data),
            },
        }
    }
}

impl<T> ApiSuccess<T> {
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

    /// 统一响应体。
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

#[cfg(test)]
mod tests {
    use axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
    };
    use http_body_util::BodyExt;
    use serde_json::Value;

    use super::*;

    async fn body_json(response: Response) -> Value {
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&bytes).unwrap()
    }

    #[tokio::test]
    async fn ok_returns_200_with_success_envelope() {
        let response = ApiSuccess::ok().into_response();
        assert_eq!(response.status(), StatusCode::OK);

        let body = body_json(response).await;
        assert_eq!(body["success"], true);
        assert_eq!(body["code"], "0");
        assert_eq!(body["message"], "ok");
        assert!(body.get("data").is_none());

        let timestamp = body["timestamp"].as_str().unwrap();
        assert!(timestamp.ends_with("+08:00"));
    }

    #[tokio::test]
    async fn created_and_accepted_return_expected_status_and_message() {
        let response = ApiSuccess::created().into_response();
        assert_eq!(response.status(), StatusCode::CREATED);
        let body = body_json(response).await;
        assert_eq!(body["message"], "created");

        let response = ApiSuccess::accepted().into_response();
        assert_eq!(response.status(), StatusCode::ACCEPTED);
        let body = body_json(response).await;
        assert_eq!(body["message"], "accepted");
    }

    #[tokio::test]
    async fn with_data_preserves_envelope_and_adds_data() {
        let response = ApiSuccess::ok().with_data(42).into_response();
        assert_eq!(response.status(), StatusCode::OK);

        let body = body_json(response).await;
        assert_eq!(body["success"], true);
        assert_eq!(body["code"], "0");
        assert_eq!(body["message"], "ok");
        assert!(body.get("timestamp").is_some());
        assert_eq!(body["data"], 42);
    }

    #[test]
    fn with_status_overrides_status() {
        let success = ApiSuccess::ok().with_status(StatusCode::CREATED);
        assert_eq!(success.status(), StatusCode::CREATED);
    }

    #[test]
    fn with_code_and_message_override_envelope() {
        let success = ApiSuccess::ok().with_code("C1").with_message("hello");
        assert_eq!(success.body().success, true);
        assert_eq!(success.body().code, "C1");
        assert_eq!(success.body().message, "hello");
        assert!(success.body().timestamp.is_some());
        assert!(success.body().data.is_none());
    }
}
