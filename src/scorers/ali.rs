use super::Scorer;
use hands::{ali, Hand};
use scoreboard;
use Round;
use Team;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AgreedBet {
    Announced(Option<Team>),
    Envit(Option<Team>),
    Val(u8, Option<Team>),
}

#[derive(Default)]
pub struct AliScorer {
    agreed_bet: Option<AgreedBet>,
}

impl AliScorer {
    pub fn set_bet(&mut self, agreed_bet: AgreedBet) {
        self.agreed_bet = Some(agreed_bet);
    }
}

impl Scorer for AliScorer {
    fn get_score(&self, round: &Round) -> Option<scoreboard::RoundScoreSection> {
        // Game must've been announced to be scored
        let game_bet = self.agreed_bet?;

        let bet_winner = match game_bet {
            | AgreedBet::Announced(winner)
            | AgreedBet::Envit(winner)
            | AgreedBet::Val(_, winner) => winner,
        };

        let cards_winner = round.get_winner_from_cards::<ali::Ali>();

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
            .filter_map(|(_, seat)| ali::Ali::from_cards(&seat.face_up_cards, round.marker))
            .map(|ali| ali.score())
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
    fn test_set_ali_bet() {
        let mut secansa_scorer = AliScorer::default();
        let bet = AgreedBet::Envit(Some(Team::Team1));

        assert!(secansa_scorer.agreed_bet.is_none());

        secansa_scorer.set_bet(bet);

        assert!(secansa_scorer.agreed_bet.is_some());

        assert_eq!(secansa_scorer.agreed_bet, Some(bet));
    }

    fn ali_tests_round_fixture(game: &Game) -> Round {
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
                        value: deck::Value::Cinco,
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
                        value: deck::Value::Uno,
                    },
                    deck::Card {
                        suit: deck::Suit::Bastos,
                        value: deck::Value::Uno,
                    },
                    deck::Card {
                        suit: deck::Suit::Espadas,
                        value: deck::Value::Uno,
                    },
                ],
            },
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Uno,
                    },
                    deck::Card {
                        suit: deck::Suit::Espadas,
                        value: deck::Value::Uno,
                    },
                    deck::Card {
                        suit: deck::Suit::Bastos,
                        value: deck::Value::Uno,
                    },
                ],
            },
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Tres,
                    },
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Tres,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Tres,
                    },
                ],
            },
        ];

        round
    }

    #[test]
    fn get_ali_score_not_announced() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let round = ali_tests_round_fixture(&game);
        let ali_scorer = AliScorer::default();

        assert!(ali_scorer.get_score(&round).is_none())
    }

    #[test]
    fn get_ali_score_announced_no_ali() {
        // This situation should be impossible! Testing as it can be done in code anyway
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let mut round = Round::new(&game, &game.players[0], deck::Deck::default());

        let mut ali_scorer = AliScorer::default();
        ali_scorer.set_bet(AgreedBet::Envit(None));

        round.seats = vec![
            // No ali
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

        assert!(ali_scorer.get_score(&round).is_none())
    }

    #[test]
    fn get_ali_score_announced_won_bet() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let round = ali_tests_round_fixture(&game);
        let mut ali_scorer = AliScorer::default();
        ali_scorer.set_bet(AgreedBet::Announced(Some(Team::Team1)));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team1, 7));
        assert_eq!(ali_scorer.get_score(&round), expected);
    }

    #[test]
    fn get_ali_score_announced_won_from_cards() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let round = ali_tests_round_fixture(&game);
        let mut ali_scorer = AliScorer::default();
        ali_scorer.set_bet(AgreedBet::Announced(None));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 9));
        assert_eq!(ali_scorer.get_score(&round), expected);
    }

    #[test]
    fn get_ali_score_envit_won_bet() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let round = ali_tests_round_fixture(&game);
        let mut ali_scorer = AliScorer::default();
        ali_scorer.set_bet(AgreedBet::Envit(Some(Team::Team1)));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team1, 8));
        assert_eq!(ali_scorer.get_score(&round), expected);
    }

    #[test]
    fn get_ali_score_envit_won_from_cards() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let round = ali_tests_round_fixture(&game);
        let mut ali_scorer = AliScorer::default();
        ali_scorer.set_bet(AgreedBet::Envit(None));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 10));
        assert_eq!(ali_scorer.get_score(&round), expected);
    }

    #[test]
    fn get_ali_score_tres_val_won_bet() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let round = ali_tests_round_fixture(&game);
        let mut ali_scorer = AliScorer::default();
        ali_scorer.set_bet(AgreedBet::Val(3, Some(Team::Team1)));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team1, 9));
        assert_eq!(ali_scorer.get_score(&round), expected);
    }

    #[test]
    fn get_ali_score_tres_val_won_from_cards() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let round = ali_tests_round_fixture(&game);
        let mut ali_scorer = AliScorer::default();
        ali_scorer.set_bet(AgreedBet::Val(3, None));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 11));
        assert_eq!(ali_scorer.get_score(&round), expected);
    }
}
