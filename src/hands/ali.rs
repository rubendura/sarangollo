use itertools::Itertools;
use deck::Card;

struct Ali {
    cards: Vec<Card>,
}

impl Ali {
    fn from_cards(cards: [Card; 3]) -> Option<Self> {
        // if let Some(cards) = Self::ali_cards(cards);
        cards
            .into_iter()
            .sorted_by_key(|card| card.value)
            .into_iter()
            .group_by(|card| card.value)
            .into_iter()
            .map(|(_, group)| group.cloned().collect::<Vec<_>>())
            .filter(|group| group.len() >= 2)
            .max_by_key(|group| group.len())
            .map(|group| Self { cards: group })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use deck::{Suit, Value};

    #[test]
    fn is_ali_3_true() {
        let hand = [
            Card {
                suit: Suit::Oros,
                value: Value::Uno,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Uno,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Uno,
            },
        ];
        assert!(Ali::from_cards(hand).is_some());

        let hand = [
            Card {
                suit: Suit::Copas,
                value: Value::Rey,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Rey,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Rey,
            },
        ];
        assert!(Ali::from_cards(hand).is_some());
    }

    #[test]
    fn is_ali_2_true() {
        let hand = [
            Card {
                suit: Suit::Oros,
                value: Value::Uno,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Uno,
            },
            Card {
                suit: Suit::Oros,
                value: Value::Cuatro,
            },
        ];
        assert!(Ali::from_cards(hand).is_some());

        let hand = [
            Card {
                suit: Suit::Copas,
                value: Value::Siete,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Rey,
            },
            Card {
                suit: Suit::Copas,
                value: Value::Rey,
            },
        ];
        assert!(Ali::from_cards(hand).is_some());
    }

    #[test]
    fn is_ali_false() {
        let hand = [
            Card {
                suit: Suit::Oros,
                value: Value::Uno,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Tres,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Cinco,
            },
        ];
        assert!(Ali::from_cards(hand).is_none());

        let hand = [
            Card {
                suit: Suit::Copas,
                value: Value::Caballo,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Sota,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Rey,
            },
        ];
        assert!(Ali::from_cards(hand).is_none());
    }
}
