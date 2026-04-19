use std::sync::Arc;

use axum::extract::State;
use axum::Json;

use crate::adapter::inbound::http::dto::*;
use crate::adapter::inbound::http::error::HttpError;
use crate::domain::entity::sale::Sale;
use crate::domain::usecase::sale_usecase::SaleUseCase;

/// Registra uma nova venda e atualiza o estoque.
#[utoipa::path(
    post,
    path = "/api/sales",
    request_body = CreateSaleDto,
    responses(
        (status = 201, description = "Venda registrada", body = Sale),
        (status = 400, description = "Estoque insuficiente ou dados inválidos"),
        (status = 404, description = "Produto não encontrado"),
    ),
    tag = "Vendas"
)]
pub async fn create_sale(
    State(uc): State<Arc<SaleUseCase>>,
    Json(body): Json<CreateSaleDto>,
) -> Result<(axum::http::StatusCode, Json<Sale>), HttpError> {
    let sale = uc.create(body.into_input()).await?;
    Ok((axum::http::StatusCode::CREATED, Json(sale)))
}
