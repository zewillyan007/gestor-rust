use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;

use crate::adapter::inbound::http::dto::*;
use crate::adapter::inbound::http::error::HttpError;
use crate::domain::entity::warranty::Warranty;
use crate::domain::usecase::warranty_usecase::WarrantyUseCase;

/// Lista todas as garantias.
#[utoipa::path(
    get,
    path = "/api/warranties",
    responses((status = 200, description = "Lista de garantias", body = Vec<Warranty>)),
    tag = "Garantias"
)]
pub async fn list_warranties(
    State(uc): State<Arc<WarrantyUseCase>>,
) -> Result<Json<Vec<Warranty>>, HttpError> {
    Ok(Json(uc.list().await?))
}

/// Busca uma garantia pelo ID.
#[utoipa::path(
    get,
    path = "/api/warranties/{id}",
    params(("id" = String, Path, description = "ID da garantia")),
    responses(
        (status = 200, description = "Garantia encontrada", body = Warranty),
        (status = 404, description = "Garantia não encontrada"),
    ),
    tag = "Garantias"
)]
pub async fn get_warranty(
    State(uc): State<Arc<WarrantyUseCase>>,
    Path(id): Path<String>,
) -> Result<Json<Warranty>, HttpError> {
    Ok(Json(uc.get(&id).await?))
}

/// Registra uma nova garantia.
#[utoipa::path(
    post,
    path = "/api/warranties",
    request_body = CreateWarrantyDto,
    responses(
        (status = 201, description = "Garantia criada", body = Warranty),
        (status = 404, description = "Produto não encontrado"),
    ),
    tag = "Garantias"
)]
pub async fn create_warranty(
    State(uc): State<Arc<WarrantyUseCase>>,
    Json(body): Json<CreateWarrantyDto>,
) -> Result<(axum::http::StatusCode, Json<Warranty>), HttpError> {
    let warranty = uc.create(body.into_input()).await?;
    Ok((axum::http::StatusCode::CREATED, Json(warranty)))
}

/// Atualiza o status de uma garantia.
#[utoipa::path(
    patch,
    path = "/api/warranties/{id}/status",
    params(("id" = String, Path, description = "ID da garantia")),
    request_body = UpdateWarrantyStatusDto,
    responses(
        (status = 200, description = "Status atualizado", body = Warranty),
        (status = 400, description = "Status inválido"),
        (status = 404, description = "Garantia não encontrada"),
    ),
    tag = "Garantias"
)]
pub async fn update_warranty_status(
    State(uc): State<Arc<WarrantyUseCase>>,
    Path(id): Path<String>,
    Json(body): Json<UpdateWarrantyStatusDto>,
) -> Result<Json<Warranty>, HttpError> {
    Ok(Json(uc.update_status(&id, &body.status).await?))
}
