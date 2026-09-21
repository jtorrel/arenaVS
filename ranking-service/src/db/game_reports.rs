// --- Rankking Service ---
// --- src/db/game_reports.rs ---

/// Crée un rapport de partie.
///
/// # Errors    
/// Retourne l'erreur SQL rencontrée lors de l'insertion ou de la récupération de la partie.
pub async fn create_game_report(
    pool: &PgPool,
    report: &GameReport,
) -> Result<GameReport, sqlx::Error> {
    sqlx::query_as!(
        GameReport,
        r#"
        INSERT INTO game_reports (id, game_id, reporter_id, p1_score, p2_score)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, game_id, reporter_id, p1_score, p2_score
        "#,
        report.id,
        report.game_id,
        report.reporter_id,
        report.p1_score,
        report.p2_score
    )
    .fetch_one(pool)
    .await
}

/// Récupère un rapport de partie par son ID.
///
/// # Errors
/// Retourne `RowNotFound` si l'ID n'existe pas en base.
pub async fn get_game_report_by_id(
    pool: &PgPool,
    report_id: Uuid,
) -> Result<GameReport, sqlx::Error> {
    sqlx::query_as!(
        GameReport,
        r#"
        SELECT id, game_id, reporter_id, p1_score, p2_score
        FROM game_reports
        WHERE id = $1
        "#,
        report_id
    )
    .fetch_one(pool)
    .await
}

/// Récupère un rapport de partie par l'ID du rapporteur.
///
/// # Errors
/// Retourne `RowNotFound` si l'ID n'existe pas en base.
pub async fn get_game_report_by_reporter_id(
    pool: &PgPool,
    reporter_id: Uuid,
) -> Result<Vec<GameReport>, sqlx::Error> {
    sqlx::query_as!(
        GameReport,
        r#"
        SELECT id, game_id, reporter_id, p1_score, p2_score
        FROM game_reports
        WHERE reporter_id = $1
        "#,
        reporter_id
    )
    .fetch_all(pool)
    .await
}

/// Récupère un rapport de partie par l'ID de la partie.
///
/// # Errors
/// Retourne `RowNotFound` si l'ID n'existe pas en base.
pub async fn get_game_report_by_game_id(
    pool: &PgPool,
    game_id: Uuid,
) -> Result<Vec<GameReport>, sqlx::Error> {
    sqlx::query_as!(
        GameReport,
        r#"
        SELECT id, game_id, reporter_id, p1_score, p2_score
        FROM game_reports
        WHERE game_id = $1
        "#,
        game_id
    )
    .fetch_all(pool)
    .await
}
