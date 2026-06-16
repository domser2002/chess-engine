use crate::r#move::{LONG_CASTLING_SQUARE, Move, SHORT_CASTLING_SQUARE};
use crate::piece::{
    BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK, Piece, PieceColor,
    PieceType, WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
    get_unicode_char,
};
use crate::square::{COL_NUM, ROW_NUM, get_col_and_row, get_square_idx};
use colored::Colorize;

use crate::bitboard::Bitboard;
use crate::attacks::{KNIGHT_ATTACKS, KING_ATTACKS, WHITE_PAWN_ATTACKS, BLACK_PAWN_ATTACKS, get_rook_attacks, get_bishop_attacks, get_queen_attacks};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Position {
    pub piece_colors: [Bitboard; 2],
    pub piece_types: [Bitboard; 6],
    pub turn: PieceColor,
    pub last_move: Option<Move>,
    pub white_short_castle: bool,
    pub white_long_castle: bool,
    pub black_short_castle: bool,
    pub black_long_castle: bool,
    pub hash: u64,
}

impl Position {
    pub fn get_piece(&self, row: usize, col: usize) -> Option<Piece> {
        let sq = row * 8 + col;
        let bit = 1u64 << sq;
        let white = self.piece_colors[PieceColor::White as usize].0 & bit;
        let black = self.piece_colors[PieceColor::Black as usize].0 & bit;

        if white == 0 && black == 0 {
            return None;
        }

        let color = if white != 0 {
            PieceColor::White
        } else {
            PieceColor::Black
        };

        if (self.piece_types[PieceType::Pawn as usize].0 & bit) != 0 {
            return Some(Piece {
                piece_type: PieceType::Pawn,
                piece_color: color,
            });
        }
        if (self.piece_types[PieceType::Knight as usize].0 & bit) != 0 {
            return Some(Piece {
                piece_type: PieceType::Knight,
                piece_color: color,
            });
        }
        if (self.piece_types[PieceType::Bishop as usize].0 & bit) != 0 {
            return Some(Piece {
                piece_type: PieceType::Bishop,
                piece_color: color,
            });
        }
        if (self.piece_types[PieceType::Rook as usize].0 & bit) != 0 {
            return Some(Piece {
                piece_type: PieceType::Rook,
                piece_color: color,
            });
        }
        if (self.piece_types[PieceType::Queen as usize].0 & bit) != 0 {
            return Some(Piece {
                piece_type: PieceType::Queen,
                piece_color: color,
            });
        }
        if (self.piece_types[PieceType::King as usize].0 & bit) != 0 {
            return Some(Piece {
                piece_type: PieceType::King,
                piece_color: color,
            });
        }
        None
    }

    pub fn set_piece(&mut self, row: usize, col: usize, piece: Option<Piece>) {
        let sq = row * 8 + col;
        let mask = 1u64 << sq;
        let not_mask = !mask;

        // Clear existing
        self.piece_colors[0].0 &= not_mask;
        self.piece_colors[1].0 &= not_mask;
        for i in 0..6 {
            self.piece_types[i].0 &= not_mask;
        }

        if let Some(p) = piece {
            self.piece_colors[p.piece_color as usize].0 |= mask;
            self.piece_types[p.piece_type as usize].0 |= mask;
        }
    }
}

/// Stores the state that changed during a move, enough to undo it.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct UndoInfo {
    pub captured: Option<Piece>,
    pub white_short_castle: bool,
    pub white_long_castle: bool,
    pub black_short_castle: bool,
    pub black_long_castle: bool,
    pub last_move: Option<Move>,
    pub hash: u64,
}

/// Stores the state needed to undo a null move.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct NullUndoInfo {
    pub last_move: Option<Move>,
    pub hash: u64,
}

const ROOK_DIRS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const BISHOP_DIRS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
const KING_DIRS: [(i8, i8); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];
const KNIGHT_OFFSETS: [(i8, i8); 8] = [
    (2, 1),
    (1, 2),
    (-2, 1),
    (-1, 2),
    (2, -1),
    (1, -2),
    (-2, -1),
    (-1, -2),
];

pub fn empty_position(turn: PieceColor) -> Position {
    Position {
        piece_colors: [Bitboard::empty(); 2],
        piece_types: [Bitboard::empty(); 6],
        turn,
        last_move: None,
        white_short_castle: false,
        white_long_castle: false,
        black_short_castle: false,
        black_long_castle: false,
        hash: 0,
    }
}

