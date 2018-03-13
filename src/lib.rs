#[macro_use]
extern crate itertools;
extern crate rand;

mod deck;
pub mod scoreboard;
mod hands;

use hands::{ali, flor, secansa};

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
            hand: vec![
                deck::Card {
                    suit: deck::Suit::Bastos,
                    value: deck::Value::Caballo,
                },
            ],
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
            hand: vec![
                deck::Card {
                    suit: deck::Suit::Bastos,
                    value: deck::Value::Caballo,
                },
            ],
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

}
