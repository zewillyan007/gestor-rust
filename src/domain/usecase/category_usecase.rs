use std::sync::Arc;

use crate::domain::entity::category::{
    Category, CreateCategoryInput, UpdateCategoryInput,
};
use crate::domain::error::DomainError;
use crate::domain::port::category_repository::CategoryRepository;
use crate::domain::port::product_repository::ProductRepository;

pub struct CategoryUseCase {
    repo: Arc<dyn CategoryRepository>,
    product_repo: Arc<dyn ProductRepository>,
}

impl CategoryUseCase {
    pub fn new(repo: Arc<dyn CategoryRepository>, product_repo: Arc<dyn ProductRepository>) -> Self {
        Self { repo, product_repo }
    }

    pub async fn list(&self) -> Result<Vec<Category>, DomainError> {
        self.repo.list().await
    }

    pub async fn get(&self, id: &str) -> Result<Category, DomainError> {
        self.repo.find_by_id(id).await?.ok_or_else(|| {
            DomainError::NotFound(format!("Categoria com id '{}' não encontrada", id))
        })
    }

    pub async fn create(&self, input: CreateCategoryInput) -> Result<Category, DomainError> {
        self.repo.create(&input).await
    }

    pub async fn update(&self, id: &str, input: UpdateCategoryInput) -> Result<Category, DomainError> {
        self.repo.find_by_id(id).await?.ok_or_else(|| {
            DomainError::NotFound(format!("Categoria com id '{}' não encontrada", id))
        })?;
        self.repo.update(id, &input).await
    }

    pub async fn delete(&self, id: &str) -> Result<(), DomainError> {
        self.repo.delete(id).await
    }

    pub async fn link_product(&self, product_id: &str, category_id: &str) -> Result<(), DomainError> {
        if !self.product_repo.exists(product_id).await? {
            return Err(DomainError::NotFound(format!("Produto com id '{}' não encontrado", product_id)));
        }
        if !self.repo.exists(category_id).await? {
            return Err(DomainError::NotFound(format!("Categoria com id '{}' não encontrada", category_id)));
        }
        self.repo.link_product(product_id, category_id).await
    }

    pub async fn unlink_product(&self, product_id: &str, category_id: &str) -> Result<(), DomainError> {
        self.repo.unlink_product(product_id, category_id).await
    }
}