pub fn start_position() -> Position {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    parse_fen(fen).unwrap()
}

/// Parse a FEN string into a Position.
///
/// FEN format: `<pieces> <color> <castling> <en_passant> [<halfmove> <fullmove>]`
///
/// Example: `"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"`
///
/// Returns `None` if the FEN string is malformed.
/// Halfmove clock and fullmove number are parsed but not stored
/// (the engine does not track them).
pub fn parse_fen(fen: &str) -> Option<Position> {
    let parts: Vec<&str> = fen.split_whitespace().collect();
    if parts.len() < 4 {
        return None;
    }

    let mut pos = empty_position(PieceColor::White);

    // 1. Piece placement — ranks from 8 (top) to 1 (bottom), separated by '/'
    let ranks: Vec<&str> = parts[0].split('/').collect();
    if ranks.len() != 8 {
        return None;
    }

    for (rank_from_top, rank_str) in ranks.iter().enumerate() {
        let row = 7 - rank_from_top; // FEN rank 8 = row index 7
        let mut col: usize = 0;
        for ch in rank_str.chars() {
            if col > 8 {
                return None;
            }
            if let Some(digit) = ch.to_digit(10) {
                col += digit as usize;
            } else {
                let piece = fen_char_to_piece(ch)?;
                if col >= 8 {
                    return None;
                }
                pos.set_piece(row, col, Some(piece));
                col += 1;
            }
        }
        if col != 8 {
            return None;
        }
    }

    // 2. Active color
    pos.turn = match parts[1] {
        "w" => PieceColor::White,
        "b" => PieceColor::Black,
        _ => return None,
    };

    // 3. Castling availability
    let castling = parts[2];
    if castling != "-" {
        pos.white_short_castle = castling.contains('K');
        pos.white_long_castle = castling.contains('Q');
        pos.black_short_castle = castling.contains('k');
        pos.black_long_castle = castling.contains('q');
    }

    // 4. En passant target square
    //    Synthesize a last_move representing the double pawn push so the
    //    existing en passant logic in add_pawn_moves / try_en_passant works.
    if parts[3] != "-" {
        let ep = parts[3];
        if ep.len() != 2 {
            return None;
        }
        let file = ep.as_bytes()[0].wrapping_sub(b'a');
        let rank = ep.as_bytes()[1].wrapping_sub(b'1');
        if file >= 8 || rank >= 8 {
            return None;
        }

        if rank == 2 {
            // Target on rank 3 → white pawn moved from rank 2 to rank 4
            let from = get_square_idx(1, file);
            let to = get_square_idx(3, file);
            pos.last_move = Some(Move::build(WHITE_PAWN, from, to));
        } else if rank == 5 {
            // Target on rank 6 → black pawn moved from rank 7 to rank 5
            let from = get_square_idx(6, file);
            let to = get_square_idx(4, file);
            pos.last_move = Some(Move::build(BLACK_PAWN, from, to));
        }
    }

    // Fields 5–6 (halfmove clock, fullmove number) are accepted but not stored.
    pos.hash = crate::zobrist::compute_hash(&pos);
    Some(pos)
}

/// Convert a FEN piece character to a Piece.
fn fen_char_to_piece(ch: char) -> Option<Piece> {
    match ch {
        'P' => Some(WHITE_PAWN),
        'N' => Some(WHITE_KNIGHT),
        'B' => Some(WHITE_BISHOP),
        'R' => Some(WHITE_ROOK),
        'Q' => Some(WHITE_QUEEN),
        'K' => Some(WHITE_KING),
        'p' => Some(BLACK_PAWN),
        'n' => Some(BLACK_KNIGHT),
        'b' => Some(BLACK_BISHOP),
        'r' => Some(BLACK_ROOK),
        'q' => Some(BLACK_QUEEN),
        'k' => Some(BLACK_KING),
        _ => None,
    }
}

#[cfg_attr(test, allow(dead_code))]
fn print_colored(c: char, pc: PieceColor) {
    let text = c.to_string();
    let colored = match pc {
        PieceColor::Black => text.black(),
        PieceColor::White => text.yellow(),
    };

    print!("{colored}");
}

