use crate::piece::*;
use crate::square::{self, get_square_name};
use crate::position::Position;

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

/// Parse a square name like "e4" to a 1-indexed square number (1..=64).
fn parse_square(s: &str) -> Option<u8> {
    if s.len() != 2 {
        return None;
    }
    let file = s.as_bytes()[0];
    let rank = s.as_bytes()[1];
    let col = match file {
        b'a'..=b'h' => file - b'a',
        b'A'..=b'H' => file - b'A',
        _ => return None,
    };
    let row = match rank {
        b'1'..=b'8' => rank - b'1',
        _ => return None,
    };
    Some(col * 8 + row + 1)
}

/// Parse a move in UCI coordinate notation: "e2e4", "g1f3", "e7e8q".
/// Looks up the board to determine which piece is moving.
pub fn parse_move_uci(s: &str, position: &Position) -> Option<Move> {
    // Handle castling
    if s == "e1g1" || s == "e1h1" {
        if position.turn == PieceColor::White {
            return Some(Move::build_short_castling(PieceColor::White));
        }
    }
    if s == "e8g8" || s == "e8h8" {
        if position.turn == PieceColor::Black {
            return Some(Move::build_short_castling(PieceColor::Black));
        }
    }
    if s == "e1c1" || s == "e1a1" {
        if position.turn == PieceColor::White {
            return Some(Move::build_long_castling(PieceColor::White));
        }
    }
    if s == "e8c8" || s == "e8a8" {
        if position.turn == PieceColor::Black {
            return Some(Move::build_long_castling(PieceColor::Black));
        }
    }

    let (from_str, rest) = s.split_at(2);
    if rest.len() < 2 {
        return None;
    }
    let to_str = &rest[..2];
    let promo_char = rest.as_bytes().get(2).copied();

    let from = parse_square(from_str)?;
    let to = parse_square(to_str)?;

    let (from_col, from_row) = square::get_col_and_row(from);
    let piece = position.pieces[from_row as usize][from_col as usize]?;

    if piece.piece_color != position.turn {
        return None;
    }

    let (to_col, to_row) = square::get_col_and_row(to);
    let target = position.pieces[to_row as usize][to_col as usize];

    let mut m = Move::build(piece, from, to);

    // Is it a capture?
    if target.is_some() {
        m = m.capture();
    }
    // En passant: pawn capture to empty square (different file, no piece)
    if piece.piece_type == PieceType::Pawn && target.is_none() && from_col != to_col {
        m = m.capture();
    }

    // Promotion?
    if let Some(pc) = promo_char {
        let promoted = match pc {
            b'q' | b'Q' => {
                if piece.piece_color == PieceColor::White { WHITE_QUEEN } else { BLACK_QUEEN }
            }
            b'r' | b'R' => {
                if piece.piece_color == PieceColor::White { WHITE_ROOK } else { BLACK_ROOK }
            }
            b'b' | b'B' => {
                if piece.piece_color == PieceColor::White { WHITE_BISHOP } else { BLACK_BISHOP }
            }
            b'n' | b'N' => {
                if piece.piece_color == PieceColor::White { WHITE_KNIGHT } else { BLACK_KNIGHT }
            }
            _ => return None,
        };
        m = m.promote_to(promoted);
    }

    Some(m)
}

/// Parse a move in standard algebraic notation: "e4", "Nf3", "exd6", "0-0", "f8=Q".
/// Generates all legal moves and finds the one whose print_move output matches.
pub fn parse_move_algebraic(s: &str, position: &Position) -> Option<Move> {
    let s = s.trim();

    // Handle castling
    if s == "O-O" || s == "0-0" {
        let color = position.turn;
        return Some(if color == PieceColor::White {
            Move::build_short_castling(PieceColor::White)
        } else {
            Move::build_short_castling(PieceColor::Black)
        });
    }
    if s == "O-O-O" || s == "0-0-0" {
        let color = position.turn;
        return Some(if color == PieceColor::White {
            Move::build_long_castling(PieceColor::White)
        } else {
            Move::build_long_castling(PieceColor::Black)
        });
    }

    let moves = crate::position::legal_moves(*position);
    moves.into_iter().find(|m| print_move(*m) == s)
}