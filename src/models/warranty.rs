use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Status da garantia.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum WarrantyStatus {
    Active,
    Expired,
    Claimed,
}

/// Garantia de um produto vendido.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct Warranty {
    pub id: String,
    pub product_id: String,
    pub customer_name: String,
    pub customer_contact: Option<String>,
    pub purchase_date: String,
    pub warranty_days: i32,
    pub expires_at: String,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Dados para registrar uma nova garantia.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateWarranty {
    /// ID do produto.
    pub product_id: String,
    /// Nome do cliente.
    #[schema(example = "Maria Silva")]
    pub customer_name: String,
    /// Contato do cliente (telefone ou email).
    pub customer_contact: Option<String>,
    /// Data da compra (YYYY-MM-DD).
    #[schema(example = "2026-04-18")]
    pub purchase_date: String,
    /// Duração da garantia em dias.
    #[schema(example = 90)]
    pub warranty_days: i32,
    /// Observações adicionais.
    pub notes: Option<String>,
}

/// Dados para atualizar o status de uma garantia.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateWarrantyStatus {
    /// Novo status: active, expired ou claimed.
    pub status: String,
}
