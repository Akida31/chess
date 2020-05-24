use std::collections::HashMap;
use std::fmt;

use crate::board::Color::White;

#[derive(Clone, Hash, PartialEq, Eq)]
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
    piece: Option<Piece>,
    color: Option<Color>,
}

impl Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut pieces_white: HashMap<Piece, &str> = HashMap::new();
        pieces_white.insert(Piece::King, "♔");
        pieces_white.insert(Piece::Queen, "♕");
        pieces_white.insert(Piece::Rook, "♖");
        pieces_white.insert(Piece::Bishop, "♗");
        pieces_white.insert(Piece::Knight, "♘");
        pieces_white.insert(Piece::Pawn, "♙");
        let mut pieces_black: HashMap<Piece, &str> = HashMap::new();
        pieces_black.insert(Piece::King, "♚");
        pieces_black.insert(Piece::Queen, "♛");
        pieces_black.insert(Piece::Rook, "♜");
        pieces_black.insert(Piece::Bishop, "♝");
        pieces_black.insert(Piece::Knight, "♞");
        pieces_black.insert(Piece::Pawn, "♟");
        let mut pieces: HashMap<Color, HashMap<Piece, &str>> = HashMap::new();
        pieces.insert(Color::White, pieces_white);
        pieces.insert(Color::Black, pieces_black);
        match &self.piece {
            Some(piece) => write!(f, "{}", pieces[&self.color.as_ref().unwrap()][piece]),
            None => write!(f, " "),
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

impl Field {
    fn new(piece: Option<Piece>, color: Option<Color>) -> Field {
        Field { piece, color }
    }

    fn empty() -> Field {
        Field {
            piece: None,
            color: None,
        }
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

#[derive(Debug)]
pub struct Board {
    fields: Vec<Vec<Field>>,
    players: [Player; 2],
    current_player: Color,
}

impl Board {
    pub fn new(player_name1: String, player_name2: String, time: u16) -> Board {
        let mut fields = vec![vec![Field::empty(); 8]; 8];
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
            .map(|x| Field::new(Some(x), Some(Color::Black)))
            .collect();
        fields[1] = vec![Field::new(Some(Piece::Pawn), Some(Color::Black)); 8];
        fields[6] = vec![Field::new(Some(Piece::Pawn), Some(Color::White)); 8];
        fields[7] = pieces
            .into_iter()
            .map(|x| Field::new(Some(x), Some(Color::White)))
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
        println!("   ABCDEFGH");
        for (i, row) in self.fields.iter().enumerate() {
            print!("{}. ", i + 1);
            for field in row {
                print!("{:#?}", field);
            }
            println!();
        }
    }

    fn check_move(&self, the_move: Move) -> bool {
        the_move.start.1 < 8 && the_move.end.1 < 8
    }

    pub fn move_piece(&mut self, the_move: String) -> Result<(), &str> {
        if the_move.len() != 4 {
            return Err("invalid move format");
        }
        let mut chars = the_move.to_lowercase();
        let mut chars = chars.chars();
        let letters = "abcdefgh";
        let start_x = match letters.chars().position(|x| Some(x) == chars.nth(0)) {
            Some(c) => c as u8,
            None => return Err("piece not found"),
        };
        let start_y = match chars.next() {
            Some(x) => match x.to_digit(10) {
                Some(c) => (c - 1) as u8,
                None => return Err("invalid move format"),
            },
            None => return Err("invalid move format"),
        };
        let end_x = match letters.chars().position(|x| Some(x) == chars.next()) {
            Some(c) => c as u8,
            None => return Err("piece not found"),
        };
        let end_y = match chars.next() {
            Some(x) => match x.to_digit(10) {
                Some(c) => (c - 1) as u8,
                None => return Err("invalid move format"),
            },
            None => return Err("invalid move format"),
        };
        if !self.check_move(Move::new(&self.current_player, (start_x, start_y), (end_x, end_y))) {
            return Err("move not allowed");
        }
        let start_x = start_x as usize;
        let start_y = start_y as usize;
        self.fields[end_y as usize][end_x as usize] = self.fields[start_y][start_x].clone();
        self.fields[start_y][start_x] = Field::empty();
        Ok(())
    }
}
