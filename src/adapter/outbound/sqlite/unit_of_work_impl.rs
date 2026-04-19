use async_trait::async_trait;
use sqlx::{SqlitePool, Sqlite, Row};
use tokio::sync::Mutex;
use std::sync::Arc;

use crate::domain::error::DomainError;
use crate::domain::entity::product::{Product, CreateProductInput, UpdateProductInput};
use crate::domain::entity::category::{Category, CreateCategoryInput, UpdateCategoryInput};
use crate::domain::entity::price::{Price, CreatePriceInput, UpdatePriceInput};
use crate::domain::entity::stock::{
    Stock, StockMovement, CreateStockMovementInput, CreateStockInput, LowStockProduct,
};
use crate::domain::entity::warranty::{Warranty, CreateWarrantyInput};
use crate::domain::entity::return_model::{Return, CreateReturnInput, UpdateReturnStatusInput};
use crate::domain::entity::sale::{Sale, CreateSaleInput};
use crate::domain::entity::report::{SalesReportItem, StockReportItem, ReturnReportItem, ReportFilter};
use crate::domain::port::{UnitOfWorkFactory, UnitOfWork};
use crate::domain::port::product_repository::ProductRepository;
use crate::domain::port::category_repository::CategoryRepository;
use crate::domain::port::price_repository::PriceRepository;
use crate::domain::port::stock_repository::StockRepository;
use crate::domain::port::warranty_repository::WarrantyRepository;
use crate::domain::port::return_repository::ReturnRepository;
use crate::domain::port::sale_repository::SaleRepository;
use crate::domain::port::report_repository::ReportRepository;
use crate::domain::entity::new_id;
use crate::adapter::outbound::sqlite::row_mapping::*;
use crate::adapter::outbound::sqlite::helpers::{map_err, now};

// ============================================================
// Transactional repo wrappers
// ============================================================

struct TxProductRepo { tx: Arc<Mutex<Option<sqlx::Transaction<'static, Sqlite>>>> }
#[allow(dead_code)]
struct TxCategoryRepo { tx: Arc<Mutex<Option<sqlx::Transaction<'static, Sqlite>>>> }
#[allow(dead_code)]
struct TxPriceRepo { tx: Arc<Mutex<Option<sqlx::Transaction<'static, Sqlite>>>> }
struct TxStockRepo { tx: Arc<Mutex<Option<sqlx::Transaction<'static, Sqlite>>>> }
#[allow(dead_code)]
struct TxWarrantyRepo { tx: Arc<Mutex<Option<sqlx::Transaction<'static, Sqlite>>>> }
#[allow(dead_code)]
struct TxReturnRepo { tx: Arc<Mutex<Option<sqlx::Transaction<'static, Sqlite>>>> }
struct TxSaleRepo { tx: Arc<Mutex<Option<sqlx::Transaction<'static, Sqlite>>>> }
#[allow(dead_code)]
struct TxReportRepo { tx: Arc<Mutex<Option<sqlx::Transaction<'static, Sqlite>>>> }

const PRODUCT_COLS: &str = "id, name, description, sku, brand, status, created_at, updated_at";

// --- Product ---

#[async_trait]
impl ProductRepository for TxProductRepo {
    async fn list(&self) -> Result<Vec<Product>, DomainError> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or_else(|| DomainError::Internal("Transação já finalizada".to_string()))?;
        let rows = sqlx::query(&format!("SELECT {} FROM products ORDER BY created_at DESC", PRODUCT_COLS))
            .fetch_all(&mut **tx).await.map_err(map_err)?;
        Ok(rows.iter().map(map_product).collect())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Product>, DomainError> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or_else(|| DomainError::Internal("Transação já finalizada".to_string()))?;
        let row = sqlx::query(&format!("SELECT {} FROM products WHERE id = ?", PRODUCT_COLS))
            .bind(id).fetch_optional(&mut **tx).await.map_err(map_err)?;
        Ok(row.as_ref().map(map_product))
    }

    async fn find_by_sku(&self, _sku: &str) -> Result<Option<Product>, DomainError> {
        // Not needed in transactional context for sale
        Ok(None)
    }

    async fn create(&self, _input: &CreateProductInput) -> Result<Product, DomainError> {
        Err(DomainError::Internal("Não implementado em contexto transacional".to_string()))
    }

    async fn update(&self, _id: &str, _input: &UpdateProductInput) -> Result<Product, DomainError> {
        Err(DomainError::Internal("Não implementado em contexto transacional".to_string()))
    }

    async fn update_status(&self, _id: &str, _status: &str) -> Result<Product, DomainError> {
        Err(DomainError::Internal("Não implementado em contexto transacional".to_string()))
    }

