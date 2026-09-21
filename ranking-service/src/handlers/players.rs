// --- Rankking Service ---
// --- src/handlers/players.rs ---

use crate::auth::{AuthenticatedUser, Role};
use crate::db;
use crate::elo::EloConfig;
use crate::errors::AppError;
use crate::models::player::Player;
use axum::{Json, http::HeaderMap};

/// Récupère un joueur par son identifiant.
pub async fn get_player_by_id(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(player_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    db::players::get_player_by_id(&pool, player_id)
        .await
        .map(Json)
        .map_err(AppError::Database)
}

#[derive(Debug, Deserialize)]
struct CreatePlayerRequest {
    id: Uuid,
    nickname: String,
}

/// Crée un joueur à partir des données transmises par le serveur de jeu.
pub async fn create_player(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Json(payload): Json<CreatePlayerRequest>,
) -> Result<impl IntoResponse, AppError> {
    if auth.role != Role::GameServer {
        return Err(AppError::Forbidden("...".to_string()));
    }

    let config = EloConfig::default();
    let player = Player::new(payload.id, payload.nickname, &config);

    let player = db::players::create_player(&pool, &player)
        .await
        .map_err(AppError::Database)?;

    Ok(Json(player))
}
