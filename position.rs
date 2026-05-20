use crate::{Piece, PieceColor, r#move::Move, piece::*, square::*};
use colored::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Position {
    pub pieces: [[Option<Piece>; COL_NUM as usize]; ROW_NUM as usize],
    pub turn: PieceColor,
    // for later
    pub last_move: Option<Move>,
    // pub white_long_castle: bool,
    // pub white_short_castle: bool,
    // pub black_long_castle: bool,
    // pub black_short_castle: bool,
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
    }
}

fn print_wrapper(c: char, pc: PieceColor) {
    let text = c.to_string();
    let colored = match pc {
        PieceColor::Black => text.black(),
        PieceColor::White => text.yellow(),
    };

    print!("{colored}");
}

// this will not be unit tested as it is only for UI purpose
pub fn print_position(position: Position) {
    let horizontal_line: String = std::iter::repeat('-').take(16).collect();
    let vertical_line: String = "|".to_string();
    for row_idx in (0..ROW_NUM).rev() {
        println!("{horizontal_line}");
        for col_idx in 0..COL_NUM {
            print!("{vertical_line}");
            let maybe_piece: Option<Piece> = position.pieces[row_idx as usize][col_idx as usize];
            match maybe_piece {
                Some(piece) => {
                    let piece_char = get_char(piece);
                    match piece_char {
                        Some(c) => {
                            print_wrapper(c, piece.piece_color);
                        }
                        None => {
                            print_wrapper('p', piece.piece_color);
                        }
                    }
                }
                None => {
                    print!(" ");
                }
            }
        }
        println!("{vertical_line}");
    }
    println!("{horizontal_line}");
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

pub fn legal_moves(position: Position) -> Vec<Move> {
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
            PieceType::King => add_sliding_moves(&mut v, position, piece, square_num, &KING_DIRS, 1),
        }
    }
    v
}

// TODO:
// 1. Check if move is legal.
// 2. Allow special moves- castling, promotion, en passant.
pub fn make_move(position: &mut Position, m: Move) {
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
                    else {}
                }
                None => {} 
            }
            position.pieces[from_row_idx as usize][from_col_idx as usize] = None;
            position.pieces[to_row_idx as usize][to_col_idx as usize] = Some(src_piece);
        }
        None => {
            panic!("No piece on source square!");
        }
    }
    if position.turn == PieceColor::White {
        position.turn = PieceColor::Black;
    }
    else {
        position.turn = PieceColor::White;
    }
    position.last_move = Some(m);
}