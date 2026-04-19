use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Status de uma devolução.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum ReturnStatus {
    Requested,
    Approved,
    Rejected,
    Completed,
}

/// Devolução de produto.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
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
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateReturn {
    /// ID do produto.
    pub product_id: String,
    /// ID da garantia vinculada (opcional).
    pub warranty_id: Option<String>,
    /// Motivo da devolução.
    #[schema(example = "Produto com defeito de fabricação")]
    pub reason: String,
    /// Valor do reembolso.
    pub refund_amount: Option<f64>,
}

/// Dados para atualizar o status de uma devolução.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateReturnStatus {
    /// Novo status: requested, approved, rejected ou completed.
    pub status: String,
    /// Valor do reembolso (preenchido na aprovação).
    pub refund_amount: Option<f64>,
}