    async fn delete(&self, _id: &str) -> Result<(), DomainError> {
        Err(DomainError::Internal("Não implementado em contexto transacional".to_string()))
    }

    async fn exists(&self, id: &str) -> Result<bool, DomainError> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or_else(|| DomainError::Internal("Transação já finalizada".to_string()))?;
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM products WHERE id = ?")
            .bind(id).fetch_one(&mut **tx).await.map_err(map_err)?;
        Ok(count > 0)
    }
}

// --- Stock ---

#[async_trait]
impl StockRepository for TxStockRepo {
    async fn find_by_product(&self, _product_id: &str) -> Result<Option<Stock>, DomainError> {
        Err(DomainError::Internal("Não implementado em contexto transacional".to_string()))
    }

    async fn create(&self, _input: &CreateStockInput) -> Result<Stock, DomainError> {
        Err(DomainError::Internal("Não implementado em contexto transacional".to_string()))
    }

    async fn update_quantity(&self, product_id: &str, quantity: i32) -> Result<(), DomainError> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or_else(|| DomainError::Internal("Transação já finalizada".to_string()))?;
        let ts = now();
        sqlx::query("UPDATE stocks SET quantity = ?, updated_at = ? WHERE product_id = ?")
            .bind(quantity).bind(&ts).bind(product_id)
            .execute(&mut **tx).await.map_err(map_err)?;
        Ok(())
    }

    async fn create_or_get(&self, product_id: &str) -> Result<i32, DomainError> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or_else(|| DomainError::Internal("Transação já finalizada".to_string()))?;

        let row = sqlx::query("SELECT quantity FROM stocks WHERE product_id = ?")
            .bind(product_id).fetch_optional(&mut **tx).await.map_err(map_err)?;

        if let Some(r) = row {
            let qty: i32 = r.get("quantity");
            return Ok(qty);
        }

        let id = new_id();
        let ts = now();
        sqlx::query(
            "INSERT INTO stocks (id, product_id, quantity, min_quantity, location, updated_at) VALUES (?, ?, 0, 0, NULL, ?)"
        )
        .bind(&id).bind(product_id).bind(&ts)
        .execute(&mut **tx).await.map_err(map_err)?;
        Ok(0)
    }

    async fn create_movement(&self, input: &CreateStockMovementInput) -> Result<StockMovement, DomainError> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or_else(|| DomainError::Internal("Transação já finalizada".to_string()))?;

        let id = new_id();
        let ts = now();
        sqlx::query(
            "INSERT INTO stock_movements (id, product_id, movement_type, quantity, reason, reference, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&id).bind(&input.product_id).bind(&input.movement_type)
        .bind(input.quantity).bind(&input.reason).bind(&input.reference).bind(&ts)
        .execute(&mut **tx).await.map_err(map_err)?;

        Ok(StockMovement {
            id,
            product_id: input.product_id.clone(),
            movement_type: input.movement_type.clone(),
            quantity: input.quantity,
            reason: input.reason.clone(),
            reference: input.reference.clone(),
            created_at: ts,
        })
    }

    async fn list_movements(&self) -> Result<Vec<StockMovement>, DomainError> {
        Err(DomainError::Internal("Não implementado em contexto transacional".to_string()))
    }

    async fn list_low_stock(&self) -> Result<Vec<LowStockProduct>, DomainError> {
        Err(DomainError::Internal("Não implementado em contexto transacional".to_string()))
    }

    async fn atomic_increment(&self, product_id: &str, delta: i32) -> Result<i32, DomainError> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or_else(|| DomainError::Internal("Transação já finalizada".to_string()))?;
        let ts = now();

        // Garante que a linha existe
        let row = sqlx::query("SELECT quantity FROM stocks WHERE product_id = ?")
            .bind(product_id).fetch_optional(&mut **tx).await.map_err(map_err)?;
        if row.is_none() {
            let id = new_id();
            sqlx::query(
                "INSERT INTO stocks (id, product_id, quantity, min_quantity, location, updated_at) VALUES (?, ?, 0, 0, NULL, ?)"
            )
            .bind(&id).bind(product_id).bind(&ts)
            .execute(&mut **tx).await.map_err(map_err)?;
        }

        let updated = sqlx::query("UPDATE stocks SET quantity = quantity + ?, updated_at = ? WHERE product_id = ? RETURNING quantity")
            .bind(delta).bind(&ts).bind(product_id)
            .fetch_one(&mut **tx).await.map_err(map_err)?;
        let new_qty: i32 = updated.get("quantity");
        Ok(new_qty)
    }

    async fn atomic_decrement(&self, product_id: &str, delta: i32) -> Result<i32, DomainError> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or_else(|| DomainError::Internal("Transação já finalizada".to_string()))?;
        let ts = now();

