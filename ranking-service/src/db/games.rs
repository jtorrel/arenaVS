// --- Rankking Service ---
// --- src/db/games.rs ---

/// Crée une partie
///
/// # Errors
/// Retourne l'erreur SQL rencontrée lors de l'insertion ou de la récupération de la partie.
pub async fn create_game(pool: &PgPool, game: &Game) -> Result<Game, sqlx::Error> {
    sqlx::query_as!(
        Game,
        r#"
        INSERT INTO games (id, player1_id, player2_id)
        VALUES ($1, $2, $3)
        RETURNING id, player1_id, player2_id, p1_report_id, p2_report_id, state, created_at, updated_at
        "#,
        game.id,
        game.player1_id,
        game.player2_id
    )
    .fetch_one(pool)
    .await
}

/// Récupère une partie par son ID.
///
/// # Errors
/// Retourne `RowNotFound` si l'ID n'existe pas en base.
pub async fn get_game_by_id(pool: &PgPool, game_id: Uuid) -> Result<Game, sqlx::Error> {
    sqlx::query_as!(
        Game,
        r#"
        SELECT id, player1_id, player2_id, p1_report_id, p2_report_id, state, created_at, updated_at
        FROM games
        WHERE id = $1
        "#,
        game_id
    )
    .fetch_one(pool)
    .await
}

/// Récupère les parties par leur état.
///
/// # Errors
/// Retourne l'erreur SQL rencontrée lors de la récupération des parties ou un vecteur vide si l'état n'existe pas.
pub async fn get_games_by_state(pool: &PgPool, state: GameState) -> Result<Vec<Game>, sqlx::Error> {
    sqlx::query_as!(
        Game,
        r#"
        SELECT id, player1_id, player2_id, p1_report_id, p2_report_id, state, created_at, updated_at
        FROM games
        WHERE state = $1
        "#,
        state
    )
    .fetch_all(pool)
    .await
}

/// Récupère les parties par l'ID d'un joueur.
///
/// # Errors
/// Retourne l'erreur SQL rencontrée lors de la récupération des parties ou un vecteur vide si le joueur n'a pas de parties.
pub async fn get_games_by_player_id(
    pool: &PgPool,
    player_id: Uuid,
) -> Result<Vec<Game>, sqlx::Error> {
    sqlx::query_as!(
        Game,
        r#"
        SELECT id, player1_id, player2_id, p1_report_id, p2_report_id, state, created_at, updated_at
        FROM games
        WHERE player1_id = $1 OR player2_id = $1
        "#,
        player_id
    )
    .fetch_all(pool)
    .await
}

/// Met à jour l'état d'une partie.
///
/// # Errors
/// Retourne l'erreur SQL rencontrée lors de la mise à jour.
pub async fn update_game_state(
    pool: &PgPool,
    game_id: Uuid,
    state: GameState,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE games
        SET state = $2, updated_at = NOW()
        WHERE id = $1
        "#,
        game_id,
        state
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Soumet le résultat d'une partie.
///
/// # Errors
/// Retourne l'erreur SQL rencontrée lors de la soumission.
pub async fn submit_game_report(
    pool: &PgPool,
    report: GameReport,
    game: Game,
) -> Result<Game, sqlx::Error> {
    let p1_or_p2 = report.reporter_id == game.player1_id;
    sqlx::query_as!(
        Game,
        r#"
        UPDATE games
        SET 
            p1_report_id = CASE WHEN $3 = true THEN $2 ELSE p1_report_id END,
            p2_report_id = CASE WHEN $3 = false THEN $2 ELSE p2_report_id END,
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, player1_id, player2_id, p1_report_id, p2_report_id, state, created_at, updated_at
        "#,
        game.id,
        report.id,
        p1_or_p2,
    )
    .fetch_one(pool)
    .await?;
    Ok(())
}

pub async fn get_game_details(pool: &PgPool, game_id: Uuid) -> Result<GameDetails, sqlx::Error> {
    let game = get_game_by_id(pool, game_id).await?;
    let p1 = db::players::get_player_by_id(pool, game.player1_id).await?;
    let p2 = db::players::get_player_by_id(pool, game.player2_id).await?;

    let p1_report = match game.p1_report_id {
        Some(id) => Some(db::game_reports::get_game_report_by_id(pool, id).await?),
        None => None,
    };

    let p2_report = match game.p2_report_id {
        Some(id) => Some(db::game_reports::get_game_report_by_id(pool, id).await?),
        None => None,
    };

    Ok(GameDetails {
        game_data: game,
        p1,
        p2,
        p1_report,
        p2_report,
    })
}
