mod board;
mod tests;

fn main() {
    let mut board = board::Board::new(String::from("James"), String::from("Jeff"), 15 * 60);
    board.show();
    if let Err(e) = board.move_piece(String::from("D2D4")) {
        println!("Error: {}", e);
    };
    board.show();if let Err(e) = board.move_piece(String::from("E7E5")) {
        println!("Error: {}", e);
    };
    board.show();
}
