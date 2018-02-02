#[macro_use]
extern crate itertools;

mod deck;
mod scoreboard;

use scoreboard::Scoreboard;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Team {
    Team1,
    Team2,
}

#[derive(Debug)]
struct Game {
    team1: Vec<Player>,
    team2: Vec<Player>,
    scoreboard: Scoreboard,
}

enum JoinError {
    FullTeam(Team),
}

impl Game {
    fn new() -> Self {
        Game {
            team1: vec![],
            team2: vec![],
            scoreboard: Scoreboard::new(),
        }
    }

    fn join(&mut self, player: Player, team: Team) -> Result<(), JoinError> {
        let (chosen_team, other_team) = match team {
            Team::Team1 => (&mut self.team1, &mut self.team2),
            Team::Team2 => (&mut self.team2, &mut self.team1),
        };

        // If player already belongs to team we're done
        if chosen_team.contains(&player) {
            return Ok(());
        }

        // Teams are composed of three players. Not sure this is the best way to enforce this.
        if chosen_team.len() >= 3 {
            return Err(JoinError::FullTeam(team));
        }

        // If player was already in the other team, remove it from there
        let pos = other_team.iter().position(|x| *x == player);
        if let Some(pos) = pos {
            other_team.remove(pos);
        }

        // Finally add player to team
        chosen_team.push(player);

        Ok(())
    }
}

struct PlayerPiles<'p> {
    player: &'p Player,
    hand: Vec<deck::Card>,
    up_facing: Vec<deck::Card>,
}

impl<'p> PlayerPiles<'p> {
    fn new(player: &'p Player) -> PlayerPiles<'p> {
        PlayerPiles {
            player: player,
            hand: vec![],
            up_facing: vec![],
        }
    }

    fn deal(&mut self, cards: Vec<deck::Card>) {
        self.hand = cards;
    }

    fn discard(&mut self, card: deck::Card) {
        let pos = self.hand.iter().position(|x| *x == card);
        if let Some(pos) = pos {
            self.hand.remove(pos);
        }
    }

    fn show(&mut self, card: deck::Card) {
        let pos = self.hand.iter().position(|&x| x == card);
        if let Some(pos) = pos {
            self.hand.remove(pos);
            self.up_facing.push(card);
        }
    }
}

struct GameRoundState<'p> {
    deck: deck::SpanishDeck,
    player_piles: Vec<PlayerPiles<'p>>,
}

#[derive(Debug, Eq, PartialEq)]
struct Player {
    name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discard_card_from_hand() {
        let player = Player {
            name: "Ruben".to_string(),
        };
        let mut piles = PlayerPiles::new(&player);
        let card = deck::Card {
            suit: deck::Suit::Bastos,
            value: deck::Value::Caballo,
        };
        piles.deal(vec![card]);
        piles.discard(card);
        assert_eq!(piles.hand, vec![]);
        assert_eq!(piles.up_facing, vec![]);
    }

    #[test]
    fn show_card_from_hand() {
        let player = Player {
            name: "Ruben".to_string(),
        };
        let mut piles = PlayerPiles::new(&player);
        let card = deck::Card {
            suit: deck::Suit::Bastos,
            value: deck::Value::Caballo,
        };
        piles.deal(vec![card]);
        piles.show(card);
        assert_eq!(piles.hand, vec![]);
        assert_eq!(piles.up_facing, vec![card]);
    }

    #[test]
    fn join_team() {
        let mut game = Game {
            team1: vec![],
            team2: vec![],
            scoreboard: Scoreboard::new(),
        };
        let player = Player {
            name: "Ruben".into(),
        };
        let success = game.join(player, Team::Team1);
        assert!(success.is_ok());
        assert_eq!(game.team1.len(), 1);
    }

    #[test]
    fn join_team_move() {
        let player = Player {
            name: "Ruben".into(),
        };
        let mut game = Game {
            team1: vec![
                Player {
                    name: "Ruben".into(),
                },
            ],
            team2: vec![],
            scoreboard: Scoreboard::new(),
        };
        let success = game.join(player, Team::Team2);
        assert!(success.is_ok());
        assert_eq!(game.team1.len(), 0);
        assert_eq!(game.team2.len(), 1);
    }

    #[test]
    fn join_team_full() {
        let player = Player {
            name: "Ruben".into(),
        };
        let mut game = Game {
            team1: vec![
                Player {
                    name: "Roser".into(),
                },
                Player {
                    name: "Whisky".into(),
                },
                Player {
                    name: "Rateta".into(),
                },
            ],
            team2: vec![],
            scoreboard: Scoreboard::new(),
        };
        let success = game.join(player, Team::Team1);
        assert!(success.is_err());
        assert_eq!(game.team1.len(), 3);
        assert_eq!(game.team2.len(), 0);
    }

    #[test]
    fn join_team_existing() {
        let player = Player {
            name: "Ruben".into(),
        };
        let mut game = Game {
            team1: vec![
                Player {
                    name: "Ruben".into(),
                },
            ],
            team2: vec![],
            scoreboard: Scoreboard::new(),
        };
        let success = game.join(player, Team::Team1);
        assert!(success.is_ok());
        assert_eq!(game.team1.len(), 1);
        assert_eq!(game.team2.len(), 0);
    }

}
