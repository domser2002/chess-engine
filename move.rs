use crate::Piece;
use crate::piece::get_char;
use crate::square::get_square_name;

pub struct Move {
    pub piece: Piece,
    pub from: u8,
    pub to: u8,
}

pub fn print_move(m: Move) -> String
{
    let mut s = String::new();
    s.extend(get_char(m.piece));
    s.push_str(&get_square_name(m.to));
    s
    // println!("{s}");
}