use super::Scorer;
use deck::{self, Card, Suit, Value};
use scoreboard;
use Round;
use Team;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
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

impl TrucValue {
    fn new(card: deck::Card, marker: deck::Card) -> TrucValue {
        match card {
            card if card.is_perico(marker) => TrucValue::Perico,
            card if card.is_perica(marker) => TrucValue::Perica,
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Bet {
    None,
    Truc(Option<Team>),
    Retruc(Option<Team>),
    NouVal,
}

impl Default for Bet {
    fn default() -> Bet {
        Bet::None
    }
}

impl Bet {
    fn get_score(&self) -> u8 {
        match self {
            Bet::None => 1,
            Bet::Truc(_) => 3,
            Bet::Retruc(_) => 6,
            Bet::NouVal => 9,
        }
    }
}

struct AgreedBet {
    bet: Bet,
}

enum BazaWinner {
    Team1,
    Team2,
    Tie,
}

#[derive(Copy, Clone)]
struct BazaCard {
    card: TrucValue,
    team: Team,
}

struct Baza {
    cards: Vec<BazaCard>,
}

impl Baza {
    fn new(baza: &[(deck::Card, Team)], marker: deck::Card) -> Baza {
        let baza_cards = baza.iter()
            .map(|&(card, team)| BazaCard {
                card: TrucValue::new(card, marker),
                team,
            })
            .collect();
        Baza { cards: baza_cards }
    }

    fn winner(&self) -> BazaWinner {
        // If there's no cards it automatically a Tie
        if self.cards.len() == 0 {
            return BazaWinner::Tie;
        }

        let mut truc_cards = self.cards.clone();
        truc_cards.sort_by_key(|baza_card| baza_card.card);
        truc_cards.reverse();

        let highest_card = truc_cards[0].card;

        // Get the highest cards and check the teams.
        // If only one, that's the winner. Otherwise there is a tie
        let mut high_card_teams = truc_cards
            .iter()
            .filter(|baza_card| baza_card.card == highest_card)
            .map(|baza_card| baza_card.team)
            .collect::<Vec<_>>();

        high_card_teams.dedup();

        match high_card_teams.as_slice() {
            [Team::Team1] => BazaWinner::Team1,
            [Team::Team2] => BazaWinner::Team2,
            _ => BazaWinner::Tie,
        }
    }
}

#[derive(Default)]
pub struct TrucScorer {
    agreed_bet: Bet,
}

impl TrucScorer {
    pub fn set_bet(&mut self, agreed_bet: Bet) {
        self.agreed_bet = agreed_bet;
    }
}

impl Scorer for TrucScorer {
    fn get_score(&self, round: &Round) -> Option<scoreboard::RoundScoreSection> {
        // Create bazas

        // Find the seat with the most shown cards
        let max_cards = round
            .seats
            .iter()
            .map(|seat| seat.face_up_cards.len())
            .max();

        // If no players have been sat we can't compute any score
        let max_cards = match max_cards {
            Some(max_cards) => max_cards,
            None => return None,
        };

        // Iterate that seat while taking cards on the same position on other seats (our Bazas)
        let bazas = (0..max_cards)
            .map(|i| {
                round
                    .seats
                    .iter()
                    .enumerate()
                    .filter_map(|(pos, seat)| {
                        if let Some(card) = seat.face_up_cards.get(i) {
                            Some((*card, seat.get_team(pos as u8)))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(Card, Team)>>()
            })
            .map(|baza_cards| Baza::new(baza_cards.as_slice(), round.marker))
            .map(|baza| baza.winner())
            .collect::<Vec<_>>();

        let winner = get_truc_winner(&bazas);

        let winner_score = self.agreed_bet.get_score();

        winner.map(|winner| scoreboard::RoundScoreSection(winner, winner_score))
    }
}

fn get_truc_winner(bazas: &[BazaWinner]) -> Option<Team> {
    bazas
        .iter()
        .scan((0, 0), |state, baza_winner| {
            Some(match baza_winner {
                BazaWinner::Team1 => (state.0 + 1, state.1),
                BazaWinner::Team2 => (state.0, state.1 + 1),
                BazaWinner::Tie => (state.0 + 1, state.1 + 1),
            })
        })
        .filter_map(|state| match state {
            (team1, team2) if team1 == team2 => None,
            (team1, _) if team1 >= 2 => Some(Team::Team1),
            (_, score2) if score2 >= 2 => Some(Team::Team2),
            _ => None,
        })
        .nth(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_baza_winner_no_cards() {
        // TrucScorer::get_baza_winner([], _) == Tie
    }

    #[test]
    fn get_baza_winner_tie() {
        unimplemented!()
    }

    #[test]
    fn get_baza_winner_tie_tree_different_teams() {
        unimplemented!()
    }

    #[test]
    fn get_baza_winner_tie_three_same_team() {
        unimplemented!()
    }

}
