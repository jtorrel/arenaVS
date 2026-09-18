// --- src/db/players.rs ---

use crate::models::player::Player;
use sqlx::PgPool;

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
