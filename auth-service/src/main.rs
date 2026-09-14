use axum::{
    routing::{get, post},
    Router, Json,
    http::HeaderMap,
};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation};
use chrono::Utc;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        .route("/auth/login", post(login))
        .route("/auth/me", get(me));  // ← nouveau

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server started on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Serialize)]
struct MeResponse {
    email: String,
}

fn get_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret".to_string())
}

async fn login(
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

async fn me(
    headers: HeaderMap,  // ← Axum nous donne accès à tous les headers
) -> Json<serde_json::Value> {
    // 1. Lire le header Authorization
    let auth_header = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    // 2. Extraire le token (enlever "Bearer ")
    let token = match auth_header.strip_prefix("Bearer ") {
        Some(t) => t,
        None => return Json(serde_json::json!({ "error": "missing token" })),
    };

    // 3. Valider le token
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