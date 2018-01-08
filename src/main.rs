extern crate blackjack;
use blackjack::table::{Table, TableOption};
use std::io;

fn main() {
    println!("Welcome to Toy Blackjack");
    let table = Table::open();

    loop {
        println!("\n\nChoose an option:");

        let options = table.options();
        for option in options.iter() {
            println!("{}", option.to_string())
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.pop();

        let action: TableOption = match TableOption::from_string(input.as_str()) {
            Some(option) => {
                println!("Nice: {}", option);
                option
            }
            None  => {
                println!("Invalid Option");
                continue;
            }
        };
    }
}