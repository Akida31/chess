use crate::game;
use std::io::{stdin, stdout, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::{Duration, SystemTime};
use crate::game::{GameStatus, Color};

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

fn get_player_input(remaining_time: u64) -> String {
    let mut remaining_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH + Duration::from_secs(remaining_time))
        .unwrap()
        .as_secs();
    let (sender, receiver): (Sender<String>, Receiver<String>) = channel();
    thread::spawn(move || {
        let c = input();
        sender.send(c).expect("Couldn't write messages");
    });
    println!();
    loop {
        if let Ok(inp) = receiver.try_recv() {
            return inp;
        }
        let minutes = remaining_time / 60;
        let seconds = remaining_time % 60;
        //save cursor position, move the cursor up, go to the left side
        //print the time and go to saved cursor position
        print!("\x1B[s\x1B[1A\x1B[9D{:?}m:{:?}s\x1B[u", minutes, seconds);
        stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        remaining_time -= 1;
    }
}

pub fn run() {
    let mut current_game = game::Game::new(String::from("James"), String::from("Jeff"), 15 * 60);
    while current_game.get_status() == GameStatus::Running {
        current_game.show();
        if let Err(e) = current_game.move_piece(get_player_input(current_game.get_players_clone().current.remaining_time)) {
            println!("Error: {}", e);
        };
    }
    if current_game.get_status() == GameStatus::Won {
        let player = current_game.get_players_clone();
        println!("{}({}) won the game", player.current.name, match player.current.color {
            Color::White => "White",
            Color::Black => "Black",
        })
    }
}
