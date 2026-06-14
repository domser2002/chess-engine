use crate::piece::*;
use crate::r#move::{Move, SHORT_CASTLING_SQUARE, LONG_CASTLING_SQUARE};
use crate::square::*;
use crate::square::Squares::*;
use colored::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Position {
    pub pieces: [[Option<Piece>; COL_NUM as usize]; ROW_NUM as usize],
    pub turn: PieceColor,
    pub last_move: Option<Move>,
    pub white_short_castle: bool,
    pub white_long_castle: bool,
    pub black_short_castle: bool,
    pub black_long_castle: bool,
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
}

const ROOK_DIRS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const BISHOP_DIRS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
const KING_DIRS: [(i8, i8); 8] = [
    (1, 0), (-1, 0), (0, 1), (0, -1),
    (1, 1), (1, -1), (-1, 1), (-1, -1),
];
const KNIGHT_OFFSETS: [(i8, i8); 8] = [
    (2, 1), (1, 2), (-2, 1), (-1, 2),
    (2, -1), (1, -2), (-2, -1), (-1, -2),
];

pub fn empty_position(turn: PieceColor) -> Position {
    Position {
        pieces: [[None; COL_NUM as usize]; ROW_NUM as usize],
        turn,
        last_move: None,
        white_short_castle: false,
        white_long_castle: false,
        black_short_castle: false,
        black_long_castle: false,
    }
}

