use axum::extract::{Path, State};
use axum::Json;
use sqlx::SqlitePool;

use crate::error::AppError;
use crate::models::price::*;
use crate::models::product::new_id;

/// Lista o histórico de preços de um produto.
#[utoipa::path(
    get,
    path = "/api/products/{id}/prices",
    params(
        ("id" = String, Path, description = "ID do produto")
    ),
    responses(
        (status = 200, description = "Histórico de preços", body = Vec<Price>),
        (status = 404, description = "Produto não encontrado"),
    ),
    tag = "Preços"
)]
pub async fn list_prices(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<Vec<Price>>, AppError> {
    let prices = sqlx::query_as::<_, Price>(
        "SELECT id, product_id, cost_price, sale_price, effective_date, created_at FROM prices WHERE product_id = ? ORDER BY effective_date DESC"
    )
    .bind(&id)
    .fetch_all(&pool)
    .await?;

    Ok(Json(prices))
}

/// Define um novo preço para um produto.
#[utoipa::path(
    post,
    path = "/api/products/{id}/prices",
    params(
        ("id" = String, Path, description = "ID do produto")
    ),
    request_body = CreatePrice,
    responses(
        (status = 201, description = "Preço criado", body = Price),
        (status = 404, description = "Produto não encontrado"),
    ),
    tag = "Preços"
)]
pub async fn create_price(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(body): Json<CreatePrice>,
) -> Result<(axum::http::StatusCode, Json<Price>), AppError> {
    // Verifica se o produto existe
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM products WHERE id = ?")
        .bind(&id)
        .fetch_one(&pool)
        .await?;
    let exists = count > 0;

    if !exists {
        return Err(AppError::NotFound(format!("Produto com id '{}' não encontrado", id)));
    }

    let price_id = new_id();
    let now = chrono::Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string();

    sqlx::query(
        "INSERT INTO prices (id, product_id, cost_price, sale_price, effective_date, created_at) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&price_id)
    .bind(&id)
    .bind(body.cost_price)
    .bind(body.sale_price)
    .bind(&body.effective_date)
    .bind(&now)
    .execute(&pool)
    .await?;

    let price = sqlx::query_as::<_, Price>(
        "SELECT id, product_id, cost_price, sale_price, effective_date, created_at FROM prices WHERE id = ?"
    )
    .bind(&price_id)
    .fetch_one(&pool)
    .await?;

    Ok((axum::http::StatusCode::CREATED, Json(price)))
}

/// Atualiza um preço existente.
#[utoipa::path(
    put,
    path = "/api/prices/{id}",
    params(
        ("id" = String, Path, description = "ID do preço")
    ),
    request_body = UpdatePrice,
    responses(
        (status = 200, description = "Preço atualizado", body = Price),
        (status = 404, description = "Preço não encontrado"),
    ),
    tag = "Preços"
)]
pub async fn update_price(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(body): Json<UpdatePrice>,
) -> Result<Json<Price>, AppError> {
    let existing = sqlx::query_as::<_, Price>(
        "SELECT id, product_id, cost_price, sale_price, effective_date, created_at FROM prices WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Preço com id '{}' não encontrado", id)))?;

    let cost_price = body.cost_price.unwrap_or(existing.cost_price);
    let sale_price = body.sale_price.unwrap_or(existing.sale_price);
    let effective_date = body.effective_date.unwrap_or(existing.effective_date);

    sqlx::query(
        "UPDATE prices SET cost_price = ?, sale_price = ?, effective_date = ? WHERE id = ?"
    )
    .bind(cost_price)
    .bind(sale_price)
    .bind(&effective_date)
    .bind(&id)
    .execute(&pool)
    .await?;

    let price = sqlx::query_as::<_, Price>(
        "SELECT id, product_id, cost_price, sale_price, effective_date, created_at FROM prices WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(price))
}

/// Remove um preço do histórico.
#[utoipa::path(
    delete,
    path = "/api/prices/{id}",
    params(
        ("id" = String, Path, description = "ID do preço")
    ),
    responses(
        (status = 200, description = "Preço removido"),
        (status = 404, description = "Preço não encontrado"),
    ),
    tag = "Preços"
)]
pub async fn delete_price(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let result = sqlx::query("DELETE FROM prices WHERE id = ?")
        .bind(&id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Preço com id '{}' não encontrado", id)));
    }

    Ok(Json(serde_json::json!({ "message": "Preço removido com sucesso" })))
}
