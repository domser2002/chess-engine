use crate::piece::{PieceType, PieceColor, Piece};
use crate::position::Position;
use std::sync::LazyLock;

pub struct ZobristKeys {
    pub pieces: [[[u64; 64]; 6]; 2], // [Color 0=White, 1=Black][Type 0..5][Square 0..63]
    pub castling: [u64; 16],
    pub en_passant: [u64; 8], // Indexed by column (0..7)
    pub side_to_move: u64,
}

impl ZobristKeys {
    pub fn new() -> Self {
        let mut prng = Prng::new(1070372); // Arbitrary seed
        
        let mut pieces = [[[0; 64]; 6]; 2];
        for c in 0..2 {
            for t in 0..6 {
                for s in 0..64 {
                    pieces[c][t][s] = prng.rand64();
                }
            }
        }
        
        let mut castling = [0; 16];
        for i in 0..16 {
            castling[i] = prng.rand64();
        }
        
        let mut en_passant = [0; 8];
        for i in 0..8 {
            en_passant[i] = prng.rand64();
        }
        
        Self {
            pieces,
            castling,
            en_passant,
            side_to_move: prng.rand64(),
        }
    }
}

struct Prng {
    s: u64,
}

impl Prng {
    fn new(seed: u64) -> Self { Self { s: seed } }
    fn rand64(&mut self) -> u64 {
        self.s ^= self.s >> 12;
        self.s ^= self.s << 25;
        self.s ^= self.s >> 27;
        self.s.wrapping_mul(2685821657736338717)
    }
}

pub static ZOBRIST: LazyLock<ZobristKeys> = LazyLock::new(ZobristKeys::new);

/// Computes the Zobrist hash of a position from scratch.
pub fn compute_hash(pos: &Position) -> u64 {
    let mut hash = 0;

    for row in 0..8 {
        for col in 0..8 {
            if let Some(piece) = pos.get_piece(row, col) {
                let color_idx = if piece.piece_color == PieceColor::White { 0 } else { 1 };
                let type_idx = piece_type_to_index(piece.piece_type);
                let sq_idx = row * 8 + col;
                hash ^= ZOBRIST.pieces[color_idx][type_idx][sq_idx];
            }
        }
    }

    if pos.turn == PieceColor::Black {
        hash ^= ZOBRIST.side_to_move;
    }

    let castling_index = castling_rights_to_index(
        pos.white_short_castle,
        pos.white_long_castle,
        pos.black_short_castle,
        pos.black_long_castle
    );
    hash ^= ZOBRIST.castling[castling_index];

    if let Some(ep_col) = pos.en_passant_col() {
        hash ^= ZOBRIST.en_passant[ep_col];
    }

    hash
}

pub fn piece_type_to_index(pt: PieceType) -> usize {
    match pt {
        PieceType::Pawn => 0,
        PieceType::Knight => 1,
        PieceType::Bishop => 2,
        PieceType::Rook => 3,
        PieceType::Queen => 4,
        PieceType::King => 5,
    }
}

pub fn castling_rights_to_index(ws: bool, wl: bool, bs: bool, bl: bool) -> usize {
    let mut idx = 0;
    if ws { idx |= 1; }
    if wl { idx |= 2; }
    if bs { idx |= 4; }
    if bl { idx |= 8; }
    idx
}

impl Position {
    /// Helper to find if there's a valid en passant target in this position
    /// based on the `last_move`.
    pub fn en_passant_col(&self) -> Option<usize> {
        if let Some(m) = self.last_move {
            if m.piece.piece_type == PieceType::Pawn {
                let from_row = m.from / 8;
                let to_row = m.to / 8;
                if (from_row as i8 - to_row as i8).abs() == 2 {
                    return Some((m.to % 8) as usize);
                }
            }
        }
        None
    }
}

