use super::Scorer;
use hands::{secansa, Hand};
use scoreboard;
use Round;
use Team;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AgreedBet {
    Announced(Option<Team>),
    Envit(Option<Team>),
    Val(u8, Option<Team>),
}

pub struct SecansaScorer {
    agreed_bet: Option<AgreedBet>,
}

impl SecansaScorer {
    pub fn set_bet(&mut self, agreed_bet: AgreedBet) {
        self.agreed_bet = Some(agreed_bet);
    }
}

impl Default for SecansaScorer {
    fn default() -> Self {
        SecansaScorer { agreed_bet: None }
    }
}

impl Scorer for SecansaScorer {
    fn get_score(&self, round: &Round) -> Option<scoreboard::RoundScoreSection> {
        // Game must've been announced to be scored
        let game_bet = self.agreed_bet?;

        let bet_winner = match game_bet {
            | AgreedBet::Announced(winner)
            | AgreedBet::Envit(winner)
            | AgreedBet::Val(_, winner) => winner,
        };

        let cards_winner = round.get_winner_from_cards::<secansa::Secansa>();

        let winner = match (bet_winner, cards_winner) {
            // If there was a rejected bet, we've got a direct winner
            (Some(winner), _) => winner,
            // Otherwise get the winner from the facing up cards
            (None, Some(winner)) => winner,
            // If there still no winner, we can't score anything
            _ => return None,
        };

        let games_value: u8 = round
            .seats
            .iter()
            .enumerate()
            .filter(|&(pos, seat)| seat.get_team(pos as u8) == winner)
            .filter_map(|(_, seat)| secansa::Secansa::from_cards(&seat.face_up_cards, round.marker))
            .map(|secansa| secansa.score())
            .sum();

        let extra = match game_bet {
            AgreedBet::Envit(_) => 1,
            AgreedBet::Val(extra, _) => extra - 1, // e.g.: tres val gives 2 points
            _ => 0,
        };

        let total = games_value + extra;

        Some(scoreboard::RoundScoreSection(winner, total))
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
    fn test_set_secansa_bet() {
        let mut secansa_scorer = SecansaScorer::default();
        let bet = AgreedBet::Envit(Some(Team::Team1));

        assert!(secansa_scorer.agreed_bet.is_none());

        secansa_scorer.set_bet(bet);

        assert!(secansa_scorer.agreed_bet.is_some());

        assert_eq!(secansa_scorer.agreed_bet, Some(bet));
    }

    fn secansa_tests_round_fixture(game: &Game) -> Round {
        let mut round = Round::new(game, &game.players[0], deck::Deck::default());

        round.seats = vec![
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Cinco,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Seis,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Sota,
                    },
                ],
            },
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Sota,
                    },
                    deck::Card {
                        suit: deck::Suit::Bastos,
                        value: deck::Value::Caballo,
                    },
                    deck::Card {
                        suit: deck::Suit::Espadas,
                        value: deck::Value::Rey,
                    },
                ],
            },
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Sota,
                    },
                    deck::Card {
                        suit: deck::Suit::Espadas,
                        value: deck::Value::Caballo,
                    },
                    deck::Card {
                        suit: deck::Suit::Bastos,
                        value: deck::Value::Rey,
                    },
                ],
            },
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Cinco,
                    },
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Seis,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Siete,
                    },
                ],
            },
        ];

        round
    }

    #[test]
    fn get_secansa_score_not_announced() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let round = secansa_tests_round_fixture(&game);
        let secansa_scorer = SecansaScorer::default();

        assert!(secansa_scorer.get_score(&round).is_none())
    }

    #[test]
    fn get_secansa_score_announced_no_secansa() {
        // This situation should be impossible! Testing as it can be done in code anyway
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let mut round = Round::new(&game, &game.players[0], deck::Deck::default());

        let mut secansa_scorer = SecansaScorer::default();
        secansa_scorer.set_bet(AgreedBet::Envit(None));

        round.seats = vec![
            // No secansa
            Seat {
                player: &game.players[0],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Siete,
                    },
                    deck::Card {
                        suit: deck::Suit::Bastos,
                        value: deck::Value::Tres,
                    },
                    deck::Card {
                        suit: round.marker.suit,
                        value: deck::Value::Caballo,
                    },
                ],
                hand: vec![],
            },
        ];

        assert!(secansa_scorer.get_score(&round).is_none())
    }

    #[test]
    fn get_secansa_score_announced_won_bet() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let round = secansa_tests_round_fixture(&game);

        let mut secansa_scorer = SecansaScorer::default();
        secansa_scorer.set_bet(AgreedBet::Announced(Some(Team::Team1)));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team1, 4));
        assert_eq!(secansa_scorer.get_score(&round), expected);
    }

    #[test]
    fn get_secansa_score_announced_won_from_cards() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let round = secansa_tests_round_fixture(&game);

        let mut secansa_scorer = SecansaScorer::default();
        secansa_scorer.set_bet(AgreedBet::Announced(None));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 6));
        assert_eq!(secansa_scorer.get_score(&round), expected);
    }

    #[test]
    fn get_secansa_score_envit_won_bet() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let round = secansa_tests_round_fixture(&game);

        let mut secansa_scorer = SecansaScorer::default();
        secansa_scorer.set_bet(AgreedBet::Envit(Some(Team::Team1)));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team1, 5));
        assert_eq!(secansa_scorer.get_score(&round), expected);
    }

    #[test]
    fn get_secansa_score_envit_won_from_cards() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let round = secansa_tests_round_fixture(&game);

        let mut secansa_scorer = SecansaScorer::default();
        secansa_scorer.set_bet(AgreedBet::Envit(None));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 7));
        assert_eq!(secansa_scorer.get_score(&round), expected);
    }

    #[test]
    fn get_secansa_score_tres_val_won_bet() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let round = secansa_tests_round_fixture(&game);

        let mut secansa_scorer = SecansaScorer::default();
        secansa_scorer.set_bet(AgreedBet::Val(3, Some(Team::Team1)));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team1, 6));
        assert_eq!(secansa_scorer.get_score(&round), expected);
    }

    #[test]
    fn get_secansa_score_tres_val_won_from_cards() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let round = secansa_tests_round_fixture(&game);

        let mut secansa_scorer = SecansaScorer::default();
        secansa_scorer.set_bet(AgreedBet::Val(3, None));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 8));
        assert_eq!(secansa_scorer.get_score(&round), expected);
    }
}
