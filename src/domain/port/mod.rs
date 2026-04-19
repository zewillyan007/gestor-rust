pub mod product_repository;
pub mod category_repository;
pub mod price_repository;
pub mod stock_repository;
pub mod warranty_repository;
pub mod return_repository;
pub mod sale_repository;
pub mod report_repository;
pub mod unit_of_work;

use async_trait::async_trait;
use crate::domain::error::DomainError;

/// Fábrica de UnitOfWork — cria uma nova transação.
#[async_trait]
pub trait UnitOfWorkFactory: Send + Sync {
    async fn begin(&self) -> Result<Box<dyn UnitOfWork>, DomainError>;
}

/// UnitOfWork — encapsula uma transação e fornece acesso a todos os repositórios.
#[async_trait]
pub trait UnitOfWork: Send {
    fn products(&mut self) -> &mut dyn product_repository::ProductRepository;
    fn categories(&mut self) -> &mut dyn category_repository::CategoryRepository;
    fn prices(&mut self) -> &mut dyn price_repository::PriceRepository;
    fn stocks(&mut self) -> &mut dyn stock_repository::StockRepository;
    fn warranties(&mut self) -> &mut dyn warranty_repository::WarrantyRepository;
    fn returns(&mut self) -> &mut dyn return_repository::ReturnRepository;
    fn sales(&mut self) -> &mut dyn sale_repository::SaleRepository;
    fn reports(&mut self) -> &mut dyn report_repository::ReportRepository;
    async fn commit(self: Box<Self>) -> Result<(), DomainError>;
    async fn rollback(self: Box<Self>) -> Result<(), DomainError>;
}
