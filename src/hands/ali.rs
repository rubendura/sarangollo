use deck::{Card, Value};
use hands::Hand;
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
pub struct Ali {
    cards: Vec<Card>,
}

impl<'a> Hand<'a> for Ali {
    fn from_cards(cards: &[Card], _marker: Card) -> Option<Self> {
        Ali::from_cards_slice(cards)
    }
}

impl Ali {
    fn from_cards_slice(cards: &[Card]) -> Option<Self> {
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

    fn is_ali_3_cards(&self) -> bool {
        self.cards.len() == 3
    }

    fn is_ali_aces(&self) -> bool {
        self.cards.iter().all(|card| card.value == Value::Uno)
    }

    pub fn score(&self) -> u8 {
        match (self.is_ali_3_cards(), self.is_ali_aces()) {
            (true, true) => 6,
            (true, false) | (false, true) => 3,
            (false, false) => 1,
        }
    }
}

impl Ord for Ali {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_ali_3_cards() && !other.is_ali_3_cards() {
            Ordering::Greater
        } else if !self.is_ali_3_cards() && other.is_ali_3_cards() {
            Ordering::Less
        } else {
            let self_value = self.cards[0].value;
            let other_value = other.cards[0].value;
            // There is a special case for Uno, being the highest value in Ali
            if self_value == Value::Uno && other_value != Value::Uno {
                Ordering::Greater
            } else if self_value != Value::Uno && other_value == Value::Uno {
                Ordering::Less
            } else {
                self_value.cmp(&other_value)
            }
        }
    }
}

impl PartialOrd for Ali {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
        assert!(Ali::from_cards_slice(&hand).is_some());

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
        assert!(Ali::from_cards_slice(&hand).is_some());
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
        assert!(Ali::from_cards_slice(&hand).is_some());

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
        assert!(Ali::from_cards_slice(&hand).is_some());
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
        assert!(Ali::from_cards_slice(&hand).is_none());

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
        assert!(Ali::from_cards_slice(&hand).is_none());
    }

    #[test]
    fn is_ali_3_cards_true() {
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
        assert!(Ali::from_cards_slice(&hand).unwrap().is_ali_3_cards());
    }

    #[test]
    fn is_ali_3_cards_false() {
        let hand = [
            Card {
                suit: Suit::Oros,
                value: Value::Rey,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Cuatro,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Cuatro,
            },
        ];
        assert!(!Ali::from_cards_slice(&hand).unwrap().is_ali_3_cards());
    }

    #[test]
    fn ali_score_2_cards() {
        let hand = [
            Card {
                suit: Suit::Oros,
                value: Value::Rey,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Cuatro,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Cuatro,
            },
        ];
        assert_eq!(Ali::from_cards_slice(&hand).unwrap().score(), 1);
    }

    #[test]
    fn ali_score_2_aces() {
        let hand = [
            Card {
                suit: Suit::Oros,
                value: Value::Rey,
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
        assert_eq!(Ali::from_cards_slice(&hand).unwrap().score(), 3);
    }

    #[test]
    fn ali_score_3_cards() {
        let hand = [
            Card {
                suit: Suit::Oros,
                value: Value::Cuatro,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Cuatro,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Cuatro,
            },
        ];
        assert_eq!(Ali::from_cards_slice(&hand).unwrap().score(), 3);
    }

    #[test]
    fn ali_score_3_aces() {
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
        assert_eq!(Ali::from_cards_slice(&hand).unwrap().score(), 6);
    }

    #[test]
    fn ali_ordering() {
        let ali_3_reyes = Ali::from_cards_slice(&[
            Card {
                suit: Suit::Oros,
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
        ]);
        let ali_3_unos = Ali::from_cards_slice(&[
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
        ]);
        let ali_3_normal = Ali::from_cards_slice(&[
            Card {
                suit: Suit::Oros,
                value: Value::Seis,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Seis,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Seis,
            },
        ]);
        let ali_3_low = Ali::from_cards_slice(&[
            Card {
                suit: Suit::Oros,
                value: Value::Dos,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Dos,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Dos,
            },
        ]);
        let ali_2_reyes = Ali::from_cards_slice(&[
            Card {
                suit: Suit::Oros,
                value: Value::Rey,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Tres,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Rey,
            },
        ]);
        let ali_2_unos = Ali::from_cards_slice(&[
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
                value: Value::Rey,
            },
        ]);
        let ali_2_normal = Ali::from_cards_slice(&[
            Card {
                suit: Suit::Oros,
                value: Value::Cinco,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Cinco,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Seis,
            },
        ]);
        let ali_2_low = Ali::from_cards_slice(&[
            Card {
                suit: Suit::Oros,
                value: Value::Dos,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Tres,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Dos,
            },
        ]);
        let expected = [
            &ali_2_low,
            &ali_2_normal,
            &ali_2_reyes,
            &ali_2_unos,
            &ali_3_low,
            &ali_3_normal,
            &ali_3_reyes,
            &ali_3_unos,
        ];
        let mut result = [
            &ali_3_unos,
            &ali_2_low,
            &ali_3_reyes,
            &ali_2_unos,
            &ali_2_normal,
            &ali_3_normal,
            &ali_3_low,
            &ali_2_reyes,
        ];
        result.sort();
        assert_eq!(expected, result);
    }
}
