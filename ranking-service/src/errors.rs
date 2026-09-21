// --- Rankking Service ---
// --- src/errors.rs ---

use thiserror::Error;

#[derive(Error, Debug)]
/// Erreurs applicatives retournées par le service.
pub enum AppError {
    /// Requête invalide.
    #[error("400 Bad Request: {0}")]
    BadRequest(String),
    /// Authentification manquante ou invalide.
    #[error("401 Unauthorized: {0}")]
    Unauthorized(String),
    /// Utilisateur authentifié mais non autorisé.
    #[error("403 Forbidden: {0}")]
    Forbidden(String),
    /// Ressource demandée introuvable.
    #[error("404 Not Found: {0}")]
    NotFound(String),
    /// Erreur rencontrée lors d'une opération en base de données.
    #[error("500 Internal Server Error")]
    Database(#[from] sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg).into_response(),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg).into_response(),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg).into_response(),
            AppError::InternalServerError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
            AppError::Database(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
}
