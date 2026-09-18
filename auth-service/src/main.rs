// --- src/main.rs ---

mod errors;
mod handlers;
mod models;

use axum::{
    Router,
    routing::{get, post},
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(handlers::health))
        .route("/auth/login", post(handlers::login))
        .route("/auth/me", get(handlers::me))
        .route("/auth/introspect", post(handlers::introspect));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server started on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
