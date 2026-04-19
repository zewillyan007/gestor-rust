use async_trait::async_trait;
use crate::domain::entity::return_model::{Return, CreateReturnInput, UpdateReturnStatusInput};
use crate::domain::error::DomainError;

#[async_trait]
pub trait ReturnRepository: Send + Sync {
    async fn list(&self) -> Result<Vec<Return>, DomainError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Return>, DomainError>;
    async fn create(&self, input: &CreateReturnInput) -> Result<Return, DomainError>;
    async fn update_status(&self, id: &str, input: &UpdateReturnStatusInput) -> Result<Return, DomainError>;
}
