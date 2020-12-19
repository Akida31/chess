pub mod config;
pub mod game;

pub struct Player {
    name: String,
}

impl Player {
    fn new(name: String) -> Player {
        Player { name }
    }
}
