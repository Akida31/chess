

#[cfg(test)]
mod tests {
    use super::super::create_board;
    #[test]
    fn valid() {
        let mut board = create_board();
        assert_eq!(board.move_piece(String::from("D2D4")), Ok(()));
    }
    #[test]
    fn invalid() {
        let mut board = create_board();
        assert_eq!(board.move_piece(String::from("D2D5")), Err("move not allowed"));
        assert_eq!(board.move_piece(String::from("D2E3")), Err("move not allowed"));
        assert_eq!(board.move_piece(String::from("D2D5")), Err("move not allowed"));
    }
}