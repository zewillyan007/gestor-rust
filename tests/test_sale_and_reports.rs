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
async fn test_create_sale() {
    let (app, product_id) = setup_product_with_stock("SKU-SALE-001", 100).await;
    let (status, body) = make_request(app.clone(), Method::POST, "/api/sales", Some(json!({
        "product_id": product_id, "quantity": 5, "unit_price": 59.90, "customer_name": "Maria Silva"
    }))).await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["quantity"], 5);
    assert_eq!(body["total_price"], 299.5);
}

#[tokio::test]
async fn test_sale_updates_stock() {
    let (app, product_id) = setup_product_with_stock("SKU-SALE-STK", 100).await;
    make_request(app.clone(), Method::POST, "/api/sales", Some(json!({
        "product_id": product_id, "quantity": 10, "unit_price": 50.0
    }))).await;

    let (_, stock) = make_request(app, Method::GET, &format!("/api/products/{}/stock", product_id), None).await;
    assert_eq!(stock["quantity"], 90);
}

#[tokio::test]
async fn test_sale_insufficient_stock() {
    let (app, product_id) = setup_product_with_stock("SKU-SALE-INS", 5).await;
    let (status, _) = make_request(app, Method::POST, "/api/sales", Some(json!({
        "product_id": product_id, "quantity": 10, "unit_price": 50.0
    }))).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_sale_product_not_found() {
    let app = create_test_app().await;
    let (status, _) = make_request(app, Method::POST, "/api/sales", Some(json!({
        "product_id": "nonexistent", "quantity": 1, "unit_price": 50.0
    }))).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_sale_unavailable_product() {
    let app = create_test_app().await;
    let (_, prod) = make_request(app.clone(), Method::POST, "/api/products", Some(json!({"name": "Unavailable", "sku": "SKU-SALE-UNAV"}))).await;
    let product_id = prod["id"].as_str().unwrap();

    make_request(app.clone(), Method::POST, "/api/stocks", Some(json!({"product_id": product_id, "quantity": 10}))).await;
    make_request(app.clone(), Method::PATCH, &format!("/api/products/{}/status", product_id), Some(json!({"status": "unavailable"}))).await;

    let (status, _) = make_request(app, Method::POST, "/api/sales", Some(json!({
        "product_id": product_id, "quantity": 1, "unit_price": 50.0
    }))).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_sales_report() {
    let (app, product_id) = setup_product_with_stock("SKU-SALE-RPT", 100).await;
    make_request(app.clone(), Method::POST, "/api/sales", Some(json!({"product_id": &product_id, "quantity": 5, "unit_price": 20.0}))).await;
    make_request(app.clone(), Method::POST, "/api/sales", Some(json!({"product_id": &product_id, "quantity": 3, "unit_price": 25.0}))).await;

    let (status, report) = make_request(app, Method::GET, "/api/reports/sales", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(report["total_items_sold"], 8);
}

#[tokio::test]
async fn test_stock_report() {
    let (app, _) = setup_product_with_stock("SKU-REPORT-S", 50).await;
    let (status, report) = make_request(app, Method::GET, "/api/reports/stock", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(report.as_array().unwrap().len() >= 1);
}

#[tokio::test]
async fn test_returns_report() {
    let app = create_test_app().await;
    let (_, prod) = make_request(app.clone(), Method::POST, "/api/products", Some(json!({"name": "Rpt Product", "sku": "SKU-REPORT-R"}))).await;
    let product_id = prod["id"].as_str().unwrap();

    make_request(app.clone(), Method::POST, "/api/returns", Some(json!({"product_id": product_id, "reason": "Defeito"}))).await;

    let (status, report) = make_request(app, Method::GET, "/api/reports/returns", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(report.as_array().unwrap().len() >= 1);
}
