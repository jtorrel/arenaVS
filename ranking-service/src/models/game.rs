// --- src/models/game.rs ---

use crate::models::player::Player;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameReport {
    pub p1_score: u8, // score of the player submitting the report
    pub p2_score: u8, // score of the opponent player
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: Uuid,
    pub p1_id: Uuid,
    pub p2_id: Uuid,
    pub p1_report: Option<GameReport>, // report submitted by player 1
    pub p2_report: Option<GameReport>, // report submitted by player 2
    pub state: GameState,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameDetails {
    pub game_data: Game,
    pub p1: Player,
    pub p2: Player,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GameState {
    Pending,         // game created, waiting for players
    InProgress,      // game in progress
    AwaitingResults, // game completed, waiting for reports
    Completed,       // reports are consistent, ELO updated
    Disputed,        // reports are contradictory
}
