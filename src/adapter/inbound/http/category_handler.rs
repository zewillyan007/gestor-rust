use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;

use crate::adapter::inbound::http::dto::*;
use crate::adapter::inbound::http::error::HttpError;
use crate::domain::entity::category::Category;
use crate::domain::usecase::category_usecase::CategoryUseCase;

/// Lista todas as categorias.
#[utoipa::path(
    get,
    path = "/api/categories",
    responses((status = 200, description = "Lista de categorias", body = Vec<Category>)),
    tag = "Categorias"
)]
pub async fn list_categories(
    State(uc): State<Arc<CategoryUseCase>>,
) -> Result<Json<Vec<Category>>, HttpError> {
    Ok(Json(uc.list().await?))
}

/// Busca uma categoria pelo ID.
#[utoipa::path(
    get,
    path = "/api/categories/{id}",
    params(("id" = String, Path, description = "ID da categoria")),
    responses(
        (status = 200, description = "Categoria encontrada", body = Category),
        (status = 404, description = "Categoria não encontrada"),
    ),
    tag = "Categorias"
)]
pub async fn get_category(
    State(uc): State<Arc<CategoryUseCase>>,
    Path(id): Path<String>,
) -> Result<Json<Category>, HttpError> {
    Ok(Json(uc.get(&id).await?))
}

/// Cria uma nova categoria.
#[utoipa::path(
    post,
    path = "/api/categories",
    request_body = CreateCategoryDto,
    responses((status = 201, description = "Categoria criada", body = Category)),
    tag = "Categorias"
)]
pub async fn create_category(
    State(uc): State<Arc<CategoryUseCase>>,
    Json(body): Json<CreateCategoryDto>,
) -> Result<(axum::http::StatusCode, Json<Category>), HttpError> {
    let cat = uc.create(body.into_input()).await?;
    Ok((axum::http::StatusCode::CREATED, Json(cat)))
}

/// Atualiza uma categoria.
#[utoipa::path(
    put,
    path = "/api/categories/{id}",
    params(("id" = String, Path, description = "ID da categoria")),
    request_body = UpdateCategoryDto,
    responses(
        (status = 200, description = "Categoria atualizada", body = Category),
        (status = 404, description = "Categoria não encontrada"),
    ),
    tag = "Categorias"
)]
pub async fn update_category(
    State(uc): State<Arc<CategoryUseCase>>,
    Path(id): Path<String>,
    Json(body): Json<UpdateCategoryDto>,
) -> Result<Json<Category>, HttpError> {
    Ok(Json(uc.update(&id, body.into_input()).await?))
}

/// Remove uma categoria.
#[utoipa::path(
    delete,
    path = "/api/categories/{id}",
    params(("id" = String, Path, description = "ID da categoria")),
    responses(
        (status = 200, description = "Categoria removida"),
        (status = 404, description = "Categoria não encontrada"),
    ),
    tag = "Categorias"
)]
pub async fn delete_category(
    State(uc): State<Arc<CategoryUseCase>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, HttpError> {
    uc.delete(&id).await?;
    Ok(Json(serde_json::json!({ "message": "Categoria removida com sucesso" })))
}

/// Associa um produto a uma categoria.
#[utoipa::path(
    post,
    path = "/api/products/{product_id}/categories/{category_id}",
    params(
        ("product_id" = String, Path, description = "ID do produto"),
        ("category_id" = String, Path, description = "ID da categoria")
    ),
    responses(
        (status = 200, description = "Associação criada"),
        (status = 404, description = "Produto ou categoria não encontrado"),
    ),
    tag = "Categorias"
)]
pub async fn link_product_category(
    State(uc): State<Arc<CategoryUseCase>>,
    Path((product_id, category_id)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, HttpError> {
    uc.link_product(&product_id, &category_id).await?;
    Ok(Json(serde_json::json!({ "message": "Produto associado à categoria com sucesso" })))
}

/// Remove a associação de um produto com uma categoria.
#[utoipa::path(
    delete,
    path = "/api/products/{product_id}/categories/{category_id}",
    params(
        ("product_id" = String, Path, description = "ID do produto"),
        ("category_id" = String, Path, description = "ID da categoria")
    ),
    responses((status = 200, description = "Associação removida")),
    tag = "Categorias"
)]
pub async fn unlink_product_category(
    State(uc): State<Arc<CategoryUseCase>>,
    Path((product_id, category_id)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, HttpError> {
    uc.unlink_product(&product_id, &category_id).await?;
    Ok(Json(serde_json::json!({ "message": "Associação removida com sucesso" })))
}
