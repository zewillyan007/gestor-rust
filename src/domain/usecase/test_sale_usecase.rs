use std::sync::Arc;

use crate::domain::entity::product::Product;
use crate::domain::entity::sale::CreateSaleInput;
use crate::domain::entity::stock::Stock;
use crate::domain::error::DomainError;
use crate::domain::port::mocks::{MockProductRepository, MockSaleRepository, MockStockRepository, MockUnitOfWorkFactory};
use crate::domain::usecase::sale_usecase::SaleUseCase;

fn sample_product(id: &str, status: &str) -> Product {
    Product {
        id: id.to_string(),
        name: "Produto".to_string(),
        description: None,
        sku: format!("SKU-{}", id),
        brand: None,
        status: status.to_string(),
        created_at: "2026-01-01 00:00:00".to_string(),
        updated_at: "2026-01-01 00:00:00".to_string(),
    }
}

fn sample_stock(product_id: &str, qty: i32) -> Stock {
    Stock {
        id: "stock-1".to_string(),
        product_id: product_id.to_string(),
        quantity: qty,
        min_quantity: 5,
        location: None,
        updated_at: "2026-01-01 00:00:00".to_string(),
    }
}

fn make_uc(
    products: Vec<Product>,
    stocks: Vec<Stock>,
) -> SaleUseCase {
    let prod_mock = MockProductRepository::new();
    prod_mock.with_products(products);
    let stock_mock = MockStockRepository::new();
    for s in stocks {
        stock_mock.stocks.lock().unwrap().push(s);
    }
    let sale_mock = MockSaleRepository::new();

    let uow_factory = Arc::new(MockUnitOfWorkFactory::new(prod_mock, stock_mock, sale_mock));
    SaleUseCase::new(uow_factory)
}

#[tokio::test]
async fn test_create_sale_ok() {
    let uc = make_uc(
        vec![sample_product("p1", "available")],
        vec![sample_stock("p1", 100)],
    );
    let input = CreateSaleInput {
        product_id: "p1".to_string(),
        quantity: 5,
        unit_price: 59.90,
        customer_name: Some("Maria Silva".to_string()),
    };
    let sale = uc.create(input).await.unwrap();
    assert_eq!(sale.quantity, 5);
    assert_eq!(sale.unit_price, 59.90);
    assert_eq!(sale.total_price, 299.5);
}

#[tokio::test]
async fn test_create_sale_invalid_quantity() {
    let uc = make_uc(vec![], vec![]);
    let input = CreateSaleInput {
        product_id: "p1".to_string(),
        quantity: 0,
        unit_price: 50.0,
        customer_name: None,
    };
    let err = uc.create(input).await.unwrap_err();
    match err {
        DomainError::BadRequest(msg) => assert!(msg.contains("Quantidade")),
        _ => panic!("Esperava BadRequest"),
    }
}

#[tokio::test]
async fn test_create_sale_invalid_price() {
    let uc = make_uc(vec![], vec![]);
    let input = CreateSaleInput {
        product_id: "p1".to_string(),
        quantity: 1,
        unit_price: 0.0,
        customer_name: None,
    };
    let err = uc.create(input).await.unwrap_err();
    match err {
        DomainError::BadRequest(msg) => assert!(msg.contains("Preço")),
        _ => panic!("Esperava BadRequest"),
    }
}

#[tokio::test]
async fn test_create_sale_product_not_found() {
    let uc = make_uc(vec![], vec![]);
    let input = CreateSaleInput {
        product_id: "nonexistent".to_string(),
        quantity: 1,
        unit_price: 50.0,
        customer_name: None,
    };
    let err = uc.create(input).await.unwrap_err();
    match err {
        DomainError::NotFound(msg) => assert!(msg.contains("nonexistent")),
        _ => panic!("Esperava NotFound, got {:?}", err),
    }
}

#[tokio::test]
async fn test_create_sale_unavailable_product() {
    let uc = make_uc(
        vec![sample_product("p1", "unavailable")],
        vec![sample_stock("p1", 100)],
    );
    let input = CreateSaleInput {
        product_id: "p1".to_string(),
        quantity: 1,
        unit_price: 50.0,
        customer_name: None,
    };
    let err = uc.create(input).await.unwrap_err();
    match err {
        DomainError::BadRequest(msg) => assert!(msg.contains("disponível")),
        _ => panic!("Esperava BadRequest, got {:?}", err),
    }
}

#[tokio::test]
async fn test_create_sale_insufficient_stock() {
    let uc = make_uc(
        vec![sample_product("p1", "available")],
        vec![sample_stock("p1", 5)],
    );
    let input = CreateSaleInput {
        product_id: "p1".to_string(),
        quantity: 50,
        unit_price: 50.0,
        customer_name: None,
    };
    let err = uc.create(input).await.unwrap_err();
    match err {
        DomainError::BadRequest(msg) => assert!(msg.contains("insuficiente")),
        _ => panic!("Esperava BadRequest, got {:?}", err),
    }
}
