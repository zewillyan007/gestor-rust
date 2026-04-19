use axum::extract::{Path, State};
use axum::Json;
use sqlx::SqlitePool;

use crate::error::AppError;
use crate::models::return_model::*;
use crate::models::product::new_id;

/// Lista todas as devoluções.
#[utoipa::path(
    get,
    path = "/api/returns",
    responses(
        (status = 200, description = "Lista de devoluções", body = Vec<Return>),
    ),
    tag = "Devoluções"
)]
pub async fn list_returns(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Return>>, AppError> {
    let returns = sqlx::query_as::<_, Return>(
        "SELECT id, product_id, warranty_id, reason, status, refund_amount, created_at, updated_at FROM returns ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(returns))
}

/// Busca uma devolução pelo ID.
#[utoipa::path(
    get,
    path = "/api/returns/{id}",
    params(
        ("id" = String, Path, description = "ID da devolução")
    ),
    responses(
        (status = 200, description = "Devolução encontrada", body = Return),
        (status = 404, description = "Devolução não encontrada"),
    ),
    tag = "Devoluções"
)]
pub async fn get_return(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<Return>, AppError> {
    let ret = sqlx::query_as::<_, Return>(
        "SELECT id, product_id, warranty_id, reason, status, refund_amount, created_at, updated_at FROM returns WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Devolução com id '{}' não encontrada", id)))?;

    Ok(Json(ret))
}

/// Abre uma nova solicitação de devolução.
#[utoipa::path(
    post,
    path = "/api/returns",
    request_body = CreateReturn,
    responses(
        (status = 201, description = "Devolução criada", body = Return),
        (status = 404, description = "Produto não encontrado"),
    ),
    tag = "Devoluções"
)]
pub async fn create_return(
    State(pool): State<SqlitePool>,
    Json(body): Json<CreateReturn>,
) -> Result<(axum::http::StatusCode, Json<Return>), AppError> {
    // Verifica se o produto existe
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM products WHERE id = ?")
        .bind(&body.product_id)
        .fetch_one(&pool)
        .await?;
    let exists = count > 0;

    if !exists {
        return Err(AppError::NotFound(format!("Produto com id '{}' não encontrado", body.product_id)));
    }

    let id = new_id();
    let now = chrono::Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string();

    sqlx::query(
        "INSERT INTO returns (id, product_id, warranty_id, reason, status, refund_amount, created_at, updated_at) VALUES (?, ?, ?, ?, 'requested', ?, ?, ?)"
    )
    .bind(&id)
    .bind(&body.product_id)
    .bind(&body.warranty_id)
    .bind(&body.reason)
    .bind(body.refund_amount)
    .bind(&now)
    .bind(&now)
    .execute(&pool)
    .await?;

    let ret = sqlx::query_as::<_, Return>(
        "SELECT id, product_id, warranty_id, reason, status, refund_amount, created_at, updated_at FROM returns WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&pool)
    .await?;

    Ok((axum::http::StatusCode::CREATED, Json(ret)))
}

/// Atualiza o status de uma devolução.
#[utoipa::path(
    patch,
    path = "/api/returns/{id}/status",
    params(
        ("id" = String, Path, description = "ID da devolução")
    ),
    request_body = UpdateReturnStatus,
    responses(
        (status = 200, description = "Status atualizado", body = Return),
        (status = 400, description = "Status inválido"),
        (status = 404, description = "Devolução não encontrada"),
    ),
    tag = "Devoluções"
)]
pub async fn update_return_status(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(body): Json<UpdateReturnStatus>,
) -> Result<Json<Return>, AppError> {
    let valid_statuses = ["requested", "approved", "rejected", "completed"];
    if !valid_statuses.contains(&body.status.as_str()) {
        return Err(AppError::BadRequest(format!(
            "Status inválido. Use: {}",
            valid_statuses.join(", ")
        )));
    }

    let now = chrono::Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string();

    // Atualiza o refund_amount se fornecido
    if let Some(amount) = body.refund_amount {
        sqlx::query(
            "UPDATE returns SET status = ?, refund_amount = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&body.status)
        .bind(amount)
        .bind(&now)
        .bind(&id)
        .execute(&pool)
        .await?;
    } else {
        sqlx::query(
            "UPDATE returns SET status = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&body.status)
        .bind(&now)
        .bind(&id)
        .execute(&pool)
        .await?;
    }

    let ret = sqlx::query_as::<_, Return>(
        "SELECT id, product_id, warranty_id, reason, status, refund_amount, created_at, updated_at FROM returns WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(ret))
}
