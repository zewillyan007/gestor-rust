use axum::extract::{Path, State};
use axum::Json;
use sqlx::SqlitePool;

use crate::error::AppError;
use crate::models::product::*;

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
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Product>>, AppError> {
    let products = sqlx::query_as::<_, Product>(
        "SELECT id, name, description, sku, brand, status, created_at, updated_at FROM products ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(products))
}

/// Busca um produto pelo ID.
#[utoipa::path(
    get,
    path = "/api/products/{id}",
    params(
        ("id" = String, Path, description = "ID do produto")
    ),
    responses(
        (status = 200, description = "Produto encontrado", body = Product),
        (status = 404, description = "Produto não encontrado"),
    ),
    tag = "Produtos"
)]
pub async fn get_product(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<Product>, AppError> {
    let product = sqlx::query_as::<_, Product>(
        "SELECT id, name, description, sku, brand, status, created_at, updated_at FROM products WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Produto com id '{}' não encontrado", id)))?;

    Ok(Json(product))
}

/// Cria um novo produto.
#[utoipa::path(
    post,
    path = "/api/products",
    request_body = CreateProduct,
    responses(
        (status = 201, description = "Produto criado com sucesso", body = Product),
        (status = 409, description = "SKU já existe"),
    ),
    tag = "Produtos"
)]
pub async fn create_product(
    State(pool): State<SqlitePool>,
    Json(body): Json<CreateProduct>,
) -> Result<(axum::http::StatusCode, Json<Product>), AppError> {
    let id = new_id();
    let now = chrono::Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string();

    sqlx::query(
        "INSERT INTO products (id, name, description, sku, brand, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, 'available', ?, ?)"
    )
    .bind(&id)
    .bind(&body.name)
    .bind(&body.description)
    .bind(&body.sku)
    .bind(&body.brand)
    .bind(&now)
    .bind(&now)
    .execute(&pool)
    .await?;

    let product = sqlx::query_as::<_, Product>(
        "SELECT id, name, description, sku, brand, status, created_at, updated_at FROM products WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&pool)
    .await?;

    Ok((axum::http::StatusCode::CREATED, Json(product)))
}

/// Atualiza os dados de um produto.
#[utoipa::path(
    put,
    path = "/api/products/{id}",
    params(
        ("id" = String, Path, description = "ID do produto")
    ),
    request_body = UpdateProduct,
    responses(
        (status = 200, description = "Produto atualizado", body = Product),
        (status = 404, description = "Produto não encontrado"),
    ),
    tag = "Produtos"
)]
pub async fn update_product(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(body): Json<UpdateProduct>,
) -> Result<Json<Product>, AppError> {
    let existing = sqlx::query_as::<_, Product>(
        "SELECT id, name, description, sku, brand, status, created_at, updated_at FROM products WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Produto com id '{}' não encontrado", id)))?;

    let now = chrono::Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string();
    let name = body.name.unwrap_or(existing.name);
    let description = body.description.or(existing.description);
    let brand = body.brand.or(existing.brand);

    sqlx::query(
        "UPDATE products SET name = ?, description = ?, brand = ?, updated_at = ? WHERE id = ?"
    )
    .bind(&name)
    .bind(&description)
    .bind(&brand)
    .bind(&now)
    .bind(&id)
    .execute(&pool)
    .await?;

    let product = sqlx::query_as::<_, Product>(
        "SELECT id, name, description, sku, brand, status, created_at, updated_at FROM products WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(product))
}

/// Altera o status de disponibilidade do produto.
#[utoipa::path(
    patch,
    path = "/api/products/{id}/status",
    params(
        ("id" = String, Path, description = "ID do produto")
    ),
    request_body = UpdateProductStatus,
    responses(
        (status = 200, description = "Status atualizado", body = Product),
        (status = 400, description = "Status inválido"),
        (status = 404, description = "Produto não encontrado"),
    ),
    tag = "Produtos"
)]
pub async fn update_product_status(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(body): Json<UpdateProductStatus>,
) -> Result<Json<Product>, AppError> {
    let valid_statuses = ["available", "unavailable", "discontinued"];
    if !valid_statuses.contains(&body.status.as_str()) {
        return Err(AppError::BadRequest(format!(
            "Status inválido. Use: {}",
            valid_statuses.join(", ")
        )));
    }

    let now = chrono::Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string();

    let result = sqlx::query(
        "UPDATE products SET status = ?, updated_at = ? WHERE id = ?"
    )
    .bind(&body.status)
    .bind(&now)
    .bind(&id)
    .execute(&pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Produto com id '{}' não encontrado", id)));
    }

    let product = sqlx::query_as::<_, Product>(
        "SELECT id, name, description, sku, brand, status, created_at, updated_at FROM products WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(product))
}

/// Remove um produto.
#[utoipa::path(
    delete,
    path = "/api/products/{id}",
    params(
        ("id" = String, Path, description = "ID do produto")
    ),
    responses(
        (status = 200, description = "Produto removido"),
        (status = 404, description = "Produto não encontrado"),
    ),
    tag = "Produtos"
)]
pub async fn delete_product(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let result = sqlx::query("DELETE FROM products WHERE id = ?")
        .bind(&id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Produto com id '{}' não encontrado", id)));
    }

    Ok(Json(serde_json::json!({ "message": "Produto removido com sucesso" })))
}
