use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

use crate::domain::error::DomainError;

/// Erro HTTP — converte DomainError em respostas Axum.
#[derive(Debug)]
pub struct HttpError(pub DomainError);

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl From<DomainError> for HttpError {
    fn from(err: DomainError) -> Self {
        HttpError(err)
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let (status, message) = match self.0 {
            DomainError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            DomainError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            DomainError::Internal(msg) => {
                tracing::error!("Erro interno: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Erro interno do servidor".to_string())
            }
            DomainError::Conflict(msg) => (StatusCode::CONFLICT, msg),
        };

        let body = ErrorResponse {
            error: status.canonical_reason().unwrap_or("Error").to_string(),
            message,
        };

        (status, axum::Json(body)).into_response()
    }
}
