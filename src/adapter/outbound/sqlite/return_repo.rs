use async_trait::async_trait;
use sqlx::SqlitePool;
use std::sync::Arc;

use crate::domain::entity::return_model::{Return, CreateReturnInput, UpdateReturnStatusInput};
use crate::domain::error::DomainError;
use crate::domain::port::return_repository::ReturnRepository;
use crate::domain::entity::new_id;
use crate::adapter::outbound::sqlite::row_mapping::map_return;
use crate::adapter::outbound::sqlite::helpers::{map_err, now};

pub struct SqliteReturnRepository {
    pool: Arc<SqlitePool>,
}

impl SqliteReturnRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}

const RETURN_COLS: &str = "id, product_id, warranty_id, reason, status, refund_amount, created_at, updated_at";

#[async_trait]
impl ReturnRepository for SqliteReturnRepository {
    async fn list(&self) -> Result<Vec<Return>, DomainError> {
        let rows = sqlx::query(&format!(
            "SELECT {} FROM returns ORDER BY created_at DESC", RETURN_COLS
        ))
        .fetch_all(&*self.pool).await.map_err(map_err)?;
        Ok(rows.iter().map(map_return).collect())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Return>, DomainError> {
        let row = sqlx::query(&format!("SELECT {} FROM returns WHERE id = ?", RETURN_COLS))
            .bind(id).fetch_optional(&*self.pool).await.map_err(map_err)?;
        Ok(row.as_ref().map(map_return))
    }

    async fn create(&self, input: &CreateReturnInput) -> Result<Return, DomainError> {
        let id = new_id();
        let ts = now();
        sqlx::query(
            "INSERT INTO returns (id, product_id, warranty_id, reason, status, refund_amount, created_at, updated_at) VALUES (?, ?, ?, ?, 'requested', ?, ?, ?)"
        )
        .bind(&id).bind(&input.product_id).bind(&input.warranty_id).bind(&input.reason)
        .bind(input.refund_amount).bind(&ts).bind(&ts)
        .execute(&*self.pool).await.map_err(map_err)?;

        let row = sqlx::query(&format!("SELECT {} FROM returns WHERE id = ?", RETURN_COLS))
            .bind(&id).fetch_one(&*self.pool).await.map_err(map_err)?;
        Ok(map_return(&row))
    }

    async fn update_status(&self, id: &str, input: &UpdateReturnStatusInput) -> Result<Return, DomainError> {
        let ts = now();
        if let Some(amount) = input.refund_amount {
            sqlx::query("UPDATE returns SET status = ?, refund_amount = ?, updated_at = ? WHERE id = ?")
                .bind(&input.status).bind(amount).bind(&ts).bind(id)
                .execute(&*self.pool).await.map_err(map_err)?;
        } else {
            sqlx::query("UPDATE returns SET status = ?, updated_at = ? WHERE id = ?")
                .bind(&input.status).bind(&ts).bind(id)
                .execute(&*self.pool).await.map_err(map_err)?;
        }
        let row = sqlx::query(&format!("SELECT {} FROM returns WHERE id = ?", RETURN_COLS))
            .bind(id).fetch_one(&*self.pool).await.map_err(map_err)?;
        Ok(map_return(&row))
    }
}
