// --- src/models/player.rs ---

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub nickname: String,
    pub elo: u16,
}
