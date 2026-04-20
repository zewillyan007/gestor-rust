use std::sync::Arc;

use crate::domain::entity::product::Product;
use crate::domain::entity::return_model::{CreateReturnInput, UpdateReturnStatusInput, Return};
use crate::domain::error::DomainError;
use crate::domain::port::mocks::{MockProductRepository, MockReturnRepository};
use crate::domain::usecase::return_usecase::ReturnUseCase;

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

fn sample_return(id: &str) -> Return {
    Return {
        id: id.to_string(),
        product_id: "p1".to_string(),
        warranty_id: None,
        reason: "Defeito".to_string(),
        status: "requested".to_string(),
        refund_amount: None,
        created_at: "2026-01-01 00:00:00".to_string(),
        updated_at: "2026-01-01 00:00:00".to_string(),
    }
}

fn make_uc() -> (ReturnUseCase, Arc<MockReturnRepository>, Arc<MockProductRepository>) {
    let ret_repo = Arc::new(MockReturnRepository::new());
    let prod_repo = Arc::new(MockProductRepository::new());
    let uc = ReturnUseCase::new(ret_repo.clone(), prod_repo.clone());
    (uc, ret_repo, prod_repo)
}

#[tokio::test]
async fn test_list_returns() {
    let (uc, ret_repo, _) = make_uc();
    ret_repo.returns.lock().unwrap().push(sample_return("r1"));
    let result = uc.list().await.unwrap();
    assert_eq!(result.len(), 1);
}

#[tokio::test]
async fn test_get_return_found() {
    let (uc, ret_repo, _) = make_uc();
    ret_repo.returns.lock().unwrap().push(sample_return("r1"));
    let ret = uc.get("r1").await.unwrap();
    assert_eq!(ret.id, "r1");
}

#[tokio::test]
async fn test_get_return_not_found() {
    let (uc, _, _) = make_uc();
    let err = uc.get("nonexistent").await.unwrap_err();
    match err {
        DomainError::NotFound(msg) => assert!(msg.contains("nonexistent")),
        _ => panic!("Esperava NotFound"),
    }
}

#[tokio::test]
async fn test_create_return_ok() {
    let (uc, _, prod_repo) = make_uc();
    prod_repo.with_products(vec![sample_product("p1")]);
    let input = CreateReturnInput {
        product_id: "p1".to_string(),
        warranty_id: None,
        reason: "Produto com defeito".to_string(),
        refund_amount: Some(59.90),
    };
    let ret = uc.create(input).await.unwrap();
    assert_eq!(ret.status, "requested");
    assert_eq!(ret.reason, "Produto com defeito");
}

#[tokio::test]
async fn test_create_return_product_not_found() {
    let (uc, _, _) = make_uc();
    let input = CreateReturnInput {
        product_id: "nonexistent".to_string(),
        warranty_id: None,
        reason: "Teste".to_string(),
        refund_amount: None,
    };
    let err = uc.create(input).await.unwrap_err();
    match err {
        DomainError::NotFound(msg) => assert!(msg.contains("nonexistent")),
        _ => panic!("Esperava NotFound"),
    }
}

#[tokio::test]
async fn test_update_status_ok() {
    let (uc, ret_repo, _) = make_uc();
    ret_repo.returns.lock().unwrap().push(sample_return("r1"));
    let input = UpdateReturnStatusInput {
        status: "approved".to_string(),
        refund_amount: Some(100.0),
    };
    let updated = uc.update_status("r1", input).await.unwrap();
    assert_eq!(updated.status, "approved");
    assert_eq!(updated.refund_amount, Some(100.0));
}

#[tokio::test]
async fn test_update_status_invalid() {
    let (uc, _, _) = make_uc();
    let input = UpdateReturnStatusInput {
        status: "invalid".to_string(),
        refund_amount: None,
    };
    let err = uc.update_status("r1", input).await.unwrap_err();
    match err {
        DomainError::BadRequest(msg) => assert!(msg.contains("Status inválido")),
        _ => panic!("Esperava BadRequest"),
    }
}
