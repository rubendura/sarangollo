use deck::Card;
use hands::Hand;
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
pub struct Secansa {
    cards: Vec<Card>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Bet {
    Announced,
    Envit,
    Val(u8),
}

impl<'a> Hand<'a> for Secansa {
    fn from_cards(cards: &[Card], _marker: Card) -> Option<Self> {
        Secansa::from_cards_slice(cards)
    }
}

impl Secansa {
    pub fn from_cards_slice(cards: &[Card]) -> Option<Self> {
        if let Some(cards) = Self::sorted_secansa_cards(cards) {
            Some(Secansa { cards })
        } else {
            None
        }
    }

    fn is_secansa_3_cards(&self) -> bool {
        self.cards.len() == 3
    }

    fn highest_card(&self) -> &Card {
        self.cards.last().unwrap()
    }

    fn sorted_secansa_cards(cards: &[Card]) -> Option<Vec<Card>> {
        let mut sorted = cards.to_vec();
        sorted.sort_by_key(|card| card.value);

        let mut cards = sorted
            .iter()
            // zip with next card
            .zip(sorted.iter().skip(1))
            // filter consecutive values
            .filter(|&(card, next_card)| card.value.next() == Some(next_card.value))
            // take both cards if they are consecutive
            .flat_map(|(card, next_card)| vec![*card, *next_card])
            .collect::<Vec<_>>();

        // dedup cards
        cards.dedup();

        // If result vector only has 1 card, no secansa can be formed
        if cards.len() >= 2 {
            Some(cards)
        } else {
            None
        }
    }

    pub fn score(&self) -> u8 {
        if self.is_secansa_3_cards() {
            3
        } else {
            1
        }
    }
}

impl Ord for Secansa {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_secansa_3_cards() && !other.is_secansa_3_cards() {
            Ordering::Greater
        } else if !self.is_secansa_3_cards() && other.is_secansa_3_cards() {
            Ordering::Less
        } else {
            let self_max = self.highest_card().value;
            let other_max = other.highest_card().value;
            self_max.cmp(&other_max)
        }
    }
}

impl PartialOrd for Secansa {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use deck::{Suit, Value};

    #[test]
    fn is_secansa_3_true() {
        let hand = [
            Card {
                suit: Suit::Oros,
                value: Value::Uno,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Dos,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Tres,
            },
        ];
        assert!(Secansa::sorted_secansa_cards(&hand).is_some());

        let hand = [
            Card {
                suit: Suit::Copas,
                value: Value::Sota,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Caballo,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Rey,
            },
        ];
        assert!(Secansa::sorted_secansa_cards(&hand).is_some());
    }

    #[test]
    fn is_secansa_2_true() {
        let hand = [
            Card {
                suit: Suit::Oros,
                value: Value::Uno,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Dos,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Cuatro,
            },
        ];
        assert!(Secansa::sorted_secansa_cards(&hand).is_some());

        let hand = [
            Card {
                suit: Suit::Copas,
                value: Value::Siete,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Caballo,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Rey,
            },
        ];
        assert!(Secansa::sorted_secansa_cards(&hand).is_some());
    }

    #[test]
    fn is_secansa_false() {
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
        assert!(Secansa::sorted_secansa_cards(&hand).is_none());

        let hand = [
            Card {
                suit: Suit::Copas,
                value: Value::Sota,
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
        assert!(Secansa::sorted_secansa_cards(&hand).is_none());
    }

    #[test]
    fn is_secansa_3_cards_true() {
        let hand = [
            Card {
                suit: Suit::Oros,
                value: Value::Uno,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Dos,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Tres,
            },
        ];
        assert!(
            Secansa::from_cards_slice(&hand)
                .unwrap()
                .is_secansa_3_cards()
        );
    }

    #[test]
    fn is_secansa_3_cards_false() {
        let hand = [
            Card {
                suit: Suit::Oros,
                value: Value::Uno,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Dos,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Cuatro,
            },
        ];
        assert!(!Secansa::from_cards_slice(&hand)
            .unwrap()
            .is_secansa_3_cards());
    }

    #[test]
    fn secansa_score_2_cards() {
        let hand = [
            Card {
                suit: Suit::Oros,
                value: Value::Uno,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Dos,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Cuatro,
            },
        ];
        assert_eq!(Secansa::from_cards_slice(&hand).unwrap().score(), 1);
    }

    #[test]
    fn secansa_score_3_cards() {
        let hand = [
            Card {
                suit: Suit::Oros,
                value: Value::Uno,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Dos,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Tres,
            },
        ];
        assert_eq!(Secansa::from_cards_slice(&hand).unwrap().score(), 3);
    }

    #[test]
    fn secansa_ordering() {
        let secansa_3_cards = Secansa::from_cards_slice(&[
            Card {
                suit: Suit::Oros,
                value: Value::Uno,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Dos,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Tres,
            },
        ]);
        let secansa_real = Secansa::from_cards_slice(&[
            Card {
                suit: Suit::Oros,
                value: Value::Uno,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Dos,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Tres,
            },
        ]);
        let secansa_2_top = Secansa::from_cards_slice(&[
            Card {
                suit: Suit::Oros,
                value: Value::Sota,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Caballo,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Tres,
            },
        ]);
        let secansa_2_low = Secansa::from_cards_slice(&[
            Card {
                suit: Suit::Oros,
                value: Value::Dos,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Uno,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Cinco,
            },
        ]);
        let secansa_2_low_with_high_card = Secansa::from_cards_slice(&[
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
                value: Value::Rey,
            },
        ]);
        let expected = [
            &secansa_2_low,
            &secansa_2_low_with_high_card,
            &secansa_2_top,
            &secansa_3_cards,
            &secansa_real,
        ];
        let mut result = [
            &secansa_3_cards,
            &secansa_2_low_with_high_card,
            &secansa_2_top,
            &secansa_real,
            &secansa_2_low,
        ];
        result.sort();
        assert_eq!(expected, result);
    }
}
