use deck::Deck;
use player::Player;

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
  round: u32
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
      round
    }
  }

  pub fn add_player(&mut self, player: Player) {
    self.players.push(player);
  }

  pub fn start_round(&mut self) -> Result<(), &'static str> {
    match self.status {
        TableStatus::Idle => {
          for p in &mut self.players {
            // TODO: Refill deck when it's out of cards
            let mut card = self.cards.cards.pop().expect("Deck out of cards");
            card.reveal();
            println!("Player {} was dealt a {}", p.name, card.to_string());
            p.take_card(card);
          }

          let mut card = self.cards.cards.pop().expect("Deck out of cards");
          card.reveal();
          println!("Player {} was dealt a {}", self.crupier.name, card.to_string());
          self.crupier.take_card(card);
          Ok(())
        }
        _ => Err("Table is busy")
    }
  }
}