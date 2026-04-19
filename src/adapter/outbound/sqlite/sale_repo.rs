use async_trait::async_trait;
use sqlx::SqlitePool;
use std::sync::Arc;

use crate::domain::entity::sale::{Sale, CreateSaleInput};
use crate::domain::error::DomainError;
use crate::domain::port::sale_repository::SaleRepository;
use crate::domain::entity::new_id;
use crate::adapter::outbound::sqlite::row_mapping::map_sale;
use crate::adapter::outbound::sqlite::helpers::{map_err, now};

pub struct SqliteSaleRepository {
    pool: Arc<SqlitePool>,
}

impl SqliteSaleRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}

const SALE_COLS: &str = "id, product_id, quantity, unit_price, total_price, sale_date, customer_name, created_at";

#[async_trait]
impl SaleRepository for SqliteSaleRepository {
    async fn create(&self, input: &CreateSaleInput, total_price: f64) -> Result<Sale, DomainError> {
        let id = new_id();
        let ts = now();
        sqlx::query(
            "INSERT INTO sales (id, product_id, quantity, unit_price, total_price, sale_date, customer_name, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&id).bind(&input.product_id).bind(input.quantity).bind(input.unit_price)
        .bind(total_price).bind(&ts).bind(&input.customer_name).bind(&ts)
        .execute(&*self.pool).await.map_err(map_err)?;

        let row = sqlx::query(&format!("SELECT {} FROM sales WHERE id = ?", SALE_COLS))
            .bind(&id).fetch_one(&*self.pool).await.map_err(map_err)?;
        Ok(map_sale(&row))
    }
}
