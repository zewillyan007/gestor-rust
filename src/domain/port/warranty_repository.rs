use async_trait::async_trait;
use crate::domain::entity::warranty::{Warranty, CreateWarrantyInput};
use crate::domain::error::DomainError;

#[async_trait]
pub trait WarrantyRepository: Send + Sync {
    async fn list(&self) -> Result<Vec<Warranty>, DomainError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Warranty>, DomainError>;
    async fn create(&self, input: &CreateWarrantyInput, expires_at: &str) -> Result<Warranty, DomainError>;
    async fn update_status(&self, id: &str, status: &str) -> Result<Warranty, DomainError>;
}
