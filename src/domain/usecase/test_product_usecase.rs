use std::sync::Arc;

use crate::domain::entity::product::{Product, CreateProductInput, UpdateProductInput};
use crate::domain::error::DomainError;
use crate::domain::port::mocks::MockProductRepository;
use crate::domain::usecase::product_usecase::ProductUseCase;

fn sample_product(id: &str, sku: &str, status: &str) -> Product {
    Product {
        id: id.to_string(),
        name: "Produto Teste".to_string(),
        description: None,
        sku: sku.to_string(),
        brand: None,
        status: status.to_string(),
        created_at: "2026-01-01 00:00:00".to_string(),
        updated_at: "2026-01-01 00:00:00".to_string(),
    }
}

fn make_uc() -> (ProductUseCase, Arc<MockProductRepository>) {
    let repo = Arc::new(MockProductRepository::new());
    let uc = ProductUseCase::new(repo.clone());
    (uc, repo)
}

#[tokio::test]
async fn test_list_products_ok() {
    let (uc, repo) = make_uc();
    repo.with_products(vec![
        sample_product("1", "SKU-1", "available"),
        sample_product("2", "SKU-2", "available"),
    ]);
    let result = uc.list().await.unwrap();
    assert_eq!(result.len(), 2);
}

#[tokio::test]
async fn test_get_product_found() {
    let (uc, repo) = make_uc();
    repo.with_products(vec![sample_product("abc", "SKU-1", "available")]);
    let product = uc.get("abc").await.unwrap();
    assert_eq!(product.id, "abc");
}

#[tokio::test]
async fn test_get_product_not_found() {
    let (uc, _) = make_uc();
    let err = uc.get("nonexistent").await.unwrap_err();
    match err {
        DomainError::NotFound(msg) => assert!(msg.contains("nonexistent")),
        _ => panic!("Esperava NotFound"),
    }
}

#[tokio::test]
async fn test_create_product_ok() {
    let (uc, _) = make_uc();
    let input = CreateProductInput {
        name: "Colar".to_string(),
        description: None,
        sku: "COL-001".to_string(),
        brand: None,
    };
    let product = uc.create(input).await.unwrap();
    assert_eq!(product.name, "Colar");
    assert_eq!(product.sku, "COL-001");
    assert_eq!(product.status, "available");
}

#[tokio::test]
async fn test_create_product_duplicate_sku() {
    let (uc, repo) = make_uc();
    repo.with_products(vec![sample_product("1", "COL-001", "available")]);
    let input = CreateProductInput {
        name: "Outro Colar".to_string(),
        description: None,
        sku: "COL-001".to_string(),
        brand: None,
    };
    let err = uc.create(input).await.unwrap_err();
    match err {
        DomainError::Conflict(msg) => assert!(msg.contains("COL-001")),
        _ => panic!("Esperava Conflict, got {:?}", err),
    }
}

#[tokio::test]
async fn test_update_product_ok() {
    let (uc, repo) = make_uc();
    repo.with_products(vec![sample_product("1", "SKU-1", "available")]);
    let input = UpdateProductInput {
        name: Some("Nome Novo".to_string()),
        description: None,
        brand: None,
    };
    let updated = uc.update("1", input).await.unwrap();
    assert_eq!(updated.name, "Nome Novo");
}

#[tokio::test]
async fn test_update_product_not_found() {
    let (uc, _) = make_uc();
    let input = UpdateProductInput {
        name: Some("Novo".to_string()),
        description: None,
        brand: None,
    };
    let err = uc.update("nonexistent", input).await.unwrap_err();
    match err {
        DomainError::NotFound(msg) => assert!(msg.contains("nonexistent")),
        _ => panic!("Esperava NotFound"),
    }
}

#[tokio::test]
async fn test_update_status_ok() {
    let (uc, repo) = make_uc();
    repo.with_products(vec![sample_product("1", "SKU-1", "available")]);
    let updated = uc.update_status("1", "unavailable").await.unwrap();
    assert_eq!(updated.status, "unavailable");
}

#[tokio::test]
async fn test_update_status_invalid() {
    let (uc, _) = make_uc();
    let err = uc.update_status("1", "invalid_status").await.unwrap_err();
    match err {
        DomainError::BadRequest(msg) => assert!(msg.contains("Status inválido")),
        _ => panic!("Esperava BadRequest"),
    }
}

#[tokio::test]
async fn test_delete_product_ok() {
    let (uc, repo) = make_uc();
    repo.with_products(vec![sample_product("1", "SKU-1", "available")]);
    uc.delete("1").await.unwrap();
    assert!(repo.products.lock().unwrap().is_empty());
}

#[tokio::test]
async fn test_delete_product_not_found() {
    let (uc, _) = make_uc();
    let err = uc.delete("nonexistent").await.unwrap_err();
    match err {
        DomainError::NotFound(_) => {}
        _ => panic!("Esperava NotFound"),
    }
}
