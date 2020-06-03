mod pawn;

#[cfg(test)]
use super::board;

#[cfg(test)]
fn create_board() -> board::Board {
    board::Board::new(String::from("James"), String::from("Jeff"), 15 * 60)
}

#[cfg(test)]
mod tests {
    use super::create_board;
    #[test]
    fn move_not_allowed() {
        assert_eq!(create_board().move_piece(String::from("E7E5")), Err("move not allowed"));
    }
}