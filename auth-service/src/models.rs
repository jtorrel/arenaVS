// --- src/models.rs ---

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Deserialize)]
pub struct IntrospectRequest {
    pub token: String,
}

#[derive(Serialize)]
pub struct IntrospectResponse {
    pub active: bool,
    pub email: Option<String>, // Option car si token invalide, pas d'email
}
