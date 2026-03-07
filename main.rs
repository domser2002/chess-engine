use crate::r#move::Move;
use crate::piece::{Piece, WHITE_KNIGHT};
use crate::r#move::print_move;
// use crate::square;

mod r#move;
mod position;
mod piece;
mod square;
#[cfg(test)]
mod public_test;

fn main()
{
    let e4 = Move {piece: WHITE_KNIGHT, square: 1};
    let s = print_move(e4);
    println!("{s}");
}