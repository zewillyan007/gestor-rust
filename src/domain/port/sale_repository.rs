use async_trait::async_trait;
use crate::domain::entity::sale::{Sale, CreateSaleInput};
use crate::domain::error::DomainError;

#[async_trait]
pub trait SaleRepository: Send + Sync {
    async fn create(&self, input: &CreateSaleInput, total_price: f64) -> Result<Sale, DomainError>;
}
