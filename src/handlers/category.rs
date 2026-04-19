use axum::extract::{Path, State};
use axum::Json;
use sqlx::SqlitePool;

use crate::error::AppError;
use crate::models::category::*;
use crate::models::product::new_id;

/// Lista todas as categorias.
#[utoipa::path(
    get,
    path = "/api/categories",
    responses(
        (status = 200, description = "Lista de categorias", body = Vec<Category>),
    ),
    tag = "Categorias"
)]
pub async fn list_categories(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Category>>, AppError> {
    let categories = sqlx::query_as::<_, Category>(
        "SELECT id, name, description, parent_id, created_at, updated_at FROM categories ORDER BY name"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(categories))
}

/// Busca uma categoria pelo ID.
#[utoipa::path(
    get,
    path = "/api/categories/{id}",
    params(
        ("id" = String, Path, description = "ID da categoria")
    ),
    responses(
        (status = 200, description = "Categoria encontrada", body = Category),
        (status = 404, description = "Categoria não encontrada"),
    ),
    tag = "Categorias"
)]
pub async fn get_category(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<Category>, AppError> {
    let category = sqlx::query_as::<_, Category>(
        "SELECT id, name, description, parent_id, created_at, updated_at FROM categories WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Categoria com id '{}' não encontrada", id)))?;

    Ok(Json(category))
}

/// Cria uma nova categoria.
#[utoipa::path(
    post,
    path = "/api/categories",
    request_body = CreateCategory,
    responses(
        (status = 201, description = "Categoria criada", body = Category),
    ),
    tag = "Categorias"
)]
pub async fn create_category(
    State(pool): State<SqlitePool>,
    Json(body): Json<CreateCategory>,
) -> Result<(axum::http::StatusCode, Json<Category>), AppError> {
    let id = new_id();
    let now = chrono::Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string();

    sqlx::query(
        "INSERT INTO categories (id, name, description, parent_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&body.name)
    .bind(&body.description)
    .bind(&body.parent_id)
    .bind(&now)
    .bind(&now)
    .execute(&pool)
    .await?;

    let category = sqlx::query_as::<_, Category>(
        "SELECT id, name, description, parent_id, created_at, updated_at FROM categories WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&pool)
    .await?;

    Ok((axum::http::StatusCode::CREATED, Json(category)))
}

/// Atualiza uma categoria.
#[utoipa::path(
    put,
    path = "/api/categories/{id}",
    params(
        ("id" = String, Path, description = "ID da categoria")
    ),
    request_body = UpdateCategory,
    responses(
        (status = 200, description = "Categoria atualizada", body = Category),
        (status = 404, description = "Categoria não encontrada"),
    ),
    tag = "Categorias"
)]
pub async fn update_category(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(body): Json<UpdateCategory>,
) -> Result<Json<Category>, AppError> {
    let existing = sqlx::query_as::<_, Category>(
        "SELECT id, name, description, parent_id, created_at, updated_at FROM categories WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Categoria com id '{}' não encontrada", id)))?;

    let now = chrono::Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string();
    let name = body.name.unwrap_or(existing.name);
    let description = body.description.or(existing.description);
    let parent_id = body.parent_id.or(existing.parent_id);

    sqlx::query(
        "UPDATE categories SET name = ?, description = ?, parent_id = ?, updated_at = ? WHERE id = ?"
    )
    .bind(&name)
    .bind(&description)
    .bind(&parent_id)
    .bind(&now)
    .bind(&id)
    .execute(&pool)
    .await?;

    let category = sqlx::query_as::<_, Category>(
        "SELECT id, name, description, parent_id, created_at, updated_at FROM categories WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(category))
}

/// Remove uma categoria.
#[utoipa::path(
    delete,
    path = "/api/categories/{id}",
    params(
        ("id" = String, Path, description = "ID da categoria")
    ),
    responses(
        (status = 200, description = "Categoria removida"),
        (status = 404, description = "Categoria não encontrada"),
    ),
    tag = "Categorias"
)]
pub async fn delete_category(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let result = sqlx::query("DELETE FROM categories WHERE id = ?")
        .bind(&id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Categoria com id '{}' não encontrada", id)));
    }

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
    State(pool): State<SqlitePool>,
    Path((product_id, category_id)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Valida existência do produto
    let product_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM products WHERE id = ?")
        .bind(&product_id)
        .fetch_one(&pool)
        .await?;
    if product_count == 0 {
        return Err(AppError::NotFound(format!("Produto com id '{}' não encontrado", product_id)));
    }

    // Valida existência da categoria
    let category_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM categories WHERE id = ?")
        .bind(&category_id)
        .fetch_one(&pool)
        .await?;
    if category_count == 0 {
        return Err(AppError::NotFound(format!("Categoria com id '{}' não encontrada", category_id)));
    }

    sqlx::query(
        "INSERT OR IGNORE INTO product_categories (product_id, category_id) VALUES (?, ?)"
    )
    .bind(&product_id)
    .bind(&category_id)
    .execute(&pool)
    .await?;

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
    responses(
        (status = 200, description = "Associação removida"),
    ),
    tag = "Categorias"
)]
pub async fn unlink_product_category(
    State(pool): State<SqlitePool>,
    Path((product_id, category_id)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, AppError> {
    sqlx::query(
        "DELETE FROM product_categories WHERE product_id = ? AND category_id = ?"
    )
    .bind(&product_id)
    .bind(&category_id)
    .execute(&pool)
    .await?;

    Ok(Json(serde_json::json!({ "message": "Associação removida com sucesso" })))
}
