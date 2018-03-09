use std::cmp::Ordering;
use itertools::Itertools;
use deck::{Card, Value};

#[derive(Debug, Eq, PartialEq)]
pub struct Flor<'a> {
    cards: &'a [Card],
    marker: Card,
}

impl<'a> Flor<'a> {
    fn is_flor(cards: &[Card], marker: Card) -> bool {
        if cards.len() != 3 {
            return false;
        }
        cards
            .iter()
            .filter(|card| !card.is_perico(marker))
            .filter(|card| !card.is_perica(marker))
            .map(|card| card.suit)
            .all_equal()
    }

    pub fn from_cards(cards: &'a [Card], marker: Card) -> Option<Self> {
        if Self::is_flor(cards, marker) {
            Some(Flor { cards, marker })
        } else {
            None
        }
    }

    fn value(&self) -> u8 {
        let total: u8 = self.cards
            .iter()
            .map(|card| match card.value {
                Value::Uno => 1,
                Value::Dos => 2,
                Value::Tres => 3,
                Value::Cuatro => 4,
                Value::Cinco => 5,
                Value::Seis => 6,
                Value::Siete => 7,
                Value::Sota if card.is_perica(self.marker) => 7,
                Value::Caballo if card.is_perico(self.marker) => 8,
                _ => 0,
            })
            .sum();

        // Flor is counted from 20 ¯\_(ツ)_/¯
        20 + total
    }
}

