use std::sync::Arc;

use crate::domain::entity::category::{Category, CreateCategoryInput, UpdateCategoryInput};
use crate::domain::entity::product::Product;
use crate::domain::error::DomainError;
use crate::domain::port::mocks::{MockCategoryRepository, MockProductRepository};
use crate::domain::usecase::category_usecase::CategoryUseCase;

fn sample_category(id: &str, name: &str) -> Category {
    Category {
        id: id.to_string(),
        name: name.to_string(),
        description: None,
        parent_id: None,
        created_at: "2026-01-01 00:00:00".to_string(),
        updated_at: "2026-01-01 00:00:00".to_string(),
    }
}

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

fn make_uc() -> (CategoryUseCase, Arc<MockCategoryRepository>, Arc<MockProductRepository>) {
    let cat_repo = Arc::new(MockCategoryRepository::new());
    let prod_repo = Arc::new(MockProductRepository::new());
    let uc = CategoryUseCase::new(cat_repo.clone(), prod_repo.clone());
    (uc, cat_repo, prod_repo)
}

#[tokio::test]
async fn test_list_categories() {
    let (uc, cat_repo, _) = make_uc();
    cat_repo.with_categories(vec![sample_category("1", "Colares"), sample_category("2", "Brincos")]);
    let result = uc.list().await.unwrap();
    assert_eq!(result.len(), 2);
}

#[tokio::test]
async fn test_get_category_found() {
    let (uc, cat_repo, _) = make_uc();
    cat_repo.with_categories(vec![sample_category("c1", "Colares")]);
    let cat = uc.get("c1").await.unwrap();
    assert_eq!(cat.name, "Colares");
}

#[tokio::test]
async fn test_get_category_not_found() {
    let (uc, _, _) = make_uc();
    let err = uc.get("nonexistent").await.unwrap_err();
    match err {
        DomainError::NotFound(msg) => assert!(msg.contains("nonexistent")),
        _ => panic!("Esperava NotFound"),
    }
}

#[tokio::test]
async fn test_create_category_ok() {
    let (uc, _, _) = make_uc();
    let input = CreateCategoryInput {
        name: "Anéis".to_string(),
        description: None,
        parent_id: None,
    };
    let cat = uc.create(input).await.unwrap();
    assert_eq!(cat.name, "Anéis");
}

#[tokio::test]
async fn test_update_category_ok() {
    let (uc, cat_repo, _) = make_uc();
    cat_repo.with_categories(vec![sample_category("c1", "Velho")]);
    let input = UpdateCategoryInput {
        name: Some("Novo".to_string()),
        description: None,
        parent_id: None,
    };
    let updated = uc.update("c1", input).await.unwrap();
    assert_eq!(updated.name, "Novo");
}

#[tokio::test]
async fn test_update_category_not_found() {
    let (uc, _, _) = make_uc();
    let input = UpdateCategoryInput {
        name: Some("Novo".to_string()),
        description: None,
        parent_id: None,
    };
    let err = uc.update("nonexistent", input).await.unwrap_err();
    match err {
        DomainError::NotFound(_) => {}
        _ => panic!("Esperava NotFound"),
    }
}

#[tokio::test]
async fn test_delete_category_ok() {
    let (uc, cat_repo, _) = make_uc();
    cat_repo.with_categories(vec![sample_category("c1", "Colares")]);
    uc.delete("c1").await.unwrap();
    assert!(cat_repo.categories.lock().unwrap().is_empty());
}

#[tokio::test]
async fn test_delete_category_not_found() {
    let (uc, _, _) = make_uc();
    let err = uc.delete("nonexistent").await.unwrap_err();
    match err {
        DomainError::NotFound(_) => {}
        _ => panic!("Esperava NotFound"),
    }
}

#[tokio::test]
async fn test_link_product_ok() {
    let (uc, cat_repo, prod_repo) = make_uc();
    prod_repo.with_products(vec![sample_product("p1")]);
    cat_repo.with_categories(vec![sample_category("c1", "Colares")]);
    uc.link_product("p1", "c1").await.unwrap();
    let links = cat_repo.links.lock().unwrap();
    assert!(links.iter().any(|(p, c)| p == "p1" && c == "c1"));
}

#[tokio::test]
async fn test_link_product_not_found() {
    let (uc, cat_repo, _) = make_uc();
    cat_repo.with_categories(vec![sample_category("c1", "Colares")]);
    let err = uc.link_product("nonexistent", "c1").await.unwrap_err();
    match err {
        DomainError::NotFound(msg) => assert!(msg.contains("nonexistent")),
        _ => panic!("Esperava NotFound"),
    }
}

#[tokio::test]
async fn test_link_category_not_found() {
    let (uc, _, prod_repo) = make_uc();
    prod_repo.with_products(vec![sample_product("p1")]);
    let err = uc.link_product("p1", "nonexistent").await.unwrap_err();
    match err {
        DomainError::NotFound(msg) => assert!(msg.contains("nonexistent")),
        _ => panic!("Esperava NotFound"),
    }
}

#[tokio::test]
async fn test_unlink_product_ok() {
    let (uc, cat_repo, prod_repo) = make_uc();
    prod_repo.with_products(vec![sample_product("p1")]);
    cat_repo.with_categories(vec![sample_category("c1", "Colares")]);
    cat_repo.links.lock().unwrap().push(("p1".to_string(), "c1".to_string()));
    uc.unlink_product("p1", "c1").await.unwrap();
    assert!(cat_repo.links.lock().unwrap().is_empty());
}
