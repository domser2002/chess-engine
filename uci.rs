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
            // Parse go parameters
            use crate::search::SearchConfig;
            let mut config = SearchConfig::default();
            config.uci_output = true;

            if let Some(params) = trimmed.strip_prefix("go ") {
                let tokens: Vec<&str> = params.split_whitespace().collect();
                for (i, token) in tokens.iter().enumerate() {
                    match *token {
                        "wtime" => {
                            if let Some(val) = tokens.get(i + 1).and_then(|s| s.parse::<u64>().ok()) {
                                if position.turn == PieceColor::White {
                                    config.time_per_move_ms = val / 40;
                                }
                            }
                        }
                        "btime" => {
                            if let Some(val) = tokens.get(i + 1).and_then(|s| s.parse::<u64>().ok()) {
                                if position.turn == PieceColor::Black {
                                    config.time_per_move_ms = val / 40;
                                }
                            }
                        }
                        "depth" => {
                            if let Some(val) = tokens.get(i + 1).and_then(|s| s.parse::<i32>().ok()) {
                                config.max_depth = val;
                                config.time_per_move_ms = 60_000;
                            }
                        }
                        _ => {}
                    }
                }
            }

            let result = crate::search::search_with_time(position, config);
            match result.best_move {
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
    } else if rest.starts_with("fen ") {
        let fen_and_moves = rest.strip_prefix("fen ").unwrap_or("");
        // FEN has 4–6 space-separated fields; "moves" keyword separates them
        let (fen_str, moves_str) = if let Some(idx) = fen_and_moves.find(" moves ") {
            (&fen_and_moves[..idx], Some(&fen_and_moves[idx + 7..]))
        } else {
            (fen_and_moves, None)
        };

        if let Some(pos) = position::parse_fen(fen_str) {
            *position = pos;
        }

        if let Some(moves) = moves_str {
            for move_str in moves.split_whitespace() {
                if let Some(m) = parse_move_uci(move_str, position) {
                    make_move(position, m);
                }
            }
        }
    }
}

/// Convert a Move to UCI notation (e.g. "e2e4", "e7e8q").
pub fn move_to_uci(m: Move) -> String {
    use crate::square::get_square_name;

    if m.from == SHORT_CASTLING_SQUARE {
        return if m.piece.piece_color == PieceColor::White {
            "e1g1".to_string()
        } else {
            "e8g8".to_string()
        };
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