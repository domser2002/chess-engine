use crate::Piece;
use crate::piece::{PieceType, get_char};
use crate::square::get_square_name;

#[derive(Clone, Copy)]
pub struct Move {
    pub piece: Piece,
    pub from: u8,
    pub to: u8,
    pub capture: bool,
    pub new_piece: Option<Piece>,
}

impl Move {
    pub fn build(piece: Piece, from: u8, to: u8) -> Self {
        Self {
            piece, from, to,
            capture: false,
            new_piece: None,
        }
    }

    pub fn capture(mut self) -> Self {
        self.capture = true;
        self
    }

    pub fn promote_to(mut self, piece: Piece) -> Self {
        if self.piece.piece_type != PieceType::Pawn {
            panic!("Only pawns can promote!");
        }
        self.new_piece = Some(piece);
        self
    }
}

pub fn print_move(m: Move) -> String
{
    let mut s = String::new();
    s.extend(get_char(m.piece));
    if m.capture && m.piece.piece_type == PieceType::Pawn {
        let src = get_square_name(m.from);
        s.extend(src.chars().next());
    }
    s.push_str(&get_square_name(m.to));
    if m.piece.piece_type == PieceType::Pawn {
        match m.new_piece {
            Some(new_piece) => {
                s.push('=');
                s.extend(get_char(new_piece));
            }
            None => {}
        }
    }
    s
    // println!("{s}");
}