use crate::config::Config;
use crate::Player;
use std::convert::TryInto;
use std::fmt;
use Color::{Black, White};
/// All Pieces
#[derive(Copy, Clone)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

/// The colors of both players
/// It can be formatted nicely
#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    Black,
    White,
}

/// A non-empty field of the board.
/// It can be formatted nicely
#[derive(Copy, Clone)]
pub struct Field {
    piece: Piece,
    color: Color,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.color {
                White => match self.piece {
                    Piece::King => "♚",
                    Piece::Queen => "♛",
                    Piece::Rook => "♜",
                    Piece::Bishop => "♝",
                    Piece::Knight => "♞",
                    Piece::Pawn => "♟",
                },
                Black => match self.piece {
                    Piece::King => "♔",
                    Piece::Queen => "♕",
                    Piece::Rook => "♖",
                    Piece::Bishop => "♗",
                    Piece::Knight => "♘",
                    Piece::Pawn => "♙",
                },
            }
        )
    }
}

impl Field {
    fn new(piece: Piece, color: Color) -> Field {
        Field { piece, color }
    }
}

/// The state of the Game
/// keeping the configuration of the players
/// and the board
pub struct Game {
    board: [[Option<Field>; 8]; 8],
    black: Player,
    white: Player,
    current_player: Color,
}

impl Game {
    pub fn new(config: Config) -> Game {
        // create an empty board
        let mut board = [[None; 8]; 8];
        let pieces = [
            Piece::Rook,
            Piece::Knight,
            Piece::Bishop,
            Piece::Queen,
            Piece::King,
            Piece::Bishop,
            Piece::Knight,
            Piece::Rook,
        ];
        // add pawns to board
        board[1] = [Some(Field::new(Piece::Pawn, Black)); 8];
        board[7] = [Some(Field::new(Piece::Pawn, White)); 8];

        // add the pieces to board
        board[0] = pieces
            .clone()
            .iter()
            .map(|x| Some(Field::new(*x, Black)))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|_| panic!("wrong length of vec"));

