#[macro_use]
extern crate itertools;
extern crate rand;

mod deck;
mod hands;
pub mod scoreboard;

use hands::{ali, flor, secansa, Hand};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Team {
    Team1,
    Team2,
}

#[derive(Debug, Eq, PartialEq)]
struct Player {
    name: String,
}

impl Player {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}

#[derive(Clone)]
struct Seat<'a> {
    player: &'a Player,
    hand: Vec<deck::Card>,
    face_up_cards: Vec<deck::Card>,
}

impl<'a> Seat<'a> {
    fn new(player: &'a Player) -> Self {
        Self {
            player,
            hand: Vec::default(),
            face_up_cards: Vec::default(),
        }
    }

    fn get_team(&self, seat_number: u8) -> Team {
        if seat_number % 2 == 0 {
            Team::Team1
        } else {
            Team::Team2
        }
    }

    fn discard(&mut self, card: deck::Card) -> Option<deck::Card> {
        if let Some(pos) = self.hand.iter().position(|x| *x == card) {
            Some(self.hand.remove(pos))
        } else {
            None
        }
    }

    fn show_card(&mut self, card: deck::Card) -> Option<deck::Card> {
        if self.discard(card).is_some() {
            self.face_up_cards.push(card);
            Some(card)
        } else {
            None
        }
    }
}

struct Game {
    players: Vec<Player>,
    scoreboard: scoreboard::Scoreboard,
}

