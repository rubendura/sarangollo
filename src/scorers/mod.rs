pub mod ali;
pub mod flor;
pub mod rey;
pub mod secansa;
pub mod truc;

use scoreboard;
use Round;

pub trait Scorer {
    fn get_score(&self, round: &Round) -> Option<scoreboard::RoundScoreSection>;
}
