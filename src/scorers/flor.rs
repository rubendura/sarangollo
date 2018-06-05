use super::Scorer;
use hands::{flor, Hand};
use scoreboard;
use GameBet;
use Round;
use Team;

pub struct FlorScorer {
    flor_bet: Option<GameBet<flor::Bet>>,
}

impl FlorScorer {
    pub fn set_bet(&mut self, agreed_bet: flor::Bet, winner: Option<Team>) {
        self.flor_bet = Some(GameBet { agreed_bet, winner });
    }
}

impl Default for FlorScorer {
    fn default() -> Self {
        FlorScorer { flor_bet: None }
    }
}

impl Scorer for FlorScorer {
    fn get_score(&self, round: &Round) -> Option<scoreboard::RoundScoreSection> {
        // Game must've been announced to be scored
        let game_bet = self.flor_bet?;

        let cards_winner = round.get_winner_from_cards::<flor::Flor>();

        let winner = match (game_bet.winner, cards_winner) {
            // If there was a rejected bet, we've got a direct winner
            (Some(winner), _) => winner,
            // Otherwise get the winner from the facing up cards
            (None, Some(winner)) => winner,
            // If there still no winner, we can't score anything
            _ => return None,
        };

        let winner_flor_count = round
            .seats
            .iter()
            .enumerate()
            .filter(|&(pos, seat)| seat.get_team(pos as u8) == winner)
            .filter_map(|(_, seat)| flor::Flor::from_cards(&seat.face_up_cards, round.marker))
            .count() as u8;

        let total_flor_count = round
            .seats
            .iter()
            .enumerate()
            .filter_map(|(_, seat)| flor::Flor::from_cards(&seat.face_up_cards, round.marker))
            .count() as u8;

        // Compute resto
        let max_score = round.game.scoreboard.current_cama_score().max();
        let cama_win_score = round.game.scoreboard.game_config.cama_win_score;
        let resto = cama_win_score - max_score;

        let score = match game_bet.agreed_bet {
            flor::Bet::Announced => winner_flor_count * 3,
            flor::Bet::Envit => total_flor_count * 3,
            flor::Bet::Resto => total_flor_count * 3 + resto,
        };

        Some(scoreboard::RoundScoreSection(winner, score))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use deck;
    use Game;
    use Player;
    use Seat;

    #[test]
    fn test_set_flor_bet() {
        unimplemented!()
    }

    #[test]
    fn test_set_flor_bet_resto_with_winner_fails() {
        unimplemented!()
    }

    fn flor_tests_round_fixture(game: &Game) -> Round {
        let mut round = Round::new(game, &game.players[0], deck::Deck::default());

        round.seats = vec![
            // 34
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Tres,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Cinco,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Seis,
                    },
                ],
            },
            // 35
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Tres,
                    },
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Cinco,
                    },
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Siete,
                    },
                ],
            },
            // 35
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Tres,
                    },
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Cinco,
                    },
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Siete,
                    },
                ],
            },
            // 34
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Tres,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Cinco,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Seis,
                    },
                ],
            },
        ];

        round
    }

    #[test]
    fn get_flor_score_not_announced() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let round = flor_tests_round_fixture(&game);
        let flor_scorer = FlorScorer::default();

        assert!(flor_scorer.get_score(&round).is_none())
    }

    #[test]
    fn get_flor_score_announced_no_flor() {
        unimplemented!();

        // This situation should be impossible! Testing as it can be done in code anyway
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let mut round = Round::new(&game, &game.players[0], deck::Deck::default());

        round.seats = vec![
            // No flor
            Seat {
                player: &game.players[0],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Siete,
                    },
                    deck::Card {
                        suit: deck::Suit::Bastos,
                        value: deck::Value::Sota,
                    },
                    deck::Card {
                        suit: round.marker.suit,
                        value: deck::Value::Caballo,
                    },
                ],
                hand: vec![],
            },
        ];

        let mut flor_scorer = FlorScorer::default();
        flor_scorer.set_bet(flor::Bet::Envit, None);

        assert!(flor_scorer.get_score(&round).is_none())
    }

    #[test]
    fn get_flor_score_announced_won_bet() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let round = flor_tests_round_fixture(&game);

        let mut flor_scorer = FlorScorer::default();
        flor_scorer.set_bet(flor::Bet::Announced, Some(Team::Team2));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 6));
        assert_eq!(flor_scorer.get_score(&round), expected);
    }

    #[test]
    fn get_flor_score_announced_won_from_cards() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let round = flor_tests_round_fixture(&game);

        let mut flor_scorer = FlorScorer::default();
        flor_scorer.set_bet(flor::Bet::Announced, None);

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 6));
        assert_eq!(flor_scorer.get_score(&round), expected);
    }

    #[test]
    fn get_flor_score_envit_won_bet() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let round = flor_tests_round_fixture(&game);

        let mut flor_scorer = FlorScorer::default();
        flor_scorer.set_bet(flor::Bet::Envit, Some(Team::Team2));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 12));
        assert_eq!(flor_scorer.get_score(&round), expected);
    }

    #[test]
    fn get_flor_score_envit_won_from_cards() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let round = flor_tests_round_fixture(&game);

        let mut flor_scorer = FlorScorer::default();
        flor_scorer.set_bet(flor::Bet::Envit, None);

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 12));
        assert_eq!(flor_scorer.get_score(&round), expected);
    }

    #[test]
    fn get_flor_score_resto_won_bet() {
        unimplemented!("This test should not make sense");

        let mut game = Game::new(vec![Player::new("a"), Player::new("b")]);
        game.scoreboard.annotate(scoreboard::RoundScore {
            rey: None,
            flor: None,
            secansa: None,
            ali: None,
            truc: scoreboard::RoundScoreSection(Team::Team1, 25),
        });
        let round = flor_tests_round_fixture(&game);

        let mut flor_scorer = FlorScorer::default();
        flor_scorer.set_bet(flor::Bet::Resto, Some(Team::Team2));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 27));
        assert_eq!(flor_scorer.get_score(&round), expected);
    }

    #[test]
    fn get_flor_score_resto_won_from_cards() {
        let mut game = Game::new(vec![Player::new("a"), Player::new("b")]);
        game.scoreboard.annotate(scoreboard::RoundScore {
            rey: None,
            flor: None,
            secansa: None,
            ali: None,
            truc: scoreboard::RoundScoreSection(Team::Team1, 25),
        });
        let round = flor_tests_round_fixture(&game);

        let mut flor_scorer = FlorScorer::default();
        flor_scorer.set_bet(flor::Bet::Resto, None);

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 27));
        assert_eq!(flor_scorer.get_score(&round), expected);
    }
}
