use crate::Piece;
use crate::piece::{BLACK_KING, PieceColor, PieceType, WHITE_KING, get_char};
use crate::square::get_square_name;
pub const SHORT_CASTLING_SQUARE: u8 = 65;
pub const LONG_CASTLING_SQUARE: u8 = 66;

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug)]
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

    pub fn build_short_castling(color: PieceColor) -> Self {
        let piece;
        if color == PieceColor::White {
            piece = WHITE_KING;
        }
        else {
            piece = BLACK_KING;
        }
        Self {
            piece: piece,
            from: SHORT_CASTLING_SQUARE,
            to: SHORT_CASTLING_SQUARE,
            capture: false,
            new_piece: None
        }
    }

    pub fn build_long_castling(color: PieceColor) -> Self {
        let piece;
        if color == PieceColor::White {
            piece = WHITE_KING;
        }
        else {
            piece = BLACK_KING;
        }
        Self {
            piece: piece,
            from: LONG_CASTLING_SQUARE,
            to: LONG_CASTLING_SQUARE,
            capture: false,
            new_piece: None
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
    if m.from == SHORT_CASTLING_SQUARE {
        s.push_str("0-0");
        return s;
    }
    if m.from == LONG_CASTLING_SQUARE {
        s.push_str("0-0-0");
        return s;
    }
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