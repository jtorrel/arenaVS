use axum::{Json, http::HeaderMap};
use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation};
use chrono::Utc;

use crate::models::{Claims, HealthResponse, LoginRequest, LoginResponse, IntrospectRequest, IntrospectResponse};
use crate::errors::AppError;

pub fn get_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret".to_string())
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

pub async fn login(
    Json(body): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {  // ← Result au lieu de Json direct
    let exp = Utc::now()
        .checked_add_signed(chrono::Duration::hours(1))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: body.email.clone(),
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_secret().as_bytes()),
    ).map_err(|_| AppError::Internal)?;  // ← ? au lieu de unwrap()

    Ok(Json(LoginResponse {
        token,
        email: body.email,
    }))
}

pub async fn me(
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, AppError> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(AppError::MissingToken)?;  // ← ? au lieu de unwrap_or("")

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::MissingToken)?;

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_secret().as_bytes()),
        &Validation::default(),
    ).map_err(|_| AppError::InvalidToken)?;

    Ok(Json(serde_json::json!({
        "email": data.claims.sub,
    })))
}

pub async fn introspect(
    Json(body): Json<IntrospectRequest>,
) -> Json<IntrospectResponse> {
    let result = decode::<Claims>(
        &body.token,
        &DecodingKey::from_secret(get_secret().as_bytes()),
        &Validation::default(),
    );

    match result {
        Ok(data) => Json(IntrospectResponse {
            active: true,
            email: Some(data.claims.sub),
        }),
        Err(_) => Json(IntrospectResponse {
            active: false,
            email: None,
        }),
    }
}