use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

// Nos types d'erreurs possibles
pub enum AppError {
    InvalidToken,
    MissingToken,
    Internal,
}

// On dit à Axum comment transformer une AppError en réponse HTTP
impl IntoResponse for AppError {
    /// Convertit une erreur applicative en réponse HTTP JSON.
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid or expired token"),
            AppError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing token"),
            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
