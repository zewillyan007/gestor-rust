//! Mocks para testes unitários dos Use Cases.
//!
//! Cada mock armazena estado interno em `Mutex` para permitir
//! configuração e verificação nos testes.

use std::sync::Mutex;

use async_trait::async_trait;

use crate::domain::entity::product::{Product, CreateProductInput, UpdateProductInput};
use crate::domain::entity::category::{Category, CreateCategoryInput, UpdateCategoryInput};
use crate::domain::entity::price::{Price, CreatePriceInput, UpdatePriceInput};
use crate::domain::entity::stock::{
    Stock, StockMovement, CreateStockMovementInput, CreateStockInput, LowStockProduct,
};
use crate::domain::entity::warranty::{Warranty, CreateWarrantyInput};
use crate::domain::entity::return_model::{Return, CreateReturnInput, UpdateReturnStatusInput};
use crate::domain::entity::sale::{Sale, CreateSaleInput};
use crate::domain::entity::report::{
    SalesReportItem, StockReportItem, ReturnReportItem, ReportFilter,
};
use crate::domain::error::DomainError;
use super::product_repository::ProductRepository;
use super::category_repository::CategoryRepository;
use super::price_repository::PriceRepository;
use super::stock_repository::StockRepository;
use super::warranty_repository::WarrantyRepository;
use super::return_repository::ReturnRepository;
use super::sale_repository::SaleRepository;
use super::report_repository::ReportRepository;
use super::{UnitOfWorkFactory, UnitOfWork};

// ============================================================
// Mock Product Repository
// ============================================================

pub struct MockProductRepository {
    pub products: Mutex<Vec<Product>>,
    pub should_error: Mutex<bool>,
}

impl MockProductRepository {
    pub fn new() -> Self {
        Self {
            products: Mutex::new(Vec::new()),
            should_error: Mutex::new(false),
        }
    }

    pub fn with_products(&self, products: Vec<Product>) {
        *self.products.lock().unwrap() = products;
    }

    pub fn set_should_error(&self, val: bool) {
        *self.should_error.lock().unwrap() = val;
    }

    fn check_error(&self) -> Result<(), DomainError> {
        if *self.should_error.lock().unwrap() {
            Err(DomainError::Internal("Erro simulado".to_string()))
        } else {
            Ok(())
        }
    }
}

#[async_trait]
impl ProductRepository for MockProductRepository {
    async fn list(&self) -> Result<Vec<Product>, DomainError> {
        self.check_error()?;
        Ok(self.products.lock().unwrap().clone())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Product>, DomainError> {
        self.check_error()?;
        Ok(self.products.lock().unwrap().iter().find(|p| p.id == id).cloned())
    }

    async fn find_by_sku(&self, sku: &str) -> Result<Option<Product>, DomainError> {
        self.check_error()?;
        Ok(self.products.lock().unwrap().iter().find(|p| p.sku == sku).cloned())
    }

    async fn create(&self, input: &CreateProductInput) -> Result<Product, DomainError> {
        self.check_error()?;
        let product = Product {
            id: "mock-id".to_string(),
            name: input.name.clone(),
            description: input.description.clone(),
            sku: input.sku.clone(),
            brand: input.brand.clone(),
            status: "available".to_string(),
            created_at: "2026-01-01 00:00:00".to_string(),
            updated_at: "2026-01-01 00:00:00".to_string(),
        };
        self.products.lock().unwrap().push(product.clone());
        Ok(product)
    }

    async fn update(&self, id: &str, input: &UpdateProductInput) -> Result<Product, DomainError> {
        self.check_error()?;
        let mut products = self.products.lock().unwrap();
        let product = products.iter_mut().find(|p| p.id == id).ok_or_else(|| {
            DomainError::NotFound(format!("Produto com id '{}' não encontrado", id))
        })?;
        if let Some(ref name) = input.name { product.name = name.clone(); }
        if let Some(ref desc) = input.description { product.description = Some(desc.clone()); }
        if let Some(ref brand) = input.brand { product.brand = Some(brand.clone()); }
        Ok(product.clone())
    }

    async fn update_status(&self, id: &str, status: &str) -> Result<Product, DomainError> {
        self.check_error()?;
        let mut products = self.products.lock().unwrap();
        let product = products.iter_mut().find(|p| p.id == id).ok_or_else(|| {
            DomainError::NotFound(format!("Produto com id '{}' não encontrado", id))
        })?;
        product.status = status.to_string();
        Ok(product.clone())
    }

    async fn delete(&self, id: &str) -> Result<(), DomainError> {
        self.check_error()?;
        let mut products = self.products.lock().unwrap();
        let len_before = products.len();
        products.retain(|p| p.id != id);
        if products.len() == len_before {
            return Err(DomainError::NotFound(format!("Produto com id '{}' não encontrado", id)));
        }
        Ok(())
    }

    async fn exists(&self, id: &str) -> Result<bool, DomainError> {
        self.check_error()?;
        Ok(self.products.lock().unwrap().iter().any(|p| p.id == id))
    }
}

// ============================================================
// Mock Category Repository
// ============================================================

pub struct MockCategoryRepository {
    pub categories: Mutex<Vec<Category>>,
    pub links: Mutex<Vec<(String, String)>>, // (product_id, category_id)
}

impl MockCategoryRepository {
    pub fn new() -> Self {
        Self {
            categories: Mutex::new(Vec::new()),
            links: Mutex::new(Vec::new()),
        }
    }

