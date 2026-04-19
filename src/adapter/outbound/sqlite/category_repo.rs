use async_trait::async_trait;
use sqlx::SqlitePool;
use std::sync::Arc;

use crate::domain::entity::category::{Category, CreateCategoryInput, UpdateCategoryInput};
use crate::domain::error::DomainError;
use crate::domain::port::category_repository::CategoryRepository;
use crate::domain::entity::new_id;
use crate::adapter::outbound::sqlite::row_mapping::map_category;
use crate::adapter::outbound::sqlite::helpers::{map_err, now};

pub struct SqliteCategoryRepository {
    pool: Arc<SqlitePool>,
}

impl SqliteCategoryRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}

const CATEGORY_COLS: &str = "id, name, description, parent_id, created_at, updated_at";

#[async_trait]
impl CategoryRepository for SqliteCategoryRepository {
    async fn list(&self) -> Result<Vec<Category>, DomainError> {
        let rows = sqlx::query(&format!(
            "SELECT {} FROM categories ORDER BY name", CATEGORY_COLS
        ))
        .fetch_all(&*self.pool).await.map_err(map_err)?;
        Ok(rows.iter().map(map_category).collect())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Category>, DomainError> {
        let row = sqlx::query(&format!("SELECT {} FROM categories WHERE id = ?", CATEGORY_COLS))
            .bind(id).fetch_optional(&*self.pool).await.map_err(map_err)?;
        Ok(row.as_ref().map(map_category))
    }

    async fn create(&self, input: &CreateCategoryInput) -> Result<Category, DomainError> {
        let id = new_id();
        let ts = now();
        sqlx::query(
            "INSERT INTO categories (id, name, description, parent_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&id).bind(&input.name).bind(&input.description).bind(&input.parent_id).bind(&ts).bind(&ts)
        .execute(&*self.pool).await.map_err(map_err)?;

        let row = sqlx::query(&format!("SELECT {} FROM categories WHERE id = ?", CATEGORY_COLS))
            .bind(&id).fetch_one(&*self.pool).await.map_err(map_err)?;
        Ok(map_category(&row))
    }

    async fn update(&self, id: &str, input: &UpdateCategoryInput) -> Result<Category, DomainError> {
        let existing = self.find_by_id(id).await?.ok_or_else(|| {
            DomainError::NotFound(format!("Categoria com id '{}' não encontrada", id))
        })?;
        let ts = now();
        let name = input.name.as_ref().unwrap_or(&existing.name);
        let description = input.description.as_ref().or(existing.description.as_ref());
        let parent_id = input.parent_id.as_ref().or(existing.parent_id.as_ref());

        sqlx::query("UPDATE categories SET name = ?, description = ?, parent_id = ?, updated_at = ? WHERE id = ?")
            .bind(name).bind(description).bind(parent_id).bind(&ts).bind(id)
            .execute(&*self.pool).await.map_err(map_err)?;

        let row = sqlx::query(&format!("SELECT {} FROM categories WHERE id = ?", CATEGORY_COLS))
            .bind(id).fetch_one(&*self.pool).await.map_err(map_err)?;
        Ok(map_category(&row))
    }

    async fn delete(&self, id: &str) -> Result<(), DomainError> {
        let result = sqlx::query("DELETE FROM categories WHERE id = ?")
            .bind(id).execute(&*self.pool).await.map_err(map_err)?;
        if result.rows_affected() == 0 {
            return Err(DomainError::NotFound(format!("Categoria com id '{}' não encontrada", id)));
        }
        Ok(())
    }

    async fn exists(&self, id: &str) -> Result<bool, DomainError> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM categories WHERE id = ?")
            .bind(id).fetch_one(&*self.pool).await.map_err(map_err)?;
        Ok(count > 0)
    }

    async fn link_product(&self, product_id: &str, category_id: &str) -> Result<(), DomainError> {
        sqlx::query("INSERT OR IGNORE INTO product_categories (product_id, category_id) VALUES (?, ?)")
            .bind(product_id).bind(category_id)
            .execute(&*self.pool).await.map_err(map_err)?;
        Ok(())
    }

    async fn unlink_product(&self, product_id: &str, category_id: &str) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM product_categories WHERE product_id = ? AND category_id = ?")
            .bind(product_id).bind(category_id)
            .execute(&*self.pool).await.map_err(map_err)?;
        Ok(())
    }
}
