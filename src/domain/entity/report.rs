use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Resumo de vendas por produto.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SalesReportItem {
    pub product_id: String,
    pub product_name: String,
    pub sku: String,
    pub total_quantity: i32,
    pub total_revenue: f64,
}

/// Resumo geral de vendas.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SalesReport {
    pub items: Vec<SalesReportItem>,
    pub total_revenue: f64,
    pub total_items_sold: i64,
}

/// Produto no relatório de estoque.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StockReportItem {
    pub product_id: String,
    pub product_name: String,
    pub sku: String,
    pub quantity: i32,
    pub min_quantity: i32,
    pub status: String,
}

/// Resumo de devoluções.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReturnReportItem {
    pub id: String,
    pub product_id: String,
    pub product_name: String,
    pub reason: String,
    pub status: String,
    pub refund_amount: Option<f64>,
    pub created_at: String,
}

/// Parâmetros de filtro para relatórios.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, utoipa::IntoParams)]
pub struct ReportFilter {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}
