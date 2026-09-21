// --- Rankking Service ---
// --- src/models/game.rs ---

use crate::models::player::Player;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
/// Résultat soumis par un joueur pour une partie.
pub struct GameReport {
    /// Identifiant du rapport.
    pub id: Uuid,
    /// Identifiant de la partie concernée.
    pub game_id: Uuid,
    /// Identifiant du joueur qui soumet le rapport.
    pub reporter_id: Uuid,
    /// Score du joueur qui soumet le rapport.
    pub p1_score: i32, // score of the player submitting the report
    /// Score du joueur adverse.
    pub p2_score: i32, // score of the opponent player
}

#[derive(Debug, Serialize, Deserialize)]
/// Partie opposant deux joueurs.
pub struct Game {
    /// Identifiant de la partie.
    pub id: Uuid,
    /// Identifiant du premier joueur.
    pub player1_id: Uuid,
    /// Identifiant du second joueur.
    pub player2_id: Uuid,
    /// Identifiant du rapport soumis par le premier joueur.
    pub p1_report_id: Option<Uuid>,
    /// Identifiant du rapport soumis par le second joueur.
    pub p2_report_id: Option<Uuid>,
    /// État courant de la partie.
    pub state: GameState,
    /// Date de création de la partie.
    pub created_at: Option<NaiveDateTime>,
    /// Date de dernière mise à jour de la partie.
    pub updated_at: Option<NaiveDateTime>,
}

impl Game {
    /// Construit une partie en cours entre deux joueurs.
    pub fn new(player1_id: Uuid, player2_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            player1_id,
            player2_id,
            p1_report_id: None,
            p2_report_id: None,
            state: GameState::InProgress,
            created_at: None,
            updated_at: None,
        }
    }

    // Vérifie la cohérence des rapports de partie
    pub fn check_reports(game_details: &GameDetails) -> bool {
        match (&game_details.player1_report, &game_details.player2_report) {
            (Some(r1), Some(r2)) => r1.p1_score == r2.p1_score && r1.p2_score == r2.p2_score,
            _ => false, // un des deux rapports manque
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// Partie enrichie avec ses joueurs et leurs rapports éventuels.
pub struct GameDetails {
    /// Données de la partie.
    pub game_data: Game,
    /// Premier joueur de la partie.
    pub player1: Player,
    /// Second joueur de la partie.
    pub player2: Player,
    /// Rapport du premier joueur, s'il a été soumis.
    pub player1_report: Option<GameReport>,
    /// Rapport du second joueur, s'il a été soumis.
    pub player2_report: Option<GameReport>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
/// État d'avancement d'une partie.
pub enum GameState {
    /// Partie actuellement en cours.
    InProgress, // game in progress
    /// Partie terminée, en attente des rapports.
    AwaitingResults, // game completed, waiting for reports
    /// Rapports cohérents et ELO mis à jour.
    Completed, // reports are consistent, ELO updated
    /// Rapports contradictoires nécessitant un traitement.
    Disputed, // reports are contradictory
}
