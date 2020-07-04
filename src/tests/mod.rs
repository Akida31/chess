mod pawn;

use super::game;

fn create_game() -> game::Game {
    game::Game::new(String::from("James"), String::from("Jeff"), 15 * 60)
}

#[test]
fn move_not_allowed() {
    assert_eq!(
        create_game().move_piece(String::from("E7E5")),
        Err("move not allowed")
    );
}
