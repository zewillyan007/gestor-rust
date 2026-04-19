use async_trait::async_trait;
use sqlx::SqlitePool;
use std::sync::Arc;

use crate::domain::entity::product::{Product, CreateProductInput, UpdateProductInput};
use crate::domain::error::DomainError;
use crate::domain::port::product_repository::ProductRepository;
use crate::domain::entity::new_id;
use crate::adapter::outbound::sqlite::row_mapping::map_product;
use crate::adapter::outbound::sqlite::helpers::{map_err, now};

pub struct SqliteProductRepository {
    pool: Arc<SqlitePool>,
}

impl SqliteProductRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}

const PRODUCT_COLS: &str = "id, name, description, sku, brand, status, created_at, updated_at";

#[async_trait]
impl ProductRepository for SqliteProductRepository {
    async fn list(&self) -> Result<Vec<Product>, DomainError> {
        let rows = sqlx::query(&format!(
            "SELECT {} FROM products ORDER BY created_at DESC", PRODUCT_COLS
        ))
        .fetch_all(&*self.pool)
        .await
        .map_err(map_err)?;
        Ok(rows.iter().map(map_product).collect())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Product>, DomainError> {
        let row = sqlx::query(&format!(
            "SELECT {} FROM products WHERE id = ?", PRODUCT_COLS
        ))
        .bind(id)
        .fetch_optional(&*self.pool)
        .await
        .map_err(map_err)?;
        Ok(row.as_ref().map(map_product))
    }

    async fn find_by_sku(&self, sku: &str) -> Result<Option<Product>, DomainError> {
        let row = sqlx::query(&format!(
            "SELECT {} FROM products WHERE sku = ?", PRODUCT_COLS
        ))
        .bind(sku)
        .fetch_optional(&*self.pool)
        .await
        .map_err(map_err)?;
        Ok(row.as_ref().map(map_product))
    }

    async fn create(&self, input: &CreateProductInput) -> Result<Product, DomainError> {
        let id = new_id();
        let ts = now();
        sqlx::query(
            "INSERT INTO products (id, name, description, sku, brand, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, 'available', ?, ?)"
        )
        .bind(&id).bind(&input.name).bind(&input.description).bind(&input.sku).bind(&input.brand).bind(&ts).bind(&ts)
        .execute(&*self.pool).await.map_err(map_err)?;

        let row = sqlx::query(&format!("SELECT {} FROM products WHERE id = ?", PRODUCT_COLS))
            .bind(&id).fetch_one(&*self.pool).await.map_err(map_err)?;
        Ok(map_product(&row))
    }

    async fn update(&self, id: &str, input: &UpdateProductInput) -> Result<Product, DomainError> {
        let existing = self.find_by_id(id).await?.ok_or_else(|| {
            DomainError::NotFound(format!("Produto com id '{}' não encontrado", id))
        })?;
        let ts = now();
        let name = input.name.as_ref().unwrap_or(&existing.name);
        let description = input.description.as_ref().or(existing.description.as_ref());
        let brand = input.brand.as_ref().or(existing.brand.as_ref());

        sqlx::query("UPDATE products SET name = ?, description = ?, brand = ?, updated_at = ? WHERE id = ?")
            .bind(name).bind(description).bind(brand).bind(&ts).bind(id)
            .execute(&*self.pool).await.map_err(map_err)?;

        let row = sqlx::query(&format!("SELECT {} FROM products WHERE id = ?", PRODUCT_COLS))
            .bind(id).fetch_one(&*self.pool).await.map_err(map_err)?;
        Ok(map_product(&row))
    }

    async fn update_status(&self, id: &str, status: &str) -> Result<Product, DomainError> {
        let ts = now();
        let result = sqlx::query("UPDATE products SET status = ?, updated_at = ? WHERE id = ?")
            .bind(status).bind(&ts).bind(id)
            .execute(&*self.pool).await.map_err(map_err)?;
        if result.rows_affected() == 0 {
            return Err(DomainError::NotFound(format!("Produto com id '{}' não encontrado", id)));
        }
        let row = sqlx::query(&format!("SELECT {} FROM products WHERE id = ?", PRODUCT_COLS))
            .bind(id).fetch_one(&*self.pool).await.map_err(map_err)?;
        Ok(map_product(&row))
    }

    async fn delete(&self, id: &str) -> Result<(), DomainError> {
        let result = sqlx::query("DELETE FROM products WHERE id = ?")
            .bind(id).execute(&*self.pool).await.map_err(map_err)?;
        if result.rows_affected() == 0 {
            return Err(DomainError::NotFound(format!("Produto com id '{}' não encontrado", id)));
        }
        Ok(())
    }

    async fn exists(&self, id: &str) -> Result<bool, DomainError> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM products WHERE id = ?")
            .bind(id).fetch_one(&*self.pool).await.map_err(map_err)?;
        Ok(count > 0)
    }
}
