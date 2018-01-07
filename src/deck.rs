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
            cards.push(Card::new(CardType::Ace(Suit::from_number(i)), 1));
            cards.push(Card::new(CardType::Two(Suit::from_number(i)), 2));
            cards.push(Card::new(CardType::Three(Suit::from_number(i)), 3));
            cards.push(Card::new(CardType::Four(Suit::from_number(i)), 4));
            cards.push(Card::new(CardType::Five(Suit::from_number(i)), 5));
            cards.push(Card::new(CardType::Six(Suit::from_number(i)), 6));
            cards.push(Card::new(CardType::Seven(Suit::from_number(i)), 7));
            cards.push(Card::new(CardType::Eight(Suit::from_number(i)), 8));
            cards.push(Card::new(CardType::Nine(Suit::from_number(i)), 9));
            cards.push(Card::new(CardType::Ten(Suit::from_number(i)), 10));
            cards.push(Card::new(CardType::Jack(Suit::from_number(i)), 10));
            cards.push(Card::new(CardType::Queen(Suit::from_number(i)), 10));
            cards.push(Card::new(CardType::King(Suit::from_number(i)), 10));
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