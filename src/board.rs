use std::fmt;
use std::collections::HashMap;
use std::convert::TryInto;

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
            None => write!(f, " ")
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
        Field {
            piece,
            color,
        }
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

#[derive(Debug)]
pub struct Board {
    fields: Vec<Vec<Field>>,
    players: [Player; 2],
}


impl Board {
    pub fn new(player_name1: String, player_name2: String, time: u16) -> Board {
        let mut fields = vec![vec![Field::empty(); 8]; 8];
        fields[0] = vec![Piece::Rook, Piece::Knight, Piece::Bishop, Piece::Queen, Piece::King,
                         Piece::Bishop, Piece::Knight, Piece::Rook].into_iter().map(|x| { Field::new(Some(x), Some(Color::Black)) }).collect();
        fields[1] = vec![Field::new(Some(Piece::Pawn), Some(Color::Black)); 8];
        fields[6] = vec![Field::new(Some(Piece::Pawn), Some(Color::White)); 8];
        fields[7] = vec![Piece::Rook, Piece::Knight, Piece::Bishop, Piece::King, Piece::Queen,
                         Piece::Bishop, Piece::Knight, Piece::Rook].into_iter().map(|x| Field::new(Some(x), Some(Color::White))).collect();
        Board {
            fields,
            players: [Player::new(player_name1, time), Player::new(player_name2, time)],
        }
    }

    pub fn show(&self) {
        println!("   ABCDEFGHI");
        for (i, row) in self.fields.iter().enumerate() {
            print!("{}. ", i+1);
            for field in row {
                print!("{:#?}", field);
            }
            println!();
        }
    }

    fn check_move(&self, piece: Piece, start: u8, end: u8) -> bool {
        true
    }

    pub fn move_piece(&mut self, the_move: String) -> Result<(), ()> {
        if the_move.len() != 4 {
            return Err(());
        }
        let mut chars = the_move.chars();

        let start_y = match chars.nth(1) {
            Some(x) => {
                match x.to_digit(10) {
                    Some(c) => c,
                    None => return Err(())
                }
            },
            None => return Err(())
        };
        let end_y = match chars.nth(3) {
            Some(x) => {
                match x.to_digit(10) {
                    Some(c) => c,
                    None => return Err(())
                }
            },
            None => return Err(())
        };
        Ok(())
    }
}
