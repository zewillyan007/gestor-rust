use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Categoria de produto (suporta subcategorias via parent_id).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Dados para criação de uma nova categoria.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCategoryInput {
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<String>,
}

/// Dados para atualização de uma categoria.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCategoryInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<String>,
}