// this will not be unit tested as it is only for UI purpose
#[cfg_attr(test, allow(dead_code))]
pub fn print_position(position: Position) {
    let horizontal_line: String = std::iter::repeat_n('-', 17).collect();
    for row_idx in (0..ROW_NUM).rev() {
        println!("  {horizontal_line}");
        print!("{} |", row_idx + 1);
        for col_idx in 0..COL_NUM {
            let maybe_piece: Option<Piece> = position.get_piece(row_idx as usize, col_idx as usize);
            match maybe_piece {
                Some(piece) => {
                    let c = get_unicode_char(piece);
                    print_colored(c, piece.piece_color);
                }
                None => {
                    print!(" ");
                }
            }
            print!("|");
        }
        println!(" {}", row_idx + 1);
    }
    println!("  {horizontal_line}");
    println!("   a b c d e f g h");
}

pub fn square_to_lerf(sq: u8) -> usize {
    let col = (sq - 1) / 8;
    let row = (sq - 1) % 8;
    (row * 8 + col) as usize
}

pub fn lerf_to_square(lerf: usize) -> u8 {
    let row = lerf / 8;
    let col = lerf % 8;
    (col * 8 + row + 1) as u8
}

pub fn is_square_attacked(position: Position, square: u8, by_color: PieceColor) -> bool {
    let sq = square_to_lerf(square);
    let occ = Bitboard(position.piece_colors[0].0 | position.piece_colors[1].0);

    let opp_pieces = position.piece_colors[by_color as usize];
    let opp_pawns = Bitboard(opp_pieces.0 & position.piece_types[PieceType::Pawn as usize].0);
    let opp_knights = Bitboard(opp_pieces.0 & position.piece_types[PieceType::Knight as usize].0);
    let opp_bishops = Bitboard(opp_pieces.0 & position.piece_types[PieceType::Bishop as usize].0);
    let opp_rooks = Bitboard(opp_pieces.0 & position.piece_types[PieceType::Rook as usize].0);
    let opp_queens = Bitboard(opp_pieces.0 & position.piece_types[PieceType::Queen as usize].0);
    let opp_kings = Bitboard(opp_pieces.0 & position.piece_types[PieceType::King as usize].0);

    if (KNIGHT_ATTACKS[sq].0 & opp_knights.0) != 0 {
        return true;
    }
    if (KING_ATTACKS[sq].0 & opp_kings.0) != 0 {
        return true;
    }

    let pawn_attacks = if by_color == PieceColor::White {
        BLACK_PAWN_ATTACKS[sq]
    } else {
        WHITE_PAWN_ATTACKS[sq]
    };
    if (pawn_attacks.0 & opp_pawns.0) != 0 {
        return true;
    }

    if (get_bishop_attacks(sq, occ).0 & (opp_bishops.0 | opp_queens.0)) != 0 {
        return true;
    }
    if (get_rook_attacks(sq, occ).0 & (opp_rooks.0 | opp_queens.0)) != 0 {
        return true;
    }

    false
}

pub fn is_in_check(position: Position, color: PieceColor) -> bool {
    let kings =
        position.piece_colors[color as usize].0 & position.piece_types[PieceType::King as usize].0;
    if kings == 0 {
        return false;
    }
    let king_sq = kings.trailing_zeros() as usize;
    let enemy = if color == PieceColor::White { PieceColor::Black } else { PieceColor::White };
    is_square_attacked(position, lerf_to_square(king_sq), enemy)
}

