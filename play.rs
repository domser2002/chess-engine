use crate::r#move::{parse_move_algebraic, print_move};
use crate::piece::PieceColor;
use crate::position::{self, is_in_check, legal_moves, make_move, start_position};
use std::io::{self, BufRead, Write};

/// Run an interactive game where the user types moves in algebraic notation
/// and the engine responds with the best move found by 1-ply search.
#[cfg_attr(test, allow(dead_code))]
pub fn interactive_loop() {
    let mut position = start_position();
    let mut move_count = 0;
    let mut move_time_ms = 10_000; // Default to 10 seconds

    println!("Chess Engine — Interactive Mode");
    println!("Type moves in algebraic notation (e4, Nf3, exd5, 0-0, f8=Q)");
    println!("Type 'time <seconds>' to change the engine's thinking time (e.g., 'time 5')");
    println!("Type 'quit' to exit, 'moves' to see all legal moves\n");

    loop {
        // Print the board
        position::print_position(position);

        // Check game status
        if position.turn == PieceColor::White {
            println!("\nWhite's turn");
        } else {
            println!("\nBlack's turn");
        }

        // Check if game is over
        let mut moves = crate::r#move::MoveList::new();
        legal_moves(position, &mut moves);
        if moves.is_empty() {
            let in_check = is_in_check(position, position.turn);
            let side = match position.turn {
                PieceColor::White => "White",
                PieceColor::Black => "Black",
            };
            println!(
                "Game over after {move_count} moves. {}{}",
                side,
                if in_check {
                    " is checkmated!"
                } else {
                    " is stalemated."
                }
            );
            break;
        }

        // Read user input
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().lock().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if input == "quit" || input == "exit" {
                    println!("Bye!");
                    break;
                }
                if input.starts_with("time ") {
                    if let Ok(secs) = input[5..].trim().parse::<u64>() {
                        move_time_ms = secs * 1000;
                        println!("Engine thinking time set to {secs} seconds.");
                    } else {
                        println!("Invalid time format. Use 'time <seconds>'");
                    }
                    continue;
                }
                if input == "moves" {
                    for &m in moves.as_slice() {
                        println!("  {}", print_move(m));
                    }
                    continue;
                }
                if input.is_empty() {
                    continue;
                }

                // Parse the move
                match parse_move_algebraic(input, &position) {
                    Some(user_move) => {
                        let s = print_move(user_move);
                        make_move(&mut position, user_move);
                        move_count += 1;
                        println!("\nYour move: {s}");

                        // Check if game is over after user's move
                        position::print_position(position);
                        let mut opponent_moves = crate::r#move::MoveList::new();
                        legal_moves(position, &mut opponent_moves);
                        if opponent_moves.is_empty() {
                            let in_check = is_in_check(position, position.turn);
                            let side = match position.turn {
                                PieceColor::White => "White",
                                PieceColor::Black => "Black",
                            };
                            println!(
                                "Game over after {move_count} moves. {}{}",
                                side,
                                if in_check {
                                    " is checkmated!"
                                } else {
                                    " is stalemated."
                                }
                            );
                            break;
                        }

                        // Engine's turn — iterative deepening with time management
                        let mut config: crate::search::SearchConfig = Default::default();
                        config.time_per_move_ms = move_time_ms;
                        let result = crate::search::search_with_time(
                            position,
                            config,
                            crate::search::no_stop(),
                        );
                        if let Some(engine_move) = result.best_move {
                            let engine_str = print_move(engine_move);
                            make_move(&mut position, engine_move);
                            move_count += 1;
                            println!(
                                "Engine move: {engine_str} (depth {}, score {}, {} nodes)",
                                result.depth_reached, result.score, result.nodes
                            );
                            println!();
                        } else {
                            println!("Engine has no legal moves!");
                            break;
                        }
                    }
                    None => {
                        println!("Invalid move '{input}'. Type 'moves' to see available moves.");
                    }
                }
            }
            Err(_) => break,
        }
    }
}