impl Game {
    fn new(players: Vec<Player>) -> Self {
        Game {
            players,
            scoreboard: Default::default(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct GameBet<T> {
    winner: Option<Team>,
    agreed_bet: T,
}

struct Round<'a> {
    game: &'a Game,
    seats: Vec<Seat<'a>>,
    dealer: &'a Player,
    deck: deck::Deck,
    marker: deck::Card,
    flor_bet: Option<GameBet<flor::Bet>>,
    secansa_bet: Option<GameBet<secansa::Bet>>,
    ali_bet: Option<GameBet<ali::Bet>>,
}

impl<'a> Round<'a> {
    fn new(game: &'a Game, dealer: &'a Player, mut deck: deck::Deck) -> Self {
        let seats = game.players.iter().map(Seat::new).collect();
        Self {
            game,
            seats,
            dealer,
            marker: deck.draw().unwrap(),
            deck,
            flor_bet: None,
            secansa_bet: None,
            ali_bet: None,
        }
    }

    fn dealer_position(&self) -> usize {
        self.seats
            .iter()
            .position(|seat| seat.player == self.dealer)
            .expect("Round not properly set up: dealer is not seated")
    }

    fn deal(&mut self, num_cards: usize) {
        for _ in 0..num_cards {
            for seat in &mut self.seats {
                if let Some(card) = self.deck.draw() {
                    seat.hand.push(card);
                }
            }
        }
    }

    fn is_finished(&self) -> bool {
        self.seats.iter().all(|seat| seat.hand.is_empty())
    }

    // fn compute_game_winners(&mut self) -> Result<(), RoundNotFinishedError> {}

    fn set_flor_bet(&mut self, agreed_bet: flor::Bet, winner: Option<Team>) {
        self.flor_bet = Some(GameBet { agreed_bet, winner })
    }

    fn set_secansa_bet(&mut self, agreed_bet: secansa::Bet, winner: Option<Team>) {
        self.secansa_bet = Some(GameBet { agreed_bet, winner })
    }

    fn set_ali_bet(&mut self, agreed_bet: ali::Bet, winner: Option<Team>) {
        self.ali_bet = Some(GameBet { agreed_bet, winner })
    }

    fn get_round_score(&self) -> scoreboard::RoundScore {
        scoreboard::RoundScore {
            flor: self.get_flor_score(),
            secansa: self.get_secansa_score(),
            ali: self.get_ali_score(),
            rey: None,
            truc: scoreboard::RoundScoreSection(Team::Team1, 0),
        }
    }

    fn iter_from_hand(&self) -> impl Iterator<Item = (Team, &Seat)> {
        self.seats
            .iter()

            // Enumerate each seat's position
            .enumerate()

            // Set the first item to the hand, that is, the seat after the dealer (+1)
            .cycle()
            .skip(self.dealer_position() + 1)
            .take(self.seats.len())

            .map(|(pos, seat)| {
                let team = seat.get_team(pos as u8);
                (team, seat)
            })
    }

    fn get_winner_from_cards<'b, H>(&'b self) -> Option<Team>
    where
        H: Hand<'b>,
    {
        // Remove any seats without the hand
        let mut hands_by_team = self.iter_from_hand()
            .filter_map(|(team, seat)| {
                let face_up_cards = &seat.face_up_cards;
                let hand = H::from_cards(face_up_cards, self.marker);
                match hand {
                    Some(hand) => Some((team, hand)),
                    None => None,
                }
            })
            .collect::<Vec<_>>();

        // Iterator::max_by returns the last element that matches. We need the first.
        hands_by_team.reverse();

        hands_by_team
            .into_iter()
            // https://stackoverflow.com/a/49312019/2221217
            .max_by(|&(_, ref hand1), &(_, ref hand2)| hand1.cmp(hand2))
            .map(|(team, _)| team)
    }

    fn get_flor_score(&self) -> Option<scoreboard::RoundScoreSection> {
        // Game must've been announced to be scored
        let game_bet = self.flor_bet?;

        let cards_winner = self.get_winner_from_cards::<flor::Flor>();

        let winner = match (game_bet.winner, cards_winner) {
            // If there was a rejected bet, we've got a direct winner
            (Some(winner), _) => winner,
            // Otherwise get the winner from the facing up cards
            (None, Some(winner)) => winner,
            // If there still no winner, we can't score anything
            _ => return None,
        };

        let winner_flor_count = self.seats
            .iter()
            .enumerate()
            .filter(|&(pos, seat)| seat.get_team(pos as u8) == winner)
            .filter_map(|(_, seat)| flor::Flor::from_cards(&seat.face_up_cards, self.marker))
            .count() as u8;

        let total_flor_count = self.seats
            .iter()
            .enumerate()
            .filter_map(|(_, seat)| flor::Flor::from_cards(&seat.face_up_cards, self.marker))
            .count() as u8;

        // Compute resto
        let max_score = self.game.scoreboard.current_cama_score().max();
        let cama_win_score = self.game.scoreboard.game_config.cama_win_score;
        let resto = cama_win_score - max_score;

        let score = match game_bet.agreed_bet {
            flor::Bet::Announced => winner_flor_count * 3,
            flor::Bet::Envit => total_flor_count * 3,
            flor::Bet::Resto => total_flor_count * 3 + resto,
        };

        Some(scoreboard::RoundScoreSection(winner, score))
    }

    fn get_secansa_score(&self) -> Option<scoreboard::RoundScoreSection> {
        // Game must've been announced to be scored
        let game_bet = self.secansa_bet?;

        let cards_winner = self.get_winner_from_cards::<secansa::Secansa>();

        let winner = match (game_bet.winner, cards_winner) {
            // If there was a rejected bet, we've got a direct winner
            (Some(winner), _) => winner,
            // Otherwise get the winner from the facing up cards
            (None, Some(winner)) => winner,
            // If there still no winner, we can't score anything
            _ => return None,
        };

        let games_value: u8 = self.seats
            .iter()
            .enumerate()
            .filter(|&(pos, seat)| seat.get_team(pos as u8) == winner)
            .filter_map(|(_, seat)| secansa::Secansa::from_cards_slice(&seat.face_up_cards))
            .map(|secansa| secansa.score())
            .sum();

        let extra = match game_bet.agreed_bet {
            secansa::Bet::Envit => 1,
            secansa::Bet::Val(extra) => extra - 1, // e.g.: tres val gives 2 points
            _ => 0,
        };

        let total = games_value + extra;

        Some(scoreboard::RoundScoreSection(winner, total))
    }

    fn get_ali_score(&self) -> Option<scoreboard::RoundScoreSection> {
        // Game must've been announced to be scored
        let game_bet = self.ali_bet?;

        let cards_winner = self.get_winner_from_cards::<ali::Ali>();

        let winner = match (game_bet.winner, cards_winner) {
            // If there was a rejected bet, we've got a direct winner
            (Some(winner), _) => winner,
            // Otherwise get the winner from the facing up cards
            (None, Some(winner)) => winner,
            // If there still no winner, we can't score anything
            _ => return None,
        };

        let games_value: u8 = self.seats
            .iter()
            .enumerate()
            .filter(|&(pos, seat)| seat.get_team(pos as u8) == winner)
            .filter_map(|(_, seat)| ali::Ali::from_cards_slice(&seat.face_up_cards))
            .map(|ali| ali.score())
            .sum();

        let extra = match game_bet.agreed_bet {
            ali::Bet::Envit => 1,
            ali::Bet::Val(extra) => extra - 1, // e.g.: tres val gives 2 points
            _ => 0,
        };

        let total = games_value + extra;

        Some(scoreboard::RoundScoreSection(winner, total))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seat_get_team() {
        let player = Player::new("a");
        let seat = Seat::new(&player);
        assert_eq!(seat.get_team(0), Team::Team1);
        assert_eq!(seat.get_team(1), Team::Team2);
        assert_eq!(seat.get_team(2), Team::Team1);
        assert_eq!(seat.get_team(3), Team::Team2);
        assert_eq!(seat.get_team(4), Team::Team1);
        assert_eq!(seat.get_team(5), Team::Team2);
    }

    #[test]
    fn round_new() {
        let game = Game::new(vec![
            Player::new("a"),
            Player::new("b"),
            Player::new("c"),
            Player::new("d"),
            Player::new("e"),
            Player::new("f"),
        ]);
        let mut round = Round::new(&game, &game.players[0], deck::Deck::default());
        assert_eq!(round.deck.remaining_cards(), 39);
        let mut cards = Vec::default();
        while let Some(card) = round.deck.draw() {
            cards.push(card);
        }
        assert!(!cards.contains(&round.marker));
    }

    #[test]
    fn dealer_position() {
        let game = Game::new(vec![
            Player::new("a"),
            Player::new("b"),
            Player::new("c"),
            Player::new("d"),
            Player::new("e"),
            Player::new("f"),
        ]);
        let deck = deck::Deck::default();
        for i in 0..6 {
            let round = Round::new(&game, &game.players[i], deck.clone());
            assert_eq!(round.dealer_position(), i);
        }
    }

    #[test]
    fn round_deal() {
        let game = Game::new(vec![
            Player::new("a"),
            Player::new("b"),
            Player::new("c"),
            Player::new("d"),
            Player::new("e"),
            Player::new("f"),
        ]);
        let mut round = Round::new(&game, &game.players[0], deck::Deck::default());
        for i in 0..6 {
            assert_eq!(round.seats[i].hand.len(), 0);
            assert_eq!(round.seats[i].face_up_cards.len(), 0);
        }
        const NUM_CARDS: usize = 6;
        round.deal(NUM_CARDS);
        for i in 0..6 {
            assert_eq!(round.seats[i].hand.len(), NUM_CARDS);
            assert_eq!(round.seats[i].face_up_cards.len(), 0);
        }
    }

    #[test]
    fn discard_bad_card() {
        let mut seat = Seat {
            player: &Player::new("a"),
            hand: vec![deck::Card {
                suit: deck::Suit::Bastos,
                value: deck::Value::Caballo,
            }],
            face_up_cards: Vec::new(),
        };
        let result = seat.discard(deck::Card {
            suit: deck::Suit::Bastos,
            value: deck::Value::Cinco,
        });
        assert!(result.is_none());
    }

    #[test]
    fn discard_ok() {
        let card = deck::Card {
            suit: deck::Suit::Bastos,
            value: deck::Value::Caballo,
        };
        let mut seat = Seat {
            player: &Player::new("a"),
            hand: vec![card],
            face_up_cards: Vec::new(),
        };
        let result = seat.discard(deck::Card {
            suit: deck::Suit::Bastos,
            value: deck::Value::Caballo,
        });
        assert!(result.is_some());
        assert_eq!(result.unwrap(), card);
    }

    #[test]
    fn show_bad_card() {
        let mut seat = Seat {
            player: &Player::new("a"),
            hand: vec![deck::Card {
                suit: deck::Suit::Bastos,
                value: deck::Value::Caballo,
            }],
            face_up_cards: Vec::new(),
        };
        let result = seat.show_card(deck::Card {
            suit: deck::Suit::Bastos,
            value: deck::Value::Cinco,
        });
        assert!(result.is_none());
        assert!(seat.face_up_cards.is_empty());
    }

    #[test]
    fn show_card_ok() {
        let card = deck::Card {
            suit: deck::Suit::Bastos,
            value: deck::Value::Caballo,
        };
        let mut seat = Seat {
            player: &Player::new("a"),
            hand: vec![card],
            face_up_cards: Vec::new(),
        };
        let result = seat.show_card(deck::Card {
            suit: deck::Suit::Bastos,
            value: deck::Value::Caballo,
        });
        assert!(result.is_some());
        assert_eq!(result.unwrap(), card);
        assert_eq!(seat.face_up_cards.len(), 1);
        assert!(seat.face_up_cards.contains(&card));
    }

    #[test]
    fn is_round_finished() {
        let game = Game::new(vec![
            Player::new("a"),
            Player::new("b"),
            Player::new("c"),
            Player::new("d"),
            Player::new("e"),
            Player::new("f"),
        ]);
        let mut round = Round::new(&game, &game.players[0], deck::Deck::default());
        round.seats[0].hand.push(deck::Card {
            suit: deck::Suit::Bastos,
            value: deck::Value::Caballo,
        });
        assert!(!round.is_finished());
        round.seats[0].hand.clear();
        assert!(round.is_finished());
    }

    #[test]
    fn test_set_flor_bet() {
        let game = Game::new(vec![Player::new("a")]);
        let mut round = Round::new(&game, &game.players[0], deck::Deck::default());

        assert_eq!(round.flor_bet, None);

        round.set_flor_bet(flor::Bet::Envit, None);
        let expected = Some(GameBet {
            winner: None,
            agreed_bet: flor::Bet::Envit,
        });
        assert_eq!(round.flor_bet, expected);
    }

    #[test]
    fn test_set_secansa_bet() {
        let game = Game::new(vec![Player::new("a")]);
        let mut round = Round::new(&game, &game.players[0], deck::Deck::default());

        assert_eq!(round.secansa_bet, None);

        round.set_secansa_bet(secansa::Bet::Val(3), None);
        let expected = Some(GameBet {
            winner: None,
            agreed_bet: secansa::Bet::Val(3),
        });
        assert_eq!(round.secansa_bet, expected);
    }

    #[test]
    fn test_set_ali_bet() {
        let game = Game::new(vec![Player::new("a")]);
        let mut round = Round::new(&game, &game.players[0], deck::Deck::default());

        assert_eq!(round.ali_bet, None);

        round.set_ali_bet(ali::Bet::Announced, None);
        let expected = Some(GameBet {
            winner: None,
            agreed_bet: ali::Bet::Announced,
        });
        assert_eq!(round.ali_bet, expected);
    }

    // Flor bets

    #[test]
    fn get_flor_winner_from_cards_no_flor() {
        let game = Game::new(vec![Player::new("a")]);
        let mut round = Round::new(&game, &game.players[0], deck::Deck::default());

        round.marker = deck::Card {
            suit: deck::Suit::Oros,
            value: deck::Value::Uno,
        };

        round.seats = vec![
            // No flor
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Uno,
                    },
                    deck::Card {
                        suit: deck::Suit::Bastos,
                        value: deck::Value::Dos,
                    },
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Tres,
                    },
                ],
            },
        ];

