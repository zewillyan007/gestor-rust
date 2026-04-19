use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Garantia de um produto vendido.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWarrantyInput {
    pub product_id: String,
    pub customer_name: String,
    pub customer_contact: Option<String>,
    pub purchase_date: String,
    pub warranty_days: i32,
    pub notes: Option<String>,
}

/// Status válidos de garantia.
pub const VALID_WARRANTY_STATUSES: &[&str] = &["active", "expired", "claimed"];
