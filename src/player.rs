use card::Card;
use table::Table;

#[derive(Debug)]
pub struct Player {
  cards: Vec<Card>,
  name: &'static str
}

impl Player {
  pub fn new(name: &'static str) -> Player {
    let cards: Vec<Card> = Vec::new();
    Player{ cards, name }
  }

  pub fn seat(self, table: &mut Table) {
    table.seat(self);
  }
}