mod board;

fn main() {
    let board = board::Board::new(String::from("James"), String::from("Jeff"), 15*60);
    board.show();
}
