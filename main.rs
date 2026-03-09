use crate::r#move::Move;
// use crate::r#move::Move;
use crate::piece::{Piece, *};
// use crate::r#move::print_move;
use crate::position::{START_POSITION, make_move, print_position};
use crate::square::Squares::*;

mod r#move;
mod position;
mod piece;
mod square;
#[cfg(test)]
mod public_test;

fn main()
{
    let mut position = START_POSITION;
    print_position(position);
    let e4 = Move {piece: WHITE_PAWN, from: E2 as u8 , to: E4 as u8};
    make_move(&mut position, e4);
    print_position(position);
    let d5 = Move {piece: BLACK_PAWN, from: D7 as u8, to: D5 as u8};
    make_move(&mut position, d5);
    print_position(position);
    let ed5 = Move {piece: WHITE_PAWN, from: E4 as u8, to: D5 as u8};
    make_move(&mut position, ed5);
    print_position(position);
}