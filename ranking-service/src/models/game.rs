// --- src/models/game.rs ---

use crate::models::player::Player;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameReport {
    pub id: Uuid,
    pub game_id: Uuid,
    pub reporter_id: Uuid,
    pub p1_score: i32, // score of the player submitting the report
    pub p2_score: i32, // score of the opponent player
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: Uuid,
    pub player1_id: Uuid,
    pub player2_id: Uuid,
    pub p1_report_id: Option<Uuid>,
    pub p2_report_id: Option<Uuid>,
    pub state: GameState,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameDetails {
    pub game_data: Game,
    pub player1: Player,
    pub player2: Player,
    pub player1_report: Option<GameReport>,
    pub player2_report: Option<GameReport>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GameState {
    InProgress,      // game in progress
    AwaitingResults, // game completed, waiting for reports
    Completed,       // reports are consistent, ELO updated
    Disputed,        // reports are contradictory
}
