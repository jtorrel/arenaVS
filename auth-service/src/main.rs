use axum::{routing::{get, post}, Router, Json};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, EncodingKey, Header};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        .route("/auth/login", post(login));

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

// Les données embarquées dans le JWT
#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,   // subject = qui est l'utilisateur (son email ici)
    exp: usize,    // expiration (timestamp unix)
}

async fn login(
    Json(body): Json<LoginRequest>,
) -> Json<LoginResponse> {
    let claims = Claims {
        sub: body.email.clone(),
        exp: 9999999999,  // timestamp fixe pour l'instant, on verra la vraie expiration après
    };

    let token = encode(
        &Header::default(),           // algorithme HS256 par défaut
        &claims,
        &EncodingKey::from_secret("mon-secret".as_bytes()),
    ).unwrap();

    Json(LoginResponse {
        token,
        email: body.email,
    })
}