mod common;

use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;
use serde_json::json;

use common::create_test_app;

async fn make_request(app: axum::Router, method: Method, uri: &str, body: Option<serde_json::Value>) -> (StatusCode, serde_json::Value) {
    let request = match body {
        Some(b) => Request::builder().method(method).uri(uri)
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&b).unwrap())).unwrap(),
        None => Request::builder().method(method).uri(uri).body(Body::empty()).unwrap(),
    };
    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    (status, serde_json::from_slice(&body_bytes).unwrap_or(serde_json::Value::Null))
}

#[tokio::test]
async fn test_create_return() {
    let app = create_test_app().await;
    let (_, body) = make_request(app.clone(), Method::POST, "/api/products", Some(json!({"name": "Produto", "sku": "SKU-RET-001"}))).await;
    let product_id = body["id"].as_str().unwrap();

    let (status, body) = make_request(app, Method::POST, "/api/returns", Some(json!({
        "product_id": product_id, "reason": "Produto com defeito", "refund_amount": 59.90
    }))).await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["status"], "requested");
    assert_eq!(body["reason"], "Produto com defeito");
}

#[tokio::test]
async fn test_create_return_product_not_found() {
    let app = create_test_app().await;
    let (status, _) = make_request(app, Method::POST, "/api/returns", Some(json!({
        "product_id": "nonexistent", "reason": "Teste"
    }))).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_update_return_status() {
    let app = create_test_app().await;
    let (_, body) = make_request(app.clone(), Method::POST, "/api/products", Some(json!({"name": "Produto", "sku": "SKU-RET-STS"}))).await;
    let product_id = body["id"].as_str().unwrap();

    let (_, ret) = make_request(app.clone(), Method::POST, "/api/returns", Some(json!({
        "product_id": product_id, "reason": "Teste"
    }))).await;
    let return_id = ret["id"].as_str().unwrap();

    let (status, updated) = make_request(app.clone(), Method::PATCH, &format!("/api/returns/{}/status", return_id), Some(json!({
        "status": "approved", "refund_amount": 100.0
    }))).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated["status"], "approved");
    assert_eq!(updated["refund_amount"], 100.0);

    let (status, completed) = make_request(app, Method::PATCH, &format!("/api/returns/{}/status", return_id), Some(json!({"status": "completed"}))).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(completed["status"], "completed");
}

#[tokio::test]
async fn test_update_return_invalid_status() {
    let app = create_test_app().await;
    let (_, body) = make_request(app.clone(), Method::POST, "/api/products", Some(json!({"name": "Produto", "sku": "SKU-RET-INV"}))).await;
    let product_id = body["id"].as_str().unwrap();

    let (_, ret) = make_request(app.clone(), Method::POST, "/api/returns", Some(json!({
        "product_id": product_id, "reason": "Teste"
    }))).await;
    let return_id = ret["id"].as_str().unwrap();

    let (status, _) = make_request(app, Method::PATCH, &format!("/api/returns/{}/status", return_id), Some(json!({"status": "invalid"}))).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_reject_return() {
    let app = create_test_app().await;
    let (_, body) = make_request(app.clone(), Method::POST, "/api/products", Some(json!({"name": "Produto", "sku": "SKU-RET-REJ"}))).await;
    let product_id = body["id"].as_str().unwrap();

    let (_, ret) = make_request(app.clone(), Method::POST, "/api/returns", Some(json!({
        "product_id": product_id, "reason": "Não gostei"
    }))).await;
    let return_id = ret["id"].as_str().unwrap();

    let (status, updated) = make_request(app, Method::PATCH, &format!("/api/returns/{}/status", return_id), Some(json!({"status": "rejected"}))).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated["status"], "rejected");
}
