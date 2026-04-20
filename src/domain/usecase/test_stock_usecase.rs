use std::sync::Arc;

use crate::domain::entity::product::Product;
use crate::domain::entity::stock::{CreateStockInput, CreateStockMovementInput, Stock};
use crate::domain::error::DomainError;
use crate::domain::port::mocks::{MockProductRepository, MockStockRepository, MockUnitOfWorkFactory, MockSaleRepository};
use crate::domain::usecase::stock_usecase::StockUseCase;

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

fn make_uc() -> (StockUseCase, Arc<MockStockRepository>, Arc<MockProductRepository>) {
    let stock_repo = Arc::new(MockStockRepository::new());
    let prod_repo = Arc::new(MockProductRepository::new());
    let _sale_repo = Arc::new(MockSaleRepository::new());

    // O uow_factory precisa ter seus próprios mocks com dados que serão clonados na transação
    let uow_prod_repo = MockProductRepository::new();
    let uow_stock_repo = MockStockRepository::new();
    let uow_sale_repo = MockSaleRepository::new();

    let uow_factory = Arc::new(MockUnitOfWorkFactory::new(uow_prod_repo, uow_stock_repo, uow_sale_repo));
    let uc = StockUseCase::new(stock_repo.clone(), prod_repo.clone(), uow_factory);
    (uc, stock_repo, prod_repo)
}

// Helper para criar UC com dados pré-configurados no UoW (para create_movement)
fn make_uc_with_uow_data(
    products: Vec<Product>,
    stocks: Vec<Stock>,
) -> StockUseCase {
    let stock_repo = Arc::new(MockStockRepository::new());
    let prod_repo = Arc::new(MockProductRepository::new());
    prod_repo.with_products(products);
    let _sale_repo = Arc::new(MockSaleRepository::new());

    // Configura dados no UoW factory para que begin() clone-os para a transação
    let uow_prod_repo = MockProductRepository::new();
    let uow_stock_repo = MockStockRepository::new();
    for s in stocks {
        uow_stock_repo.stocks.lock().unwrap().push(s);
    }
    let uow_sale_repo = MockSaleRepository::new();

    let uow_factory = Arc::new(MockUnitOfWorkFactory::new(uow_prod_repo, uow_stock_repo, uow_sale_repo));
    StockUseCase::new(stock_repo, prod_repo, uow_factory)
}

#[tokio::test]
async fn test_get_by_product_found() {
    let (uc, stock_repo, _) = make_uc();
    stock_repo.stocks.lock().unwrap().push(sample_stock("p1", 100));
    let stock = uc.get_by_product("p1").await.unwrap();
    assert_eq!(stock.quantity, 100);
}

#[tokio::test]
async fn test_get_by_product_not_found() {
    let (uc, _, _) = make_uc();
    let err = uc.get_by_product("nonexistent").await.unwrap_err();
    match err {
        DomainError::NotFound(msg) => assert!(msg.contains("nonexistent")),
        _ => panic!("Esperava NotFound"),
    }
}

#[tokio::test]
async fn test_create_stock_ok() {
    let (uc, _, prod_repo) = make_uc();
    prod_repo.with_products(vec![sample_product("p1")]);
    let input = CreateStockInput {
        product_id: "p1".to_string(),
        quantity: Some(50),
        min_quantity: Some(10),
        location: Some("Prateleira A1".to_string()),
    };
    let stock = uc.create(input).await.unwrap();
    assert_eq!(stock.quantity, 50);
}

#[tokio::test]
async fn test_create_stock_product_not_found() {
    let (uc, _, _) = make_uc();
    let input = CreateStockInput {
        product_id: "nonexistent".to_string(),
        quantity: Some(50),
        min_quantity: Some(10),
        location: None,
    };
    let err = uc.create(input).await.unwrap_err();
    match err {
        DomainError::NotFound(msg) => assert!(msg.contains("nonexistent")),
        _ => panic!("Esperava NotFound"),
    }
}

#[tokio::test]
async fn test_create_stock_negative_quantity() {
    let (uc, _, prod_repo) = make_uc();
    prod_repo.with_products(vec![sample_product("p1")]);
    let input = CreateStockInput {
        product_id: "p1".to_string(),
        quantity: Some(-5),
        min_quantity: None,
        location: None,
    };
    let err = uc.create(input).await.unwrap_err();
    match err {
        DomainError::BadRequest(msg) => assert!(msg.contains("negativa")),
        _ => panic!("Esperava BadRequest, got {:?}", err),
    }
}