    pub fn with_categories(&self, categories: Vec<Category>) {
        *self.categories.lock().unwrap() = categories;
    }
}

#[async_trait]
impl CategoryRepository for MockCategoryRepository {
    async fn list(&self) -> Result<Vec<Category>, DomainError> {
        Ok(self.categories.lock().unwrap().clone())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Category>, DomainError> {
        Ok(self.categories.lock().unwrap().iter().find(|c| c.id == id).cloned())
    }

    async fn create(&self, input: &CreateCategoryInput) -> Result<Category, DomainError> {
        let cat = Category {
            id: "mock-cat-id".to_string(),
            name: input.name.clone(),
            description: input.description.clone(),
            parent_id: input.parent_id.clone(),
            created_at: "2026-01-01 00:00:00".to_string(),
            updated_at: "2026-01-01 00:00:00".to_string(),
        };
        self.categories.lock().unwrap().push(cat.clone());
        Ok(cat)
    }

    async fn update(&self, id: &str, input: &UpdateCategoryInput) -> Result<Category, DomainError> {
        let mut cats = self.categories.lock().unwrap();
        let cat = cats.iter_mut().find(|c| c.id == id).ok_or_else(|| {
            DomainError::NotFound(format!("Categoria com id '{}' não encontrada", id))
        })?;
        if let Some(ref name) = input.name { cat.name = name.clone(); }
        if let Some(ref desc) = input.description { cat.description = Some(desc.clone()); }
        if let Some(ref pid) = input.parent_id { cat.parent_id = Some(pid.clone()); }
        Ok(cat.clone())
    }

    async fn delete(&self, id: &str) -> Result<(), DomainError> {
        let mut cats = self.categories.lock().unwrap();
        let len_before = cats.len();
        cats.retain(|c| c.id != id);
        if cats.len() == len_before {
            return Err(DomainError::NotFound(format!("Categoria com id '{}' não encontrada", id)));
        }
        Ok(())
    }

    async fn exists(&self, id: &str) -> Result<bool, DomainError> {
        Ok(self.categories.lock().unwrap().iter().any(|c| c.id == id))
    }

    async fn link_product(&self, product_id: &str, category_id: &str) -> Result<(), DomainError> {
        self.links.lock().unwrap().push((product_id.to_string(), category_id.to_string()));
        Ok(())
    }

    async fn unlink_product(&self, product_id: &str, category_id: &str) -> Result<(), DomainError> {
        self.links.lock().unwrap().retain(|(p, c)| !(p == product_id && c == category_id));
        Ok(())
    }
}

// ============================================================
// Mock Price Repository
// ============================================================

pub struct MockPriceRepository {
    pub prices: Mutex<Vec<Price>>,
}

impl MockPriceRepository {
    pub fn new() -> Self {
        Self { prices: Mutex::new(Vec::new()) }
    }
}

#[async_trait]
impl PriceRepository for MockPriceRepository {
    async fn list_by_product(&self, product_id: &str) -> Result<Vec<Price>, DomainError> {
        Ok(self.prices.lock().unwrap().iter().filter(|p| p.product_id == product_id).cloned().collect())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Price>, DomainError> {
        Ok(self.prices.lock().unwrap().iter().find(|p| p.id == id).cloned())
    }

    async fn create(&self, product_id: &str, input: &CreatePriceInput) -> Result<Price, DomainError> {
        let price = Price {
            id: "mock-price-id".to_string(),
            product_id: product_id.to_string(),
            cost_price: input.cost_price,
            sale_price: input.sale_price,
            effective_date: input.effective_date.clone(),
            created_at: "2026-01-01 00:00:00".to_string(),
        };
        self.prices.lock().unwrap().push(price.clone());
        Ok(price)
    }

    async fn update(&self, id: &str, input: &UpdatePriceInput) -> Result<Price, DomainError> {
        let mut prices = self.prices.lock().unwrap();
        let price = prices.iter_mut().find(|p| p.id == id).ok_or_else(|| {
            DomainError::NotFound(format!("Preço com id '{}' não encontrado", id))
        })?;
        if let Some(cp) = input.cost_price { price.cost_price = cp; }
        if let Some(sp) = input.sale_price { price.sale_price = sp; }
        if let Some(ref ed) = input.effective_date { price.effective_date = ed.clone(); }
        Ok(price.clone())
    }

    async fn delete(&self, id: &str) -> Result<(), DomainError> {
        let mut prices = self.prices.lock().unwrap();
        let len_before = prices.len();
        prices.retain(|p| p.id != id);
        if prices.len() == len_before {
            return Err(DomainError::NotFound(format!("Preço com id '{}' não encontrado", id)));
        }
        Ok(())
    }
}

// ============================================================
// Mock Stock Repository
// ============================================================

pub struct MockStockRepository {
    pub stocks: Mutex<Vec<Stock>>,
    pub movements: Mutex<Vec<StockMovement>>,
}

impl MockStockRepository {
    pub fn new() -> Self {
        Self {
            stocks: Mutex::new(Vec::new()),
            movements: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl StockRepository for MockStockRepository {
    async fn find_by_product(&self, product_id: &str) -> Result<Option<Stock>, DomainError> {
        Ok(self.stocks.lock().unwrap().iter().find(|s| s.product_id == product_id).cloned())
    }

    async fn create(&self, input: &CreateStockInput) -> Result<Stock, DomainError> {
        let stock = Stock {
            id: "mock-stock-id".to_string(),
            product_id: input.product_id.clone(),
            quantity: input.quantity.unwrap_or(0),
            min_quantity: input.min_quantity.unwrap_or(0),
            location: input.location.clone(),
            updated_at: "2026-01-01 00:00:00".to_string(),
        };
        self.stocks.lock().unwrap().push(stock.clone());
        Ok(stock)
    }

    async fn update_quantity(&self, product_id: &str, quantity: i32) -> Result<(), DomainError> {
        let mut stocks = self.stocks.lock().unwrap();
        if let Some(s) = stocks.iter_mut().find(|s| s.product_id == product_id) {
            s.quantity = quantity;
            Ok(())
        } else {
            Err(DomainError::NotFound("Estoque não encontrado".to_string()))
        }
    }

    async fn create_or_get(&self, product_id: &str) -> Result<i32, DomainError> {
        let stocks = self.stocks.lock().unwrap();
        if let Some(s) = stocks.iter().find(|s| s.product_id == product_id) {
            return Ok(s.quantity);
        }
        drop(stocks);
        let stock = Stock {
            id: "mock-stock-id".to_string(),
            product_id: product_id.to_string(),
            quantity: 0,
            min_quantity: 0,
            location: None,
            updated_at: "2026-01-01 00:00:00".to_string(),
        };
        let qty = stock.quantity;
        self.stocks.lock().unwrap().push(stock);
        Ok(qty)
    }

    async fn create_movement(&self, input: &CreateStockMovementInput) -> Result<StockMovement, DomainError> {
        let movement = StockMovement {
            id: "mock-movement-id".to_string(),
            product_id: input.product_id.clone(),
            movement_type: input.movement_type.clone(),
            quantity: input.quantity,
            reason: input.reason.clone(),
            reference: input.reference.clone(),
            created_at: "2026-01-01 00:00:00".to_string(),
        };
        self.movements.lock().unwrap().push(movement.clone());
        Ok(movement)
    }

    async fn list_movements(&self) -> Result<Vec<StockMovement>, DomainError> {
        Ok(self.movements.lock().unwrap().clone())
    }

    async fn list_low_stock(&self) -> Result<Vec<LowStockProduct>, DomainError> {
        Ok(Vec::new())
    }

    async fn atomic_increment(&self, product_id: &str, delta: i32) -> Result<i32, DomainError> {
        // Garante que a linha existe
        {
            let stocks = self.stocks.lock().unwrap();
            if !stocks.iter().any(|s| s.product_id == product_id) {
                drop(stocks);
                self.stocks.lock().unwrap().push(Stock {
                    id: "mock-stock-id".to_string(),
                    product_id: product_id.to_string(),
                    quantity: 0,
                    min_quantity: 0,
                    location: None,
                    updated_at: "2026-01-01 00:00:00".to_string(),
                });
            }
        }
        let mut stocks = self.stocks.lock().unwrap();
        let s = stocks.iter_mut().find(|s| s.product_id == product_id).unwrap();
        s.quantity += delta;
        Ok(s.quantity)
    }

    async fn atomic_decrement(&self, product_id: &str, delta: i32) -> Result<i32, DomainError> {
        {
            let stocks = self.stocks.lock().unwrap();
            if !stocks.iter().any(|s| s.product_id == product_id) {
                drop(stocks);
                self.stocks.lock().unwrap().push(Stock {
                    id: "mock-stock-id".to_string(),
                    product_id: product_id.to_string(),
                    quantity: 0,
                    min_quantity: 0,
                    location: None,
                    updated_at: "2026-01-01 00:00:00".to_string(),
                });
            }
        }
        let mut stocks = self.stocks.lock().unwrap();
        let s = stocks.iter_mut().find(|s| s.product_id == product_id).unwrap();
        if s.quantity < delta {
            return Err(DomainError::BadRequest(format!(
                "Estoque insuficiente. Disponível: {}, Solicitado: {}", s.quantity, delta
            )));
        }
        s.quantity -= delta;
        Ok(s.quantity)
    }
}

// ============================================================
// Mock Warranty Repository
// ============================================================

pub struct MockWarrantyRepository {
    pub warranties: Mutex<Vec<Warranty>>,
}

impl MockWarrantyRepository {
    pub fn new() -> Self {
        Self { warranties: Mutex::new(Vec::new()) }
    }
}

#[async_trait]
impl WarrantyRepository for MockWarrantyRepository {
    async fn list(&self) -> Result<Vec<Warranty>, DomainError> {
        Ok(self.warranties.lock().unwrap().clone())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Warranty>, DomainError> {
        Ok(self.warranties.lock().unwrap().iter().find(|w| w.id == id).cloned())
    }

    async fn create(&self, input: &CreateWarrantyInput, expires_at: &str) -> Result<Warranty, DomainError> {
        let warranty = Warranty {
            id: "mock-warranty-id".to_string(),
            product_id: input.product_id.clone(),
            customer_name: input.customer_name.clone(),
            customer_contact: input.customer_contact.clone(),
            purchase_date: input.purchase_date.clone(),
            warranty_days: input.warranty_days,
            expires_at: expires_at.to_string(),
            status: "active".to_string(),
            notes: input.notes.clone(),
            created_at: "2026-01-01 00:00:00".to_string(),
            updated_at: "2026-01-01 00:00:00".to_string(),
        };
        self.warranties.lock().unwrap().push(warranty.clone());
        Ok(warranty)
    }

    async fn update_status(&self, id: &str, status: &str) -> Result<Warranty, DomainError> {
        let mut ws = self.warranties.lock().unwrap();
        let w = ws.iter_mut().find(|w| w.id == id).ok_or_else(|| {
            DomainError::NotFound(format!("Garantia com id '{}' não encontrada", id))
        })?;
        w.status = status.to_string();
        Ok(w.clone())
    }
}

// ============================================================
// Mock Return Repository
// ============================================================

pub struct MockReturnRepository {
    pub returns: Mutex<Vec<Return>>,
}

impl MockReturnRepository {
    pub fn new() -> Self {
        Self { returns: Mutex::new(Vec::new()) }
    }
}

#[async_trait]
impl ReturnRepository for MockReturnRepository {
    async fn list(&self) -> Result<Vec<Return>, DomainError> {
        Ok(self.returns.lock().unwrap().clone())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Return>, DomainError> {
        Ok(self.returns.lock().unwrap().iter().find(|r| r.id == id).cloned())
    }

    async fn create(&self, input: &CreateReturnInput) -> Result<Return, DomainError> {
        let ret = Return {
            id: "mock-return-id".to_string(),
            product_id: input.product_id.clone(),
            warranty_id: input.warranty_id.clone(),
            reason: input.reason.clone(),
            status: "requested".to_string(),
            refund_amount: input.refund_amount,
            created_at: "2026-01-01 00:00:00".to_string(),
            updated_at: "2026-01-01 00:00:00".to_string(),
        };
        self.returns.lock().unwrap().push(ret.clone());
        Ok(ret)
    }

    async fn update_status(&self, id: &str, input: &UpdateReturnStatusInput) -> Result<Return, DomainError> {
        let mut rets = self.returns.lock().unwrap();
        let ret = rets.iter_mut().find(|r| r.id == id).ok_or_else(|| {
            DomainError::NotFound(format!("Devolução com id '{}' não encontrada", id))
        })?;
        ret.status = input.status.clone();
        if let Some(amt) = input.refund_amount { ret.refund_amount = Some(amt); }
        Ok(ret.clone())
    }
}

// ============================================================
// Mock Sale Repository
// ============================================================

pub struct MockSaleRepository {
    pub sales: Mutex<Vec<Sale>>,
}

impl MockSaleRepository {
    pub fn new() -> Self {
        Self { sales: Mutex::new(Vec::new()) }
    }
}

#[async_trait]
impl SaleRepository for MockSaleRepository {
    async fn create(&self, input: &CreateSaleInput, total_price: f64) -> Result<Sale, DomainError> {
        let sale = Sale {
            id: "mock-sale-id".to_string(),
            product_id: input.product_id.clone(),
            quantity: input.quantity,
            unit_price: input.unit_price,
            total_price,
            sale_date: "2026-01-01 00:00:00".to_string(),
            customer_name: input.customer_name.clone(),
            created_at: "2026-01-01 00:00:00".to_string(),
        };
        self.sales.lock().unwrap().push(sale.clone());
        Ok(sale)
    }
}

// ============================================================
// Mock Report Repository
// ============================================================

pub struct MockReportRepository {
    pub sales_items: Mutex<Vec<SalesReportItem>>,
    pub stock_items: Mutex<Vec<StockReportItem>>,
    pub return_items: Mutex<Vec<ReturnReportItem>>,
}

impl MockReportRepository {
    pub fn new() -> Self {
        Self {
            sales_items: Mutex::new(Vec::new()),
            stock_items: Mutex::new(Vec::new()),
            return_items: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl ReportRepository for MockReportRepository {
    async fn sales_report(&self, _filter: &ReportFilter) -> Result<Vec<SalesReportItem>, DomainError> {
        Ok(self.sales_items.lock().unwrap().clone())
    }

    async fn stock_report(&self) -> Result<Vec<StockReportItem>, DomainError> {
        Ok(self.stock_items.lock().unwrap().clone())
    }

    async fn returns_report(&self, _filter: &ReportFilter) -> Result<Vec<ReturnReportItem>, DomainError> {
        Ok(self.return_items.lock().unwrap().clone())
    }
}

// ============================================================
// Mock UnitOfWork (para SaleUseCase e StockUseCase)
// ============================================================

pub struct MockUnitOfWork {
    pub product_repo: MockProductRepository,
    pub stock_repo: MockStockRepository,
    pub sale_repo: MockSaleRepository,
    pub committed: Mutex<bool>,
    pub rolled_back: Mutex<bool>,
}

impl MockUnitOfWork {
    pub fn new(
        product_repo: MockProductRepository,
        stock_repo: MockStockRepository,
        sale_repo: MockSaleRepository,
    ) -> Self {
        Self {
            product_repo,
            stock_repo,
            sale_repo,
            committed: Mutex::new(false),
            rolled_back: Mutex::new(false),
        }
    }
}

#[async_trait]
impl UnitOfWork for MockUnitOfWork {
    fn products(&mut self) -> &mut dyn ProductRepository { &mut self.product_repo }
    fn categories(&mut self) -> &mut dyn CategoryRepository {
        unimplemented!("MockUnitOfWork não implementa categories")
    }
    fn prices(&mut self) -> &mut dyn PriceRepository {
        unimplemented!("MockUnitOfWork não implementa prices")
    }
    fn stocks(&mut self) -> &mut dyn StockRepository { &mut self.stock_repo }
    fn warranties(&mut self) -> &mut dyn WarrantyRepository {
        unimplemented!("MockUnitOfWork não implementa warranties")
    }
    fn returns(&mut self) -> &mut dyn ReturnRepository {
        unimplemented!("MockUnitOfWork não implementa returns")
    }
    fn sales(&mut self) -> &mut dyn SaleRepository { &mut self.sale_repo }
    fn reports(&mut self) -> &mut dyn ReportRepository {
        unimplemented!("MockUnitOfWork não implementa reports")
    }

    async fn commit(self: Box<Self>) -> Result<(), DomainError> {
        *self.committed.lock().unwrap() = true;
        Ok(())
    }

    async fn rollback(self: Box<Self>) -> Result<(), DomainError> {
        *self.rolled_back.lock().unwrap() = true;
        Ok(())
    }
}

pub struct MockUnitOfWorkFactory {
    pub product_repo: MockProductRepository,
    pub stock_repo: MockStockRepository,
    pub sale_repo: MockSaleRepository,
}

impl MockUnitOfWorkFactory {
    pub fn new(
        product_repo: MockProductRepository,
        stock_repo: MockStockRepository,
        sale_repo: MockSaleRepository,
    ) -> Self {
        Self { product_repo, stock_repo, sale_repo }
    }
}

#[async_trait]
impl UnitOfWorkFactory for MockUnitOfWorkFactory {
    async fn begin(&self) -> Result<Box<dyn UnitOfWork>, DomainError> {
        Ok(Box::new(MockUnitOfWork::new(
            // Cloneamos o estado dos mocks para a transação
            MockProductRepository { products: Mutex::new(self.product_repo.products.lock().unwrap().clone()), should_error: Mutex::new(false) },
            MockStockRepository { stocks: Mutex::new(self.stock_repo.stocks.lock().unwrap().clone()), movements: Mutex::new(Vec::new()) },
            MockSaleRepository { sales: Mutex::new(Vec::new()) },
        )))
    }
}
