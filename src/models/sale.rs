use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Registro de venda.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct Sale {
    pub id: String,
    pub product_id: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub total_price: f64,
    pub sale_date: String,
    pub customer_name: Option<String>,
    pub created_at: String,
}

/// Dados para registrar uma nova venda.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateSale {
    /// ID do produto vendido.
    pub product_id: String,
    /// Quantidade vendida.
    pub quantity: i32,
    /// Preço unitário praticado.
    pub unit_price: f64,
    /// Nome do cliente (opcional).
    pub customer_name: Option<String>,
}
