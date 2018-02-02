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


mod spanish_deck {
    use super::Deck;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Suit {
        Bastos,
        Copas,
        Oros,
        Espadas,
    }

    static SUITS: [Suit; 4] = [Suit::Oros, Suit::Copas, Suit::Bastos, Suit::Espadas];

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    fn create_deck() {}
}
