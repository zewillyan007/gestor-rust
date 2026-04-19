use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Devolução de produto.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Return {
    pub id: String,
    pub product_id: String,
    pub warranty_id: Option<String>,
    pub reason: String,
    pub status: String,
    pub refund_amount: Option<f64>,
    pub created_at: String,
    pub updated_at: String,
}

/// Dados para abrir uma nova devolução.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReturnInput {
    pub product_id: String,
    pub warranty_id: Option<String>,
    pub reason: String,
    pub refund_amount: Option<f64>,
}

/// Dados para atualizar o status de uma devolução.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateReturnStatusInput {
    pub status: String,
    pub refund_amount: Option<f64>,
}

/// Status válidos de devolução.
pub const VALID_RETURN_STATUSES: &[&str] = &["requested", "approved", "rejected", "completed"];