        // Garante que a linha existe
        let row = sqlx::query("SELECT quantity FROM stocks WHERE product_id = ?")
            .bind(product_id).fetch_optional(&mut **tx).await.map_err(map_err)?;
        if row.is_none() {
            let id = new_id();
            sqlx::query(
                "INSERT INTO stocks (id, product_id, quantity, min_quantity, location, updated_at) VALUES (?, ?, 0, 0, NULL, ?)"
            )
            .bind(&id).bind(product_id).bind(&ts)
            .execute(&mut **tx).await.map_err(map_err)?;
        }

        let updated = sqlx::query(
            "UPDATE stocks SET quantity = quantity - ?, updated_at = ? WHERE product_id = ? AND quantity >= ? RETURNING quantity"
        )
        .bind(delta).bind(&ts).bind(product_id).bind(delta)
        .fetch_optional(&mut **tx).await.map_err(map_err)?;

        match updated {
            Some(r) => {
                let new_qty: i32 = r.get("quantity");
                Ok(new_qty)
            }
            None => {
                let current_row = sqlx::query("SELECT quantity FROM stocks WHERE product_id = ?")
                    .bind(product_id).fetch_one(&mut **tx).await.map_err(map_err)?;
                let current: i32 = current_row.get("quantity");
                Err(DomainError::BadRequest(format!(
                    "Estoque insuficiente. Disponível: {}, Solicitado: {}", current, delta
                )))
            }
        }
    }
}

// --- Sale ---

#[async_trait]
impl SaleRepository for TxSaleRepo {
    async fn create(&self, input: &CreateSaleInput, total_price: f64) -> Result<Sale, DomainError> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or_else(|| DomainError::Internal("Transação já finalizada".to_string()))?;

