pub mod ali;
pub mod flor;
pub mod secansa;

use scoreboard;
use Round;

pub trait Scorer {
    fn get_score(&self, round: &Round) -> Option<scoreboard::RoundScoreSection>;
}
