// --- src/errors.rs ---

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("400 Bad Request: {0}")]
    BadRequest(String),
    #[error("401 Unauthorized: {0}")]
    Unauthorized(String),
    #[error("403 Forbidden: {0}")]
    Forbidden(String),
    #[error("404 Not Found: {0}")]
    NotFound(String),
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