pub fn pseudo_legal_moves(position: Position) -> Vec<Move> {
    let mut v = Vec::with_capacity(60);
    let us = position.turn;
    let them = if us == PieceColor::White { PieceColor::Black } else { PieceColor::White };

    let my_pieces = position.piece_colors[us as usize];
    let opp_pieces = position.piece_colors[them as usize];
    let occ = Bitboard(my_pieces.0 | opp_pieces.0);
    let empty = Bitboard(!occ.0);

    // PAWNS
    let mut pawns = Bitboard(my_pieces.0 & position.piece_types[PieceType::Pawn as usize].0);
    let dir: i32 = if us == PieceColor::White { 8 } else { -8 };
    let start_row = if us == PieceColor::White { 1 } else { 6 };
    let prom_row = if us == PieceColor::White { 7 } else { 0 };

    while pawns.0 != 0 {
        let from_lerf = pawns.pop_lsb() as usize;
        let from_sq = lerf_to_square(from_lerf);
        let piece = Piece {
            piece_type: PieceType::Pawn,
            piece_color: us,
        };
        let q = if us == PieceColor::White {
            WHITE_QUEEN
        } else {
            BLACK_QUEEN
        };
        let r = if us == PieceColor::White {
            WHITE_ROOK
        } else {
            BLACK_ROOK
        };
        let b = if us == PieceColor::White {
            WHITE_BISHOP
        } else {
            BLACK_BISHOP
        };
        let n = if us == PieceColor::White {
            WHITE_KNIGHT
        } else {
            BLACK_KNIGHT
        };

        let to_lerf = (from_lerf as i32 + dir) as usize;
        if to_lerf < 64 && empty.test_bit(to_lerf as u8) {
            let to_sq = lerf_to_square(to_lerf);
            let row = to_lerf / 8;
            if row == prom_row {
                v.push(Move::build(piece, from_sq, to_sq).promote_to(q));
                v.push(Move::build(piece, from_sq, to_sq).promote_to(r));
                v.push(Move::build(piece, from_sq, to_sq).promote_to(b));
                v.push(Move::build(piece, from_sq, to_sq).promote_to(n));
            } else {
                v.push(Move::build(piece, from_sq, to_sq));
                if from_lerf / 8 == start_row {
                    let to2_lerf = (from_lerf as i32 + 2 * dir) as usize;
                    if empty.test_bit(to2_lerf as u8) {
                        v.push(Move::build(piece, from_sq, lerf_to_square(to2_lerf)));
                    }
                }
            }
        }

        let attacks = if us == PieceColor::White {
            WHITE_PAWN_ATTACKS[from_lerf]
        } else {
            BLACK_PAWN_ATTACKS[from_lerf]
        };
        let mut caps = Bitboard(attacks.0 & opp_pieces.0);
        while caps.0 != 0 {
            let to_lerf = caps.pop_lsb() as usize;
            let to_sq = lerf_to_square(to_lerf);
            let row = to_lerf / 8;
            if row == prom_row {
                v.push(Move::build(piece, from_sq, to_sq).capture().promote_to(q));
                v.push(Move::build(piece, from_sq, to_sq).capture().promote_to(r));
                v.push(Move::build(piece, from_sq, to_sq).capture().promote_to(b));
                v.push(Move::build(piece, from_sq, to_sq).capture().promote_to(n));
            } else {
                v.push(Move::build(piece, from_sq, to_sq).capture());
            }
        }

        if let Some(last_move) = position.last_move {
            if last_move.piece.piece_type == PieceType::Pawn {
                let last_from_lerf = square_to_lerf(last_move.from);
                let last_to_lerf = square_to_lerf(last_move.to);
                let diff = last_to_lerf as i32 - last_from_lerf as i32;
                if diff == 16 || diff == -16 {
                    if (from_lerf as i32 - last_to_lerf as i32).abs() == 1
                        && (from_lerf / 8) == (last_to_lerf / 8)
                    {
                        let to_lerf = (last_to_lerf as i32 + dir) as usize;
                        v.push(Move::build(piece, from_sq, lerf_to_square(to_lerf)).capture());
                    }
                }
            }
        }
    }

    // KNIGHTS
    let mut knights = Bitboard(my_pieces.0 & position.piece_types[PieceType::Knight as usize].0);
    while knights.0 != 0 {
        let from_lerf = knights.pop_lsb() as usize;
        let from_sq = lerf_to_square(from_lerf);
        let piece = Piece {
            piece_type: PieceType::Knight,
            piece_color: us,
        };
        let mut attacks = Bitboard(KNIGHT_ATTACKS[from_lerf].0 & !my_pieces.0);
        while attacks.0 != 0 {
            let to_lerf = attacks.pop_lsb() as usize;
            let to_sq = lerf_to_square(to_lerf);
            let mut m = Move::build(piece, from_sq, to_sq);
            if (opp_pieces.0 & (1u64 << to_lerf)) != 0 {
                m = m.capture();
            }
            v.push(m);
        }
    }

    // BISHOPS
    let mut bishops = Bitboard(my_pieces.0 & position.piece_types[PieceType::Bishop as usize].0);
    while bishops.0 != 0 {
        let from_lerf = bishops.pop_lsb() as usize;
        let from_sq = lerf_to_square(from_lerf);
        let piece = Piece {
            piece_type: PieceType::Bishop,
            piece_color: us,
        };
        let mut attacks = Bitboard(get_bishop_attacks(from_lerf, occ).0 & !my_pieces.0);
        while attacks.0 != 0 {
            let to_lerf = attacks.pop_lsb() as usize;
            let to_sq = lerf_to_square(to_lerf);
            let mut m = Move::build(piece, from_sq, to_sq);
            if (opp_pieces.0 & (1u64 << to_lerf)) != 0 {
                m = m.capture();
            }
            v.push(m);
        }
    }

    // ROOKS
    let mut rooks = Bitboard(my_pieces.0 & position.piece_types[PieceType::Rook as usize].0);
    while rooks.0 != 0 {
        let from_lerf = rooks.pop_lsb() as usize;
        let from_sq = lerf_to_square(from_lerf);
        let piece = Piece {
            piece_type: PieceType::Rook,
            piece_color: us,
        };
        let mut attacks = Bitboard(get_rook_attacks(from_lerf, occ).0 & !my_pieces.0);
        while attacks.0 != 0 {
            let to_lerf = attacks.pop_lsb() as usize;
            let to_sq = lerf_to_square(to_lerf);
            let mut m = Move::build(piece, from_sq, to_sq);
            if (opp_pieces.0 & (1u64 << to_lerf)) != 0 {
                m = m.capture();
            }
            v.push(m);
        }
    }

    // QUEENS
    let mut queens = Bitboard(my_pieces.0 & position.piece_types[PieceType::Queen as usize].0);
    while queens.0 != 0 {
        let from_lerf = queens.pop_lsb() as usize;
        let from_sq = lerf_to_square(from_lerf);
        let piece = Piece {
            piece_type: PieceType::Queen,
            piece_color: us,
        };
        let mut attacks = Bitboard(get_queen_attacks(from_lerf, occ).0 & !my_pieces.0);
        while attacks.0 != 0 {
            let to_lerf = attacks.pop_lsb() as usize;
            let to_sq = lerf_to_square(to_lerf);
            let mut m = Move::build(piece, from_sq, to_sq);
            if (opp_pieces.0 & (1u64 << to_lerf)) != 0 {
                m = m.capture();
            }
            v.push(m);
        }
    }

    // KINGS
    let mut kings = Bitboard(my_pieces.0 & position.piece_types[PieceType::King as usize].0);
    while kings.0 != 0 {
        let from_lerf = kings.pop_lsb() as usize;
        let from_sq = lerf_to_square(from_lerf);
        let piece = Piece {
            piece_type: PieceType::King,
            piece_color: us,
        };
        let mut attacks = Bitboard(KING_ATTACKS[from_lerf].0 & !my_pieces.0);
        while attacks.0 != 0 {
            let to_lerf = attacks.pop_lsb() as usize;
            let to_sq = lerf_to_square(to_lerf);
            let mut m = Move::build(piece, from_sq, to_sq);
            if (opp_pieces.0 & (1u64 << to_lerf)) != 0 {
                m = m.capture();
            }
            v.push(m);
        }
    }

    add_castling_moves(&mut v, position, us);
    v
}