pub fn start_position() -> Position {
 Position {
    pieces: [
        [Some(WHITE_ROOK), Some(WHITE_KNIGHT), Some(WHITE_BISHOP), Some(WHITE_QUEEN), Some(WHITE_KING), Some(WHITE_BISHOP), Some(WHITE_KNIGHT), Some(WHITE_ROOK)],
        [Some(WHITE_PAWN); 8], 
        [None;8], [None;8], [None; 8], [None; 8],
        [Some(BLACK_PAWN); 8], 
        [Some(BLACK_ROOK) , Some(BLACK_KNIGHT), Some(BLACK_BISHOP), Some(BLACK_QUEEN), Some(BLACK_KING), Some(BLACK_BISHOP), Some(BLACK_KNIGHT), Some(BLACK_ROOK)]],
        turn: PieceColor::White,
        last_move: None,
        white_short_castle: true,
        white_long_castle: true,
        black_short_castle: true,
        black_long_castle: true,
    }
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
                pos.pieces[row][col] = Some(piece);
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
    let horizontal_line: String = std::iter::repeat('-').take(17).collect();
    for row_idx in (0..ROW_NUM).rev() {
        println!("  {horizontal_line}");
        print!("{} |", row_idx + 1);
        for col_idx in 0..COL_NUM {
            let maybe_piece: Option<Piece> = position.pieces[row_idx as usize][col_idx as usize];
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

fn add_sliding_moves(
    v: &mut Vec<Move>,
    position: Position,
    piece: Piece,
    square: u8,
    directions: &[(i8, i8)],
    max_len: i8,
) {
    let (col, row) = get_col_and_row(square);
    for (dc, dr) in directions {
        for i in 1..=max_len {
            let nr = row as i8 + dr * i;
            let nc = col as i8 + dc * i;
            if nr < 0 || nr >= ROW_NUM as i8 || nc < 0 || nc >= COL_NUM as i8 {
                break;
            }
            let target = position.pieces[nr as usize][nc as usize];
            match target {
                None => {
                    v.push(Move::build(piece, square, get_square_idx(nr as u8, nc as u8)));
                }
                Some(p) if p.piece_color != piece.piece_color => {
                    v.push(Move::build(piece, square, get_square_idx(nr as u8, nc as u8)).capture());
                    break;
                }
                _ => break, // own piece blocks
            }
        }
    }
}

fn add_knight_moves(
    v: &mut Vec<Move>,
    position: Position,
    piece: Piece,
    square: u8,
) {
    let (col, row) = get_col_and_row(square);
    for (dc, dr) in KNIGHT_OFFSETS {
        let nr = row as i8 + dr;
        let nc = col as i8 + dc;
        if nr < 0 || nr >= ROW_NUM as i8 || nc < 0 || nc >= COL_NUM as i8 {
            continue;
        }
        let target = position.pieces[nr as usize][nc as usize];
        match target {
            None => {
                v.push(Move::build(piece, square, get_square_idx(nr as u8, nc as u8)));
            }
            Some(p) if p.piece_color != piece.piece_color => {
                v.push(Move::build(piece, square, get_square_idx(nr as u8, nc as u8)).capture());
            }
            _ => {} // own piece blocks
        }
    }
}

fn add_pawn_moves(
    v: &mut Vec<Move>,
    position: Position,
    piece: Piece,
    square: u8,
) {
    let (col, row) = get_col_and_row(square);
    let direction: i8 = if piece.piece_color == PieceColor::White { 1 } else { -1 };
    let max_len: i8 = if (row == 1 && direction == 1) || (row == 6 && direction == -1) { 2 } else { 1 };

    // One step forward
    let nr = row as i8 + direction;
    if nr >= 0 && nr < ROW_NUM as i8 {
        let target = position.pieces[nr as usize][col as usize];
        if target.is_none() {
            let forward_move = Move::build(piece, square, get_square_idx(nr as u8, col as u8));
            let dest_row_idx = if direction == 1 { nr } else { nr };
            if !try_promotions(dest_row_idx, piece, v, forward_move) {
                v.push(forward_move);
            }
        }
    }

    // Double step forward from starting rank
    if max_len == 2 {
        let nr2 = row as i8 + 2 * direction;
        let nr1 = row as i8 + direction;
        let target1 = position.pieces[nr1 as usize][col as usize];
        let target2 = position.pieces[nr2 as usize][col as usize];
        if target1.is_none() && target2.is_none() {
            v.push(Move::build(piece, square, get_square_idx(nr2 as u8, col as u8)));
        }
    }

    // Captures: forward-left and forward-right
    for dc in [-1, 1] {
        let nc = col as i8 + dc;
        let nr = row as i8 + direction;
        if nr >= 0 && nr < ROW_NUM as i8 && nc >= 0 && nc < COL_NUM as i8 {
            let target = position.pieces[nr as usize][nc as usize];
            match target {
                Some(p) if p.piece_color != piece.piece_color => {
                    let capture_move = Move::build(piece, square, get_square_idx(nr as u8, nc as u8)).capture();
                    if !try_promotions(nr, piece, v, capture_move) {
                        v.push(capture_move);
                    }
                }
                _ => {}
            }
        }
    }

    // En passant
    try_en_passant(position, piece, col, row, square, direction, v);
}

fn try_promotions(dest_row_idx: i8, piece: Piece, v: &mut Vec<Move>, possible_move: Move) -> bool {
    if piece.piece_type == PieceType::Pawn && dest_row_idx == 0 {
        v.push(possible_move.promote_to(BLACK_QUEEN));
        v.push(possible_move.promote_to(BLACK_ROOK));
        v.push(possible_move.promote_to(BLACK_BISHOP));
        v.push(possible_move.promote_to(BLACK_KNIGHT));
        return true;
    }
    else if piece.piece_type == PieceType::Pawn && dest_row_idx == 7 {
        v.push(possible_move.promote_to(WHITE_QUEEN));
        v.push(possible_move.promote_to(WHITE_ROOK));
        v.push(possible_move.promote_to(WHITE_BISHOP));
        v.push(possible_move.promote_to(WHITE_KNIGHT));
        return true;
    }
    false
}

fn try_en_passant(position: Position, piece: Piece, col_idx: u8, row_idx: u8, square: u8, direction: i8, v: &mut Vec<Move>) {
    match position.last_move {
        None => {}
        Some(last_move) => {
            match last_move.piece.piece_type {
            PieceType::Pawn => {
                let (last_col_idx, last_row_idx) = get_col_and_row(last_move.to);
                let (_, last_from_row_idx) = get_col_and_row(last_move.from);
                let diff = last_row_idx as i8 - last_from_row_idx as i8;
                if diff == 2 || diff == -2 {
                    let mut diff_col = col_idx as i8 - last_col_idx as i8;
                        if diff_col == -1 {
                        diff_col = 1;
                        }
                    if last_row_idx == row_idx && diff_col == 1 {
                        let possible_move = Move::build(piece, square, get_square_idx(((last_row_idx as i8) + direction) as u8, last_col_idx)).capture();
                        v.push(possible_move);
                    }
                }
            }
            _ => {}
            }
        }
    }
}

/// Check if a given square is attacked by any piece of `by_color`.
/// Scans outward from the square using the same logic as is_in_check.
pub fn is_square_attacked(position: Position, square: u8, by_color: PieceColor) -> bool {
    let (target_col, target_row) = get_col_and_row(square);

    // Check knight attacks
    for (dc, dr) in KNIGHT_OFFSETS {
        let nr = target_row as i8 + dr;
        let nc = target_col as i8 + dc;
        if nr < 0 || nr >= ROW_NUM as i8 || nc < 0 || nc >= COL_NUM as i8 {
            continue;
        }
        if let Some(piece) = position.pieces[nr as usize][nc as usize] {
            if piece.piece_color == by_color && piece.piece_type == PieceType::Knight {
                return true;
            }
        }
    }

    // Check king attacks (enemy king adjacent)
    for (dc, dr) in KING_DIRS {
        let nr = target_row as i8 + dr;
        let nc = target_col as i8 + dc;
        if nr < 0 || nr >= ROW_NUM as i8 || nc < 0 || nc >= COL_NUM as i8 {
            continue;
        }
        if let Some(piece) = position.pieces[nr as usize][nc as usize] {
            if piece.piece_color == by_color && piece.piece_type == PieceType::King {
                return true;
            }
        }
    }

    // Check pawn attacks
    // White pawns move upward (row+1), so they attack from one row below (row-1)
    // Black pawns move downward (row-1), so they attack from one row above (row+1)
    let pawn_dir: i8 = if by_color == PieceColor::White { -1 } else { 1 };
    for dc in [-1, 1] {
        let nr = target_row as i8 + pawn_dir;
        let nc = target_col as i8 + dc;
        if nr < 0 || nr >= ROW_NUM as i8 || nc < 0 || nc >= COL_NUM as i8 {
            continue;
        }
        if let Some(piece) = position.pieces[nr as usize][nc as usize] {
            if piece.piece_color == by_color && piece.piece_type == PieceType::Pawn {
                return true;
            }
        }
    }

    // Check sliding attacks (rooks, bishops, queens)
    // Rook directions (orthogonal)
    for (dc, dr) in ROOK_DIRS {
        let mut i: i8 = 1;
        loop {
            let nr = target_row as i8 + dr * i;
            let nc = target_col as i8 + dc * i;
            if nr < 0 || nr >= ROW_NUM as i8 || nc < 0 || nc >= COL_NUM as i8 {
                break;
            }
            if let Some(piece) = position.pieces[nr as usize][nc as usize] {
                if piece.piece_color == by_color
                    && (piece.piece_type == PieceType::Rook || piece.piece_type == PieceType::Queen)
                {
                    return true;
                }
                break; // blocked by any piece
            }
            i += 1;
        }
    }

    // Bishop directions (diagonal)
    for (dc, dr) in BISHOP_DIRS {
        let mut i: i8 = 1;
        loop {
            let nr = target_row as i8 + dr * i;
            let nc = target_col as i8 + dc * i;
            if nr < 0 || nr >= ROW_NUM as i8 || nc < 0 || nc >= COL_NUM as i8 {
                break;
            }
            if let Some(piece) = position.pieces[nr as usize][nc as usize] {
                if piece.piece_color == by_color
                    && (piece.piece_type == PieceType::Bishop || piece.piece_type == PieceType::Queen)
                {
                    return true;
                }
                break;
            }
            i += 1;
        }
    }

    false
}

/// Check if the king of the given color is in check.
pub fn is_in_check(position: Position, color: PieceColor) -> bool {
    // Find the king
    for square_num in 1..=(ROW_NUM * COL_NUM) {
        let (col_idx, row_idx) = get_col_and_row(square_num);
        if let Some(piece) = position.pieces[row_idx as usize][col_idx as usize] {
            if piece.piece_type == PieceType::King && piece.piece_color == color {
                let enemy = if color == PieceColor::White { PieceColor::Black } else { PieceColor::White };
                return is_square_attacked(position, square_num, enemy);
            }
        }
    }
    false
}

fn add_castling_moves(v: &mut Vec<Move>, position: Position, color: PieceColor) {
    let (rights_short, rights_long) = match color {
        PieceColor::White => (position.white_short_castle, position.white_long_castle),
        PieceColor::Black => (position.black_short_castle, position.black_long_castle),
    };
    let king_square: u8 = if color == PieceColor::White { E1 as u8 } else { E8 as u8 };
    let rank: usize = if color == PieceColor::White { 0 } else { 7 };
    let enemy = if color == PieceColor::White { PieceColor::Black } else { PieceColor::White };

    // Short castling (kingside)
    if rights_short {
        // Squares between king and rook must be empty: F1, G1 (or F8, G8)
        if position.pieces[rank][5].is_none() && position.pieces[rank][6].is_none() {
            // King must not be in check, pass through check, or end in check
            let f_sq: u8 = if color == PieceColor::White { F1 as u8 } else { F8 as u8 };
            let g_sq: u8 = if color == PieceColor::White { G1 as u8 } else { G8 as u8 };
            if !is_square_attacked(position, king_square, enemy)
                && !is_square_attacked(position, f_sq, enemy)
                && !is_square_attacked(position, g_sq, enemy)
            {
                v.push(Move::build_short_castling(color));
            }
        }
    }

    // Long castling (queenside)
    if rights_long {
        // Squares between king and rook must be empty: B1, C1, D1 (or B8, C8, D8)
        if position.pieces[rank][1].is_none()
            && position.pieces[rank][2].is_none()
            && position.pieces[rank][3].is_none()
        {
            // King must not be in check, pass through check, or end in check
            let d_sq: u8 = if color == PieceColor::White { D1 as u8 } else { D8 as u8 };
            let c_sq: u8 = if color == PieceColor::White { C1 as u8 } else { C8 as u8 };
            if !is_square_attacked(position, king_square, enemy)
                && !is_square_attacked(position, d_sq, enemy)
                && !is_square_attacked(position, c_sq, enemy)
            {
                v.push(Move::build_long_castling(color));
            }
        }
    }
}

/// Generate pseudo-legal moves (moves that don't account for leaving own king in check).
pub fn pseudo_legal_moves(position: Position) -> Vec<Move> {
    let mut v = Vec::new();
    for square_num in 1..=(ROW_NUM * COL_NUM) {
        let (col_idx, row_idx) = get_col_and_row(square_num);
        let maybe_piece = position.pieces[row_idx as usize][col_idx as usize];
        let Some(piece) = maybe_piece else { continue; };
        if position.turn != piece.piece_color {
            continue;
        }
        match piece.piece_type {
            PieceType::Pawn => add_pawn_moves(&mut v, position, piece, square_num),
            PieceType::Knight => add_knight_moves(&mut v, position, piece, square_num),
            PieceType::Bishop => add_sliding_moves(&mut v, position, piece, square_num, &BISHOP_DIRS, 8),
            PieceType::Rook => add_sliding_moves(&mut v, position, piece, square_num, &ROOK_DIRS, 8),
            PieceType::Queen => {
                add_sliding_moves(&mut v, position, piece, square_num, &ROOK_DIRS, 8);
                add_sliding_moves(&mut v, position, piece, square_num, &BISHOP_DIRS, 8);
            }
            PieceType::King => {
                add_sliding_moves(&mut v, position, piece, square_num, &KING_DIRS, 1);
            }
        }
    }
    // Add castling moves for the side to move
    add_castling_moves(&mut v, position, position.turn);
    v
}

/// Generate legal moves (pseudo-legal moves filtered by "doesn't leave king in check").
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

// TODO:
// 1. Check if move is legal.
// 2. Allow special moves- castling, promotion, en passant.
pub fn make_move(position: &mut Position, m: Move) -> UndoInfo {
    // Save state for undo
    let undo = UndoInfo {
        captured: None,
        white_short_castle: position.white_short_castle,
        white_long_castle: position.white_long_castle,
        black_short_castle: position.black_short_castle,
        black_long_castle: position.black_long_castle,
        last_move: position.last_move,
    };

    // Handle castling moves first (they use sentinel squares, not real coordinates)
    if m.from == SHORT_CASTLING_SQUARE || m.from == LONG_CASTLING_SQUARE {
        let is_long = m.from == LONG_CASTLING_SQUARE;
        let (king_rank, rook_from_col, rook_to_col) = if m.piece.piece_color == PieceColor::White {
            if is_long { (0usize, 0usize, 3usize) } else { (0usize, 7usize, 5usize) }
        } else {
            if is_long { (7usize, 0usize, 3usize) } else { (7usize, 7usize, 5usize) }
        };

        // Move king (always to column 6 for short, column 2 for long)
        let king_to_col = if is_long { 2usize } else { 6usize };
        position.pieces[king_rank][4] = None;
        position.pieces[king_rank][king_to_col] = Some(m.piece);
        // Move rook
        position.pieces[king_rank][rook_from_col] = None;
        position.pieces[king_rank][rook_to_col] = match m.piece.piece_color {
            PieceColor::White => Some(WHITE_ROOK),
            PieceColor::Black => Some(BLACK_ROOK),
        };

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
        return undo;
    }

    let (from_col_idx, from_row_idx) = get_col_and_row(m.from);
    let (to_col_idx, to_row_idx) = get_col_and_row(m.to);
    let maybe_piece_on_src: Option<Piece> = position.pieces[from_row_idx as usize][from_col_idx as usize];
    match maybe_piece_on_src {
        Some(src_piece) => {
            let maybe_piece_on_dest: Option<Piece> = position.pieces[to_row_idx as usize][to_col_idx as usize];
            match maybe_piece_on_dest {
                Some(dst_piece) => {
                    if dst_piece.piece_color == src_piece.piece_color {
                        panic!("Square is already occupied!");
                    }
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
                    position.pieces[from_row_idx as usize][from_col_idx as usize] = None;
                    // Handle promotion
                    match m.new_piece {
                        Some(promoted) => {
                            position.pieces[to_row_idx as usize][to_col_idx as usize] = Some(promoted);
                        }
                        None => {
                            position.pieces[to_row_idx as usize][to_col_idx as usize] = Some(src_piece);
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
                    }
                    else {
                        position.turn = PieceColor::White;
                    }
                    position.last_move = Some(m);
                    return captured_undo;
                }
                None => {
                    // En passant: pawn capture to an apparently empty square
                    if src_piece.piece_type == PieceType::Pawn && m.capture {
                        position.pieces[from_row_idx as usize][to_col_idx as usize] = None;
                    }
                    position.pieces[from_row_idx as usize][from_col_idx as usize] = None;
                    // Handle promotion
                    match m.new_piece {
                        Some(promoted) => {
                            position.pieces[to_row_idx as usize][to_col_idx as usize] = Some(promoted);
                        }
                        None => {
                            position.pieces[to_row_idx as usize][to_col_idx as usize] = Some(src_piece);
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
                    }
                    else {
                        position.turn = PieceColor::White;
                    }
                    position.last_move = Some(m);
                    return undo; // undo.captured is None — signals en passant
                }
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

    // Flip the turn back
    if position.turn == PieceColor::White {
        position.turn = PieceColor::Black;
    } else {
        position.turn = PieceColor::White;
    }

    // Handle castling
    if m.from == SHORT_CASTLING_SQUARE || m.from == LONG_CASTLING_SQUARE {
        let is_long = m.from == LONG_CASTLING_SQUARE;
        let king_rank = if m.piece.piece_color == PieceColor::White { 0usize } else { 7usize };
        let king_to_col = if is_long { 2usize } else { 6usize };
        let rook_from_col = if is_long { 3usize } else { 5usize };
        let rook_to_col = if is_long { 0usize } else { 7usize };

        // Restore king to starting square (column 4)
        position.pieces[king_rank][king_to_col] = None;
        position.pieces[king_rank][4] = Some(m.piece);
        // Restore rook
        position.pieces[king_rank][rook_from_col] = None;
        position.pieces[king_rank][rook_to_col] = match m.piece.piece_color {
            PieceColor::White => Some(WHITE_ROOK),
            PieceColor::Black => Some(BLACK_ROOK),
        };
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
    position.pieces[from_row as usize][from_col as usize] = Some(moving_piece);

    // Restore the destination square (or en passant captured pawn)
    match undo.captured {
        Some(captured) => {
            // Normal capture: put the captured piece back
            position.pieces[to_row as usize][to_col as usize] = Some(captured);
        }
        None => {
            if m.capture {
                // En passant: pawn was captured on (from_row, to_col), not (to_row, to_col)
                // The destination becomes empty, and the captured pawn goes back
                // on the rank of the capturing pawn's from-row, same file as capture target.
                position.pieces[to_row as usize][to_col as usize] = None;
                let ep_color = match moving_piece.piece_color {
                    PieceColor::White => PieceColor::Black,
                    PieceColor::Black => PieceColor::White,
                };
                let ep_piece = match ep_color {
                    PieceColor::White => WHITE_PAWN,
                    PieceColor::Black => BLACK_PAWN,
                };
                position.pieces[from_row as usize][to_col as usize] = Some(ep_piece);
            } else {
                // Non-capture: destination becomes empty
                position.pieces[to_row as usize][to_col as usize] = None;
            }
        }
    }
}