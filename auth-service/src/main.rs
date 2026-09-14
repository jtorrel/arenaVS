use axum::{routing::{get, post}, Router, Json};
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        .route("/auth/login", post(login));  // ← nouvelle route

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Serveur démarré sur http://localhost:3000");
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
    message: String,
    email: String,
    password: String,
}

async fn login(
    Json(body): Json<LoginRequest>,
) -> Json<LoginResponse> {
    Json(LoginResponse {
        message: "ok, you're authenticated".to_string(),
        email: body.email,
        password: body.password,
    })
}