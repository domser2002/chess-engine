use crate::piece::{
    BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_QUEEN, BLACK_ROOK, Piece, PieceColor, PieceType,
    WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_QUEEN, WHITE_ROOK, get_char,
};
use crate::position::Position;
use crate::square::{self, get_square_name};

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

#[derive(Clone, Copy)]
pub struct MoveList {
    pub moves: [Move; 256],
    pub count: usize,
}


impl std::cmp::PartialEq for MoveList {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl std::cmp::PartialEq<Vec<Move>> for MoveList {
    fn eq(&self, other: &Vec<Move>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl std::fmt::Debug for MoveList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self.as_slice(), f)
    }
}

impl MoveList {
    pub fn new() -> Self {
        Self {
            moves: [Move::build(crate::piece::WHITE_PAWN, 0, 0); 256],
            count: 0,
        }
    }

    pub fn push(&mut self, m: Move) {
        self.moves[self.count] = m;
        self.count += 1;
    }

    pub fn as_slice(&self) -> &[Move] {
        &self.moves[..self.count]
    }


    
    pub fn len(&self) -> usize {
        self.count
    }
}

pub struct MoveListIter {
    list: MoveList,
    index: usize,
}

impl Iterator for MoveListIter {
    type Item = Move;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.list.count {
            let m = self.list.moves[self.index];
            self.index += 1;
            Some(m)
        } else {
            None
        }
    }
}

impl IntoIterator for MoveList {
    type Item = Move;
    type IntoIter = MoveListIter;

    fn into_iter(self) -> Self::IntoIter {
        MoveListIter {
            list: self,
            index: 0,
        }
    }
}

impl std::ops::Deref for MoveList {
    type Target = [Move];
    fn deref(&self) -> &Self::Target {
        &self.moves[..self.count]
    }
}

impl std::ops::DerefMut for MoveList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.moves[..self.count]
    }
}

impl Move {
    pub fn build(piece: Piece, from: u8, to: u8) -> Self {
        Self {
            piece,
            from,
            to,
            capture: false,
            new_piece: None,
        }
    }

    pub fn build_short_castling(color: PieceColor) -> Self {
        let piece;
        if color == PieceColor::White {
            piece = WHITE_KING;
        } else {
            piece = BLACK_KING;
        }
        Self {
            piece,
            from: SHORT_CASTLING_SQUARE,
            to: SHORT_CASTLING_SQUARE,
            capture: false,
            new_piece: None,
        }
    }

    pub fn build_long_castling(color: PieceColor) -> Self {
        let piece;
        if color == PieceColor::White {
            piece = WHITE_KING;
        } else {
            piece = BLACK_KING;
        }
        Self {
            piece,
            from: LONG_CASTLING_SQUARE,
            to: LONG_CASTLING_SQUARE,
            capture: false,
            new_piece: None,
        }
    }

    pub fn capture(mut self) -> Self {
        self.capture = true;
        self
    }

    pub fn promote_to(mut self, piece: Piece) -> Self {
        assert!(
            (self.piece.piece_type == PieceType::Pawn),
            "Only pawns can promote!"
        );
        self.new_piece = Some(piece);
        self
    }
}

