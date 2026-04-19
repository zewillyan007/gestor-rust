use std::sync::Arc;

use axum::extract::{Query, State};
use axum::Json;

use crate::adapter::inbound::http::dto::*;
use crate::adapter::inbound::http::error::HttpError;
use crate::domain::entity::report::{SalesReport, StockReportItem, ReturnReportItem};
use crate::domain::usecase::report_usecase::ReportUseCase;

/// Relatório de vendas (total por produto).
#[utoipa::path(
    get,
    path = "/api/reports/sales",
    params(ReportFilterDto),
    responses((status = 200, description = "Relatório de vendas", body = SalesReport)),
    tag = "Relatórios"
)]
pub async fn sales_report(
    State(uc): State<Arc<ReportUseCase>>,
    Query(filter): Query<ReportFilterDto>,
) -> Result<Json<SalesReport>, HttpError> {
    Ok(Json(uc.sales_report(filter.into_filter()).await?))
}

/// Relatório de estoque atual de todos os produtos.
#[utoipa::path(
    get,
    path = "/api/reports/stock",
    responses((status = 200, description = "Relatório de estoque", body = Vec<StockReportItem>)),
    tag = "Relatórios"
)]
pub async fn stock_report(
    State(uc): State<Arc<ReportUseCase>>,
) -> Result<Json<Vec<StockReportItem>>, HttpError> {
    Ok(Json(uc.stock_report().await?))
}

/// Relatório de devoluções.
#[utoipa::path(
    get,
    path = "/api/reports/returns",
    params(ReportFilterDto),
    responses((status = 200, description = "Relatório de devoluções", body = Vec<ReturnReportItem>)),
    tag = "Relatórios"
)]
pub async fn returns_report(
    State(uc): State<Arc<ReportUseCase>>,
    Query(filter): Query<ReportFilterDto>,
) -> Result<Json<Vec<ReturnReportItem>>, HttpError> {
    Ok(Json(uc.returns_report(filter.into_filter()).await?))
}
