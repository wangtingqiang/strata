use axum::extract::{FromRequest, Request, rejection::JsonRejection};

use crate::response::ApiFailure;

/// JSON 请求体提取器（拒绝时返回统一错误响应）。
pub struct Json<T>(pub T);

impl<T> Json<T> {
    /// 取回内部值。
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<S, T> FromRequest<S> for Json<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = ApiFailure;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let value = axum::Json::<T>::from_request(req, state)
            .await
            .map_err(|rejection| {
                let code = match rejection {
                    JsonRejection::JsonDataError(_) => "JSON_DATA_ERROR",
                    JsonRejection::JsonSyntaxError(_) => "JSON_SYNTAX_ERROR",
                    JsonRejection::MissingJsonContentType(_) => "MISSING_JSON_CONTENT_TYPE",
                    JsonRejection::BytesRejection(_) => "BYTES_REJECTION",
                    _ => "JSON_REJECTION",
                };

                tracing::debug!(
                    rejection.kind = %code,
                    rejection.detail = %rejection.body_text(),
                    "json extractor rejected"
                );

                ApiFailure::new(rejection.status(), code, "请求参数错误")
            })?;

        Ok(Self(value.0))
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode, header},
    };
    use serde::Deserialize;

    use super::*;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Payload {
        name: String,
        age: u8,
    }

    fn json_request(body: &str) -> Request<Body> {
        Request::builder()
            .method("POST")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(body.to_owned()))
            .unwrap()
    }

    #[tokio::test]
    async fn extracts_valid_json() {
        let request = json_request(r#"{"name":"alice","age":30}"#);

        let json = Json::<Payload>::from_request(request, &()).await.unwrap();

        assert_eq!(
            json.0,
            Payload {
                name: "alice".to_owned(),
                age: 30
            }
        );
        assert_eq!(json.into_inner().name, "alice");
    }

    #[tokio::test]
    async fn rejects_malformed_json_with_syntax_error() {
        let request = json_request("not json");

        let rejection = Json::<Payload>::from_request(request, &())
            .await
            .err()
            .unwrap();

        assert_eq!(rejection.status(), StatusCode::BAD_REQUEST);
        assert_eq!(rejection.code(), "JSON_SYNTAX_ERROR");
        assert_eq!(rejection.message(), "请求参数错误");
    }

    #[tokio::test]
    async fn rejects_wrong_shape_with_data_error() {
        let request = json_request(r#"{"name":"alice"}"#);

        let rejection = Json::<Payload>::from_request(request, &())
            .await
            .err()
            .unwrap();

        assert_eq!(rejection.status(), StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(rejection.code(), "JSON_DATA_ERROR");
    }

    #[tokio::test]
    async fn rejects_missing_content_type() {
        let request = Request::builder()
            .method("POST")
            .body(Body::from(r#"{"name":"alice","age":30}"#))
            .unwrap();

        let rejection = Json::<Payload>::from_request(request, &())
            .await
            .err()
            .unwrap();

        assert_eq!(rejection.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
        assert_eq!(rejection.code(), "MISSING_JSON_CONTENT_TYPE");
    }
}
