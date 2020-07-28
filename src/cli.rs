use crate::game::{self, Color, GameStatus};
use std::fmt::Display;
use std::io::{stdin, stdout, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

fn input() -> String {
    let mut s = String::new();
    stdin()
        .read_line(&mut s)
        .expect("Please enter a correct string");
    s.trim().to_string()
}

// TODO remove this?
fn _print_input(msg: String) -> String {
    print!("{}", msg);
    stdout().flush().unwrap();
    input()
}

fn get_player_input(
    remaining_time: u64,
    player_name: impl Display,
    player_color: impl Display,
) -> String {
    let mut remaining_time = remaining_time;
    let (sender, receiver): (Sender<String>, Receiver<String>) = channel();
    thread::spawn(move || {
        let c = input();
        sender.send(c).expect("Couldn't write messages");
    });
    let player_msg = format!("{}({})", player_name, player_color);
    loop {
        if let Ok(inp) = receiver.try_recv() {
            return inp;
        }
        let minutes = remaining_time / 60;
        let seconds = remaining_time % 60;
        //save cursor position, move the cursor up, go to the left side
        //print the time and go to saved cursor position
        print!(
            "\x1B[s\x1B[1A\x1B[9D{}    {}m:{}s\x1B[u",
            player_msg, minutes, seconds
        );
        stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        remaining_time -= 1;
    }
}

pub fn run() {
    let mut current_game = game::Game::new(String::from("James"), String::from("Jeff"), 15 * 60);
    while current_game.get_status() == GameStatus::Running {
        current_game.show();
        let current_player = current_game.get_players_clone().get_current();
        if let Err(e) = current_game.move_piece(get_player_input(
            current_player.get_remaining_time(),
            current_player.get_name(),
            current_player.get_color(),
        )) {
            println!("Error: {}", e);
        };
    }
    if current_game.get_status() == GameStatus::Won {
        let player = current_game.get_players_clone();
        println!(
            "{}({}) won the game",
            player.get_current().get_name(),
            match player.get_current().get_color() {
                Color::White => "White",
                Color::Black => "Black",
            }
        )
    }
}