        assert!(round.get_winner_from_cards::<flor::Flor>().is_none());
    }

    #[test]
    fn get_flor_winner_from_cards() {
        let game = Game::new(vec![Player::new("a")]);
        let mut round = Round::new(&game, &game.players[0], deck::Deck::default());

        round.marker = deck::Card {
            suit: deck::Suit::Oros,
            value: deck::Value::Uno,
        };

        round.seats = vec![
            // No flor
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Uno,
                    },
                    deck::Card {
                        suit: deck::Suit::Bastos,
                        value: deck::Value::Dos,
                    },
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Tres,
                    },
                ],
            },
            // Low Flor
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Uno,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Dos,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Tres,
                    },
                ],
            },
            // Medium Flor
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Cinco,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Seis,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Siete,
                    },
                ],
            },
            // High Flor
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Siete,
                    },
                    deck::Card {
                        suit: round.marker.suit,
                        value: deck::Value::Sota,
                    },
                    deck::Card {
                        suit: round.marker.suit,
                        value: deck::Value::Caballo,
                    },
                ],
            },
        ];

        let expected = Some(Team::Team2);
        assert_eq!(round.get_winner_from_cards::<flor::Flor>(), expected);
    }

    #[test]
    fn get_flor_winner_from_cards_tie_wins_hand() {
        let game = Game::new(vec![
            Player::new("a"),
            Player::new("b"),
            Player::new("c"),
            Player::new("d"),
        ]);
        let mut round = Round::new(&game, &game.players[1], deck::Deck::default());

        round.seats = vec![
            // No flor
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Uno,
                    },
                    deck::Card {
                        suit: deck::Suit::Bastos,
                        value: deck::Value::Dos,
                    },
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Tres,
                    },
                ],
            },
            // 35
            Seat {
                player: &game.players[1],
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
                player: &game.players[2],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Cuatro,
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
                player: &game.players[3],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Bastos,
                        value: deck::Value::Tres,
                    },
                    deck::Card {
                        suit: deck::Suit::Bastos,
                        value: deck::Value::Cinco,
                    },
                    deck::Card {
                        suit: deck::Suit::Bastos,
                        value: deck::Value::Siete,
                    },
                ],
            },
        ];

        let expected = Some(Team::Team1);
        assert_eq!(round.get_winner_from_cards::<flor::Flor>(), expected);
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

        assert!(round.get_flor_score().is_none())
    }

    #[test]
    fn get_flor_score_announced_no_flor() {
        // This situation should be impossible! Testing as it can be done in code anyway
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let mut round = Round::new(&game, &game.players[0], deck::Deck::default());
        round.set_flor_bet(flor::Bet::Envit, None);

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

        assert!(round.get_flor_score().is_none())
    }

    #[test]
    fn get_flor_score_announced_won_bet() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let mut round = flor_tests_round_fixture(&game);
        round.set_flor_bet(flor::Bet::Announced, Some(Team::Team2));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 6));
        assert_eq!(round.get_flor_score(), expected);
    }

    #[test]
    fn get_flor_score_announced_won_from_cards() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let mut round = flor_tests_round_fixture(&game);
        round.set_flor_bet(flor::Bet::Announced, None);

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 6));
        assert_eq!(round.get_flor_score(), expected);
    }

    #[test]
    fn get_flor_score_envit_won_bet() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let mut round = flor_tests_round_fixture(&game);
        round.set_flor_bet(flor::Bet::Envit, Some(Team::Team2));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 12));
        assert_eq!(round.get_flor_score(), expected);
    }

    #[test]
    fn get_flor_score_envit_won_from_cards() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let mut round = flor_tests_round_fixture(&game);
        round.set_flor_bet(flor::Bet::Envit, None);

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 12));
        assert_eq!(round.get_flor_score(), expected);
    }

    #[test]
    fn get_flor_score_resto_won_bet() {
        let mut game = Game::new(vec![Player::new("a"), Player::new("b")]);
        game.scoreboard.annotate(scoreboard::RoundScore {
            rey: None,
            flor: None,
            secansa: None,
            ali: None,
            truc: scoreboard::RoundScoreSection(Team::Team1, 25),
        });
        let mut round = flor_tests_round_fixture(&game);
        round.set_flor_bet(flor::Bet::Resto, Some(Team::Team2));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 27));
        assert_eq!(round.get_flor_score(), expected);
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
        let mut round = flor_tests_round_fixture(&game);
        round.set_flor_bet(flor::Bet::Resto, None);

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 27));
        assert_eq!(round.get_flor_score(), expected);
    }

    // Secansa bets

    #[test]
    fn get_secansa_winner_from_cards_no_secansa() {
        let game = Game::new(vec![Player::new("a")]);
        let mut round = Round::new(&game, &game.players[0], deck::Deck::default());

        round.seats = vec![
            // No secansa
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Uno,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Tres,
                    },
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Cinco,
                    },
                ],
            },
        ];

        assert!(round.get_winner_from_cards::<secansa::Secansa>().is_none());
    }

    #[test]
    fn get_secansa_winner_from_cards() {
        let game = Game::new(vec![Player::new("a")]);
        let mut round = Round::new(&game, &game.players[0], deck::Deck::default());

        round.seats = vec![
            // No secansa
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Uno,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Tres,
                    },
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Cinco,
                    },
                ],
            },
            // Two card secansa
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
                        value: deck::Value::Dos,
                    },
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Cinco,
                    },
                ],
            },
            // Secansa real
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Sota,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Caballo,
                    },
                    deck::Card {
                        suit: deck::Suit::Espadas,
                        value: deck::Value::Rey,
                    },
                ],
            },
            // Three card secansa
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
                        value: deck::Value::Dos,
                    },
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Tres,
                    },
                ],
            },
        ];

        let expected = Some(Team::Team1);
        assert_eq!(round.get_winner_from_cards::<secansa::Secansa>(), expected);
    }

    #[test]
    fn get_secansa_winner_from_cards_tie_wins_hand() {
        let game = Game::new(vec![
            Player::new("a"),
            Player::new("b"),
            Player::new("c"),
            Player::new("d"),
        ]);
        let mut round = Round::new(&game, &game.players[1], deck::Deck::default());

        round.seats = vec![
            // No secansa
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
                        value: deck::Value::Tres,
                    },
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Cinco,
                    },
                ],
            },
            // Secansa real
            Seat {
                player: &game.players[1],
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
                        suit: deck::Suit::Copas,
                        value: deck::Value::Rey,
                    },
                ],
            },
            // Secansa real
            Seat {
                player: &game.players[2],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Sota,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Caballo,
                    },
                    deck::Card {
                        suit: deck::Suit::Bastos,
                        value: deck::Value::Rey,
                    },
                ],
            },
            // Secansa real
            Seat {
                player: &game.players[3],
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
                        suit: deck::Suit::Copas,
                        value: deck::Value::Rey,
                    },
                ],
            },
        ];

        let expected = Some(Team::Team1);
        assert_eq!(round.get_winner_from_cards::<secansa::Secansa>(), expected);
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

        assert!(round.get_secansa_score().is_none())
    }

    #[test]
    fn get_secansa_score_announced_no_secansa() {
        // This situation should be impossible! Testing as it can be done in code anyway
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let mut round = Round::new(&game, &game.players[0], deck::Deck::default());
        round.set_secansa_bet(secansa::Bet::Envit, None);

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

        assert!(round.get_secansa_score().is_none())
    }

    #[test]
    fn get_secansa_score_announced_won_bet() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let mut round = secansa_tests_round_fixture(&game);
        round.set_secansa_bet(secansa::Bet::Announced, Some(Team::Team1));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team1, 4));
        assert_eq!(round.get_secansa_score(), expected);
    }

    #[test]
    fn get_secansa_score_announced_won_from_cards() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let mut round = secansa_tests_round_fixture(&game);
        round.set_secansa_bet(secansa::Bet::Announced, None);

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 6));
        assert_eq!(round.get_secansa_score(), expected);
    }

    #[test]
    fn get_secansa_score_envit_won_bet() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let mut round = secansa_tests_round_fixture(&game);
        round.set_secansa_bet(secansa::Bet::Envit, Some(Team::Team1));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team1, 5));
        assert_eq!(round.get_secansa_score(), expected);
    }

    #[test]
    fn get_secansa_score_envit_won_from_cards() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let mut round = secansa_tests_round_fixture(&game);
        round.set_secansa_bet(secansa::Bet::Envit, None);

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 7));
        assert_eq!(round.get_secansa_score(), expected);
    }

    #[test]
    fn get_secansa_score_tres_val_won_bet() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let mut round = secansa_tests_round_fixture(&game);
        round.set_secansa_bet(secansa::Bet::Val(3), Some(Team::Team1));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team1, 6));
        assert_eq!(round.get_secansa_score(), expected);
    }

    #[test]
    fn get_secansa_score_tres_val_won_from_cards() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let mut round = secansa_tests_round_fixture(&game);
        round.set_secansa_bet(secansa::Bet::Val(3), None);

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 8));
        assert_eq!(round.get_secansa_score(), expected);
    }

    // Ali bets

    #[test]
    fn get_ali_winner_from_cards_no_ali() {
        let game = Game::new(vec![Player::new("a")]);
        let mut round = Round::new(&game, &game.players[0], deck::Deck::default());

        round.seats = vec![
            // No ali
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Uno,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Tres,
                    },
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Cinco,
                    },
                ],
            },
        ];

        assert!(round.get_winner_from_cards::<ali::Ali>().is_none());
    }

    #[test]
    fn get_ali_winner_from_cards() {
        let game = Game::new(vec![Player::new("a")]);
        let mut round = Round::new(&game, &game.players[0], deck::Deck::default());

        round.seats = vec![
            // No ali
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Uno,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Tres,
                    },
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Cinco,
                    },
                ],
            },
            // Two card ali
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
                        value: deck::Value::Cinco,
                    },
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Cinco,
                    },
                ],
            },
            // ali aces
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Uno,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Uno,
                    },
                    deck::Card {
                        suit: deck::Suit::Espadas,
                        value: deck::Value::Dos,
                    },
                ],
            },
            // Three card ali
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Caballo,
                    },
                    deck::Card {
                        suit: deck::Suit::Bastos,
                        value: deck::Value::Caballo,
                    },
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Caballo,
                    },
                ],
            },
        ];

        let expected = Some(Team::Team2);
        assert_eq!(round.get_winner_from_cards::<ali::Ali>(), expected);
    }

    #[test]
    fn get_ali_winner_from_cards_tie_wins_hand() {
        let game = Game::new(vec![
            Player::new("a"),
            Player::new("b"),
            Player::new("c"),
            Player::new("d"),
        ]);
        let mut round = Round::new(&game, &game.players[1], deck::Deck::default());

        round.seats = vec![
            // No ali
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
                        value: deck::Value::Tres,
                    },
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Cinco,
                    },
                ],
            },
            // ali aces
            Seat {
                player: &game.players[1],
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
                        suit: deck::Suit::Copas,
                        value: deck::Value::Uno,
                    },
                ],
            },
            // ali aces
            Seat {
                player: &game.players[2],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Uno,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Uno,
                    },
                    deck::Card {
                        suit: deck::Suit::Bastos,
                        value: deck::Value::Uno,
                    },
                ],
            },
            // ali aces
            Seat {
                player: &game.players[3],
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
                        suit: deck::Suit::Copas,
                        value: deck::Value::Uno,
                    },
                ],
            },
        ];

        let expected = Some(Team::Team1);
        assert_eq!(round.get_winner_from_cards::<ali::Ali>(), expected);
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

        assert!(round.get_ali_score().is_none())
    }

    #[test]
    fn get_ali_score_announced_no_ali() {
        // This situation should be impossible! Testing as it can be done in code anyway
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let mut round = Round::new(&game, &game.players[0], deck::Deck::default());
        round.set_ali_bet(ali::Bet::Envit, None);

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

        assert!(round.get_ali_score().is_none())
    }

    #[test]
    fn get_ali_score_announced_won_bet() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let mut round = ali_tests_round_fixture(&game);
        round.set_ali_bet(ali::Bet::Announced, Some(Team::Team1));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team1, 7));
        assert_eq!(round.get_ali_score(), expected);
    }

    #[test]
    fn get_ali_score_announced_won_from_cards() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let mut round = ali_tests_round_fixture(&game);
        round.set_ali_bet(ali::Bet::Announced, None);

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 9));
        assert_eq!(round.get_ali_score(), expected);
    }

    #[test]
    fn get_ali_score_envit_won_bet() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let mut round = ali_tests_round_fixture(&game);
        round.set_ali_bet(ali::Bet::Envit, Some(Team::Team1));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team1, 8));
        assert_eq!(round.get_ali_score(), expected);
    }

    #[test]
    fn get_ali_score_envit_won_from_cards() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let mut round = ali_tests_round_fixture(&game);
        round.set_ali_bet(ali::Bet::Envit, None);

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 10));
        assert_eq!(round.get_ali_score(), expected);
    }

    #[test]
    fn get_ali_score_tres_val_won_bet() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let mut round = ali_tests_round_fixture(&game);
        round.set_ali_bet(ali::Bet::Val(3), Some(Team::Team1));

        let expected = Some(scoreboard::RoundScoreSection(Team::Team1, 9));
        assert_eq!(round.get_ali_score(), expected);
    }

    #[test]
    fn get_ali_score_tres_val_won_from_cards() {
        let game = Game::new(vec![Player::new("a"), Player::new("b")]);
        let mut round = ali_tests_round_fixture(&game);
        round.set_ali_bet(ali::Bet::Val(3), None);

        let expected = Some(scoreboard::RoundScoreSection(Team::Team2, 11));
        assert_eq!(round.get_ali_score(), expected);
    }
}
