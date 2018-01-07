use deck::Deck;
use player::Player;

#[derive(Debug)]
pub struct Table {
  cards: Deck,
  players: Vec<Player>,
  crupier: Player,
  round: u32
}

impl Table {
  pub fn open() -> Table {
    let crupier = Player::new("crupier");
    let players: Vec<Player> = Vec::new();
    let round = 0;

    let mut cards = Deck::new();
    cards.shuffle();

    Table {
      cards,
      crupier,
      players,
      round
    }
  }

  pub fn seat(&mut self, player: Player) {
    self.players.push(player);
  }
}