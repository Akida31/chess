mod board;

fn main() {
    let mut board = board::Board::new(String::from("James"), String::from("Jeff"), 15 * 60);
    board.show();
    println!("{:?}", board.move_piece(String::from("D2D4")));
    board.show();
    println!("{:?}", board.move_piece(String::from("E7E5")));
    board.show();
}