fn add_castling_moves(v: &mut Vec<Move>, position: Position, color: PieceColor) {
    let (rights_short, rights_long) = match color {
        PieceColor::White => (position.white_short_castle, position.white_long_castle),
        PieceColor::Black => (position.black_short_castle, position.black_long_castle),
    };
    use crate::square::Squares::{C1, C8, D1, D8, E1, E8, F1, F8, G1, G8};
    let king_square: u8 = if color == PieceColor::White {
        E1 as u8
    } else {
        E8 as u8
    };
    let rank: usize = if color == PieceColor::White { 0 } else { 7 };
    let enemy = if color == PieceColor::White {
        PieceColor::Black
    } else {
        PieceColor::White
    };

    if rights_short {
        if position.get_piece(rank, 5).is_none() && position.get_piece(rank, 6).is_none() {
            let f_sq: u8 = if color == PieceColor::White {
                F1 as u8
            } else {
                F8 as u8
            };
            let g_sq: u8 = if color == PieceColor::White {
                G1 as u8
            } else {
                G8 as u8
            };
            if !is_square_attacked(position, king_square, enemy)
                && !is_square_attacked(position, f_sq, enemy)
                && !is_square_attacked(position, g_sq, enemy)
            {
                v.push(Move::build_short_castling(color));
            }
        }
    }

    if rights_long {
        if position.get_piece(rank, 1).is_none()
            && position.get_piece(rank, 2).is_none()
            && position.get_piece(rank, 3).is_none()
        {
            let d_sq: u8 = if color == PieceColor::White {
                D1 as u8
            } else {
                D8 as u8
            };
            let c_sq: u8 = if color == PieceColor::White {
                C1 as u8
            } else {
                C8 as u8
            };
            if !is_square_attacked(position, king_square, enemy)
                && !is_square_attacked(position, d_sq, enemy)
                && !is_square_attacked(position, c_sq, enemy)
            {
                v.push(Move::build_long_castling(color));
            }
        }
    }
}

