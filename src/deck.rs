use rand::{self, Rng};

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

#[derive(Clone, Eq, PartialEq)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        rng.shuffle(&mut self.cards);
    }
}

impl Default for Deck {
    fn default() -> Self {
        let mut cards = vec![];
        for (&suit, &value) in iproduct!(SUITS.into_iter(), VALUES.into_iter()) {
            cards.push(Card { suit, value })
        }
        Self { cards }
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

    #[test]
    fn shuffle() {
        let deck: Deck = Default::default();
        let mut deck2 = deck.clone();
        deck2.shuffle();
        assert!(deck != deck2);
    }
}
