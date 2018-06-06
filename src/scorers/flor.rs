use super::Scorer;
use hands::{flor, Hand};
use scoreboard;
use Round;
use Team;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AgreedBet {
    // Announced and Envit can be agreed but we gotta check the cards to get the winner
    Announced(Option<Team>),
    Envit(Option<Team>),
    // If Resto is agreed we gotta check the cards to get the winner
    Resto,
}

pub struct FlorScorer {
    agreed_bet: Option<AgreedBet>,
}

impl FlorScorer {
    pub fn set_bet(&mut self, agreed_bet: AgreedBet) {
        self.agreed_bet = Some(agreed_bet);
    }
}

impl Default for FlorScorer {
    fn default() -> Self {
        FlorScorer { agreed_bet: None }
    }
}

impl Scorer for FlorScorer {
    fn get_score(&self, round: &Round) -> Option<scoreboard::RoundScoreSection> {
        // Game must've been announced to be scored
        let game_bet = self.agreed_bet?;

        let bet_winner = match game_bet {
            AgreedBet::Announced(winner) | AgreedBet::Envit(winner) => winner,
            _ => None,
        };

        let cards_winner = round.get_winner_from_cards::<flor::Flor>();

        let winner = match (bet_winner, cards_winner) {
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

        let score = match game_bet {
            AgreedBet::Announced(_) => winner_flor_count * 3,
            AgreedBet::Envit(_) => total_flor_count * 3,
            AgreedBet::Resto => total_flor_count * 3 + resto,
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
        let mut flor_scorer = FlorScorer::default();
        let bet = AgreedBet::Envit(Some(Team::Team1));

        assert!(flor_scorer.agreed_bet.is_none());

        flor_scorer.set_bet(bet);

        assert!(flor_scorer.agreed_bet.is_some());

        assert_eq!(flor_scorer.agreed_bet, Some(bet));
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
        flor_scorer.set_bet(AgreedBet::Envit(None));

        assert!(flor_scorer.get_score(&round).is_none())
    }

    #[test]
    fn get_flor_score_announced_won_bet() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let round = flor_tests_round_fixture(&game);

        let mut flor_scorer = FlorScorer::default();
        flor_scorer.set_bet(AgreedBet::Announced(Some(Team::Team2)));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 6));
        assert_eq!(flor_scorer.get_score(&round), expected);
    }

    #[test]
    fn get_flor_score_announced_won_from_cards() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let round = flor_tests_round_fixture(&game);

        let mut flor_scorer = FlorScorer::default();
        flor_scorer.set_bet(AgreedBet::Announced(None));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 6));
        assert_eq!(flor_scorer.get_score(&round), expected);
    }

    #[test]
    fn get_flor_score_envit_won_bet() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let round = flor_tests_round_fixture(&game);

        let mut flor_scorer = FlorScorer::default();
        flor_scorer.set_bet(AgreedBet::Envit(Some(Team::Team2)));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 12));
        assert_eq!(flor_scorer.get_score(&round), expected);
    }

    #[test]
    fn get_flor_score_envit_won_from_cards() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let round = flor_tests_round_fixture(&game);

        let mut flor_scorer = FlorScorer::default();
        flor_scorer.set_bet(AgreedBet::Envit(None));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 12));
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
        flor_scorer.set_bet(AgreedBet::Resto);

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 27));
        assert_eq!(flor_scorer.get_score(&round), expected);
    }
}
