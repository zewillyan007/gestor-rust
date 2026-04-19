use async_trait::async_trait;
use sqlx::SqlitePool;
use std::sync::Arc;

use crate::domain::entity::report::{
    SalesReportItem, StockReportItem, ReturnReportItem, ReportFilter,
};
use crate::domain::error::DomainError;
use crate::domain::port::report_repository::ReportRepository;
use crate::adapter::outbound::sqlite::row_mapping::{
    map_sales_report_item, map_stock_report_item, map_return_report_item,
};
use crate::adapter::outbound::sqlite::helpers::map_err;

pub struct SqliteReportRepository {
    pool: Arc<SqlitePool>,
}

impl SqliteReportRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReportRepository for SqliteReportRepository {
    async fn sales_report(&self, filter: &ReportFilter) -> Result<Vec<SalesReportItem>, DomainError> {
        let rows = if filter.start_date.is_some() && filter.end_date.is_some() {
            sqlx::query(
                "SELECT s.product_id, p.name as product_name, p.sku, \
                 SUM(s.quantity) as total_quantity, SUM(s.total_price) as total_revenue \
                 FROM sales s JOIN products p ON p.id = s.product_id \
                 WHERE DATE(s.sale_date) >= ? AND DATE(s.sale_date) <= ? \
                 GROUP BY s.product_id, p.name, p.sku ORDER BY total_revenue DESC"
            )
            .bind(&filter.start_date).bind(&filter.end_date)
            .fetch_all(&*self.pool).await.map_err(map_err)?
        } else {
            sqlx::query(
                "SELECT s.product_id, p.name as product_name, p.sku, \
                 SUM(s.quantity) as total_quantity, SUM(s.total_price) as total_revenue \
                 FROM sales s JOIN products p ON p.id = s.product_id \
                 GROUP BY s.product_id, p.name, p.sku ORDER BY total_revenue DESC"
            )
            .fetch_all(&*self.pool).await.map_err(map_err)?
        };
        Ok(rows.iter().map(map_sales_report_item).collect())
    }

    async fn stock_report(&self) -> Result<Vec<StockReportItem>, DomainError> {
        let rows = sqlx::query(
            "SELECT p.id as product_id, p.name as product_name, p.sku, \
             COALESCE(s.quantity, 0) as quantity, COALESCE(s.min_quantity, 0) as min_quantity, \
             p.status FROM products p LEFT JOIN stocks s ON s.product_id = p.id ORDER BY p.name"
        )
        .fetch_all(&*self.pool).await.map_err(map_err)?;
        Ok(rows.iter().map(map_stock_report_item).collect())
    }

    async fn returns_report(&self, filter: &ReportFilter) -> Result<Vec<ReturnReportItem>, DomainError> {
        let rows = if filter.start_date.is_some() && filter.end_date.is_some() {
            sqlx::query(
                "SELECT r.id, r.product_id, p.name as product_name, r.reason, r.status, r.refund_amount, r.created_at \
                 FROM returns r JOIN products p ON p.id = r.product_id \
                 WHERE DATE(r.created_at) >= ? AND DATE(r.created_at) <= ? ORDER BY r.created_at DESC"
            )
            .bind(&filter.start_date).bind(&filter.end_date)
            .fetch_all(&*self.pool).await.map_err(map_err)?
        } else {
            sqlx::query(
                "SELECT r.id, r.product_id, p.name as product_name, r.reason, r.status, r.refund_amount, r.created_at \
                 FROM returns r JOIN products p ON p.id = r.product_id ORDER BY r.created_at DESC"
            )
            .fetch_all(&*self.pool).await.map_err(map_err)?
        };
        Ok(rows.iter().map(map_return_report_item).collect())
    }
}
