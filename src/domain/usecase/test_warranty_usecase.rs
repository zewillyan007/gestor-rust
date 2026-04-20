use std::sync::Arc;

use crate::domain::entity::product::Product;
use crate::domain::entity::warranty::CreateWarrantyInput;
use crate::domain::error::DomainError;
use crate::domain::port::mocks::{MockProductRepository, MockWarrantyRepository};
use crate::domain::usecase::warranty_usecase::WarrantyUseCase;

fn sample_product(id: &str) -> Product {
    Product {
        id: id.to_string(),
        name: "Produto".to_string(),
        description: None,
        sku: format!("SKU-{}", id),
        brand: None,
        status: "available".to_string(),
        created_at: "2026-01-01 00:00:00".to_string(),
        updated_at: "2026-01-01 00:00:00".to_string(),
    }
}

fn make_uc() -> (WarrantyUseCase, Arc<MockWarrantyRepository>, Arc<MockProductRepository>) {
    let warranty_repo = Arc::new(MockWarrantyRepository::new());
    let prod_repo = Arc::new(MockProductRepository::new());
    let uc = WarrantyUseCase::new(warranty_repo.clone(), prod_repo.clone());
    (uc, warranty_repo, prod_repo)
}

#[tokio::test]
async fn test_list_warranties() {
    let (uc, _, _) = make_uc();
    assert!(uc.list().await.unwrap().is_empty());
}

#[tokio::test]
async fn test_get_warranty_not_found() {
    let (uc, _, _) = make_uc();
    let err = uc.get("nonexistent").await.unwrap_err();
    match err {
        DomainError::NotFound(msg) => assert!(msg.contains("nonexistent")),
        _ => panic!("Esperava NotFound"),
    }
}

#[tokio::test]
async fn test_create_warranty_ok() {
    let (uc, _, prod_repo) = make_uc();
    prod_repo.with_products(vec![sample_product("p1")]);
    let input = CreateWarrantyInput {
        product_id: "p1".to_string(),
        customer_name: "Maria Silva".to_string(),
        customer_contact: None,
        purchase_date: "2026-01-15".to_string(),
        warranty_days: 90,
        notes: None,
    };
    let warranty = uc.create(input).await.unwrap();
    assert_eq!(warranty.customer_name, "Maria Silva");
    assert_eq!(warranty.status, "active");
    assert_eq!(warranty.expires_at, "2026-04-15");
}

#[tokio::test]
async fn test_create_warranty_product_not_found() {
    let (uc, _, _) = make_uc();
    let input = CreateWarrantyInput {
        product_id: "nonexistent".to_string(),
        customer_name: "Maria".to_string(),
        customer_contact: None,
        purchase_date: "2026-01-15".to_string(),
        warranty_days: 90,
        notes: None,
    };
    let err = uc.create(input).await.unwrap_err();
    match err {
        DomainError::NotFound(msg) => assert!(msg.contains("nonexistent")),
        _ => panic!("Esperava NotFound"),
    }
}

#[tokio::test]
async fn test_create_warranty_invalid_date() {
    let (uc, _, prod_repo) = make_uc();
    prod_repo.with_products(vec![sample_product("p1")]);
    let input = CreateWarrantyInput {
        product_id: "p1".to_string(),
        customer_name: "Maria".to_string(),
        customer_contact: None,
        purchase_date: "data-invalida".to_string(),
        warranty_days: 90,
        notes: None,
    };
    let err = uc.create(input).await.unwrap_err();
    match err {
        DomainError::BadRequest(msg) => assert!(msg.contains("data")),
        _ => panic!("Esperava BadRequest, got {:?}", err),
    }
}

#[tokio::test]
async fn test_create_warranty_zero_days() {
    let (uc, _, prod_repo) = make_uc();
    prod_repo.with_products(vec![sample_product("p1")]);
    let input = CreateWarrantyInput {
        product_id: "p1".to_string(),
        customer_name: "Maria".to_string(),
        customer_contact: None,
        purchase_date: "2026-01-15".to_string(),
        warranty_days: 0,
        notes: None,
    };
    let err = uc.create(input).await.unwrap_err();
    match err {
        DomainError::BadRequest(msg) => assert!(msg.contains("maior que zero")),
        _ => panic!("Esperava BadRequest"),
    }
}

#[tokio::test]
async fn test_update_status_ok() {
    let (uc, warranty_repo, _) = make_uc();
    use crate::domain::entity::warranty::Warranty;
    warranty_repo.warranties.lock().unwrap().push(Warranty {
        id: "w1".to_string(),
        product_id: "p1".to_string(),
        customer_name: "Maria".to_string(),
        customer_contact: None,
        purchase_date: "2026-01-15".to_string(),
        warranty_days: 90,
        expires_at: "2026-04-15".to_string(),
        status: "active".to_string(),
        notes: None,
        created_at: "2026-01-01 00:00:00".to_string(),
        updated_at: "2026-01-01 00:00:00".to_string(),
    });
    let updated = uc.update_status("w1", "claimed").await.unwrap();
    assert_eq!(updated.status, "claimed");
}

#[tokio::test]
async fn test_update_status_invalid() {
    let (uc, _, _) = make_uc();
    let err = uc.update_status("w1", "invalid").await.unwrap_err();
    match err {
        DomainError::BadRequest(msg) => assert!(msg.contains("Status inválido")),
        _ => panic!("Esperava BadRequest"),
    }
}