        board[7] = pieces
            .iter()
            .map(|x| Some(Field::new(*x, White)))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|_| panic!("wrong length of vec"));

        Game {
            board,
            white: config.white,
            black: config.black,
            current_player: White,
        }
    }

    /// check if a player is chess and return its color
    pub fn check_chess(&self) -> Option<Color> {
        None
    }

    /// check if the piece at (x, y) can move to (x2, y2)
    /// the moves are NOT checked for chess
    fn check_move_possibility_to(&self, the_move: &Move) -> bool {
        let (start_x, start_y, end_x, end_y) = the_move.inner();
        let field = match self.board[start_y][start_x] {
            Some(p) => p,
            None => return false,
        };
        let player = field.color;
        match field.piece {
            Piece::Pawn => {
                let direction = match player {
                    White => -1,
                    Black => 1,
                };

                // normal move forward
                if start_y as isize + direction == end_y as isize
                    && start_x == end_x
                    && self.board[end_y][end_x].is_none()
                {
                    return true;
                }

                // double move
                if (start_y as isize - direction == 0 || start_y as isize - direction == 7)
                    && start_y as isize + 2 * direction == end_y as isize
                    && start_x == end_x
                    && self.board[(start_y as isize + direction) as usize][start_x].is_none()
                    && self.board[end_y][end_x].is_none()
                {
                    return true;
                }

                // the pawn must move one to the side on a hit
                if (end_x as isize - start_x as isize).abs() != 1 {
                    return false;
                }

                if let Some(field) = self.board[end_y][end_x] {
                    if field.color != player {
                        return true;
                    }
                }
                false

                // TODO: check en passante
                // to do this create a move history
            }
            Piece::Rook => {
                let dx = start_x as isize - end_x as isize;
                let dy = start_y as isize - end_y as isize;
                if dy == 0 {
                    let direction = if dx > 0 { 1 } else { 0 };
                    for i in 0..dx.abs() {
                        if self.board[start_y][(start_x as isize + direction * i) as usize]
                            .is_some()
                        {
                            return false;
                        }
                    }
                } else if dx == 0 {
                    let direction = if dy > 0 { 1 } else { 0 };
                    for i in 0..dy.abs() {
                        if self.board[(start_y as isize + direction * i) as usize][start_x]
                            .is_some()
                        {
                            return false;
                        }
                    }
                } else {
                    return false;
                }
                true
            }
            Piece::Knight => {
                (start_x as isize - end_x as isize).abs()
                    * (start_y as isize - end_y as isize).abs()
                    == 2
            }
            Piece::Bishop => {
                let dx = start_x as isize - end_x as isize;
                let dy = start_y as isize - end_y as isize;
                if dx.abs() != dy.abs() {
                    return false;
                }
                let dir_x = if dx > 0 { 1 } else { 0 };
                let dir_y = if dy > 0 { 1 } else { 0 };
                for i in 0..dx.abs() {
                    if self.board[(start_y as isize + dir_y * i) as usize]
                        [(start_x as isize + dir_x * i) as usize]
                        .is_some()
                    {
                        return false;
                    }
                }
                true
            }
            // TODO: check king distance
            // TODO: check castling
            Piece::King => {
                let dx = start_x as isize - end_x as isize;
                let dy = start_y as isize - end_y as isize;
                dx.abs() + dy.abs() == 1
            }
            Piece::Queen => {
                let dx = start_x as isize - end_x as isize;
                let dy = start_y as isize - end_y as isize;
                if dy == 0 {
                    let direction = if dx > 0 { 1 } else { 0 };
                    for i in 0..dx.abs() {
                        if self.board[start_y][(start_x as isize + direction * i) as usize]
                            .is_some()
                        {
                            return false;
                        }
                    }
                } else if dx == 0 {
                    let direction = if dy > 0 { 1 } else { 0 };
                    for i in 0..dy.abs() {
                        if self.board[(start_y as isize + direction * i) as usize][start_x]
                            .is_some()
                        {
                            return false;
                        }
                    }
                } else if dx.abs() == dy.abs() {
                    let dir_x = if dx > 0 { 1 } else { 0 };
                    let dir_y = if dy > 0 { 1 } else { 0 };
                    for i in 0..dx.abs() {
                        if self.board[(start_y as isize + dir_y * i) as usize]
                            [(start_x as isize + dir_x * i) as usize]
                            .is_some()
                        {
                            return false;
                        }
                    }
                } else {
                    return false;
                }
                true
            }
        }
    }

    /// check if a move is valid
    pub fn check_move(&self, the_move: &Move) -> Result<(), &'static str> {
        let player = self.current_player;
        if let Some(color) = self.check_chess() {
            if color == player {
                return Err("player would be chess after move");
            }
        };
        if the_move.start_y >= 8 || the_move.start_x >= 8 {
            return Err("move outside of the board");
        }
        if the_move.start_x == the_move.end_x && the_move.start_y == the_move.end_y {
            return Err("you have to move");
        }
        if let Some(field) = self.board[the_move.start_y][the_move.start_x] {
            if field.color != player {
                return Err("not an own piece to move");
            }
        } else {
            return Err("not an own piece to move");
        }
        if !self.check_move_possibility_to(the_move) {
            return Err("invalid move");
        }
        Ok(())
    }

    /// check if a move is valid and then do it
    pub fn do_move(&mut self, the_move: Move) -> Result<(), &'static str> {
        self.check_move(&the_move)?;
        let piece = self.board[the_move.start_y][the_move.start_x].take();
        self.board[the_move.end_y][the_move.end_x] = piece;
        self.current_player = match self.current_player {
            Black => White,
            White => Black,
        };
        Ok(())
    }
}

pub struct Move {
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}

impl Move {
    pub fn new(start_x: usize, start_y: usize, end_x: usize, end_y: usize) -> Move {
        Move {
            start_x,
            start_y,
            end_x,
            end_y,
        }
    }

    /// destruct the move into inner
    /// returns (start_x, start_y, end_x, end_y)
    pub fn inner(&self) -> (usize, usize, usize, usize) {
        (self.start_x, self.start_y, self.end_x, self.end_y)
    }
}