pub fn legal_moves(position: Position) -> Vec<Move> {
    let candidates = pseudo_legal_moves(position);
    candidates
        .into_iter()
        .filter(|&m| {
            let mut pos = position;
            make_move(&mut pos, m);
            !is_in_check(pos, position.turn)
        })
        .collect()
}

pub fn capture_moves(position: Position) -> Vec<Move> {
    pseudo_legal_moves(position)
        .into_iter()
        .filter(|m| m.capture)
        .filter(|&m| {
            let mut pos = position;
            make_move(&mut pos, m);
            !is_in_check(pos, position.turn)
        })
        .collect()
}

pub fn make_move(position: &mut Position, m: Move) -> UndoInfo {
    // Save state for undo
    let undo = UndoInfo {
        captured: None,
        white_short_castle: position.white_short_castle,
        white_long_castle: position.white_long_castle,
        black_short_castle: position.black_short_castle,
        black_long_castle: position.black_long_castle,
        last_move: position.last_move,
        hash: position.hash,
    };

    // Handle castling moves first (they use sentinel squares, not real coordinates)
    if m.from == SHORT_CASTLING_SQUARE || m.from == LONG_CASTLING_SQUARE {
        let is_long = m.from == LONG_CASTLING_SQUARE;
        let (king_rank, rook_from_col, rook_to_col) = if m.piece.piece_color == PieceColor::White {
            if is_long {
                (0usize, 0usize, 3usize)
            } else {
                (0usize, 7usize, 5usize)
            }
        } else if is_long {
            (7usize, 0usize, 3usize)
        } else {
            (7usize, 7usize, 5usize)
        };

        // Move king (always to column 6 for short, column 2 for long)
        let king_to_col = if is_long { 2usize } else { 6usize };
        position.set_piece(king_rank, 4, None);
        position.set_piece(king_rank, king_to_col, Some(m.piece));
        // Move rook
        position.set_piece(king_rank, rook_from_col, None);
        position.set_piece(
            king_rank,
            rook_to_col,
            match m.piece.piece_color {
                PieceColor::White => Some(WHITE_ROOK),
                PieceColor::Black => Some(BLACK_ROOK),
            },
        );

        // Lost castle rights for this side
        if m.piece.piece_color == PieceColor::White {
            position.white_short_castle = false;
            position.white_long_castle = false;
        } else {
            position.black_short_castle = false;
            position.black_long_castle = false;
        }

        if position.turn == PieceColor::White {
            position.turn = PieceColor::Black;
        } else {
            position.turn = PieceColor::White;
        }
        position.last_move = Some(m);
        crate::zobrist::update_hash_make_move(position, m, &undo);
        return undo;
    }

    let (from_col_idx, from_row_idx) = get_col_and_row(m.from);
    let (to_col_idx, to_row_idx) = get_col_and_row(m.to);
    let maybe_piece_on_src: Option<Piece> =
        position.get_piece(from_row_idx as usize, from_col_idx as usize);
    match maybe_piece_on_src {
        Some(src_piece) => {
            let maybe_piece_on_dest: Option<Piece> =
                position.get_piece(to_row_idx as usize, to_col_idx as usize);
            if let Some(dst_piece) = maybe_piece_on_dest {
                assert!(
                    (dst_piece.piece_color != src_piece.piece_color),
                    "Square is already occupied!"
                );
                // Save captured piece
                let mut captured_undo = undo;
                captured_undo.captured = Some(dst_piece);
                // If a rook is captured, lose the corresponding castle right
                if dst_piece.piece_type == PieceType::Rook {
                    if to_row_idx == 0 && to_col_idx == 0 {
                        position.white_long_castle = false;
                    } else if to_row_idx == 0 && to_col_idx == 7 {
                        position.white_short_castle = false;
                    } else if to_row_idx == 7 && to_col_idx == 0 {
                        position.black_long_castle = false;
                    } else if to_row_idx == 7 && to_col_idx == 7 {
                        position.black_short_castle = false;
                    }
                }
                position.set_piece(from_row_idx as usize, from_col_idx as usize, None);
                // Handle promotion
                match m.new_piece {
                    Some(promoted) => {
                        position.set_piece(
                            to_row_idx as usize,
                            to_col_idx as usize,
                            Some(promoted),
                        );
                    }
                    None => {
                        position.set_piece(
                            to_row_idx as usize,
                            to_col_idx as usize,
                            Some(src_piece),
                        );
                    }
                }
                // Update castle rights when king or rook moves
                if src_piece.piece_type == PieceType::King {
                    if src_piece.piece_color == PieceColor::White {
                        position.white_short_castle = false;
                        position.white_long_castle = false;
                    } else {
                        position.black_short_castle = false;
                        position.black_long_castle = false;
                    }
                }
                if src_piece.piece_type == PieceType::Rook {
                    if from_row_idx == 0 && from_col_idx == 0 {
                        position.white_long_castle = false;
                    } else if from_row_idx == 0 && from_col_idx == 7 {
                        position.white_short_castle = false;
                    } else if from_row_idx == 7 && from_col_idx == 0 {
                        position.black_long_castle = false;
                    } else if from_row_idx == 7 && from_col_idx == 7 {
                        position.black_short_castle = false;
                    }
                }
                if position.turn == PieceColor::White {
                    position.turn = PieceColor::Black;
                } else {
                    position.turn = PieceColor::White;
                }
                position.last_move = Some(m);
                crate::zobrist::update_hash_make_move(position, m, &captured_undo);
                captured_undo
            } else {
                // En passant: pawn capture to an apparently empty square
                if src_piece.piece_type == PieceType::Pawn && m.capture {
                    position.set_piece(from_row_idx as usize, to_col_idx as usize, None);
                }
                position.set_piece(from_row_idx as usize, from_col_idx as usize, None);
                // Handle promotion
                match m.new_piece {
                    Some(promoted) => {
                        position.set_piece(
                            to_row_idx as usize,
                            to_col_idx as usize,
                            Some(promoted),
                        );
                    }
                    None => {
                        position.set_piece(
                            to_row_idx as usize,
                            to_col_idx as usize,
                            Some(src_piece),
                        );
                    }
                }
                // We don't need a separate "captured_undo" for en passant because
                // we saved the captured pawn info in UndoInfo.captured already.
                // Actually no — for en passant, `captured` remains None but the
                // pawn *was* removed. We'll detect this in unmake_move.
                // Update castle rights when king or rook moves
                if src_piece.piece_type == PieceType::King {
                    if src_piece.piece_color == PieceColor::White {
                        position.white_short_castle = false;
                        position.white_long_castle = false;
                    } else {
                        position.black_short_castle = false;
                        position.black_long_castle = false;
                    }
                }
                if src_piece.piece_type == PieceType::Rook {
                    if from_row_idx == 0 && from_col_idx == 0 {
                        position.white_long_castle = false;
                    } else if from_row_idx == 0 && from_col_idx == 7 {
                        position.white_short_castle = false;
                    } else if from_row_idx == 7 && from_col_idx == 0 {
                        position.black_long_castle = false;
                    } else if from_row_idx == 7 && from_col_idx == 7 {
                        position.black_short_castle = false;
                    }
                }
                if position.turn == PieceColor::White {
                    position.turn = PieceColor::Black;
                } else {
                    position.turn = PieceColor::White;
                }
                position.last_move = Some(m);
                crate::zobrist::update_hash_make_move(position, m, &undo);
                undo // undo.captured is None — signals en passant
            }
        }
        None => {
            panic!("No piece on source square!");
        }
    }
}