        let id = new_id();
        let ts = now();
        sqlx::query(
            "INSERT INTO sales (id, product_id, quantity, unit_price, total_price, sale_date, customer_name, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&id).bind(&input.product_id).bind(input.quantity).bind(input.unit_price)
        .bind(total_price).bind(&ts).bind(&input.customer_name).bind(&ts)
        .execute(&mut **tx).await.map_err(map_err)?;

        Ok(Sale {
            id,
            product_id: input.product_id.clone(),
            quantity: input.quantity,
            unit_price: input.unit_price,
            total_price,
            sale_date: ts.clone(),
            customer_name: input.customer_name.clone(),
            created_at: ts,
        })
    }
}

// --- Stubs para Category, Price, Warranty, Return, Report (não usados em transação) ---

// Implementações stub mínimas para satisfazer o trait

#[async_trait]
impl CategoryRepository for TxCategoryRepo {
    async fn list(&self) -> Result<Vec<Category>, DomainError> { Err(DomainError::Internal("Não implementado em contexto transacional".to_string())) }
    async fn find_by_id(&self, _: &str) -> Result<Option<Category>, DomainError> { Err(DomainError::Internal("Não implementado".to_string())) }
    async fn create(&self, _: &CreateCategoryInput) -> Result<Category, DomainError> { Err(DomainError::Internal("Não implementado".to_string())) }
    async fn update(&self, _: &str, _: &UpdateCategoryInput) -> Result<Category, DomainError> { Err(DomainError::Internal("Não implementado".to_string())) }
    async fn delete(&self, _: &str) -> Result<(), DomainError> { Err(DomainError::Internal("Não implementado".to_string())) }
    async fn exists(&self, _: &str) -> Result<bool, DomainError> { Err(DomainError::Internal("Não implementado".to_string())) }
    async fn link_product(&self, _: &str, _: &str) -> Result<(), DomainError> { Err(DomainError::Internal("Não implementado".to_string())) }
    async fn unlink_product(&self, _: &str, _: &str) -> Result<(), DomainError> { Err(DomainError::Internal("Não implementado".to_string())) }
}

#[async_trait]
impl PriceRepository for TxPriceRepo {
    async fn list_by_product(&self, _: &str) -> Result<Vec<Price>, DomainError> { Err(DomainError::Internal("Não implementado".to_string())) }
    async fn find_by_id(&self, _: &str) -> Result<Option<Price>, DomainError> { Err(DomainError::Internal("Não implementado".to_string())) }
    async fn create(&self, _: &str, _: &CreatePriceInput) -> Result<Price, DomainError> { Err(DomainError::Internal("Não implementado".to_string())) }
    async fn update(&self, _: &str, _: &UpdatePriceInput) -> Result<Price, DomainError> { Err(DomainError::Internal("Não implementado".to_string())) }
    async fn delete(&self, _: &str) -> Result<(), DomainError> { Err(DomainError::Internal("Não implementado".to_string())) }
}

#[async_trait]
impl WarrantyRepository for TxWarrantyRepo {
    async fn list(&self) -> Result<Vec<Warranty>, DomainError> { Err(DomainError::Internal("Não implementado".to_string())) }
    async fn find_by_id(&self, _: &str) -> Result<Option<Warranty>, DomainError> { Err(DomainError::Internal("Não implementado".to_string())) }
    async fn create(&self, _: &CreateWarrantyInput, _: &str) -> Result<Warranty, DomainError> { Err(DomainError::Internal("Não implementado".to_string())) }
    async fn update_status(&self, _: &str, _: &str) -> Result<Warranty, DomainError> { Err(DomainError::Internal("Não implementado".to_string())) }
}

#[async_trait]
impl ReturnRepository for TxReturnRepo {
    async fn list(&self) -> Result<Vec<Return>, DomainError> { Err(DomainError::Internal("Não implementado".to_string())) }
    async fn find_by_id(&self, _: &str) -> Result<Option<Return>, DomainError> { Err(DomainError::Internal("Não implementado".to_string())) }
    async fn create(&self, _: &CreateReturnInput) -> Result<Return, DomainError> { Err(DomainError::Internal("Não implementado".to_string())) }
    async fn update_status(&self, _: &str, _: &UpdateReturnStatusInput) -> Result<Return, DomainError> { Err(DomainError::Internal("Não implementado".to_string())) }
}

#[async_trait]
impl ReportRepository for TxReportRepo {
    async fn sales_report(&self, _: &ReportFilter) -> Result<Vec<SalesReportItem>, DomainError> { Err(DomainError::Internal("Não implementado".to_string())) }
    async fn stock_report(&self) -> Result<Vec<StockReportItem>, DomainError> { Err(DomainError::Internal("Não implementado".to_string())) }
    async fn returns_report(&self, _: &ReportFilter) -> Result<Vec<ReturnReportItem>, DomainError> { Err(DomainError::Internal("Não implementado".to_string())) }
}

// ============================================================
// UnitOfWork Factory & Implementation
// ============================================================

pub struct SqliteUnitOfWorkFactory {
    pool: Arc<SqlitePool>,
}

impl SqliteUnitOfWorkFactory {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}

pub struct SqliteUnitOfWork {
    tx: Arc<Mutex<Option<sqlx::Transaction<'static, Sqlite>>>>,
    product_repo: TxProductRepo,
    category_repo: TxCategoryRepo,
    price_repo: TxPriceRepo,
    stock_repo: TxStockRepo,
    warranty_repo: TxWarrantyRepo,
    return_repo: TxReturnRepo,
    sale_repo: TxSaleRepo,
    report_repo: TxReportRepo,
}

#[async_trait]
impl UnitOfWorkFactory for SqliteUnitOfWorkFactory {
    async fn begin(&self) -> Result<Box<dyn UnitOfWork>, DomainError> {
        let tx = self.pool.begin().await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let tx_arc = Arc::new(Mutex::new(Some(tx)));

        Ok(Box::new(SqliteUnitOfWork {
            tx: tx_arc.clone(),
            product_repo: TxProductRepo { tx: tx_arc.clone() },
            category_repo: TxCategoryRepo { tx: tx_arc.clone() },
            price_repo: TxPriceRepo { tx: tx_arc.clone() },
            stock_repo: TxStockRepo { tx: tx_arc.clone() },
            warranty_repo: TxWarrantyRepo { tx: tx_arc.clone() },
            return_repo: TxReturnRepo { tx: tx_arc.clone() },
            sale_repo: TxSaleRepo { tx: tx_arc.clone() },
            report_repo: TxReportRepo { tx: tx_arc },
        }))
    }
}

#[async_trait]
impl UnitOfWork for SqliteUnitOfWork {
    fn products(&mut self) -> &mut dyn ProductRepository { &mut self.product_repo }
    fn categories(&mut self) -> &mut dyn CategoryRepository { &mut self.category_repo }
    fn prices(&mut self) -> &mut dyn PriceRepository { &mut self.price_repo }
    fn stocks(&mut self) -> &mut dyn StockRepository { &mut self.stock_repo }
    fn warranties(&mut self) -> &mut dyn WarrantyRepository { &mut self.warranty_repo }
    fn returns(&mut self) -> &mut dyn ReturnRepository { &mut self.return_repo }
    fn sales(&mut self) -> &mut dyn SaleRepository { &mut self.sale_repo }
    fn reports(&mut self) -> &mut dyn ReportRepository { &mut self.report_repo }

    async fn commit(self: Box<Self>) -> Result<(), DomainError> {
        let tx_opt = self.tx.lock().await.take();
        if let Some(tx) = tx_opt {
            tx.commit().await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn rollback(self: Box<Self>) -> Result<(), DomainError> {
        let tx_opt = self.tx.lock().await.take();
        if let Some(tx) = tx_opt {
            tx.rollback().await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}
