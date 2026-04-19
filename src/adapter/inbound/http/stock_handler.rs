use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;

use crate::adapter::inbound::http::dto::*;
use crate::adapter::inbound::http::error::HttpError;
use crate::domain::entity::stock::{Stock, StockMovement, LowStockProduct};
use crate::domain::usecase::stock_usecase::StockUseCase;

/// Retorna o estoque atual de um produto.
#[utoipa::path(
    get,
    path = "/api/products/{id}/stock",
    params(("id" = String, Path, description = "ID do produto")),
    responses(
        (status = 200, description = "Dados do estoque", body = Stock),
        (status = 404, description = "Estoque não encontrado"),
    ),
    tag = "Estoque"
)]
pub async fn get_stock(
    State(uc): State<Arc<StockUseCase>>,
    Path(id): Path<String>,
) -> Result<Json<Stock>, HttpError> {
    Ok(Json(uc.get_by_product(&id).await?))
}

/// Inicializa o estoque de um produto.
#[utoipa::path(
    post,
    path = "/api/stocks",
    request_body = CreateStockDto,
    responses((status = 201, description = "Estoque criado", body = Stock)),
    tag = "Estoque"
)]
pub async fn create_stock(
    State(uc): State<Arc<StockUseCase>>,
    Json(body): Json<CreateStockDto>,
) -> Result<(axum::http::StatusCode, Json<Stock>), HttpError> {
    let stock = uc.create(body.into_input()).await?;
    Ok((axum::http::StatusCode::CREATED, Json(stock)))
}

/// Registra uma movimentação de estoque.
#[utoipa::path(
    post,
    path = "/api/stock/movements",
    request_body = CreateStockMovementDto,
    responses(
        (status = 201, description = "Movimentação registrada", body = StockMovement),
        (status = 400, description = "Estoque insuficiente para saída"),
    ),
    tag = "Estoque"
)]
pub async fn create_stock_movement(
    State(uc): State<Arc<StockUseCase>>,
    Json(body): Json<CreateStockMovementDto>,
) -> Result<(axum::http::StatusCode, Json<StockMovement>), HttpError> {
    let movement = uc.create_movement(body.into_input()).await?;
    Ok((axum::http::StatusCode::CREATED, Json(movement)))
}

/// Lista todas as movimentações de estoque.
#[utoipa::path(
    get,
    path = "/api/stock/movements",
    responses((status = 200, description = "Lista de movimentações", body = Vec<StockMovement>)),
    tag = "Estoque"
)]
pub async fn list_stock_movements(
    State(uc): State<Arc<StockUseCase>>,
) -> Result<Json<Vec<StockMovement>>, HttpError> {
    Ok(Json(uc.list_movements().await?))
}

/// Lista produtos com estoque abaixo do mínimo.
#[utoipa::path(
    get,
    path = "/api/stock/low",
    responses((status = 200, description = "Produtos com estoque baixo", body = Vec<LowStockProduct>)),
    tag = "Estoque"
)]
pub async fn list_low_stock(
    State(uc): State<Arc<StockUseCase>>,
) -> Result<Json<Vec<LowStockProduct>>, HttpError> {
    Ok(Json(uc.list_low_stock().await?))
}
