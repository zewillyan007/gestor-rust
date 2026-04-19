use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;

use crate::adapter::inbound::http::dto::*;
use crate::adapter::inbound::http::error::HttpError;
use crate::domain::entity::product::Product;
use crate::domain::usecase::product_usecase::ProductUseCase;

/// Lista todos os produtos cadastrados.
#[utoipa::path(
    get,
    path = "/api/products",
    responses(
        (status = 200, description = "Lista de produtos", body = Vec<Product>),
    ),
    tag = "Produtos"
)]
pub async fn list_products(
    State(uc): State<Arc<ProductUseCase>>,
) -> Result<Json<Vec<Product>>, HttpError> {
    let products = uc.list().await?;
    Ok(Json(products))
}

/// Busca um produto pelo ID.
#[utoipa::path(
    get,
    path = "/api/products/{id}",
    params(("id" = String, Path, description = "ID do produto")),
    responses(
        (status = 200, description = "Produto encontrado", body = Product),
        (status = 404, description = "Produto não encontrado"),
    ),
    tag = "Produtos"
)]
pub async fn get_product(
    State(uc): State<Arc<ProductUseCase>>,
    Path(id): Path<String>,
) -> Result<Json<Product>, HttpError> {
    let product = uc.get(&id).await?;
    Ok(Json(product))
}

/// Cria um novo produto.
#[utoipa::path(
    post,
    path = "/api/products",
    request_body = CreateProductDto,
    responses(
        (status = 201, description = "Produto criado com sucesso", body = Product),
        (status = 409, description = "SKU já existe"),
    ),
    tag = "Produtos"
)]
pub async fn create_product(
    State(uc): State<Arc<ProductUseCase>>,
    Json(body): Json<CreateProductDto>,
) -> Result<(axum::http::StatusCode, Json<Product>), HttpError> {
    let product = uc.create(body.into_input()).await?;
    Ok((axum::http::StatusCode::CREATED, Json(product)))
}

/// Atualiza os dados de um produto.
#[utoipa::path(
    put,
    path = "/api/products/{id}",
    params(("id" = String, Path, description = "ID do produto")),
    request_body = UpdateProductDto,
    responses(
        (status = 200, description = "Produto atualizado", body = Product),
        (status = 404, description = "Produto não encontrado"),
    ),
    tag = "Produtos"
)]
pub async fn update_product(
    State(uc): State<Arc<ProductUseCase>>,
    Path(id): Path<String>,
    Json(body): Json<UpdateProductDto>,
) -> Result<Json<Product>, HttpError> {
    let product = uc.update(&id, body.into_input()).await?;
    Ok(Json(product))
}

/// Altera o status de disponibilidade do produto.
#[utoipa::path(
    patch,
    path = "/api/products/{id}/status",
    params(("id" = String, Path, description = "ID do produto")),
    request_body = UpdateProductStatusDto,
    responses(
        (status = 200, description = "Status atualizado", body = Product),
        (status = 400, description = "Status inválido"),
        (status = 404, description = "Produto não encontrado"),
    ),
    tag = "Produtos"
)]
pub async fn update_product_status(
    State(uc): State<Arc<ProductUseCase>>,
    Path(id): Path<String>,
    Json(body): Json<UpdateProductStatusDto>,
) -> Result<Json<Product>, HttpError> {
    let product = uc.update_status(&id, &body.status).await?;
    Ok(Json(product))
}

/// Remove um produto.
#[utoipa::path(
    delete,
    path = "/api/products/{id}",
    params(("id" = String, Path, description = "ID do produto")),
    responses(
        (status = 200, description = "Produto removido"),
        (status = 404, description = "Produto não encontrado"),
    ),
    tag = "Produtos"
)]
pub async fn delete_product(
    State(uc): State<Arc<ProductUseCase>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, HttpError> {
    uc.delete(&id).await?;
    Ok(Json(serde_json::json!({ "message": "Produto removido com sucesso" })))
}
