// --- src/models/score.rs ---
#[derive(Debug)]
pub enum Score {
    Win,
    Draw,
    Loss,
}

impl Score {
    pub fn as_score(&self) -> f64 {
        match self {
            Score::Win => 1.0,
            Score::Draw => 0.5,
            Score::Loss => 0.0,
        }
    }
}
