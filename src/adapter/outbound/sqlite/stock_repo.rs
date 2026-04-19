use async_trait::async_trait;
use sqlx::{SqlitePool, Row};
use std::sync::Arc;

use crate::domain::entity::stock::{
    Stock, StockMovement, CreateStockMovementInput, CreateStockInput, LowStockProduct,
};
use crate::domain::error::DomainError;
use crate::domain::port::stock_repository::StockRepository;
use crate::domain::entity::new_id;
use crate::adapter::outbound::sqlite::row_mapping::{map_stock, map_stock_movement, map_low_stock_product};
use crate::adapter::outbound::sqlite::helpers::{map_err, now};

pub struct SqliteStockRepository {
    pool: Arc<SqlitePool>,
}

impl SqliteStockRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}

const STOCK_COLS: &str = "id, product_id, quantity, min_quantity, location, updated_at";
const MOVEMENT_COLS: &str = "id, product_id, movement_type, quantity, reason, reference, created_at";

#[async_trait]
impl StockRepository for SqliteStockRepository {
    async fn find_by_product(&self, product_id: &str) -> Result<Option<Stock>, DomainError> {
        let row = sqlx::query(&format!("SELECT {} FROM stocks WHERE product_id = ?", STOCK_COLS))
            .bind(product_id).fetch_optional(&*self.pool).await.map_err(map_err)?;
        Ok(row.as_ref().map(map_stock))
    }

    async fn create(&self, input: &CreateStockInput) -> Result<Stock, DomainError> {
        let id = new_id();
        let ts = now();
        sqlx::query(
            "INSERT INTO stocks (id, product_id, quantity, min_quantity, location, updated_at) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&id).bind(&input.product_id).bind(input.quantity.unwrap_or(0))
        .bind(input.min_quantity.unwrap_or(0)).bind(&input.location).bind(&ts)
        .execute(&*self.pool).await.map_err(map_err)?;

        let row = sqlx::query(&format!("SELECT {} FROM stocks WHERE id = ?", STOCK_COLS))
            .bind(&id).fetch_one(&*self.pool).await.map_err(map_err)?;
        Ok(map_stock(&row))
    }

    async fn update_quantity(&self, product_id: &str, quantity: i32) -> Result<(), DomainError> {
        let ts = now();
        sqlx::query("UPDATE stocks SET quantity = ?, updated_at = ? WHERE product_id = ?")
            .bind(quantity).bind(&ts).bind(product_id)
            .execute(&*self.pool).await.map_err(map_err)?;
        Ok(())
    }

    async fn create_or_get(&self, product_id: &str) -> Result<i32, DomainError> {
        if let Some(row) = sqlx::query(&format!("SELECT {} FROM stocks WHERE product_id = ?", STOCK_COLS))
            .bind(product_id).fetch_optional(&*self.pool).await.map_err(map_err)?
        {
            return Ok(map_stock(&row).quantity);
        }
        let id = new_id();
        let ts = now();
        sqlx::query(
            "INSERT INTO stocks (id, product_id, quantity, min_quantity, location, updated_at) VALUES (?, ?, 0, 0, NULL, ?)"
        )
        .bind(&id).bind(product_id).bind(&ts)
        .execute(&*self.pool).await.map_err(map_err)?;
        Ok(0)
    }

    async fn create_movement(&self, input: &CreateStockMovementInput) -> Result<StockMovement, DomainError> {
        let id = new_id();
        let ts = now();
        sqlx::query(
            "INSERT INTO stock_movements (id, product_id, movement_type, quantity, reason, reference, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&id).bind(&input.product_id).bind(&input.movement_type)
        .bind(input.quantity).bind(&input.reason).bind(&input.reference).bind(&ts)
        .execute(&*self.pool).await.map_err(map_err)?;

        let row = sqlx::query(&format!("SELECT {} FROM stock_movements WHERE id = ?", MOVEMENT_COLS))
            .bind(&id).fetch_one(&*self.pool).await.map_err(map_err)?;
        Ok(map_stock_movement(&row))
    }

    async fn list_movements(&self) -> Result<Vec<StockMovement>, DomainError> {
        let rows = sqlx::query(&format!(
            "SELECT {} FROM stock_movements ORDER BY created_at DESC", MOVEMENT_COLS
        ))
        .fetch_all(&*self.pool).await.map_err(map_err)?;
        Ok(rows.iter().map(map_stock_movement).collect())
    }

    async fn list_low_stock(&self) -> Result<Vec<LowStockProduct>, DomainError> {
        let rows = sqlx::query(
            "SELECT s.product_id, p.name as product_name, p.sku, s.quantity, s.min_quantity \
             FROM stocks s JOIN products p ON p.id = s.product_id \
             WHERE s.quantity <= s.min_quantity ORDER BY s.quantity ASC"
        )
        .fetch_all(&*self.pool).await.map_err(map_err)?;
        Ok(rows.iter().map(map_low_stock_product).collect())
    }

    async fn atomic_increment(&self, product_id: &str, delta: i32) -> Result<i32, DomainError> {
        let ts = now();
        // Garante que a linha existe
        self.create_or_get(product_id).await?;
        // UPDATE atômico
        let row = sqlx::query("UPDATE stocks SET quantity = quantity + ?, updated_at = ? WHERE product_id = ? RETURNING quantity")
            .bind(delta).bind(&ts).bind(product_id)
            .fetch_one(&*self.pool).await.map_err(map_err)?;
        let new_qty: i32 = row.get("quantity");
        Ok(new_qty)
    }

    async fn atomic_decrement(&self, product_id: &str, delta: i32) -> Result<i32, DomainError> {
        let ts = now();
        // Garante que a linha existe
        self.create_or_get(product_id).await?;
        // UPDATE atômico com verificação de saldo
        let row = sqlx::query(
            "UPDATE stocks SET quantity = quantity - ?, updated_at = ? WHERE product_id = ? AND quantity >= ? RETURNING quantity"
        )
        .bind(delta).bind(&ts).bind(product_id).bind(delta)
        .fetch_optional(&*self.pool).await.map_err(map_err)?;

        match row {
            Some(r) => {
                let new_qty: i32 = r.get("quantity");
                Ok(new_qty)
            }
            None => {
                // Busca quantidade atual para mensagem de erro
                let current = self.create_or_get(product_id).await?;
                Err(DomainError::BadRequest(format!(
                    "Estoque insuficiente. Disponível: {}, Solicitado: {}", current, delta
                )))
            }
        }
    }
}