pub fn print_move(m: Move) -> String {
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
    if m.piece.piece_type == PieceType::Pawn
        && let Some(new_piece) = m.new_piece
    {
        s.push('=');
        s.extend(get_char(new_piece));
    }
    s
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
pub fn parse_move_uci(s: &str, position: &Position) -> Option<Move> {
    if (s == "e1g1" || s == "e1h1") && position.turn == PieceColor::White {
        if let Some(p) = position.get_piece(0, 4) {
            if p.piece_type == PieceType::King {
                return Some(Move::build_short_castling(PieceColor::White));
            }
        }
    }
    if (s == "e8g8" || s == "e8h8") && position.turn == PieceColor::Black {
        if let Some(p) = position.get_piece(7, 4) {
            if p.piece_type == PieceType::King {
                return Some(Move::build_short_castling(PieceColor::Black));
            }
        }
    }
    if (s == "e1c1" || s == "e1a1") && position.turn == PieceColor::White {
        if let Some(p) = position.get_piece(0, 4) {
            if p.piece_type == PieceType::King {
                return Some(Move::build_long_castling(PieceColor::White));
            }
        }
    }
    if (s == "e8c8" || s == "e8a8") && position.turn == PieceColor::Black {
        if let Some(p) = position.get_piece(7, 4) {
            if p.piece_type == PieceType::King {
                return Some(Move::build_long_castling(PieceColor::Black));
            }
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
    let piece = position.get_piece(from_row as usize, from_col as usize)?;

    if piece.piece_color != position.turn {
        return None;
    }

    let (to_col, to_row) = square::get_col_and_row(to);
    let target = position.get_piece(to_row as usize, to_col as usize);

    let mut m = Move::build(piece, from, to);

    if target.is_some() {
        m = m.capture();
    }
    if piece.piece_type == PieceType::Pawn && target.is_none() && from_col != to_col {
        m = m.capture();
    }

    if let Some(pc) = promo_char {
        let promoted = match pc {
            b'q' | b'Q' => {
                if piece.piece_color == PieceColor::White {
                    WHITE_QUEEN
                } else {
                    BLACK_QUEEN
                }
            }
            b'r' | b'R' => {
                if piece.piece_color == PieceColor::White {
                    WHITE_ROOK
                } else {
                    BLACK_ROOK
                }
            }
            b'b' | b'B' => {
                if piece.piece_color == PieceColor::White {
                    WHITE_BISHOP
                } else {
                    BLACK_BISHOP
                }
            }
            b'n' | b'N' => {
                if piece.piece_color == PieceColor::White {
                    WHITE_KNIGHT
                } else {
                    BLACK_KNIGHT
                }
            }
            _ => return None,
        };
        m = m.promote_to(promoted);
    }

    Some(m)
}

/// Parse a move in standard algebraic notation: "e4", "Nf3", "exd6", "Rfc1", "0-0", "f8=Q".
/// Handles disambiguation (file, rank, or both) for non-pawn pieces.
pub fn parse_move_algebraic(s: &str, position: &Position) -> Option<Move> {
    let s = s.trim();

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

    let bytes = s.as_bytes();
    let len = bytes.len();
    if len < 2 {
        return None;
    }

    // Determine piece type and where the body starts (pos = after piece letter)
    let (piece_type, pos) = match bytes[0] {
        b'N' => (PieceType::Knight, 1),
        b'B' => (PieceType::Bishop, 1),
        b'R' => (PieceType::Rook, 1),
        b'Q' => (PieceType::Queen, 1),
        b'K' => (PieceType::King, 1),
        _ => (PieceType::Pawn, 0),
    };

    // Check for promotion first: "...=Q" or "...=q"
    let promo = if len >= pos + 4 && bytes[len - 2] == b'=' {
        let promo_char = bytes[len - 1];
        Some(match promo_char {
            b'Q' | b'q' => {
                if position.turn == PieceColor::White {
                    WHITE_QUEEN
                } else {
                    BLACK_QUEEN
                }
            }
            b'R' | b'r' => {
                if position.turn == PieceColor::White {
                    WHITE_ROOK
                } else {
                    BLACK_ROOK
                }
            }
            b'B' | b'b' => {
                if position.turn == PieceColor::White {
                    WHITE_BISHOP
                } else {
                    BLACK_BISHOP
                }
            }
            b'N' | b'n' => {
                if position.turn == PieceColor::White {
                    WHITE_KNIGHT
                } else {
                    BLACK_KNIGHT
                }
            }
            _ => return None,
        })
    } else {
        None
    };

    // The body of the move (without piece letter and without promotion suffix)
    let body_end = if promo.is_some() { len - 2 } else { len };
    let body = &s[pos..body_end];

    parse_move_algebraic_body(body, piece_type, promo, position)
}

/// Parse the body of an algebraic move (after the piece letter and promotion).
/// body examples: "e4", "f3", "exd5", "fc1", "f1c1", "1c1"
fn parse_move_algebraic_body(
    body: &str,
    piece_type: PieceType,
    promo: Option<Piece>,
    position: &Position,
) -> Option<Move> {
    let len = body.len();
    if len < 2 {
        return None;
    }
    let to_str = &body[len - 2..];
    let to = parse_square(to_str)?;

    let mut moves = crate::r#move::MoveList::new();
    crate::position::legal_moves(*position, &mut moves);

    // Build disambiguation info from whatever characters come before the destination
    let prefix = &body[..len - 2]; // e.g. "", "e", "1", "e1", "x", "ex"

    // Separate an optional 'x' (capture indicator) from the disambiguation characters
    let mut disambig = prefix;
    if let Some(idx) = disambig.rfind('x') {
        disambig = &disambig[..idx]; // characters before 'x'
    }

    // Parse disambiguation components
    let ambig_file: Option<u8> = disambig
        .chars()
        .find(char::is_ascii_lowercase)
        .map(|c| (c as u8) - b'a');
    let ambig_rank: Option<u8> = disambig
        .chars()
        .find(char::is_ascii_digit)
        .map(|c| (c as u8) - b'1');

    moves.into_iter().find(|m| {
        if m.from == SHORT_CASTLING_SQUARE || m.from == LONG_CASTLING_SQUARE {
            return false;
        }
        if m.piece.piece_type != piece_type {
            return false;
        }
        if m.to != to {
            return false;
        }
        // Promotion match
        if let Some(pp) = promo {
            if m.new_piece != Some(pp) {
                return false;
            }
        } else if m.new_piece.is_some() {
            return false;
        }
        // Check disambiguation for all pieces (including pawns, even without 'x')
        let (from_col, from_row) = square::get_col_and_row(m.from);
        if let Some(f) = ambig_file
            && from_col != f
        {
            return false;
        }
        if let Some(r) = ambig_rank
            && from_row != r
        {
            return false;
        }
        true
    })
}
