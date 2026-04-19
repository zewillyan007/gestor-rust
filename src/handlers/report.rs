use axum::extract::{Query, State};
use axum::Json;
use sqlx::SqlitePool;

use crate::error::AppError;
use crate::models::report::*;

/// Relatório de vendas (total por produto).
#[utoipa::path(
    get,
    path = "/api/reports/sales",
    params(ReportFilter),
    responses(
        (status = 200, description = "Relatório de vendas", body = SalesReport),
    ),
    tag = "Relatórios"
)]
pub async fn sales_report(
    State(pool): State<SqlitePool>,
    Query(filter): Query<ReportFilter>,
) -> Result<Json<SalesReport>, AppError> {
    let items = if filter.start_date.is_some() && filter.end_date.is_some() {
        sqlx::query_as::<_, SalesReportItem>(
            "SELECT s.product_id, p.name as product_name, p.sku, \
             SUM(s.quantity) as total_quantity, SUM(s.total_price) as total_revenue \
             FROM sales s \
             JOIN products p ON p.id = s.product_id \
             WHERE DATE(s.sale_date) >= ? AND DATE(s.sale_date) <= ? \
             GROUP BY s.product_id, p.name, p.sku \
             ORDER BY total_revenue DESC"
        )
        .bind(&filter.start_date)
        .bind(&filter.end_date)
        .fetch_all(&pool)
        .await?
    } else {
        sqlx::query_as::<_, SalesReportItem>(
            "SELECT s.product_id, p.name as product_name, p.sku, \
             SUM(s.quantity) as total_quantity, SUM(s.total_price) as total_revenue \
             FROM sales s \
             JOIN products p ON p.id = s.product_id \
             GROUP BY s.product_id, p.name, p.sku \
             ORDER BY total_revenue DESC"
        )
        .fetch_all(&pool)
        .await?
    };

    let total_revenue: f64 = items.iter().map(|i| i.total_revenue).sum();
    let total_items_sold: i64 = items.iter().map(|i| i.total_quantity as i64).sum();

    Ok(Json(SalesReport {
        items,
        total_revenue,
        total_items_sold,
    }))
}

/// Relatório de estoque atual de todos os produtos.
#[utoipa::path(
    get,
    path = "/api/reports/stock",
    responses(
        (status = 200, description = "Relatório de estoque", body = Vec<StockReportItem>),
    ),
    tag = "Relatórios"
)]
pub async fn stock_report(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<StockReportItem>>, AppError> {
    let items = sqlx::query_as::<_, StockReportItem>(
        "SELECT p.id as product_id, p.name as product_name, p.sku, \
         COALESCE(s.quantity, 0) as quantity, COALESCE(s.min_quantity, 0) as min_quantity, \
         p.status \
         FROM products p \
         LEFT JOIN stocks s ON s.product_id = p.id \
         ORDER BY p.name"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(items))
}

/// Relatório de devoluções.
#[utoipa::path(
    get,
    path = "/api/reports/returns",
    params(ReportFilter),
    responses(
        (status = 200, description = "Relatório de devoluções", body = Vec<ReturnReportItem>),
    ),
    tag = "Relatórios"
)]
pub async fn returns_report(
    State(pool): State<SqlitePool>,
    Query(filter): Query<ReportFilter>,
) -> Result<Json<Vec<ReturnReportItem>>, AppError> {
    let items = if filter.start_date.is_some() && filter.end_date.is_some() {
        sqlx::query_as::<_, ReturnReportItem>(
            "SELECT r.id, r.product_id, p.name as product_name, r.reason, r.status, r.refund_amount, r.created_at \
             FROM returns r \
             JOIN products p ON p.id = r.product_id \
             WHERE DATE(r.created_at) >= ? AND DATE(r.created_at) <= ? \
             ORDER BY r.created_at DESC"
        )
        .bind(&filter.start_date)
        .bind(&filter.end_date)
        .fetch_all(&pool)
        .await?
    } else {
        sqlx::query_as::<_, ReturnReportItem>(
            "SELECT r.id, r.product_id, p.name as product_name, r.reason, r.status, r.refund_amount, r.created_at \
             FROM returns r \
             JOIN products p ON p.id = r.product_id \
             ORDER BY r.created_at DESC"
        )
        .fetch_all(&pool)
        .await?
    };

    Ok(Json(items))
}
