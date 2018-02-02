use super::Team;
use std::ops::Add;

#[derive(Debug)]
pub struct Scoreboard {
    cotos: Vec<Coto>,
}

impl Scoreboard {

    pub fn new() -> Scoreboard {
        let mut scoreboard = Scoreboard { cotos: Vec::new() };
        scoreboard.start_coto();
        scoreboard
    }

    fn start_coto(&mut self) {
        self.cotos.push(Coto::new());
    }

    fn get_current_coto(&self) -> &Coto {
        unimplemented!();
    }

}

#[derive(Debug, PartialEq, Copy, Clone)]
struct RoundScoreSection(Team, u8);

impl RoundScoreSection {
    fn to_score_delta(&self) -> ScoreDelta {
        match self.0 {
            Team::Team1 => ScoreDelta { team1: self.1, team2: 0 },
            Team::Team2 => ScoreDelta { team1: 0, team2: self.1 }
        }
    }
}

#[derive(Debug, PartialEq)]
struct ScoreDelta {
    team1: u8,
    team2: u8
}

impl Add<ScoreDelta> for CamaScore {
    type Output = CamaScore;
    fn add(self, rhs: ScoreDelta) -> Self::Output {
        CamaScore{
            team1: self.team1 + rhs.team1,
            team2: self.team2 + rhs.team2
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct RoundScore {
    rey: Option<RoundScoreSection>,
    flor: Option<RoundScoreSection>,
    secansa: Option<RoundScoreSection>,
    ali: Option<RoundScoreSection>,
    truc: RoundScoreSection,
}

impl RoundScore {
    fn to_score_deltas(&self) -> Vec<ScoreDelta> {
        let deltas = [
            self.rey,
            self.flor,
            self.secansa,
            self.ali
        ];
        let mut deltas: Vec<ScoreDelta> = deltas.iter().flat_map(|x| x.iter()).map(|x| x.to_score_delta()).collect();
        deltas.push(self.truc.to_score_delta());
        deltas
    }
}

#[derive(Debug, PartialEq, Default)]
struct CamaScore {
    team1: u8,
    team2: u8
}

#[derive(Debug, Default, PartialEq, Clone)]
struct Cama {
    rounds: Vec<RoundScore>,
}

impl Cama {

    fn get_current_score(&self) -> CamaScore {
        self.rounds.iter().flat_map(|x| x.to_score_deltas()).fold(CamaScore::default(), |acc, delta| acc + delta)
    }

    fn annotate(&mut self, score: RoundScore) {
        self.rounds.push(score);
    }

    fn winner(&self) -> Option<Team> {
        let winning_score = 40;
        let mut team1_score = 0;
        let mut team2_score = 0;
        for round in &self.rounds {
            let mut score_parts = vec![];
            if let Some(ref rey) = round.rey {
                score_parts.push(rey);
            }
            if let Some(ref flor) = round.flor {
                score_parts.push(flor);
            }
            if let Some(ref secansa) = round.secansa {
                score_parts.push(secansa);
            }
            if let Some(ref ali) = round.ali {
                score_parts.push(ali);
            }
            score_parts.push(&round.truc);
            for part in &score_parts {
                match part.0 {
                    Team::Team1 => {
                        team1_score += part.1;
                        if team1_score >= winning_score {
                            return Some(Team::Team1);
                        }
                    }
                    Team::Team2 => {
                        team2_score += part.1;
                        if team2_score >= winning_score {
                            return Some(Team::Team2);
                        }
                    }
                }
            }
        }
        None
    }
}

#[derive(Debug)]
struct Coto {
    cames: Vec<Cama>,
}

impl Coto {

    fn new() -> Coto {
        let mut coto = Coto { cames: vec![] };
        coto.start_cama();
        coto
    }

    fn start_cama(&mut self) {
        self.cames.push(Cama::default());
    }

    fn get_current_cama(&mut self) -> &mut Cama {
        self.cames.last_mut().expect("Coto not properly initialised")
    }

    fn winner(&self) -> Option<Team> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scoreboard_get_current_coto() {
        unimplemented!();
    }

    #[test]
    fn scoreboard_start_coto() {
        unimplemented!();
    }

    #[test]
    fn scoreboard_new() {
        unimplemented!();
    }

    #[test]
    fn coto_start_cama() {
        let mut coto = Coto { cames: Vec::new() };
        coto.start_cama();
        assert!(!coto.cames.is_empty());
    }

    #[test]
    fn coto_new() {
        let coto = Coto::new();
        assert!(!coto.cames.is_empty());
        assert!(coto.cames.first().unwrap().rounds.is_empty());
    }

    #[test]
    fn coto_get_current_cama() {
        let mut coto = Coto::new();
        coto.start_cama();
        let mut cama1 = coto.get_current_cama().clone();
        cama1.annotate(RoundScore {
            rey: None,
            flor: Some(RoundScoreSection(Team::Team1, 3)),
            secansa: Some(RoundScoreSection(Team::Team1, 1)),
            ali: Some(RoundScoreSection(Team::Team2, 5)),
            truc: RoundScoreSection(Team::Team1, 1)
        });
        let cama2 = coto.get_current_cama();
        assert!(cama1 != *cama2);
    }

    #[test]
    fn cama_annotate() {
        let round_score = RoundScore {
            rey: None,
            flor: Some(RoundScoreSection(Team::Team1, 3)),
            secansa: Some(RoundScoreSection(Team::Team1, 1)),
            ali: Some(RoundScoreSection(Team::Team2, 5)),
            truc: RoundScoreSection(Team::Team1, 1)
        };
        let mut cama = Cama::default();
        assert!(cama.rounds.len() == 0);
        cama.annotate(round_score);
        assert!(cama.rounds.len() == 1);
        assert!(*cama.rounds.first().unwrap() == round_score);
    }

    #[test]
    fn cama_get_current_score() {
        let mut cama = Cama::default();
        cama.annotate(RoundScore {
            rey: None,
            flor: Some(RoundScoreSection(Team::Team1, 3)),
            secansa: Some(RoundScoreSection(Team::Team1, 1)),
            ali: Some(RoundScoreSection(Team::Team2, 5)),
            truc: RoundScoreSection(Team::Team1, 1)
        });
        cama.annotate(RoundScore {
            rey: Some(RoundScoreSection(Team::Team1, 2)),
            flor: Some(RoundScoreSection(Team::Team2, 6)),
            secansa: Some(RoundScoreSection(Team::Team1, 3)),
            ali: Some(RoundScoreSection(Team::Team1, 1)),
            truc: RoundScoreSection(Team::Team1, 1)
        });
        let score = cama.get_current_score();
        let expected = CamaScore { team1: 12, team2: 11 };
        assert_eq!(score, expected);
    }

    #[test]
    fn winner() {
        // Team1: 35, Team2: 34
        let current_score = RoundScore {
            rey: None,
            flor: Some(RoundScoreSection(Team::Team1, 35)),
            secansa: None,
            ali: None,
            truc: RoundScoreSection(Team::Team2, 34),
        };

        let mut current_cama = Cama {
            rounds: vec![current_score],
        };

        assert_eq!(current_cama.winner(), None);

        // Team1: 5, Team2: 6
        current_cama.annotate(RoundScore {
            rey: Some(RoundScoreSection(Team::Team2, 1)), // Team2: 35
            flor: Some(RoundScoreSection(Team::Team1, 3)), // Team1: 38
            secansa: Some(RoundScoreSection(Team::Team1, 1)), // Team1: 39
            ali: Some(RoundScoreSection(Team::Team2, 5)), // Team2: 40 -> Cama!
            truc: RoundScoreSection(Team::Team1, 1),      // Not used
        });

        let winner = current_cama.winner();

        assert_eq!(winner, Some(Team::Team2));
    }
}
