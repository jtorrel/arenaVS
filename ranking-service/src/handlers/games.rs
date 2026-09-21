// --- Rankking Service ---
// --- src/handlers/games.rs ---

use crate::auth::{AuthenticatedUser, Role};
use crate::db;
use crate::elo::EloConfig;
use crate::errors::AppError;
use crate::models::player::Player;
use axum::{Json, http::HeaderMap};

/// Récupère une partie par son identifiant.
pub async fn get_game_by_id(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(game_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    db::games::get_game_by_id(&pool, game_id)
        .await
        .map(Json)
        .map_err(AppError::Database)
}

/// Récupère les parties associées à un joueur.
pub async fn get_games_by_player_id(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(player_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    db::games::get_games_by_player_id(&pool, player_id)
        .await
        .map(Json)
        .map_err(AppError::Database)
}

#[derive(Debug, Deserialize)]
struct CreateGameRequest {
    player1_id: Uuid,
    player2_id: Uuid,
}

/// Crée une partie entre deux joueurs.
pub async fn create_game(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Json(payload): Json<CreateGameRequest>,
) -> Result<impl IntoResponse, AppError> {
    // GameServer only
    if auth.role != Role::GameServer {
        return Err(AppError::Forbidden("...".to_string()));
    }

    let config = EloConfig::default();
    let game = Game::new(payload.player1_id, payload.player2_id);

    let game = db::games::create_game(&pool, &game)
        .await
        .map_err(AppError::Database)?;

    Ok(Json(game))
}

/// Met à jour l'état d'une partie.
pub async fn update_game_state(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(game_id): Path<Uuid>,
    Json(new_state): Json<models::game::GameState>,
) -> Result<impl IntoResponse, AppError> {
    // GameServer only
    if auth.role != Role::GameServer {
        return Err(AppError::Forbidden("...".to_string()));
    }

    let updated_game = db::games::update_game_state(&pool, game_id, new_state)
        .await
        .map_err(AppError::Database)?;

    Ok(Json(updated_game))
}

pub async fn update_elo(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(game_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // GameServer only !
    if auth.role != Role::GameServer {
        return Err(AppError::Forbidden("...".to_string()));
    }

    let mut game = db::games::get_game_by_id(&pool, game_id).await?;
    if game.state != GameState::AwaitingResults {
        return Err(AppError::BadRequest(
            "Game is not awaiting results".to_string(),
        ));
    }

    let game_details = db::games::get_game_details(&pool, game_id).await?;

    // Rapports cohérents ?
    if !check_reports(&game_details) {
        db::game::update_game_state(&pool, game_id, models::game::GameState::Disputed);
        return Err(AppError::BadRequest("Reports are not matching".to_string()));
    }

    // La partie est valide
    // Calcul du résultat
    let p1_score = game_details.p1_report.as_ref().unwrap().p1_score;
    let p2_score = game_details.p1_report.as_ref().unwrap().p2_score;

    let p1_result = if p1_score > p2_score {
        Score::Win
    } else if p1_score == p2_score {
        Score::Draw
    } else {
        Score::Loss
    };

    let p2_result = if p1_score < p2_score {
        Score::Win
    } else if p1_score == p2_score {
        Score::Draw
    } else {
        Score::Loss
    };

    // Mise à jour des ELO
    let config = EloConfig::default();

    let new_p1_elo = elo::update_elo(
        game_details.p1.elo as f64,
        game_details.p2.elo as f64,
        p1_result,
        config.k_factor,
    );
    let new_p2_elo = elo::update_elo(
        game_details.p2.elo as f64,
        game_details.p1.elo as f64,
        p2_result,
        config.k_factor,
    );

    // Mise à jour de la BDD
    db::players::update_elo(&pool, game_details.player1.id, new_p1_elo.round() as i32)
        .await
        .map_err(AppError::Database)?;
    db::players::update_elo(&pool, game_details.player2.id, new_p2_elo.round() as i32)
        .await
        .map_err(AppError::Database)?;

    db::games::update_game_state(&pool, game_id, GameState::Completed)
        .await
        .map_err(AppError::Database)?;
    game.state = GameState::Completed;

    Ok(Json(game))
}
