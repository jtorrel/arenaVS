// --- Rankking Service ---
// --- src/models/score.rs ---
#[derive(Debug)]
/// Résultat d'une partie pour un joueur.
pub enum Score {
    /// Victoire du joueur.
    Win,
    /// Match nul.
    Draw,
    /// Défaite du joueur.
    Loss,
}

impl Score {
    /// Convertit le résultat en valeur numérique utilisée par le calcul ELO.
    pub fn as_score(&self) -> f64 {
        match self {
            Score::Win => 1.0,
            Score::Draw => 0.5,
            Score::Loss => 0.0,
        }
    }
}
