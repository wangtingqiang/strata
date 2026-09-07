//! 端到端路由契约测试：提取器与统一响应体走真实 HTTP 栈。

use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
    routing::{get, post},
};
use http_body_util::BodyExt;
use serde::Deserialize;
use serde_json::Value;
use tower::ServiceExt;

use strata_axum::{
    extract::{Json, Path, Query},
    response::ApiSuccess,
};

#[derive(Debug, Deserialize)]
struct PageQuery {
    page: u32,
    size: u32,
}

#[derive(Debug, Deserialize)]
struct NewItem {
    name: String,
}

fn app() -> Router {
    Router::new()
        .route(
            "/users/{id}",
            get(
                |Path(id): Path<u64>, Query(page): Query<PageQuery>| async move {
                    ApiSuccess::ok().with_data(serde_json::json!({
                        "id": id,
                        "page": page.page,
                        "size": page.size,
                    }))
                },
            ),
        )
        .route(
            "/items",
            post(|Json(item): Json<NewItem>| async move {
                ApiSuccess::created().with_data(serde_json::json!({ "name": item.name }))
            }),
        )
}

async fn body_json(response: axum::response::Response) -> Value {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn get_with_path_and_query_returns_envelope_and_data() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/users/42?page=2&size=10")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = body_json(response).await;
    assert_eq!(body["success"], true);
    assert_eq!(body["code"], "0");
    assert_eq!(body["message"], "ok");
    assert_eq!(body["data"]["id"], 42);
    assert_eq!(body["data"]["page"], 2);
    assert_eq!(body["data"]["size"], 10);
}

#[tokio::test]
async fn invalid_path_param_rejects_with_failure_envelope() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/users/abc?page=2&size=10")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = body_json(response).await;
    assert_eq!(body["success"], false);
    assert_eq!(body["code"], "FAILED_TO_DESERIALIZE_PATH_PARAMS");
    assert_eq!(body["message"], "请求参数错误");
}

#[tokio::test]
async fn post_with_json_body_returns_created() {
    let response = app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/items")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name":"widget"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = body_json(response).await;
    assert_eq!(body["success"], true);
    assert_eq!(body["code"], "0");
    assert_eq!(body["message"], "created");
    assert_eq!(body["data"]["name"], "widget");
}

#[tokio::test]
async fn post_with_malformed_json_rejects() {
    let response = app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/items")
                .header("content-type", "application/json")
                .body(Body::from("not json"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = body_json(response).await;
    assert_eq!(body["success"], false);
    assert_eq!(body["code"], "JSON_SYNTAX_ERROR");
}
