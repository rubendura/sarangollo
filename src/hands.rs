use std::cmp::Ordering;
use itertools::Itertools;
use deck::{Card, Value};

#[derive(Debug, Eq, PartialEq)]
struct Flor {
    cards: [Card; 3],
    marker: Card,
}

impl Flor {
    fn is_flor(cards: [Card; 3], marker: Card) -> bool {
        cards
            .iter()
            .filter(|card| !card.is_perico(marker))
            .filter(|card| !card.is_perica(marker))
            .map(|card| card.suit)
            .all_equal()
    }

    fn from_cards(cards: [Card; 3], marker: Card) -> Option<Flor> {
        if Flor::is_flor(cards, marker) {
            Some(Flor {
                cards: cards,
                marker: marker,
            })
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

impl Ord for Flor {
    fn cmp(&self, other: &Flor) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Flor {
    fn partial_cmp(&self, other: &Flor) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn is_secansa(cards: &[&Card; 3]) -> bool {
    let values = cards.iter().map(|card| card.value).sorted();
    let next_values = values.iter().filter_map(|value| value.next());
    values
        .iter()
        .skip(1)
        .zip(next_values)
        .any(|(value1, value2)| *value1 == value2)
}

fn is_ali(cards: &[&Card; 3]) -> bool {
    let values = cards.iter().map(|card| card.value).sorted();
    values
        .iter()
        .skip(1)
        .zip(values.iter())
        .any(|(value1, value2)| value1 == value2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use deck::spanish_deck::{Suit, Value};

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
        assert!(Flor::is_flor(hand, marker));
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
        assert!(!Flor::is_flor(hand, marker));
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
        assert!(!Flor::is_flor(hand, marker));
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
        assert!(!Flor::is_flor(hand, marker));
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
        assert!(Flor::is_flor(hand, marker));
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
        assert!(Flor::is_flor(hand, marker));
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
        assert!(Flor::is_flor(hand, marker));
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
        let result = Flor::from_cards(hand, marker);
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
        let result = Flor::from_cards(hand, marker);
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
        let result = Flor::from_cards(hand, marker).unwrap().value();
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
        let result = Flor::from_cards(hand, marker).unwrap().value();
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
        let result = Flor::from_cards(hand, marker).unwrap().value();
        assert_eq!(result, 42);
    }

    #[test]
    fn flor_sorting() {
        let marker = Card {
            suit: Suit::Oros,
            value: Value::Uno,
        };
        let flor_42 = Flor::from_cards(
            [
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
            [
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
            [
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

    #[test]
    fn is_secansa_3_true() {
        let hand = [
            &Card {
                suit: Suit::Oros,
                value: Value::Uno,
            },
            &Card {
                suit: Suit::Bastos,
                value: Value::Dos,
            },
            &Card {
                suit: Suit::Espadas,
                value: Value::Tres,
            },
        ];
        assert!(is_secansa(&hand));

        let hand = [
            &Card {
                suit: Suit::Copas,
                value: Value::Sota,
            },
            &Card {
                suit: Suit::Bastos,
                value: Value::Caballo,
            },
            &Card {
                suit: Suit::Espadas,
                value: Value::Rey,
            },
        ];
        assert!(is_secansa(&hand));
    }

    #[test]
    fn is_secansa_2_true() {
        let hand = [
            &Card {
                suit: Suit::Oros,
                value: Value::Uno,
            },
            &Card {
                suit: Suit::Bastos,
                value: Value::Dos,
            },
            &Card {
                suit: Suit::Espadas,
                value: Value::Cuatro,
            },
        ];
        assert!(is_secansa(&hand));

        let hand = [
            &Card {
                suit: Suit::Copas,
                value: Value::Siete,
            },
            &Card {
                suit: Suit::Bastos,
                value: Value::Caballo,
            },
            &Card {
                suit: Suit::Espadas,
                value: Value::Rey,
            },
        ];
        assert!(is_secansa(&hand));
    }

    #[test]
    fn is_secansa_false() {
        let hand = [
            &Card {
                suit: Suit::Oros,
                value: Value::Uno,
            },
            &Card {
                suit: Suit::Bastos,
                value: Value::Tres,
            },
            &Card {
                suit: Suit::Espadas,
                value: Value::Cinco,
            },
        ];
        assert!(!is_secansa(&hand));

        let hand = [
            &Card {
                suit: Suit::Copas,
                value: Value::Sota,
            },
            &Card {
                suit: Suit::Bastos,
                value: Value::Sota,
            },
            &Card {
                suit: Suit::Espadas,
                value: Value::Rey,
            },
        ];
        assert!(!is_secansa(&hand));
    }

    #[test]
    fn is_ali_3_true() {
        let hand = [
            &Card {
                suit: Suit::Oros,
                value: Value::Uno,
            },
            &Card {
                suit: Suit::Bastos,
                value: Value::Uno,
            },
            &Card {
                suit: Suit::Espadas,
                value: Value::Uno,
            },
        ];
        assert!(is_ali(&hand));

        let hand = [
            &Card {
                suit: Suit::Copas,
                value: Value::Rey,
            },
            &Card {
                suit: Suit::Bastos,
                value: Value::Rey,
            },
            &Card {
                suit: Suit::Espadas,
                value: Value::Rey,
            },
        ];
        assert!(is_ali(&hand));
    }

    #[test]
    fn is_ali_2_true() {
        let hand = [
            &Card {
                suit: Suit::Oros,
                value: Value::Uno,
            },
            &Card {
                suit: Suit::Bastos,
                value: Value::Uno,
            },
            &Card {
                suit: Suit::Oros,
                value: Value::Cuatro,
            },
        ];
        assert!(is_ali(&hand));

        let hand = [
            &Card {
                suit: Suit::Copas,
                value: Value::Siete,
            },
            &Card {
                suit: Suit::Bastos,
                value: Value::Rey,
            },
            &Card {
                suit: Suit::Copas,
                value: Value::Rey,
            },
        ];
        assert!(is_ali(&hand));
    }

    #[test]
    fn is_ali_false() {
        let hand = [
            &Card {
                suit: Suit::Oros,
                value: Value::Uno,
            },
            &Card {
                suit: Suit::Bastos,
                value: Value::Tres,
            },
            &Card {
                suit: Suit::Espadas,
                value: Value::Cinco,
            },
        ];
        assert!(!is_ali(&hand));

        let hand = [
            &Card {
                suit: Suit::Copas,
                value: Value::Caballo,
            },
            &Card {
                suit: Suit::Bastos,
                value: Value::Sota,
            },
            &Card {
                suit: Suit::Espadas,
                value: Value::Rey,
            },
        ];
        assert!(!is_ali(&hand));
    }
}
