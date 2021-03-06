use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Color {
    Red,
    Black
}

#[derive(Debug, Clone, PartialEq)]
pub enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades
}

impl Suit {
    pub fn color(&self) -> Color {
        match *self {
            Suit::Diamonds => Color::Red,
            Suit::Clubs => Color::Black,
            Suit::Hearts => Color::Red,
            Suit::Spades => Color::Black
        }
    }

    pub fn from_number(n: u32) -> Option<Suit> {
        match n {
            0 => Some(Suit::Diamonds),
            1 => Some(Suit::Clubs),
            2 => Some(Suit::Hearts),
            3 => Some(Suit::Spades),
            _ => None
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CardType {
    Ace(Suit),
    Two(Suit),
    Three(Suit),
    Four(Suit),
    Five(Suit),
    Six(Suit),
    Seven(Suit),
    Eight(Suit),
    Nine(Suit),
    Ten(Suit),
    Jack(Suit),
    Queen(Suit),
    King(Suit),
    Joker(Color)
}

#[derive(Debug, PartialEq)]
pub enum Visibility {
  Hidden,
  Exposed
}

#[derive(Debug, PartialEq)]
pub struct Card {
    card_type: CardType,
    visibility: Visibility,
    value: u32
}

impl Card {
    pub fn new(card_type: CardType, value: u32) -> Card {
        let visibility = Visibility::Hidden;
        Card {
            card_type,
            visibility,
            value
        }
    }

    pub fn reveal(&mut self) {
      match self.visibility {
          Visibility::Hidden => self.visibility = Visibility::Exposed,
          Visibility::Exposed => println!("card {} is already exposed", self.to_string())
      }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.visibility {
            Visibility::Hidden => write!(f, "Hidden Card"),
            Visibility::Exposed => write!(f, "{:?}", self.card_type),
        }
    }
}