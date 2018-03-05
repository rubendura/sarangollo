#[macro_use]
extern crate itertools;
extern crate rand;

mod deck;
pub mod scoreboard;
mod hands;

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
}

struct Game {
    players: Vec<Player>,
}

struct Round<'a> {
    seats: Vec<Seat<'a>>,
    dealer: &'a Player,
    deck: deck::Deck,
    marker: deck::Card,
}

impl<'a> Round<'a> {
    fn new(game: &'a Game, dealer: &'a Player, mut deck: deck::Deck) -> Self {
        let seats = game.players.iter().map(Seat::new).collect();
        Self {
            seats,
            dealer,
            marker: deck.draw().unwrap(),
            deck,
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
        let game = Game {
            players: vec![
                Player::new("a"),
                Player::new("b"),
                Player::new("c"),
                Player::new("d"),
                Player::new("e"),
                Player::new("f"),
            ],
        };
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
        let game = Game {
            players: vec![
                Player::new("a"),
                Player::new("b"),
                Player::new("c"),
                Player::new("d"),
                Player::new("e"),
                Player::new("f"),
            ],
        };
        let deck = deck::Deck::default();
        for i in 0..6 {
            let round = Round::new(&game, &game.players[i], deck.clone());
            assert_eq!(round.dealer_position(), i);
        }
    }

    #[test]
    fn round_deal() {
        let game = Game {
            players: vec![
                Player::new("a"),
                Player::new("b"),
                Player::new("c"),
                Player::new("d"),
                Player::new("e"),
                Player::new("f"),
            ],
        };
        let mut round = Round::new(&game, &game.players[0], deck::Deck::default());
        for i in 0..6 {
            assert_eq!(round.seats[i].hand.len(), 0);
            assert_eq!(round.seats[i].face_up_cards.len(), 0);
        }
        const num_cards: usize = 6;
        round.deal(num_cards);
        for i in 0..6 {
            assert_eq!(round.seats[i].hand.len(), 6);
            assert_eq!(round.seats[i].face_up_cards.len(), 0);
        }
    }

}
