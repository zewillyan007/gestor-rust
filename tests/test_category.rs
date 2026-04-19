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
            .method(method).uri(uri)
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&b).unwrap())).unwrap(),
        None => Request::builder().method(method).uri(uri).body(Body::empty()).unwrap(),
    };
    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body_json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap_or(serde_json::Value::Null);
    (status, body_json)
}

#[tokio::test]
async fn test_create_category() {
    let app = create_test_app().await;
    let (status, body) = make_request(app, Method::POST, "/api/categories", Some(json!({
        "name": "Colares", "description": "Categoria de colares"
    }))).await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["name"], "Colares");
}

#[tokio::test]
async fn test_create_subcategory() {
    let app = create_test_app().await;
    let (_, parent) = make_request(app.clone(), Method::POST, "/api/categories", Some(json!({"name": "Acessórios"}))).await;
    let parent_id = parent["id"].as_str().unwrap();

    let (status, child) = make_request(app, Method::POST, "/api/categories", Some(json!({
        "name": "Colares", "parent_id": parent_id
    }))).await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(child["parent_id"], parent_id);
}

#[tokio::test]
async fn test_list_categories() {
    let app = create_test_app().await;
    make_request(app.clone(), Method::POST, "/api/categories", Some(json!({"name": "Cat A"}))).await;
    make_request(app.clone(), Method::POST, "/api/categories", Some(json!({"name": "Cat B"}))).await;

    let (status, body) = make_request(app, Method::GET, "/api/categories", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn test_update_category() {
    let app = create_test_app().await;
    let (_, body) = make_request(app.clone(), Method::POST, "/api/categories", Some(json!({"name": "Original"}))).await;
    let cat_id = body["id"].as_str().unwrap();

    let (status, updated) = make_request(app, Method::PUT, &format!("/api/categories/{}", cat_id), Some(json!({"name": "Atualizada"}))).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated["name"], "Atualizada");
}

#[tokio::test]
async fn test_delete_category() {
    let app = create_test_app().await;
    let (_, body) = make_request(app.clone(), Method::POST, "/api/categories", Some(json!({"name": "Delete Me"}))).await;
    let cat_id = body["id"].as_str().unwrap();

    let (status, _) = make_request(app.clone(), Method::DELETE, &format!("/api/categories/{}", cat_id), None).await;
    assert_eq!(status, StatusCode::OK);

    let (status, _) = make_request(app, Method::GET, &format!("/api/categories/{}", cat_id), None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_link_unlink_product_category() {
    let app = create_test_app().await;
    let (_, prod) = make_request(app.clone(), Method::POST, "/api/products", Some(json!({"name": "Produto", "sku": "SKU-LINK-001"}))).await;
    let (_, cat) = make_request(app.clone(), Method::POST, "/api/categories", Some(json!({"name": "Cat"}))).await;
    let product_id = prod["id"].as_str().unwrap();
    let category_id = cat["id"].as_str().unwrap();

    let (status, _) = make_request(app.clone(), Method::POST, &format!("/api/products/{}/categories/{}", product_id, category_id), None).await;
    assert_eq!(status, StatusCode::OK);

    let (status, _) = make_request(app, Method::DELETE, &format!("/api/products/{}/categories/{}", product_id, category_id), None).await;
    assert_eq!(status, StatusCode::OK);
}
