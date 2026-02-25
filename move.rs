use crate::{Piece, piece::PieceType};

pub struct Move {
    pub piece: Piece,
    pub square: u8,
}

pub fn print_move(m: Move) -> String
{
    let mut s = String::new();
    match m.piece.piece_type {
        PieceType::Pawn => {
        },
        PieceType::Knight => {
            s.push('N');

        },
        PieceType::Bishop => {
            s.push('B');

        },
        PieceType::Rook => {
            s.push('R');

        },
        PieceType::Queen => {
            s.push('Q');

        },
        PieceType::King => {
            s.push('K');

        }
    }
    let row_idx: u8 = (m.square + 8) / 8 as u8;
    let row = (('a' as u8) + (row_idx - 1)) as char;
    s.push(row);
    let mut column_idx = m.square % 8;
    if column_idx == 0 {
        column_idx = 8;
    }
    s.push_str(&column_idx.to_string());
    s
    // println!("{s}");
}