mod common;

use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;
use serde_json::json;

use common::create_test_app;

async fn make_request(app: axum::Router, method: Method, uri: &str, body: Option<serde_json::Value>) -> (StatusCode, serde_json::Value) {
    let request = match body {
        Some(b) => Request::builder()
            .method(method)
            .uri(uri)
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&b).unwrap()))
            .unwrap(),
        None => Request::builder()
            .method(method)
            .uri(uri)
            .body(Body::empty())
            .unwrap(),
    };

    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body_json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap_or(serde_json::Value::Null);
    (status, body_json)
}

#[tokio::test]
async fn test_health_check() {
    let app = create_test_app().await;
    let (status, body) = make_request(app, Method::GET, "/api/products", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, json!([]));
}

#[tokio::test]
async fn test_create_product() {
    let app = create_test_app().await;
    let (status, body) = make_request(app, Method::POST, "/api/products", Some(json!({
        "name": "Colar de Pérolas", "description": "Colar elegante",
        "sku": "COL-PER-001", "brand": "Marca Luxo"
    }))).await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["name"], "Colar de Pérolas");
    assert_eq!(body["sku"], "COL-PER-001");
    assert_eq!(body["status"], "available");
    assert!(body["id"].is_string());
}

#[tokio::test]
async fn test_create_product_duplicate_sku() {
    let app = create_test_app().await;
    make_request(app.clone(), Method::POST, "/api/products", Some(json!({
        "name": "Produto 1", "sku": "SKU-UNIQUE"
    }))).await;

    let (status, _) = make_request(app, Method::POST, "/api/products", Some(json!({
        "name": "Produto 2", "sku": "SKU-UNIQUE"
    }))).await;

    assert_eq!(status, StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_get_product_not_found() {
    let app = create_test_app().await;
    let (status, body) = make_request(app, Method::GET, "/api/products/nonexistent-id", None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(body["message"].as_str().unwrap().contains("não encontrado"));
}

#[tokio::test]
async fn test_update_product() {
    let app = create_test_app().await;
    let (_, body) = make_request(app.clone(), Method::POST, "/api/products", Some(json!({
        "name": "Produto Original", "sku": "SKU-UPD-001"
    }))).await;
    let product_id = body["id"].as_str().unwrap();

    let (status, updated) = make_request(app, Method::PUT, &format!("/api/products/{}", product_id), Some(json!({
        "name": "Produto Atualizado", "brand": "Nova Marca"
    }))).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated["name"], "Produto Atualizado");
    assert_eq!(updated["brand"], "Nova Marca");
}

#[tokio::test]
async fn test_update_product_status() {
    let app = create_test_app().await;
    let (_, body) = make_request(app.clone(), Method::POST, "/api/products", Some(json!({
        "name": "Produto Status", "sku": "SKU-STS-001"
    }))).await;
    let product_id = body["id"].as_str().unwrap();

    let (status, updated) = make_request(app, Method::PATCH, &format!("/api/products/{}/status", product_id), Some(json!({
        "status": "unavailable"
    }))).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated["status"], "unavailable");
}

#[tokio::test]
async fn test_update_product_invalid_status() {
    let app = create_test_app().await;
    let (_, body) = make_request(app.clone(), Method::POST, "/api/products", Some(json!({
        "name": "Produto Status Inv", "sku": "SKU-INV-001"
    }))).await;
    let product_id = body["id"].as_str().unwrap();

    let (status, _) = make_request(app, Method::PATCH, &format!("/api/products/{}/status", product_id), Some(json!({
        "status": "invalid_status"
    }))).await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_delete_product() {
    let app = create_test_app().await;
    let (_, body) = make_request(app.clone(), Method::POST, "/api/products", Some(json!({
        "name": "Produto Delete", "sku": "SKU-DEL-001"
    }))).await;
    let product_id = body["id"].as_str().unwrap();

    let (status, _) = make_request(app.clone(), Method::DELETE, &format!("/api/products/{}", product_id), None).await;
    assert_eq!(status, StatusCode::OK);

    let (status, _) = make_request(app, Method::GET, &format!("/api/products/{}", product_id), None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_product_not_found() {
    let app = create_test_app().await;
    let (status, _) = make_request(app, Method::DELETE, "/api/products/nonexistent", None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}
