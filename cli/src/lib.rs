use chess_core::config::Config;
use chess_core::game::{Board, Color, Field, Game, GameEnd, GameState, Move as GameMove, Piece};
use chess_core::Player;
use std::io::Write;

/// a convenience macro allowing to print before inputting
macro_rules! input {
    () => {{
        let mut buffer = String::new();
        ::std::io::stdin()
            .read_line(&mut buffer)
            .expect("Error reading input");
        buffer.trim().to_string()
    }};
    ($($arg:tt)*) => {{
        print!("{}", format_args!($($arg)*));
        ::std::io::stdout().flush().expect("Error writing output");
        input!()
    }};
}

/// get the player names through input (stdin)
fn get_player_names() -> (String, String) {
    println!("Please input your names");
    let white = input!("White: ");
    let black = input!("Black: ");
    println!();
    (white, black)
}

/// parse the move if possible
fn parse_move(the_move: &str) -> Option<GameMove> {
    if the_move.len() != 4 {
        return None;
    }
    let mut the_move = the_move.chars();

    let next = the_move.next()?;
    let start_x = if next.is_digit(10) {
        return None;
    } else {
        next.to_digit(18)? as usize - 10
    };
    let start_y = the_move.next()?.to_digit(10)? as usize;
    let next = the_move.next()?;
    let end_x = if next.is_digit(10) {
        return None;
    } else {
        next.to_digit(18)? as usize - 10
    };
    let end_y = the_move.next()?.to_digit(10)? as usize;
    Some(GameMove::new(start_x, start_y, end_x, end_y))
}

/// get a move from player input (stdin)
fn get_move(player: Player) -> Move {
    loop {
        let the_move = input!("{}: ", player.get_name()).to_lowercase();
        if the_move == "resign" {
            return Move::Resign;
        }
        if let Some(the_move) = parse_move(&the_move) {
            return Move::Move(the_move);
        }

        println!("invalid input! try again");
    }
}

/// render the board to stdout
fn render_board(board: Board, players: &(Player, Player)) {
    println!("Black: {}", players.0.get_name());
    for (y, row) in board.iter().enumerate() {
        print!("{}", 8 - y);
        for field in row {
            match field {
                Some(p) => print!(" {}", fmt_field(p)),
                None => print!("  "),
            }
        }
        println!();
    }
    println!("  a b c d e f g h");
    println!("White: {}\n", players.1.get_name());
}

fn fmt_field(field: &Field) -> &'static str {
    let (piece, color) = field.inner();
    match color {
        Color::White => match piece {
            Piece::King => "♚",
            Piece::Queen => "♛",
            Piece::Rook => "♜",
            Piece::Bishop => "♝",
            Piece::Knight => "♞",
            Piece::Pawn => "♟",
        },
        Color::Black => match piece {
            Piece::King => "♔",
            Piece::Queen => "♕",
            Piece::Rook => "♖",
            Piece::Bishop => "♗",
            Piece::Knight => "♘",
            Piece::Pawn => "♙",
        },
    }
}

fn fmt_color(color: Color) -> &'static str {
    match color {
        Color::White => "White",
        Color::Black => "Black",
    }
}

/// entry point for the cli
pub fn run() {
    let (white, black) = get_player_names();
    let config = Config::new(white, black);
    let mut game = Game::new(config);
    loop {
        // check if the game has ended
        if let GameState::End(e) = game.state() {
            print!("The game has ended: ");
            match e {
                GameEnd::Resign(c) => println!("{} resigned", fmt_color(c)),
                GameEnd::Draw => println!("Draw"),
                GameEnd::Checkmate(c) => println!("{} is checkmate", fmt_color(c)),
            }
            break;
        }
        let players = game.get_players();
        render_board(game.get_board(), &players);
        let current_player = if game.get_current_player() == Color::White {
            players.0
        } else {
            players.1
        };
        let res = match get_move(current_player) {
            Move::Move(the_move) => game.do_move(the_move),
            Move::Resign => game.resign(),
        };
        if let Err(e) = res {
            eprintln!("ERROR: {}", e);
        }
    }
}

enum Move {
    Move(GameMove),
    Resign,
}
