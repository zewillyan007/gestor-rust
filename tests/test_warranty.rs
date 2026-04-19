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
async fn test_create_warranty() {
    let app = create_test_app().await;
    let (_, body) = make_request(app.clone(), Method::POST, "/api/products", Some(json!({"name": "Produto", "sku": "SKU-WARR-001"}))).await;
    let product_id = body["id"].as_str().unwrap();

    let (status, body) = make_request(app, Method::POST, "/api/warranties", Some(json!({
        "product_id": product_id, "customer_name": "Maria Silva",
        "customer_contact": "maria@email.com", "purchase_date": "2026-04-18",
        "warranty_days": 90, "notes": "Garantia padrão"
    }))).await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["customer_name"], "Maria Silva");
    assert_eq!(body["status"], "active");
    assert_eq!(body["expires_at"], "2026-07-17");
}

#[tokio::test]
async fn test_create_warranty_invalid_date() {
    let app = create_test_app().await;
    let (_, body) = make_request(app.clone(), Method::POST, "/api/products", Some(json!({"name": "Produto", "sku": "SKU-WARR-DATE"}))).await;
    let product_id = body["id"].as_str().unwrap();

    let (status, _) = make_request(app, Method::POST, "/api/warranties", Some(json!({
        "product_id": product_id, "customer_name": "Teste",
        "purchase_date": "data-invalida", "warranty_days": 30
    }))).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_warranty_product_not_found() {
    let app = create_test_app().await;
    let (status, _) = make_request(app, Method::POST, "/api/warranties", Some(json!({
        "product_id": "nonexistent", "customer_name": "Teste",
        "purchase_date": "2026-04-18", "warranty_days": 30
    }))).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_update_warranty_status() {
    let app = create_test_app().await;
    let (_, body) = make_request(app.clone(), Method::POST, "/api/products", Some(json!({"name": "Produto", "sku": "SKU-WARR-STS"}))).await;
    let product_id = body["id"].as_str().unwrap();

    let (_, warranty) = make_request(app.clone(), Method::POST, "/api/warranties", Some(json!({
        "product_id": product_id, "customer_name": "João",
        "purchase_date": "2026-04-18", "warranty_days": 30
    }))).await;
    let warranty_id = warranty["id"].as_str().unwrap();

    let (status, updated) = make_request(app, Method::PATCH, &format!("/api/warranties/{}/status", warranty_id), Some(json!({"status": "claimed"}))).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated["status"], "claimed");
}

#[tokio::test]
async fn test_update_warranty_invalid_status() {
    let app = create_test_app().await;
    let (_, body) = make_request(app.clone(), Method::POST, "/api/products", Some(json!({"name": "Produto", "sku": "SKU-WARR-INV"}))).await;
    let product_id = body["id"].as_str().unwrap();

    let (_, warranty) = make_request(app.clone(), Method::POST, "/api/warranties", Some(json!({
        "product_id": product_id, "customer_name": "Teste",
        "purchase_date": "2026-04-18", "warranty_days": 30
    }))).await;
    let warranty_id = warranty["id"].as_str().unwrap();

    let (status, _) = make_request(app, Method::PATCH, &format!("/api/warranties/{}/status", warranty_id), Some(json!({"status": "invalid_status"}))).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}
