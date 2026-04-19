use async_trait::async_trait;
use crate::domain::entity::product::{Product, CreateProductInput, UpdateProductInput};
use crate::domain::error::DomainError;

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn list(&self) -> Result<Vec<Product>, DomainError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Product>, DomainError>;
    async fn find_by_sku(&self, sku: &str) -> Result<Option<Product>, DomainError>;
    async fn create(&self, input: &CreateProductInput) -> Result<Product, DomainError>;
    async fn update(&self, id: &str, input: &UpdateProductInput) -> Result<Product, DomainError>;
    async fn update_status(&self, id: &str, status: &str) -> Result<Product, DomainError>;
    async fn delete(&self, id: &str) -> Result<(), DomainError>;
    async fn exists(&self, id: &str) -> Result<bool, DomainError>;
}
