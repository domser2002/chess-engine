use crate::r#move::print_move;
use crate::piece::*;
use crate::position::*;
use rand::seq::SliceRandom;

mod r#move;
mod position;
mod piece;
mod square;
#[cfg(test)]
mod public_test;

fn main() {
    let mut position = start_position();
    let mut rng = rand::thread_rng();
    let mut move_count = 0;

    println!("Starting random chess game\n");
    print_position(position);

    loop {
        let moves = legal_moves(position);
        if moves.is_empty() {
            // No legal moves — game over (checkmate or stalemate)
            let in_check = is_in_check(position, position.turn);
            println!(
                "\nGame over after {move_count} moves. {} {}",
                match position.turn {
                    PieceColor::White => "White",
                    PieceColor::Black => "Black",
                },
                if in_check { "is checkmated!" } else { "is stalemated." }
            );
            break;
        }

        // Pick a random legal move
        let choice = moves.choose(&mut rng).unwrap();
        let s = print_move(*choice);
        println!("\nMove {move_count}: {s}");
        make_move(&mut position, *choice);
        print_position(position);

        move_count += 1;

        // Safety limit to prevent infinite loops in weird edge cases
        if move_count >= 500 {
            println!("\nReached 500 move limit — stopping.");
            break;
        }
    }
}