pub fn update_hash_make_move(pos: &mut Position, m: crate::r#move::Move, undo: &crate::position::UndoInfo) {
    // 1. XOR out old castling
    pos.hash ^= ZOBRIST.castling[castling_rights_to_index(undo.white_short_castle, undo.white_long_castle, undo.black_short_castle, undo.black_long_castle)];
    
    // 2. XOR in new castling
    pos.hash ^= ZOBRIST.castling[castling_rights_to_index(pos.white_short_castle, pos.white_long_castle, pos.black_short_castle, pos.black_long_castle)];

    // 3. XOR out old en_passant
    if let Some(old_m) = undo.last_move {
        if old_m.piece.piece_type == PieceType::Pawn {
            let from_row = old_m.from / 8;
            let to_row = old_m.to / 8;
            if (from_row as i8 - to_row as i8).abs() == 2 {
                pos.hash ^= ZOBRIST.en_passant[(old_m.to % 8) as usize];
            }
        }
    }

    // 4. XOR in new en_passant
    if let Some(new_ep) = pos.en_passant_col() {
        pos.hash ^= ZOBRIST.en_passant[new_ep];
    }

    // 5. Switch turn
    pos.hash ^= ZOBRIST.side_to_move;

    // 6. Handle pieces
    if m.from == crate::r#move::SHORT_CASTLING_SQUARE || m.from == crate::r#move::LONG_CASTLING_SQUARE {
        let is_long = m.from == crate::r#move::LONG_CASTLING_SQUARE;
        let (king_rank, rook_from_col, rook_to_col) = if m.piece.piece_color == PieceColor::White {
            if is_long { (0, 0, 3) } else { (0, 7, 5) }
        } else if is_long { (7, 0, 3) } else { (7, 7, 5) };
        let king_to_col = if is_long { 2 } else { 6 };
        
        let color_idx = if m.piece.piece_color == PieceColor::White { 0 } else { 1 };
        
        // XOR out King from E file (col 4)
        pos.hash ^= ZOBRIST.pieces[color_idx][piece_type_to_index(PieceType::King)][king_rank * 8 + 4];
        // XOR in King
        pos.hash ^= ZOBRIST.pieces[color_idx][piece_type_to_index(PieceType::King)][king_rank * 8 + king_to_col];
        
        // XOR out Rook
        pos.hash ^= ZOBRIST.pieces[color_idx][piece_type_to_index(PieceType::Rook)][king_rank * 8 + rook_from_col];
        // XOR in Rook
        pos.hash ^= ZOBRIST.pieces[color_idx][piece_type_to_index(PieceType::Rook)][king_rank * 8 + rook_to_col];
    } else {
        let (from_col, from_row) = crate::square::get_col_and_row(m.from);
        let from_sq = (from_row * 8 + from_col) as usize;
        let (to_col, to_row) = crate::square::get_col_and_row(m.to);
        let to_sq = (to_row * 8 + to_col) as usize;
        let color_idx = if m.piece.piece_color == PieceColor::White { 0 } else { 1 };
        
        // Piece that moved
        let moving_piece = if m.new_piece.is_some() {
            // It's a pawn that promoted
            Piece { piece_color: m.piece.piece_color, piece_type: PieceType::Pawn }
        } else {
            m.piece
        };
        
        // XOR out from square
        pos.hash ^= ZOBRIST.pieces[color_idx][piece_type_to_index(moving_piece.piece_type)][from_sq];
        
        // XOR in to square
        let piece_to_sq = m.new_piece.unwrap_or(moving_piece);
        pos.hash ^= ZOBRIST.pieces[color_idx][piece_type_to_index(piece_to_sq.piece_type)][to_sq];
        
        // Handle capture
        if let Some(captured) = undo.captured {
            let cap_color_idx = if captured.piece_color == PieceColor::White { 0 } else { 1 };
            // In normal capture, it was at to_sq.
            pos.hash ^= ZOBRIST.pieces[cap_color_idx][piece_type_to_index(captured.piece_type)][to_sq];
        } else if m.capture {
            // En passant capture!
            let cap_color_idx = if m.piece.piece_color == PieceColor::White { 1 } else { 0 };
            let cap_sq = (from_row * 8 + to_col) as usize; // Same row as from, same col as to
            pos.hash ^= ZOBRIST.pieces[cap_color_idx][piece_type_to_index(PieceType::Pawn)][cap_sq];
        }
    }
}
