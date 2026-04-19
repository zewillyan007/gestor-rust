use axum::extract::State;
use axum::Json;
use sqlx::SqlitePool;

use crate::error::AppError;
use crate::models::sale::*;
use crate::models::product::new_id;

/// Registra uma nova venda e atualiza o estoque (tudo em transação).
#[utoipa::path(
    post,
    path = "/api/sales",
    request_body = CreateSale,
    responses(
        (status = 201, description = "Venda registrada", body = Sale),
        (status = 400, description = "Estoque insuficiente ou dados inválidos"),
        (status = 404, description = "Produto não encontrado"),
    ),
    tag = "Vendas"
)]
pub async fn create_sale(
    State(pool): State<SqlitePool>,
    Json(body): Json<CreateSale>,
) -> Result<(axum::http::StatusCode, Json<Sale>), AppError> {
    // Validações de entrada
    if body.quantity <= 0 {
        return Err(AppError::BadRequest("Quantidade deve ser maior que zero".to_string()));
    }
    if body.unit_price <= 0.0 {
        return Err(AppError::BadRequest("Preço unitário deve ser maior que zero".to_string()));
    }

    // Inicia transação para evitar condição de corrida
    let mut tx = pool.begin().await?;

    // Verifica se o produto existe e está disponível
    let product_status: Option<String> = sqlx::query_scalar(
        "SELECT status FROM products WHERE id = ?"
    )
    .bind(&body.product_id)
    .fetch_optional(&mut *tx)
    .await?;

    match product_status {
        None => return Err(AppError::NotFound(format!("Produto com id '{}' não encontrado", body.product_id))),
        Some(s) if s != "available" => return Err(AppError::BadRequest("Produto não está disponível para venda".to_string())),
        _ => {}
    }

    // Verifica estoque
    let current_stock: Option<i32> = sqlx::query_scalar(
        "SELECT quantity FROM stocks WHERE product_id = ?"
    )
    .bind(&body.product_id)
    .fetch_optional(&mut *tx)
    .await?;

    let available = current_stock.unwrap_or(0);
    if available < body.quantity {
        return Err(AppError::BadRequest(format!(
            "Estoque insuficiente. Disponível: {}, Solicitado: {}",
            available, body.quantity
        )));
    }

    let id = new_id();
    let now = chrono::Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string();
    let total_price = body.unit_price * body.quantity as f64;

    // Registra a venda
    sqlx::query(
        "INSERT INTO sales (id, product_id, quantity, unit_price, total_price, sale_date, customer_name, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&body.product_id)
    .bind(body.quantity)
    .bind(body.unit_price)
    .bind(total_price)
    .bind(&now)
    .bind(&body.customer_name)
    .bind(&now)
    .execute(&mut *tx)
    .await?;

    // Atualiza o estoque (saída)
    let new_qty = available - body.quantity;
    sqlx::query(
        "UPDATE stocks SET quantity = ?, updated_at = ? WHERE product_id = ?"
    )
    .bind(new_qty)
    .bind(&now)
    .bind(&body.product_id)
    .execute(&mut *tx)
    .await?;

    // Registra movimentação de estoque
    let movement_id = new_id();
    sqlx::query(
        "INSERT INTO stock_movements (id, product_id, movement_type, quantity, reason, reference, created_at) VALUES (?, ?, 'out', ?, 'Venda', ?, ?)"
    )
    .bind(&movement_id)
    .bind(&body.product_id)
    .bind(body.quantity)
    .bind(&id)
    .bind(&now)
    .execute(&mut *tx)
    .await?;

    // Commit da transação
    tx.commit().await?;

    // Busca a venda criada para retorno (fora da transação)
    let sale = sqlx::query_as::<_, Sale>(
        "SELECT id, product_id, quantity, unit_price, total_price, sale_date, customer_name, created_at FROM sales WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&pool)
    .await?;

    Ok((axum::http::StatusCode::CREATED, Json(sale)))
}
