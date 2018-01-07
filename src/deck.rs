extern crate rand;

use card::{Card, CardType, Suit};
use self::rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::new();
        for i in 0..4 {
            let suit = Suit::from_number(i).expect("Invalid suit number");
            cards.push(Card::new(CardType::Ace(suit.clone()), 1));
            cards.push(Card::new(CardType::Two(suit.clone()), 2));
            cards.push(Card::new(CardType::Three(suit.clone()), 3));
            cards.push(Card::new(CardType::Four(suit.clone()), 4));
            cards.push(Card::new(CardType::Five(suit.clone()), 5));
            cards.push(Card::new(CardType::Six(suit.clone()), 6));
            cards.push(Card::new(CardType::Seven(suit.clone()), 7));
            cards.push(Card::new(CardType::Eight(suit.clone()), 8));
            cards.push(Card::new(CardType::Nine(suit.clone()), 9));
            cards.push(Card::new(CardType::Ten(suit.clone()), 10));
            cards.push(Card::new(CardType::Jack(suit.clone()), 10));
            cards.push(Card::new(CardType::Queen(suit.clone()), 10));
            cards.push(Card::new(CardType::King(suit.clone()), 10));
        }
        Deck { cards }
    }

    pub fn add_new_deck(&mut self) {
      let mut new_deck = Deck::new();
      self.cards.append(&mut new_deck.cards);
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        rng.shuffle(&mut self.cards);
    }
}