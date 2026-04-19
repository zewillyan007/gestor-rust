use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Preço de um produto (histórico de preços).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Price {
    pub id: String,
    pub product_id: String,
    pub cost_price: f64,
    pub sale_price: f64,
    pub effective_date: String,
    pub created_at: String,
}

/// Dados para definir um novo preço de produto.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePriceInput {
    pub cost_price: f64,
    pub sale_price: f64,
    pub effective_date: String,
}

/// Dados para atualizar um preço existente.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePriceInput {
    pub cost_price: Option<f64>,
    pub sale_price: Option<f64>,
    pub effective_date: Option<String>,
}
