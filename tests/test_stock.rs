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

async fn setup_product_with_stock(sku: &str, qty: i32) -> (axum::Router, String) {
    let app = create_test_app().await;
    let (_, body) = make_request(app.clone(), Method::POST, "/api/products", Some(json!({
        "name": format!("Produto {}", sku), "sku": sku
    }))).await;
    let product_id = body["id"].as_str().unwrap().to_string();
    make_request(app.clone(), Method::POST, "/api/stocks", Some(json!({
        "product_id": &product_id, "quantity": qty, "min_quantity": 5
    }))).await;
    (app, product_id)
}

#[tokio::test]
async fn test_create_stock() {
    let app = create_test_app().await;
    let (_, body) = make_request(app.clone(), Method::POST, "/api/products", Some(json!({"name": "Produto", "sku": "SKU-STOCK-001"}))).await;
    let product_id = body["id"].as_str().unwrap();

    let (status, body) = make_request(app, Method::POST, "/api/stocks", Some(json!({
        "product_id": product_id, "quantity": 100, "min_quantity": 10, "location": "Prateleira A1"
    }))).await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["quantity"], 100);
    assert_eq!(body["location"], "Prateleira A1");
}

#[tokio::test]
async fn test_stock_movement_in() {
    let (app, product_id) = setup_product_with_stock("SKU-MOV-IN", 10).await;

    let (status, mov) = make_request(app.clone(), Method::POST, "/api/stock/movements", Some(json!({
        "product_id": product_id, "movement_type": "in", "quantity": 5, "reason": "Reposição"
    }))).await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(mov["movement_type"], "in");

    let (_, stock) = make_request(app, Method::GET, &format!("/api/products/{}/stock", product_id), None).await;
    assert_eq!(stock["quantity"], 15);
}

#[tokio::test]
async fn test_stock_movement_out_insufficient() {
    let (app, product_id) = setup_product_with_stock("SKU-MOV-OUT-ERR", 5).await;
    let (status, _) = make_request(app, Method::POST, "/api/stock/movements", Some(json!({
        "product_id": product_id, "movement_type": "out", "quantity": 10
    }))).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_stock_movement_invalid_quantity() {
    let app = create_test_app().await;
    let (status, _) = make_request(app, Method::POST, "/api/stock/movements", Some(json!({
        "product_id": "any", "movement_type": "in", "quantity": 0
    }))).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_stock_movement_invalid_type() {
    let app = create_test_app().await;
    let (status, _) = make_request(app, Method::POST, "/api/stock/movements", Some(json!({
        "product_id": "any", "movement_type": "invalid", "quantity": 5
    }))).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_low_stock_report() {
    let (app, product_id) = setup_product_with_stock("SKU-LOW", 2).await;
    let (status, body) = make_request(app, Method::GET, "/api/stock/low", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.as_array().unwrap().iter().any(|i| i["product_id"] == product_id));
}
