use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Registro de venda.
// TODO: Considerar migrar `unit_price` e `total_price` de f64 para i64 (centavos)
// para eliminar imprecisão de ponto flutuante em cálculos financeiros.
// Aceitável por enquanto dado o escopo do projeto (loja pequena, sem cálculos complexos).
// Se houver integração com gateways de pagamento ou contabilidade, priorizar esta migração.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSaleInput {
    pub product_id: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub customer_name: Option<String>,
}
