use std::sync::Arc;

use crate::domain::entity::report::{
    SalesReport, StockReportItem, ReturnReportItem, ReportFilter,
};
use crate::domain::error::DomainError;
use crate::domain::port::report_repository::ReportRepository;

pub struct ReportUseCase {
    repo: Arc<dyn ReportRepository>,
}

impl ReportUseCase {
    pub fn new(repo: Arc<dyn ReportRepository>) -> Self {
        Self { repo }
    }

    pub async fn sales_report(&self, filter: ReportFilter) -> Result<SalesReport, DomainError> {
        if filter.start_date.is_some() != filter.end_date.is_some() {
            return Err(DomainError::BadRequest(
                "Forneça ambas as datas (início e fim) ou nenhuma".to_string()
            ));
        }
        let items = self.repo.sales_report(&filter).await?;
        let total_revenue: f64 = items.iter().map(|i| i.total_revenue).sum();
        let total_items_sold: i64 = items.iter().map(|i| i.total_quantity as i64).sum();
        Ok(SalesReport { items, total_revenue, total_items_sold })
    }

    pub async fn stock_report(&self) -> Result<Vec<StockReportItem>, DomainError> {
        self.repo.stock_report().await
    }

    pub async fn returns_report(&self, filter: ReportFilter) -> Result<Vec<ReturnReportItem>, DomainError> {
        if filter.start_date.is_some() != filter.end_date.is_some() {
            return Err(DomainError::BadRequest(
                "Forneça ambas as datas (início e fim) ou nenhuma".to_string()
            ));
        }
        self.repo.returns_report(&filter).await
    }
}
