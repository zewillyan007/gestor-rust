use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Produto cadastrado na loja.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub sku: String,
    pub brand: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Dados para criação de um novo produto.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductInput {
    pub name: String,
    pub description: Option<String>,
    pub sku: String,
    pub brand: Option<String>,
}

/// Dados para atualização de um produto.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProductInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub brand: Option<String>,
}

/// Status válidos de produto.
pub const VALID_PRODUCT_STATUSES: &[&str] = &["available", "unavailable", "discontinued"];
