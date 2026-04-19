use async_trait::async_trait;
use sqlx::SqlitePool;
use std::sync::Arc;

use crate::domain::entity::warranty::{Warranty, CreateWarrantyInput};
use crate::domain::error::DomainError;
use crate::domain::port::warranty_repository::WarrantyRepository;
use crate::domain::entity::new_id;
use crate::adapter::outbound::sqlite::row_mapping::map_warranty;
use crate::adapter::outbound::sqlite::helpers::{map_err, now};

pub struct SqliteWarrantyRepository {
    pool: Arc<SqlitePool>,
}

impl SqliteWarrantyRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}

const WARRANTY_COLS: &str = "id, product_id, customer_name, customer_contact, purchase_date, warranty_days, expires_at, status, notes, created_at, updated_at";

#[async_trait]
impl WarrantyRepository for SqliteWarrantyRepository {
    async fn list(&self) -> Result<Vec<Warranty>, DomainError> {
        let rows = sqlx::query(&format!(
            "SELECT {} FROM warranties ORDER BY created_at DESC", WARRANTY_COLS
        ))
        .fetch_all(&*self.pool).await.map_err(map_err)?;
        Ok(rows.iter().map(map_warranty).collect())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Warranty>, DomainError> {
        let row = sqlx::query(&format!("SELECT {} FROM warranties WHERE id = ?", WARRANTY_COLS))
            .bind(id).fetch_optional(&*self.pool).await.map_err(map_err)?;
        Ok(row.as_ref().map(map_warranty))
    }

    async fn create(&self, input: &CreateWarrantyInput, expires_at: &str) -> Result<Warranty, DomainError> {
        let id = new_id();
        let ts = now();
        sqlx::query(
            "INSERT INTO warranties (id, product_id, customer_name, customer_contact, purchase_date, warranty_days, expires_at, status, notes, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, 'active', ?, ?, ?)"
        )
        .bind(&id).bind(&input.product_id).bind(&input.customer_name).bind(&input.customer_contact)
        .bind(&input.purchase_date).bind(input.warranty_days).bind(expires_at).bind(&input.notes)
        .bind(&ts).bind(&ts)
        .execute(&*self.pool).await.map_err(map_err)?;

        let row = sqlx::query(&format!("SELECT {} FROM warranties WHERE id = ?", WARRANTY_COLS))
            .bind(&id).fetch_one(&*self.pool).await.map_err(map_err)?;
        Ok(map_warranty(&row))
    }

    async fn update_status(&self, id: &str, status: &str) -> Result<Warranty, DomainError> {
        let ts = now();
        let result = sqlx::query("UPDATE warranties SET status = ?, updated_at = ? WHERE id = ?")
            .bind(status).bind(&ts).bind(id)
            .execute(&*self.pool).await.map_err(map_err)?;
        if result.rows_affected() == 0 {
            return Err(DomainError::NotFound(format!("Garantia com id '{}' não encontrada", id)));
        }
        let row = sqlx::query(&format!("SELECT {} FROM warranties WHERE id = ?", WARRANTY_COLS))
            .bind(id).fetch_one(&*self.pool).await.map_err(map_err)?;
        Ok(map_warranty(&row))
    }
}
