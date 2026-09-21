// --- src/db/players.rs ---

use crate::models::player::Player;
use sqlx::PgPool;
use uuid::Uuid;

/// Crée un joueur dans la base de données.
///
/// # Errors
/// Retourne l'erreur SQL rencontrée lors de l'insertion ou de la récupération du joueur.
pub async fn create_player(pool: &PgPool, player: &Player) -> Result<Player, sqlx::Error> {
    sqlx::query_as!(
        Player,
        r#"
        INSERT INTO players (id, nickname, elo)
        VALUES ($1, $2, $3)
        RETURNING id, nickname, elo, created_at, updated_at
        "#,
        player.id,
        player.nickname,
        player.elo
    )
    .fetch_one(pool)
    .await
}

/// Récupère un joueur par son ID.
///
/// # Errors
/// Retourne `RowNotFound` si l'ID n'existe pas en base.
pub async fn get_player(pool: &PgPool, player_id: Uuid) -> Result<Player, sqlx::Error> {
    sqlx::query_as!(
        Player,
        r#"
        SELECT id, nickname, elo, created_at, updated_at
        FROM players
        WHERE id = $1
        "#,
        player_id
    )
    .fetch_one(pool)
    .await
}
