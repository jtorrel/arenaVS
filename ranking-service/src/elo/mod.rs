// --- src/elo/mod.rs ---
use crate::models::score::Score;

#[derive(Debug)]
pub struct EloConfig {
    pub k_factor: f64,
    pub initial_elo: f64,
}

impl Default for EloConfig {
    /// Retourne la configuration ELO par défaut.
    fn default() -> Self {
        Self {
            k_factor: 32.0,
            initial_elo: 1000.0,
        }
    }
}

/// Calcule le score attendu d'un joueur face à son adversaire.
pub fn calculate_expected_score(player_elo: f64, opponent_elo: f64) -> f64 {
    1.0 / (1.0 + 10.0_f64.powf((opponent_elo - player_elo) / 400.0))
}

/// Calcule le nouvel ELO d'un joueur après une partie.
pub fn update_elo(player_elo: f64, opponent_elo: f64, score: Score, k_factor: f64) -> f64 {
    let expected_score = calculate_expected_score(player_elo, opponent_elo);
    player_elo + k_factor * (score.as_score() - expected_score)
}
