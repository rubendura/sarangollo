pub mod flor;

use scoreboard;
use Round;

pub trait Scorer {
    fn get_score(&self, round: &Round) -> Option<scoreboard::RoundScoreSection>;
}
