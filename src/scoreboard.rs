use super::Team;
use std::ops::Add;

#[derive(Debug)]
pub struct Scoreboard {
    cotos: Vec<Coto>,
}

impl Scoreboard {
    pub fn annotate(&mut self, round_score: RoundScore) {
        //! Annotate a round on the scoreboard and perform management tasks to rotate camas and cotos when required

        self.get_current_coto().annotate(round_score);
        if self.get_current_coto().winner().is_some() {
            self.start_coto();
        }
    }

    pub fn winner(&self) -> Option<Team> {
        let winning_score = 2;
        self.cotos
            .iter()
            .map(|coto| coto.winner())
            .scan((0, 0), |state, x| {
                *state = match x {
                    Some(Team::Team1) => (state.0 + 1, state.1),
                    Some(Team::Team2) => (state.0, state.1 + 1),
                    _ => *state,
                };
                Some(*state)
            })
            .filter(|coto_score| coto_score.0 >= winning_score || coto_score.1 >= winning_score)
            .map(|coto_score| {
                if coto_score.0 >= winning_score {
                    Team::Team1
                } else {
                    Team::Team2
                }
            })
            .nth(0)
    }

    fn start_coto(&mut self) {
        self.cotos.push(Coto::new());
    }

    fn get_current_coto(&mut self) -> &mut Coto {
        self.cotos
            .last_mut()
            .expect("Scoreboard not properly initialised")
    }
}

