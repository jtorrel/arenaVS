// --- Rankking Service ---
// --- src/models/player.rs ---

use crate::elo::EloConfig;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
/// Joueur enregistré dans le service de classement.
pub struct Player {
    /// Identifiant du joueur.
    pub id: Uuid,
    /// Nom affiché du joueur.
    pub nickname: String,
    /// Classement ELO courant du joueur.
    pub elo: i32,
    /// Date de création du joueur.
    pub created_at: Option<NaiveDateTime>,
    /// Date de dernière mise à jour du joueur.
    pub updated_at: Option<NaiveDateTime>,
}

impl Player {
    /// Construit un joueur avec l'ELO initial fourni par la configuration.
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
