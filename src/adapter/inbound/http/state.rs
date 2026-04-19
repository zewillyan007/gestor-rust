use std::sync::Arc;

use axum::extract::FromRef;

use crate::domain::usecase::product_usecase::ProductUseCase;
use crate::domain::usecase::category_usecase::CategoryUseCase;
use crate::domain::usecase::price_usecase::PriceUseCase;
use crate::domain::usecase::stock_usecase::StockUseCase;
use crate::domain::usecase::warranty_usecase::WarrantyUseCase;
use crate::domain::usecase::return_usecase::ReturnUseCase;
use crate::domain::usecase::sale_usecase::SaleUseCase;
use crate::domain::usecase::report_usecase::ReportUseCase;
use crate::infrastructure::server::AppState;

impl FromRef<AppState> for Arc<ProductUseCase> {
    fn from_ref(state: &AppState) -> Self { state.product_uc.clone() }
}

impl FromRef<AppState> for Arc<CategoryUseCase> {
    fn from_ref(state: &AppState) -> Self { state.category_uc.clone() }
}

impl FromRef<AppState> for Arc<PriceUseCase> {
    fn from_ref(state: &AppState) -> Self { state.price_uc.clone() }
}

impl FromRef<AppState> for Arc<StockUseCase> {
    fn from_ref(state: &AppState) -> Self { state.stock_uc.clone() }
}

impl FromRef<AppState> for Arc<WarrantyUseCase> {
    fn from_ref(state: &AppState) -> Self { state.warranty_uc.clone() }
}

impl FromRef<AppState> for Arc<ReturnUseCase> {
    fn from_ref(state: &AppState) -> Self { state.return_uc.clone() }
}

impl FromRef<AppState> for Arc<SaleUseCase> {
    fn from_ref(state: &AppState) -> Self { state.sale_uc.clone() }
}

impl FromRef<AppState> for Arc<ReportUseCase> {
    fn from_ref(state: &AppState) -> Self { state.report_uc.clone() }
}