/// Undo a move previously made by `make_move`. The `undo` parameter must be
/// the `UndoInfo` returned by that `make_move` call.
pub fn unmake_move(position: &mut Position, m: Move, undo: UndoInfo) {
    // Restore castle rights and last_move first (cheap, no branches)
    position.white_short_castle = undo.white_short_castle;
    position.white_long_castle = undo.white_long_castle;
    position.black_short_castle = undo.black_short_castle;
    position.black_long_castle = undo.black_long_castle;
    position.last_move = undo.last_move;
    position.hash = undo.hash;

    // Flip the turn back
    if position.turn == PieceColor::White {
        position.turn = PieceColor::Black;
    } else {
        position.turn = PieceColor::White;
    }

    // Handle castling
    if m.from == SHORT_CASTLING_SQUARE || m.from == LONG_CASTLING_SQUARE {
        let is_long = m.from == LONG_CASTLING_SQUARE;
        let king_rank = if m.piece.piece_color == PieceColor::White {
            0usize
        } else {
            7usize
        };
        let king_to_col = if is_long { 2usize } else { 6usize };
        let rook_from_col = if is_long { 3usize } else { 5usize };
        let rook_to_col = if is_long { 0usize } else { 7usize };

        // Restore king to starting square (column 4)
        position.set_piece(king_rank, king_to_col, None);
        position.set_piece(king_rank, 4, Some(m.piece));
        // Restore rook
        position.set_piece(king_rank, rook_from_col, None);
        position.set_piece(
            king_rank,
            rook_to_col,
            match m.piece.piece_color {
                PieceColor::White => Some(WHITE_ROOK),
                PieceColor::Black => Some(BLACK_ROOK),
            },
        );
        return;
    }

    let (from_col, from_row) = get_col_and_row(m.from);
    let (to_col, to_row) = get_col_and_row(m.to);

    // Restore the moving piece to its from-square.
    // If it was a promotion, the piece on to-square is the promoted piece;
    // we need to put a pawn back on the from-square.
    let moving_piece = if m.new_piece.is_some() {
        // The piece that moved was a pawn, even though `m.piece` is the promoted type
        let color = m.piece.piece_color;
        match color {
            PieceColor::White => WHITE_PAWN,
            PieceColor::Black => BLACK_PAWN,
        }
    } else {
        m.piece
    };
    position.set_piece(from_row as usize, from_col as usize, Some(moving_piece));

    // Restore the destination square (or en passant captured pawn)
    match undo.captured {
        Some(captured) => {
            // Normal capture: put the captured piece back
            position.set_piece(to_row as usize, to_col as usize, Some(captured));
        }
        None => {
            if m.capture {
                // En passant: pawn was captured on (from_row, to_col), not (to_row, to_col)
                // The destination becomes empty, and the captured pawn goes back
                // on the rank of the capturing pawn's from-row, same file as capture target.
                position.set_piece(to_row as usize, to_col as usize, None);
                let ep_color = match moving_piece.piece_color {
                    PieceColor::White => PieceColor::Black,
                    PieceColor::Black => PieceColor::White,
                };
                let ep_piece = match ep_color {
                    PieceColor::White => WHITE_PAWN,
                    PieceColor::Black => BLACK_PAWN,
                };
                position.set_piece(from_row as usize, to_col as usize, Some(ep_piece));
            } else {
                // Non-capture: destination becomes empty
                position.set_piece(to_row as usize, to_col as usize, None);
            }
        }
    }
}

