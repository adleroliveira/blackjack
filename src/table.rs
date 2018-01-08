use deck::Deck;
use player::Player;
use card::{Visibility, Card};
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TableStatus {
    WaitingForPlayers,
    Ready,
    Dealing,
    WatingForPlayer(String)
}

pub enum TableOption {
    NewPlayer,
    Play,
    Quit
}

impl fmt::Display for TableOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TableOption::NewPlayer => write!(f, "[n] New Player"),
            TableOption::Play => write!(f, "[p] Play new round"),
            TableOption::Quit => write!(f, "[q] Quit"),
        }
    }
}

impl TableOption {
    pub fn from_string(c: &str) -> Option<TableOption> {
        match c {
            "n" => Some(TableOption::NewPlayer),
            "p" => Some(TableOption::Play),
            "q" => Some(TableOption::Quit),
            _ => None
        }
    }
}

#[derive(Debug)]
pub struct Table {
    status: TableStatus,
    cards: Deck,
    players: Vec<Player>,
    crupier: Player,
    round: u32,
}

impl Table {
    pub fn open() -> Table {
        let crupier = Player::new(String::from("crupier"));
        let players: Vec<Player> = Vec::new();
        let status = TableStatus::WaitingForPlayers;
        let round = 0;

        let mut cards = Deck::new();
        cards.shuffle();

        Table {
            status,
            cards,
            crupier,
            players,
            round,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        // TODO: check if player with same name exists
        if self.players.len() == 0 && self.status == TableStatus::WaitingForPlayers {
            self.status = TableStatus::Ready;
        }
        self.players.push(player);
    }

    pub fn play_round(&mut self) -> Result<(), &'static str> {
        match self.status {
            TableStatus::Ready => {
                self.round = self.round + 1;

                println!("Starting round {}", self.round);

                for i in 0..self.players.len() {
                    let card_1 = self.take_card(Visibility::Exposed);
                    let card_2 = self.take_card(Visibility::Exposed);

                    Table::deal(&mut self.players[i], card_1);
                    Table::deal(&mut self.players[i], card_2);
                }

                let crupier_card_1 = self.take_card(Visibility::Exposed);
                let crupier_card_2 = self.take_card(Visibility::Hidden);

                Table::deal(&mut self.crupier, crupier_card_1);
                Table::deal(&mut self.crupier, crupier_card_2);

                self.status = TableStatus::WatingForPlayer(self.players[0].name.clone());
                Ok(())
            }
            _ => Err("Table isn't ready"),
        }
    }

    pub fn take_card(&mut self, visibility: Visibility) -> Card {
      // TODO: Refill deck when it's out of cards
      let mut card = self.cards.cards.pop().expect("Deck out of cards");
      if visibility == Visibility::Exposed {
          card.reveal();
      }
      card
    }

    pub fn deal(player: &mut Player, card: Card) {
        println!("{} was dealt a {}", player.name, card.to_string());
        player.receive(card);
    }

    pub fn options(&self) -> Vec<TableOption> {
        let mut options = Vec::new();
        match self.status {

            TableStatus::WaitingForPlayers => {
                options.push(TableOption::NewPlayer);
            }

            TableStatus::Ready => {
                options.push(TableOption::NewPlayer);
                options.push(TableOption::Play);
            }

            _ => {}
        }
        options.push(TableOption::Quit);
        options
    }

    pub fn take_action(&mut self, action: TableOption) {
        match action {
            TableOption::NewPlayer => {
                println!("What's the player's name?");
            }
            _ => {}
        }
    }
}