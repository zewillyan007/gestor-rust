use async_trait::async_trait;
use crate::domain::entity::price::{Price, CreatePriceInput, UpdatePriceInput};
use crate::domain::error::DomainError;

#[async_trait]
pub trait PriceRepository: Send + Sync {
    async fn list_by_product(&self, product_id: &str) -> Result<Vec<Price>, DomainError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Price>, DomainError>;
    async fn create(&self, product_id: &str, input: &CreatePriceInput) -> Result<Price, DomainError>;
    async fn update(&self, id: &str, input: &UpdatePriceInput) -> Result<Price, DomainError>;
    async fn delete(&self, id: &str) -> Result<(), DomainError>;
}
