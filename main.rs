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
    // print_position(position);
    // for _i in 0..10 {
    //     let legal_moves = legal_moves(position);
    //     let mut rng = rand::thread_rng();
    //     if let Some(choice) = legal_moves.choose(&mut rng) {
    //         let s = print_move(*choice);
    //         println!("{s}");
    //         make_move(&mut position, *choice);
    //         print_position(position);
    //     }
    // }
    let e4 = Move::build(WHITE_PAWN, E2 as u8, E4 as u8);
    let e5 = Move::build(WHITE_PAWN, E4 as u8, E5 as u8);
    let b5 = Move::build(BLACK_PAWN, B7 as u8, B5 as u8);
    let d5 = Move::build(BLACK_PAWN, D7 as u8, D5 as u8);
    make_move(&mut position, e4);
    make_move(&mut position, b5);
    make_move(&mut position, e5);
    make_move(&mut position, d5);
    print_position(position);
    let legal_moves = legal_moves(position);
    for legal_move in legal_moves {
        let s = print_move(legal_move);
        println!("{s}");
    }
}