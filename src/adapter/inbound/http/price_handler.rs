use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;

use crate::adapter::inbound::http::dto::*;
use crate::adapter::inbound::http::error::HttpError;
use crate::domain::entity::price::Price;
use crate::domain::usecase::price_usecase::PriceUseCase;

/// Lista o histórico de preços de um produto.
#[utoipa::path(
    get,
    path = "/api/products/{id}/prices",
    params(("id" = String, Path, description = "ID do produto")),
    responses((status = 200, description = "Histórico de preços", body = Vec<Price>)),
    tag = "Preços"
)]
pub async fn list_prices(
    State(uc): State<Arc<PriceUseCase>>,
    Path(id): Path<String>,
) -> Result<Json<Vec<Price>>, HttpError> {
    Ok(Json(uc.list_by_product(&id).await?))
}

/// Define um novo preço para um produto.
#[utoipa::path(
    post,
    path = "/api/products/{id}/prices",
    params(("id" = String, Path, description = "ID do produto")),
    request_body = CreatePriceDto,
    responses(
        (status = 201, description = "Preço criado", body = Price),
        (status = 404, description = "Produto não encontrado"),
    ),
    tag = "Preços"
)]
pub async fn create_price(
    State(uc): State<Arc<PriceUseCase>>,
    Path(id): Path<String>,
    Json(body): Json<CreatePriceDto>,
) -> Result<(axum::http::StatusCode, Json<Price>), HttpError> {
    let price = uc.create(&id, body.into_input()).await?;
    Ok((axum::http::StatusCode::CREATED, Json(price)))
}

/// Atualiza um preço existente.
#[utoipa::path(
    put,
    path = "/api/prices/{id}",
    params(("id" = String, Path, description = "ID do preço")),
    request_body = UpdatePriceDto,
    responses(
        (status = 200, description = "Preço atualizado", body = Price),
        (status = 404, description = "Preço não encontrado"),
    ),
    tag = "Preços"
)]
pub async fn update_price(
    State(uc): State<Arc<PriceUseCase>>,
    Path(id): Path<String>,
    Json(body): Json<UpdatePriceDto>,
) -> Result<Json<Price>, HttpError> {
    Ok(Json(uc.update(&id, body.into_input()).await?))
}

/// Remove um preço do histórico.
#[utoipa::path(
    delete,
    path = "/api/prices/{id}",
    params(("id" = String, Path, description = "ID do preço")),
    responses(
        (status = 200, description = "Preço removido"),
        (status = 404, description = "Preço não encontrado"),
    ),
    tag = "Preços"
)]
pub async fn delete_price(
    State(uc): State<Arc<PriceUseCase>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, HttpError> {
    uc.delete(&id).await?;
    Ok(Json(serde_json::json!({ "message": "Preço removido com sucesso" })))
}
