use crate::Player;

pub struct Config {
    pub white: Player,
    pub black: Player,
}

impl Config {
    pub fn new(white: String, black: String) -> Self {
        let white = Player::new(white);
        let black = Player::new(black);
        Self { white, black }
    }
}
