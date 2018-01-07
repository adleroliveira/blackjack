use deck::Deck;
use player::Player;
use card::{Visibility, Card};

#[derive(Debug)]
pub enum TableStatus {
    Idle,
    Dealing,
    WatingForPlayer(Player)
}

#[derive(Debug)]
pub struct Table {
    name: String,
    status: TableStatus,
    cards: Deck,
    players: Vec<Player>,
    crupier: Player,
    round: u32,
}

impl Table {
    pub fn open(name: String) -> Table {
        let crupier = Player::new(String::from("crupier"));
        let players: Vec<Player> = Vec::new();
        let status = TableStatus::Idle;
        let round = 0;

        let mut cards = Deck::new();
        cards.shuffle();

        Table {
            name,
            status,
            cards,
            crupier,
            players,
            round,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn start_round(&mut self) -> Result<(), &'static str> {
        match self.status {
            TableStatus::Idle => {
                for i in 0..self.players.len() {
                    let card = self.take_card(Visibility::Exposed);
                    Table::deal(&mut self.players[i], card);
                }
                let crupier_card = self.take_card(Visibility::Exposed);
                Table::deal(&mut self.crupier, crupier_card);
                self.round = self.round + 1;
                Ok(())
            }
            _ => Err("Table is busy"),
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
}