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
async fn test_create_price() {
    let app = create_test_app().await;
    let (_, body) = make_request(app.clone(), Method::POST, "/api/products", Some(json!({
        "name": "Produto", "sku": "SKU-PRICE-001"
    }))).await;
    let product_id = body["id"].as_str().unwrap();

    let (status, body) = make_request(app, Method::POST, &format!("/api/products/{}/prices", product_id), Some(json!({
        "cost_price": 25.0, "sale_price": 59.90, "effective_date": "2026-04-18"
    }))).await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["cost_price"], 25.0);
    assert_eq!(body["sale_price"], 59.9);
}

#[tokio::test]
async fn test_create_price_product_not_found() {
    let app = create_test_app().await;
    let (status, _) = make_request(app, Method::POST, "/api/products/nonexistent/prices", Some(json!({
        "cost_price": 10.0, "sale_price": 20.0, "effective_date": "2026-04-18"
    }))).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_update_price() {
    let app = create_test_app().await;
    let (_, body) = make_request(app.clone(), Method::POST, "/api/products", Some(json!({"name": "Produto", "sku": "SKU-PRICE-UPD"}))).await;
    let product_id = body["id"].as_str().unwrap();

    let (_, price) = make_request(app.clone(), Method::POST, &format!("/api/products/{}/prices", product_id), Some(json!({
        "cost_price": 10.0, "sale_price": 20.0, "effective_date": "2026-01-01"
    }))).await;
    let price_id = price["id"].as_str().unwrap();

    let (status, updated) = make_request(app, Method::PUT, &format!("/api/prices/{}", price_id), Some(json!({"sale_price": 25.0}))).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated["sale_price"], 25.0);
    assert_eq!(updated["cost_price"], 10.0);
}

#[tokio::test]
async fn test_delete_price() {
    let app = create_test_app().await;
    let (_, body) = make_request(app.clone(), Method::POST, "/api/products", Some(json!({"name": "Produto", "sku": "SKU-PRICE-DEL"}))).await;
    let product_id = body["id"].as_str().unwrap();

    let (_, price) = make_request(app.clone(), Method::POST, &format!("/api/products/{}/prices", product_id), Some(json!({
        "cost_price": 10.0, "sale_price": 20.0, "effective_date": "2026-01-01"
    }))).await;
    let price_id = price["id"].as_str().unwrap();

    let (status, _) = make_request(app, Method::DELETE, &format!("/api/prices/{}", price_id), None).await;
    assert_eq!(status, StatusCode::OK);
}
