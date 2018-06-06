use super::Scorer;
use deck;
use scoreboard;
use Round;

#[derive(Default)]
pub struct ReyScorer;

impl Scorer for ReyScorer {
    fn get_score(&self, round: &Round) -> Option<scoreboard::RoundScoreSection> {
        let winner_team = round
            .iter_from_hand()
            .skip_while(|&(_team, seat)| {
                !seat.face_up_cards
                    .iter()
                    .map(|card| card.value)
                    .any(|value| value == deck::Value::Rey)
            })
            .nth(0)?
            .0;

        let rey_count = round
            .iter_from_hand()
            .filter(|&(team, _seat)| team == winner_team)
            .flat_map(|(_team, seat)| &seat.face_up_cards)
            .filter(|&card| card.value == deck::Value::Rey)
            .count() as u8;

        Some(scoreboard::RoundScoreSection(winner_team, rey_count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Game;
    use Player;
    use Seat;
    use Team;

    #[test]
    fn get_rey_score_no_reyes() {
        let game = Game::new(vec![Player::new("a")]);
        let mut round = Round::new(&game, &game.players[0], deck::Deck::default());

        round.seats = vec![Seat {
            player: &game.players[0],
            hand: vec![],
            face_up_cards: vec![
                deck::Card {
                    suit: deck::Suit::Oros,
                    value: deck::Value::Cinco,
                },
                deck::Card {
                    suit: deck::Suit::Copas,
                    value: deck::Value::Cinco,
                },
                deck::Card {
                    suit: deck::Suit::Copas,
                    value: deck::Value::Sota,
                },
            ],
        }];

        let result = ReyScorer::default().get_score(&round);
        let expected = None;

        assert_eq!(result, expected);
    }

    #[test]
    fn get_rey_score_one_rey() {
        let game = Game::new(vec![Player::new("a")]);
        let mut round = Round::new(&game, &game.players[0], deck::Deck::default());

        round.seats = vec![Seat {
            player: &game.players[0],
            hand: vec![],
            face_up_cards: vec![
                deck::Card {
                    suit: deck::Suit::Oros,
                    value: deck::Value::Rey,
                },
                deck::Card {
                    suit: deck::Suit::Copas,
                    value: deck::Value::Cinco,
                },
                deck::Card {
                    suit: deck::Suit::Copas,
                    value: deck::Value::Sota,
                },
            ],
        }];

        let result = ReyScorer::default().get_score(&round);
        let expected = Some(scoreboard::RoundScoreSection(Team::Team1, 1));

        assert_eq!(result, expected);
    }

    #[test]
    fn get_rey_score_tres_rey() {
        let game = Game::new(vec![Player::new("a")]);
        let mut round = Round::new(&game, &game.players[0], deck::Deck::default());

        round.seats = vec![Seat {
            player: &game.players[0],
            hand: vec![],
            face_up_cards: vec![
                deck::Card {
                    suit: deck::Suit::Oros,
                    value: deck::Value::Rey,
                },
                deck::Card {
                    suit: deck::Suit::Copas,
                    value: deck::Value::Rey,
                },
                deck::Card {
                    suit: deck::Suit::Copas,
                    value: deck::Value::Rey,
                },
            ],
        }];

        let result = ReyScorer::default().get_score(&round);
        let expected = Some(scoreboard::RoundScoreSection(Team::Team1, 3));

        assert_eq!(result, expected);
    }

    #[test]
    fn get_rey_score_many_in_team() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let mut round = Round::new(&game, &game.players[0], deck::Deck::default());

        round.seats = vec![
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Rey,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Rey,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
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
                        value: deck::Value::Rey,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Dos,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Cinco,
                    },
                ],
            },
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Rey,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Sota,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Caballo,
                    },
                ],
            },
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Rey,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Uno,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Cinco,
                    },
                ],
            },
        ];

        let result = ReyScorer::default().get_score(&round);
        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 2));

        assert_eq!(result, expected);
    }

    #[test]
    fn get_rey_score_many_in_team_at_hand() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let mut round = Round::new(&game, &game.players[1], deck::Deck::default());

        round.seats = vec![
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Rey,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Rey,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Rey,
                    },
                ],
            },
            Seat {
                player: &game.players[1],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Rey,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Cuatro,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Cinco,
                    },
                ],
            },
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Rey,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Sota,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Caballo,
                    },
                ],
            },
        ];

        let result = ReyScorer::default().get_score(&round);
        let expected = Some(scoreboard::RoundScoreSection(Team::Team1, 4));

        assert_eq!(result, expected);
    }

}
