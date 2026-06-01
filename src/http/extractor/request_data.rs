use axum::{
    extract::{
        FromRequest, FromRequestParts, Request,
        rejection::{JsonRejection, PathRejection, QueryRejection},
    },
    http::request::Parts,
};

use crate::http::api::ApiFailure;

const BAD_REQUEST_CODE: &str = "BAD_REQUEST";

/// JSON 请求体提取器，统一将解析失败映射为业务错误结构。
pub struct Json<T>(pub T);

/// 路径参数提取器，统一将解析失败映射为业务错误结构。
pub struct Path<T>(pub T);

/// Query 参数提取器，统一将解析失败映射为业务错误结构。
pub struct Query<T>(pub T);

impl<T> Json<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Path<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Query<T> {
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
        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => Err(map_json_rejection(rejection)),
        }
    }
}

impl<S, T> FromRequestParts<S> for Path<T>
where
    axum::extract::Path<T>: FromRequestParts<S, Rejection = PathRejection>,
    S: Send + Sync,
{
    type Rejection = ApiFailure;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Path::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => Err(map_path_rejection(rejection)),
        }
    }
}

impl<S, T> FromRequestParts<S> for Query<T>
where
    axum::extract::Query<T>: FromRequestParts<S, Rejection = QueryRejection>,
    S: Send + Sync,
{
    type Rejection = ApiFailure;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Query::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => Err(map_query_rejection(rejection)),
        }
    }
}

fn map_json_rejection(rejection: JsonRejection) -> ApiFailure {
    let status = rejection.status();
    let description = rejection.body_text();

    let (message, rejection_kind) = match rejection {
        JsonRejection::JsonDataError(_) => ("请求体字段类型错误", "json_data_error"),
        JsonRejection::JsonSyntaxError(_) => ("请求体 JSON 格式错误", "json_syntax_error"),
        JsonRejection::MissingJsonContentType(_) => (
            "请求头 Content-Type 必须为 application/json",
            "missing_json_content_type",
        ),
        JsonRejection::BytesRejection(_) => ("请求体读取失败", "bytes_rejection"),
        _ => ("请求体解析失败", "json_rejection"),
    };

    tracing::debug!(
        rejection_kind,
        description,
        "json request extraction rejected"
    );
    ApiFailure::new(status, BAD_REQUEST_CODE, message)
}

fn map_path_rejection(rejection: PathRejection) -> ApiFailure {
    let status = rejection.status();
    let description = rejection.body_text();

    let (message, rejection_kind) = match rejection {
        PathRejection::FailedToDeserializePathParams(_) => {
            ("路径参数格式错误", "failed_to_deserialize_path_params")
        }
        PathRejection::MissingPathParams(_) => ("缺少必需的路径参数", "missing_path_params"),
        _ => ("路径参数错误", "path_rejection"),
    };

    tracing::debug!(
        rejection_kind,
        description,
        "path request extraction rejected"
    );
    ApiFailure::new(status, BAD_REQUEST_CODE, message)
}

fn map_query_rejection(rejection: QueryRejection) -> ApiFailure {
    let status = rejection.status();
    let description = rejection.body_text();

    let (message, rejection_kind) = match rejection {
        QueryRejection::FailedToDeserializeQueryString(_) => {
            ("查询参数格式错误", "failed_to_deserialize_query_string")
        }
        _ => ("查询参数错误", "query_rejection"),
    };

    tracing::debug!(
        rejection_kind,
        description,
        "query request extraction rejected"
    );
    ApiFailure::new(status, BAD_REQUEST_CODE, message)
}
