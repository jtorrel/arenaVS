use axum::{Json, http::HeaderMap};
use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation};
use chrono::Utc;

use crate::models::{Claims, HealthResponse, LoginRequest, LoginResponse, IntrospectRequest, IntrospectResponse};

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
) -> Json<LoginResponse> {
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
    ).unwrap();

    Json(LoginResponse {
        token,
        email: body.email,
    })
}

pub async fn me(
    headers: HeaderMap,
) -> Json<serde_json::Value> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let token = match auth_header.strip_prefix("Bearer ") {
        Some(t) => t,
        None => return Json(serde_json::json!({ "error": "missing token" })),
    };

    let result = decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_secret().as_bytes()),
        &Validation::default(),
    );

    match result {
        Ok(data) => Json(serde_json::json!({
            "email": data.claims.sub,
        })),
        Err(_) => Json(serde_json::json!({
            "error": "invalid or expired token"
        })),
    }
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