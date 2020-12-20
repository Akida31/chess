use crate::config::Config;
use crate::Player;
use std::convert::TryInto;
use Color::{Black, White};

/// All Pieces
#[derive(Copy, Clone, PartialEq)]
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

impl Field {
    fn new(piece: Piece, color: Color) -> Field {
        Field { piece, color }
    }

    /// destruct the field into its piece and color
    pub fn inner(&self) -> (Piece, Color) {
        (self.piece, self.color)
    }
}

/// the chess board
pub type Board = [[Option<Field>; 8]; 8];

/// The state of the Game
/// keeping the configuration of the players
/// and the board
pub struct Game {
    board: Board,
    black: Player,
    white: Player,
    current_player: Color,
    state: GameState,
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
        board[6] = [Some(Field::new(Piece::Pawn, White)); 8];

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
            state: GameState::Running,
        }
    }

    /// get the player names
    pub fn get_players(&self) -> (Player, Player) {
        (self.white.clone(), self.black.clone())
    }

    /// check if a player is chess and return its color
    pub fn check_chess(&self) -> Option<Color> {
        Game::check_chess_of_board(self.board)
    }

    /// check if a player no this board is chess and return its color
    fn check_chess_of_board(board: Board) -> Option<Color> {
        // get position of the kings
        // and all other pieces
        let mut pieces = Vec::new();
        let mut white_king = None;
        let mut black_king = None;
        for (y, row) in board.iter().enumerate() {
            for (x, field) in row.iter().enumerate() {
                if let Some(field) = field {
                    if field.piece == Piece::King {
                        match field.color {
                            White => white_king = Some((x, y)),
                            Black => black_king = Some((x, y)),
                        }
                    } else {
                        pieces.push((x, y, field.color));
                    }
                }
            }
        }

        // check if a piece can move on the field of the enemy king
        for (x, y, color) in pieces {
            // get the position of the enemy king
            let (kx, ky) = match color {
                White => black_king,
                Black => white_king,
            }
            .unwrap(); // safe because the board has to contain both kings
            let the_move = Move::new(x, y, kx, ky);
            if Game::check_move_possibility_to(board, &the_move) {
                return Some(color);
            }
        }
        None
    }

    /// check if the piece at (x, y) can move to (x2, y2)
    /// the moves are NOT checked for chess
    fn check_move_possibility_to(board: Board, the_move: &Move) -> bool {
        let (start_x, start_y, end_x, end_y) = the_move.inner();
        let field = match board[start_y][start_x] {
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
                    && board[end_y][end_x].is_none()
                {
                    return true;
                }

                // double move
                if (start_y as isize - direction == 0 || start_y as isize - direction == 7)
                    && start_y as isize + 2 * direction == end_y as isize
                    && start_x == end_x
                    && board[(start_y as isize + direction) as usize][start_x].is_none()
                    && board[end_y][end_x].is_none()
                {
                    return true;
                }

                // the pawn must move one to the side on a hit
                if (end_x as isize - start_x as isize).abs() != 1 {
                    return false;
                }

                if let Some(field) = board[end_y][end_x] {
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
                        if board[start_y][(start_x as isize + direction * i) as usize].is_some() {
                            return false;
                        }
                    }
                } else if dx == 0 {
                    let direction = if dy > 0 { 1 } else { 0 };
                    for i in 0..dy.abs() {
                        if board[(start_y as isize + direction * i) as usize][start_x].is_some() {
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
                    if board[(start_y as isize + dir_y * i) as usize]
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
                        if board[start_y][(start_x as isize + direction * i) as usize].is_some() {
                            return false;
                        }
                    }
                } else if dx == 0 {
                    let direction = if dy > 0 { 1 } else { 0 };
                    for i in 0..dy.abs() {
                        if board[(start_y as isize + direction * i) as usize][start_x].is_some() {
                            return false;
                        }
                    }
                } else if dx.abs() == dy.abs() {
                    let dir_x = if dx > 0 { 1 } else { 0 };
                    let dir_y = if dy > 0 { 1 } else { 0 };
                    for i in 0..dx.abs() {
                        if board[(start_y as isize + dir_y * i) as usize]
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
        self.unwrap_end()?;
        let player = self.current_player;
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
        if !Game::check_move_possibility_to(self.board, the_move) {
            return Err("invalid move");
        }
        let mut board = self.board;
        board = Game::move_piece(the_move, board);
        if let Some(color) = Game::check_chess_of_board(board) {
            if color == player {
                return Err("player would be chess after move");
            }
        };
        Ok(())
    }

    /// the actual moving of the piece
    fn move_piece(the_move: &Move, mut board: Board) -> Board {
        let piece = board[the_move.start_y][the_move.start_x].take();
        board[the_move.end_y][the_move.end_x] = piece;
        board
    }

    /// check if a move is valid and then do it
    pub fn do_move(&mut self, the_move: Move) -> Result<(), &'static str> {
        self.unwrap_end()?;
        self.check_move(&the_move)?;
        self.board = Game::move_piece(&the_move, self.board);
        self.current_player = match self.current_player {
            Black => White,
            White => Black,
        };
        Ok(())
    }

    /// return the current state of the Game
    pub fn state(&mut self) -> GameState {
        self.state
    }

    /// return the current board
    pub fn get_board(&self) -> Board {
        self.board
    }

    /// return the current player
    pub fn get_current_player(&self) -> Color {
        self.current_player
    }

    /// returns an Err if the game has already ended
    fn unwrap_end(&self) -> Result<(), &'static str> {
        if let GameState::End(_) = self.state {
            Err("the game has already ended")
        } else {
            Ok(())
        }
    }

    /// the current player resigns
    pub fn resign(&mut self) -> Result<(), &'static str> {
        self.unwrap_end()?;
        self.state = GameState::End(GameEnd::Resign(self.current_player));
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub enum GameState {
    Running,
    End(GameEnd),
}

#[derive(Copy, Clone)]
pub enum GameEnd {
    Checkmate(Color),
    Resign(Color),
    Draw,
    // TODO include all other GameEnds
    // https://en.wikipedia.org/wiki/Rules_of_chess#End_of_the_game
}

/// a single move
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
