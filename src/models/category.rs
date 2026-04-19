use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Categoria de produto (suporta subcategorias via parent_id).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Dados para criação de uma nova categoria.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateCategory {
    /// Nome da categoria.
    #[schema(example = "Colares")]
    pub name: String,
    /// Descrição da categoria.
    pub description: Option<String>,
    /// ID da categoria pai (para subcategorias).
    pub parent_id: Option<String>,
}

/// Dados para atualização de uma categoria.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<String>,
}
