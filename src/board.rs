use std::fmt;

use crate::board::Color::{Black, White};

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum Color {
    Black,
    White,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Field {
    piece: Piece,
    color: Color,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.color {
                Color::White => match self.piece {
                    Piece::King => "♚",
                    Piece::Queen => "♛",
                    Piece::Rook => "♜",
                    Piece::Bishop => "♝",
                    Piece::Knight => "♞",
                    Piece::Pawn => "♟",
                },
                Color::Black => match self.piece {
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

#[derive(Debug)]
struct Player {
    name: String,
    remaining_time: u16,
}

impl Player {
    fn new(name: String, time: u16) -> Player {
        Player {
            name,
            remaining_time: time,
        }
    }
}

struct Move<'a> {
    player: &'a Color,
    start: (u8, u8),
    end: (u8, u8),
}

impl Move<'_> {
    fn new(player: &Color, start: (u8, u8), end: (u8, u8)) -> Move {
        Move { player, start, end }
    }
}

pub struct Board {
    fields: Vec<Vec<Option<Field>>>,
    players: [Player; 2],
    current_player: Color,
}

impl Board {
    pub fn new(player_name1: String, player_name2: String, time: u16) -> Board {
        let mut fields = vec![vec![None; 8]; 8];
        let pieces = vec![
            Piece::Rook,
            Piece::Knight,
            Piece::Bishop,
            Piece::Queen,
            Piece::King,
            Piece::Bishop,
            Piece::Knight,
            Piece::Rook,
        ];
        fields[0] = pieces
            .clone()
            .into_iter()
            .map(|x| Some(Field::new(x, Color::Black)))
            .collect();
        fields[1] = vec![Some(Field::new(Piece::Pawn, Color::Black)); 8];
        fields[6] = vec![Some(Field::new(Piece::Pawn, Color::White)); 8];
        fields[7] = pieces
            .into_iter()
            .map(|x| Some(Field::new(x, Color::White)))
            .collect();
        Board {
            fields,
            players: [
                Player::new(player_name1, time),
                Player::new(player_name2, time),
            ],
            current_player: White,
        }
    }

    pub fn show(&self) {
        // TODO show the time of the players
        println!("Players:\n  White: {}\n  Black: {}", self.players[0].name, self.players[1].name);
        println!("Board:\n   ABCDEFGH");
        for (i, row) in self.fields.iter().enumerate() {
            print!("{}. ", 8 - i);
            for field in row {
                if let Some(field) = field {
                    print!("{}", field);
                } else {
                    print!(" ");
                };
            }
            println!();
        }
        println!("Current Player: {:?}\n", self.current_player);
    }

    fn check_move(&self, the_move: Move) -> bool {
        if !(the_move.start.0 < 8
            && the_move.end.0 < 8
            && the_move.start.1 < 8
            && the_move.end.1 < 8)
        {
            return false;
        }
        let field = &self.fields[the_move.start.1 as usize][the_move.start.0 as usize];
        if let None = field {
            return false;
        }
        let field = field.as_ref().unwrap();
        if the_move.player != &(field.color) {
            return false;
        }
        match field.piece {
            Piece::Pawn => {
                the_move.start.0 == the_move.end.0
                    && ((the_move.start.1 as i8 - the_move.end.1 as i8).abs() == 1
                        || ((the_move.start.1 == 1 || the_move.start.1 == 6)
                            && (the_move.start.1 as i8 - the_move.end.1 as i8).abs() == 2))
            }
            _ => true,
        }
    }

    pub fn move_piece(&mut self, the_move: String) -> Result<(), &str> {
        if the_move.len() != 4 {
            return Err("invalid move format");
        }
        let chars = the_move.to_lowercase();
        let mut chars = chars.chars();
        let letters = "abcdefgh";
        let char = chars.next();
        let start_x = match letters.chars().position(|x| Some(x) == char) {
            Some(c) => c as u8,
            None => return Err("piece not found"),
        };
        let start_y = match chars.next() {
            Some(x) => match x.to_digit(10) {
                Some(c) => (8 - c) as u8,
                None => return Err("invalid move format"),
            },
            None => return Err("invalid move format"),
        };
        let char = chars.next();
        let end_x = match letters.chars().position(|x| Some(x) == char) {
            Some(c) => c as u8,
            None => return Err("piece not found"),
        };
        let end_y = match chars.next() {
            Some(x) => match x.to_digit(10) {
                Some(c) => (8 - c) as u8,
                None => return Err("invalid move format"),
            },
            None => return Err("invalid move format"),
        };
        if !self.check_move(Move::new(
            &self.current_player,
            (start_x, start_y),
            (end_x, end_y),
        )) {
            return Err("move not allowed");
        }
        let start_x = start_x as usize;
        let start_y = start_y as usize;
        self.fields[end_y as usize][end_x as usize] = self.fields[start_y][start_x].clone();
        self.fields[start_y][start_x] = None;
        self.current_player = match self.current_player {
            White => Black,
            Black => White,
        };
        Ok(())
    }
}
