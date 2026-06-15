use std::io::{self, BufRead, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};
use crate::piece::*;
use crate::position::{self, *};
use crate::r#move::*;

/// Run the UCI protocol loop — reads commands from stdin until "quit" or EOF.
pub fn uci_loop() {
    let stdin = io::stdin();
    let mut position = start_position();
    let stop = Arc::new(AtomicBool::new(false));
    let mut search_handle: Option<JoinHandle<()>> = None;

    let mut log_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/chess_uci.log")
        .unwrap_or_else(|_| std::fs::File::create("/dev/null").unwrap());
        
    writeln!(log_file, "--- ENGINE STARTED ---").ok();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }
        
        writeln!(log_file, "> {}", trimmed).ok();

        if trimmed == "quit" {
            stop.store(true, Ordering::Relaxed);
            if let Some(handle) = search_handle.take() {
                let _ = handle.join();
            }
            break;
        }

        if trimmed == "uci" {
            println!("id name chess-engine");
            println!("id author domser");
            println!("uciok");
            writeln!(log_file, "< id name chess-engine").ok();
            writeln!(log_file, "< id author domser").ok();
            writeln!(log_file, "< uciok").ok();
            io::stdout().flush().unwrap();
            continue;
        }

        if trimmed == "isready" {
            println!("readyok");
            writeln!(log_file, "< readyok").ok();
            io::stdout().flush().unwrap();
            continue;
        }

        if trimmed == "ucinewgame" {
            position = start_position();
            continue;
        }

        if trimmed == "stop" {
            stop.store(true, Ordering::Relaxed);
            if let Some(handle) = search_handle.take() {
                let _ = handle.join();
            }
            continue;
        }

        if trimmed.starts_with("position") {
            parse_position(&mut position, trimmed);
            continue;
        }

        if trimmed == "go" || trimmed.starts_with("go ") {
            // Wait for any previous search to finish
            if let Some(handle) = search_handle.take() {
                let _ = handle.join();
            }

            stop.store(false, Ordering::Relaxed);

            use crate::search::SearchConfig;
            let mut config = SearchConfig::default();
            config.uci_output = true;

            if let Some(params) = trimmed.strip_prefix("go ") {
                let tokens: Vec<&str> = params.split_whitespace().collect();
                let mut is_infinite = false;
                for (i, token) in tokens.iter().enumerate() {
                    match *token {
                        "infinite" => {
                            is_infinite = true;
                        }
                        "movetime" => {
                            if let Some(val) = tokens.get(i + 1).and_then(|s| s.parse::<u64>().ok()) {
                                config.time_per_move_ms = val;
                            }
                        }
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
                if is_infinite {
                    // Effectively infinite — engine searches until `stop` is received
                    config.time_per_move_ms = u64::MAX / 2;
                }
            }

            let pos = position; // Position is Copy
            let stop_clone = stop.clone();
            search_handle = Some(thread::spawn(move || {
                let result = crate::search::search_with_time(pos, config, stop_clone);
                let uci_str = match result.best_move {
                    Some(m) => move_to_uci(m),
                    None => "0000".to_string(),
                };
                let bestmove_msg = format!("bestmove {}", uci_str);
                println!("{}", bestmove_msg);
                io::stdout().flush().unwrap();
                
                let mut log_file = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("/tmp/chess_uci.log")
                    .unwrap_or_else(|_| std::fs::File::create("/dev/null").unwrap());
                std::io::Write::write_fmt(&mut log_file, format_args!("< {}\n", bestmove_msg)).ok();
            }));
            continue;
        }

        // Unknown commands are silently ignored (per UCI spec)
    }

    // If stdin closed (EOF), wait for any running search to finish
    if let Some(handle) = search_handle.take() {
        let _ = handle.join();
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