extern crate blackjack;
use blackjack::table::Table;
use blackjack::player::Player;

fn main() {
    let table_name = String::from("table1");
    let player_name = String::from("p1");

    let mut table = Table::open(table_name);
    let p1 = Player::new(player_name);

    p1.seat_at(&mut table);

    table.start_round().unwrap();
    println!("{:#?}", table);
}