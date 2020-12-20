pub mod config;
pub mod game;

#[derive(Clone)]
pub struct Player {
    name: String,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player { name }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
