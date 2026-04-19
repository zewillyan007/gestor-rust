use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Status de disponibilidade do produto.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "lowercase")]
pub enum ProductStatus {
    /// Disponível para venda.
    Available,
    /// Indisponível temporariamente.
    Unavailable,
    /// Descontinuado (não será mais vendido).
    Discontinued,
}

impl std::fmt::Display for ProductStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductStatus::Available => write!(f, "available"),
            ProductStatus::Unavailable => write!(f, "unavailable"),
            ProductStatus::Discontinued => write!(f, "discontinued"),
        }
    }
}

/// Produto cadastrado na loja.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
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
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateProduct {
    /// Nome do produto.
    #[schema(example = "Colar de Pérolas")]
    pub name: String,
    /// Descrição detalhada.
    pub description: Option<String>,
    /// Código SKU único.
    #[schema(example = "COL-PER-001")]
    pub sku: String,
    /// Marca/fabricante.
    pub brand: Option<String>,
}

/// Dados para atualização de um produto.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateProduct {
    pub name: Option<String>,
    pub description: Option<String>,
    pub brand: Option<String>,
}

/// Dados para alterar o status de disponibilidade do produto.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateProductStatus {
    /// Novo status: available, unavailable ou discontinued.
    pub status: String,
}

/// Helper para gerar IDs.
pub fn new_id() -> String {
    Uuid::new_v4().to_string()
}
