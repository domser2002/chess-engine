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
mod board;
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
    let a4 = Move::build(WHITE_PAWN, A2 as u8, A4 as u8);
    let b5 = Move::build(BLACK_PAWN, B7 as u8, B5 as u8);
    let ab5 = Move::build(WHITE_PAWN, A4 as u8, B5 as u8).capture();
    let a5 = Move::build(BLACK_PAWN, A7 as u8, A5 as u8);
    let b6 = Move::build(WHITE_PAWN, B5 as u8, B6 as u8);
    let nc6 = Move::build(BLACK_KNIGHT, B8 as u8, C6 as u8);
    let b7 = Move::build(WHITE_PAWN, B6 as u8, B7 as u8);
    let ra7 = Move::build(BLACK_ROOK, A8 as u8, A7 as u8);

    make_move(&mut position, a4);
    make_move(&mut position, b5);
    make_move(&mut position, ab5);
    make_move(&mut position, a5);
    make_move(&mut position, b6);
    make_move(&mut position, nc6);
    make_move(&mut position, b7);
    make_move(&mut position, ra7);

    print_position(position);
    let legal_moves = legal_moves(position);
    for legal_move in legal_moves {
        let s = print_move(legal_move);
        println!("{s}");
    }
}