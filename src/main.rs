use std::io::{stdin, stdout, Write};

mod game;

#[cfg(test)]
mod tests;

fn input() -> String {
    let mut s = String::new();
    stdin()
        .read_line(&mut s)
        .expect("Please enter a correct string");
    s.trim().to_string()
}

fn print_input(msg: String) -> String {
    print!("{}", msg);
    stdout().flush().unwrap();
    input()
}

fn main() {
    let mut game = game::Game::new(String::from("James"), String::from("Jeff"), 15 * 60);
    game.show();
    if let Err(e) = game.move_piece(String::from("D2D4")) {
        println!("Error: {}", e);
    };
    game.show();
    if let Err(e) = game.move_piece(String::from("E7E5")) {
        println!("Error: {}", e);
    };
    game.show();
    if let Err(e) = game.move_piece(String::from("G2G3")) {
        println!("Error: {}", e);
    };
    game.show();
}