impl<'a> Ord for Flor<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl<'a> PartialOrd for Flor<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use deck::{Suit, Value};

    #[test]
    fn is_flor_true() {
        let hand = [
            Card {
                suit: Suit::Oros,
                value: Value::Uno,
            },
            Card {
                suit: Suit::Oros,
                value: Value::Dos,
            },
            Card {
                suit: Suit::Oros,
                value: Value::Tres,
            },
        ];
        let marker = Card {
            suit: Suit::Bastos,
            value: Value::Uno,
        };
        assert!(Flor::is_flor(&hand, marker));
    }

    #[test]
    fn is_flor_false() {
        let hand = [
            Card {
                suit: Suit::Oros,
                value: Value::Uno,
            },
            Card {
                suit: Suit::Oros,
                value: Value::Dos,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Tres,
            },
        ];
        let marker = Card {
            suit: Suit::Bastos,
            value: Value::Uno,
        };
        assert!(!Flor::is_flor(&hand, marker));
    }

    #[test]
    fn is_flor_false_perico() {
        let hand = [
            Card {
                suit: Suit::Oros,
                value: Value::Uno,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Dos,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Caballo,
            },
        ];
        let marker = Card {
            suit: Suit::Bastos,
            value: Value::Uno,
        };
        assert!(!Flor::is_flor(&hand, marker));
    }

    #[test]
    fn is_flor_false_perica() {
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
                suit: Suit::Bastos,
                value: Value::Sota,
            },
        ];
        let marker = Card {
            suit: Suit::Bastos,
            value: Value::Uno,
        };
        assert!(!Flor::is_flor(&hand, marker));
    }

    #[test]
    fn is_flor_perico() {
        let hand = [
            Card {
                suit: Suit::Oros,
                value: Value::Uno,
            },
            Card {
                suit: Suit::Oros,
                value: Value::Dos,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Caballo,
            },
        ];
        let marker = Card {
            suit: Suit::Bastos,
            value: Value::Uno,
        };
        assert!(Flor::is_flor(&hand, marker));
    }

    #[test]
    fn is_flor_perica() {
        let hand = [
            Card {
                suit: Suit::Oros,
                value: Value::Uno,
            },
            Card {
                suit: Suit::Oros,
                value: Value::Dos,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Sota,
            },
        ];
        let marker = Card {
            suit: Suit::Bastos,
            value: Value::Uno,
        };
        assert!(Flor::is_flor(&hand, marker));
    }

    #[test]
    fn is_flor_perico_perica() {
        let hand = [
            Card {
                suit: Suit::Oros,
                value: Value::Uno,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Sota,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Caballo,
            },
        ];
        let marker = Card {
            suit: Suit::Bastos,
            value: Value::Uno,
        };
        assert!(Flor::is_flor(&hand, marker));
    }

    #[test]
    fn flor_from_cards_some() {
        let hand = [
            Card {
                suit: Suit::Oros,
                value: Value::Sota,
            },
            Card {
                suit: Suit::Oros,
                value: Value::Caballo,
            },
            Card {
                suit: Suit::Oros,
                value: Value::Rey,
            },
        ];
        let marker = Card {
            suit: Suit::Bastos,
            value: Value::Uno,
        };
        let result = Flor::from_cards(&hand, marker);
        assert!(result.is_some());
    }

    #[test]
    fn flor_from_cards_none() {
        let hand = [
            Card {
                suit: Suit::Oros,
                value: Value::Sota,
            },
            Card {
                suit: Suit::Oros,
                value: Value::Caballo,
            },
            Card {
                suit: Suit::Bastos,
                value: Value::Rey,
            },
        ];
        let marker = Card {
            suit: Suit::Bastos,
            value: Value::Uno,
        };
        let result = Flor::from_cards(&hand, marker);
        assert!(result.is_none());
    }

    #[test]
    fn flor_value_figures() {
        let hand = [
            Card {
                suit: Suit::Oros,
                value: Value::Sota,
            },
            Card {
                suit: Suit::Oros,
                value: Value::Caballo,
            },
            Card {
                suit: Suit::Oros,
                value: Value::Rey,
            },
        ];
        let marker = Card {
            suit: Suit::Bastos,
            value: Value::Uno,
        };
        let result = Flor::from_cards(&hand, marker).unwrap().value();
        assert_eq!(result, 20);
    }

    #[test]
    fn flor_value_standard() {
        let hand = [
            Card {
                suit: Suit::Oros,
                value: Value::Siete,
            },
            Card {
                suit: Suit::Oros,
                value: Value::Cinco,
            },
            Card {
                suit: Suit::Oros,
                value: Value::Tres,
            },
        ];
        let marker = Card {
            suit: Suit::Oros,
            value: Value::Uno,
        };
        let result = Flor::from_cards(&hand, marker).unwrap().value();
        assert_eq!(result, 35);
    }

    #[test]
    fn flor_value_42() {
        let hand = [
            Card {
                suit: Suit::Oros,
                value: Value::Sota,
            },
            Card {
                suit: Suit::Oros,
                value: Value::Caballo,
            },
            Card {
                suit: Suit::Espadas,
                value: Value::Siete,
            },
        ];
        let marker = Card {
            suit: Suit::Oros,
            value: Value::Uno,
        };
        let result = Flor::from_cards(&hand, marker).unwrap().value();
        assert_eq!(result, 42);
    }

    #[test]
    fn flor_ordering() {
        let marker = Card {
            suit: Suit::Oros,
            value: Value::Uno,
        };
        let flor_42 = Flor::from_cards(
            &[
                Card {
                    suit: Suit::Oros,
                    value: Value::Sota,
                },
                Card {
                    suit: Suit::Oros,
                    value: Value::Caballo,
                },
                Card {
                    suit: Suit::Espadas,
                    value: Value::Siete,
                },
            ],
            marker,
        ).unwrap();
        let flor_20 = Flor::from_cards(
            &[
                Card {
                    suit: Suit::Bastos,
                    value: Value::Sota,
                },
                Card {
                    suit: Suit::Bastos,
                    value: Value::Caballo,
                },
                Card {
                    suit: Suit::Bastos,
                    value: Value::Rey,
                },
            ],
            marker,
        ).unwrap();
        let flor_35 = Flor::from_cards(
            &[
                Card {
                    suit: Suit::Oros,
                    value: Value::Siete,
                },
                Card {
                    suit: Suit::Oros,
                    value: Value::Cinco,
                },
                Card {
                    suit: Suit::Oros,
                    value: Value::Tres,
                },
            ],
            marker,
        ).unwrap();
        let mut flors = [&flor_42, &flor_20, &flor_35];
        flors.sort();
        let expected = [&flor_20, &flor_35, &flor_42];
        assert_eq!(flors, expected);
    }
}
