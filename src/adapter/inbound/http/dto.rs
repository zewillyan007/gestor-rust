use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa::IntoParams;

use crate::domain::entity::product::{CreateProductInput, UpdateProductInput};
use crate::domain::entity::category::{CreateCategoryInput, UpdateCategoryInput};
use crate::domain::entity::price::{CreatePriceInput, UpdatePriceInput};
use crate::domain::entity::stock::{CreateStockMovementInput, CreateStockInput};
use crate::domain::entity::warranty::CreateWarrantyInput;
use crate::domain::entity::return_model::{CreateReturnInput, UpdateReturnStatusInput};
use crate::domain::entity::sale::CreateSaleInput;
use crate::domain::entity::report::ReportFilter;

// ============================================================
// Produtos
// ============================================================

/// Dados para criação de um novo produto.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateProductDto {
    #[schema(example = "Colar de Pérolas")]
    pub name: String,
    pub description: Option<String>,
    #[schema(example = "COL-PER-001")]
    pub sku: String,
    pub brand: Option<String>,
}

impl CreateProductDto {
    pub fn into_input(self) -> CreateProductInput {
        CreateProductInput {
            name: self.name,
            description: self.description,
            sku: self.sku,
            brand: self.brand,
        }
    }
}

/// Dados para atualização de um produto.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateProductDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub brand: Option<String>,
}

impl UpdateProductDto {
    pub fn into_input(self) -> UpdateProductInput {
        UpdateProductInput {
            name: self.name,
            description: self.description,
            brand: self.brand,
        }
    }
}

/// Dados para alterar o status do produto.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateProductStatusDto {
    pub status: String,
}

// ============================================================
// Categorias
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateCategoryDto {
    #[schema(example = "Colares")]
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<String>,
}

impl CreateCategoryDto {
    pub fn into_input(self) -> CreateCategoryInput {
        CreateCategoryInput { name: self.name, description: self.description, parent_id: self.parent_id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateCategoryDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<String>,
}

impl UpdateCategoryDto {
    pub fn into_input(self) -> UpdateCategoryInput {
        UpdateCategoryInput { name: self.name, description: self.description, parent_id: self.parent_id }
    }
}

// ============================================================
// Preços
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreatePriceDto {
    #[schema(example = 25.0)]
    pub cost_price: f64,
    #[schema(example = 59.90)]
    pub sale_price: f64,
    #[schema(example = "2026-04-18")]
    pub effective_date: String,
}

impl CreatePriceDto {
    pub fn into_input(self) -> CreatePriceInput {
        CreatePriceInput { cost_price: self.cost_price, sale_price: self.sale_price, effective_date: self.effective_date }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdatePriceDto {
    pub cost_price: Option<f64>,
    pub sale_price: Option<f64>,
    pub effective_date: Option<String>,
}

impl UpdatePriceDto {
    pub fn into_input(self) -> UpdatePriceInput {
        UpdatePriceInput { cost_price: self.cost_price, sale_price: self.sale_price, effective_date: self.effective_date }
    }
}

// ============================================================
// Estoque
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateStockDto {
    pub product_id: String,
    pub quantity: Option<i32>,
    pub min_quantity: Option<i32>,
    pub location: Option<String>,
}

impl CreateStockDto {
    pub fn into_input(self) -> CreateStockInput {
        CreateStockInput { product_id: self.product_id, quantity: self.quantity, min_quantity: self.min_quantity, location: self.location }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateStockMovementDto {
    pub product_id: String,
    #[schema(example = "in")]
    pub movement_type: String,
    pub quantity: i32,
    #[schema(example = "Compra de fornecedor")]
    pub reason: Option<String>,
    pub reference: Option<String>,
}

impl CreateStockMovementDto {
    pub fn into_input(self) -> CreateStockMovementInput {
        CreateStockMovementInput { product_id: self.product_id, movement_type: self.movement_type, quantity: self.quantity, reason: self.reason, reference: self.reference }
    }
}

// ============================================================
// Garantias
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateWarrantyDto {
    pub product_id: String,
    #[schema(example = "Maria Silva")]
    pub customer_name: String,
    pub customer_contact: Option<String>,
    #[schema(example = "2026-04-18")]
    pub purchase_date: String,
    #[schema(example = 90)]
    pub warranty_days: i32,
    pub notes: Option<String>,
}

impl CreateWarrantyDto {
    pub fn into_input(self) -> CreateWarrantyInput {
        CreateWarrantyInput { product_id: self.product_id, customer_name: self.customer_name, customer_contact: self.customer_contact, purchase_date: self.purchase_date, warranty_days: self.warranty_days, notes: self.notes }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateWarrantyStatusDto {
    pub status: String,
}

// ============================================================
// Devoluções
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateReturnDto {
    pub product_id: String,
    pub warranty_id: Option<String>,
    #[schema(example = "Produto com defeito de fabricação")]
    pub reason: String,
    pub refund_amount: Option<f64>,
}

impl CreateReturnDto {
    pub fn into_input(self) -> CreateReturnInput {
        CreateReturnInput { product_id: self.product_id, warranty_id: self.warranty_id, reason: self.reason, refund_amount: self.refund_amount }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateReturnStatusDto {
    pub status: String,
    pub refund_amount: Option<f64>,
}

impl UpdateReturnStatusDto {
    pub fn into_input(self) -> UpdateReturnStatusInput {
        UpdateReturnStatusInput { status: self.status, refund_amount: self.refund_amount }
    }
}

// ============================================================
// Vendas
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateSaleDto {
    pub product_id: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub customer_name: Option<String>,
}

impl CreateSaleDto {
    pub fn into_input(self) -> CreateSaleInput {
        CreateSaleInput { product_id: self.product_id, quantity: self.quantity, unit_price: self.unit_price, customer_name: self.customer_name }
    }
}

// ============================================================
// Relatórios (filtro via query params)
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct ReportFilterDto {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

impl ReportFilterDto {
    pub fn into_filter(self) -> ReportFilter {
        ReportFilter { start_date: self.start_date, end_date: self.end_date }
    }
}
