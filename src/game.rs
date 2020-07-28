use std::fmt;
use std::ops::Add;
use std::time::{Duration, SystemTime};

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
pub enum Color {
    Black,
    White,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Color::Black => "Black",
            Color::White => "White",
        })
    }
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

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    remaining_time: u64,
    color: Color,
}

impl Player {
    fn new(name: String, time: u64, color: Color) -> Player {
        Player {
            name,
            remaining_time: time,
            color,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_remaining_time(&self) -> u64 {
        self.remaining_time
    }

    pub fn get_color(&self) -> Color {
        self.color.clone()
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

#[derive(Clone)]
pub struct Players {
    current: Player,
    waiting: Player,
}

impl Players {
    pub fn get_current(&self) -> Player {
        self.current.clone()
    }
}

#[derive(Clone, PartialEq)]
pub enum GameStatus {
    Running,
    Won,
    Paused,
}

#[derive(Clone)]
pub struct Game {
    fields: Vec<Vec<Option<Field>>>,
    last_update: u64,
    status: GameStatus,
    players: Players,
}

impl Game {
    pub fn new(player_name1: String, player_name2: String, time: u64) -> Game {
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
        Game {
            fields,
            status: GameStatus::Running,
            players: Players {
                current: Player::new(player_name1, time, Color::White),
                waiting: Player::new(player_name2, time, Color::Black),
            },
            last_update: 0,
        }
    }

    pub fn show(&self) {
        let (player1, player2) = if self.players.current.color == Color::White {
            (
                String::from("> ") + &self.players.current.name.clone() + "<",
                String::from("  ") + &self.players.waiting.name.clone(),
            )
        } else {
            (
                String::from("  ") + &self.players.waiting.name.clone(),
                String::from("> ") + &self.players.current.name.clone() + "<",
            )
        };
        println!("Players:\n  White: {}\n  Black: {}", player1, player2);
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
        println!();
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
        if field.is_none() {
            return false;
        }
        let field = field.as_ref().unwrap();
        if the_move.player != &(field.color) {
            return false;
        }
        let diff_x = (the_move.start.0 as i8 - the_move.end.0 as i8).abs();
        let diff_y = (the_move.start.1 as i8 - the_move.end.1 as i8).abs();
        if diff_y == 0 && diff_x == 0 {
            return false;
        }
        if !match field.piece {
            Piece::Pawn => {
                the_move.start.0 == the_move.end.0
                    && ((the_move.start.1 as i8 - the_move.end.1 as i8).abs() == 1
                        || ((the_move.start.1 == 1 || the_move.start.1 == 6)
                            && (the_move.start.1 as i8 - the_move.end.1 as i8).abs() == 2))
            }
            Piece::King => (diff_x <= 1 && diff_y <= 1),
            Piece::Knight => ((diff_x == 1 && diff_y == 2) || (diff_x == 2 && diff_y == 1)),
            Piece::Rook => (diff_x == 0 || diff_y == 0),
            Piece::Bishop => (diff_x == diff_y),
            Piece::Queen => (diff_x == diff_y || (diff_x == 0 || diff_y == 0)),
        } {
            return false;
        }
        let chessed = self.check_chess();
        for player in [&self.players.current, &self.players.waiting].iter() {
            if &player.color != the_move.player && chessed == Some(&player.color) {
                println!("{} is chess!", player.name);
            }
        }
        self.check_chess() != Some(the_move.player)
    }

    fn check_chess(&self) -> Option<&Color> {
        for (end_y, row) in self.fields.iter().enumerate() {
            for (end_x, field) in row.iter().enumerate() {
                if let Some(field) = field {
                    if field.piece == Piece::King {
                        for (start_y, row) in self.fields.iter().enumerate() {
                            for (start_x, enemy) in row.iter().enumerate() {
                                if let Some(enemy) = enemy {
                                    if enemy.color != field.color
                                        && self.check_move(Move::new(
                                            &enemy.color,
                                            (start_x as u8, start_y as u8),
                                            (end_x as u8, end_y as u8),
                                        ))
                                    {
                                        println!("found");
                                        return Some(&field.color);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        None
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
        let start_y = matching(chars.next())?;
        let char = chars.next();
        let end_x = match letters.chars().position(|x| Some(x) == char) {
            Some(c) => c as u8,
            None => return Err("piece not found"),
        };
        let end_y = matching(chars.next())?;
        // TODO remove this var?
        let _current_player = if !self.check_move(Move::new(
            // TODO check the design of check_move
            &self.players.current.color,
            (start_x, start_y),
            (end_x, end_y),
        )) {
            return Err("move not allowed");
        };
        let start_x = start_x as usize;
        let start_y = start_y as usize;
        self.fields[end_y as usize][end_x as usize] = self.fields[start_y][start_x].clone();
        self.fields[start_y][start_x] = None;
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH.add(Duration::from_secs(self.last_update)))
            .unwrap()
            .as_secs();
        self.players.current.remaining_time = self.players.current.remaining_time - time;
        self.status = GameStatus::Running;
        std::mem::swap(&mut self.players.waiting, &mut self.players.current);
        self.last_update = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Ok(())
    }

    pub fn get_status(&self) -> GameStatus {
        self.status.clone()
    }

    pub fn get_players_clone(&self) -> Players {
        self.players.clone()
    }
}

fn matching(inp: Option<char>) -> Result<u8, &'static str> {
    match inp {
        Some(x) => match x.to_digit(10) {
            Some(c) => Ok((8 - c) as u8),
            None => return Err("invalid move format"),
        },
        None => return Err("invalid move format"),
    }
}
