use std::sync::Arc;

use crate::domain::entity::product::{
    Product, CreateProductInput, UpdateProductInput, VALID_PRODUCT_STATUSES,
};
use crate::domain::error::DomainError;
use crate::domain::port::product_repository::ProductRepository;

pub struct ProductUseCase {
    repo: Arc<dyn ProductRepository>,
}

impl ProductUseCase {
    pub fn new(repo: Arc<dyn ProductRepository>) -> Self {
        Self { repo }
    }

    pub async fn list(&self) -> Result<Vec<Product>, DomainError> {
        self.repo.list().await
    }

    pub async fn get(&self, id: &str) -> Result<Product, DomainError> {
        self.repo.find_by_id(id).await?.ok_or_else(|| {
            DomainError::NotFound(format!("Produto com id '{}' não encontrado", id))
        })
    }

    pub async fn create(&self, input: CreateProductInput) -> Result<Product, DomainError> {
        // Valida SKU único
        if let Some(_) = self.repo.find_by_sku(&input.sku).await? {
            return Err(DomainError::Conflict(format!("SKU '{}' já existe", input.sku)));
        }
        self.repo.create(&input).await
    }

    pub async fn update(&self, id: &str, input: UpdateProductInput) -> Result<Product, DomainError> {
        // Verifica se existe
        self.repo.find_by_id(id).await?.ok_or_else(|| {
            DomainError::NotFound(format!("Produto com id '{}' não encontrado", id))
        })?;
        self.repo.update(id, &input).await
    }

    pub async fn update_status(&self, id: &str, status: &str) -> Result<Product, DomainError> {
        if !VALID_PRODUCT_STATUSES.contains(&status) {
            return Err(DomainError::BadRequest(format!(
                "Status inválido. Use: {}",
                VALID_PRODUCT_STATUSES.join(", ")
            )));
        }
        self.repo.update_status(id, status).await
    }

    pub async fn delete(&self, id: &str) -> Result<(), DomainError> {
        self.repo.delete(id).await
    }
}
