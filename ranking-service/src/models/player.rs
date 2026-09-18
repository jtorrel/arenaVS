// --- src/models/player.rs ---

use crate::elo::EloConfig;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub nickname: String,
    pub elo: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Player {
    pub fn new(id: Uuid, nickname: String, config: &EloConfig) -> Self {
        Self {
            id,
            nickname,
            elo: config.initial_elo.round() as i32,
            created_at: None,
            updated_at: None,
        }
    }
}
