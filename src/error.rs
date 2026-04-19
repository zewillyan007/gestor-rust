use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

/// Tipo unificado de erro da aplicação.
#[derive(Debug)]
pub enum AppError {
    /// Recurso não encontrado.
    NotFound(String),
    /// Erro de validação de dados.
    BadRequest(String),
    /// Erro interno do servidor.
    Internal(String),
    /// Conflito (ex: SKU duplicado).
    Conflict(String),
}

/// Estrutura padrão de resposta de erro.
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal(msg) => {
                tracing::error!("Erro interno: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Erro interno do servidor".to_string())
            }
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
        };

        let body = ErrorResponse {
            error: status.canonical_reason().unwrap_or("Error").to_string(),
            message,
        };

        (status, axum::Json(body)).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Recurso não encontrado".to_string()),
            sqlx::Error::Database(ref db_err) => {
                if db_err.message().contains("UNIQUE constraint") {
                    AppError::Conflict("Registro duplicado".to_string())
                } else {
                    AppError::Internal(format!("Erro no banco de dados: {}", db_err.message()))
                }
            }
            _ => AppError::Internal(format!("Erro no banco de dados: {}", err)),
        }
    }
}
