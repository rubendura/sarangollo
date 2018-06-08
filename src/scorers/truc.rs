use super::Scorer;
use deck::{self, Card, Suit, Value};
use scoreboard;
use std::cmp::Ordering;
use Round;
use Team;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum TrucValue {
    Perico,
    Perica,
    AsEspadas,
    AsBastos,
    SieteEspadas,
    SieteOros,
    Tres,
    Dos,
    AsBobo,
    Rey,
    Caballo,
    Sota,
    SieteBobo,
    Seis,
    Cinco,
    Cuatro,
}

struct TrucCard<'a: 'b, 'b> {
    card: deck::Card,
    round: &'b Round<'a>,
}

impl<'a, 'b> TrucCard<'a, 'b> {
    fn truc_value(&self) -> TrucValue {
        match self.card {
            card if card.is_perico(self.round.marker) => TrucValue::Perico,
            card if card.is_perica(self.round.marker) => TrucValue::Perica,
            Card {
                value: Value::Uno,
                suit: Suit::Espadas,
            } => TrucValue::AsEspadas,
            Card {
                value: Value::Uno,
                suit: Suit::Bastos,
            } => TrucValue::AsBastos,
            Card {
                value: Value::Siete,
                suit: Suit::Espadas,
            } => TrucValue::SieteEspadas,
            Card {
                value: Value::Siete,
                suit: Suit::Oros,
            } => TrucValue::SieteOros,
            Card {
                value: Value::Tres, ..
            } => TrucValue::Tres,
            Card {
                value: Value::Dos, ..
            } => TrucValue::Dos,
            // No need to check the suit for the following cases as any relevant TrucValues where matched for above
            Card {
                value: Value::Uno, ..
            } => TrucValue::AsBobo,
            Card {
                value: Value::Rey, ..
            } => TrucValue::Rey,
            Card {
                value: Value::Caballo,
                ..
            } => TrucValue::Caballo,
            Card {
                value: Value::Sota, ..
            } => TrucValue::Sota,
            Card {
                value: Value::Siete,
                ..
            } => TrucValue::SieteBobo,
            Card {
                value: Value::Seis, ..
            } => TrucValue::Seis,
            Card {
                value: Value::Cinco,
                ..
            } => TrucValue::Cinco,
            Card {
                value: Value::Cuatro,
                ..
            } => TrucValue::Cuatro,
        }
    }
}

impl<'a, 'b> PartialEq for TrucCard<'a, 'b> {
    fn eq(&self, other: &Self) -> bool {
        self.truc_value() == other.truc_value()
    }
}

impl<'a, 'b> Eq for TrucCard<'a, 'b> {}

impl<'a, 'b> Ord for TrucCard<'a, 'b> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.truc_value().cmp(&other.truc_value())
    }
}

impl<'a, 'b> PartialOrd for TrucCard<'a, 'b> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AgreedBet {
    Truc(Option<Team>),
    Retruc(Option<Team>),
    NouVal,
}

#[derive(Default)]
pub struct TrucScorer {
    agreed_bet: Option<AgreedBet>,
}

impl TrucScorer {
    pub fn set_bet(&mut self, agreed_bet: AgreedBet) {
        self.agreed_bet = Some(agreed_bet);
    }
}

impl Scorer for TrucScorer {
    fn get_score(&self, round: &Round) -> Option<scoreboard::RoundScoreSection> {
        // First, map all shown cards to their respective TrucCard so we can compare them
        round.iter_from_hand().map(|(team, seat)| {
            let truc_cards = seat.face_up_cards.iter().map(|card| TrucCard {
                card: *card,
                round: round,
            });
            (team, truc_cards)
        });

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
