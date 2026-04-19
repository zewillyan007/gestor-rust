use axum::extract::{Path, State};
use axum::Json;
use sqlx::SqlitePool;

use crate::error::AppError;
use crate::models::stock::*;
use crate::models::product::new_id;

/// Retorna o estoque atual de um produto.
#[utoipa::path(
    get,
    path = "/api/products/{id}/stock",
    params(
        ("id" = String, Path, description = "ID do produto")
    ),
    responses(
        (status = 200, description = "Dados do estoque", body = Stock),
        (status = 404, description = "Estoque não encontrado"),
    ),
    tag = "Estoque"
)]
pub async fn get_stock(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<Stock>, AppError> {
    let stock = sqlx::query_as::<_, Stock>(
        "SELECT id, product_id, quantity, min_quantity, location, updated_at FROM stocks WHERE product_id = ?"
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Estoque do produto '{}' não encontrado", id)))?;

    Ok(Json(stock))
}

/// Inicializa o estoque de um produto.
#[utoipa::path(
    post,
    path = "/api/stocks",
    request_body = CreateStock,
    responses(
        (status = 201, description = "Estoque criado", body = Stock),
    ),
    tag = "Estoque"
)]
pub async fn create_stock(
    State(pool): State<SqlitePool>,
    Json(body): Json<CreateStock>,
) -> Result<(axum::http::StatusCode, Json<Stock>), AppError> {
    let mut tx = pool.begin().await?;

    // Verifica se o produto existe dentro da transação
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM products WHERE id = ?")
        .bind(&body.product_id)
        .fetch_one(&mut *tx)
        .await?;
    if count == 0 {
        return Err(AppError::NotFound(format!("Produto com id '{}' não encontrado", body.product_id)));
    }

    let id = new_id();
    let now = chrono::Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string();

    sqlx::query(
        "INSERT INTO stocks (id, product_id, quantity, min_quantity, location, updated_at) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&body.product_id)
    .bind(body.quantity.unwrap_or(0))
    .bind(body.min_quantity.unwrap_or(0))
    .bind(&body.location)
    .bind(&now)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    let stock = sqlx::query_as::<_, Stock>(
        "SELECT id, product_id, quantity, min_quantity, location, updated_at FROM stocks WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&pool)
    .await?;

    Ok((axum::http::StatusCode::CREATED, Json(stock)))
}

/// Registra uma movimentação de estoque e atualiza a quantidade.
#[utoipa::path(
    post,
    path = "/api/stock/movements",
    request_body = CreateStockMovement,
    responses(
        (status = 201, description = "Movimentação registrada", body = StockMovement),
        (status = 400, description = "Estoque insuficiente para saída"),
    ),
    tag = "Estoque"
)]
pub async fn create_stock_movement(
    State(pool): State<SqlitePool>,
    Json(body): Json<CreateStockMovement>,
) -> Result<(axum::http::StatusCode, Json<StockMovement>), AppError> {
    if body.quantity <= 0 {
        return Err(AppError::BadRequest("Quantidade deve ser maior que zero".to_string()));
    }

    let valid_types = ["in", "out"];
    if !valid_types.contains(&body.movement_type.as_str()) {
        return Err(AppError::BadRequest(
            "Tipo de movimentação inválido. Use: in, out".to_string()
        ));
    }

    // Inicia transação para evitar condição de corrida
    let mut tx = pool.begin().await?;

    // Verifica se existe registro de estoque
    let current: Option<Stock> = sqlx::query_as::<_, Stock>(
        "SELECT id, product_id, quantity, min_quantity, location, updated_at FROM stocks WHERE product_id = ?"
    )
    .bind(&body.product_id)
    .fetch_optional(&mut *tx)
    .await?;

    let current_qty = match &current {
        Some(s) => s.quantity,
        None => {
            let stock_id = new_id();
            let now = chrono::Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string();
            sqlx::query(
                "INSERT INTO stocks (id, product_id, quantity, min_quantity, location, updated_at) VALUES (?, ?, 0, 0, NULL, ?)"
            )
            .bind(&stock_id)
            .bind(&body.product_id)
            .bind(&now)
            .execute(&mut *tx)
            .await?;
            0
        }
    };

    let new_qty = match body.movement_type.as_str() {
        "in" => current_qty + body.quantity,
        "out" => {
            if current_qty < body.quantity {
                return Err(AppError::BadRequest(format!(
                    "Estoque insuficiente. Disponível: {}, Solicitado: {}",
                    current_qty, body.quantity
                )));
            }
            current_qty - body.quantity
        }
        _ => unreachable!("movement_type já validado acima como 'in' ou 'out'"),
    };

    let now = chrono::Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string();

    // Atualiza o estoque
    sqlx::query(
        "UPDATE stocks SET quantity = ?, updated_at = ? WHERE product_id = ?"
    )
    .bind(new_qty)
    .bind(&now)
    .bind(&body.product_id)
    .execute(&mut *tx)
    .await?;

    // Registra a movimentação
    let movement_id = new_id();
    sqlx::query(
        "INSERT INTO stock_movements (id, product_id, movement_type, quantity, reason, reference, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&movement_id)
    .bind(&body.product_id)
    .bind(&body.movement_type)
    .bind(body.quantity)
    .bind(&body.reason)
    .bind(&body.reference)
    .bind(&now)
    .execute(&mut *tx)
    .await?;

    // Commit da transação
    tx.commit().await?;

    let movement = sqlx::query_as::<_, StockMovement>(
        "SELECT id, product_id, movement_type, quantity, reason, reference, created_at FROM stock_movements WHERE id = ?"
    )
    .bind(&movement_id)
    .fetch_one(&pool)
    .await?;

    Ok((axum::http::StatusCode::CREATED, Json(movement)))
}

/// Lista todas as movimentações de estoque.
#[utoipa::path(
    get,
    path = "/api/stock/movements",
    responses(
        (status = 200, description = "Lista de movimentações", body = Vec<StockMovement>),
    ),
    tag = "Estoque"
)]
pub async fn list_stock_movements(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<StockMovement>>, AppError> {
    let movements = sqlx::query_as::<_, StockMovement>(
        "SELECT id, product_id, movement_type, quantity, reason, reference, created_at FROM stock_movements ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(movements))
}

/// Lista produtos com estoque abaixo do mínimo.
#[utoipa::path(
    get,
    path = "/api/stock/low",
    responses(
        (status = 200, description = "Produtos com estoque baixo", body = Vec<LowStockProduct>),
    ),
    tag = "Estoque"
)]
pub async fn list_low_stock(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<LowStockProduct>>, AppError> {
    let products = sqlx::query_as::<_, LowStockProduct>(
        "SELECT s.product_id, p.name as product_name, p.sku, s.quantity, s.min_quantity \
         FROM stocks s \
         JOIN products p ON p.id = s.product_id \
         WHERE s.quantity <= s.min_quantity \
         ORDER BY s.quantity ASC"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(products))
}
