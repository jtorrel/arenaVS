// --- Rankking Service ---
// --- src/auth/mod.rs ---

use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,  // l'UUID du joueur
    role: String, // "player" ou "game_server"
}

#[derive(Debug, Clone, PartialEq)]
/// Rôle attribué à l'utilisateur authentifié.
pub enum Role {
    /// Utilisateur correspondant à un joueur.
    Player,
    /// Serveur autorisé à gérer les parties.
    GameServer,
}

#[derive(Debug, Clone)]
/// Utilisateur identifié à partir du token JWT.
pub struct AuthenticatedUser {
    /// Identifiant de l'utilisateur authentifié.
    pub user_id: Uuid,
    /// Rôle de l'utilisateur authentifié.
    pub role: Role,
}

impl AuthenticatedUser {
    /// Construit un utilisateur authentifié avec son identifiant et son rôle.
    pub fn new(user_id: Uuid, role: Role) -> Self {
        Self { user_id, role }
    }
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 1. Extraire le header Authorization
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        // 2. Vérifier le format "Bearer <token>"
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(StatusCode::UNAUTHORIZED)?;

        // 3. Valider le JWT
        let secret = std::env::var("JWT_SECRET").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

        // 4. Extraire sub et role
        let user_id =
            Uuid::parse_str(&token_data.claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)?;

        let role = match token_data.claims.role.as_str() {
            "game_server" => Role::GameServer,
            _ => Role::Player,
        };

        Ok(AuthenticatedUser::new(user_id, role))
    }
}
