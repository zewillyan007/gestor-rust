use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;

use crate::adapter::inbound::http::dto::*;
use crate::adapter::inbound::http::error::HttpError;
use crate::domain::entity::return_model::Return;
use crate::domain::usecase::return_usecase::ReturnUseCase;

/// Lista todas as devoluções.
#[utoipa::path(
    get,
    path = "/api/returns",
    responses((status = 200, description = "Lista de devoluções", body = Vec<Return>)),
    tag = "Devoluções"
)]
pub async fn list_returns(
    State(uc): State<Arc<ReturnUseCase>>,
) -> Result<Json<Vec<Return>>, HttpError> {
    Ok(Json(uc.list().await?))
}

/// Busca uma devolução pelo ID.
#[utoipa::path(
    get,
    path = "/api/returns/{id}",
    params(("id" = String, Path, description = "ID da devolução")),
    responses(
        (status = 200, description = "Devolução encontrada", body = Return),
        (status = 404, description = "Devolução não encontrada"),
    ),
    tag = "Devoluções"
)]
pub async fn get_return(
    State(uc): State<Arc<ReturnUseCase>>,
    Path(id): Path<String>,
) -> Result<Json<Return>, HttpError> {
    Ok(Json(uc.get(&id).await?))
}

/// Abre uma nova solicitação de devolução.
#[utoipa::path(
    post,
    path = "/api/returns",
    request_body = CreateReturnDto,
    responses(
        (status = 201, description = "Devolução criada", body = Return),
        (status = 404, description = "Produto não encontrado"),
    ),
    tag = "Devoluções"
)]
pub async fn create_return(
    State(uc): State<Arc<ReturnUseCase>>,
    Json(body): Json<CreateReturnDto>,
) -> Result<(axum::http::StatusCode, Json<Return>), HttpError> {
    let ret = uc.create(body.into_input()).await?;
    Ok((axum::http::StatusCode::CREATED, Json(ret)))
}

/// Atualiza o status de uma devolução.
#[utoipa::path(
    patch,
    path = "/api/returns/{id}/status",
    params(("id" = String, Path, description = "ID da devolução")),
    request_body = UpdateReturnStatusDto,
    responses(
        (status = 200, description = "Status atualizado", body = Return),
        (status = 400, description = "Status inválido"),
        (status = 404, description = "Devolução não encontrada"),
    ),
    tag = "Devoluções"
)]
pub async fn update_return_status(
    State(uc): State<Arc<ReturnUseCase>>,
    Path(id): Path<String>,
    Json(body): Json<UpdateReturnStatusDto>,
) -> Result<Json<Return>, HttpError> {
    Ok(Json(uc.update_status(&id, body.into_input()).await?))
}
