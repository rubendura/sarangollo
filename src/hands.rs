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

    fn from_cards(cards: [Card; 3], marker: Card) -> Option<Self> {
        if Self::is_flor(cards, marker) {
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
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Flor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Secansa {
    cards: Vec<Card>,
}

impl Secansa {
    fn is_secansa_3_cards(&self) -> bool {
        self.cards.len() == 3
    }

    fn highest_card(&self) -> &Card {
        self.cards.last().unwrap()
    }

    fn sorted_secansa_cards(cards: [Card; 3]) -> Option<Vec<Card>> {
        let mut sorted = cards.clone();
        sorted.sort_by_key(|card| card.value);

        let mut res = Vec::new();

        // If .next() returns None, card is a Rey.
        // We can't form a secansa if first card of sorted hand is Rey
        if let Some(value) = cards[0].value.next() {
            if value == cards[1].value {
                res.push(cards[0]);
            }
        } else {
            return None;
        }

        // On a sorted hand, if there is secansa, the middle cart will always be there
        res.push(cards[1]);

        // If middle card is a Rey, we can't form secansa with the last card
        if let Some(value) = cards[1].value.next() {
            if value == cards[2].value {
                res.push(cards[1]);
            }
        }

        // If result vector only has 1 card, no secansa can be formed
        if res.len() == 1 {
            None
        } else {
            Some(res)
        }
    }

    fn from_cards(cards: [Card; 3]) -> Option<Self> {
        if let Some(cards) = Self::sorted_secansa_cards(cards) {
            Some(Secansa { cards: cards })
        } else {
            None
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
    fn flor_ordering() {
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
        assert!(Secansa::sorted_secansa_cards(hand).is_some());

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
        assert!(Secansa::sorted_secansa_cards(hand).is_some());
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
        assert!(Secansa::sorted_secansa_cards(hand).is_some());

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
        assert!(Secansa::sorted_secansa_cards(hand).is_some());
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
        assert!(Secansa::sorted_secansa_cards(hand).is_none());

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
        assert!(Secansa::sorted_secansa_cards(hand).is_none());
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
        assert!(Secansa::from_cards(hand).unwrap().is_secansa_3_cards());
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
        assert!(!Secansa::from_cards(hand).unwrap().is_secansa_3_cards());
    }

    #[test]
    fn secansa_ordering() {
        let secansa_3_cards = Secansa::from_cards([
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
        let secansa_real = Secansa::from_cards([
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
        let secansa_2_top = Secansa::from_cards([
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
        let secansa_2_low = Secansa::from_cards([
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
        let secansa_2_low_with_high_card = Secansa::from_cards([
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
