use async_trait::async_trait;
use crate::domain::entity::category::{Category, CreateCategoryInput, UpdateCategoryInput};
use crate::domain::error::DomainError;

#[async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn list(&self) -> Result<Vec<Category>, DomainError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Category>, DomainError>;
    async fn create(&self, input: &CreateCategoryInput) -> Result<Category, DomainError>;
    async fn update(&self, id: &str, input: &UpdateCategoryInput) -> Result<Category, DomainError>;
    async fn delete(&self, id: &str) -> Result<(), DomainError>;
    async fn exists(&self, id: &str) -> Result<bool, DomainError>;
    async fn link_product(&self, product_id: &str, category_id: &str) -> Result<(), DomainError>;
    async fn unlink_product(&self, product_id: &str, category_id: &str) -> Result<(), DomainError>;
}
