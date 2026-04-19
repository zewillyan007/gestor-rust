use axum::extract::{Path, State};
use axum::Json;
use sqlx::SqlitePool;

use crate::error::AppError;
use crate::models::warranty::*;
use crate::models::product::new_id;

/// Lista todas as garantias.
#[utoipa::path(
    get,
    path = "/api/warranties",
    responses(
        (status = 200, description = "Lista de garantias", body = Vec<Warranty>),
    ),
    tag = "Garantias"
)]
pub async fn list_warranties(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Warranty>>, AppError> {
    let warranties = sqlx::query_as::<_, Warranty>(
        "SELECT id, product_id, customer_name, customer_contact, purchase_date, warranty_days, expires_at, status, notes, created_at, updated_at FROM warranties ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(warranties))
}

/// Busca uma garantia pelo ID.
#[utoipa::path(
    get,
    path = "/api/warranties/{id}",
    params(
        ("id" = String, Path, description = "ID da garantia")
    ),
    responses(
        (status = 200, description = "Garantia encontrada", body = Warranty),
        (status = 404, description = "Garantia não encontrada"),
    ),
    tag = "Garantias"
)]
pub async fn get_warranty(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<Warranty>, AppError> {
    let warranty = sqlx::query_as::<_, Warranty>(
        "SELECT id, product_id, customer_name, customer_contact, purchase_date, warranty_days, expires_at, status, notes, created_at, updated_at FROM warranties WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Garantia com id '{}' não encontrada", id)))?;

    Ok(Json(warranty))
}

/// Registra uma nova garantia.
#[utoipa::path(
    post,
    path = "/api/warranties",
    request_body = CreateWarranty,
    responses(
        (status = 201, description = "Garantia criada", body = Warranty),
        (status = 404, description = "Produto não encontrado"),
    ),
    tag = "Garantias"
)]
pub async fn create_warranty(
    State(pool): State<SqlitePool>,
    Json(body): Json<CreateWarranty>,
) -> Result<(axum::http::StatusCode, Json<Warranty>), AppError> {
    // Verifica se o produto existe
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM products WHERE id = ?")
        .bind(&body.product_id)
        .fetch_one(&pool)
        .await?;
    let exists = count > 0;

    if !exists {
        return Err(AppError::NotFound(format!("Produto com id '{}' não encontrado", body.product_id)));
    }

    if body.warranty_days <= 0 {
        return Err(AppError::BadRequest("Dias de garantia deve ser maior que zero".to_string()));
    }

    let id = new_id();
    let now = chrono::Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string();

    // Calcula data de expiração
    let purchase_date = chrono::NaiveDate::parse_from_str(&body.purchase_date, "%Y-%m-%d")
        .map_err(|_| AppError::BadRequest("Formato de data inválido. Use YYYY-MM-DD".to_string()))?;
    let expires_at = purchase_date + chrono::Duration::days(body.warranty_days as i64);
    let expires_at_str = expires_at.format("%Y-%m-%d").to_string();

    sqlx::query(
        "INSERT INTO warranties (id, product_id, customer_name, customer_contact, purchase_date, warranty_days, expires_at, status, notes, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, 'active', ?, ?, ?)"
    )
    .bind(&id)
    .bind(&body.product_id)
    .bind(&body.customer_name)
    .bind(&body.customer_contact)
    .bind(&body.purchase_date)
    .bind(body.warranty_days)
    .bind(&expires_at_str)
    .bind(&body.notes)
    .bind(&now)
    .bind(&now)
    .execute(&pool)
    .await?;

    let warranty = sqlx::query_as::<_, Warranty>(
        "SELECT id, product_id, customer_name, customer_contact, purchase_date, warranty_days, expires_at, status, notes, created_at, updated_at FROM warranties WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&pool)
    .await?;

    Ok((axum::http::StatusCode::CREATED, Json(warranty)))
}

/// Atualiza o status de uma garantia.
#[utoipa::path(
    patch,
    path = "/api/warranties/{id}/status",
    params(
        ("id" = String, Path, description = "ID da garantia")
    ),
    request_body = UpdateWarrantyStatus,
    responses(
        (status = 200, description = "Status atualizado", body = Warranty),
        (status = 400, description = "Status inválido"),
        (status = 404, description = "Garantia não encontrada"),
    ),
    tag = "Garantias"
)]
pub async fn update_warranty_status(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(body): Json<UpdateWarrantyStatus>,
) -> Result<Json<Warranty>, AppError> {
    let valid_statuses = ["active", "expired", "claimed"];
    if !valid_statuses.contains(&body.status.as_str()) {
        return Err(AppError::BadRequest(format!(
            "Status inválido. Use: {}",
            valid_statuses.join(", ")
        )));
    }

    let now = chrono::Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string();

    let result = sqlx::query(
        "UPDATE warranties SET status = ?, updated_at = ? WHERE id = ?"
    )
    .bind(&body.status)
    .bind(&now)
    .bind(&id)
    .execute(&pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Garantia com id '{}' não encontrada", id)));
    }

    let warranty = sqlx::query_as::<_, Warranty>(
        "SELECT id, product_id, customer_name, customer_contact, purchase_date, warranty_days, expires_at, status, notes, created_at, updated_at FROM warranties WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(warranty))
}
