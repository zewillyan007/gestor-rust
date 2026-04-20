use std::sync::Arc;

use crate::domain::entity::price::{Price, CreatePriceInput, UpdatePriceInput};
use crate::domain::entity::product::Product;
use crate::domain::error::DomainError;
use crate::domain::port::mocks::{MockPriceRepository, MockProductRepository};
use crate::domain::usecase::price_usecase::PriceUseCase;

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

fn sample_price(id: &str, product_id: &str) -> Price {
    Price {
        id: id.to_string(),
        product_id: product_id.to_string(),
        cost_price: 10.0,
        sale_price: 25.0,
        effective_date: "2026-01-01".to_string(),
        created_at: "2026-01-01 00:00:00".to_string(),
    }
}

fn make_uc() -> (PriceUseCase, Arc<MockPriceRepository>, Arc<MockProductRepository>) {
    let price_repo = Arc::new(MockPriceRepository::new());
    let prod_repo = Arc::new(MockProductRepository::new());
    let uc = PriceUseCase::new(price_repo.clone(), prod_repo.clone());
    (uc, price_repo, prod_repo)
}

#[tokio::test]
async fn test_list_by_product() {
    let (uc, price_repo, prod_repo) = make_uc();
    prod_repo.with_products(vec![sample_product("p1")]);
    price_repo.prices.lock().unwrap().push(sample_price("pr1", "p1"));
    let result = uc.list_by_product("p1").await.unwrap();
    assert_eq!(result.len(), 1);
}

#[tokio::test]
async fn test_create_price_ok() {
    let (uc, _, prod_repo) = make_uc();
    prod_repo.with_products(vec![sample_product("p1")]);
    let input = CreatePriceInput {
        cost_price: 25.0,
        sale_price: 59.90,
        effective_date: "2026-04-18".to_string(),
    };
    let price = uc.create("p1", input).await.unwrap();
    assert_eq!(price.cost_price, 25.0);
    assert_eq!(price.sale_price, 59.90);
}

#[tokio::test]
async fn test_create_price_product_not_found() {
    let (uc, _, _) = make_uc();
    let input = CreatePriceInput {
        cost_price: 25.0,
        sale_price: 59.90,
        effective_date: "2026-04-18".to_string(),
    };
    let err = uc.create("nonexistent", input).await.unwrap_err();
    match err {
        DomainError::NotFound(msg) => assert!(msg.contains("nonexistent")),
        _ => panic!("Esperava NotFound"),
    }
}

#[tokio::test]
async fn test_create_price_negative_cost() {
    let (uc, _, prod_repo) = make_uc();
    prod_repo.with_products(vec![sample_product("p1")]);
    let input = CreatePriceInput {
        cost_price: -5.0,
        sale_price: 59.90,
        effective_date: "2026-04-18".to_string(),
    };
    let err = uc.create("p1", input).await.unwrap_err();
    match err {
        DomainError::BadRequest(msg) => assert!(msg.contains("maiores que zero")),
        _ => panic!("Esperava BadRequest"),
    }
}

#[tokio::test]
async fn test_create_price_negative_sale() {
    let (uc, _, prod_repo) = make_uc();
    prod_repo.with_products(vec![sample_product("p1")]);
    let input = CreatePriceInput {
        cost_price: 25.0,
        sale_price: -10.0,
        effective_date: "2026-04-18".to_string(),
    };
    let err = uc.create("p1", input).await.unwrap_err();
    match err {
        DomainError::BadRequest(msg) => assert!(msg.contains("maiores que zero")),
        _ => panic!("Esperava BadRequest"),
    }
}

#[tokio::test]
async fn test_update_price_ok() {
    let (uc, price_repo, _) = make_uc();
    price_repo.prices.lock().unwrap().push(sample_price("pr1", "p1"));
    let input = UpdatePriceInput {
        cost_price: None,
        sale_price: Some(99.90),
        effective_date: None,
    };
    let updated = uc.update("pr1", input).await.unwrap();
    assert_eq!(updated.sale_price, 99.90);
    assert_eq!(updated.cost_price, 10.0); // unchanged
}

#[tokio::test]
async fn test_update_price_not_found() {
    let (uc, _, _) = make_uc();
    let input = UpdatePriceInput {
        cost_price: None,
        sale_price: Some(50.0),
        effective_date: None,
    };
    let err = uc.update("nonexistent", input).await.unwrap_err();
    match err {
        DomainError::NotFound(msg) => assert!(msg.contains("nonexistent")),
        _ => panic!("Esperava NotFound"),
    }
}

#[tokio::test]
async fn test_update_price_negative_cost() {
    let (uc, price_repo, _) = make_uc();
    price_repo.prices.lock().unwrap().push(sample_price("pr1", "p1"));
    let input = UpdatePriceInput {
        cost_price: Some(-5.0),
        sale_price: None,
        effective_date: None,
    };
    let err = uc.update("pr1", input).await.unwrap_err();
    match err {
        DomainError::BadRequest(msg) => assert!(msg.contains("custo")),
        _ => panic!("Esperava BadRequest"),
    }
}

#[tokio::test]
async fn test_update_price_negative_sale() {
    let (uc, price_repo, _) = make_uc();
    price_repo.prices.lock().unwrap().push(sample_price("pr1", "p1"));
    let input = UpdatePriceInput {
        cost_price: None,
        sale_price: Some(-10.0),
        effective_date: None,
    };
    let err = uc.update("pr1", input).await.unwrap_err();
    match err {
        DomainError::BadRequest(msg) => assert!(msg.contains("venda")),
        _ => panic!("Esperava BadRequest"),
    }
}

#[tokio::test]
async fn test_delete_price_ok() {
    let (uc, price_repo, _) = make_uc();
    price_repo.prices.lock().unwrap().push(sample_price("pr1", "p1"));
    uc.delete("pr1").await.unwrap();
    assert!(price_repo.prices.lock().unwrap().is_empty());
}

#[tokio::test]
async fn test_delete_price_not_found() {
    let (uc, _, _) = make_uc();
    let err = uc.delete("nonexistent").await.unwrap_err();
    match err {
        DomainError::NotFound(_) => {}
        _ => panic!("Esperava NotFound"),
    }
}
