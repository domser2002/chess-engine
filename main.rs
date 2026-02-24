use crate::domain::Move;
use crate::domain::Piece;
use crate::domain::print_move;

mod domain;
#[cfg(test)]
mod public_test;

fn main()
{
    let e4 = Move {piece: Piece::Knight, square: 1};
    let s = print_move(e4);
    println!("{s}");
}