impl Position {
    /// Applies a null move: passes the turn and clears en-passant state.
    pub fn make_null_move(&mut self) -> NullUndoInfo {
        let undo = NullUndoInfo {
            last_move: self.last_move,
            hash: self.hash,
        };

        // Clear en passant hash
        if let Some(ep) = self.en_passant_col() {
            self.hash ^= crate::zobrist::ZOBRIST.en_passant[ep];
        }

        // Set last_move to None to clear en_passant state
        self.last_move = None;

        // Switch turn
        self.turn = match self.turn {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        };
        self.hash ^= crate::zobrist::ZOBRIST.side_to_move;

        undo
    }

    /// Undoes a null move.
    pub fn unmake_null_move(&mut self, undo: NullUndoInfo) {
        self.turn = match self.turn {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        };
        self.last_move = undo.last_move;
        self.hash = undo.hash;
    }

    /// Checks if the given side has any non-pawn, non-king material.
    /// Used for Null Move Pruning to avoid zugzwang endgames.
    pub fn has_non_pawn_material(&self, color: PieceColor) -> bool {
        for row in 0..8 {
            for col in 0..8 {
                if let Some(p) = self.get_piece(row, col) {
                    if p.piece_color == color
                        && p.piece_type != PieceType::Pawn
                        && p.piece_type != PieceType::King
                    {
                        return true;
                    }
                }
            }
        }
        false
    }
}
