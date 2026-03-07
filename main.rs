// use crate::r#move::Move;
use crate::piece::{Piece};
// use crate::r#move::print_move;
use crate::position::{START_POSITION, print_position};
// use crate::square;

mod r#move;
mod position;
mod piece;
mod square;
#[cfg(test)]
mod public_test;

fn main()
{
    print_position(START_POSITION);
}