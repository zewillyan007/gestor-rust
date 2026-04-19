use async_trait::async_trait;
use sqlx::SqlitePool;
use std::sync::Arc;

use crate::domain::entity::price::{Price, CreatePriceInput, UpdatePriceInput};
use crate::domain::error::DomainError;
use crate::domain::port::price_repository::PriceRepository;
use crate::domain::entity::new_id;
use crate::adapter::outbound::sqlite::row_mapping::map_price;
use crate::adapter::outbound::sqlite::helpers::{map_err, now};

pub struct SqlitePriceRepository {
    pool: Arc<SqlitePool>,
}

impl SqlitePriceRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}

const PRICE_COLS: &str = "id, product_id, cost_price, sale_price, effective_date, created_at";

#[async_trait]
impl PriceRepository for SqlitePriceRepository {
    async fn list_by_product(&self, product_id: &str) -> Result<Vec<Price>, DomainError> {
        let rows = sqlx::query(&format!(
            "SELECT {} FROM prices WHERE product_id = ? ORDER BY effective_date DESC", PRICE_COLS
        ))
        .bind(product_id).fetch_all(&*self.pool).await.map_err(map_err)?;
        Ok(rows.iter().map(map_price).collect())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Price>, DomainError> {
        let row = sqlx::query(&format!("SELECT {} FROM prices WHERE id = ?", PRICE_COLS))
            .bind(id).fetch_optional(&*self.pool).await.map_err(map_err)?;
        Ok(row.as_ref().map(map_price))
    }

    async fn create(&self, product_id: &str, input: &CreatePriceInput) -> Result<Price, DomainError> {
        let id = new_id();
        let ts = now();
        sqlx::query(
            "INSERT INTO prices (id, product_id, cost_price, sale_price, effective_date, created_at) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&id).bind(product_id).bind(input.cost_price).bind(input.sale_price).bind(&input.effective_date).bind(&ts)
        .execute(&*self.pool).await.map_err(map_err)?;

        let row = sqlx::query(&format!("SELECT {} FROM prices WHERE id = ?", PRICE_COLS))
            .bind(&id).fetch_one(&*self.pool).await.map_err(map_err)?;
        Ok(map_price(&row))
    }

    async fn update(&self, id: &str, input: &UpdatePriceInput) -> Result<Price, DomainError> {
        let existing = self.find_by_id(id).await?.ok_or_else(|| {
            DomainError::NotFound(format!("Preço com id '{}' não encontrado", id))
        })?;
        let cost_price = input.cost_price.unwrap_or(existing.cost_price);
        let sale_price = input.sale_price.unwrap_or(existing.sale_price);
        let effective_date = input.effective_date.as_ref().cloned().unwrap_or(existing.effective_date);

        sqlx::query("UPDATE prices SET cost_price = ?, sale_price = ?, effective_date = ? WHERE id = ?")
            .bind(cost_price).bind(sale_price).bind(&effective_date).bind(id)
            .execute(&*self.pool).await.map_err(map_err)?;

        let row = sqlx::query(&format!("SELECT {} FROM prices WHERE id = ?", PRICE_COLS))
            .bind(id).fetch_one(&*self.pool).await.map_err(map_err)?;
        Ok(map_price(&row))
    }

    async fn delete(&self, id: &str) -> Result<(), DomainError> {
        let result = sqlx::query("DELETE FROM prices WHERE id = ?")
            .bind(id).execute(&*self.pool).await.map_err(map_err)?;
        if result.rows_affected() == 0 {
            return Err(DomainError::NotFound(format!("Preço com id '{}' não encontrado", id)));
        }
        Ok(())
    }
}
