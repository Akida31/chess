mod tests {
    use super::super::create_game;
    #[test]
    fn valid() {
        let mut game = create_game();
        assert_eq!(game.move_piece(String::from("D2D4")), Ok(()));
    }
    #[test]
    fn invalid() {
        let mut game = create_game();
        assert_eq!(
            game.move_piece(String::from("D2D5")),
            Err("move not allowed")
        );
        assert_eq!(
            game.move_piece(String::from("D2E3")),
            Err("move not allowed")
        );
        assert_eq!(
            game.move_piece(String::from("D2D5")),
            Err("move not allowed")
        );
    }
}
