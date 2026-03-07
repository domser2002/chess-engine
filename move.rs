use crate::{Piece, piece::{PieceType, get_char}};
use crate::{square::get_square_name};

pub struct Move {
    pub piece: Piece,
    pub square: u8,
}

pub fn print_move(m: Move) -> String
{
    let mut s = String::new();
    s.extend(get_char(m.piece));
    s.push_str(&get_square_name(m.square));
    s
    // println!("{s}");
}