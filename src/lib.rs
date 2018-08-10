#[macro_use]
extern crate itertools;
extern crate rand;

mod deck;
mod hands;
pub mod scoreboard;
mod scorers;
mod test_runner;

use hands::Hand;
use scorers::Scorer;

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
            scoreboard: scoreboard::Scoreboard::default(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct GameBet<T> {
    winner: Option<Team>,
    agreed_bet: T,
}

pub struct Round<'a> {
    game: &'a Game,
    seats: Vec<Seat<'a>>,
    dealer: &'a Player,
    deck: deck::Deck,
    marker: deck::Card,
    flor_scorer: scorers::flor::FlorScorer,
    secansa_scorer: scorers::secansa::SecansaScorer,
    ali_scorer: scorers::ali::AliScorer,
    truc_scorer: scorers::truc::TrucScorer,
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
            flor_scorer: Default::default(),
            secansa_scorer: Default::default(),
            ali_scorer: Default::default(),
            truc_scorer: Default::default(),
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

    fn set_flor_bet(&mut self, agreed_bet: scorers::flor::AgreedBet) {
        self.flor_scorer.set_bet(agreed_bet);
    }

    fn set_secansa_bet(&mut self, agreed_bet: scorers::secansa::AgreedBet) {
        self.secansa_scorer.set_bet(agreed_bet)
    }

    fn set_ali_bet(&mut self, agreed_bet: scorers::ali::AgreedBet) {
        self.ali_scorer.set_bet(agreed_bet)
    }

    fn set_truc_bet(&mut self, agreed_bet: scorers::truc::Bet) {
        self.truc_scorer.set_bet(agreed_bet)
    }

    fn get_round_score(&self) -> scoreboard::RoundScore {
        scoreboard::RoundScore {
            flor: self.flor_scorer.get_score(self),
            secansa: self.secansa_scorer.get_score(self),
            ali: self.ali_scorer.get_score(self),
            rey: scorers::rey::ReyScorer::default().get_score(self),
            truc: self.truc_scorer.get_score(self),
        }
    }

    fn iter_with_team(&self) -> impl Iterator<Item = (Team, &Seat)> {
        self.seats
            .iter()

            // Enumerate each seat's position
            .enumerate()

            .map(|(pos, seat)| {
                let team = seat.get_team(pos as u8);
                (team, seat)
            })
    }

    fn iter_from_hand(&'a self) -> impl Iterator<Item = (Team, &'a Seat<'a>)> {
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
        let mut hands_by_team = self
            .iter_from_hand()
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use hands::{ali, flor, secansa};

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

        // Before announcing
        assert!(round.flor_scorer.get_score(&round).is_none());

        // After announcing
        round.set_flor_bet(scorers::flor::AgreedBet::Announced(None));

        if let Some(scoreboard::RoundScoreSection(_team, value)) =
            round.flor_scorer.get_score(&round)
        {
            assert_eq!(value, 3);
        } else {
            panic!("Get flor score failed when it shouldn't")
        };

        // After setting a bet
        round.set_flor_bet(scorers::flor::AgreedBet::Envit(None));

        if let Some(scoreboard::RoundScoreSection(_team, value)) =
            round.flor_scorer.get_score(&round)
        {
            assert_eq!(value, 6);
        } else {
            panic!("Get flor score failed when it shouldn't")
        };
    }

    #[test]
    fn test_set_secansa_bet() {
        let game = Game::new(vec![Player::new("a")]);
        let mut round = Round::new(&game, &game.players[0], deck::Deck::default());

        round.seats = vec![Seat {
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
                    value: deck::Value::Cuatro,
                },
            ],
        }];

        // Before announcing
        assert!(round.secansa_scorer.get_score(&round).is_none());

        // After announcing
        round.set_secansa_bet(scorers::secansa::AgreedBet::Announced(None));

        if let Some(scoreboard::RoundScoreSection(_team, value)) =
            round.secansa_scorer.get_score(&round)
        {
            assert_eq!(value, 3);
        } else {
            panic!("Get secansa score failed when it shouldn't")
        };

        // After setting a bet
        round.set_secansa_bet(scorers::secansa::AgreedBet::Envit(None));

        if let Some(scoreboard::RoundScoreSection(_team, value)) =
            round.secansa_scorer.get_score(&round)
        {
            assert_eq!(value, 4);
        } else {
            panic!("Get secansa score failed when it shouldn't")
        };
    }

    #[test]
    fn test_set_ali_bet() {
        let game = Game::new(vec![Player::new("a")]);
        let mut round = Round::new(&game, &game.players[0], deck::Deck::default());

        round.seats = vec![Seat {
            player: &game.players[0],
            hand: vec![],
            face_up_cards: vec![
                deck::Card {
                    suit: deck::Suit::Copas,
                    value: deck::Value::Tres,
                },
                deck::Card {
                    suit: deck::Suit::Copas,
                    value: deck::Value::Tres,
                },
                deck::Card {
                    suit: deck::Suit::Copas,
                    value: deck::Value::Tres,
                },
            ],
        }];

        // Before announcing
        assert!(round.ali_scorer.get_score(&round).is_none());

        // After announcing
        round.set_ali_bet(scorers::ali::AgreedBet::Announced(None));

        if let Some(scoreboard::RoundScoreSection(_team, value)) =
            round.ali_scorer.get_score(&round)
        {
            assert_eq!(value, 3);
        } else {
            panic!("Get ali score failed when it shouldn't")
        };

        // After setting a bet
        round.set_ali_bet(scorers::ali::AgreedBet::Envit(None));

        if let Some(scoreboard::RoundScoreSection(_team, value)) =
            round.ali_scorer.get_score(&round)
        {
            assert_eq!(value, 4);
        } else {
            panic!("Get ali score failed when it shouldn't")
        };
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

    #[test]
    fn get_round_score() {
        let game = Game::new(vec![
            Player::new("a"),
            Player::new("b"),
            Player::new("c"),
            Player::new("d"),
        ]);
        let mut round = Round::new(&game, &game.players[1], deck::Deck::default());

        round.seats = vec![
            // 39 flor, 7-sota secansa, perica
            Seat {
                player: &game.players[0],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Siete,
                    },
                    deck::Card {
                        suit: deck::Suit::Espadas,
                        value: deck::Value::Sota,
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
            // secansa real, perico
            Seat {
                player: &game.players[2],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Espadas,
                        value: deck::Value::Caballo,
                    },
                    deck::Card {
                        suit: deck::Suit::Copas,
                        value: deck::Value::Rey,
                    },
                    deck::Card {
                        suit: deck::Suit::Bastos,
                        value: deck::Value::Sota,
                    },
                ],
            },
            // secansa 3
            Seat {
                player: &game.players[3],
                hand: vec![],
                face_up_cards: vec![
                    deck::Card {
                        suit: deck::Suit::Oros,
                        value: deck::Value::Cinco,
                    },
                    deck::Card {
                        suit: deck::Suit::Bastos,
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
            .flor_scorer
            .set_bet(scorers::flor::AgreedBet::Announced(None));
        round
            .secansa_scorer
            .set_bet(scorers::secansa::AgreedBet::Announced(None));
        round
            .ali_scorer
            .set_bet(scorers::ali::AgreedBet::Announced(None));

        let expected = scoreboard::RoundScore {
            flor: Some(scoreboard::RoundScoreSection(Team::Team1, 3)),
            secansa: Some(scoreboard::RoundScoreSection(Team::Team1, 4)),
            ali: Some(scoreboard::RoundScoreSection(Team::Team2, 6)),
            rey: Some(scoreboard::RoundScoreSection(Team::Team1, 1)),
            truc: Some(scoreboard::RoundScoreSection(Team::Team1, 1)),
        };
        assert_eq!(expected, round.get_round_score());
    }

}
