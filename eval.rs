use crate::piece::{Piece, PieceType, PieceColor};
use crate::position::Position;

/// Piece material values in centipawns.
const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 320;
const BISHOP_VALUE: i32 = 330;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;
const KING_VALUE: i32 = 20_000;

pub fn piece_value(piece: Piece) -> i32 {
    match piece.piece_type {
        PieceType::Pawn => PAWN_VALUE,
        PieceType::Knight => KNIGHT_VALUE,
        PieceType::Bishop => BISHOP_VALUE,
        PieceType::Rook => ROOK_VALUE,
        PieceType::Queen => QUEEN_VALUE,
        PieceType::King => KING_VALUE,
    }
}

/// Evaluate a position from the side-to-move's perspective.
/// Positive = good for the side to move, negative = bad.
/// Material only — no piece-square tables yet.
pub fn evaluate(position: Position) -> i32 {
    let mut score = 0i32;

    for row in 0..8 {
        for col in 0..8 {
            if let Some(piece) = position.pieces[row][col] {
                let val = piece_value(piece);
                score += match piece.piece_color {
                    PieceColor::White => val,
                    PieceColor::Black => -val,
                };
            }
        }
    }

    // Return from perspective of the side to move
    match position.turn {
        PieceColor::White => score,
        PieceColor::Black => -score,
    }
}