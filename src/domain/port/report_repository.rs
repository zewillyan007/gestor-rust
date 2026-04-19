use async_trait::async_trait;
use crate::domain::entity::report::{
    SalesReportItem, StockReportItem, ReturnReportItem, ReportFilter,
};
use crate::domain::error::DomainError;

#[async_trait]
pub trait ReportRepository: Send + Sync {
    async fn sales_report(&self, filter: &ReportFilter) -> Result<Vec<SalesReportItem>, DomainError>;
    async fn stock_report(&self) -> Result<Vec<StockReportItem>, DomainError>;
    async fn returns_report(&self, filter: &ReportFilter) -> Result<Vec<ReturnReportItem>, DomainError>;
}
