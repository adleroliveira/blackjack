use card::Card;
use table::Table;

#[derive(Debug)]
pub struct Player {
  cards: Vec<Card>,
  pub name: String
}

impl Player {
  pub fn new(name: String) -> Player {
    let cards: Vec<Card> = Vec::new();
    Player{ cards, name }
  }

  pub fn seat_at(self, table: &mut Table) {
    table.add_player(self);
  }

  pub fn receive(&mut self, card: Card) {
    self.cards.push(card);
  }
}