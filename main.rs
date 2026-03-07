use crate::r#move::Move;
// use crate::r#move::Move;
use crate::piece::{Piece, *};
// use crate::r#move::print_move;
use crate::position::{START_POSITION, make_move, print_position};
// use crate::square;

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
    let e4 = Move {piece: WHITE_PAWN, from: 34, to: 36};
    make_move(&mut position, e4);
    print_position(position);
}