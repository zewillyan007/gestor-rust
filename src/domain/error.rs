/// Erro genérico do domínio — sem dependência de framework.
#[derive(Debug)]
pub enum DomainError {
    /// Recurso não encontrado.
    NotFound(String),
    /// Erro de validação de dados.
    BadRequest(String),
    /// Erro interno (detalhes não vazam para o cliente).
    Internal(String),
    /// Conflito (ex: SKU duplicado, registro duplicado).
    Conflict(String),
}
