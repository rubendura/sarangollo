use itertools::Itertools;
use deck::Card;

fn is_flor(cards: &[&Card; 3]) -> bool {
    cards.iter().map(|card| card.suit).all_equal()
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
