use crate::r#move::{Move, print_move};
// use crate::r#move::Move;
use crate::piece::{Piece, *};
// use crate::r#move::print_move;
use crate::position::*;
use crate::square::Squares::*;

mod r#move;
mod position;
mod piece;
mod square;
#[cfg(test)]
mod public_test;
use rand::seq::SliceRandom;

fn main()
{
    let mut position = start_position();
    print_position(position);
    for _i in 0..1000 {
        let legal_moves = legal_moves(position);
        let mut rng = rand::thread_rng();
        if let Some(choice) = legal_moves.choose(&mut rng) {
            let s = print_move(*choice);
            println!("{s}");
            make_move(&mut position, *choice);
            print_position(position);
        }
    }
}