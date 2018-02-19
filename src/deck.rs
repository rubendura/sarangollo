pub use self::spanish_deck::SpanishDeck;
pub use self::spanish_deck::Card;
pub use self::spanish_deck::Suit;
pub use self::spanish_deck::Value;

pub trait Deck {
    type Card;
    fn new() -> Self;
    fn draw_top(&mut self) -> Option<Self::Card>;
    fn shuffle(&mut self);
}

pub mod spanish_deck {
    use super::Deck;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Suit {
        Bastos,
        Copas,
        Oros,
        Espadas,
    }

    static SUITS: [Suit; 4] = [Suit::Oros, Suit::Copas, Suit::Bastos, Suit::Espadas];

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Value {
        Uno,
        Dos,
        Tres,
        Cuatro,
        Cinco,
        Seis,
        Siete,
        Sota,
        Caballo,
        Rey,
    }

    impl Value {
        pub fn next(&self) -> Option<Value> {
            match *self {
                Value::Uno => Some(Value::Dos),
                Value::Dos => Some(Value::Tres),
                Value::Tres => Some(Value::Cuatro),
                Value::Cuatro => Some(Value::Cinco),
                Value::Cinco => Some(Value::Seis),
                Value::Seis => Some(Value::Siete),
                Value::Siete => Some(Value::Sota),
                Value::Sota => Some(Value::Caballo),
                Value::Caballo => Some(Value::Rey),
                Value::Rey => None,
            }
        }
    }

    static VALUES: [Value; 10] = [
        Value::Uno,
        Value::Dos,
        Value::Tres,
        Value::Cuatro,
        Value::Cinco,
        Value::Seis,
        Value::Siete,
        Value::Sota,
        Value::Caballo,
        Value::Rey,
    ];

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Card {
        pub suit: Suit,
        pub value: Value,
    }

    impl Card {
        pub fn is_perico(&self, marker: Card) -> bool {
            match marker {
                Card {
                    suit,
                    value: Value::Caballo,
                } => self.suit == suit && self.value == Value::Rey,
                Card { suit, .. } => self.suit == suit && self.value == Value::Caballo,
            }
        }

        pub fn is_perica(&self, marker: Card) -> bool {
            match marker {
                Card {
                    suit,
                    value: Value::Sota,
                } => self.suit == suit && self.value == Value::Rey,
                Card { suit, .. } => self.suit == suit && self.value == Value::Sota,
            }
        }
    }

    pub struct SpanishDeck {
        cards: Vec<Card>,
    }

    impl Deck for SpanishDeck {
        type Card = Card;
        fn new() -> Self {
            let mut cards = vec![];
            for (&suit, &value) in iproduct!(SUITS.into_iter(), VALUES.into_iter()) {
                cards.push(Card {
                    suit: suit,
                    value: value,
                })
            }
            SpanishDeck { cards: cards }
        }
        fn draw_top(&mut self) -> Option<Self::Card> {
            self.cards.pop()
        }
        fn shuffle(&mut self) {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_is_perico() {
        let marker = Card {
            suit: Suit::Oros,
            value: Value::Uno,
        };
        let card = Card {
            suit: Suit::Oros,
            value: Value::Caballo,
        };
        assert!(card.is_perico(marker));

        let marker = Card {
            suit: Suit::Oros,
            value: Value::Caballo,
        };
        let card = Card {
            suit: Suit::Oros,
            value: Value::Rey,
        };
        assert!(card.is_perico(marker));

        let marker = Card {
            suit: Suit::Oros,
            value: Value::Uno,
        };
        let card = Card {
            suit: Suit::Oros,
            value: Value::Tres,
        };
        assert!(!card.is_perico(marker));

        let marker = Card {
            suit: Suit::Oros,
            value: Value::Uno,
        };
        let card = Card {
            suit: Suit::Bastos,
            value: Value::Caballo,
        };
        assert!(!card.is_perico(marker));
    }

    #[test]
    fn card_is_perica() {
        let marker = Card {
            suit: Suit::Oros,
            value: Value::Uno,
        };
        let card = Card {
            suit: Suit::Oros,
            value: Value::Sota,
        };
        assert!(card.is_perica(marker));

        let marker = Card {
            suit: Suit::Oros,
            value: Value::Sota,
        };
        let card = Card {
            suit: Suit::Oros,
            value: Value::Rey,
        };
        assert!(card.is_perica(marker));

        let marker = Card {
            suit: Suit::Oros,
            value: Value::Uno,
        };
        let card = Card {
            suit: Suit::Oros,
            value: Value::Tres,
        };
        assert!(!card.is_perica(marker));

        let marker = Card {
            suit: Suit::Oros,
            value: Value::Uno,
        };
        let card = Card {
            suit: Suit::Bastos,
            value: Value::Sota,
        };
        assert!(!card.is_perica(marker));
    }
}
