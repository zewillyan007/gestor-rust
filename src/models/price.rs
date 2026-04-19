use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Preço de um produto (histórico de preços).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct Price {
    pub id: String,
    pub product_id: String,
    pub cost_price: f64,
    pub sale_price: f64,
    pub effective_date: String,
    pub created_at: String,
}

/// Dados para definir um novo preço de produto.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreatePrice {
    /// Preço de custo.
    #[schema(example = 25.0)]
    pub cost_price: f64,
    /// Preço de venda.
    #[schema(example = 59.90)]
    pub sale_price: f64,
    /// Data em que o preço entra em vigor (YYYY-MM-DD).
    #[schema(example = "2026-04-18")]
    pub effective_date: String,
}

/// Dados para atualizar um preço existente.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdatePrice {
    pub cost_price: Option<f64>,
    pub sale_price: Option<f64>,
    pub effective_date: Option<String>,
}