#[tokio::test]
async fn test_create_stock_negative_min_quantity() {
    let (uc, _, prod_repo) = make_uc();
    prod_repo.with_products(vec![sample_product("p1")]);
    let input = CreateStockInput {
        product_id: "p1".to_string(),
        quantity: None,
        min_quantity: Some(-3),
        location: None,
    };
    let err = uc.create(input).await.unwrap_err();
    match err {
        DomainError::BadRequest(msg) => assert!(msg.contains("mínima")),
        _ => panic!("Esperava BadRequest, got {:?}", err),
    }
}

#[tokio::test]
async fn test_create_movement_in_ok() {
    let uc = make_uc_with_uow_data(
        vec![sample_product("p1")],
        vec![sample_stock("p1", 10)],
    );
    let input = CreateStockMovementInput {
        product_id: "p1".to_string(),
        movement_type: "in".to_string(),
        quantity: 5,
        reason: Some("Reposição".to_string()),
        reference: None,
    };
    let movement = uc.create_movement(input).await.unwrap();
    assert_eq!(movement.movement_type, "in");
    assert_eq!(movement.quantity, 5);
}

#[tokio::test]
async fn test_create_movement_out_ok() {
    let uc = make_uc_with_uow_data(
        vec![sample_product("p1")],
        vec![sample_stock("p1", 100)],
    );
    let input = CreateStockMovementInput {
        product_id: "p1".to_string(),
        movement_type: "out".to_string(),
        quantity: 10,
        reason: None,
        reference: None,
    };
    let movement = uc.create_movement(input).await.unwrap();
    assert_eq!(movement.movement_type, "out");
}

#[tokio::test]
async fn test_create_movement_out_insufficient() {
    let uc = make_uc_with_uow_data(
        vec![sample_product("p1")],
        vec![sample_stock("p1", 5)],
    );
    let input = CreateStockMovementInput {
        product_id: "p1".to_string(),
        movement_type: "out".to_string(),
        quantity: 50,
        reason: None,
        reference: None,
    };
    let err = uc.create_movement(input).await.unwrap_err();
    match err {
        DomainError::BadRequest(msg) => assert!(msg.contains("insuficiente")),
        _ => panic!("Esperava BadRequest, got {:?}", err),
    }
}

#[tokio::test]
async fn test_create_movement_invalid_quantity() {
    let (uc, _, _) = make_uc();
    let input = CreateStockMovementInput {
        product_id: "p1".to_string(),
        movement_type: "in".to_string(),
        quantity: 0,
        reason: None,
        reference: None,
    };
    let err = uc.create_movement(input).await.unwrap_err();
    match err {
        DomainError::BadRequest(msg) => assert!(msg.contains("maior que zero")),
        _ => panic!("Esperava BadRequest"),
    }
}

#[tokio::test]
async fn test_create_movement_invalid_type() {
    let (uc, _, _) = make_uc();
    let input = CreateStockMovementInput {
        product_id: "p1".to_string(),
        movement_type: "invalid".to_string(),
        quantity: 5,
        reason: None,
        reference: None,
    };
    let err = uc.create_movement(input).await.unwrap_err();
    match err {
        DomainError::BadRequest(msg) => assert!(msg.contains("Tipo de movimentação")),
        _ => panic!("Esperava BadRequest"),
    }
}

#[tokio::test]
async fn test_create_movement_product_not_found() {
    let (uc, _, _) = make_uc();
    let input = CreateStockMovementInput {
        product_id: "nonexistent".to_string(),
        movement_type: "in".to_string(),
        quantity: 5,
        reason: None,
        reference: None,
    };
    let err = uc.create_movement(input).await.unwrap_err();
    match err {
        DomainError::NotFound(msg) => assert!(msg.contains("nonexistent")),
        _ => panic!("Esperava NotFound, got {:?}", err),
    }
}

#[tokio::test]
async fn test_list_movements() {
    let (uc, _stock_repo, _) = make_uc();
    assert!(uc.list_movements().await.unwrap().is_empty());
}

#[tokio::test]
async fn test_list_low_stock() {
    let (uc, _, _) = make_uc();
    assert!(uc.list_low_stock().await.unwrap().is_empty());
}
