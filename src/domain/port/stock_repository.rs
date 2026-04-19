use async_trait::async_trait;
use crate::domain::entity::stock::{
    Stock, StockMovement, CreateStockMovementInput, CreateStockInput, LowStockProduct,
};
use crate::domain::error::DomainError;

#[async_trait]
pub trait StockRepository: Send + Sync {
    async fn find_by_product(&self, product_id: &str) -> Result<Option<Stock>, DomainError>;
    async fn create(&self, input: &CreateStockInput) -> Result<Stock, DomainError>;
    /// **DEPRECATED**: Use `atomic_increment` ou `atomic_decrement` para evitar TOCTOU.
    /// Este método faz SET absoluto e será removido em versão futura.
    async fn update_quantity(&self, product_id: &str, quantity: i32) -> Result<(), DomainError>;
    async fn create_or_get(&self, product_id: &str) -> Result<i32, DomainError>;
    async fn create_movement(&self, input: &CreateStockMovementInput) -> Result<StockMovement, DomainError>;
    async fn list_movements(&self) -> Result<Vec<StockMovement>, DomainError>;
    async fn list_low_stock(&self) -> Result<Vec<LowStockProduct>, DomainError>;

    /// Incrementa o estoque atomicamente. Retorna a nova quantidade.
    async fn atomic_increment(&self, product_id: &str, delta: i32) -> Result<i32, DomainError>;
    /// Decrementa o estoque atomicamente, verificando que há saldo suficiente.
    /// Retorna a nova quantidade, ou erro se estoque insuficiente.
    async fn atomic_decrement(&self, product_id: &str, delta: i32) -> Result<i32, DomainError>;
}
