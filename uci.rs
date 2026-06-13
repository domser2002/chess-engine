use std::io::{self, BufRead, Write};
use crate::piece::*;
use crate::position::{self, *};
use crate::r#move::*;

/// Run the UCI protocol loop — reads commands from stdin until "quit".
pub fn uci_loop() {
    let stdin = io::stdin();
    let mut position = start_position();

    println!("id name chess-engine");
    println!("id author domser");
    println!("uciok");
    io::stdout().flush().unwrap();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        let trimmed = line.trim();

        if trimmed == "quit" {
            break;
        }

        if trimmed == "uci" {
            println!("id name chess-engine");
            println!("id author domser");
            println!("uciok");
            io::stdout().flush().unwrap();
            continue;
        }

        if trimmed == "isready" {
            println!("readyok");
            io::stdout().flush().unwrap();
            continue;
        }

        if trimmed == "ucinewgame" {
            position = start_position();
            continue;
        }

        if trimmed.starts_with("position") {
            parse_position(&mut position, trimmed);
            continue;
        }

        if trimmed == "go" || trimmed.starts_with("go ") {
            match crate::search::best_move(position) {
                Some(m) => {
                    let uci_str = move_to_uci(m);
                    println!("bestmove {uci_str}");
                }
                None => {
                    println!("bestmove 0000");
                }
            }
            io::stdout().flush().unwrap();
            continue;
        }
    }
}

/// Parse a "position" command:
///   position startpos
///   position startpos moves e2e4 e7e5 ...
fn parse_position(position: &mut Position, line: &str) {
    let rest = line.strip_prefix("position ").unwrap_or(line);

    if rest.starts_with("startpos") {
        *position = start_position();
        let moves_part = rest.strip_prefix("startpos").unwrap_or("");
        let moves_part = moves_part.trim();
        if let Some(moves_str) = moves_part.strip_prefix("moves ") {
            for move_str in moves_str.split_whitespace() {
                if let Some(m) = parse_move_uci(move_str, position) {
                    make_move(position, m);
                }
            }
        }
    } else if rest.starts_with("fen") {
        // FEN parsing not implemented yet — reset to start as fallback
        *position = start_position();
    }
}

/// Convert a Move to UCI notation (e.g. "e2e4", "e7e8q").
fn move_to_uci(m: Move) -> String {
    use crate::square::get_square_name;

    if m.from == SHORT_CASTLING_SQUARE {
        return "e1g1".to_string();
    }
    if m.from == LONG_CASTLING_SQUARE {
        // Determine color from piece
        return if m.piece.piece_color == PieceColor::White {
            "e1c1".to_string()
        } else {
            "e8c8".to_string()
        };
    }

    let from_name = get_square_name(m.from);
    let to_name = get_square_name(m.to);
    let mut s = from_name + &to_name;

    if let Some(new_piece) = m.new_piece {
        let promo_char = match new_piece.piece_type {
            PieceType::Queen => 'q',
            PieceType::Rook => 'r',
            PieceType::Bishop => 'b',
            PieceType::Knight => 'n',
            _ => unreachable!(),
        };
        s.push(promo_char);
    }

    s
}