impl Default for Scoreboard {
    fn default() -> Self {
        let mut scoreboard = Scoreboard { cotos: Vec::new() };
        scoreboard.start_coto();
        scoreboard
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct RoundScoreSection(Team, u8);

impl RoundScoreSection {
    fn to_score_delta(&self) -> ScoreDelta {
        match self.0 {
            Team::Team1 => ScoreDelta {
                team1: self.1,
                team2: 0,
            },
            Team::Team2 => ScoreDelta {
                team1: 0,
                team2: self.1,
            },
        }
    }
}

#[derive(Debug, PartialEq)]
struct ScoreDelta {
    team1: u8,
    team2: u8,
}

impl Add<ScoreDelta> for CamaScore {
    type Output = CamaScore;
    fn add(self, rhs: ScoreDelta) -> Self::Output {
        CamaScore {
            team1: self.team1 + rhs.team1,
            team2: self.team2 + rhs.team2,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct RoundScore {
    pub rey: Option<RoundScoreSection>,
    pub flor: Option<RoundScoreSection>,
    pub secansa: Option<RoundScoreSection>,
    pub ali: Option<RoundScoreSection>,
    pub truc: RoundScoreSection,
}

impl RoundScore {
    fn to_score_deltas(&self) -> Vec<ScoreDelta> {
        let deltas = [self.rey, self.flor, self.secansa, self.ali, Some(self.truc)];
        deltas
            .iter()
            .filter_map(|x| *x)
            .map(|x| x.to_score_delta())
            .collect()
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
struct CamaScore {
    team1: u8,
    team2: u8,
}

#[derive(Debug, Default, PartialEq, Clone)]
struct Cama {
    rounds: Vec<RoundScore>,
}

impl Cama {
    fn get_current_score(&self) -> CamaScore {
        self.rounds
            .iter()
            .flat_map(|x| x.to_score_deltas())
            .fold(CamaScore::default(), |acc, delta| acc + delta)
    }

    fn annotate(&mut self, score: RoundScore) {
        self.rounds.push(score);
    }

    fn winner(&self) -> Option<Team> {
        let winning_score = 40;
        self.rounds
            .iter()
            .flat_map(|x| x.to_score_deltas())
            .scan(CamaScore::default(), |state, x| {
                *state = *state + x;
                Some(*state)
            })
            .filter(|cama_score| {
                cama_score.team1 >= winning_score || cama_score.team2 >= winning_score
            })
            .map(|cama_score| {
                if cama_score.team1 >= winning_score {
                    Team::Team1
                } else {
                    Team::Team2
                }
            })
            .nth(0)
    }
}

#[derive(Debug, PartialEq, Clone)]
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
        self.cames
            .last_mut()
            .expect("Coto not properly initialised")
    }

    fn annotate(&mut self, round_score: RoundScore) {
        //! Annotate a round on the coto and perform management tasks to rotate camas when required

        self.get_current_cama().annotate(round_score);
        if self.get_current_cama().winner().is_some() {
            self.start_cama();
        }
    }

    fn winner(&self) -> Option<Team> {
        let winning_score = 2;
        self.cames
            .iter()
            .map(|cama| cama.winner())
            .scan((0, 0), |state, x| {
                *state = match x {
                    Some(Team::Team1) => (state.0 + 1, state.1),
                    Some(Team::Team2) => (state.0, state.1 + 1),
                    _ => *state,
                };
                Some(*state)
            })
            .filter(|cama_score| cama_score.0 >= winning_score || cama_score.1 >= winning_score)
            .map(|cama_score| {
                if cama_score.0 >= winning_score {
                    Team::Team1
                } else {
                    Team::Team2
                }
            })
            .nth(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scoreboard_get_current_coto() {
        let mut scoreboard: Scoreboard = Default::default();
        scoreboard.get_current_coto().start_cama();
        let coto1 = scoreboard.get_current_coto().clone();
        scoreboard.start_coto();
        let coto2 = scoreboard.get_current_coto();
        assert!(coto1 != *coto2);
    }

    #[test]
    fn scoreboard_start_coto() {
        let mut scoreboard = Scoreboard { cotos: Vec::new() };
        scoreboard.start_coto();
        assert!(!scoreboard.cotos.is_empty());
    }

    #[test]
    fn scoreboard_new() {
        let scoreboard: Scoreboard = Default::default();
        assert!(!scoreboard.cotos.is_empty());
        assert!(!scoreboard.cotos.first().unwrap().cames.is_empty());
    }

    #[test]
    fn scoreboard_annotate_writes_round_score() {
        let mut scoreboard: Scoreboard = Default::default();
        scoreboard.annotate(RoundScore {
            rey: None,
            flor: None,
            secansa: None,
            ali: None,
            truc: RoundScoreSection(Team::Team1, 3),
        });
        assert!(!scoreboard
            .get_current_coto()
            .get_current_cama()
            .rounds
            .is_empty());
    }

    #[test]
    fn scoreboard_annotate_rotates_camas() {
        let mut scoreboard: Scoreboard = Default::default();
        scoreboard.annotate(RoundScore {
            rey: None,
            flor: None,
            secansa: None,
            ali: None,
            truc: RoundScoreSection(Team::Team1, 3),
        });
        assert_eq!(scoreboard.get_current_coto().cames.len(), 1);
        scoreboard.annotate(RoundScore {
            rey: None,
            flor: None,
            secansa: None,
            ali: None,
            truc: RoundScoreSection(Team::Team1, 37),
        });
        assert_eq!(scoreboard.get_current_coto().cames.len(), 2);
    }

    #[test]
    fn scoreboard_annotate_rotates_cotos() {
        let mut scoreboard: Scoreboard = Default::default();
        scoreboard.annotate(RoundScore {
            rey: None,
            flor: None,
            secansa: None,
            ali: None,
            truc: RoundScoreSection(Team::Team1, 40),
        });
        scoreboard.annotate(RoundScore {
            rey: None,
            flor: None,
            secansa: None,
            ali: None,
            truc: RoundScoreSection(Team::Team2, 40),
        });
        assert_eq!(scoreboard.cotos.len(), 1);
        scoreboard.annotate(RoundScore {
            rey: None,
            flor: None,
            secansa: None,
            ali: None,
            truc: RoundScoreSection(Team::Team1, 40),
        });
        assert_eq!(scoreboard.cotos.len(), 2);
    }

    #[test]
    fn scoreboard_winner() {
        let mut scoreboard: Scoreboard = Default::default();
        assert!(scoreboard.winner().is_none());

        fn annotate(scoreboard: &mut Scoreboard, team: Team) {
            scoreboard
                .get_current_coto()
                .get_current_cama()
                .annotate(RoundScore {
                    rey: None,
                    flor: None,
                    secansa: None,
                    ali: None,
                    truc: RoundScoreSection(team, 40),
                });
            scoreboard.get_current_coto().start_cama();
        }

        annotate(&mut scoreboard, Team::Team1);
        assert!(scoreboard.winner().is_none());

        annotate(&mut scoreboard, Team::Team2);
        assert!(scoreboard.winner().is_none());

        annotate(&mut scoreboard, Team::Team2);
        // Team2 coto
        assert!(scoreboard.winner().is_none());

        scoreboard.start_coto();

        annotate(&mut scoreboard, Team::Team2);
        assert!(scoreboard.winner().is_none());

        annotate(&mut scoreboard, Team::Team1);
        assert!(scoreboard.winner().is_none());

        annotate(&mut scoreboard, Team::Team1);
        // Team1 coto
        assert!(scoreboard.winner().is_none());

        scoreboard.start_coto();

        annotate(&mut scoreboard, Team::Team1);
        assert!(scoreboard.winner().is_none());

        annotate(&mut scoreboard, Team::Team1);
        // Team1 wins!
        assert_eq!(scoreboard.winner(), Some(Team::Team1));
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
        coto.get_current_cama().annotate(RoundScore {
            rey: None,
            flor: Some(RoundScoreSection(Team::Team1, 3)),
            secansa: Some(RoundScoreSection(Team::Team1, 1)),
            ali: Some(RoundScoreSection(Team::Team2, 5)),
            truc: RoundScoreSection(Team::Team1, 1),
        });
        let cama1 = coto.get_current_cama().clone();
        coto.start_cama();
        let cama2 = coto.get_current_cama();
        assert!(cama1 != *cama2);
    }

    #[test]
    fn coto_annotate_writes_round_score() {
        let mut coto = Coto::new();
        coto.annotate(RoundScore {
            rey: None,
            flor: None,
            secansa: None,
            ali: None,
            truc: RoundScoreSection(Team::Team1, 3),
        });
        assert!(!coto.get_current_cama().rounds.is_empty());
    }

    #[test]
    fn coto_annotate_rotates_camas() {
        let mut coto = Coto::new();
        coto.annotate(RoundScore {
            rey: None,
            flor: None,
            secansa: None,
            ali: None,
            truc: RoundScoreSection(Team::Team1, 3),
        });
        assert_eq!(coto.cames.len(), 1);
        coto.annotate(RoundScore {
            rey: None,
            flor: None,
            secansa: None,
            ali: None,
            truc: RoundScoreSection(Team::Team1, 37),
        });
        assert_eq!(coto.cames.len(), 2);
    }

    #[test]
    fn coto_winner() {
        let mut coto = Coto::new();
        assert_eq!(coto.winner(), None);

        coto.start_cama();
        coto.get_current_cama().annotate(RoundScore {
            rey: None,
            flor: None,
            secansa: None,
            ali: None,
            truc: RoundScoreSection(Team::Team1, 40),
        });
        assert_eq!(coto.winner(), None);

        coto.start_cama();
        coto.get_current_cama().annotate(RoundScore {
            rey: None,
            flor: None,
            secansa: None,
            ali: None,
            truc: RoundScoreSection(Team::Team2, 40),
        });
        assert_eq!(coto.winner(), None);

        coto.start_cama();
        coto.get_current_cama().annotate(RoundScore {
            rey: None,
            flor: None,
            secansa: None,
            ali: None,
            truc: RoundScoreSection(Team::Team1, 40),
        });

        assert_eq!(coto.winner(), Some(Team::Team1));
    }

    #[test]
    fn cama_annotate() {
        let round_score = RoundScore {
            rey: None,
            flor: Some(RoundScoreSection(Team::Team1, 3)),
            secansa: Some(RoundScoreSection(Team::Team1, 1)),
            ali: Some(RoundScoreSection(Team::Team2, 5)),
            truc: RoundScoreSection(Team::Team1, 1),
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
            truc: RoundScoreSection(Team::Team1, 1),
        });
        cama.annotate(RoundScore {
            rey: Some(RoundScoreSection(Team::Team1, 2)),
            flor: Some(RoundScoreSection(Team::Team2, 6)),
            secansa: Some(RoundScoreSection(Team::Team1, 3)),
            ali: Some(RoundScoreSection(Team::Team1, 1)),
            truc: RoundScoreSection(Team::Team1, 1),
        });
        let score = cama.get_current_score();
        let expected = CamaScore {
            team1: 12,
            team2: 11,
        };
        assert_eq!(score, expected);
    }

    #[test]
    fn cama_winner() {
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
