extern crate blackjack;
use blackjack::table::Table;
use blackjack::player::Player;

fn main() {
    let mut table = Table::open();
    let p = Player::new("p1");
    p.seat(&mut table);

    println!("{:#?}", table);
}