use std::io::{self, BufRead, Write};
use rand::seq::SliceRandom;
use crate::position::{self, *};
use crate::r#move::*;
use crate::piece::*;

/// Run an interactive game where the user types moves in algebraic notation
/// and the engine responds with random legal moves.
pub fn interactive_loop() {
    let mut position = start_position();
    let mut rng = rand::thread_rng();
    let mut move_count = 0;

    println!("Chess Engine — Interactive Mode");
    println!("Type moves in algebraic notation (e4, Nf3, exd5, 0-0, f8=Q)");
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
        let moves = legal_moves(position);
        if moves.is_empty() {
            let in_check = is_in_check(position, position.turn);
            let side = match position.turn {
                PieceColor::White => "White",
                PieceColor::Black => "Black",
            };
            println!(
                "Game over after {move_count} moves. {}{}",
                side,
                if in_check { " is checkmated!" } else { " is stalemated." }
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
                if input == "moves" {
                    for m in &moves {
                        println!("  {}", print_move(*m));
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
                        let opponent_moves = legal_moves(position);
                        if opponent_moves.is_empty() {
                            let in_check = is_in_check(position, position.turn);
                            let side = match position.turn {
                                PieceColor::White => "White",
                                PieceColor::Black => "Black",
                            };
                            println!(
                                "Game over after {move_count} moves. {}{}",
                                side,
                                if in_check { " is checkmated!" } else { " is stalemated." }
                            );
                            break;
                        }

                        // Engine's turn
                        let engine_choice = opponent_moves.choose(&mut rng).unwrap();
                        let engine_str = print_move(*engine_choice);
                        make_move(&mut position, *engine_choice);
                        move_count += 1;
                        println!("Engine move: {engine_str}\n");
                    }
                    None => {
                        println!("Invalid move '{}'. Type 'moves' to see available moves.", input);
                    }
                }
            }
            Err(_) => break,
        }
    }
}