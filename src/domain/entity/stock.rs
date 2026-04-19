use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Estoque atual de um produto.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Stock {
    pub id: String,
    pub product_id: String,
    pub quantity: i32,
    pub min_quantity: i32,
    pub location: Option<String>,
    pub updated_at: String,
}

/// Movimentação de estoque (entrada ou saída).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StockMovement {
    pub id: String,
    pub product_id: String,
    pub movement_type: String,
    pub quantity: i32,
    pub reason: Option<String>,
    pub reference: Option<String>,
    pub created_at: String,
}

/// Dados para registrar uma movimentação de estoque.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStockMovementInput {
    pub product_id: String,
    pub movement_type: String,
    pub quantity: i32,
    pub reason: Option<String>,
    pub reference: Option<String>,
}

/// Dados para inicializar o estoque de um produto.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStockInput {
    pub product_id: String,
    pub quantity: Option<i32>,
    pub min_quantity: Option<i32>,
    pub location: Option<String>,
}

/// Produto com estoque baixo (para relatório).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LowStockProduct {
    pub product_id: String,
    pub product_name: String,
    pub sku: String,
    pub quantity: i32,
    pub min_quantity: i32,
}

/// Tipos válidos de movimentação.
pub const VALID_MOVEMENT_TYPES: &[&str] = &["in", "out"];
