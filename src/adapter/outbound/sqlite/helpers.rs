use crate::domain::error::DomainError;

/// Mapeia erros SQLx para DomainError.
pub fn map_err(e: sqlx::Error) -> DomainError {
    match e {
        sqlx::Error::RowNotFound => DomainError::NotFound("Recurso não encontrado".to_string()),
        sqlx::Error::Database(ref db_err) => {
            if db_err.message().contains("UNIQUE constraint") {
                DomainError::Conflict("Registro duplicado".to_string())
            } else {
                DomainError::Internal(format!("Erro no banco de dados: {}", db_err.message()))
            }
        }
        _ => DomainError::Internal(format!("Erro no banco de dados: {}", e)),
    }
}

/// Retorna o timestamp UTC atual no formato "YYYY-MM-DD HH:MM:SS".
pub fn now() -> String {
    chrono::Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string()
}
