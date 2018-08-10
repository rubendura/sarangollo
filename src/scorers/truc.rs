use super::Scorer;
use deck::{self, Card, Suit, Value};
use scoreboard;
use Round;
use Team;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum TrucValue {
    Cuatro,
    Cinco,
    Seis,
    SieteBobo,
    Sota,
    Caballo,
    Rey,
    AsBobo,
    Dos,
    Tres,
    SieteOros,
    SieteEspadas,
    AsBastos,
    AsEspadas,
    Perica,
    Perico,
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum BazaWinner {
    Team1,
    Team2,
    Parda,
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
        let baza_cards = baza
            .iter()
            .map(|&(card, team)| BazaCard {
                card: TrucValue::new(card, marker),
                team,
            })
            .collect();
        Baza { cards: baza_cards }
    }

    fn winner(&self) -> BazaWinner {
        // If there's no cards it is automatically Parda
        if self.cards.is_empty() {
            return BazaWinner::Parda;
        }

        let mut truc_cards = self.cards.clone();
        truc_cards.sort_by_key(|baza_card| baza_card.card);
        truc_cards.reverse();

        let highest_card = truc_cards[0].card;

        // Get the highest cards and check the teams.
        // If only one, that's the winner. Otherwise there is Parda
        let mut high_card_teams = truc_cards
            .iter()
            .filter(|baza_card| baza_card.card == highest_card)
            .map(|baza_card| baza_card.team)
            .collect::<Vec<_>>();

        high_card_teams.dedup();

        match high_card_teams.as_slice() {
            [Team::Team1] => BazaWinner::Team1,
            [Team::Team2] => BazaWinner::Team2,
            _ => BazaWinner::Parda,
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
    let winner = bazas
        .iter()
        .scan((0, 0), |state, baza_winner| {
            let score = match baza_winner {
                BazaWinner::Team1 => (state.0 + 1, state.1),
                BazaWinner::Team2 => (state.0, state.1 + 1),
                BazaWinner::Parda => (state.0 + 1, state.1 + 1),
            };
            *state = score;
            Some(score)
        })
        .filter_map(|(team1, team2)| {
            if team1 == team2 {
                None
            } else if team1 >= 2 && team1 > team2 {
                Some(Team::Team1)
            } else if team2 >= 2 && team2 > team1 {
                Some(Team::Team2)
            } else {
                None
            }
        })
        .nth(0);

    // Special case: Team1 wins baza 1, Team2 wins baza 2, then Parda
    // In this case the winner is whoever won the first baza
    if winner.is_none() && bazas.len() >= 3 {
        if bazas.iter().all(|&b| b == BazaWinner::Parda) {
            return None;
        }
        match bazas[0] {
            BazaWinner::Team1 => Some(Team::Team1),
            BazaWinner::Team2 => Some(Team::Team2),
            _ => unreachable!(
                "It should not be possible to finish Truc in a draw when the first baza was not parda"
            ),
        }
    } else {
        winner
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_runner;

    #[test]
    fn compare_truc_value() {
        assert!(TrucValue::AsEspadas > TrucValue::Cinco);
    }

    #[test]
    fn get_baza_winner_no_cards() {
        let baza = Baza::new(
            &[],
            Card {
                value: Value::Rey,
                suit: Suit::Bastos,
            },
        );
        assert_eq!(BazaWinner::Parda, baza.winner());
    }

    #[test]
    fn get_baza_winner_parda() {
        let baza = Baza::new(
            &[
                (
                    Card {
                        value: Value::Tres,
                        suit: Suit::Bastos,
                    },
                    Team::Team1,
                ),
                (
                    Card {
                        value: Value::Tres,
                        suit: Suit::Copas,
                    },
                    Team::Team2,
                ),
            ],
            Card {
                value: Value::Rey,
                suit: Suit::Bastos,
            },
        );
        assert_eq!(BazaWinner::Parda, baza.winner());
    }

    #[test]
    fn get_baza_winner_one_card() {
        let baza = Baza::new(
            &[(
                Card {
                    value: Value::Tres,
                    suit: Suit::Bastos,
                },
                Team::Team2,
            )],
            Card {
                value: Value::Rey,
                suit: Suit::Bastos,
            },
        );
        assert_eq!(BazaWinner::Team2, baza.winner());
    }

    #[test]
    fn get_baza_winner_dedup() {
        let baza = Baza::new(
            &[
                (
                    Card {
                        value: Value::Tres,
                        suit: Suit::Bastos,
                    },
                    Team::Team1,
                ),
                (
                    Card {
                        value: Value::Dos,
                        suit: Suit::Bastos,
                    },
                    Team::Team2,
                ),
                (
                    Card {
                        value: Value::Tres,
                        suit: Suit::Bastos,
                    },
                    Team::Team1,
                ),
            ],
            Card {
                value: Value::Rey,
                suit: Suit::Bastos,
            },
        );
        assert_eq!(BazaWinner::Team1, baza.winner());
    }

    #[test]
    fn get_baza_winner() {
        let baza = Baza::new(
            &[
                (
                    Card {
                        value: Value::Tres,
                        suit: Suit::Bastos,
                    },
                    Team::Team1,
                ),
                (
                    Card {
                        value: Value::Tres,
                        suit: Suit::Copas,
                    },
                    Team::Team2,
                ),
                (
                    Card {
                        value: Value::Sota,
                        suit: Suit::Bastos,
                    },
                    Team::Team1,
                ),
                (
                    Card {
                        value: Value::Caballo,
                        suit: Suit::Bastos,
                    },
                    Team::Team2,
                ),
                (
                    Card {
                        value: Value::Cuatro,
                        suit: Suit::Bastos,
                    },
                    Team::Team1,
                ),
                (
                    Card {
                        value: Value::Dos,
                        suit: Suit::Espadas,
                    },
                    Team::Team2,
                ),
            ],
            Card {
                value: Value::Rey,
                suit: Suit::Bastos,
            },
        );
        assert_eq!(BazaWinner::Team2, baza.winner());
    }

    #[test]
    fn get_truc_winner_no_baza() {
        let winner = get_truc_winner(&[]);
        assert_eq!(None, winner);
    }

    #[test]
    fn get_truc_winner_one_baza() {
        let winner = get_truc_winner(&[BazaWinner::Team1]);
        assert_eq!(None, winner);

        let winner = get_truc_winner(&[BazaWinner::Team2]);
        assert_eq!(None, winner);

        let winner = get_truc_winner(&[BazaWinner::Parda]);
        assert_eq!(None, winner);
    }

    #[test]
    fn get_truc_winner_two_bazas_table_test() {
        let test_cases = [
            test_runner::TestCase {
                description: "Team1 wins two first bazas",
                input: [BazaWinner::Team1, BazaWinner::Team1],
                expected: Some(Team::Team1),
            },
            test_runner::TestCase {
                description: "Team2 wins two first bazas",
                input: [BazaWinner::Team2, BazaWinner::Team2],
                expected: Some(Team::Team2),
            },
            test_runner::TestCase {
                description: "One baza each",
                input: [BazaWinner::Team1, BazaWinner::Team2],
                expected: None,
            },
            test_runner::TestCase {
                description: "Two Pardas",
                input: [BazaWinner::Parda, BazaWinner::Parda],
                expected: None,
            },
            // If first Parda, next one wins
            test_runner::TestCase {
                description: "Parda, then Team2",
                input: [BazaWinner::Parda, BazaWinner::Team2],
                expected: Some(Team::Team2),
            },
            // If second one Parda, previous wins
            test_runner::TestCase {
                description: "Team1, then Parda",
                input: [BazaWinner::Team1, BazaWinner::Parda],
                expected: Some(Team::Team1),
            },
        ];
        let runner = test_runner::run(&test_cases, |input, expected| {
            let result = get_truc_winner(&input);
            assert_eq!(expected, result);
        });
    }

    #[test]
    fn get_truc_winner_three_bazas_table_test() {
        let test_cases = [
            test_runner::TestCase {
                description: "Team1 wins all bazas (last one irrelevant)",
                input: [BazaWinner::Team1, BazaWinner::Team1, BazaWinner::Team1],
                expected: Some(Team::Team1),
            },
            test_runner::TestCase {
                description: "Team1 wins first two bazas, looses last (last one irrelevant)",
                input: [BazaWinner::Team1, BazaWinner::Team1, BazaWinner::Team2],
                expected: Some(Team::Team1),
            },
            test_runner::TestCase {
                description: "Team1 looses first, wins other 2",
                input: [BazaWinner::Team2, BazaWinner::Team1, BazaWinner::Team1],
                expected: Some(Team::Team1),
            },
            test_runner::TestCase {
                description: "Team1 wins first and last",
                input: [BazaWinner::Team1, BazaWinner::Team2, BazaWinner::Team1],
                expected: Some(Team::Team1),
            },
            test_runner::TestCase {
                description: "Parda, then Team2",
                input: [BazaWinner::Parda, BazaWinner::Team2, BazaWinner::Team1],
                expected: Some(Team::Team2),
            },
            test_runner::TestCase {
                description: "Parda, Parda, then Team2",
                input: [BazaWinner::Parda, BazaWinner::Parda, BazaWinner::Team2],
                expected: Some(Team::Team2),
            },
            // TODO confirm this
            test_runner::TestCase {
                description: "Parda, Parda, Parda",
                input: [BazaWinner::Parda, BazaWinner::Parda, BazaWinner::Parda],
                expected: None,
            },
            test_runner::TestCase {
                description: "Team2, then Parda",
                input: [BazaWinner::Team2, BazaWinner::Parda, BazaWinner::Team1],
                expected: Some(Team::Team2),
            },
            // TODO confirm this test (should win first baza?)
            test_runner::TestCase {
                description: "Team2, Team1, then Parda",
                input: [BazaWinner::Team2, BazaWinner::Team1, BazaWinner::Parda],
                expected: Some(Team::Team2),
            },
        ];
        let runner = test_runner::run(&test_cases, |input, expected| {
            let result = get_truc_winner(&input);
            assert_eq!(expected, result);
        });
    }

}
