use itertools::Itertools;
use deck::Card;

fn is_flor(cards: &[&Card; 3]) -> bool {
    cards.iter().map(|card| card.suit).all_equal()
}

#[cfg(test)]
mod tests {
    use super::*;
    use deck::spanish_deck::{Suit, Value};

    #[test]
    fn is_flor_true() {
        let hand = [
            &Card {
                suit: Suit::Oros,
                value: Value::Uno,
            },
            &Card {
                suit: Suit::Oros,
                value: Value::Dos,
            },
            &Card {
                suit: Suit::Oros,
                value: Value::Tres,
            },
        ];
        assert!(is_flor(&hand));
    }

    #[test]
    fn is_flor_false() {
        let hand = [
            &Card {
                suit: Suit::Oros,
                value: Value::Uno,
            },
            &Card {
                suit: Suit::Oros,
                value: Value::Dos,
            },
            &Card {
                suit: Suit::Bastos,
                value: Value::Tres,
            },
        ];
        assert!(!is_flor(&hand));
    }
}
